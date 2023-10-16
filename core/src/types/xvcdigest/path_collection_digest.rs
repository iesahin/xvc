//! Digest of a list of paths, e.g., a glob or a directory.
use crate::types::diff::Diffable;
use crate::types::hashalgorithm::HashAlgorithm;
use crate::{attribute_digest, XvcDigest, XvcMetadata, XvcPath};
use itertools::Itertools;

use crate::error::Result;

use serde::{Deserialize, Serialize};

use super::AttributeDigest;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// Returns a stable digest of the list of paths.
pub struct PathCollectionDigest(XvcDigest);
attribute_digest!(PathCollectionDigest, "path-collection-digest");
impl Diffable for PathCollectionDigest {
    type Item = PathCollectionDigest;

    fn diff_superficial(record: &Self::Item, actual: &Self::Item) -> crate::Diff<Self::Item> {
        if record == actual {
            crate::Diff::Identical
        } else {
            crate::Diff::Different {
                record: *record,
                actual: *actual,
            }
        }
    }

    fn diff_thorough(record: &Self::Item, actual: &Self::Item) -> crate::Diff<Self::Item> {
        Self::diff_superficial(record, actual)
    }
}

impl PathCollectionDigest {
    /// Create a new collection digest from all paths and metadata in `paths`.
    pub fn new<'a>(
        paths: impl Iterator<Item = (&'a XvcPath, &'a XvcMetadata)>,
        algorithm: HashAlgorithm,
    ) -> Result<Self> {
        let bytes = paths
            .sorted()
            .fold(Vec::<u8>::new(), |mut bytes, (xp, xmd)| {
                bytes.extend(xp.as_str().as_bytes());
                bytes.extend(xmd.digest().unwrap().digest().digest);
                bytes
            });

        Ok(Self(XvcDigest::from_bytes(&bytes, algorithm)))
    }
}
