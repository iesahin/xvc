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
use crate::common::{
    cache_path, decide_no_parallel, expanded_xvc_dir_file_targets, move_to_cache,
    pathbuf_to_xvc_target, recheck_from_cache, split_file_directory_targets,
};
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

/// Carry in (commit) changed files/directories to the cache.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case", version, author)]
pub struct CarryInCLI {
    /// Calculate digests as text or binary file without checking contents, or by automatically. (Default:
    /// auto)
    #[arg(long)]
    text_or_binary: Option<DataTextOrBinary>,
    /// Carry in targets even their content digests are not changed.
    ///
    /// This removes the file in cache and re-adds it.
    #[arg(long)]
    force: bool,
    /// Don't use parallelism
    #[arg(long)]
    no_parallel: bool,
    /// Files/directories to add
    #[arg()]
    targets: Vec<PathBuf>,
}

impl UpdateFromXvcConfig for CarryInCLI {
    /// Updates `xvc file` configuration from the configuration files.
    /// Command line options take precedence over other sources.
    /// If options are not given, they are supplied from [XvcConfig]
    fn update_from_conf(self, conf: &XvcConfig) -> xvc_config::error::Result<Box<Self>> {
        let force = self.force || conf.get_bool("file.carry-in.force")?.option;
        let no_parallel = self.no_parallel || conf.get_bool("file.carry-in.no_parallel")?.option;
        let text_or_binary = self.text_or_binary.as_ref().map_or_else(
            || Some(DataTextOrBinary::from_conf(conf)),
            |v| Some(v.to_owned()),
        );

        Ok(Box::new(Self {
            targets: self.targets.clone(),
            force,
            no_parallel,
            text_or_binary,
        }))
    }
}
/// Entry point for `xvc file carry-in` command.
///
///
/// ## Pipeline
///
/// ```mermaid
/// graph LR
///     Target --> |File| Path
///     Target -->|Directory| Dir
///     Dir --> |File| Path
///     Dir --> |Directory| Dir
///     Path --> Tracked {Do we track this path?}
///     Tracked --> |Yes| XvcPath
///     Tracked --> |No| Ignore
///     XvcPath --> |Force| XvcDigest
///     XvcPath --> Filter{Is this changed?}
///     XvcPath --> Filter{Is the source a regular file?}
///     Filter -->|Yes| XvcDigest
///     Filter -->|No| Ignore
///     XvcDigest --> CacheLocation
///     
/// ```
pub fn cmd_carry_in(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    cli_opts: CarryInCLI,
) -> Result<()> {
    let conf = xvc_root.config();
    let opts = cli_opts.update_from_conf(conf)?;
    let current_dir = conf.current_dir()?;
    let targets: Vec<PathBuf> = opts.targets.iter().map(|t| current_dir.join(t)).collect();
    let text_or_binary = opts.text_or_binary.unwrap_or_default();

    let no_parallel = decide_no_parallel(opts.no_parallel, opts.targets.as_slice());

    let (dir_targets, file_targets) = expanded_xvc_dir_file_targets(output_snd, xvc_root, targets);
    // Check if we're actually tracking the target
    // Otherwise we inform the user and skip it

    let tracked_file_targets: HashMap<XvcPath, XvcMetadata> = file_targets
        .iter()
        .filter_map(|(xvc_path, xvc_md)| {
            if let (ft) = xvc_md.file_type() {
                match ft {
                    ft::File => Some((xvc_path.clone(), xvc_md.clone())),
                    _ => {
                        info!(
                            output_snd,
                            "Only regular files are carried into the cache: {}", xvc_path
                        );
                        None
                    }
                }
            } else {
                error!(output_snd, "Not tracking file: {}", xvc_path);
                None
            }
        })
        .collect();

    let path_comparison_params = PathComparisonParams::init(xvc_root)?;
    let algorithm = (&path_comparison_params.algorithm).clone();

    info!(output_snd, "Calculating Hashes with: {:#?}", algorithm);
    let file_delta_store = if no_parallel {
        find_file_changes_serial(
            xvc_root,
            &path_comparison_params,
            &cache_type,
            &text_or_binary,
            &tracked_file_targets,
        )?
    } else {
        find_file_changes_parallel(
            xvc_root,
            &path_comparison_params,
            &cache_type,
            &text_or_binary,
            &tracked_file_targets,
        )?
    };

    // TODO: Update the dir changes function with parallel version
    // Used serial version because it works with in memory data without making IO
    let path_comparison_params =
        update_path_comparison_params_with_actual_info(path_comparison_params, &file_delta_store);

    update_file_records(xvc_root, &file_delta_store)?;
    carry_in(
        output_snd,
        xvc_root,
        &path_comparison_params,
        &file_delta_store,
        opts.force,
        !opts.no_parallel,
        algorithm,
        &text_or_binary,
        cache_type,
    )
}

fn carry_in(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    path_comparison_params: &PathComparisonParams,
    path_delta_store: &FileDeltaStore,
    force: bool,
    parallel: bool,
) -> Result<()> {

    let carry_in_paths = if force {
        path_delta_store
            .iter()
            .map(|(xp, _)| xp.clone())
            .collect::<Vec<_>>()
    } else {
        path_delta_store
            .iter()
            .filter_map(|(xp, delta)| {
                if delta.is_changed() {
                    Some(xp.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };
    }

    let checkout = |xp: &XvcPath, digest: &ContentDigest| -> Result<()> {
        let cache_path = cache_path(xp, &digest);
        if !cache_path.to_absolute_path(xvc_root).exists() {
            move_to_cache(xvc_root, xp, &cache_path)?;
            recheck_from_cache(xvc_root, xp, &cache_path, cache_type)?;
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
            recheck_from_cache(xvc_root, &xp, &cache_path, cache_type)?;
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
