use std::fs;
use std::{ffi::OsStr, path::Path};

use rayon::*;

use strum_macros::Display;
use xvc_core::{XvcPath, XvcRoot};
use xvc_ecs::persist;

use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};

/// Possible formats for recognized metrics formats.
/// Metrics files are where the pipeline writes its output in a structured format.
/// We can read these files and use them to generate reports
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub enum XvcMetricsFormat {
    /// Unknown format, we don't know how to read it
    Unknown,
    /// Comma,separated,values
    CSV,
    /// JavaScript Object Notation
    JSON,
    /// Tab separated   values
    TSV,
}

impl XvcMetricsFormat {
    /// Decide the format from extension of the given path
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

/// Possible outputs for the pipeline.
///
/// These outputs can be defined with `xvc pipeline output` command.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Display, PartialOrd, Ord)]
pub enum XvcOutput {
    /// A (possibly binary) file.
    File {
        /// Path to the file
        path: XvcPath,
    },
    /// A textual metrics file with a known [`XvcMetricsFormat`]
    Metric {
        /// Path to the file
        path: XvcPath,
        /// Format of the file
        format: XvcMetricsFormat,
    },
    /// An image file, like a plot or generated file
    Image {
        /// Path to the file
        path: XvcPath,
        // TODO: Should we add a `format` field here?
    },
    // TODO: We can add `Model` here.
}

persist!(XvcOutput, "xvc-output");

impl From<XvcOutput> for XvcPath {
    /// Return the path of a given output
    fn from(out: XvcOutput) -> XvcPath {
        match out {
            XvcOutput::File { path } => path,
            XvcOutput::Metric { path, .. } => path,
            XvcOutput::Image { path, .. } => path,
        }
    }
}

impl From<&XvcOutput> for XvcPath {
    /// Return the path of a given output
    fn from(out: &XvcOutput) -> XvcPath {
        match out {
            XvcOutput::File { path } => path.clone(),
            XvcOutput::Metric { path, .. } => path.clone(),
            XvcOutput::Image { path, .. } => path.clone(),
        }
    }
}

impl XvcOutput {
    /// Used to check whether pipeline / step output is changed (or missing.)
    pub fn fs_metadata(&self, xvc_root: &XvcRoot) -> Result<fs::Metadata> {
        let xvc_path: XvcPath = self.into();
        let abs_path = xvc_path.to_absolute_path(xvc_root);
        abs_path
            .metadata()
            .map_err(|source| Error::IoError { source })
    }
}
