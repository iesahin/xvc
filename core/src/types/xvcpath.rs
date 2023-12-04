//! Home of the [XvcPath] struct, [TextOrBinary] enum and [XvcCachePath] struct.
//!
//! [XvcPath] is the basic path type to represent repository paths in Xvc. It
//! corresponds to a path relative to the [XvcRoot]. It can be converted from a
//! [fs::path::Path] and actually is a wrapper around [RelativePathBuf].

use std::fs;
use std::{fmt::Display, path::Path};

use crate::error::Result;
use derive_more::Display as DeriveDisplay;
use path_absolutize::*;
use relative_path::{RelativePath, RelativePathBuf};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use xvc_logging::{output, uwr, watch, XvcOutputSender};
use xvc_walker::AbsolutePath;

use std::ops::Deref;

use crate::{ContentDigest, HashAlgorithm};
use xvc_ecs::persist;

use super::diff::Diffable;
use super::xvcroot::XvcRoot;

/// A file, symlink or a directory _relative to_ XvcRoot
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct XvcPath(RelativePathBuf);

persist!(XvcPath, "xvc-path");
impl Diffable for XvcPath {
    type Item = XvcPath;
}

impl Deref for XvcPath {
    type Target = RelativePathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for XvcPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<RelativePathBuf> for XvcPath {
    fn from(rpb: RelativePathBuf) -> Self {
        Self(rpb)
    }
}

impl AsRef<str> for XvcPath {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<RelativePath> for XvcPath {
    fn as_ref(&self) -> &RelativePath {
        self.0.as_ref()
    }
}

impl XvcPath {
    /// Given the current_dir and path, create an XvcPath relative to `xvc_root`
    /// and return it.
    ///
    /// ## Panics
    ///
    /// - path shouldn't be empty
    /// - if path is absolute, it must have current_dir as prefix.
    pub fn new(xvc_root: &XvcRoot, current_dir: &AbsolutePath, path: &Path) -> Result<XvcPath> {
        let path = if path.is_absolute() {
            path.strip_prefix(current_dir.as_path())?
        } else {
            path
        };

        if path.as_os_str().is_empty() {
            panic!("Path shouldn't be empty");
        }

        let abs_path = path.absolutize_from(current_dir)?;
        xvc_logging::watch!(abs_path);
        xvc_logging::watch!(current_dir);
        xvc_logging::watch!(xvc_root.absolute_path());
        let rel_path = abs_path.strip_prefix(xvc_root.absolute_path())?;
        Ok(XvcPath(RelativePathBuf::from_path(rel_path)?))
    }

    /// Converts to an absolute path in the file system
    pub fn to_absolute_path(&self, root: &AbsolutePath) -> AbsolutePath {
        AbsolutePath::from(self.0.to_path(root))
    }

    /// Returns the root "."
    pub fn root_path() -> Result<XvcPath> {
        Ok(XvcPath(RelativePathBuf::from_path(".")?))
    }

    /// Returns the clone of inner relativepathbuf for processing
    pub fn relative_pathbuf(&self) -> RelativePathBuf {
        self.0.clone()
    }

    /// Checks whether is a child path of `other`
    pub fn starts_with(&self, other: &XvcPath) -> bool {
        self.0.starts_with(AsRef::<RelativePath>::as_ref(other))
    }

    /// Calculates the content digest of the path
    pub fn digest(
        &self,
        xvc_root: &XvcRoot,
        algorithm: HashAlgorithm,
        text_or_binary: TextOrBinary,
    ) -> Result<ContentDigest> {
        let abs_path = self.to_absolute_path(xvc_root);

        ContentDigest::new(&abs_path, algorithm, text_or_binary)
    }

    /// Return all parent directories of an xvcpath
    pub fn parents(&self) -> Vec<Self> {
        let mut parents = Vec::new();

        let mut rp: &RelativePath = &self.0;

        while let Some(parent) = rp.parent() {
            if !parent.as_str().is_empty() {
                parents.push(Self(parent.to_relative_path_buf()));
                rp = parent;
            } else {
                break;
            }
        }

        parents
    }

    /// Joins two paths
    ///
    /// ```
    /// use xvc_core::XvcPath;
    /// use relative_path::RelativePathBuf;
    ///
    /// let path = XvcPath::from(RelativePathBuf::from("a/b/c"));
    /// let other = XvcPath::from(RelativePathBuf::from("d/e/f"));
    /// let joined = path.join(&other).unwrap();
    /// assert_eq!(joined, XvcPath::from(RelativePathBuf::from("a/b/c/d/e/f")));
    /// ```
    ///
    pub fn join(&self, other: &XvcPath) -> Result<XvcPath> {
        Ok(XvcPath(self.0.join(&other.0)))
    }

    /// Join only the file name portion of the other XvcPath
    /// ```
    /// use xvc_core::XvcPath;
    /// use relative_path::RelativePathBuf;
    ///
    /// let path = XvcPath::from(RelativePathBuf::from("a/b/c"));
    /// let other = XvcPath::from(RelativePathBuf::from("d/e/f"));
    /// let joined = path.join_file_name(&other).unwrap();
    /// assert_eq!(joined, XvcPath::from(RelativePathBuf::from("a/b/c/f")));
    /// ```
    pub fn join_file_name(&self, other: &XvcPath) -> Result<XvcPath> {
        let other_name = other
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("other path doesn't have a file name"))?;

