use chrono::Utc;
use crossbeam_channel::{bounded, Sender};
use derive_more::{AsRef, Deref, Display, From, FromStr};
use log::{debug, error, info, warn};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;

use xvc_config::{conf, FromConfigKey};
use xvc_config::{UpdateFromXvcConfig, XvcConfig};
use xvc_core::util::git::build_gitignore;
use xvc_core::util::xvcignore::COMMON_IGNORE_PATTERNS;
use xvc_core::{
    all_paths_and_metadata, MetadataDigest, XvcFileType, XvcPathMetadataMap, CHANNEL_BOUND,
};
use xvc_core::{CollectionDigest, ContentDigest, HashAlgorithm};
use xvc_core::{XvcRoot, XVCIGNORE_FILENAME};
use xvc_logging::{watch, XvcOutputLine};
use xvc_walker::{
    check_ignore, walk_parallel, AbsolutePath, IgnoreRules, MatchResult, WalkOptions,
};

use crate::common::compare::{
    find_dir_changes_serial, find_file_changes_parallel, find_file_changes_serial,
    update_path_comparison_params_with_actual_info, DeltaField, DirectoryDelta,
    DirectoryDeltaStore, FileDelta, FileDeltaStore, PathComparisonParams,
};
use crate::common::{cache_path, checkout_from_cache, move_to_cache};
use crate::error::{Error, Result};

use std::fs::{self, OpenOptions};

use clap::Parser;
use std::path::PathBuf;

use xvc_core::CacheType;
use xvc_core::TextOrBinary;
use xvc_core::XvcMetadata;
use xvc_core::XvcPath;
use xvc_ecs::XvcEntity;
use xvc_ecs::{persist, HStore, XvcStore};
use xvc_ecs::{R11Store, Storable};

/// Represents whether a file is a text file or not
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Hash,
    Display,
    FromStr,
    From,
    AsRef,
    Deref,
    Default,
    Copy,
)]
pub struct DataTextOrBinary(TextOrBinary);
conf!(DataTextOrBinary, "file.add.text_or_binary");
persist!(DataTextOrBinary, "data-text-or-binary");

impl DataTextOrBinary {
    pub fn as_inner(&self) -> TextOrBinary {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(about = "Track file versions using XVC", rename_all = "kebab-case")]
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
    text_or_binary: Option<DataTextOrBinary>,
    /// Add targets even if they are already tracked
    #[arg(long)]
    force: bool,
    /// Don't use parallelism
    #[arg(long)]
    no_parallel: bool,
    /// Files/directories to add
    #[arg()]
    targets: Vec<PathBuf>,
}

