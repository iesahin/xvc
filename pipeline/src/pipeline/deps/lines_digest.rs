use std::io::{self, BufRead};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::{ContentDigest, Diff, HashAlgorithm, XvcDigest, XvcMetadata, XvcPath, XvcRoot};
use xvc_ecs::persist;

use crate::XvcDependency;

#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LinesDigestDep {
    /// Path of the file in the workspace
    pub path: XvcPath,
    /// The beginning of range
    pub begin: usize,
    /// The end of range
    pub end: usize,
    pub xvc_metadata: Option<XvcMetadata>,
    pub digest: Option<ContentDigest>,
}

persist!(LinesDigestDep, "lines-digest-dependency");

impl Into<XvcDependency> for LinesDigestDep {
    fn into(self) -> XvcDependency {
        XvcDependency::LinesDigest(self)
    }
}

impl LinesDigestDep {
    pub fn new(path: XvcPath, begin: usize, end: usize) -> Self {
        Self {
            path,
            begin,
            end,
            xvc_metadata: None,
            digest: None,
        }
    }

    pub fn update_digest(self, xvc_root: &XvcRoot, algorithm: HashAlgorithm) -> Self {
        let path = self.path.to_absolute_path(xvc_root);
        let file = std::fs::File::open(path).unwrap();
        let line_reader = io::BufReader::new(file).lines();
        let lines = line_reader
            .skip(self.begin)
            .take(self.end - self.begin)
            .map(|s| s.unwrap_or("".to_string()))
            .join("\n");
        let digest: ContentDigest = XvcDigest::from_content(&lines, algorithm).into();
        Self {
            digest: Some(digest),
            ..self
        }
    }

    pub fn update_metadata(self, xvc_metadata: Option<XvcMetadata>) -> Self {
        Self {
            xvc_metadata,
            ..self
        }
    }
}

impl Diffable for LinesDigestDep {
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

        if record.digest == actual.digest {
            Diff::Identical
        } else {
            Diff::Different {
                record: record.clone(),
                actual: actual.clone(),
            }
        }
    }

    /// ⚠️ Call actual.update_metadata and actual.update_lines before calling this. ⚠️
    fn diff(record: Option<&LinesDigestDep>, actual: Option<&Self::Item>) -> Diff<Self::Item> {
        match (record, actual) {
            (Some(record), Some(actual)) => match Self::diff_superficial(&record, &actual) {
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