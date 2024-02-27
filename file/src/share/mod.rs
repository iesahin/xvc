

/// Share (uploaded and tracked) files from an S3 compatible storage
///
/// Define a storage with [`xvc storage new`][xvc_storage::new] set of commands first. Then you
/// can share the file URL with someone for a limited period with this. This files first sends a file to the remote with [`xvc file send`][xvc_file::send] if it's not present in the remote. 
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case")]
pub struct ShareCLI {
    /// Storage name or guid to send the files
    #[arg(long, short, alias = "from")]
    remote: StorageIdentifier,
    /// Period to send the files to. You can use s, m, h, d, w suffixes.
    #[arg(long, short, default="24h")]
    period: Option<String>,
    /// File to send/push/upload to storage
    #[arg()]
    target: PathBuf,
}


