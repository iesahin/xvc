//! A regex dependency is a dependency that is based on a regex search in a text file.
use std::io::{self, BufRead};

use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{ContentDigest, Diff, HashAlgorithm, XvcDigest, XvcMetadata, XvcPath, XvcRoot};
use xvc_ecs::persist;

use crate::XvcDependency;

/// When a step depends to a regex search in a text file
#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RegexDep {
    /// Path of the file in the workspace
    pub path: XvcPath,
    /// The regex to search in the file
    // We use this because Regex is not Serializable
    pub regex: String,
    /// The digest of the lines that match the regex
    pub lines_digest: Option<ContentDigest>,
    /// The metadata of the file
    pub xvc_metadata: Option<XvcMetadata>,
}

persist!(RegexDep, "regex-digest-dependency");

impl From<RegexDep> for XvcDependency {
    fn from(val: RegexDep) -> Self {
        XvcDependency::Regex(val)
    }
}

impl RegexDep {
    /// Create a new RegexDep with empty metadata and digest
    pub fn new(path: XvcPath, regex: String) -> Self {
        Self {
            path,
            regex,
            lines_digest: None,
            xvc_metadata: None,
        }
    }
    /// Update the metadata of the file
    pub fn update_metadata(self, xvc_metadata: Option<XvcMetadata>) -> Self {
        Self {
            xvc_metadata,
            ..self
        }
    }

    /// Update the digest of the file by reading the file and collecting all lines that match the regex
    pub fn update_digest(self, xvc_root: &XvcRoot, algorithm: HashAlgorithm) -> Self {
        let path = self.path.to_absolute_path(xvc_root);
        let regex = self.regex();
        let file = std::fs::File::open(path).unwrap();
        let lines = io::BufReader::new(file).lines();
        let matching_lines = lines
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
            .join("");

        let lines_digest = Some(XvcDigest::from_content(&matching_lines, algorithm).into());
        Self {
            lines_digest,
            ..self
        }
    }

    /// Returns the inner regex
    pub fn regex(&self) -> Regex {
        Regex::new(&self.regex).unwrap()
    }
}

impl Diffable for RegexDep {
    type Item = Self;

    /// ⚠️  Update the metadata with actual.update_metadata before calling this function
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

    /// ⚠️  Update the metadata and lines with actual.update_digest before calling this function
    fn diff_thorough(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        assert!(record.path == actual.path);
        if record.lines_digest == actual.lines_digest {
            Diff::Identical
        } else {
            Diff::Different {
                record: record.clone(),
                actual: actual.clone(),
            }
        }
    }
}
