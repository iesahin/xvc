//! Home of the [XvcFileType] enum.
use std::{
    fs::{self},
    io,
};

use strum_macros::Display;

use serde::{Deserialize, Serialize};

/// Describes the type of a path. It is extracted from `fs::Metadata`.
/// The default value is `XvcFileType::Missing`. When there is an error in
/// path::metadata(), the corresponding path is considered as missing.
///
/// This is used on all kinds of comparisons and path metadata to decide about
/// the relevant operations.
#[allow(dead_code)]
#[derive(
    Debug, Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Hash, Serialize, Deserialize, Display,
)]
#[strum(serialize_all = "lowercase")]
pub enum XvcFileType {
    /// The path is not found or not accessible.
    /// This is the default value.
    Missing,
    /// The path is a regular file.
    File,
    /// The path is a directory.
    Directory,
    /// The path is a symbolic link.
    Symlink,
    /// The path is a hard link.
    Hardlink,
    /// The path is a reference link.
    Reflink,
}

impl Default for XvcFileType {
    fn default() -> Self {
        Self::Missing
    }
}

impl From<fs::Metadata> for XvcFileType {
    fn from(md: fs::Metadata) -> Self {
        let ft = md.file_type();
        if ft.is_dir() {
            Self::Directory
        } else if ft.is_file() {
            Self::File
        } else if ft.is_symlink() {
            Self::Symlink
        } else {
            Self::Missing
        }
    }
}

impl From<&fs::Metadata> for XvcFileType {
    fn from(md: &fs::Metadata) -> Self {
        let ft = md.file_type();
        if ft.is_dir() {
            Self::Directory
        } else if ft.is_file() {
            Self::File
        } else if ft.is_symlink() {
            Self::Symlink
        } else {
            Self::Missing
        }
    }
}

impl From<io::Result<fs::Metadata>> for XvcFileType {
    fn from(r_md: io::Result<fs::Metadata>) -> Self {
        match r_md {
            Err(_) => Self::Missing,
            Ok(md) => {
                let ft = md.file_type();
                if ft.is_dir() {
                    Self::Directory
                } else if ft.is_file() {
                    Self::File
                } else if ft.is_symlink() {
                    Self::Symlink
                } else {
                    Self::Missing
                }
            }
        }
    }
}
