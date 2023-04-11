use serde::{Deserialize, Serialize};

/// Invalidates when the dependency step is invalidated.
#[derive(Debug, Display, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StepDep {
    /// The name of the step
    pub name: String,
}

persist!(StepDep, "step-dependency");

impl StepDep {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
