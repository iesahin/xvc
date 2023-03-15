//! Digest of a list of paths, e.g., a glob or a directory.
use crate::types::diff::Diffable;
use crate::{attribute_digest, XvcDigest, XvcPath};
use crate::{types::hashalgorithm::HashAlgorithm};
use itertools::Itertools;

use crate::error::Result;

use serde::{Deserialize, Serialize};

use super::AttributeDigest;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// Returns a stable digest of the list of paths.
pub struct CollectionDigest(XvcDigest);
attribute_digest!(CollectionDigest, "collection-digest");
impl Diffable<CollectionDigest> for CollectionDigest {}

impl CollectionDigest {
    /// Create a new collection digest from all keys in `paths`.
    pub fn new(paths: impl Iterator<Item = XvcPath>, algorithm: HashAlgorithm) -> Result<Self> {
        let paths_str = paths.sorted().fold("".to_string(), |mut s, xp| {
            s.push_str(xp.as_ref());
            s
        });

        Ok(Self(XvcDigest::from_content(&paths_str, algorithm)))
    }
}
