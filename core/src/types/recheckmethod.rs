//! Recheck methods denote how a cached file put into the working directory.
//! See [CacheType] for different types
use serde::{Deserialize, Serialize};
use strum_macros::{Display as EnumDisplay, EnumString, IntoStaticStr};
use xvc_config::{conf, FromConfigKey};

use xvc_ecs::persist;

/// Denotes how a cache file is linked to the workspace.
///
/// A file can be rechecked in four different ways.
/// The most basic one is `Copy`, where the file in cache is copied to the workspace.
/// Symlinks can be used to save space when file is read only.
/// Hardlinks are another alternative to save space when cache and working directory are on the
/// same mount point.
/// In some file systems hardlinks may be easier to handle than symlinks.
/// Reflinks are a newer kind of links with copy-on-write semantics.
/// When they are read only, they act like symlinks.
/// When they are written, they are copied to from the cache file.
/// Reflinks are supported by Btrfs, CIFS, NFS 4.2, OCFS2, overlayfs, and XFS currently.
///
/// Recheck method can be set for each file differently.
/// You can configure one file to use copy if it will be written, and a set of directories as
/// symlink if they will be read only.
///
/// Following commands have options to denote recheck method.
/// - [`xvc file track`](https://docs.xvc.dev/ref/xvc-file-track),
/// - [`xvc file recheck`](https://docs.xvc.dev/ref/xvc-file-recheck),
/// - [`xvc file copy`](https://docs.xvc.dev/ref/xvc-file-copy),
///
/// Default [RecheckMethod]  can be configured with `file.recheck.method` option.
/// [RecheckMethod]s for files are stored in `recheck-method` XvcStore.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    EnumString,
    EnumDisplay,
    IntoStaticStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum RecheckMethod {
    /// Copy the cached file to its location in working directory.
    /// There will be two identical files, one in cache, one in the workspace.
    /// If one of them is deleted, the other is not affected.
    /// Xvc can the track the content changes in copied files, and carry-in them to cache.
    Copy,
    /// Make a hardlink in workspace to the cached file.
    /// It requires the cache and working directory to be in the same mount point.
    /// There will be a single inode that contains the content of the file, but two file system
    /// records pointing to this content.
    /// If you delete the workspace copy, the other still resides in the cache.
    /// Note that, the cache file and its links are read-only.
    Hardlink,
    /// Make a symlink in workspace to the cached file.
    /// It requires a file system that supports symbolic links.
    /// The content of the file will be kept in cache, and a symbolic link to this file will be created in the workspace.
    /// If the cache file is deleted for some reason, the symbolic link will be broken and the
    /// data will be lost.
    /// If the symbolic link in the workspace is deleted, the cached content is not
    /// affected.
    Symlink,
    /// Make a reflink to the cached file in the working directory.
    /// Reflinks are similar to symlinks, but they are copied when the file is to be written.
    /// Currently reflinks are supported by Btrfs, CIFS, NFS 4.2, OCFS2, overlayfs, and XFS.
    /// If the cache file is deleted for some reason, the reflink becomes broken and won't keep
    /// the file content.
    /// If the reflink is deleted, the content in the cache won't be affected.
    Reflink,
}

impl Default for RecheckMethod {
    fn default() -> Self {
        Self::Copy
    }
}
persist!(RecheckMethod, "recheck-method");
conf!(RecheckMethod, "file.recheck.method");
