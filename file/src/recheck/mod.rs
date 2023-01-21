//! Data structures and functions for `xvc file recheck`.
//!
//! - [RecheckCLI] describes the command line options.
//! - [cmd_recheck] is the entry point for the command line.
use std::collections::HashSet;
use std::thread::JoinHandle;
use std::{fs, thread};

use crate::common::compare::{diff_cache_type, diff_content_digest, diff_xvc_path_metadata};
use crate::common::gitignore::{make_ignore_handler, IgnoreOp};
use crate::common::{
    load_targets_from_store, only_file_targets, xvc_path_metadata_map_from_disk, FileTextOrBinary,
};
use crate::{common::recheck_from_cache, Result};
use clap::Parser;
use crossbeam_channel::Sender;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use xvc_config::{FromConfigKey, UpdateFromXvcConfig, XvcConfig};

use xvc_core::{
    apply_diff, CacheType, ContentDigest, Diff, DiffStore, HashAlgorithm, XvcCachePath,
    XvcMetadata, XvcPath, XvcRoot,
};
use xvc_ecs::{HStore, XvcEntity, XvcStore};
use xvc_logging::{error, info, uwr, warn, watch, XvcOutputLine};

/// Check out file from cache by a copy or link
///
/// There are three conditions to recheck a file:
///
/// - If the workspace copy is missing.
/// - If the workspace copy is not changed but the user wants to change cache type. (e.g. from copy
/// to symlink.)
/// - If the `--force` is set.
///
/// If the workspace copy of a file is changed, this command doesn't overwrite it by default. Set
/// `--force` to do so.
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

    /// Force even if target exists.
    #[arg(long)]
    pub force: bool,

    /// Files/directories to recheck
    #[arg()]
    pub targets: Option<Vec<String>>,
}

