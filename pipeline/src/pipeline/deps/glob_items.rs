//! A path collection where each item is tracked separately.
use std::collections::BTreeMap;

use crate::{Result, XvcDependency};
use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{
    glob_paths, ContentDigest, Diff, HashAlgorithm, XvcMetadata, XvcPath, XvcPathMetadataProvider, XvcRoot
};
use xvc_ecs::persist;
use xvc_logging::watch;

/// A path collection where each item is tracked separately.
#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GlobItemsDep {
    /// The glob pattern that will be converted to a [Glob]
    pub glob: String,
    /// The metadata of paths that match the glob pattern
    pub xvc_path_metadata_map: BTreeMap<XvcPath, XvcMetadata>,
    /// The content digest of paths that match the glob pattern
    pub xvc_path_content_digest_map: BTreeMap<XvcPath, ContentDigest>,
}

impl From<GlobItemsDep> for XvcDependency {
    fn from(val: GlobItemsDep) -> Self {
        XvcDependency::GlobItems(val)
    }
}

impl GlobItemsDep {
    /// Create a new [GlobItemsDep] with the given glob pattern with empty metadata and digest maps.
    pub fn new(glob: String) -> Self {
        Self {
            glob,
            xvc_path_metadata_map: BTreeMap::new(),
            xvc_path_content_digest_map: BTreeMap::new(),
        }
    }

    /// Create a new [GlobItemsDep] with the given glob pattern and fill the metadata map from the
    /// given pmp. The content digest map in the returned [Self] is empty. 
    pub fn from_pmp(
        glob_root: &XvcPath,
        glob: String,
        pmp: &XvcPathMetadataProvider,
    ) -> Result<GlobItemsDep> {
        let empty = Self::new(glob);
        // We don't calculate the content digest map immediately, we only do that in through comparison
        empty.update_paths(glob_root, pmp)
    }

    /// Update path list by rereading the file list from disk. This doesn't update content digests
    /// of files. Use [Self::update_digests] for this. 
    pub fn update_paths(self, glob_root: &XvcPath, pmp: &XvcPathMetadataProvider) -> Result<Self> {
        watch!(self.xvc_path_metadata_map);
        let xvc_path_metadata_map = 
            glob_paths(pmp, glob_root, &self.glob).map(|paths| paths.into_iter().collect())?;
        watch!(xvc_path_metadata_map);

        Ok(Self {
            xvc_path_metadata_map,
            ..self
        })
    }
        


    /// Update the content digest map for each path in the metadata map. This doesn't update the
    /// file list defined by glob. Use [Self::update_paths] for this.  
    pub fn update_digests(self, xvc_root: &XvcRoot, algorithm: HashAlgorithm) -> Result<Self> {
        let mut xvc_path_content_digest_map = BTreeMap::new();
        for (xvc_path, _xvc_metadata) in self.xvc_path_metadata_map.iter() {
            let path = xvc_path.to_absolute_path(xvc_root);
            let content_digest =
                ContentDigest::new(&path, algorithm, xvc_core::TextOrBinary::Auto)?;
            xvc_path_content_digest_map.insert(xvc_path.clone(), content_digest);
        }
        Ok(Self {
            xvc_path_content_digest_map,
            ..self
        })
    }

    /// Update the content digest map for each path in the metadata map. This doesn't update the
    /// file list defined by glob. Use [Self::update_paths] for this.  
    ///
    /// Calculates content digests when the path metadata is different from record's. This way only
    /// the changed path's content digest is calculated. 
    pub fn update_changed_paths_digests(
        mut self,
        record: &Self,
        xvc_root: &XvcRoot,
        glob_root: &XvcPath,
        pmp: &XvcPathMetadataProvider,
        algorithm: HashAlgorithm,
    ) -> Result<Self> {
        // Update paths to get the new paths and metadata
        self = self.update_paths(glob_root, pmp)?;
        let mut xvc_path_content_digest_map = BTreeMap::new();
        for (xvc_path, xvc_metadata) in self.xvc_path_metadata_map.iter() {
            let record_metadata = record.xvc_path_metadata_map.get(xvc_path);
            let content_digest = if XvcMetadata::diff(record_metadata, Some(xvc_metadata)).changed()
            {
                let path = xvc_path.to_absolute_path(xvc_root);
                ContentDigest::new(&path, algorithm, xvc_core::TextOrBinary::Auto)?
            } else {
                *record.xvc_path_content_digest_map.get(xvc_path).unwrap()
            };

            xvc_path_content_digest_map.insert(xvc_path.clone(), content_digest);
        }
        watch!(xvc_path_content_digest_map);
        Ok(Self {
            xvc_path_content_digest_map,
            ..self
        })
    }
}

persist!(GlobItemsDep, "glob-dependency");

impl Diffable for GlobItemsDep {
    type Item = Self;

    fn diff(record: Option<&Self::Item>, actual: Option<&Self::Item>) -> xvc_core::Diff<Self::Item> {
        watch!(record);
        watch!(actual);
        match (record, actual) {
            (None, None) => std::unreachable!("Both record and actual are None"),
            (None, Some(actual)) => xvc_core::Diff::RecordMissing {
                actual: actual.clone(),
            },
            (Some(record), None) => xvc_core::Diff::ActualMissing {
                record: record.clone(),
            },
            (Some(record), Some(actual)) => {
                match Self::diff_superficial(record, actual) {
                    Diff::Identical => Diff::Identical,
                    Diff::Skipped => Diff::Skipped,
                    Diff::ActualMissing { .. } => std::unreachable!("We already checked this conditions above"),
                    Diff::RecordMissing { .. } => std::unreachable!("We already checked this conditions above"),
                    Diff::Different { record, actual } => Self::diff_thorough(&record, &actual),
                } 
            }
        }
    }

    /// Just compares the xvc_path_metadata_map field.
    fn diff_superficial(record: &Self::Item, actual: &Self::Item) -> xvc_core::Diff<Self::Item> {
        if record.xvc_path_metadata_map == actual.xvc_path_metadata_map
        {
            Diff::Identical
        } else {
            Diff::Different {
                record: record.clone(),
                actual: actual.clone(),
            }
        }
    }

    /// Just compares the xvc_content_digest_map field.
    fn diff_thorough(record: &Self::Item, actual: &Self::Item) -> xvc_core::Diff<Self::Item> {
        if record.xvc_path_content_digest_map == actual.xvc_path_content_digest_map
        {
            Diff::Identical
        } else {
            Diff::Different {
                record: record.clone(),
                actual: actual.clone(),
            }
        }
    }


}
