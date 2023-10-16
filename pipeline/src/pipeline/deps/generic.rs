//! A generic dependency that's invalidated when the given command's output has changed.
use crate::error::Error;
use crate::{Result, XvcDependency};
use serde::{Deserialize, Serialize};
use subprocess::Exec;
use xvc_core::types::diff::Diffable;
use xvc_core::{Diff, HashAlgorithm, StdoutDigest};
use xvc_ecs::persist;
use xvc_logging::watch;

#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
/// A generic dependency that's invalidated when the given command's output has changed.
pub struct GenericDep {
    /// The command that the step runs to check its output
    pub generic_command: String,
    /// The output digest collected from the command output
    pub output_digest: Option<StdoutDigest>,
}

persist!(GenericDep, "generic-dependency");

impl From<GenericDep> for XvcDependency {
    fn from(val: GenericDep) -> Self {
        XvcDependency::Generic(val)
    }
}

impl GenericDep {
    /// Create a new generic dependency with the specified command
    pub fn new(generic_command: String) -> Self {
        Self {
            generic_command,
            output_digest: None,
        }
    }

    /// Run the command and update the output digest
    pub fn update_output_digest(self) -> Result<Self> {
        let generic_command = self.generic_command;
        watch!(generic_command);

        let command_output = Exec::shell(generic_command.clone()).capture()?;
        watch!(command_output);
        let stdout = String::from_utf8(command_output.stdout)?;
        let stderr = String::from_utf8(command_output.stderr)?;
        watch!(stdout);
        watch!(stderr);
        let algorithm = HashAlgorithm::Blake3;
        let return_code = command_output.exit_status;
        if !stderr.is_empty() || !return_code.success() {
            Err(Error::ProcessError { stdout, stderr })
        } else {
            Ok(Self {
                output_digest: Some(StdoutDigest::new(&stdout, algorithm)),
                generic_command,
            })
        }
    }
}

impl Diffable for GenericDep {
    type Item = GenericDep;

    /// Always use the command output for the diff.
    fn diff_superficial(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        Self::diff_thorough(record, actual)
    }

    /// Compare the command and the output.
    /// WARN: Self::update_output_digest() must be called before this method.
    fn diff_thorough(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        watch!(record);
        watch!(actual);
        if record == actual {
            Diff::Identical
        } else {
            Diff::Different {
                record: record.clone(),
                actual: actual.clone(),
            }
        }
    }
}
