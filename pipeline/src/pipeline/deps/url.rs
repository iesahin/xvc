//! A step dependency to a URL
use crate::{Result, XvcDependency};

use reqwest::blocking::Client as HttpClient;
use serde::{Deserialize, Serialize};
use url::Url;
use xvc_core::types::diff::Diffable;
use xvc_core::{Diff, HashAlgorithm, UrlContentDigest};
use xvc_ecs::persist;
///
/// Invalidates when header of the URL get request changes.
#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UrlDigestDep {
    /// URL like https://example.com/my-file.html
    pub url: Url,
    /// ETag header from the HEAD request
    pub etag: Option<String>,
    /// Last-Modified header from the HEAD request
    pub last_modified: Option<String>,
    /// Digest of the content from the URL
    pub url_content_digest: Option<UrlContentDigest>,
}

persist!(UrlDigestDep, "url-dependency");

impl From<UrlDigestDep> for XvcDependency {
    fn from(val: UrlDigestDep) -> Self {
        XvcDependency::UrlDigest(val)
    }
}

impl UrlDigestDep {
    /// Create a new URL dependency with the given URL and empty headers and content digest.
    pub fn new(url: Url) -> Self {
        Self {
            url,
            etag: None,
            last_modified: None,
            url_content_digest: None,
        }
    }

    /// Make a HEAD request and fill Etag and Last-Modified headers.
    pub fn update_headers(self) -> Result<Self> {
        let client = HttpClient::new();
        let response = client.head(self.url.as_str()).send()?.error_for_status()?;
        let headers = response.headers();

        let etag = headers.get("ETag").map(|s| s.to_str().unwrap().to_string());

        let last_modified = headers
            .get("Last-Modified")
            .map(|s| s.to_str().unwrap().to_string());
        Ok(Self {
            etag,
            last_modified,
            ..self
        })
    }

    /// Make a GET request, download the content and fill the content digest.
    pub fn update_content_digest(self) -> Result<Self> {
        let url_get_digest = Some(UrlContentDigest::new(&self.url, HashAlgorithm::Blake3)?);
        Ok(Self {
            url_content_digest: url_get_digest,
            ..self
        })
    }
}

impl Diffable for UrlDigestDep {
    type Item = UrlDigestDep;

    /// ⚠️ Call actual.update_headers before calling this. ⚠️
    fn diff_superficial(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        assert!(record.url == actual.url);

        if actual.etag.is_none() && actual.last_modified.is_none() {
            panic!("No ETag or Last-Modified header found in response")
        } else {
            match (
                &actual.etag,
                &actual.last_modified,
                &record.etag,
                &record.last_modified,
            ) {
                (None, None, _, _) => unreachable!("We already checked for this"),
                (None, Some(_), None, None) => Diff::RecordMissing {
                    actual: actual.clone(),
                },
                (None, Some(act), None, Some(rec)) => {
                    if *act == *rec {
                        Diff::Identical
                    } else {
                        Diff::Different {
                            record: record.clone(),
                            actual: actual.clone(),
                        }
                    }
                }
                // Headers changed
                (None, Some(_), Some(_), None) => Diff::Different {
                    record: record.clone(),
                    actual: actual.clone(),
                },
                (None, Some(_), Some(_), Some(_)) => Diff::Different {
                    record: record.clone(),
                    actual: actual.clone(),
                },
                (Some(_), None, None, None) => Diff::RecordMissing {
                    actual: actual.clone(),
                },
                (Some(_), None, None, Some(_)) => Diff::Different {
                    record: record.clone(),
                    actual: actual.clone(),
                },
                (Some(act), None, Some(rec), None) => {
                    if act == rec {
                        Diff::Identical
                    } else {
                        Diff::Different {
                            record: record.clone(),
                            actual: actual.clone(),
                        }
                    }
                }

                (Some(_), None, Some(_), Some(_)) => Diff::Different {
                    record: record.clone(),
                    actual: actual.clone(),
                },
                (Some(_), Some(_), None, None) => Diff::RecordMissing {
                    actual: actual.clone(),
                },
                (Some(_), Some(_), None, Some(_)) => Diff::Different {
                    record: record.clone(),
                    actual: actual.clone(),
                },
                (Some(_), Some(_), Some(_), None) => Diff::Different {
                    record: record.clone(),
                    actual: actual.clone(),
                },
                (Some(act_etag), Some(act_lm), Some(rec_etag), Some(rec_lm)) => {
                    if act_etag == rec_etag && act_lm == rec_lm {
                        Diff::Identical
                    } else {
                        Diff::Different {
                            record: record.clone(),
                            actual: actual.clone(),
                        }
                    }
                }
            }
        }
    }

    /// ⚠️ Call actual.update_content_digest before calling this. ⚠️
    fn diff_thorough(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        match (record.url_content_digest, actual.url_content_digest) {
            (None, None) => unreachable!("Both record and actual url content digests are None."),
            (None, Some(_)) => Diff::RecordMissing {
                actual: actual.clone(),
            },
            (Some(_), None) => Diff::ActualMissing {
                record: record.clone(),
            },
            (Some(rec), Some(act)) => {
                if rec == act {
                    Diff::Identical
                } else {
                    Diff::Different {
                        record: record.clone(),
                        actual: actual.clone(),
                    }
                }
            }
        }
    }

    /// ⚠️ Call actual.update_content_digest before calling this. ⚠️
    fn diff(record: Option<&Self::Item>, actual: Option<&Self::Item>) -> Diff<Self::Item> {
        match (record, actual) {
            (None, None) => unreachable!("We should never be diffing None with None"),
            (None, Some(actual)) => Diff::RecordMissing {
                actual: actual.clone(),
            },
            (Some(record), None) => Diff::ActualMissing {
                record: record.clone(),
            },
            (Some(record), Some(actual)) => Self::diff_thorough(record, actual),
        }
    }
}
