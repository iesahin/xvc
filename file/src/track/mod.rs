//! Home of the `xvc file track` command and related functionality.
//!
//! - [`cmd_track`] is the entry point for the `xvc file track` command.
//! - [`TrackCLI`] is the command line interface
//! - [`update_file_gitignores`] and [`update_dir_gitignores`] are functions to
//!   update `.gitignore` files with the tracked paths.
//! - [`carry_in`] is a specialized carry in function for `xvc file track`.
use chrono::Utc;
use crossbeam_channel::Sender;
use derive_more::From;

use std::collections::{HashMap, HashSet};
use std::io::Write;

use xvc_config::FromConfigKey;
use xvc_config::{UpdateFromXvcConfig, XvcConfig};
use xvc_core::util::git::build_gitignore;

use xvc_core::{ContentDigest, HashAlgorithm, XvcCachePath, XvcFileType, XvcMetadata, XvcRoot};
use xvc_logging::{error, info, watch, XvcOutputLine};
use xvc_walker::{check_ignore, AbsolutePath, IgnoreRules, MatchResult};

use crate::carry_in::carry_in;
use crate::common::compare::{
    diff_cache_type, diff_content_digest, diff_text_or_binary, diff_xvc_path_metadata,
};
use crate::common::{targets_from_disk, update_store_records, FileTextOrBinary};
use crate::error::Result;

use std::fs::OpenOptions;

use clap::Parser;
use std::path::PathBuf;

use xvc_core::CacheType;
use xvc_core::XvcPath;
use xvc_ecs::{HStore, XvcEntity};

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
        let no_commit = self.no_commit || conf.get_bool("file.track.no_commit")?.option;
        let force = self.force || conf.get_bool("file.track.force")?.option;
        let no_parallel = self.no_parallel || conf.get_bool("file.track.no_parallel")?.option;
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
    watch!(targets);
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

    let content_digest_diff = diff_content_digest(
        output_snd,
        xvc_root,
        &stored_xvc_path_store,
        &stored_xvc_metadata_store,
        &stored_content_digest_store,
        &stored_text_or_binary_store,
        &xvc_path_diff,
        &xvc_metadata_diff,
        opts.text_or_binary,
        Some(hash_algorithm),
        !no_parallel,
    );

    watch!(content_digest_diff);

    update_store_records(xvc_root, &xvc_path_diff, true, false)?;
    update_store_records(xvc_root, &xvc_metadata_diff, true, false)?;
    update_store_records(xvc_root, &cache_type_diff, true, false)?;
    update_store_records(xvc_root, &text_or_binary_diff, true, false)?;
    update_store_records(xvc_root, &content_digest_diff, true, false)?;

    watch!(targets);
    let file_targets: Vec<PathBuf> = targets
        .iter()
        .filter_map(|(xp, xmd)| {
            if xmd.file_type == XvcFileType::File {
                Some(xp.to_absolute_path(xvc_root).to_path_buf())
            } else {
                None
            }
        })
        .collect();

    // Warning: This one uses `opts.targets` instead of `targets` because
    // `targets` has been filtered to only include files.
    let dir_targets: Vec<PathBuf> = opts
        .targets
        .unwrap_or_else(|| vec![current_dir.to_string()])
        .iter()
        .filter_map(|t| {
            let p = PathBuf::from(t);
            if p.is_dir() {
                Some(p)
            } else {
                None
            }
        })
        .collect();

    let current_gitignore = build_gitignore(xvc_root)?;

    watch!(file_targets);
    watch!(dir_targets);

    update_dir_gitignores(xvc_root, current_dir, &current_gitignore, &dir_targets)?;
    // We reload gitignores here to make sure we ignore the given dirs
    let current_gitignore = build_gitignore(xvc_root)?;
    update_file_gitignores(xvc_root, current_dir, &current_gitignore, &file_targets)?;

    if !opts.no_commit {
        let current_xvc_path_store = xvc_root.load_store::<XvcPath>()?;

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
            opts.force,
        )?;
    }
    Ok(())
}

/// Write file and directory names to .gitignore found in the same dir
///
/// If `current_ignore` already ignores a file, it's not added separately.
/// If the user chooses to ignore a files manually by general rules, files are not added here.
///
fn update_dir_gitignores(
    xvc_root: &XvcRoot,
    current_dir: &AbsolutePath,
    current_gitignore: &IgnoreRules,
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

            let ignore_res = check_ignore(current_gitignore, &abs_path);

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
    let mut changes = HashMap::<PathBuf, Vec<String>>::new();

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

fn update_file_gitignores(
    xvc_root: &XvcRoot,
    current_dir: &AbsolutePath,
    current_gitignore: &IgnoreRules,
    files: &[PathBuf],
) -> Result<()> {
    // Check if files are already ignored
    let file_map: HashMap<PathBuf, PathBuf> = files
        .iter()
        .filter_map(|f| {
                    let abs_path = current_dir.join(f);

            match check_ignore(current_gitignore, &abs_path) {
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
