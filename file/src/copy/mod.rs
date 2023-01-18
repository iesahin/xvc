use crate::Result;
use clap::Parser;
use crossbeam_channel::Sender;
use xvc_core::{CacheType, XvcRoot};
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
    requires_xvc_repository: &XvcRoot,
    opts: CopyCLI,
) -> Result<()> {
    todo!()
}
