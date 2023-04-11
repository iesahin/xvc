use crate::error::Error;
use crate::Result;
use serde::{Deserialize, Serialize};
use subprocess::Exec;
use xvc_core::types::diff::Diffable;
use xvc_core::{Diff, HashAlgorithm, StdoutDigest};
use xvc_ecs::persist;

#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
/// A generic dependency that's invalidated when the given command's output has changed.
pub struct GenericDep {
    pub generic_command: String,
    pub output_digest: Option<StdoutDigest>,
}

persist!(GenericDep, "generic-dependency");

impl GenericDep {
    pub fn new(generic_command: String) -> Self {
        Self {
            generic_command,
            output_digest: None,
        }
    }

    pub fn update_output_digest(self) -> Result<Self> {
        let generic_command = self.generic_command;

        let command_output = Exec::shell(generic_command).capture()?;
        let stdout = String::from_utf8(command_output.stdout)?;
        let stderr = String::from_utf8(command_output.stderr)?;
        let algorithm = HashAlgorithm::Blake3;
        let return_code = command_output.exit_status;
        if stderr.len() > 0 || !return_code.success() {
            return Err(Error::ProcessError { stdout, stderr });
        }

        Ok(Self {
            output_digest: Some(StdoutDigest::new(&stdout, algorithm).into()),
            generic_command,
        })
    }
}

impl Diffable for GenericDep {
    type Item = GenericDep;

    fn diff(record: Option<GenericDep>, actual: Option<GenericDep>) -> Diff<GenericDep> {
        match (record, actual) {
            (None, None) => unreachable!("Both record and actual are None"),
            (None, Some(actual)) => Diff::RecordMissing { actual },
            (Some(record), None) => Diff::ActualMissing { record },
            (Some(record), Some(actual)) => {
                Self::diff_thorough(record, actual)
            }
        }
    }

    fn diff_superficial(record: Self::Item, actual: Self::Item) -> Diff<Self::Item> {
        Self::diff_thorough(record, actual)
    }

    fn diff_thorough(record: Self::Item, actual: Self::Item) -> Diff<Self::Item> {
        assert!(record.generic_command == actual.generic_command);
        let actual = actual.update_output_digest().unwrap();
        if actual.output_digest == record.output_digest {
            Diff::Identical
        } else {
            Diff::Different {
                record,
                actual,
            }
        }
    }
}
