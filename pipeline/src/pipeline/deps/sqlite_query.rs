//! Implements dependencies on sqlite databases. [SqliteQueryDep] is a dependency that runs a query
//! and checks whether the result of that query has changed. It doesn't run the query if the
//! metadata of the database file hasn't changed.
use crate::XvcDependency;
use fallible_iterator::FallibleIterator;
use rusqlite::{Connection, OpenFlags};
use serde::{Deserialize, Serialize};

use crate::Result;
use xvc_core::types::diff::Diffable;
use xvc_core::{ContentDigest, Diff, HashAlgorithm, XvcDigest, XvcMetadata, XvcPath, XvcRoot};

use xvc_core::persist;

/// When a step depends to a regex search in a text file
#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SqliteQueryDep {
    /// Path of the database file
    pub path: XvcPath,
    /// The query we run on the database
    pub query: String,
    /// The digest of the lines that match the regex
    pub query_digest: Option<ContentDigest>,
    /// The metadata of the database file
    pub xvc_metadata: Option<XvcMetadata>,
}

persist!(SqliteQueryDep, "sqlite-query-dependency");

impl From<SqliteQueryDep> for XvcDependency {
    fn from(val: SqliteQueryDep) -> Self {
        XvcDependency::SqliteQueryDigest(val)
    }
}

impl SqliteQueryDep {
    /// Create a new SqliteQueryDep with empty metadata and digest
    pub fn new(path: XvcPath, query: String) -> Self {
        Self {
            path,
            query,
            query_digest: None,
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
    pub fn update_digest(self, xvc_root: &XvcRoot, algorithm: HashAlgorithm) -> Result<Self> {
        let path = self.path.to_absolute_path(xvc_root);
        let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
        let sqlite = Connection::open_with_flags(path, flags)?;
        let mut prepared = sqlite.prepare(&self.query)?;
        // TODO: Should we allow params in the queries?
        let query_res = prepared.raw_query();
        let query_lines = query_res
            .map(|row| {
                let mut i = 0;
                // TODO: Add salting with the repo id here?
                let mut els = String::new();
                while let Ok(col) = row.get_ref(i) {
                    match col.data_type() {
                        rusqlite::types::Type::Text => {
                            els.push_str(col.as_str()?);
                        }
                        rusqlite::types::Type::Integer => {
                            els.push_str(col.as_i64()?.to_string().as_str());
                        }
                        rusqlite::types::Type::Real => {
                            els.push_str(col.as_f64()?.to_string().as_str());
                        }
                        _ => {
                            els.push_str(col.as_str()?);
                        }
                    }
                    i += 1;
                }
                Ok(els)
            })
            .collect::<Vec<String>>()?
            .join("\n");

        let query_digest = Some(XvcDigest::from_content(&query_lines, algorithm).into());
        Ok(Self {
            query_digest,
            ..self
        })
    }
}

impl Diffable for SqliteQueryDep {
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
        if record.query_digest == actual.query_digest {
            Diff::Identical
        } else {
            Diff::Different {
                record: record.clone(),
                actual: actual.clone(),
            }
        }
    }
}