impl UpdateFromXvcConfig for RecheckCLI {
    fn update_from_conf(self, conf: &XvcConfig) -> xvc_config::error::Result<Box<Self>> {
        let cache_type = self
            .cache_type
            .unwrap_or_else(|| CacheType::from_conf(conf));
        let no_parallel = self.no_parallel || conf.get_bool("file.track.no_parallel")?.option;

        let force = self.force;

        Ok(Box::new(Self {
            targets: self.targets,
            cache_type: Some(cache_type),
            force,
            no_parallel,
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
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    cli_opts: RecheckCLI,
) -> Result<()> {
    let conf = xvc_root.config();
    let opts = cli_opts.update_from_conf(conf)?;
    let current_dir = conf.current_dir()?;
    let targets = load_targets_from_store(xvc_root, current_dir, &opts.targets)?;
    watch!(targets);

    let cache_type = opts.cache_type.unwrap_or_else(|| CacheType::default());
    watch!(cache_type);

    let stored_xvc_path_store = xvc_root.load_store::<XvcPath>()?;
    let stored_xvc_metadata_store = xvc_root.load_store::<XvcMetadata>()?;
    let target_files = only_file_targets(&stored_xvc_metadata_store, &targets)?;
    let target_xvc_path_metadata_map = xvc_path_metadata_map_from_disk(xvc_root, &target_files);

    let stored_cache_type_store = xvc_root.load_store::<CacheType>()?;
    let stored_content_digest_store = xvc_root.load_store::<ContentDigest>()?;
    let entities: HashSet<XvcEntity> = target_files.keys().copied().collect();
    let cache_type_diff = diff_cache_type(&stored_cache_type_store, cache_type, &entities);
    let mut cache_type_targets = cache_type_diff.filter(|_, d| d.changed());

    let xvc_path_metadata_diff = diff_xvc_path_metadata(
        xvc_root,
        &stored_xvc_path_store,
        &stored_xvc_metadata_store,
        &target_xvc_path_metadata_map,
    );
    let xvc_path_diff: DiffStore<XvcPath> = xvc_path_metadata_diff.0;
    let xvc_metadata_diff: DiffStore<XvcMetadata> = xvc_path_metadata_diff.1;

    let algorithm = HashAlgorithm::from_conf(conf);
    let stored_text_or_binary_store = xvc_root.load_store::<FileTextOrBinary>()?;

    let content_digest_diff = diff_content_digest(
        output_snd,
        xvc_root,
        &stored_xvc_path_store,
        &stored_xvc_metadata_store,
        &stored_content_digest_store,
        &stored_text_or_binary_store,
        &xvc_path_diff,
        &xvc_metadata_diff,
        None,
        Some(algorithm),
        !opts.no_parallel,
    );

    watch!(content_digest_diff);
    cache_type_targets.retain(|xe, _| {
        watch!(content_digest_diff.get(xe));
        if content_digest_diff.contains_key(xe)
            && matches!(
                content_digest_diff[&xe],
                Diff::<ContentDigest>::Different { .. }
            )
        {
            let output_snd = output_snd.clone();
            let xp = &stored_xvc_path_store[&xe];
            error!(
                output_snd,
                "{} has changed on disk. Either carry in, force, or delete the target to recheck. ",
                xp
            );
            return false;
        } else {
            return true;
        }
    });

    let no_digest_targets =
        content_digest_diff.filter(|_, d| matches!(d, Diff::ActualMissing { .. }));

    watch!(no_digest_targets);
    // We recheck files
    // - if they are not in the workspace
    // - if their cache type is different from the current cache type
    // - if they are in the workspace but force is set

    watch!(target_files);

    let files_to_recheck = target_files.filter(|xe, _| {
        opts.force || cache_type_targets.contains_key(xe) || no_digest_targets.contains_key(xe)
    });

    watch!(files_to_recheck);

    // We only record the diffs if they are in files to recheck
    let recordable_cache_type_diff = cache_type_diff.subset(files_to_recheck.keys().copied())?;
    let recordable_content_digest_diff =
        content_digest_diff.subset(files_to_recheck.keys().copied())?;

    watch!(recordable_cache_type_diff);
    watch!(recordable_content_digest_diff);

    let updated_cache_type_store = apply_diff(
        &stored_cache_type_store,
        &recordable_cache_type_diff,
        true,
        false,
    )?;

    watch!(updated_cache_type_store);

    let updated_content_digest_store = apply_diff(
        &stored_content_digest_store,
        &recordable_content_digest_diff,
        true,
        false,
    )?;
    watch!(updated_content_digest_store);

    recheck(
        output_snd,
        xvc_root,
        &files_to_recheck,
        &updated_cache_type_store,
        &updated_content_digest_store,
        opts.no_parallel,
    )?;

    xvc_root.save_store(&updated_cache_type_store)?;
    xvc_root.save_store(&updated_content_digest_store)?;

    Ok(())
}

pub enum RecheckOperation {
    Recheck {
        xvc_path: XvcPath,
        content_digest: ContentDigest,
        cache_type: CacheType,
    },
}

type RecheckOp = Option<RecheckOperation>;

pub fn make_recheck_handler(
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    ignore_handler: &Sender<IgnoreOp>,
    files_to_recheck: &HStore<&XvcPath>,
) -> Result<(Sender<RecheckOp>, JoinHandle<()>)> {
    let (recheck_op_snd, recheck_op_rvc) = crossbeam_channel::bounded(crate::CHANNEL_CAPACITY);
    let output_snd = output_snd.clone();
    let xvc_root = xvc_root.clone();
    let ignore_handler = ignore_handler.clone();

    let handle = thread::spawn(move || {
        while let Ok(Some(op)) = recheck_op_rvc.recv() {
            match op {
                RecheckOperation::Recheck {
                    xvc_path,
                    content_digest,
                    cache_type,
                } => {
                    let cache_path = XvcCachePath::new(&xvc_path, &content_digest).unwrap();
                    uwr!(
                        recheck_from_cache(
                            &output_snd,
                            &xvc_root,
                            &xvc_path,
                            &cache_path,
                            cache_type,
                            &ignore_handler,
                        ),
                        output_snd
                    );
                }
            }
        }
    });

    Ok((recheck_op_snd, handle))
}

fn recheck(
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    files_to_recheck: &HStore<&XvcPath>,
    cache_type_store: &XvcStore<CacheType>,
    content_digest_store: &XvcStore<ContentDigest>,
    parallel: bool,
) -> Result<()> {
    let (ignore_writer, ignore_thread) = make_ignore_handler(output_snd, xvc_root)?;

    let inner = |xe, xvc_path: &XvcPath| -> Result<()> {
        let content_digest = content_digest_store[&xe];
        let cache_path = XvcCachePath::new(&xvc_path, &content_digest)?;
        watch!(cache_path);
        if cache_path.to_absolute_path(xvc_root).exists() {
            let target_path = xvc_path.to_absolute_path(xvc_root);
            watch!(target_path);
            if target_path.exists() {
                info!(output_snd, "[EXISTS] {target_path}");
                fs::remove_file(&target_path)?;
                info!(output_snd, "[REMOVE] {target_path}");
            }
            let cache_type = cache_type_store[&xe];
            recheck_from_cache(
                &output_snd,
                xvc_root,
                xvc_path,
                &cache_path,
                cache_type,
                &ignore_writer,
            )
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
            inner(*xe, xp).unwrap_or_else(|e| warn!(output_snd, "{}", e));
        });
    } else {
        files_to_recheck.iter().for_each(|(xe, xp)| {
            inner(*xe, xp).unwrap_or_else(|e| warn!(output_snd, "{}", e));
        });
    }

    ignore_writer.send(None);
    ignore_thread.join().unwrap();

    Ok(())
}
