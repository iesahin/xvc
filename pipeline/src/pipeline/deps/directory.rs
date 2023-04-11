use std::collections::BTreeMap;

use crate::Result;
use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{
    ContentDigest, Diff, PathCollectionDigest, XvcMetadata, XvcMetadataDigest, XvcPath, XvcRoot,
};
use xvc_ecs::persist;
use xvc_walker::Glob;

/// Invalidates when contents of any of the files in the directory changes.
#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DirectoryDep {
    /// The path in the workspace
    pub path: XvcPath,
    pub xvc_path_metadata_map: BTreeMap<XvcPath, XvcMetadata>,
    pub xvc_path_content_digest_map: BTreeMap<XvcPath, ContentDigest>,
}

persist!(DirectoryDep, "directory-dependency");

impl DirectoryDep {
    pub fn new(path: XvcPath) -> Self {
        Self {
            path,
            xvc_path_metadata_map: BTreeMap::new(),
            xvc_path_content_digest_map: BTreeMap::new(),
        }
    }

    pub(crate) fn from_pmm(
        xvc_root: &XvcRoot,
        path: XvcPath,
        pmm: &std::collections::HashMap<XvcPath, xvc_core::XvcMetadata>,
    ) -> Self {
        let xvc_path_metadata_map: BTreeMap<XvcPath, XvcMetadata> =
            pmm.into_iter().filter(|(xp, xmd)| xp.starts_with(&path));
        // We don't calculate the content digest map immediately, we only do that in through comparison
        Self {
            path,
            xvc_path_metadata_map,
            xvc_path_content_digest_map: BTreeMap::new(),
        }
    }

    pub fn calculate_digests(&mut self, xvc_root: &XvcRoot) -> Result<()> {
        for (xvc_path, xvc_metadata) in self.xvc_path_metadata_map.iter() {
            let content_digest = xvc_path.content_digest(xvc_root)?;
            self.xvc_path_content_digest_map
                .insert(xvc_path.clone(), content_digest);
        }
        Ok(())
    }

    pub fn calculate_changed_paths_digests(
        &mut self,
        xvc_root: &XvcRoot,
        record: &Self,
    ) -> Result<()> {
        for (xvc_path, xvc_metadata) in self.xvc_path_metadata_map.iter() {
            let record_metadata = record.xvc_path_metadata_map.get(xvc_path);
            let content_digest = if XvcMetadata::diff(record_metadata, Some(xvc_metadata)).changed()
            {
                xvc_path.content_digest(xvc_root)?;
            } else {
                record
                    .xvc_path_content_digest_map
                    .get(xvc_path)
                    .unwrap()
                    .clone()
            };

            self.xvc_path_content_digest_map
                .insert(xvc_path.clone(), content_digest);
        }
        Ok(())
    }
}

impl Diffable for DirectoryDep {
    type Item = DirectoryDep;

    fn diff_superficial(record: Self::Item, actual: Self::Item) -> Diff<Self::Item> {
        assert!(record.path == actual.path);

        if record.xvc_path_metadata_map == actual.xvc_path_metadata_map {
            return Diff::Identical;
        } else {
            return Diff::Different { record, actual };
        }
    }

    fn diff_thorough(record: Self::Item, actual: Self::Item) -> Diff<Self::Item> {
        assert!(record.path == actual.path);
        let record_paths = record.xvc_path_metadata_map.keys().sorted();

        if record_paths.eq(actual.xvc_path_metadata_map.keys().sorted())
            && record_paths.eq(actual.xvc_path_content_digest_map.keys().sorted())
            && record.xvc_path_content_digest_map == actual.xvc_path_content_digest_map
        {
            return Diff::Identical;
        } else {
            return Diff::Different { record, actual };
        }
    }

    fn diff(record: Option<Self::Item>, actual: Option<Self::Item>) -> Diff<Self::Item> {
        // First we check the actual records
        match (record, actual) {
            (None, None) => unreachable!("Both record and actual are None"),
            (None, Some(actual)) => Diff::RecordMissing { actual },
            (Some(record), None) => Diff::ActualMissing { record },
            (Some(record), Some(actual)) => match Self::diff_superficial(record, actual) {
                Diff::Different { record, actual } => Self::diff_thorough(record, actual),
                diff => diff,
            },
        }
    }
}
