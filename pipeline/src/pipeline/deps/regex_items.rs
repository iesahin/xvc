//! A dependency that depends on a regex searched in a text file. Unline [RegexDep], this
//! dependency tracks all the lines that matches the regex.
use std::io::{self, BufRead};

use regex::Regex;
use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{Diff, XvcMetadata, XvcPath, XvcPathMetadataMap, XvcRoot};
use xvc_ecs::persist;

use crate::XvcDependency;

/// When a step depends to a regex searched in a text file
#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RegexItemsDep {
    /// Path of the file in the workspace
    pub path: XvcPath,
    /// The regex to search in the file
    // We use this because Regex is not Serializable
    pub regex: String,
    /// Lines that match the regex in the file
    pub lines: Vec<String>,
    /// Metadata of the file
    pub xvc_metadata: Option<XvcMetadata>,
}

persist!(RegexItemsDep, "regex-dependency");

impl From<RegexItemsDep> for XvcDependency {
    fn from(val: RegexItemsDep) -> Self {
        XvcDependency::RegexItems(val)
    }
}

impl RegexItemsDep {
    /// Create a new RegexItemsDep with empty lines and metadata
    pub fn new(path: XvcPath, regex: String) -> Self {
        Self {
            path,
            regex,
            lines: Vec::new(),
            xvc_metadata: None,
        }
    }

    /// Update the metadata of the dependency
    pub fn update_metadata(self, xvc_metadata: Option<XvcMetadata>) -> Self {
        Self {
            xvc_metadata,
            ..self
        }
    }

    /// Update the metadata of the dependency from the given path metadata map
    pub fn update_metadata_from_pmm(self, pmm: &XvcPathMetadataMap) -> Self {
        let xvc_metadata = pmm.get(&self.path).cloned();
        self.update_metadata(xvc_metadata)
    }

    /// Update the lines of the dependency by reading the file and searching the regex in it
    pub fn update_lines(self, xvc_root: &XvcRoot) -> Self {
        let path = self.path.to_absolute_path(xvc_root);
        let regex = self.regex();
        let file = std::fs::File::open(path).unwrap();
        let line_reader = io::BufReader::new(file).lines();
        let lines = line_reader
            .filter_map(|line| {
                if let Ok(line) = line {
                    if regex.is_match(&line) {
                        Some(line)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        Self { lines, ..self }
    }

    /// Returns the regex of the dependency
    pub fn regex(&self) -> Regex {
        Regex::new(&self.regex).unwrap()
    }
}

impl Diffable for RegexItemsDep {
    type Item = Self;

    /// ⚠️ Call actual.update_metadata before calling this function ⚠️
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
            (None, None) => unreachable!("Either record or actual should have metadata"),
        }
    }

    /// ⚠️ Call actual.update_lines before calling this function ⚠️
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
}
