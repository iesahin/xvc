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
/// Returns a stable digest of the list of paths.
pub struct CollectionDigest(XvcDigest);
persist!(CollectionDigest, "collection-digest");
impl CollectionDigest {
    pub fn new(paths: &XvcPathMetadataMap, algorithm: HashAlgorithm) -> Result<Self> {
        let paths_str = paths.keys().fold("".to_string(), |mut s, xp| {
            s.push_str(xp.as_ref());
            s
        });

        Ok(Self(XvcDigest::from_content(&paths_str, algorithm)))
    }
}

impl AttributeDigest for CollectionDigest {
    fn attribute(_: Self) -> String {
        "collection-digest".to_string()
    }

    fn digest(&self) -> XvcDigest {
        self.0
    }
}

impl From<XvcDigest> for CollectionDigest {
    fn from(digest: XvcDigest) -> Self {
        Self(digest)
    }
}
