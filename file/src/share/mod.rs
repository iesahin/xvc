//!  Share files from S3 compatible storages for a limited time

use crate::{common::load_targets_from_store, error, Result};
use clap::{command, Parser};
use clap_complete::ArgValueCompleter;
use humantime;
use xvc_core::{
    util::completer::xvc_path_completer, ContentDigest, XvcCachePath, XvcFileType, XvcMetadata,
    XvcRoot,
};
use xvc_core::XvcStore;
use xvc_core::{uwo, watch, XvcOutputSender};
use xvc_storage::{
    storage::{get_storage_record, storage_identifier_completer},
    StorageIdentifier, XvcStorageOperations,
};

/// Share (uploaded and tracked) files from an S3 compatible storage
///
/// Define a storage with [`xvc storage new`][xvc_storage::new] set of commands first. Then you
/// can share the file URL with someone for a limited period with this. This files first sends a file to the remote with [`xvc file send`][xvc_file::send] if it's not present in the remote.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case")]
pub struct ShareCLI {
    /// Storage name or guid to send the files
    #[arg(long, short, alias = "from", add = ArgValueCompleter::new(storage_identifier_completer))]
    storage: StorageIdentifier,

    /// Period to send the files to. You can use s, m, h, d, w suffixes.
    #[arg(long, short, default_value = "24h")]
    duration: String,
    /// File to send/push/upload to storage
    #[arg(add = ArgValueCompleter::new(xvc_path_completer))]
    target: String,
}

/// Handler function for `xvc share` command. Runs the command with `opts` within `xvc_root` and
/// sends output to `output_snd`.
pub fn cmd_share(output_snd: &XvcOutputSender, xvc_root: &XvcRoot, opts: ShareCLI) -> Result<()> {
    // TODO: TIDY UP these implementation to reuse code in other places
    let storage = get_storage_record(output_snd, xvc_root, &opts.storage)?;
    let current_dir = xvc_root.config().current_dir()?;
    let targets =
        load_targets_from_store(output_snd, xvc_root, current_dir, &Some(vec![opts.target]))?;

    let target_file_xvc_metadata = xvc_root
        .load_store::<XvcMetadata>()?
        .subset(targets.keys().copied())?
        .filter(|_, xmd| xmd.file_type == XvcFileType::File)
        .cloned();

    let target_files = targets.subset(target_file_xvc_metadata.keys().copied())?;

    if target_files.is_empty() {
        return Err(error::Error::NoFilesToShare);
    }

    if target_files.len() > 1 {
        return Err(error::Error::MultipleFilesToShare);
    }

    let (target_file_e, target_file) = target_files.iter().next().unwrap();

    let content_digest_store: XvcStore<ContentDigest> = xvc_root.load_store()?;

    let target_content_digest = uwo!(content_digest_store.get(target_file_e), output_snd);

    let cache_path = XvcCachePath::new(target_file, target_content_digest)?;

    let duration = humantime::parse_duration(&opts.duration)?;

    storage.share(output_snd, xvc_root, &cache_path, duration)?;
    Ok(())
}
