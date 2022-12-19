//! Data structures and functions for `xvc file recheck`.
//!
//! - [RecheckCLI] describes the command line options.
//! - [cmd_recheck] is the entry point for the command line.
use std::collections::HashSet;
use std::fs;

use crate::common::compare::{
    diff_cache_type, diff_content_digest, diff_text_or_binary, diff_xvc_path_metadata,
};
use crate::common::{
    only_file_targets, targets_from_store, update_store_records, xvc_path_metadata_map_from_disk,
    FileTextOrBinary,
};
use crate::{common::recheck_from_cache, Result};
use clap::Parser;
use crossbeam_channel::Sender;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use xvc_config::{FromConfigKey, UpdateFromXvcConfig, XvcConfig};
use xvc_core::{
    apply_diff, CacheType, ContentDigest, Diff, Diff3, DiffStore, DiffStore3, HashAlgorithm,
    TextOrBinary, XvcCachePath, XvcFileType, XvcMetadata, XvcPath, XvcPathMetadataMap, XvcRoot,
};
use xvc_ecs::{HStore, XvcEntity, XvcStore};
use xvc_logging::{error, warn, watch, XvcOutputLine};
use xvc_walker::Glob;

/// Check out file from cache by a copy or link
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case", author, version)]
pub struct RecheckCLI {
    /// How to track the file contents in cache: One of copy, symlink, hardlink, reflink.
    ///
    /// Note: Reflink uses copy if the underlying file system doesn't support it.
    #[arg(long, alias = "as")]
    pub cache_type: Option<CacheType>,

    /// Don't use parallelism
    #[arg(long)]
    pub no_parallel: bool,

    /// Force even if target exists
    #[arg(long)]
    pub force: bool,

    /// Recheck files as text, binary (Default: auto)
    ///
    /// Text files may go OS specific line ending replacements.
    #[arg(long)]
    pub text_or_binary: Option<FileTextOrBinary>,
    /// Files/directories to recheck
    #[arg()]
    pub targets: Option<Vec<String>>,
}

impl UpdateFromXvcConfig for RecheckCLI {
    fn update_from_conf(self, conf: &XvcConfig) -> xvc_config::error::Result<Box<Self>> {
        let cache_type = self
            .cache_type
            .unwrap_or_else(|| CacheType::from_conf(conf));
        let no_parallel = self.no_parallel || conf.get_bool("file.add.no_parallel")?.option;

        let text_or_binary = self.text_or_binary.as_ref().map_or_else(
            || Some(FileTextOrBinary::from_conf(conf)),
            |v| Some(v.to_owned()),
        );
        let force = self.force;

        Ok(Box::new(Self {
            targets: self.targets,
            cache_type: Some(cache_type),
            force,
            no_parallel,
            text_or_binary,
        }))
    }
}

