//! A path collection where each item is tracked separately.
use std::collections::{BTreeMap, HashMap};

use crate::{Result, XvcDependency};
use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{glob_paths, ContentDigest, HashAlgorithm, XvcMetadata, XvcPath, XvcRoot};
use xvc_ecs::persist;

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
    /// given pmm. The content digest map is empty.
    pub fn from_pmm(
        xvc_root: &XvcRoot,
        glob_root: &XvcPath,
        glob: String,
        pmm: &HashMap<XvcPath, XvcMetadata>,
    ) -> Result<GlobItemsDep> {
        let xvc_path_metadata_map =
            glob_paths(xvc_root, pmm, glob_root, &glob).map(|paths| paths.into_iter().collect())?;
        // We don't calculate the content digest map immediately, we only do that in through comparison
        Ok(GlobItemsDep {
            glob,
            xvc_path_metadata_map,
            xvc_path_content_digest_map: BTreeMap::new(),
        })
    }

    /// Update the content digest map for each path in the metadata map.
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

    /// Unlike update_digests, this only updates the changed paths' digest.
    /// It checks the record's metadata for the identical path and only updates the digest if the metadata has changed.
    pub fn update_changed_paths_digests(
        self,
        record: &Self,
        xvc_root: &XvcRoot,
        algorithm: HashAlgorithm,
    ) -> Result<Self> {
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
        Ok(Self {
            xvc_path_content_digest_map,
            ..self
        })
    }
}

persist!(GlobItemsDep, "glob-dependency");

impl Diffable for GlobItemsDep {
    type Item = Self;
}
