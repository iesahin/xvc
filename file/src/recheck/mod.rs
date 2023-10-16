//! Data structures and functions for `xvc file recheck`.
//!
//! - [RecheckCLI] describes the command line options.
//! - [cmd_recheck] is the entry point for the command line.
use std::collections::HashSet;
use std::thread::JoinHandle;
use std::{fs, thread};

use crate::common::compare::{diff_content_digest, diff_recheck_method, diff_xvc_path_metadata};
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
    apply_diff, ContentDigest, Diff, DiffStore, HashAlgorithm, RecheckMethod, XvcCachePath,
    XvcMetadata, XvcPath, XvcRoot,
};
use xvc_ecs::{HStore, XvcEntity, XvcStore};
use xvc_logging::{error, info, uwr, warn, watch, XvcOutputSender};

/// Check out file from cache by a copy or link
///
/// There are three conditions to recheck a file:
///
/// - If the workspace copy is missing.
/// - If the workspace copy is not changed but the user wants to change recheck method. (e.g. from copy
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
    pub recheck_method: Option<RecheckMethod>,

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
        let recheck_method = self
            .recheck_method
            .unwrap_or_else(|| RecheckMethod::from_conf(conf));
        let no_parallel = self.no_parallel || conf.get_bool("file.track.no_parallel")?.option;

        let force = self.force;

        Ok(Box::new(Self {
            targets: self.targets,
            recheck_method: Some(recheck_method),
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
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    cli_opts: RecheckCLI,
) -> Result<()> {
    let conf = xvc_root.config();
    let opts = cli_opts.update_from_conf(conf)?;
    let current_dir = conf.current_dir()?;
    let targets = load_targets_from_store(xvc_root, current_dir, &opts.targets)?;
    watch!(targets);

    let recheck_method = opts.recheck_method.unwrap_or_else(RecheckMethod::default);
    watch!(recheck_method);

    let stored_xvc_path_store = xvc_root.load_store::<XvcPath>()?;
    let stored_xvc_metadata_store = xvc_root.load_store::<XvcMetadata>()?;
    let target_files = only_file_targets(&stored_xvc_metadata_store, &targets)?;
    let target_xvc_path_metadata_map = xvc_path_metadata_map_from_disk(xvc_root, &target_files);

    let stored_recheck_method_store = xvc_root.load_store::<RecheckMethod>()?;
    let stored_content_digest_store = xvc_root.load_store::<ContentDigest>()?;
    let entities: HashSet<XvcEntity> = target_files.keys().copied().collect();
    let recheck_method_diff =
        diff_recheck_method(&stored_recheck_method_store, recheck_method, &entities);
    let mut recheck_method_targets = recheck_method_diff.filter(|_, d| d.changed());

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
    recheck_method_targets.retain(|xe, _| {
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
            false
        } else {
            true
        }
    });

    let no_digest_targets =
        content_digest_diff.filter(|_, d| matches!(d, Diff::ActualMissing { .. }));

    watch!(no_digest_targets);
    // We recheck files
    // - if they are not in the workspace
    // - if their recheck method is different from the current recheck method
    // - if they are in the workspace but force is set

    watch!(target_files);

    let files_to_recheck = target_files.filter(|xe, _| {
        opts.force || recheck_method_targets.contains_key(xe) || no_digest_targets.contains_key(xe)
    });

    watch!(files_to_recheck);

    // We only record the diffs if they are in files to recheck
    let recordable_recheck_method_diff =
        recheck_method_diff.subset(files_to_recheck.keys().copied())?;
    let recordable_content_digest_diff =
        content_digest_diff.subset(files_to_recheck.keys().copied())?;

    watch!(recordable_recheck_method_diff);
    watch!(recordable_content_digest_diff);

    let updated_recheck_method_store = apply_diff(
        &stored_recheck_method_store,
        &recordable_recheck_method_diff,
        true,
        false,
    )?;

    watch!(updated_recheck_method_store);

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
        &updated_recheck_method_store,
        &updated_content_digest_store,
        opts.no_parallel,
    )?;

    xvc_root.save_store(&updated_recheck_method_store)?;
    xvc_root.save_store(&updated_content_digest_store)?;

    Ok(())
}

/// Recheck messages to be sent to the channel created by [`make_recheck_handler`].
pub enum RecheckOperation {
    /// Recheck message to copy/link path described by `content_digest` to `xvc_path`.
    Recheck {
        /// The destination of the message.
        xvc_path: XvcPath,
        /// The content digest of the file to recheck.
        content_digest: ContentDigest,
        /// The recheck method that defines whether to recheck by copy, hardlink, symlink, reflink.
        recheck_method: RecheckMethod,
    },
}

/// The actual messages in channels are `Option<T>` to close the channel by sending `None` when the operation ends.
pub type RecheckOp = Option<RecheckOperation>;

/// Build a recheck handler in a separate thread and connect it with a channel.
/// You must build an ignore writer with [`make_ignore_handler`] before building this.
/// All rechecked files are gitignored using given `ignore_handler`.
/// Use the returned channel to send [`RecheckOp`] messages to recheck files, then send `None` to the channel to exit
/// from the loop and join the returned thread.
pub fn make_recheck_handler(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    ignore_writer: &Sender<IgnoreOp>,
) -> Result<(Sender<RecheckOp>, JoinHandle<()>)> {
    let (recheck_op_snd, recheck_op_rvc) = crossbeam_channel::bounded(crate::CHANNEL_CAPACITY);
    let output_snd = output_snd.clone();
    let xvc_root = xvc_root.clone();
    let ignore_handler = ignore_writer.clone();

    let handle = thread::spawn(move || {
        while let Ok(Some(op)) = recheck_op_rvc.recv() {
            match op {
                RecheckOperation::Recheck {
                    xvc_path,
                    content_digest,
                    recheck_method,
                } => {
                    let cache_path = XvcCachePath::new(&xvc_path, &content_digest).unwrap();
                    uwr!(
                        recheck_from_cache(
                            &output_snd,
                            &xvc_root,
                            &xvc_path,
                            &cache_path,
                            recheck_method,
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
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    files_to_recheck: &HStore<&XvcPath>,
    recheck_method_store: &XvcStore<RecheckMethod>,
    content_digest_store: &XvcStore<ContentDigest>,
    parallel: bool,
) -> Result<()> {
    let (ignore_writer, ignore_thread) = make_ignore_handler(output_snd, xvc_root)?;

    let inner = |xe, xvc_path: &XvcPath| -> Result<()> {
        let content_digest = content_digest_store[&xe];
        let cache_path = XvcCachePath::new(xvc_path, &content_digest)?;
        watch!(cache_path);
        if cache_path.to_absolute_path(xvc_root).exists() {
            let target_path = xvc_path.to_absolute_path(xvc_root);
            watch!(target_path);
            if target_path.exists() {
                info!(output_snd, "[EXISTS] {target_path}");
                fs::remove_file(&target_path)?;
                info!(output_snd, "[REMOVE] {target_path}");
            }
            let recheck_method = recheck_method_store[&xe];
            recheck_from_cache(
                output_snd,
                xvc_root,
                xvc_path,
                &cache_path,
                recheck_method,
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

    ignore_writer.send(None).unwrap();
    ignore_thread.join().unwrap();

    Ok(())
}
