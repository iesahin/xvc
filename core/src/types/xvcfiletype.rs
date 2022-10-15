use std::{
    fs::{self},
    io,
};

use strum_macros::Display;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(
    Debug, Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Hash, Serialize, Deserialize, Display,
)]
#[strum(serialize_all = "lowercase")]
pub enum XvcFileType {
    RecordOnly,
    File,
    Directory,
    Symlink,
    Hardlink,
    Reflink,
}

impl Default for XvcFileType {
    fn default() -> Self {
        Self::RecordOnly
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
            Self::RecordOnly
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
            Self::RecordOnly
        }
    }
}

impl From<io::Result<fs::Metadata>> for XvcFileType {
    fn from(r_md: io::Result<fs::Metadata>) -> Self {
        match r_md {
            Err(_) => Self::RecordOnly,
            Ok(md) => {
                let ft = md.file_type();
                if ft.is_dir() {
                    Self::Directory
                } else if ft.is_file() {
                    Self::File
                } else if ft.is_symlink() {
                    Self::Symlink
                } else {
                    Self::RecordOnly
                }
            }
        }
    }
}
