use std::fmt::Display;

use serde::{Deserialize, Serialize};
use xvc_ecs::persist;

/// Command to run for an [XvcStep].
#[derive(Debug, Clone, PartialOrd, Ord, Eq, PartialEq, Serialize, Deserialize)]
pub struct XvcStepCommand {
    /// A shell command that will be run via [subprocess::Exec::shell] in [crate::pipeline::s_waiting_to_run].
    pub command: String,
}

persist!(XvcStepCommand, "xvc-step-command");

impl Display for XvcStepCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command)
    }
}
