use crossbeam_channel::Sender;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use std::collections::HashSet;

use xvc_config::FromConfigKey;
use xvc_config::{UpdateFromXvcConfig, XvcConfig};

use xvc_core::ContentDigest;
use xvc_core::XvcRoot;
use xvc_core::{Diff, XvcCachePath};
use xvc_logging::{info, uwo, uwr, warn, watch, XvcOutputLine};

use crate::common::compare::{diff_content_digest, diff_text_or_binary, diff_xvc_path_metadata};
use crate::common::{
    move_to_cache, only_file_targets, recheck_from_cache, targets_from_store,
    xvc_path_metadata_map_from_disk,
};
use crate::common::{update_store_records, FileTextOrBinary};
use crate::error::Result;

use clap::Parser;

use xvc_core::CacheType;

use xvc_core::XvcMetadata;
use xvc_core::XvcPath;

use xvc_ecs::{HStore, XvcStore};

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
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    cli_opts: CarryInCLI,
) -> Result<()> {
    let conf = xvc_root.config();
    let opts = cli_opts.update_from_conf(conf)?;
    watch!(opts);
    let current_dir = conf.current_dir()?;
    let targets = targets_from_store(xvc_root, current_dir, &opts.targets)?;
    watch!(targets);

    let stored_xvc_path_store = xvc_root.load_store::<XvcPath>()?;
    let stored_xvc_metadata_store = xvc_root.load_store::<XvcMetadata>()?;
    let target_files =
        only_file_targets(&stored_xvc_path_store, &stored_xvc_metadata_store, &targets)?;

    let target_xvc_path_metadata_map = xvc_path_metadata_map_from_disk(xvc_root, &target_files);
    let xvc_path_metadata_diff = diff_xvc_path_metadata(
        xvc_root,
        &stored_xvc_path_store,
        &stored_xvc_metadata_store,
        &target_xvc_path_metadata_map,
    );

    let stored_text_or_binary_store: XvcStore<FileTextOrBinary> = xvc_root.load_store()?;
    let text_or_binary_diff = diff_text_or_binary(
        &stored_text_or_binary_store,
        opts.text_or_binary.unwrap_or_default(),
        &HashSet::from_iter(targets.keys().copied()),
    );
    let stored_content_digest_store: XvcStore<ContentDigest> = xvc_root.load_store()?;

    let xvc_path_diff = xvc_path_metadata_diff.0;
    let xvc_metadata_diff = xvc_path_metadata_diff.1;

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
        None,
        !opts.no_parallel,
    );

    watch!(content_digest_diff);

    let xvc_paths_to_carry = if opts.force {
        target_files
    } else {
        let content_digest_diff = &content_digest_diff;

        target_files
            .filter(|xe, _| content_digest_diff[xe].changed() || text_or_binary_diff[xe].changed())
            .cloned()
    };

    let cache_paths_to_carry: HStore<XvcCachePath> = xvc_paths_to_carry
        .iter()
        .filter_map(|(xe, xp)| match content_digest_diff[&xe] {
            Diff::Identical | Diff::Skipped => {
                // use stored digest for cache path
                info!(output_snd, "[FORCE] {xp} is identical to cached copy.");
                let digest = stored_content_digest_store.get(xe).unwrap();
                Some((*xe, uwr!(XvcCachePath::new(xp, digest), output_snd)))
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
                None
            }
            Diff::Different { actual, .. } => {
                // use actual digest for cache path
                info!(
                    output_snd,
                    "[CHANGED] {xp}");
                Some((*xe, uwr!(XvcCachePath::new(xp, &actual), output_snd)))
            }
        })
        .collect();

    let stored_cache_type_store = xvc_root.load_store::<CacheType>()?;
    watch!(xvc_paths_to_carry);
    carry_in(
        output_snd,
        xvc_root,
        &xvc_paths_to_carry,
        &cache_paths_to_carry,
        &stored_cache_type_store,
        !opts.no_parallel,
    )?;

    // We only update the records for existing paths.
    update_store_records(xvc_root, &text_or_binary_diff, false, false)?;
    update_store_records(xvc_root, &content_digest_diff, false, false)?;

    Ok(())
}

/// Move targets to the cache if there are any content changes, or if `force` is true.
/// Returns the store of carried in elements. These should be rechecked to the
/// remote.
pub fn carry_in(
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    xvc_paths_to_carry: &HStore<XvcPath>,
    cache_paths: &HStore<XvcCachePath>,
    cache_types: &XvcStore<CacheType>,
    parallel: bool,
) -> Result<()> {
    assert! {
        xvc_paths_to_carry.len() == cache_paths.len(),
        "The number of xvc paths and the number of cache paths should be the same."
    }

    let copy_path_to_cache_and_recheck = |xe, xp| {
        let cache_path = uwo!(cache_paths.get(xe).cloned(), output_snd);
        uwr!(move_to_cache(xvc_root, xp, &cache_path), output_snd);
        let cache_type = uwo!(cache_types.get(xe).cloned(), output_snd);
        info!(output_snd, "[CARRY] {xp} -> {cache_path}");
        uwr!(
            recheck_from_cache(output_snd, xvc_root, xp, &cache_path, cache_type),
            output_snd
        );
        info!(output_snd, "[RECHECK] {cache_path} -> {xp}");
    };

    if parallel {
        xvc_paths_to_carry
            .par_iter()
            .for_each(|(xe, xp)| copy_path_to_cache_and_recheck(xe, xp));
    } else {
        xvc_paths_to_carry
            .iter()
            .for_each(|(xe, xp)| copy_path_to_cache_and_recheck(xe, xp));
    }

    Ok(())
}
