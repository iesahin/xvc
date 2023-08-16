use crate::{Result, XvcMetricsFormat};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{
    ContentDigest, Diff, HashAlgorithm, PathCollectionDigest, TextOrBinary, XvcDigest, XvcPath,
    XvcPathMetadataMap, XvcRoot,
};
use xvc_ecs::persist;

use crate::XvcDependency;

/// Invalidates when contents of any of the files in the directory changes.
#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GlobDep {
    /// The path in the workspace
    pub glob: String,
    pub xvc_paths_digest: Option<PathCollectionDigest>,
    pub xvc_metadata_digest: Option<PathCollectionMetadataDigest>,
    pub content_digest: Option<PathCollectionContentDigest>,
}

#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PathCollectionMetadataDigest(XvcDigest);

persist!(
    PathCollectionMetadataDigest,
    "path-collection-metadata-digest"
);

impl Diffable for PathCollectionMetadataDigest {
    type Item = Self;
}

#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PathCollectionContentDigest(XvcDigest);
persist!(
    PathCollectionContentDigest,
    "path-collection-content-digest"
);
impl Diffable for PathCollectionContentDigest {
    type Item = Self;
}

persist!(GlobDep, "glob-digest-dependency");

impl Into<XvcDependency> for GlobDep {
    fn into(self) -> XvcDependency {
        XvcDependency::Glob(self)
    }
}

impl GlobDep {
    pub fn new(glob: String) -> Self {
        Self {
            glob,
            xvc_paths_digest: None,
            xvc_metadata_digest: None,
            content_digest: None,
        }
    }

    pub fn update_collection_digests(self, pmm: &XvcPathMetadataMap) -> Result<Self> {
        let compiled_glob = glob::Pattern::new(&self.glob)?;
        let paths = pmm
            .iter()
            .filter(|(xp, _)| compiled_glob.matches(xp.as_str()))
            .sorted();

        let xvc_paths_digest = Some(PathCollectionDigest::new(
            paths.clone(),
            HashAlgorithm::Blake3,
        )?);
        let xvc_metadata_digest_bytes =
            paths
                .map(|(_, xmd)| xmd.digest().unwrap())
                .fold(Vec::<u8>::new(), |mut acc, xmd| {
                    acc.extend(xmd.digest().digest);
                    acc
                });
        let xvc_metadata_digest = Some(PathCollectionMetadataDigest(XvcDigest::from_bytes(
            &xvc_metadata_digest_bytes,
            HashAlgorithm::Blake3,
        )));

        Ok(Self {
            xvc_paths_digest,
            xvc_metadata_digest,
            ..self
        })
    }

    pub fn update_content_digest(
        self,
        xvc_root: &XvcRoot,
        pmm: &XvcPathMetadataMap,
    ) -> Result<Self> {
        let compiled_glob = glob::Pattern::new(&self.glob)?;
        let paths = pmm
            .iter()
            .filter(|(xp, _)| compiled_glob.matches(xp.as_str()))
            .sorted();

        let content_digest_bytes = paths
            .map(|(xp, _)| {
                let path = xp.to_absolute_path(xvc_root);
                let cd =
                    ContentDigest::new(&path, HashAlgorithm::Blake3, TextOrBinary::Auto).unwrap();
                cd.digest().digest
            })
            .fold(Vec::<u8>::new(), |mut acc, bytes| {
                acc.extend(bytes);
                acc
            });

        let content_digest = Some(PathCollectionContentDigest(XvcDigest::from_bytes(
            &content_digest_bytes,
            HashAlgorithm::Blake3,
        )));

        Ok(Self {
            content_digest,
            ..self
        })
    }
}

impl Diffable for GlobDep {
    type Item = Self;

    fn diff_superficial(record: &Self::Item, actual: &Self::Item) -> xvc_core::Diff<Self::Item> {
        assert!(record.glob == actual.glob);

        if PathCollectionDigest::diff(
            record.xvc_paths_digest.as_ref(),
            actual.xvc_paths_digest.as_ref(),
        )
        .changed()
            || PathCollectionMetadataDigest::diff(
                record.xvc_metadata_digest.as_ref(),
                actual.xvc_metadata_digest.as_ref(),
            )
            .changed()
        {
            Diff::Different {
                record: record.clone(),
                actual: actual.clone(),
            }
        } else {
            Diff::Skipped
        }
    }

    fn diff_thorough(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        assert!(record.glob == actual.glob);

        if PathCollectionDigest::diff(
            record.xvc_paths_digest.as_ref(),
            actual.xvc_paths_digest.as_ref(),
        )
        .changed()
            || PathCollectionMetadataDigest::diff(
                record.xvc_metadata_digest.as_ref(),
                actual.xvc_metadata_digest.as_ref(),
            )
            .changed()
            || PathCollectionContentDigest::diff(
                record.content_digest.as_ref(),
                actual.content_digest.as_ref(),
            )
            .changed()
        {
            Diff::Different {
                record: record.clone(),
                actual: actual.clone(),
            }
        } else {
            Diff::Skipped
        }
    }

    fn diff(record: Option<&Self::Item>, actual: Option<&Self::Item>) -> Diff<Self::Item> {
        match (record, actual) {
            (Some(record), Some(actual)) => Self::diff_thorough(record, actual),
            (None, Some(actual)) => Diff::RecordMissing {
                actual: actual.clone(),
            },
            (Some(record), None) => Diff::ActualMissing {
                record: record.clone(),
            },
            (None, None) => unreachable!("Both record and actual are None"),
        }
    }
}