impl UpdateFromXvcConfig for TrackCLI {
    fn update_from_conf(self, conf: &XvcConfig) -> xvc_config::error::Result<Box<Self>> {
        let cache_type = self
            .cache_type
            .unwrap_or_else(|| CacheType::from_conf(conf));
        let no_commit = self.no_commit || conf.get_bool("file.add.no_commit")?.option;
        let force = self.force || conf.get_bool("file.add.force")?.option;
        let no_parallel = self.no_parallel || conf.get_bool("file.add.no_parallel")?.option;
        let text_or_binary = self.text_or_binary.as_ref().map_or_else(
            || Some(DataTextOrBinary::from_conf(conf)),
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

const PARALLEL_THRESHOLD: usize = 47;

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
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    cli_opts: TrackCLI,
) -> Result<()> {
    let conf = xvc_root.config();
    let opts = cli_opts.update_from_conf(conf)?;
    let current_dir = conf.current_dir()?;
    let targets: Vec<PathBuf> = opts.targets.iter().map(|t| current_dir.join(t)).collect();
    let cache_type = opts.cache_type.unwrap_or_default();
    let text_or_binary = opts.text_or_binary.unwrap_or_default();

    let no_parallel = opts.no_parallel
        || (targets.iter().all(|t| t.is_file()) && targets.len() < PARALLEL_THRESHOLD);

    let (xpmm, xvc_ignore) = all_paths_and_metadata(xvc_root);
    let to_xvc_target = |targets: &Vec<PathBuf>| -> Vec<XvcPath> {
        targets
            .into_iter()
            .filter_map(|t| {
                if t.is_file() || t.is_dir() {
                    Some(t)
                } else {
                    output_snd
                        .send(format!("Unsupported Target Type: {}", t.to_string_lossy()).into())
                        .unwrap();
                    None
                }
            })
            .filter(|t| {
                let ignore_result = check_ignore(&xvc_ignore, t);

                match ignore_result {
                    MatchResult::Ignore => {
                        warn!("Ignored: {}", t.to_string_lossy());
                        false
                    }
                    MatchResult::Whitelist => {
                        info!("Whitelisted: {}", t.to_string_lossy());
                        true
                    }
                    MatchResult::NoMatch => true,
                }
            })
            .map(|t| XvcPath::new(xvc_root, current_dir, &t))
            .filter_map(|res_xp| match res_xp {
                Ok(xp) => Some(xp),
                Err(e) => {
                    error!("{}", e);
                    None
                }
            })
            .collect()
    };

    let xvc_targets = to_xvc_target(&targets);
    let mut given_dir_targets = XvcPathMetadataMap::new();
    let mut file_targets = XvcPathMetadataMap::new();

    for xvc_target in xvc_targets {
        if let Some(xmd) = xpmm.get(&xvc_target) {
            match xmd.file_type {
                XvcFileType::RecordOnly => {
                    output_snd
                        .send(XvcOutputLine::Error(format!(
                            "Target not found: {xvc_target}"
                        )))
                        .unwrap();
                }
                XvcFileType::File => {
                    file_targets.insert(xvc_target, xmd.clone());
                }
                XvcFileType::Directory => {
                    given_dir_targets.insert(xvc_target, xmd.clone());
                }
                XvcFileType::Symlink => output_snd
                    .send(XvcOutputLine::Error(format!(
                        "Symlinks are not supported: {xvc_target}"
                    )))
                    .unwrap(),
                XvcFileType::Hardlink => output_snd
                    .send(XvcOutputLine::Error(format!(
                        "Hardlinks are not supported: {xvc_target}"
                    )))
                    .unwrap(),
                XvcFileType::Reflink => output_snd
                    .send(XvcOutputLine::Error(format!(
                        "Reflinks are not supported: {xvc_target}"
                    )))
                    .unwrap(),
            }
        } else {
            output_snd
                .send(XvcOutputLine::Warn(format!(
                    "Ignored or not found: {xvc_target}"
                )))
                .unwrap();
        }
    }

    // Add all paths under directory targets

    let mut dir_targets = XvcPathMetadataMap::new();

    for (dir_target, dir_md) in &given_dir_targets {
        for (xvc_path, xvc_md) in &xpmm {
            if xvc_path.starts_with(&dir_target) && *xvc_path != *dir_target {
                match xvc_md.file_type {
                    XvcFileType::Directory => {
                        dir_targets.insert(xvc_path.clone(), xvc_md.clone());
                    }
                    XvcFileType::File => {
                        file_targets.insert(xvc_path.clone(), xvc_md.clone());
                    }
                    _ => {
                        output_snd.send(XvcOutputLine::Error(format!(
                            "Unsupported Target: {xvc_path}"
                        )));
                    }
                }
            }
        }
        dir_targets.insert(dir_target.clone(), dir_md.clone());
    }

    // create entities for new paths
    //
    // NOTE: We don't create metadata records here, otherwise FileDelta and DirectoryDelta
    // operations doesn't catch the differences.
    //
    // We only create entities for paths.
    // Their metadata components will be missing and will be caught as differences.

    xvc_root.with_store_mut(|xvc_path_store: &mut XvcStore<XvcPath>| {
        let mut xvc_path_imap = xvc_path_store.index_map()?;

        for (dir_target, dir_md) in &dir_targets {
            let opt_entity = xvc_path_imap.get(dir_target);
            if opt_entity == None {
                let new_target_e = xvc_root.new_entity();
                xvc_path_store.insert(new_target_e, dir_target.to_owned().clone());
                xvc_path_imap.insert(dir_target.to_owned().clone(), new_target_e);
            }
        }

        for (file_target, file_md) in &file_targets {
            let opt_entity = xvc_path_imap.get(file_target);
            if opt_entity == None {
                let new_target_e = xvc_root.new_entity();
                xvc_path_store.insert(new_target_e, file_target.to_owned().clone());
                xvc_path_imap.insert(file_target.to_owned().clone(), new_target_e);
            }
        }

        Ok(())
    })?;

    let path_comparison_params = PathComparisonParams::init(xvc_root)?;
    let algorithm = (&path_comparison_params.algorithm).clone();

    info!("Calculating Hashes with: {:#?}", algorithm);
    let file_delta_store = if no_parallel {
        find_file_changes_serial(
            xvc_root,
            &path_comparison_params,
            &cache_type,
            &text_or_binary,
            &file_targets,
        )?
    } else {
        find_file_changes_parallel(
            xvc_root,
            &path_comparison_params,
            &cache_type,
            &text_or_binary,
            &file_targets,
        )?
    };

    // TODO: Update the dir changes function with parallel version
    // Used serial version because it works with in memory data without making IO
    let path_comparison_params =
        update_path_comparison_params_with_actual_info(path_comparison_params, &file_delta_store);

    let dir_delta_store: DirectoryDeltaStore =
        find_dir_changes_serial(xvc_root, &path_comparison_params, &given_dir_targets)?;

    update_file_records(xvc_root, &file_delta_store)?;
    update_dir_records(xvc_root, &dir_delta_store)?;

    let current_gitignore = build_gitignore(xvc_root)?;

    update_gitignores(
        xvc_root,
        current_dir,
        &current_gitignore,
        file_targets
            .iter()
            .map(|(xp, _)| xp.to_absolute_path(xvc_root).to_path_buf())
            .collect::<Vec<PathBuf>>()
            .as_ref(),
        given_dir_targets
            .iter()
            .map(|(xp, _)| xp.to_absolute_path(xvc_root).to_path_buf())
            .collect::<Vec<PathBuf>>()
            .as_ref(),
    )?;

    if !opts.no_commit {
        commit(
            output_snd,
            xvc_root,
            &path_comparison_params,
            &file_delta_store,
            opts.force,
            !opts.no_parallel,
            algorithm,
            &text_or_binary,
            cache_type,
        )?;
    }
    Ok(())
}

/// Record store records checking their DeltaField status
fn update_store_records<T>(xvc_root: &XvcRoot, delta_store: HStore<&DeltaField<T>>) -> Result<()>
where
    T: Storable,
{
    xvc_root.with_store_mut(|store: &mut XvcStore<T>| {
        for (xe, dd) in delta_store.iter() {
            match dd {
                DeltaField::Identical | DeltaField::Skipped => {
                    info!("Not changed: {:?}", xe);
                }

                DeltaField::RecordMissing { actual } => {
                    store.insert(*xe, actual.clone());
                }
                DeltaField::ActualMissing { .. } => {
                    info!("Record not changed. {}", xe);
                }
                DeltaField::Different { actual, .. } => {
                    store.insert(*xe, actual.clone());
                }
            }
        }
        Ok(())
    })?;

    Ok(())
}

fn update_dir_records(xvc_root: &XvcRoot, dir_delta_store: &HStore<DirectoryDelta>) -> Result<()> {
    let collection_delta_store: HStore<&DeltaField<CollectionDigest>> = dir_delta_store
        .iter()
        .map(|(xe, dd)| (*xe, &dd.delta_collection_digest))
        .collect();
    update_store_records(xvc_root, collection_delta_store)?;

    let metadata_digest_delta_store: HStore<&DeltaField<MetadataDigest>> = dir_delta_store
        .iter()
        .map(|(xe, dd)| (*xe, &dd.delta_metadata_digest))
        .collect();
    update_store_records(xvc_root, metadata_digest_delta_store)?;

    let content_delta_store: HStore<&DeltaField<ContentDigest>> = dir_delta_store
        .iter()
        .map(|(xe, dd)| (*xe, &dd.delta_content_digest))
        .collect();
    update_store_records(xvc_root, content_delta_store)?;

    let metadata_delta_store: HStore<&DeltaField<XvcMetadata>> = dir_delta_store
        .iter()
        .map(|(xe, dd)| (*xe, &dd.delta_xvc_metadata))
        .collect();
    update_store_records(xvc_root, metadata_delta_store)?;

    Ok(())
}

/// Record changes in `path_delta_store` to various stores in `xvc_root`
/// TODO: Refactor using [update_store_records]
fn update_file_records(xvc_root: &XvcRoot, path_delta_store: &FileDeltaStore) -> Result<()> {
    xvc_root.with_r11store_mut(|xp_md_r11s: &mut R11Store<XvcPath, XvcMetadata>| {
        for (xe, pd) in path_delta_store.iter() {
            let xp = xp_md_r11s.left[xe].clone();
            match pd.delta_md {
                DeltaField::Identical | DeltaField::Skipped => {
                    info!("Not changed: {}", xp);
                }
                DeltaField::RecordMissing { actual } => {
                    xp_md_r11s.insert(&xe, xp, actual);
                }
                // TODO: Think about changing XvcMetadata to `RecordOnly` in this case.
                DeltaField::ActualMissing { .. } => {
                    info!("File not found. {} Record not changed.", xp);
                }
                DeltaField::Different { actual, .. } => {
                    xp_md_r11s.insert(&xe, xp, actual);
                }
            }
        }
        Ok(())
    })?;

    xvc_root.with_r11store_mut(
        |xp_content_digest_r11s: &mut R11Store<XvcPath, ContentDigest>| {
            for (xe, pd) in path_delta_store.iter() {
                let xp = xp_content_digest_r11s.left[xe].clone();
                match pd.delta_content_digest {
                    DeltaField::Identical | DeltaField::Skipped => {
                        info!("Not changed: {}", xp);
                    }
                    DeltaField::RecordMissing { actual } => {
                        xp_content_digest_r11s.insert(&xe, xp, actual);
                    }
                    // TODO: Think about deleting the record in this case.
                    DeltaField::ActualMissing { .. } => {
                        info!("File not found. {} Record not changed.", xp);
                    }
                    DeltaField::Different { actual, .. } => {
                        xp_content_digest_r11s.insert(&xe, xp, actual);
                    }
                }
            }
            Ok(())
        },
    )?;

    xvc_root.with_r11store_mut(
        |xp_metadata_digest_r11s: &mut R11Store<XvcPath, MetadataDigest>| {
            for (xe, pd) in path_delta_store.iter() {
                let xp = xp_metadata_digest_r11s.left[xe].clone();
                match pd.delta_metadata_digest {
                    DeltaField::Identical | DeltaField::Skipped => {
                        info!("Not changed: {}", xp);
                    }
                    DeltaField::RecordMissing { actual } => {
                        xp_metadata_digest_r11s.insert(&xe, xp, actual);
                    }
                    // TODO: Think about deleting the record in this case.
                    DeltaField::ActualMissing { .. } => {
                        info!("File not found. {} Record not changed.", xp);
                    }
                    DeltaField::Different { actual, .. } => {
                        xp_metadata_digest_r11s.insert(&xe, xp, actual);
                    }
                }
            }
            Ok(())
        },
    )?;

    xvc_root.with_r11store_mut(|xp_ct_r11s: &mut R11Store<XvcPath, CacheType>| {
        for (xe, pd) in path_delta_store.iter() {
            let xp = xp_ct_r11s.left[xe].clone();
            match pd.delta_cache_type {
                DeltaField::ActualMissing { .. } | DeltaField::Identical | DeltaField::Skipped => {}
                DeltaField::RecordMissing { actual } => {
                    xp_ct_r11s.insert(xe, xp.to_owned(), actual);
                }
                DeltaField::Different { actual, .. } => {
                    xp_ct_r11s.insert(xe, xp.to_owned(), actual);
                }
            }
        }
        Ok(())
    })?;

    xvc_root.with_r11store_mut(|xp_tb_r11s: &mut R11Store<XvcPath, DataTextOrBinary>| {
        for (xe, pd) in path_delta_store.iter() {
            let xp = xp_tb_r11s.left[xe].clone();
            match pd.delta_text_or_binary {
                DeltaField::ActualMissing { .. } | DeltaField::Identical | DeltaField::Skipped => {}
                DeltaField::RecordMissing { actual } => {
                    xp_tb_r11s.insert(xe, xp.to_owned(), actual);
                }
                DeltaField::Different { actual, .. } => {
                    xp_tb_r11s.insert(xe, xp.to_owned(), actual);
                }
            }
        }
        Ok(())
    })?;
    Ok(())
}

fn commit(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    path_comparison_params: &PathComparisonParams,
    path_delta_store: &FileDeltaStore,
    force: bool,
    parallel: bool,
    algorithm: HashAlgorithm,
    text_or_binary: &DataTextOrBinary,
    cache_type: CacheType,
) -> Result<()> {
    let checkout = |xp: &XvcPath, digest: &ContentDigest| -> Result<()> {
        let cache_path = cache_path(xp, &digest);
        if !cache_path.to_absolute_path(xvc_root).exists() {
            move_to_cache(xvc_root, xp, &cache_path)?;
            checkout_from_cache(xvc_root, xp, &cache_path, cache_type)?;
            let _ = &output_snd.send(XvcOutputLine::Info(format!(
                "[COMMIT] {xp} -> {}",
                cache_path
            )))?;
        }
        Ok(())
    };

    let force_checkout = |xp: &XvcPath, digest: &ContentDigest| -> Result<()> {
        let cache_path = cache_path(&xp, &digest);
        if !cache_path.to_absolute_path(xvc_root).exists() {
            let abs_path = xp.to_absolute_path(xvc_root);
            fs::remove_file(&abs_path)?;
            let _ = &output_snd.send(XvcOutputLine::Info(format!("[DELETE] {xp}")))?;
            move_to_cache(xvc_root, &xp, &cache_path)?;
            checkout_from_cache(xvc_root, &xp, &cache_path, cache_type)?;
            let _ = &output_snd.send(XvcOutputLine::Info(format!(
                "[CHECKOUT] {xp} -> {abs_path}"
            )))?;
        }
        Ok(())
    };

    let inner = |(xe, pd): (&XvcEntity, &FileDelta)| -> Result<()> {
        let xp = path_comparison_params.xvc_path_store[xe].clone();
        match pd.delta_content_digest {
            DeltaField::Identical | DeltaField::Skipped => {
                let record_digest = path_comparison_params.content_digest_store[xe];

                match pd.delta_cache_type {
                    DeltaField::Identical | DeltaField::Skipped => {
                        debug!("No change to checkout: {}", xp);
                        Ok(())
                    }
                    // We assume the record is created before, in update records.
                    // So this is actually no "RecordMissing"
                    DeltaField::RecordMissing { .. } => force_checkout(&xp, &record_digest),
                    DeltaField::ActualMissing { .. } => force_checkout(&xp, &record_digest),
                    DeltaField::Different { .. } => force_checkout(&xp, &record_digest),
                }
            }
            // We assume the record is created before, in update records.
            // So this is actually no "RecordMissing"
            DeltaField::RecordMissing { actual } => checkout(&xp, &actual),
            DeltaField::ActualMissing { record } => checkout(&xp, &record),
            DeltaField::Different { record, .. } => {
                if force {
                    force_checkout(&xp, &record)
                } else {
                    output_snd.send(XvcOutputLine::Error(format!(
                        "Changes in {xp} are not cached. Use --force to overwrite"
                    )))?;
                    Ok(())
                }
            }
        }
    };

    if parallel {
        path_delta_store.par_iter().for_each(|p| {
            inner(p)
                .map_err(|e| Error::from(e).error())
                .unwrap_or_else(|_| ());
        });
    } else {
        path_delta_store.iter().for_each(|p| {
            inner(p)
                .map_err(|e| Error::from(e).error())
                .unwrap_or_else(|_| ());
        });
    }

    Ok(())
}

// fn update_cache_type(
//     xvc_root: &XvcRoot,
//     xvc_path: &XvcPath,
//     digest: &XvcDigest,
//     cache_type: CacheType,
// ) -> Result<()> {
//     let cache_path = cache_path(xvc_root, xvc_path, digest);
//     // remove actual path if cache_path exists
//     if !cache_path.exists() {
//         Err(Error::CannotFindFileInCache {
//             xvc_path: xvc_path.to_string(),
//             cache_path: cache_path.to_string_lossy().to_string(),
//         })
//     } else {
//         let path = xvc_path.to_absolute_path(xvc_root);
//         fs::remove_file(path)?;
//         checkout_from_cache(xvc_root, xvc_path, &cache_path, cache_type)
//     }
// }

/// Writes a file names to the .gitignore found in the same dir
fn update_gitignores(
    xvc_root: &XvcRoot,
    current_dir: &AbsolutePath,
    current_ignore: &IgnoreRules,
    files: &[PathBuf],
    dirs: &[PathBuf],
) -> Result<()> {
    let file_map: HashMap<PathBuf, PathBuf> = files
        .iter()
        .filter_map(|f| {
                    let abs_path = current_dir.join(f);

            match check_ignore(current_ignore, &abs_path) {
                MatchResult::NoMatch => {

                    Some((f.clone(),
                          f.parent()
                            .map(|p| p.join(".gitignore"))
                            .unwrap_or_else(|| PathBuf::from(".gitinore"))))
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
                            .unwrap_or_else(|| PathBuf::from(".gitinore"))))
                }
                MatchResult::Whitelist => {
                    error!("Path is whitelisted in Git. Please remove/modify the whitelisting rule: {}",
                        abs_path.to_string_lossy());
                    None
                }
            }}).collect();

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
