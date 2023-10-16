//! LineItemsDep is a dependency that contains a range of lines from a file.
use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{Diff, XvcMetadata, XvcPath, XvcRoot};
use xvc_ecs::persist;

use crate::XvcDependency;

/// A dependency that contains a range of lines from a file. Unlike [LinesDep], this keeps track of
/// the lines themselves.
#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LineItemsDep {
    /// Path of the file in the workspace
    pub path: XvcPath,
    /// The beginning of range
    pub begin: usize,
    /// The end of range
    pub end: usize,
    /// Metadata of the file
    pub xvc_metadata: Option<XvcMetadata>,
    /// Lines of the file
    pub lines: Vec<String>,
}

persist!(LineItemsDep, "lines-dependency");

impl From<LineItemsDep> for XvcDependency {
    fn from(val: LineItemsDep) -> Self {
        XvcDependency::LineItems(val)
    }
}

impl LineItemsDep {
    /// Create a new [LineItemsDep] with blank metadata and lines.
    pub fn new(path: XvcPath, begin: usize, end: usize) -> Self {
        Self {
            path,
            begin,
            end,
            xvc_metadata: None,
            lines: Vec::new(),
        }
    }

    /// Update the lines by reading the file
    pub fn update_lines(self, xvc_root: &XvcRoot) -> Self {
        let path = self.path.to_absolute_path(xvc_root);
        let file = std::fs::File::open(path).unwrap();
        let line_reader = io::BufReader::new(file).lines();
        let lines = line_reader
            .skip(self.begin)
            .take(self.end - self.begin)
            .map(|s| s.unwrap_or("".to_string()))
            .collect();
        Self { lines, ..self }
    }

    /// Update metadata with the supplied metadata
    pub fn update_metadata(self, xvc_metadata: Option<XvcMetadata>) -> Self {
        Self {
            xvc_metadata,
            ..self
        }
    }
}

impl Diffable for LineItemsDep {
    type Item = Self;

    /// ⚠️ Call actual.update_metadata before calling this. ⚠️
    fn diff_superficial(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        assert!(record.path == actual.path);

        match (record.xvc_metadata, actual.xvc_metadata) {
            (Some(rec_md), Some(act_md)) => {
                if rec_md == act_md {
                    Diff::Identical
                } else {
                    Diff::Different {
                        record: record.clone(),
                        actual: actual.clone(),
                    }
                }
            }
            (None, Some(_)) => Diff::RecordMissing {
                actual: actual.clone(),
            },
            (Some(_), None) => Diff::ActualMissing {
                record: record.clone(),
            },
            (None, None) => Diff::Identical,
        }
    }

    /// ⚠️ Call actual.update_lines before calling this. ⚠️
    fn diff_thorough(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        assert!(record.path == actual.path);

        if record.lines == actual.lines {
            Diff::Identical
        } else {
            Diff::Different {
                record: record.clone(),
                actual: actual.clone(),
            }
        }
    }

    /// ⚠️ Call actual.update_metadata and actual.update_lines before calling this. ⚠️
    fn diff(record: Option<&LineItemsDep>, actual: Option<&Self::Item>) -> Diff<Self::Item> {
        match (record, actual) {
            (Some(record), Some(actual)) => match Self::diff_superficial(record, actual) {
                Diff::Different { record, actual } => Self::diff_thorough(&record, &actual),
                diff => diff,
            },
            (None, Some(actual)) => Diff::RecordMissing {
                actual: actual.clone(),
            },
            (Some(record), None) => Diff::ActualMissing {
                record: record.clone(),
            },
            (None, None) => unreachable!("Both record and actual are None"),
        }
    }
}
