use crate::types::diff::Diffable;
use crate::{attribute_digest, XvcDigest};
use crate::{types::hashalgorithm::HashAlgorithm, XvcPathMetadataMap};

use crate::error::Result;
use blake2::Digest;

use serde::{Deserialize, Serialize};

use super::AttributeDigest;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// Returns a stable digest of the list of paths.
pub struct CollectionDigest(XvcDigest);
attribute_digest!(CollectionDigest, "collection-digest");
impl Diffable<CollectionDigest> for CollectionDigest {}

impl CollectionDigest {
    pub fn new(paths: &XvcPathMetadataMap, algorithm: HashAlgorithm) -> Result<Self> {
        let paths_str = paths.keys().fold("".to_string(), |mut s, xp| {
            s.push_str(xp.as_ref());
            s
        });

        Ok(Self(XvcDigest::from_content(&paths_str, algorithm)))
    }
}
