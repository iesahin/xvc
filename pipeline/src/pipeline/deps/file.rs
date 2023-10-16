//! File dependencies for a pipeline step
use std::ffi::OsString;

use crate::error::Error;
use crate::{Result, XvcDependency};
use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{
    ContentDigest, Diff, HashAlgorithm, TextOrBinary, XvcMetadata, XvcPath, XvcPathMetadataMap,
    XvcRoot,
};
use xvc_ecs::persist;

/// A file dependency for a pipeline step.
/// It keeps track of path, metadata and the digest of the file.
#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileDep {
    /// The path in the workspace
    pub path: XvcPath,
    /// [XvcMetadata] of the file if it's available
    pub xvc_metadata: Option<XvcMetadata>,
    /// [ContentDigest] of the file if it's available
    pub content_digest: Option<ContentDigest>,
}
persist!(FileDep, "file-dependency");

impl From<FileDep> for XvcDependency {
    fn from(val: FileDep) -> Self {
        XvcDependency::File(val)
    }
}

impl FileDep {
    /// Create a new file dependency with metadata and digest set to None.
    pub fn new(path: XvcPath) -> Self {
        Self {
            path,
            xvc_metadata: None,
            content_digest: None,
        }
    }

    /// Create a file dependency with the given path and metadata obtained from [XvcPathMetadataMap]
    pub fn from_pmm(path: &XvcPath, pmm: &XvcPathMetadataMap) -> Result<Self> {
        let path = path.clone();
        let xvc_metadata = pmm.get(&path).cloned();
        if xvc_metadata.is_none() {
            return Err(Error::PathNotFound {
                path: OsString::from(path.as_str()),
            });
        }

        Ok(FileDep {
            path,
            xvc_metadata,
            content_digest: None,
        })
    }

    /// Returns a new instance with the updated content digest
    pub fn calculate_content_digest(
        self,
        xvc_root: &XvcRoot,
        algorithm: HashAlgorithm,
        text_or_binary: TextOrBinary,
    ) -> Result<Self> {
        let path = self.path.to_absolute_path(xvc_root);
        let content_digest = ContentDigest::new(&path, algorithm, text_or_binary)?;
        Ok(Self {
            content_digest: Some(content_digest),
            ..self
        })
    }

    /// Returns a new instance with a new content digest if the metadata has changed
    pub fn calculate_content_digest_if_changed(
        self,
        xvc_root: &XvcRoot,
        record: &Self,
        algorithm: HashAlgorithm,
        text_or_binary: TextOrBinary,
    ) -> Result<Self> {
        if Self::diff_superficial(record, &self).changed() {
            self.calculate_content_digest(xvc_root, algorithm, text_or_binary)
        } else {
            Ok(Self {
                content_digest: record.content_digest,
                ..self
            })
        }
    }
}

impl Diffable for FileDep {
    type Item = Self;

    fn diff_superficial(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        assert!(record.path == actual.path);
        match (record.xvc_metadata, actual.xvc_metadata) {
            (None, None) => unreachable!("Both record and actual are None"),
            (None, Some(_)) => Diff::RecordMissing {
                actual: actual.clone(),
            },
            (Some(_), None) => Diff::ActualMissing {
                record: record.clone(),
            },
            (Some(rec_metadata_digest), Some(act_metadata_digest)) => {
                if rec_metadata_digest == act_metadata_digest {
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
    /// ⚠️ Call actual.update_metadata and actual.calculate_content_digest_if_changed before calling this ⚠️
    fn diff_thorough(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        assert!(record.path == actual.path);
        match (record.content_digest, actual.content_digest) {
            (None, None) => unreachable!("Both record and actual content digests are None"),
            (None, Some(_)) => Diff::RecordMissing {
                actual: actual.clone(),
            },
            (Some(_), None) => Diff::ActualMissing {
                record: record.clone(),
            },
            (Some(rec_content_digest), Some(act_content_digest)) => {
                if rec_content_digest == act_content_digest {
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
