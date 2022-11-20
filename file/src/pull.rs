use crate::{
    checkout::{cmd_checkout, CheckoutCLI},
    common::cache_path,
    fetch::{cmd_fetch, FetchCLI},
    Result,
};
use std::{path::PathBuf, process::exit, str::FromStr};

use clap::Parser;
use crossbeam_channel::Sender;
use derive_more::Display;
use xvc_core::{ContentDigest, XvcCachePath, XvcPath, XvcRoot};
use xvc_ecs::{HStore, XvcStore};
use xvc_logging::{watch, XvcOutputLine};
use xvc_storage::{
    storage::get_remote_from_store, StorageIdentifier, XvcStorage, XvcStorageGuid,
    XvcStorageOperations,
};
use xvc_walker::Glob;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(about = "Fetch files from remote", rename_all = "kebab-case")]
pub struct PullCLI {
    /// remote name or guid to send the files
    #[arg(long, short, alias = "from")]
    remote: StorageIdentifier,
    /// force even if the files are already present in the workspace
    #[arg(long)]
    force: bool,
    /// targets to push to remote
    #[arg()]
    targets: Vec<String>,
}

pub fn cmd_pull(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    opts: PullCLI,
) -> Result<()> {
    let fetch_opts = FetchCLI {
        remote: opts.remote,
        force: opts.force,
        targets: opts.targets.clone(),
    };

    cmd_fetch(output_snd.clone(), xvc_root, fetch_opts)?;

    let checkout_targets = opts.targets.clone();

    watch!(checkout_targets);

    let checkout_opts = CheckoutCLI {
        cache_type: None,
        no_parallel: false,
        force: opts.force,
        text_or_binary: None,
        targets: checkout_targets,
    };

    cmd_checkout(output_snd, xvc_root, checkout_opts)?;

    Ok(())
}
