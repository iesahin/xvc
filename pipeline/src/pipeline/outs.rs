use std::fs;
use std::{ffi::OsStr, path::Path};

use rayon::*;

use strum_macros::Display;
use xvc_core::{XvcPath, XvcRoot};
use xvc_ecs::persist;

use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub enum XvcMetricsFormat {
    Unknown,
    CSV,
    JSON,
    TSV,
}

impl XvcMetricsFormat {
    pub fn from_path(path: &Path) -> Self {
        match path
            .extension()
            .unwrap_or_else(|| OsStr::new(""))
            .to_ascii_lowercase()
            .to_str()
            .unwrap_or("")
        {
            "csv" => Self::CSV,
            "json" => Self::JSON,
            "tsv" => Self::TSV,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Display, PartialOrd, Ord)]
pub enum XvcOutput {
    File {
        path: XvcPath,
    },
    Metric {
        path: XvcPath,
        format: XvcMetricsFormat,
    },
    Image {
        path: XvcPath,
    },
}

persist!(XvcOutput, "xvc-output");

impl From<XvcOutput> for XvcPath {
    fn from(out: XvcOutput) -> XvcPath {
        match out {
            XvcOutput::File { path } => path,
            XvcOutput::Metric { path, .. } => path,
            XvcOutput::Image { path, .. } => path,
        }
    }
}

impl From<&XvcOutput> for XvcPath {
    fn from(out: &XvcOutput) -> XvcPath {
        match out {
            XvcOutput::File { path } => path.clone(),
            XvcOutput::Metric { path, .. } => path.clone(),
            XvcOutput::Image { path, .. } => path.clone(),
        }
    }
}

impl XvcOutput {
    pub fn fs_metadata(&self, xvc_root: &XvcRoot) -> Result<fs::Metadata> {
        let xvc_path: XvcPath = self.into();
        let abs_path = xvc_path.to_absolute_path(xvc_root);
        abs_path
            .metadata()
            .map_err(|source| Error::IoError { source })
    }
}
