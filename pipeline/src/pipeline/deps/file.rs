use crate::Result;
use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{
    ContentDigest, Diff, HashAlgorithm, TextOrBinary, XvcMetadataDigest, XvcPath,
    XvcPathMetadataMap,
};
use xvc_ecs::persist;

#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileDep {
    /// The path in the workspace
    pub path: XvcPath,
    pub xvc_metadata_digest: Option<XvcMetadataDigest>,
    pub content_digest: Option<ContentDigest>,
}
persist!(FileDep, "file-dependency");

impl FileDep {
    pub fn new(path: XvcPath) -> Self {
        Self {
            path,
            xvc_metadata_digest: None,
            content_digest: None,
        }
    }

    pub fn from_pmm(path: &XvcPath, pmm: &XvcPathMetadataMap) -> Self {
        let path = path.clone();
        let xvc_metadata_digest = pmm.get(&path).cloned();

        FileDep {
            path,
            xvc_metadata_digest,
            content_digest: None,
        }
    }

    pub fn calculate_content_digest(
        self,
        algorithm: HashAlgorithm,
        text_or_binary: TextOrBinary,
    ) -> Result<Self> {
        let content_digest = ContentDigest::new(&self.path, algorithm, text_or_binary)?;
        Ok(Self {
            content_digest: Some(content_digest),
            ..self
        })
    }

    pub fn calculate_content_digest_if_changed(
        self,
        record: &Self,
        algorithm: HashAlgorithm,
        text_or_binary: TextOrBinary,
    ) -> Result<Self> {
        if Self::diff_superficial(record, &self).changed() {
            self.calculate_content_digest(algorithm, text_or_binary)
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

    /// Returns identical if xvc_metadata_digests and paths are identical.
    /// This can be used for layered diff, e.g., compare first the metadata and if it returns changed, calculate the
    /// content digest.
    fn diff(record: Option<FileDep>, actual: Option<FileDep>) -> Diff<FileDep> {
        match (record, actual) {
            (None, None) => unreachable!("Both record and actual are None"),
            (None, Some(actual)) => Diff::RecordMissing { actual },
            (Some(record), None) => Diff::ActualMissing { record },
            (Some(record), Some(actual)) => match Self::diff_superficial(record, actual) {
                Diff::Different => Self::diff_thorough(record, actual),
                Diff::Identical => Diff::Identical,
                Diff::RecordMissing { actual } => {
                    let actual = actual.calculate_content_digest_if_changed(
                        &record,
                        HashAlgorithm::Blake3,
                        TextOrBinary::Auto,
                    )?;
                    Diff::RecordMissing { actual }
                }
                Diff::ActualMissing { record } => Diff::ActualMissing { record },
                Diff::Skipped => Diff::Skipped,
            },
        }
    }

    fn diff_superficial(record: Self::Item, actual: Self::Item) -> Diff<Self::Item> {
        assert!(record.path == actual.path);
        match (record.xvc_metadata_digest, actual.xvc_metadata_digest) {
            (None, None) => unreachable!("Both record and actual are None"),
            (None, Some(_)) => Diff::RecordMissing { actual },
            (Some(_), None) => Diff::ActualMissing { record },
            (Some(rec_metadata_digest), Some(act_metadata_digest)) => {
                if rec_metadata_digest == act_metadata_digest {
                    Diff::Identical
                } else {
                    Diff::Different { record, actual }
                }
            }
        }
    }
    fn diff_thorough(record: Self::Item, actual: Self::Item) -> Diff<Self::Item> {
        assert!(record.path == actual.path);

        let actual = actual.calculate_content_digest_if_changed(
            &record,
            HashAlgorithm::Blake3,
            TextOrBinary::Auto,
        )?;

        match (record.content_digest, actual.content_digest) {
            (None, None) => unreachable!("Both record and actual content digests are None"),
            (None, Some(_)) => Diff::RecordMissing { actual },
            (Some(_), None) => Diff::ActualMissing { record },
            (Some(rec_content_digest), Some(act_content_digest)) => {
                if rec_content_digest == act_content_digest {
                    Diff::Identical
                } else {
                    Diff::Different { record, actual }
                }
            }
        }
    }
}