/// Run `xvc file recheck` command on the repository `xvc_root` with `cli_opts` options.
///
/// If [`RecheckCLI.targets`] is empty, uses all paths in the repository as targets.
///
/// Uses [PathComparisonParams] to get the overview of all elements in the repository.
/// After getting the list of file targets, runs either [recheck_serial] or [recheck_parallel].
pub fn cmd_recheck(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    cli_opts: RecheckCLI,
) -> Result<()> {
    let conf = xvc_root.config();
    let opts = cli_opts.update_from_conf(conf)?;
    let current_dir = conf.current_dir()?;
    let targets = targets_from_store(xvc_root, current_dir, opts.targets)?;
    let xvc_current_dir = XvcPath::new(xvc_root, current_dir, current_dir)?;
    watch!(xvc_current_dir);

    let cache_type = opts.cache_type.unwrap_or_else(|| CacheType::default());
    let text_or_binary = opts.text_or_binary;

    let stored_xvc_path_store = xvc_root.load_store::<XvcPath>()?;
    let xvc_metadata_store = xvc_root.load_store::<XvcMetadata>()?;
    let target_files = only_file_targets(&stored_xvc_path_store, &xvc_metadata_store, &targets)?;
    let target_xvc_path_metadata_map = xvc_path_metadata_map_from_disk(xvc_root, &target_files);

    let stored_cache_type_store = xvc_root.load_store::<CacheType>()?;
    let stored_content_digest_store = xvc_root.load_store::<ContentDigest>()?;
    let entities: HashSet<XvcEntity> = target_files.keys().copied().collect();
    let cache_type_diff = diff_cache_type(&stored_cache_type_store, &cache_type, &entities);
    let mut cache_type_targets = cache_type_diff.filter(|_, d| d.changed());

    let stored_text_or_binary_store = xvc_root.load_store::<FileTextOrBinary>()?;
    let text_or_binary_diff = diff_text_or_binary(
        &stored_text_or_binary_store,
        &text_or_binary.unwrap_or_default(),
        &target_files.keys().copied().collect(),
    );

    let xvc_path_metadata_diff = diff_xvc_path_metadata(
        xvc_root,
        &stored_xvc_path_store,
        &xvc_metadata_store,
        &target_xvc_path_metadata_map,
    );
    let xvc_path_diff: DiffStore<XvcPath> = xvc_path_metadata_diff.0;
    let xvc_metadata_diff: DiffStore<XvcMetadata> = xvc_path_metadata_diff.1;

    let prerequisite_diffs = DiffStore3::<XvcPath, XvcMetadata, FileTextOrBinary>(
        xvc_path_diff,
        xvc_metadata_diff,
        text_or_binary_diff,
    );

    let algorithm = HashAlgorithm::from_conf(conf);

    let content_digest_diff = diff_content_digest(
        output_snd,
        xvc_root,
        &stored_xvc_path_store,
        &stored_content_digest_store,
        &stored_text_or_binary_store,
        &prerequisite_diffs,
        &text_or_binary,
        &Some(algorithm),
        !opts.no_parallel,
    );

    cache_type_targets.retain(|xe, d| {
        if content_digest_diff.contains_key(xe) && content_digest_diff[&xe].changed() {
            let xp = stored_xvc_path_store[&xe];
            warn!(
                output_snd,
                "{} has changed on disk. Either carry in, force, or delete the target to recheck. ",
                xp
            );
            return false;
        } else {
            return true;
        }
    });

    let missing_targets = xvc_metadata_diff.filter(|_, d| matches!(d, Diff::ActualMissing { .. }));

    // We recheck files
    // - if they are not in the workspace
    // - if their cache type is different from the current cache type
    // - if they are in the workspace but force is set

    let files_to_recheck = target_files.filter(|xe, _| {
        opts.force || cache_type_targets.contains_key(xe) || missing_targets.contains_key(xe)
    });

    let updated_cache_type_store =
        apply_diff(&stored_cache_type_store, &cache_type_diff, true, false)?;
    let updated_content_digest_store = apply_diff(
        &stored_content_digest_store,
        &content_digest_diff,
        true,
        false,
    )?;

    recheck(
        output_snd,
        xvc_root,
        &files_to_recheck,
        &updated_cache_type_store,
        &updated_content_digest_store,
        opts.no_parallel,
    );

    xvc_root.save_store(&updated_cache_type_store)?;
    xvc_root.save_store(&updated_content_digest_store)?;

    Ok(())
}

fn recheck(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    files_to_recheck: &HStore<&XvcPath>,
    cache_type_store: &XvcStore<CacheType>,
    content_digest_store: &XvcStore<ContentDigest>,
    parallel: bool,
) -> Result<()> {
    let checkout = |xe, xvc_path: &XvcPath| -> Result<()> {
        let content_digest = content_digest_store[&xe];
        let cache_path = XvcCachePath::new(&xvc_path, &content_digest)?;
        watch!(cache_path);
        if cache_path.to_absolute_path(xvc_root).exists() {
            let target_path = xvc_path.to_absolute_path(xvc_root);
            watch!(target_path);
            if target_path.exists() {
                warn!(
                    output_snd,
                    "{} already exists. Removing to recheck.", xvc_path
                );
                fs::remove_file(target_path)?;
            }
            let cache_type = cache_type_store[&xe];
            recheck_from_cache(output_snd, xvc_root, xvc_path, &cache_path, cache_type)
        } else {
            error!(
                output_snd,
                "{} cannot found in cache: {}", xvc_path, cache_path
            );
            Ok(())
        }
    };

    if parallel {
        files_to_recheck.par_iter().for_each(|(xe, xp)| {
            checkout(*xe, xp).unwrap_or_else(|e| warn!(output_snd, "{}", e));
        });
    } else {
        files_to_recheck.iter().for_each(|(xe, xp)| {
            checkout(*xe, xp).unwrap_or_else(|e| warn!(output_snd, "{}", e));
        });
    }

    Ok(())
}
