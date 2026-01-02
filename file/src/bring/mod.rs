//! Bring files from external storages to workspace.
//!
//! - [BringCLI] defines the command line options.
//!
//! - [cmd_bring]  is the entry point for the command.
//!   Uses [fetch] and [crate::recheck::cmd_recheck] to bring the file and copy/link it to the
//!   workspace.

use crate::common::{load_targets_from_store, move_to_cache};

use crate::{
    recheck::{cmd_recheck, RecheckCLI},
    Result,
};

use clap::Parser;

use clap_complete::ArgValueCompleter;
use xvc_core::util::completer::{strum_variants_completer, xvc_path_completer};
use xvc_core::{debug, error, uwr, warn, XvcOutputSender};
use xvc_core::{
    ContentDigest, HStore, RecheckMethod, XvcCachePath, XvcFileType, XvcMetadata, XvcRoot, XvcStore,
};

use xvc_core::PathSync;
use xvc_storage::storage::storage_identifier_completer;
use xvc_storage::XvcStorageEvent;
use xvc_storage::{storage::get_storage_record, StorageIdentifier, XvcStorageOperations};

/// Bring (download, pull, fetch) files from storage.
///
/// You can configure a new storage with [`xvc storage new`][xvc_storage::new] and use it to
/// download and upload tracked files.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case")]
pub struct BringCLI {
    /// Storage name or guid to send the files
    #[arg(long, short, alias = "from", add = ArgValueCompleter::new(storage_identifier_completer))]
    storage: StorageIdentifier,

    /// Force even if the files are already present in the workspace
    #[arg(long)]
    force: bool,

    /// Don't recheck (checkout) after bringing the file to cache.
    ///
    /// This makes the command similar to `git fetch` in Git.
    /// It just updates the cache, and doesn't copy/link the file to workspace.
    #[arg(long)]
    no_recheck: bool,

    /// Recheck (checkout) the file in one of the four alternative ways.
    /// (See `xvc file recheck`) and [RecheckMethod]
    #[arg(long, alias = "as", add = ArgValueCompleter::new(strum_variants_completer::<RecheckMethod>))]
    recheck_as: Option<RecheckMethod>,

    /// Targets to bring from the storage
    #[arg(add = ArgValueCompleter::new(xvc_path_completer))]
    targets: Option<Vec<String>>,
}

/// Download files in `opts.targets` from `opts.storage` to cache.
///
/// - Retrieves the storage record from `xvc_root`.
/// - Expands globs in `opts.targets`.
/// - Gets the corresponding cache path for each file target.
/// - Calls `storage.receive` for each of these targets.
pub fn fetch(output_snd: &XvcOutputSender, xvc_root: &XvcRoot, opts: &BringCLI) -> Result<()> {
    let storage = get_storage_record(output_snd, xvc_root, &opts.storage)?;

    let current_dir = xvc_root.current_dir();
    let targets = load_targets_from_store(output_snd, xvc_root, current_dir, &opts.targets)?;
    let force = opts.force;

    let target_xvc_metadata = xvc_root
        .load_store::<XvcMetadata>()?
        .subset(targets.keys().copied())?;

    let target_file_xvc_metadata =
        target_xvc_metadata.filter(|_, xmd| xmd.file_type == XvcFileType::File);

    let target_files = targets.subset(target_file_xvc_metadata.keys().copied())?;

    // Get all cache paths for these paths
    let content_digest_store: XvcStore<ContentDigest> = xvc_root.load_store()?;

    let target_content_digests = content_digest_store.subset(target_files.keys().copied())?;

    assert! {
        target_content_digests.len() == target_files.len(),
        "All files should have a content digest"
    }

    let cache_paths: HStore<XvcCachePath> = target_content_digests
        .iter()
        .filter_map(|(xe, cd)| {
            let xvc_path = target_files.get(xe).unwrap();
            match XvcCachePath::new(xvc_path, cd) {
                Ok(cp) => Some((*xe, cp)),
                Err(e) => {
                    warn!(output_snd, "Error: {}", e);
                    None
                }
            }
        })
        .filter(|(_, cp)| {
            if force {
                return true;
            }
            let cache_path = cp.to_absolute_path(xvc_root);
            if cache_path.exists() {
                debug!(output_snd, "Cache path already exists: {}", cache_path);
                false
            } else {
                true
            }
        })
        .collect();

    let (temp_dir, event) = storage
        .receive(
            output_snd,
            xvc_root,
            cache_paths
                .values()
                .cloned()
                .collect::<Vec<XvcCachePath>>()
                .as_slice(),
            opts.force,
        )
        .map_err(|e| xvc_core::Error::from(anyhow::anyhow!("Remote error: {}", e)))?;

    let path_sync = PathSync::new();
    // Move the files from temp dir to cache
    for (_, cp) in cache_paths {
        let cache_path = cp.to_absolute_path(xvc_root);
        let temp_path = temp_dir.temp_cache_path(&cp)?;
        if temp_path.exists() {
            uwr!(
                move_to_cache(&temp_path, &cache_path, &path_sync),
                output_snd
            );
        } else {
            error!(output_snd, "Could not download {}", cp);
        }
    }

    xvc_root.with_store_mut(|store: &mut XvcStore<XvcStorageEvent>| {
        store.insert(
            xvc_root.new_entity(),
            XvcStorageEvent::Receive(event.clone()),
        );
        Ok(())
    })?;

    Ok(())
}

/// Retrieve files from storage and checkout them into the workspace.
///
/// - [fetch] targets from the storage
/// - [checkout][cmd_checkout] them from storage if `opts.no_checkout` is false. (default)
pub fn cmd_bring(output_snd: &XvcOutputSender, xvc_root: &XvcRoot, opts: BringCLI) -> Result<()> {
    fetch(output_snd, xvc_root, &opts)?;
    if !opts.no_recheck {
        let recheck_targets = opts.targets.clone();

        let recheck_opts = RecheckCLI {
            recheck_method: opts.recheck_as,
            no_parallel: false,
            force: opts.force,
            targets: recheck_targets,
        };

        cmd_recheck(output_snd, xvc_root, recheck_opts)?;
    }

    Ok(())
}
