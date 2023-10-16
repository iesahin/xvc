//! Digest of file content
use crate::types::diff::Diffable;
use crate::types::hashalgorithm::HashAlgorithm;
use crate::util::file::is_text_file;
use crate::{attribute_digest, TextOrBinary, XvcDigest};

use std::{fmt::Display, path::Path};

use crate::error::Result;

use serde::{Deserialize, Serialize};

/// Digest for the content of a file.
///
/// It's calculated by reading the file contents and hashing them with the given algortthm.
/// The digest is stable, i.e. it doesn't change if the file is moved or renamed.
/// The digest is not affected by the file's path.
/// If the file is a text file (see [`is_text_file`]), the digest is calculated by
/// removing line endings before hashing.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ContentDigest(XvcDigest);
attribute_digest!(ContentDigest, "content-digest");

impl Diffable for ContentDigest {
    type Item = ContentDigest;

    fn diff_superficial(record: &Self::Item, actual: &Self::Item) -> crate::Diff<Self::Item> {
        if record == actual {
            crate::Diff::Identical
        } else {
            crate::Diff::Different {
                record: *record,
                actual: *actual,
            }
        }
    }

    fn diff_thorough(record: &Self::Item, actual: &Self::Item) -> crate::Diff<Self::Item> {
        Self::diff_superficial(record, actual)
    }
}

impl ContentDigest {
    /// Returns the content hash of the file in `path` calculated by `algorithm`.
    ///
    /// If `text_or_binary` is `TextOrBinary::Auto`, the file is checked for
    /// text-ness by [`is_text_file`] and the appropriate digest is returned.
    /// Otherwise the digest is calculated as text or binary, via
    /// [`XvcDigest::from_text_file`] or [`XvcDigest::from_binary_file`].
    pub fn new(
        path: &Path,
        algorithm: HashAlgorithm,
        text_or_binary: TextOrBinary,
    ) -> Result<Self> {
        let digest = match text_or_binary {
            TextOrBinary::Binary => XvcDigest::from_binary_file(path, algorithm)?,
            TextOrBinary::Text => XvcDigest::from_text_file(path, algorithm)?,
            TextOrBinary::Auto => {
                if is_text_file(path)? {
                    XvcDigest::from_text_file(path, algorithm)?
                } else {
                    XvcDigest::from_binary_file(path, algorithm)?
                }
            }
        };
        Ok(Self(digest))
    }
    /// Return the inner digest
    pub fn digest(&self) -> XvcDigest {
        self.0
    }
}

impl Display for ContentDigest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
