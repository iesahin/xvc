use crate::{common::cache_path, Error, Result};
use std::{path::PathBuf, process::exit, str::FromStr};

use clap::Parser;
use crossbeam_channel::Sender;
use derive_more::Display;
use xvc_core::{ContentDigest, XvcCachePath, XvcPath, XvcRoot};
use xvc_ecs::{HStore, XvcEntity, XvcStore};
use xvc_logging::XvcOutputLine;
use xvc_storage::{
    storage::{get_remote_from_store, XvcStorage, XvcStorageGuid, XvcStorageOperations},
    StorageIdentifier,
};
use xvc_walker::Glob;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(about = "Fetch files from remote", rename_all = "kebab-case")]
pub struct FetchCLI {
    /// remote name or guid to send the files
    #[arg(long, short, alias = "from")]
    pub remote: StorageIdentifier,
    /// force even if the files are already present
    #[arg(long)]
    pub force: bool,
    /// targets to push to remote
    #[arg()]
    pub targets: Vec<String>,
}

pub fn cmd_fetch(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    opts: FetchCLI,
) -> Result<()> {
    let remote = get_remote_from_store(output_snd.clone(), xvc_root, &opts.remote)?;

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
            let content_digest = content_digest_store.get(e).unwrap();
            cache_path(xvc_path, &content_digest)
        })
        .collect();

    remote
        .receive(output_snd.clone(), xvc_root, &cache_paths, opts.force)
        .map_err(|e| xvc_core::Error::from(anyhow::anyhow!("Remote error: {}", e)))?;

    Ok(())
}
