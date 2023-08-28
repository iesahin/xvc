//! Home of the [XvcMetadata] struct.
//!
//! Also contains various From implementations to convert [fs::Metadata] to [XvcMetadata].
use crate::error::{Error, Result};
use std::time::SystemTime;
use std::{fs, io};

use serde::{Deserialize, Serialize};

use crate::{
    XvcFileType, XvcMetadataDigest,
};
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
impl Diffable for XvcMetadata {
    type Item = XvcMetadata;
}

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
