//! Crate for `xvc file carry-in` command.
//!
//! The command is used to move (commit) files to Xvc cache.
//! It is used after [`xvc file track`][crate::track] or separately to update
//! the cache with changed files.

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use xvc_walker::PathSync;

use std::collections::HashSet;
use std::fs;

use xvc_config::FromConfigKey;
use xvc_config::{UpdateFromXvcConfig, XvcConfig};

use xvc_core::ContentDigest;
use xvc_core::XvcRoot;
use xvc_core::{Diff, XvcCachePath};
use xvc_logging::{info, uwo, uwr, warn, watch, XvcOutputSender};

use crate::common::compare::{diff_content_digest, diff_text_or_binary, diff_xvc_path_metadata};
use crate::common::gitignore::make_ignore_handler;
use crate::common::{
    load_targets_from_store, move_xvc_path_to_cache, only_file_targets, recheck_from_cache,
    xvc_path_metadata_map_from_disk,
};
use crate::common::{update_store_records, FileTextOrBinary};
use crate::error::Result;

use clap::Parser;

use xvc_core::RecheckMethod;

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
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    cli_opts: CarryInCLI,
) -> Result<()> {
    watch!(cli_opts);
    watch!(xvc_root);
    let conf = xvc_root.config();
    let opts = cli_opts.update_from_conf(conf)?;
    watch!(opts);
    let current_dir = conf.current_dir()?;
    let targets = load_targets_from_store(xvc_root, current_dir, &opts.targets)?;
    watch!(targets);

    let stored_xvc_path_store = xvc_root.load_store::<XvcPath>()?;
    let stored_xvc_metadata_store = xvc_root.load_store::<XvcMetadata>()?;
    let target_files = only_file_targets(&stored_xvc_metadata_store, &targets)?;

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
            Diff::ActualMissing { .. } => {
                // carry-in shouldn't be used to delete files from cache
                warn!(
                    output_snd,
                    "{xp} is deleted from workspace. Not deleting cached copy. Use `xvc file delete` if you want to delete {xp}.");
                None
            }
            Diff::RecordMissing { .. } => {
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

    let stored_recheck_method_store = xvc_root.load_store::<RecheckMethod>()?;
    watch!(xvc_paths_to_carry);
    carry_in(
        output_snd,
        xvc_root,
        &xvc_paths_to_carry,
        &cache_paths_to_carry,
        &stored_recheck_method_store,
        !opts.no_parallel,
        opts.force,
    )?;

    // We only update the records for existing paths.
    update_store_records(xvc_root, &xvc_metadata_diff, false, false)?;
    update_store_records(xvc_root, &text_or_binary_diff, false, false)?;
    update_store_records(xvc_root, &content_digest_diff, false, false)?;

    Ok(())
}

/// Move targets to the cache if there are any content changes, or if `force` is true.
/// Returns the store of carried in elements. These should be rechecked to the
/// remote.
pub fn carry_in(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    xvc_paths_to_carry: &HStore<XvcPath>,
    cache_paths: &HStore<XvcCachePath>,
    recheck_methods: &XvcStore<RecheckMethod>,
    parallel: bool,
    force: bool,
) -> Result<()> {
    assert! {
        xvc_paths_to_carry.len() == cache_paths.len(),
        "The number of xvc paths and the number of cache paths should be the same."
    }

    let (ignore_writer, ignore_thread) = make_ignore_handler(output_snd, xvc_root)?;

    let path_sync = PathSync::new();

    watch!(ignore_writer);
    watch!(ignore_thread);

    // TODO: Remove this when we set unix permissions in platform dependent fashion
    #[allow(clippy::permissions_set_readonly_false)]
    let copy_path_to_cache_and_recheck = |xe, xp| {
        let cache_path = uwo!(cache_paths.get(xe).cloned(), output_snd);
        watch!(cache_path);
        let abs_cache_path = cache_path.to_absolute_path(xvc_root);
        watch!(abs_cache_path);
        if abs_cache_path.exists() {
            if force {
                let cache_dir = uwo!(abs_cache_path.parent(), output_snd);
                watch!(cache_dir);
                let mut dir_perm = uwr!(cache_dir.metadata(), output_snd).permissions();
                watch!(dir_perm);
                dir_perm.set_readonly(false);
                watch!(dir_perm);
                uwr!(fs::set_permissions(cache_dir, dir_perm), output_snd);
                watch!(cache_dir);
                let mut file_perm =
                    uwr!(abs_cache_path.as_path().metadata(), output_snd).permissions();
                watch!(file_perm);
                watch!(abs_cache_path);
                watch!(file_perm);
                file_perm.set_readonly(false);
                uwr!(fs::set_permissions(&abs_cache_path, file_perm), output_snd);
                /* let mut dir_perm = cache_dir.metadata()?.permissions(); */
                /* dir_perm.set_readonly(true); */
                uwr!(fs::remove_file(&abs_cache_path), output_snd);
                info!(output_snd, "[REMOVE] {abs_cache_path}");
                uwr!(
                    move_xvc_path_to_cache(xvc_root, xp, &cache_path, &path_sync),
                    output_snd
                );
                info!(output_snd, "[CARRY] {xp} -> {cache_path}");
            } else {
                info!(output_snd, "[EXISTS] {abs_cache_path} for {xp}");
            }
        } else {
            watch!(&cache_path);
            watch!(&xp);
            uwr!(
                move_xvc_path_to_cache(xvc_root, xp, &cache_path, &path_sync),
                output_snd
            );
            info!(output_snd, "[CARRY] {xp} -> {cache_path}");
        }
        let target_path = xp.to_absolute_path(xvc_root);
        watch!(target_path);
        if target_path.exists() {
            uwr!(fs::remove_file(&target_path), output_snd);
            info!(output_snd, "[REMOVE] {target_path}");
        }
        let recheck_method = uwo!(recheck_methods.get(xe).cloned(), output_snd);
        uwr!(
            recheck_from_cache(
                output_snd,
                xvc_root,
                xp,
                &cache_path,
                recheck_method,
                &ignore_writer
            ),
            output_snd
        );
        watch!(&cache_path);
        watch!(recheck_method);
        watch!(&xp);
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

    ignore_writer.send(None).unwrap();
    ignore_thread.join().unwrap();

    Ok(())
}
