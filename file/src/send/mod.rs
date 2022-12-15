use crate::common::targets_from_store;
use crate::Result;

use clap::Parser;
use xvc_core::{ContentDigest, XvcCachePath, XvcFileType, XvcMetadata, XvcRoot};
use xvc_ecs::{HStore, XvcStore};
use xvc_logging::watch;
use xvc_storage::{storage::get_storage_record, StorageIdentifier, XvcStorageOperations};

/// Send (upload) tracked files to storage
///
/// When you define a new storage with [`xvc storage new`][xvc_storage::new] set of commands, you
/// can send the tracked files with this.
///
/// Sent files are placed in a directory structure similar to the local cache.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case")]
pub struct SendCLI {
    /// Storage name or guid to send the files
    #[arg(long, short, alias = "to")]
    remote: StorageIdentifier,
    /// Force even if the files are already present in the storage
    #[arg(long)]
    force: bool,
    /// Targets to send/push/upload to storage
    #[arg()]
    targets: Option<Vec<String>>,
}

/// Send a targets in `opts.targets` in `xvc_root`  to `opt.remote`
pub fn cmd_send(
    output_snd: crossbeam_channel::Sender<xvc_logging::XvcOutputLine>,
    xvc_root: &XvcRoot,
    opts: SendCLI,
) -> Result<()> {
    let remote = get_storage_record(output_snd.clone(), xvc_root, &opts.remote)?;
    watch!(remote);
    let current_dir = xvc_root.config().current_dir()?;
    let targets = targets_from_store(xvc_root, current_dir, opts.targets)?;
    watch!(targets);

    let target_file_xvc_metadata = xvc_root
        .load_store::<XvcMetadata>()?
        .subset(targets.keys().copied())?
        .filter(|xe, xmd| xmd.file_type == XvcFileType::File);

    let target_files = targets.subset(target_file_xvc_metadata.keys().copied())?;

    // Get all cache paths for these paths
    let content_digest_store: XvcStore<ContentDigest> = xvc_root.load_store()?;

    let target_content_digests = content_digest_store.subset(target_files.keys().copied())?;
    watch!(target_content_digests);

    assert! {
        target_content_digests.len() == target_files.len(),
        "All files should have a content digest"
    }

    let cache_paths: HStore<XvcCachePath> = target_content_digests
        .iter()
        .map(|xe, cd| (xe, XvcCachePath::new(xvc_root, cd)))
        .collect();

    watch!(cache_paths);

    remote
        .send(
            output_snd.clone(),
            xvc_root,
            // TODO: Change interface of XvcStorage to get an HStore instead of Vec
            &cache_paths.values().cloned().collect(),
            opts.force,
        )
        .map_err(|e| xvc_core::Error::from(anyhow::anyhow!("Remote error: {}", e)))?;

    Ok(())
}
