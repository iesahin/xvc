use chrono::Utc;
use crossbeam_channel::{bounded, Sender};
use derive_more::{AsRef, Deref, Display, From};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::str::FromStr;

use xvc_config::FromConfigKey;
use xvc_config::{UpdateFromXvcConfig, XvcConfig};
use xvc_core::util::git::build_gitignore;

use xvc_core::{
    ContentDigest, DiffStore3, HashAlgorithm, XvcCachePath, XvcFileType, XvcMetadata, XvcRoot,
};
use xvc_logging::{error, info, watch, XvcOutputLine};
use xvc_walker::{check_ignore, AbsolutePath, IgnoreRules, MatchResult};

use crate::carry_in::carry_in;
use crate::common::compare::{
    diff_cache_type, diff_content_digest, diff_text_or_binary, diff_xvc_path_metadata,
};
use crate::common::{
    decide_no_parallel, expand_xvc_dir_file_targets, targets_from_disk, update_store_records,
    FileTextOrBinary,
};
use crate::error::{Error, Result};

use std::fs::{self, OpenOptions};

use clap::Parser;
use std::path::PathBuf;

use xvc_core::CacheType;
use xvc_core::XvcPath;
use xvc_ecs::{HStore, XvcEntity, XvcStore};

/// Add files for tracking with Xvc
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, From, Parser)]
#[command(rename_all = "kebab-case")]
pub struct TrackCLI {
    /// How to track the file contents in cache: One of copy, symlink, hardlink, reflink.
    ///
    /// Note: Reflink uses copy if the underlying file system doesn't support it.
    #[arg(long)]
    cache_type: Option<CacheType>,

    /// Do not copy/link added files to the file cache
    #[arg(long)]
    no_commit: bool,
    /// Calculate digests as text or binary file without checking contents, or by automatically. (Default:
    /// auto)
    #[arg(long)]
    text_or_binary: Option<FileTextOrBinary>,
    /// Add targets even if they are already tracked
    #[arg(long)]
    force: bool,
    /// Don't use parallelism
    #[arg(long)]
    no_parallel: bool,
    /// Files/directories to track
    #[arg()]
    targets: Option<Vec<String>>,
}

impl UpdateFromXvcConfig for TrackCLI {
    /// Updates `xvc file` configuration from the configuration files.
    /// Command line options take precedence over other sources.
    /// If options are not given, they are supplied from [XvcConfig]
    fn update_from_conf(self, conf: &XvcConfig) -> xvc_config::error::Result<Box<Self>> {
        let cache_type = self
            .cache_type
            .unwrap_or_else(|| CacheType::from_conf(conf));
        let no_commit = self.no_commit || conf.get_bool("file.add.no_commit")?.option;
        let force = self.force || conf.get_bool("file.add.force")?.option;
        let no_parallel = self.no_parallel || conf.get_bool("file.add.no_parallel")?.option;
        let text_or_binary = self.text_or_binary.as_ref().map_or_else(
            || Some(FileTextOrBinary::from_conf(conf)),
            |v| Some(v.to_owned()),
        );

        Ok(Box::new(Self {
            targets: self.targets.clone(),
            cache_type: Some(cache_type),
            no_commit,
            force,
            no_parallel,
            text_or_binary,
        }))
    }
}

