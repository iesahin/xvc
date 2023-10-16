//! A short representation of the file metadata that fits into 32 bytes.
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::types::diff::Diffable;
use crate::Result;
use crate::{attribute_digest, HashAlgorithm, XvcDigest, XvcMetadata};

/// A short representation of the file metadata that fits into 32 bytes.
/// This is used to quickly compare metadata of files without individual fields.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct XvcMetadataDigest(XvcDigest);
attribute_digest!(XvcMetadataDigest, "xvc-metadata-digest");
impl Diffable for XvcMetadataDigest {
    type Item = XvcMetadataDigest;

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
}

impl XvcMetadataDigest {
    /// Return metadata information aligned to 32-bytes to compare quickly
    /// It uses HashAlgorithm::AsIs without making any calculations.
    pub fn new(xvc_metadata: &XvcMetadata) -> Result<Self> {
        let ft = xvc_metadata.file_type as u64;

        let modified = if let Some(modified) = xvc_metadata.modified {
            modified.duration_since(SystemTime::UNIX_EPOCH)?.as_secs()
        } else {
            0u64
        };

        let size = if let Some(size) = xvc_metadata.size {
            size
        } else {
            0u64
        };

        let mut bytes: [u8; 32] = [0; 32];
        bytes[..8].clone_from_slice(&ft.to_le_bytes());
        bytes[8..16].clone_from_slice(&modified.to_le_bytes());
        bytes[16..24].clone_from_slice(&size.to_le_bytes());

        Ok(Self(XvcDigest {
            digest: bytes,
            algorithm: HashAlgorithm::AsIs,
        }))
    }

    /// Return the inner digest
    pub fn digest(&self) -> XvcDigest {
        self.0
    }
}
