use crate::Result;
use anyhow::anyhow;
use reqwest::blocking::Client as HttpClient;
use serde::{Deserialize, Serialize};
use url::Url;
use xvc_core::types::diff::Diffable;
use xvc_core::{Diff, HashAlgorithm, UrlGetDigest};
use xvc_ecs::persist;
///
/// Invalidates when header of the URL get request changes.
#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UrlDep {
    /// URL like https://example.com/my-file.html
    pub url: Url,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub url_get_digest: Option<UrlGetDigest>,
}

persist!(UrlDep, "url-dependency");

impl UrlDep {
    pub fn new(url: Url) -> Self {
        Self {
            url,
            etag: None,
            last_modified: None,
            url_get_digest: None,
        }
    }

    pub fn update_headers(self) -> Result<Self> {
        let client = HttpClient::new();
        let response = client.head(self.url.as_str()).send()?.error_for_status()?;
        let headers = response.headers();
        let mut response = String::new();

        let etag = headers.get("ETag").map(|s| s.to_str().to_string().unwrap());
        let last_modified = headers
            .get("Last-Modified")
            .map(|s| s.to_str().to_string().unwrap());
        Ok(Self {
            etag,
            last_modified,
            ..self
        })
    }

    pub fn calculate_url_get_digest(self) -> Result<Self> {
        let url_get_digest = Some(UrlGetDigest::new(&self.url, HashAlgorithm::Blake3)?);
        Ok(Self {
            url_get_digest,
            ..self
        })
    }
}

impl Diffable for UrlDep {
    type Item = UrlDep;

    fn diff_superficial(record: Self::Item, actual: Self::Item) -> xvc_core::Diff<Self::Item> {
        assert!(record.url == actual.url);

        if actual.etag.is_none() && actual.last_modified.is_none() {
            Err(anyhow!("No ETag or Last-Modified header found in response").into())
        } else {
            match (
                actual.etag,
                actual.last_modified,
                record.etag,
                record.last_modified,
            ) {
                (None, None, _, _) => unreachable!("We already checked for this"),
                (None, Some(_), None, None) => Diff::RecordMissing { actual },
                (None, Some(act), None, Some(rec)) => {
                    if *act == rec {
                        Diff::Identical
                    } else {
                        Diff::Different { record, actual }
                    }
                }
                // Headers changed
                (None, Some(_), Some(_), None) => Diff::Different { record, actual },
                (None, Some(_), Some(_), Some(_)) => Diff::Different { record, actual },
                (Some(_), None, None, None) => Diff::RecordMissing { actual },
                (Some(_), None, None, Some(_)) => Diff::Different { record, actual },
                (Some(act), None, Some(rec), None) => {
                    if act == rec {
                        Diff::Identical
                    } else {
                        Diff::Different { record, actual }
                    }
                }

                (Some(_), None, Some(_), Some(_)) => Diff::Different { record, actual },
                (Some(_), Some(_), None, None) => Diff::RecordMissing { actual },
                (Some(_), Some(_), None, Some(_)) => Diff::Different { record, actual },
                (Some(_), Some(_), Some(_), None) => Diff::Different { record, actual },
                (Some(act_etag), Some(act_lm), Some(rec_etag), Some(rec_lm)) => {
                    if act_etag == rec_etag && act_lm == rec_lm {
                        Diff::Identical
                    } else {
                        Diff::Different { record, actual }
                    }
                }
            }
        }
    }

    fn diff_thorough(record: Self::Item, actual: Self::Item) -> Diff<Self::Item> {
        match Self::diff_superficial(record, actual) {
            Diff::Identical => Diff::Identical,
            Diff::Skipped => Diff::Skipped,
            Diff::RecordMissing { actual } => {
                let url_get_digest = Some(UrlGetDigest::new(&actual.url, HashAlgorithm::Blake3)?);
                let actual = Self {
                    url_get_digest,
                    ..actual
                };
                Diff::RecordMissing { actual }
            }
            Diff::ActualMissing { record } => Diff::ActualMissing { record },
            Diff::Different { record, actual } => {
                let url_get_digest = Some(UrlGetDigest::new(&actual.url, HashAlgorithm::Blake3)?);
                let actual = Self {
                    url_get_digest,
                    ..actual
                };
                Diff::Different { record, actual }
            }
        }
    }

    fn diff(record: Option<Self::Item>, actual: Option<Self::Item>) -> Diff<Self::Item> {
        match (record, actual) {
            (None, None) => unreachable!("We should never be diffing None with None"),
            (None, Some(actual)) => Diff::RecordMissing { actual },
            (Some(record), None) => Diff::ActualMissing { record },
            (Some(record), Some(actual)) => Self::diff_thorough(record, actual),
        }
    }
}
