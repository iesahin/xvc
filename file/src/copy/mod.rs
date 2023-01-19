use crate::common::targets_from_store;
use crate::Result;
use anyhow::anyhow;
use clap::Parser;
use crossbeam_channel::Sender;
use xvc_core::{CacheType, XvcMetadata, XvcRoot, XvcPath};
use xvc_logging::XvcOutputLine;

/// CLI for `xvc file copy`.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case", author, version)]
pub struct CopyCLI {
    /// How the targets should be rechecked: One of copy, symlink, hardlink, reflink.
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

    /// Source glob.
    ///
    /// If the source ends with a slash, it's considered a directory and all
    /// files in that directory are copied.
    ///
    /// If the number of source files is more than one, the target must be a directory.
    #[arg()]
    pub source: String,

    /// Target.
    ///
    /// If the target ends with a slash, it's considered a directory and
    /// created if it doesn't exist.
    ///
    /// If the number of source files is more than one, the target must be a directory.
    #[arg()]
    pub target: String,
}

pub(crate) fn cmd_copy(
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    opts: CopyCLI,
) -> Result<()> {
    // Get all files to copy

    let source_targets = if opts.source.ends_with("/") {
        let mut source = opts.source.to_string();
        source.push('*');
        vec![source]
    } else {
        vec![opts.source.to_string()]
    };

    let current_dir = xvc_root.config().current_dir()?;
    let all_sources = targets_from_store(xvc_root, current_dir, &Some(source_targets))?;
    let all_metadata = xvc_root.load_store::<XvcMetadata>()?;
    let source_metadata = all_metadata.subset(all_sources.keys().copied())?;
    let source_files = source_metadata.filter(|xe, md| md.is_file());

    if source_files.len() > 1 && !opts.target.ends_with("/") {
        return Err(anyhow!("Target must be a directory if multiple sources are given").into());
    }

    // Create targets in the store
    // If target is a directory, check if exists and create if not.
    // If target is a file, check if exists and return error if it does and
    // force is not set.
    
    xvc_root.with_r11store_mut(|store: &mut R11Store<XvcPath, XvcMetadata>|) {
        let current_dir_prefix = if *current_dir != *xvc_root.absolute_path() {
            XvcPath::new(&xvc_root, &xvc_root, current_dir)?.to_string()
        } else {
            String::new()
        };

       if opts.target.ends_with('/') {
        let possible_dir = store.left.filter(|xe, xp| xp.to);
       }
    }

    // Recheck target files

    Ok(())
}
