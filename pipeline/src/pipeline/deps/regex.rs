use regex::Regex;
use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{ContentDigest, Diff, XvcMetadata, XvcPath};
use xvc_ecs::persist;

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

    pub fn update_lines(self) -> Self {
        // TODO: Better to stream the file instead of reading it all
        let content = std::fs::read_to_string(&self.path).unwrap();
        let lines = self
            .regex()
            .find_iter(&content)
            .map(|m| m.as_str().to_string())
            .collect();
        Self { lines, ..self }
    }

    pub fn regex(&self) -> Regex {
        Regex::new(&self.regex).unwrap()
    }
}

impl Diffable for RegexDep {
    type Item = Self;

    fn diff_superficial(record: Self::Item, actual: Self::Item) -> Diff<Self::Item> {
        assert!(record.path == actual.path);

        match (record.xvc_metadata, actual.xvc_metadata) {
            (Some(record), Some(actual)) => {
                if record == actual {
                    Diff::Identical
                } else {
                    Diff::Different { record, actual }
                }
            }
            (None, Some(actual)) => Diff::RecordMissing { actual },
            (Some(record), None) => Diff::ActualMissing { record },
            (None, None) => unreachable!("Either record or actual should have metadata"),
        }
    }

    fn diff_thorough(record: Self::Item, actual: Self::Item) -> Diff<Self::Item> {
        assert!(record.path == actual.path);
        let actual = actual.update_lines();
        if record.lines == actual.lines {
            Diff::Identical
        } else {
            Diff::Different { record, actual }
        }
    }

    fn diff(record: Option<Self::Item>, actual: Option<Self::Item>) -> Diff<Self::Item> {
        match (record, actual) {
            (None, None) => unreachable!("Either record or actual should be available"),
            (None, Some(actual)) => Diff::RecordMissing { actual },
            (Some(record), None) => Diff::ActualMissing { record },
            (Some(record), Some(actual)) => match Self::diff_superficial(record, actual) {
                Diff::Different { record, actual } => Self::diff_thorough(record, actual),
                Diff::RecordMissing { actual } => Diff::RecordMissing {
                    actual: actual.update_lines(),
                },
                diff => diff,
            },
        }
    }
}
