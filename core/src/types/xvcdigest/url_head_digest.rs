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

/// Returns a digest from HTTP HEAD request to URL
/// Uses reqwest blocking API not to require
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct UrlHeadDigest(XvcDigest);
persist!(UrlHeadDigest, "url-head-digest");

impl UrlHeadDigest {
    pub fn new(url: &Url, algorithm: HashAlgorithm) -> Result<Self> {
        let client = HttpClient::new();
        let headers = client
            .head(url.as_str())
            .send()?
            .error_for_status()?
            .headers();
        let mut response = String::new();

        // TODO: We can make this configurable if other fields are also important.
        response.push_str(
            headers
                .get("ETag")
                .map(|s| s.to_str().unwrap())
                .unwrap_or(""),
        );
        response.push_str(
            headers
                .get("Last-Modified")
                .map(|s| s.to_str().unwrap())
                .unwrap_or(""),
        );

        Ok(Self(XvcDigest::from_content(&response, algorithm)))
    }
}

impl AttributeDigest for UrlHeadDigest {
    fn attribute(_: Self) -> String {
        "url-head-digest".to_string()
    }
    fn digest(&self) -> XvcDigest {
        self.0
    }
}

impl From<XvcDigest> for UrlHeadDigest {
    fn from(digest: XvcDigest) -> Self {
        Self(digest)
    }
}
