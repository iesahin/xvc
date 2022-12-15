use chrono::Utc;
use crossbeam_channel::{bounded, Sender};
use derive_more::{AsRef, Deref, Display, From, FromStr};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;

use xvc_config::{conf, FromConfigKey};
use xvc_config::{UpdateFromXvcConfig, XvcConfig};
use xvc_core::util::git::build_gitignore;
use xvc_core::util::xvcignore::COMMON_IGNORE_PATTERNS;
use xvc_core::{
    all_paths_and_metadata, MetadataDigest, XvcCachePath, XvcFileType, XvcPathMetadataMap,
    CHANNEL_BOUND,
};
use xvc_core::{CollectionDigest, ContentDigest, HashAlgorithm};
use xvc_core::{XvcRoot, XVCIGNORE_FILENAME};
use xvc_logging::{error, info, output, uwo, uwr, warn, watch, XvcOutputLine};
use xvc_walker::{
    check_ignore, walk_parallel, AbsolutePath, Glob, GlobSet, GlobSetBuilder, IgnoreRules,
    MatchResult, WalkOptions,
};

use crate::common::compare::{
    diff_cache_type, diff_content_digest, diff_text_or_binary, diff_xvc_path_metadata, Diff, Diff3,
    PathComparisonParams,
};
use crate::common::{
    decide_no_parallel, expand_xvc_dir_file_targets, move_to_cache, pathbuf_to_xvc_target,
    recheck_from_cache, split_file_directory_targets, targets_from_store, update_file_records,
};
use crate::error::{Error, Result};
use crate::recheck::recheck_serial;
use crate::track::FileTextOrBinary;

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
///
/// Carry in (commit) changed files/directories to the cache.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case", version, author)]
pub struct CarryInCLI {
    /// Calculate digests as text or binary file without checking contents, or by automatically. (Default:
    /// auto)
    #[arg(long)]
    text_or_binary: Option<FileTextOrBinary>,
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
    targets: Option<Vec<String>>,
}

impl UpdateFromXvcConfig for CarryInCLI {
    /// Updates `xvc file` configuration from the configuration files.
    /// Command line options take precedence over other sources.
    /// If options are not given, they are supplied from [XvcConfig]
    fn update_from_conf(self, conf: &XvcConfig) -> xvc_config::error::Result<Box<Self>> {
        let force = self.force || conf.get_bool("file.carry-in.force")?.option;
        let no_parallel = self.no_parallel || conf.get_bool("file.carry-in.no_parallel")?.option;
        let text_or_binary = self.text_or_binary.as_ref().map_or_else(
            || Some(FileTextOrBinary::from_conf(conf)),
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
///

pub fn cmd_carry_in(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    cli_opts: CarryInCLI,
) -> Result<()> {
    let conf = xvc_root.config();
    let opts = cli_opts.update_from_conf(conf)?;

    targets = targets_from_store(xvc_root, opts.targets)?;

    let text_or_binary = opts.text_or_binary.unwrap_or_default();
    let no_parallel = opts.no_parallel;

    // We only get the targets' actual metadata to see what has changed
    // This prevents to traverse all files in the repository.
    let target_xvc_path_metadata_map: HStore<XvcMetadata> = target_xvc_path
        .par_iter()
        .map(|(xe, xp)| {
            let p = xp.to_absolute_path(xvc_root);
            let xmd = XvcMetadata::from(p.metadata());
            (xp.clone(), xmd)
        })
        .collect();

    let xvc_path_metadata_diff =
        diff_xvc_path_metadata(&xvc_path_store, &xvc_metadata_store, &target_xvc_metadata);

    let stored_text_or_binary_store: XvcStore<FileTextOrBinary> = xvc_root.load_store()?;
    let text_or_binary_diff = diff_text_or_binary(
        &stored_text_or_binary_store,
        text_or_binary,
        HashSet::from_iter(target_xvc_path.keys().copied()),
    );

    let stored_content_digest_store: XvcStore<ContentDigest> = xvc_root.load_store()?;

    let preprequisite_diffs = Diff3::new(
        xvc_path_metadata_diff.0,
        xvc_path_metadata_diff.1,
        text_or_binary_diff,
    );

    let content_digest_diff = diff_content_digest(
        xvc_root,
        &target_xvc_path,
        &stored_content_digest_store,
        &stored_text_or_binary_store,
        prerequisite_diffs,
        requested_text_or_binary,
        requested_hash_algorithm,
        !opts.no_parallel,
    );

    let xvc_paths_to_carry = if opts.force {
        target_xvc_path
    } else {
        target_xvc_path.filter(|xe, _| {
            content_digest_diff[&xe].changed() || text_or_binary_diff[&xe].changed()
        })
    };

    let cache_paths_to_carry: HStore<XvcCachePath> = xvc_paths_to_carry
        .iter()
        .filter_map(|(xe, xp)| match content_digest_diff[&xe] {
            Diff::Identical | Diff::Skipped => {
                // use stored digest for cache path
                info!(output_snd, "[FORCE] {xp} is identical to cached copy.");
                let digest = stored_content_digest_store.get(xe).unwrap();
                Some((xe, cache_path_from_digest(xvc_root, digest)))
            }
            Diff::ActualMissing { record } => {
                // carry-in shouldn't be used to delete files from cache
                warn!(
                    output_snd,
                    "{xp} is deleted from workspace. Not deleting cached copy. Use `xvc file delete` if you want to delete {xp}.");
                None
            }
            Diff::RecordMissing { actual } => {
                // carry-in shouldn't be used to track new files.
                // This is a bug in the code.
                warn!(output_snd, "Record missing for {:?}. This is a bug. Please report.", xp);
            }
            Diff::Different { actual, .. } => {
                // use actual digest for cache path
                info!(
                    output_snd,
                    "[CHANGED] {xp}");
                Some((xe, cache_path_from_digest(xvc_root, actual)))
            }
        })
        .collect();

    let stored_cache_type_store = xvc_root.load_store::<CacheType>()?;

    carry_in(
        output_snd,
        xvc_root,
        &xvc_paths_to_carry,
        &cache_paths_to_carry,
        &stored_cache_type_store,
        !opts.no_parallel,
    )?;

    update_file_records(xvc_root, &file_delta_store)?;

    Ok(())
}

/// Move targets to the cache if there are any content changes, or if `force` is true.
/// Returns the store of carried in elements. These should be rechecked to the
/// remote.
pub fn carry_in(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    xvc_paths: &XvcStore<XvcPath>,
    cache_paths: &HStore<XvcCachePath>,
    cache_types: &XvcStore<CacheType>,
    parallel: bool,
) -> Result<HStore<XvcPath>> {
    let copy_path_to_cache_and_recheck = |xe, xp| {
        uwr!(move_to_cache(xvc_root, xp, &cache_path), output_snd);
        let cache_type = uwo!(cache_types.get(xe).cloned(), output_snd);
        info!(output_snd, "[CARRY] {xp} -> {cp}");
        uwr!(
            recheck_from_cache(xvc_root, xp, cache_path, cache_type),
            output_snd
        );
        info!(output_snd, "[RECHECK] {cp} -> {xp}");
    };

    if parallel {
        carry_in_paths
            .par_iter()
            .for_each(|(xe, xp)| copy_path_to_cache_and_recheck(xe, xp));
    } else {
        carry_in_paths
            .iter()
            .for_each(|(xe, xp)| copy_path_to_cache_and_recheck(xe, xp));
    }

    Ok(carry_in_paths)
}
