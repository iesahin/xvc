use crate::{Result, XvcDependency};
use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{
    ContentDigest, Diff, HashAlgorithm, TextOrBinary, XvcMetadata, XvcMetadataDigest, XvcPath,
    XvcPathMetadataMap, XvcRoot,
};
use xvc_ecs::persist;
use xvc_logging::watch;

#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileDep {
    /// The path in the workspace
    pub path: XvcPath,
    pub xvc_metadata: Option<XvcMetadata>,
    pub content_digest: Option<ContentDigest>,
}
persist!(FileDep, "file-dependency");

impl Into<XvcDependency> for FileDep {
    fn into(self) -> XvcDependency {
        XvcDependency::File(self)
    }
}

impl FileDep {
    pub fn new(path: XvcPath) -> Self {
        Self {
            path,
            xvc_metadata: None,
            content_digest: None,
        }
    }

    pub fn from_pmm(path: &XvcPath, pmm: &XvcPathMetadataMap) -> Self {
        let path = path.clone();
        let xvc_metadata = pmm.get(&path).cloned();
        watch!(xvc_metadata);

        FileDep {
            path,
            xvc_metadata,
            content_digest: None,
        }
    }

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
                content_digest: record.content_digest.clone(),
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