        Ok(XvcPath(self.0.join(other_name)))
    }

    /// Returns the file name of the path
    pub fn file_name(&self) -> Option<&str> {
        self.0.file_name()
    }
}

/// Represents whether a file is a text file or not
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    EnumString,
    Hash,
    Display,
    Copy,
)]
#[strum(serialize_all = "lowercase")]
pub enum TextOrBinary {
    /// Detect whether the file is text or binary with [is_text_file] function
    Auto,
    /// Remove all line endings before calculating the digest
    Text,
    /// Do not remove line endings before calculating the digest
    Binary,
}

impl Default for TextOrBinary {
    fn default() -> Self {
        Self::Auto
    }
}

persist!(TextOrBinary, "text-or-binary");

/// Cache paths are relative to `.xvc/`
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, DeriveDisplay,
)]
pub struct XvcCachePath(RelativePathBuf);
// We don't persist this for the time being
// It can be constructed from [ContentDigest] when required.
// persist!(XvcCachePath, "cache-path");

impl XvcCachePath {
    /// Construct a new cache path for the given `xvc_path` and `content_digest`.
    ///
    /// [ContentDigest] must contain an [XvcDigest], otherwise it returns an error.
    pub fn new(xvc_path: &XvcPath, content_digest: &ContentDigest) -> Result<Self> {
        let content_digest_dir = content_digest.digest().cache_dir();
        let content_digest_filename = format!("0.{}", xvc_path.extension().unwrap_or(""));
        Ok(Self(RelativePathBuf::from(format!(
            "{}/{}",
            content_digest_dir, content_digest_filename
        ))))
    }

    /// Convert the relative path to absolute
    pub fn to_absolute_path(&self, xvc_root: &XvcRoot) -> AbsolutePath {
        AbsolutePath::from(self.0.to_path(xvc_root.xvc_dir()))
    }

    /// The directory portion without the final part after the last `/`
    pub fn directory(&self) -> RelativePathBuf {
        self.0
            .parent()
            .map(|rp| rp.to_relative_path_buf())
            .unwrap_or(RelativePathBuf::from(""))
    }

    ///  Create a custom relative path used for cache global files (like temporary guid files.)
    pub fn custom(relative_path: &str) -> Self {
        Self(RelativePathBuf::from(relative_path))
    }

    /// Returns the clone of inner relativepathbuf for processing.
    /// e.g. in [xvc_storage::XvcStorageTempDir] to create temporary cache files.
    pub fn inner(&self) -> RelativePathBuf {
        self.0.clone()
    }

    /// Returns the prefix for this digest for reporting purposes
    /// len is the number of digits to return from
    pub fn digest_string(&self, len: usize) -> String {
        self.0
            .to_string()
            .chars()
            .take(len)
            // Convert path separators to '-'
            .map(|c| if c == '/' { '-' } else { c })
            .collect()
    }

    /// Remove a path from the cache.
    /// Removes all empty parent directories of the file as well.
    // TODO: Remove this when we set unix permissions in platform dependent fashion
    #[allow(clippy::permissions_set_readonly_false)]
    pub fn remove(&self, output_snd: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<()> {
        let abs_cp = self.to_absolute_path(xvc_root);
        watch!(abs_cp);
        if abs_cp.exists() {
            // Set to writable
            let parent = abs_cp.parent().unwrap();
            watch!(parent);
            let mut dir_perm = parent.metadata()?.permissions();
            dir_perm.set_readonly(false);
            fs::set_permissions(parent, dir_perm)?;

            let mut file_perm = abs_cp.metadata()?.permissions();
            file_perm.set_readonly(false);
            fs::set_permissions(&abs_cp, file_perm)?;

            uwr!(fs::remove_file(&abs_cp), output_snd);
            output!(output_snd, "[DELETE] {}", abs_cp.to_str().unwrap());
        }

        let mut rel_path = self.inner();
        watch!(rel_path);
        while let Some(parent) = rel_path.parent() {
            let parent_abs_cp = parent.to_logical_path(xvc_root.xvc_dir());
            watch!(parent_abs_cp);
            if parent_abs_cp.exists()
                && parent_abs_cp.is_dir()
                && parent_abs_cp.read_dir().unwrap().count() == 0
            {
                let mut perm = parent_abs_cp.metadata()?.permissions();
                perm.set_readonly(false);
                fs::set_permissions(&parent_abs_cp, perm)?;
                uwr!(fs::remove_dir(&parent_abs_cp), output_snd);
                output!(output_snd, "[DELETE] {}", parent_abs_cp.to_str().unwrap());
            }
            rel_path = parent.to_relative_path_buf();
        }

        Ok(())
    }
}

impl AsRef<RelativePath> for XvcCachePath {
    fn as_ref(&self) -> &RelativePath {
        self.0.as_relative_path()
    }
}
