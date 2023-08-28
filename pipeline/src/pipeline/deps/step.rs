use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;

use xvc_ecs::persist;

use crate::XvcDependency;

/// Invalidates when the dependency step is invalidated.
#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StepDep {
    /// The name of the step
    pub name: String,
}

persist!(StepDep, "step-dependency");

impl Into<XvcDependency> for StepDep {
    fn into(self) -> XvcDependency {
        XvcDependency::Step(self)
    }
}

impl StepDep {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Diffable for StepDep {
    type Item = Self;
}
