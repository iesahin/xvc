use crate::util::file::is_text_file;
use crate::{attribute_digest, TextOrBinary, XvcDigest};
use crate::{types::hashalgorithm::HashAlgorithm};



use std::{fmt::Display, path::Path};


use crate::error::Result;
use blake2::{Digest};


use serde::{Deserialize, Serialize};



use super::AttributeDigest;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ContentDigest(XvcDigest);
attribute_digest!(ContentDigest, "content-digest");
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
}

impl Display for ContentDigest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
