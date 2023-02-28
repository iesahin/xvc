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

pub struct StdoutDigest(XvcDigest);
persist!(StdoutDigest, "stdout-digest");

impl StdoutDigest {
    pub fn new(stdout: &str, algorithm: HashAlgorithm) -> Self {
        Self(XvcDigest::from_content(stdout, algorithm))
    }
}

impl AttributeDigest<StdoutDigest> for StdoutDigest {
    fn attribute() -> String {
        "stdout-digest".to_string()
    }
    fn digest(&self) -> XvcDigest {
        self.0
    }
}

impl From<XvcDigest> for StdoutDigest {
    fn from(digest: XvcDigest) -> Self {
        Self(digest)
    }
}
