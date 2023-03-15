//! Digest of Etag and Last-Modified headers of a URL.
use crate::types::diff::Diffable;
use crate::types::hashalgorithm::HashAlgorithm;
use crate::{attribute_digest, XvcDigest};
use reqwest::Url;

use crate::error::Result;
use blake2::Digest;

use reqwest::blocking::Client as HttpClient;
use serde::{Deserialize, Serialize};

use super::AttributeDigest;

/// Returns a digest from HTTP HEAD request to URL
/// Uses reqwest blocking API not to require
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct UrlHeadDigest(XvcDigest);
attribute_digest!(UrlHeadDigest, "url-head-digest");

impl Diffable<UrlHeadDigest> for UrlHeadDigest {}

impl UrlHeadDigest {
    pub fn new(url: &Url, algorithm: HashAlgorithm) -> Result<Self> {
        let client = HttpClient::new();
        let response = client.head(url.as_str()).send()?.error_for_status()?;
        let headers = response.headers();
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
