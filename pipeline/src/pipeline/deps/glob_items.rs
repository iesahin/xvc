use std::cell::RefCell;
use std::collections::{BTreeMap, HashSet};

use crate::{Result, XvcDependency};
use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{
    glob_paths, ContentDigest, Diff, HashAlgorithm, PathCollectionDigest, XvcMetadata,
    XvcMetadataDigest, XvcPath, XvcPathMetadataMap, XvcRoot,
};
use xvc_ecs::persist;

#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GlobItemsDep {
    /// The glob pattern that will be converted to a [Glob]
    pub glob: String,
    pub xvc_path_metadata_map: BTreeMap<XvcPath, XvcMetadata>,
    pub xvc_path_content_digest_map: BTreeMap<XvcPath, ContentDigest>,
}

impl Into<XvcDependency> for GlobItemsDep {
    fn into(self) -> XvcDependency {
        XvcDependency::GlobItems(self)
    }
}

impl GlobItemsDep {
    pub fn new(glob: String) -> Self {
        Self {
            glob,
            xvc_path_metadata_map: BTreeMap::new(),
            xvc_path_content_digest_map: BTreeMap::new(),
        }
    }

    pub fn from_pmm(
        xvc_root: &XvcRoot,
        glob_root: &XvcPath,
        glob: String,
        pmm: &std::collections::HashMap<XvcPath, xvc_core::XvcMetadata>,
    ) -> Result<GlobItemsDep> {
        let xvc_path_metadata_map = glob_paths(xvc_root, pmm, glob_root, &glob)
            .and_then(|paths| Ok(paths.into_iter().collect()))?;
        // We don't calculate the content digest map immediately, we only do that in through comparison
        Ok(GlobItemsDep {
            glob,
            xvc_path_metadata_map,
            xvc_path_content_digest_map: BTreeMap::new(),
        })
    }

    pub fn update_digests(self, xvc_root: &XvcRoot, algorithm: HashAlgorithm) -> Result<Self> {
        let mut xvc_path_content_digest_map = BTreeMap::new();
        for (xvc_path, xvc_metadata) in self.xvc_path_metadata_map.iter() {
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
                record
                    .xvc_path_content_digest_map
                    .get(xvc_path)
                    .unwrap()
                    .clone()
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