/// ## The pipeline
///
/// ```mermaid
/// graph LR
///     Target --> |File| Path
///     Target -->|Directory| Dir
///     Dir --> |File| Path
///     Dir --> |Directory| Dir
///     Path --> Ignored{Is this ignored?}
///     Ignored --> |Yes| Ignore
///     Ignored --> |No| XvcPath
///     XvcPath --> |Force| XvcDigest
///     XvcPath --> Filter{Is this changed?}
///     Filter -->|Yes| XvcDigest
///     Filter -->|No| Ignore
///     XvcDigest --> CacheLocation
///     CacheLocation --> CacheType{What's the cache type?}
///     CacheType --> |Copy| Copy
///     CacheType --> |Symlink| Symlink
///     CacheType --> |Hardlink| Hardlink
///     CacheType --> |Reflink| Reflink
/// ```
pub fn cmd_track(
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    cli_opts: TrackCLI,
) -> Result<()> {
    let conf = xvc_root.config();
    let opts = cli_opts.update_from_conf(conf)?;
    let current_dir = conf.current_dir()?;
    let targets = targets_from_disk(xvc_root, current_dir, &opts.targets)?;
    let requested_cache_type = opts.cache_type.unwrap_or_default();
    let text_or_binary = opts.text_or_binary.unwrap_or_default();
    let no_parallel = opts.no_parallel;

    let stored_xvc_path_store = xvc_root.load_store::<XvcPath>()?;
    let stored_xvc_metadata_store = xvc_root.load_store::<XvcMetadata>()?;

    let xvc_path_metadata_diff = diff_xvc_path_metadata(
        xvc_root,
        &stored_xvc_path_store,
        &stored_xvc_metadata_store,
        &targets,
    );

    let xvc_path_diff = xvc_path_metadata_diff.0;
    let xvc_metadata_diff = xvc_path_metadata_diff.1;

    let changed_entities: HashSet<XvcEntity> =
        xvc_path_diff
            .iter()
            .filter_map(|(xe, xpd)| if xpd.changed() { Some(*xe) } else { None })
            .chain(xvc_metadata_diff.iter().filter_map(|(xe, xpd)| {
                if xpd.changed() {
                    Some(*xe)
                } else {
                    None
                }
            }))
            .collect();

    let stored_cache_type_store = xvc_root.load_store::<CacheType>()?;
    let cache_type_diff = diff_cache_type(
        &stored_cache_type_store,
        requested_cache_type,
        &changed_entities,
    );

    let stored_text_or_binary_store = xvc_root.load_store::<FileTextOrBinary>()?;
    let text_or_binary_diff = diff_text_or_binary(
        &stored_text_or_binary_store,
        text_or_binary,
        &changed_entities,
    );

    let hash_algorithm = HashAlgorithm::from_conf(conf);

    let stored_content_digest_store = xvc_root.load_store::<ContentDigest>()?;
    let prerequisite_diffs = DiffStore3::<XvcPath, XvcMetadata, FileTextOrBinary>(
        xvc_path_diff,
        xvc_metadata_diff,
        text_or_binary_diff,
    );
    let content_digest_diff = diff_content_digest(
        output_snd,
        xvc_root,
        &stored_xvc_path_store,
        &stored_content_digest_store,
        &stored_text_or_binary_store,
        &prerequisite_diffs,
        opts.text_or_binary,
        None,
        !no_parallel,
    );

    let xvc_path_diff = prerequisite_diffs.0;
    let xvc_metadata_diff = prerequisite_diffs.1;
    let text_or_binary_diff = prerequisite_diffs.2;
    update_store_records(xvc_root, &xvc_path_diff, true, false)?;
    update_store_records(xvc_root, &xvc_metadata_diff, true, false)?;
    update_store_records(xvc_root, &cache_type_diff, true, false)?;
    update_store_records(xvc_root, &text_or_binary_diff, true, false)?;
    update_store_records(xvc_root, &content_digest_diff, true, false)?;

    // We reload to get the latest file types
    let current_xvc_metadata_store = xvc_root.load_store::<XvcMetadata>()?;

    let file_entities = changed_entities.iter().filter_map(|xe| {
        current_xvc_metadata_store.get(xe).and_then(|md| {
            if md.file_type == XvcFileType::File {
                Some(*xe)
            } else {
                None
            }
        })
    });

    let directory_entities = changed_entities.iter().filter_map(|xe| {
        current_xvc_metadata_store.get(xe).and_then(|md| {
            if md.file_type == XvcFileType::Directory {
                Some(*xe)
            } else {
                None
            }
        })
    });

    let current_xvc_path_store = xvc_root.load_store::<XvcPath>()?;

    let file_targets: Vec<PathBuf> = file_entities
        .filter_map(|xe| {
            current_xvc_path_store
                .get(&xe)
                .and_then(|xp| Some(xp.to_absolute_path(xvc_root).to_path_buf()))
        })
        .collect();

    let dir_targets: Vec<PathBuf> = directory_entities
        .filter_map(|xe| {
            current_xvc_path_store
                .get(&xe)
                .and_then(|xp| Some(xp.to_absolute_path(xvc_root).to_path_buf()))
        })
        .collect();

    let current_gitignore = build_gitignore(xvc_root)?;

    update_gitignores(
        xvc_root,
        current_dir,
        &current_gitignore,
        &file_targets,
        &dir_targets,
    )?;

    if !opts.no_commit {
        let updated_content_digest_store: HStore<ContentDigest> = content_digest_diff
            .into_iter()
            .filter_map(|(xe, cdd)| match cdd {
                xvc_core::Diff::Identical => None,
                xvc_core::Diff::RecordMissing { actual } => Some((xe, actual)),
                xvc_core::Diff::ActualMissing { record } => None,
                xvc_core::Diff::Different { record, actual } => Some((xe, actual)),
                xvc_core::Diff::Skipped => None,
            })
            .collect();

        let xvc_paths_to_carry =
            current_xvc_path_store.subset(updated_content_digest_store.keys().cloned())?;

        let cache_paths = updated_content_digest_store
            .iter()
            .filter_map(|(xe, cd)| {
                current_xvc_path_store
                    .get(&xe)
                    .and_then(|xp| XvcCachePath::new(xp, cd).ok())
                    .and_then(|cp| Some((*xe, cp)))
            })
            .collect();

        let cache_type_store = xvc_root.load_store::<CacheType>()?;

        carry_in(
            output_snd,
            xvc_root,
            &xvc_paths_to_carry,
            &cache_paths,
            &cache_type_store,
            !no_parallel,
        )?;
    }
    Ok(())
}

