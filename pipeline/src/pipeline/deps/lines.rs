use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{Diff, XvcMetadata, XvcPath};
use xvc_ecs::persist;

#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LinesDep {
    /// Path of the file in the workspace
    pub path: XvcPath,
    /// The beginning of range
    pub begin: usize,
    /// The end of range
    pub end: usize,
    pub xvc_metadata: Option<XvcMetadata>,
    pub lines: Vec<String>,
}

persist!(LinesDep, "lines-dependency");

impl LinesDep {
    pub fn new(path: XvcPath, begin: usize, end: usize) -> Self {
        Self {
            path,
            begin,
            end,
            xvc_metadata: None,
            lines: Vec::new(),
        }
    }

    pub fn update_lines(self) -> Self {
        let content = std::fs::read_to_string(&self.path).unwrap();
        let lines = content
            .lines()
            .skip(self.begin)
            .take(self.end - self.begin)
            .map(|s| s.to_string())
            .collect();
        Self { lines, ..self }
    }

    pub fn update_metadata(self, xvc_metadata: Option<XvcMetadata>) -> Self {
        Self {
            xvc_metadata,
            ..self
        }
    }
}

impl Diffable for LinesDep {
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
            (None, None) => Diff::Identical,
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
            (Some(record), Some(actual)) => match Self::diff_superficial {
                Diff::Different { record, actual } => Self::diff_thorough(record, actual),
                diff => diff,
            },
            (None, Some(actual)) => Diff::RecordMissing { actual },
            (Some(record), None) => Diff::ActualMissing { record },
            (None, None) => unreachable!("Both record and actual are None"),
        }
    }
}
