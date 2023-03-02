//! Home of the [XvcMetadata] struct.
//!
//! Also contains various From implementations to convert [fs::Metadata] to [XvcMetadata].
use crate::error::{Error, Result};
use std::time::SystemTime;
use std::{fs, io};

use serde::{Deserialize, Serialize};

use crate::{attribute_digest, AttributeDigest, HashAlgorithm, XvcDigest, XvcFileType};
use xvc_ecs::persist;

use super::diff::Diffable;

/// Metadata associated with a `XvcPath`
#[derive(
    Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash,
)]
pub struct XvcMetadata {
    /// File type of this metadata
    pub file_type: XvcFileType,
    /// size of the file
    pub size: Option<u64>,
    /// Last modification time
    pub modified: Option<SystemTime>,
}

persist!(XvcMetadata, "xvc-metadata");

impl XvcMetadata {
    /// Return metadata information aligned to 32-bytes to compare quickly
    /// It uses HashAlgorithm::AsIs without making any calculations.
    pub fn digest(&self) -> Result<XvcMetadataDigest> {
        XvcMetadataDigest::new(self)
    }

    /// Returns true if the file type is a XvcFileType::File
    pub fn is_file(&self) -> bool {
        self.file_type == XvcFileType::File
    }

    /// Returns true if the file type is a XvcFileType::Directory
    pub fn is_dir(&self) -> bool {
        self.file_type == XvcFileType::Directory
    }
}

impl From<io::Result<fs::Metadata>> for XvcMetadata {
    fn from(r_md: io::Result<fs::Metadata>) -> Self {
        match r_md {
            Err(_) => Self {
                file_type: XvcFileType::Missing,
                size: None,
                modified: None,
            },
            Ok(md) => {
                let file_type = XvcFileType::from(&md);
                let size = md.len();
                let modified = md
                    .modified()
                    .map_err(|source| Error::IoError { source }.debug())
                    .ok();
                Self {
                    file_type,
                    size: Some(size),
                    modified,
                }
            }
        }
    }
}

impl From<std::result::Result<fs::Metadata, jwalk::Error>> for XvcMetadata {
    fn from(r_md: std::result::Result<fs::Metadata, jwalk::Error>) -> Self {
        match r_md {
            Err(_) => Self {
                file_type: XvcFileType::Missing,
                size: None,
                modified: None,
            },
            Ok(md) => {
                let file_type = XvcFileType::from(&md);
                let size = md.len();
                let modified = md
                    .modified()
                    .map_err(|source| Error::IoError { source }.debug())
                    .ok();
                Self {
                    file_type,
                    size: Some(size),
                    modified,
                }
            }
        }
    }
}

impl From<fs::Metadata> for XvcMetadata {
    fn from(md: fs::Metadata) -> Self {
        let modified = md
            .modified()
            .map_err(|source| Error::IoError { source }.warn())
            .ok();
        let file_type = XvcFileType::from(&md);
        Self {
            file_type,
            size: Some(md.len()),
            modified,
        }
    }
}

impl From<&fs::Metadata> for XvcMetadata {
    fn from(md: &fs::Metadata) -> Self {
        let modified = md
            .modified()
            .map_err(|source| Error::IoError { source }.warn())
            .ok();

        let file_type = XvcFileType::from(md);
        Self {
            file_type,
            size: Some(md.len()),
            modified,
        }
    }
}

impl Diffable<XvcMetadata> for XvcMetadata {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct XvcMetadataDigest(XvcDigest);
attribute_digest!(XvcMetadataDigest, "xvc-metadata-digest");
impl Diffable<XvcMetadataDigest> for XvcMetadataDigest {}

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
}
