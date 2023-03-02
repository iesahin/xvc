use crate::util::file::is_text_file;
use crate::{attribute_digest, TextOrBinary, XvcDigest, XvcMetadata};
use crate::{types::hashalgorithm::HashAlgorithm, XvcPathMetadataMap};
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
/// Returns a stable digest of the content of a URL.

pub struct UrlGetDigest(XvcDigest);
attribute_digest!(UrlGetDigest, "url-get-digest");

impl UrlGetDigest {
    pub fn new(url: &Url, algorithm: HashAlgorithm) -> Result<Self> {
        let response = HttpClient::new()
            .get(url.as_str())
            .send()?
            .error_for_status()?
            .text()?;

        Ok(Self(XvcDigest::from_content(&response, algorithm)))
    }
}
