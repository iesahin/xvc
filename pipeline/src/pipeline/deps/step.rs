//! An explicit step dependency in a pipeline. Unlike other dependencies, this depends directly to
//! other steps and runs after them.
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

impl From<StepDep> for XvcDependency {
    fn from(val: StepDep) -> Self {
        XvcDependency::Step(val)
    }
}

impl StepDep {
    /// Create a new step depdenency
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Diffable for StepDep {
    type Item = Self;
}
