use serde::{Deserialize, Serialize};
use xvc_core::types::diff::Diffable;
use xvc_core::Diff;
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

    fn diff(record: &Option<Self::Item>, actual: &Option<Self::Item>) -> Diff<Self> {
        match (record, actual) {
            (None, None) => unreachable!("Both record and actual are None"),
            (None, Some(actual)) => Diff::RecordMissing {
                actual: actual.clone(),
            },
            (Some(record), None) => Diff::ActualMissing {
                record: record.clone(),
            },
            (Some(record), Some(actual)) => Self::diff_thorough(record, actual),
        }
    }

    fn diff_superficial(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        Self::diff_thorough(record, actual)
    }

    fn diff_thorough(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
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
