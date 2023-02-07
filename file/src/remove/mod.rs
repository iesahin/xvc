use clap::Parser;
use parse_size::parse_size;
use xvc_core::XvcRoot;
use xvc_logging::XvcOutputSender;
use xvc_storage::StorageIdentifier;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct RemoveCLI {
    /// Remove files from cache
    #[arg(long)]
    from_cache: bool,

    /// Remove files from storage
    #[arg(long)]
    from_storage: StorageIdentifier,

    /// Remove all versions of the file
    #[arg(long)]
    all_versions: bool,

    /// Remove only the specified version of the file
    /// Versions are specified like b3-123-456-789abcd where b3 is the hash algorithm prefix and the rest is a (at least
    /// 3 digit) prefix of the content hash. Prefix must be unique. If the prefix is not unique, the command will fail.
    /// Dashes are optional.
    #[arg(long)]
    only_version: Option<String>,

    /// Remove all versions of the file carried in earlier than the given timestamp.
    /// Timestamps are specified like 2023-01-01T12:34:56Z in RFC3339 format.
    #[arg(long)]
    before: Option<String>,

    /// Remove all versions of the file carried in after than the given timestamp.
    /// Timestamps are specified like 2023-01-01T12:34:56Z in RFC3339 format.
    #[arg(long)]
    after: Option<String>,

    /// Remove all versions of the targets larger than the given size.
    /// Size can be specified like 1 KiB, 1 TB or 1.5 MB.
    /// See https://docs.rs/parse-size/latest/parse_size/ for more details.
    #[arg(long, parse_from_str = parse_size)]
    larger_than: Option<u64>,

    /// Remove all versions of the targets smaller than the given size.
    /// Size can be specified like 1 KiB, 1 TB or 1.5 MB.
    /// See https://docs.rs/parse-size/latest/parse_size/ for more details.
    #[arg(long, parse_from_str = parse_size)]
    smaller_than: Option<u64>,

    /// Remove the targets even if they are used by other targets (via deduplication)
    #[arg(long)]
    force: bool,

    /// Files/directories to remove
    #[arg()]
    targets: Vec<String>,
}

pub(crate) fn cmd_remove(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    opts: RemoveCLI,
) -> Result<()> {
    todo!()
}
