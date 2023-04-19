use std::io::{self, BufRead};

use regex::Regex;
use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{ContentDigest, Diff, XvcMetadata, XvcPath, XvcRoot};
use xvc_ecs::persist;

use crate::XvcDependency;

/// When a step depends to a regex searched in a text file
#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RegexDep {
    /// Path of the file in the workspace
    pub path: XvcPath,
    /// The regex to search in the file
    // We use this because Regex is not Serializable
    pub regex: String,
    pub lines: Vec<String>,
    pub xvc_metadata: Option<XvcMetadata>,
}

persist!(RegexDep, "regex-dependency");

impl Into<XvcDependency> for RegexDep {
    fn into(self) -> XvcDependency {
        XvcDependency::Regex(self)
    }
}

impl RegexDep {
    pub fn new(path: XvcPath, regex: String) -> Self {
        Self {
            path,
            regex,
            lines: Vec::new(),
            xvc_metadata: None,
        }
    }

    pub fn update_metadata(self, xvc_metadata: Option<XvcMetadata>) -> Self {
        Self {
            xvc_metadata,
            ..self
        }
    }

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

    pub fn regex(&self) -> Regex {
        Regex::new(&self.regex).unwrap()
    }
}

impl Diffable for RegexDep {
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
