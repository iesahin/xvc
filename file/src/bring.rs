//! Bring files from external storages to workspace.
//!
//! - [BringCLI] defines the command line options.
//!
//! - [cmd_bring]  is the entry point for the command.
//! Uses [fetch] and [crate::recheck::cmd_recheck] to bring the file and copy/link it to the
//! workspace.

use crate::{
    recheck::{cmd_recheck, RecheckCLI},
    Result,
};

use clap::Parser;
use crossbeam_channel::Sender;
use xvc_core::{CacheType, ContentDigest, XvcCachePath, XvcPath, XvcRoot};
use xvc_ecs::XvcStore;
use xvc_logging::{uwo, uwr, watch, XvcOutputLine};
use xvc_storage::{storage::get_storage_record, StorageIdentifier, XvcStorageOperations};
use xvc_walker::Glob;

/// Bring (download, pull, fetch) files from storage.
///
/// You can configure a new storage with [`xvc storage new`][xvc_storage::new] and use it to
/// download and upload tracked files.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case")]
pub struct BringCLI {
    /// Storage name or guid to send the files
    #[arg(long, short, alias = "from")]
    storage: StorageIdentifier,

    /// Force even if the files are already present in the workspace
    #[arg(long)]
    force: bool,

    /// Don't checkout after bringing the file to cache.
    ///
    /// This is similar to `fetch` command in Git.
    /// It just updates the cache, and doesn't bring the file to workspace.
    #[arg(long)]
    no_checkout: bool,

    /// Checkout the file in one of the four alternative ways.
    /// (See `xvc file checkout`) and [CacheType][CacheType].
    #[arg(long)]
    checkout_as: Option<CacheType>,

    /// Targets to bring from the storage
    #[arg()]
    targets: Vec<String>,
}

/// Download files in `opts.targets` from `opts.storage` to cache.
///
/// - Retrieves the storage record from `xvc_root`.
/// - Expands globs in `opts.targets`.
/// - Gets the corresponding cache path for each file target.
/// - Calls `storage.receive` for each of these targets.
pub fn fetch(output_snd: Sender<XvcOutputLine>, xvc_root: &XvcRoot, opts: &BringCLI) -> Result<()> {
    let remote = get_storage_record(output_snd.clone(), xvc_root, &opts.storage)?;

    let path_store: XvcStore<XvcPath> = xvc_root.load_store()?;

    // If the targets are empty, all paths are retrieved
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
            let content_digest = uwo!(content_digest_store.get(e), output_snd);
            uwr!(XvcCachePath::new(xvc_path, content_digest), output_snd)
        })
        .collect();

    remote
        .receive(output_snd.clone(), xvc_root, &cache_paths, opts.force)
        .map_err(|e| xvc_core::Error::from(anyhow::anyhow!("Remote error: {}", e)))?;

    Ok(())
}

/// Retrieve files from storage and checkout them into the workspace.
///
/// - [fetch] targets from the storage
/// - [checkout][cmd_checkout] them from storage if `opts.no_checkout` is false. (default)
pub fn cmd_bring(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    opts: BringCLI,
) -> Result<()> {
    fetch(output_snd.clone(), xvc_root, &opts)?;

    if !opts.no_checkout {
        let checkout_targets = opts.targets.clone();

        watch!(checkout_targets);
        let checkout_opts = RecheckCLI {
            cache_type: opts.checkout_as,
            no_parallel: false,
            force: opts.force,
            text_or_binary: None,
            targets: checkout_targets,
        };

        cmd_recheck(output_snd, xvc_root, checkout_opts)?;
    }

    Ok(())
}
