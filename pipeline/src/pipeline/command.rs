use std::fmt::Display;

use serde::{Deserialize, Serialize};
use xvc_ecs::{persist, Storable};
#[derive(Debug, Clone, PartialOrd, Ord, Eq, PartialEq, Serialize, Deserialize)]
pub struct XvcStepCommand {
    pub command: String,
}

persist!(XvcStepCommand, "xvc-step-command");

impl Display for XvcStepCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command)
    }
}
