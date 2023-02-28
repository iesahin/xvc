use crate::util::file::is_text_file;
use crate::{types::hashalgorithm::HashAlgorithm, XvcPathMetadataMap};
use crate::{TextOrBinary, XvcDigest, XvcMetadata};
use reqwest::Url;
use std::collections::{HashMap, HashSet};
use std::time::SystemTime;
use std::{fmt::Display, fs, path::Path};
use xvc_ecs::{persist, Storable, XvcStore};

use crate::error::Result;
use blake2::{Blake2s, Digest};
use relative_path::RelativePathBuf;
use reqwest::blocking::Client as HttpClient;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sha3::Sha3_256;

use super::AttributeDigest;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ContentDigest(XvcDigest);
persist!(ContentDigest, "content-digest");
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

impl AttributeDigest for ContentDigest {
    fn attribute(_: Self) -> String {
        "content-digest".to_string()
    }
    fn digest(&self) -> XvcDigest {
        self.0
    }
}

impl From<XvcDigest> for ContentDigest {
    fn from(digest: XvcDigest) -> Self {
        Self(digest)
    }
}
