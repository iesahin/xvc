//! Digest from contents of a GET request to a URL.
use crate::types::hashalgorithm::HashAlgorithm;
use crate::{attribute_digest, XvcDigest};
use reqwest::Url;

use crate::error::Result;

use reqwest::blocking::Client as HttpClient;
use serde::{Deserialize, Serialize};

use super::AttributeDigest;

/// Returns a stable digest of the content of a URL.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UrlContentDigest(XvcDigest);
attribute_digest!(UrlContentDigest, "url-get-digest");

impl UrlContentDigest {
    /// Make a GET request to `url` and return a stable digest of the response.
    pub fn new(url: &Url, algorithm: HashAlgorithm) -> Result<Self> {
        let response = HttpClient::new()
            .get(url.as_str())
            .send()?
            .error_for_status()?
            .text()?;

        Ok(Self(XvcDigest::from_content(&response, algorithm)))
    }
}
