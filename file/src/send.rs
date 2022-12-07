use crate::{common::cache_path, Result};

use clap::Parser;
use xvc_core::{ContentDigest, XvcCachePath, XvcPath, XvcRoot};
use xvc_ecs::XvcStore;
use xvc_logging::XvcOutputLine;
use xvc_storage::{storage::get_storage_record, StorageIdentifier, XvcStorageOperations};
use xvc_walker::Glob;

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
    targets: Vec<String>,
}

/// Send a targets in `opts.targets` in `xvc_root`  to `opt.remote`
pub fn cmd_send(
    output_snd: crossbeam_channel::Sender<xvc_logging::XvcOutputLine>,
    xvc_root: &XvcRoot,
    opts: SendCLI,
) -> Result<()> {
    let remote = get_storage_record(output_snd.clone(), xvc_root, &opts.remote)?;

    let path_store: XvcStore<XvcPath> = xvc_root.load_store()?;

    // If the targets are empty, all paths are pushed
    let target_store = if opts.targets.is_empty() {
        path_store
    } else {
        let mut globsetb = xvc_walker::GlobSetBuilder::new();
        for t in opts.targets.clone() {
            match Glob::new(&t) {
                Ok(g) => {
                    globsetb.add(g);
                }
                Err(e) => {
                    output_snd
                        .send(XvcOutputLine::Warn(format!("Error in glob: {} {}", t, e)))
                        .unwrap();
                }
            }
        }
        let globset = globsetb.build().map_err(|e| xvc_walker::Error::from(e))?;

        path_store.filter(|_, p| globset.is_match(p.to_string()))
    };

    // Get all cache paths for these paths
    let content_digest_store: XvcStore<ContentDigest> = xvc_root.load_store()?;

    let cache_paths: Vec<XvcCachePath> = target_store
        .iter()
        .map(|(e, xvc_path)| {
            let content_digest = content_digest_store.get(e).unwrap();
            cache_path(xvc_path, &content_digest)
        })
        .collect();

    remote
        .send(output_snd.clone(), xvc_root, &cache_paths, opts.force)
        .map_err(|e| xvc_core::Error::from(anyhow::anyhow!("Remote error: {}", e)))?;

    Ok(())
}