/* fn commit( */
/*     output_snd: &Sender<XvcOutputLine>, */
/*     xvc_root: &XvcRoot, */
/*     path_comparison_params: &PathComparisonParams, */
/*     path_delta_store: &FileDeltaStore, */
/*     force: bool, */
/*     parallel: bool, */
/*     algorithm: HashAlgorithm, */
/*     text_or_binary: &DataTextOrBinary, */
/*     cache_type: CacheType, */
/* ) -> Result<()> { */
/*     let checkout = |xp: &XvcPath, digest: &ContentDigest| -> Result<()> { */
/*         let cache_path = cache_path(xp, &digest); */
/*         if !cache_path.to_absolute_path(xvc_root).exists() { */
/*             move_to_cache(xvc_root, xp, &cache_path)?; */
/*             recheck_from_cache(xvc_root, xp, &cache_path, cache_type)?; */
/*             let _ = &output_snd.send(XvcOutputLine::Info(format!( */
/*                 "[COMMIT] {xp} -> {}", */
/*                 cache_path */
/*             )))?; */
/*         } */
/*         Ok(()) */
/*     }; */
/*  */
/*     let force_checkout = |xp: &XvcPath, digest: &ContentDigest| -> Result<()> { */
/*         let cache_path = cache_path(&xp, &digest); */
/*         if !cache_path.to_absolute_path(xvc_root).exists() { */
/*             let abs_path = xp.to_absolute_path(xvc_root); */
/*             fs::remove_file(&abs_path)?; */
/*             let _ = &output_snd.send(XvcOutputLine::Info(format!("[DELETE] {xp}")))?; */
/*             move_to_cache(xvc_root, &xp, &cache_path)?; */
/*             recheck_from_cache(xvc_root, &xp, &cache_path, cache_type)?; */
/*             let _ = &output_snd.send(XvcOutputLine::Info(format!( */
/*                 "[CHECKOUT] {xp} -> {abs_path}" */
/*             )))?; */
/*         } */
/*         Ok(()) */
/*     }; */
/*  */
/*     let inner = |(xe, pd): (&XvcEntity, &FileDelta)| -> Result<()> { */
/*         let xp = path_comparison_params.xvc_path_store[xe].clone(); */
/*         match pd.delta_content_digest { */
/*             DeltaField::Identical | DeltaField::Skipped => { */
/*                 let record_digest = path_comparison_params.content_digest_store[xe]; */
/*  */
/*                 match pd.delta_cache_type { */
/*                     DeltaField::Identical | DeltaField::Skipped => { */
/*                         debug!("No change to checkout: {}", xp); */
/*                         Ok(()) */
/*                     } */
/*                     // We assume the record is created before, in update records. */
/*                     // So this is actually no "RecordMissing" */
/*                     DeltaField::RecordMissing { .. } => force_checkout(&xp, &record_digest), */
/*                     DeltaField::ActualMissing { .. } => force_checkout(&xp, &record_digest), */
/*                     DeltaField::Different { .. } => force_checkout(&xp, &record_digest), */
/*                 } */
/*             } */
/*             // We assume the record is created before, in update records. */
/*             // So this is actually no "RecordMissing" */
/*             DeltaField::RecordMissing { actual } => checkout(&xp, &actual), */
/*             DeltaField::ActualMissing { record } => checkout(&xp, &record), */
/*             DeltaField::Different { record, .. } => { */
/*                 if force { */
/*                     force_checkout(&xp, &record) */
/*                 } else { */
/*                     output_snd.send(XvcOutputLine::Error(format!( */
/*                         "Changes in {xp} are not cached. Use --force to overwrite" */
/*                     )))?; */
/*                     Ok(()) */
/*                 } */
/*             } */
/*         } */
/*     }; */
/*  */
/*     if parallel { */
/*         path_delta_store.par_iter().for_each(|p| { */
/*             inner(p) */
/*                 .map_err(|e| Error::from(e).error()) */
/*                 .unwrap_or_else(|_| ()); */
/*         }); */
/*     } else { */
/*         path_delta_store.iter().for_each(|p| { */
/*             inner(p) */
/*                 .map_err(|e| Error::from(e).error()) */
/*                 .unwrap_or_else(|_| ()); */
/*         }); */
/*     } */
/*  */
/*     Ok(()) */
/* } */
/*  */
/// Write file and directory names to .gitignore found in the same dir
///
/// If `current_ignore` already ignores a file, it's not added separately.
/// If the user chooses to ignore a files manually by general rules, files are not added here.
///
fn update_gitignores(
    xvc_root: &XvcRoot,
    current_dir: &AbsolutePath,
    current_ignore: &IgnoreRules,
    files: &[PathBuf],
    dirs: &[PathBuf],
) -> Result<()> {
    // Check if dirs are already ignored
    let dir_map: HashMap<PathBuf, PathBuf> = dirs
        .iter()
        .filter_map(|f| {
            let abs_path = if f.ends_with("/") {
                current_dir.join(f)
            } else {
                current_dir.join(format!("{}/", f.to_string_lossy()))
            };

            let ignore_res = check_ignore(current_ignore, &abs_path);

            match ignore_res {
                MatchResult::Ignore => {
                    info!("Path is already gitignored: {}", abs_path.to_string_lossy());
                    None
                }
                MatchResult::NoMatch => {
                    Some((f.clone(),
                          f.parent()
                            .map(|p| p.join(".gitignore"))
                            .unwrap_or_else(|| PathBuf::from(".gitignore"))))
                }
                MatchResult::Whitelist => {
                    error!("Path is whitelisted in Git. Please remove/modify the whitelisting rule: {}",
                        abs_path.to_string_lossy());
                    None
                }
            }}).collect();

    watch!(dir_map);

    // Check if files are already ignored
    let file_map: HashMap<PathBuf, PathBuf> = files
        .iter()
        // filter if the directories we'll add already contains these files
        .filter(|f| {
            for (dir, _) in &dir_map {
                if f.starts_with(dir) {
                    return false }
            }
            true
        })
        .filter_map(|f| {
                    let abs_path = current_dir.join(f);

            match check_ignore(current_ignore, &abs_path) {
                MatchResult::NoMatch => {

                    Some((f.clone(),
                          f.parent()
                            .map(|p| p.join(".gitignore"))
                            .unwrap_or_else(|| PathBuf::from(".gitignore"))))
                }
                MatchResult::Ignore => {
                    info!("Already gitignored: {}", &abs_path.to_string_lossy());
                    None
                }
                MatchResult::Whitelist => {
                    error!("Path is whitelisted in Gitignore, please modify/remove the whitelisting rule: {}", &abs_path.to_string_lossy());
                None
            }}
            })
        .collect();

    watch!(file_map);

    let mut changes = HashMap::<PathBuf, Vec<String>>::new();

    for (f, gi) in file_map {
        if !changes.contains_key(&gi) {
            changes.insert(gi.clone(), Vec::<String>::new());
        }

        let path_v = changes.get_mut(&gi).unwrap();
        path_v.push(
            f.file_name()
                .map(|f| format!("/{}", f.to_string_lossy()))
                .unwrap_or_else(|| "## Path Contains final ..".to_string()),
        );
    }
    for (d, gi) in dir_map {
        if !changes.contains_key(&gi) {
            changes.insert(gi.clone(), Vec::<String>::new());
        }

        let path_v = changes.get_mut(&gi).unwrap();
        path_v.push(
            d.file_name()
                .map(|d| format!("/{}/", d.to_string_lossy()))
                .unwrap_or_else(|| "## Path Contains final ..".to_string()),
        );
    }

    for (gitignore_file, values) in changes {
        let append_str = format!(
            "### Following {} lines are added by xvc on {}\n{}",
            values.len(),
            Utc::now().to_rfc2822(),
            values.join("\n")
        );
        let gitignore_path = xvc_root.absolute_path().join(gitignore_file);

        let mut file_o = OpenOptions::new()
            .create(true)
            .append(true)
            .open(gitignore_path)?;

        writeln!(file_o, "{}", append_str)?;
    }

    Ok(())
}
