#![allow(clippy::enum_variant_names)]

use crate::error::{Error, Result};
use crate::XvcPipeline;
use sad_machine::state_machine;
use serde::{Deserialize, Serialize};
use xvc_core::XvcRoot;
use xvc_ecs::{persist, XvcEntity};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct XvcStep {
    pub name: String,
}

persist!(XvcStep, "xvc-step");

impl XvcStep {
    pub fn from_name(
        xvc_root: &XvcRoot,
        pipeline_e: &XvcEntity,
        step_name: &str,
    ) -> Result<(XvcEntity, Self)> {
        let step = XvcStep {
            name: step_name.to_string(),
        };

        let pipeline_step_store = xvc_root.load_r1nstore::<XvcPipeline, XvcStep>()?;
        let pipeline_steps = pipeline_step_store.children_of(pipeline_e)?;
        match pipeline_steps.entity_by_value(&step) {
            Some(step_e) => Ok((step_e, step)),
            None => Err(Error::StepNotFoundInPipeline {
                step: step_name.to_string(),
            }),
        }
    }
    #[allow(dead_code)]
    pub fn from_entity(
        xvc_root: &XvcRoot,
        pipeline_e: &XvcEntity,
        step_e: &XvcEntity,
    ) -> Result<(XvcEntity, Self)> {
        let pipeline_step_store = xvc_root.load_r1nstore::<XvcPipeline, XvcStep>()?;
        let pipeline_steps = pipeline_step_store.children_of(pipeline_e)?;
        match pipeline_steps.get(step_e) {
            Some(step) => Ok((*step_e, step.clone())),
            None => Err(Error::StepNotFoundInPipeline {
                step: format!("Step with entity {}", step_e),
            }),
        }
    }
}

state_machine! {
    XvcStepState {
        InitialStates { Begin }

        RunNever {
            Begin => NoNeedToRun
        }

        RunConditional {
            Begin => WaitingDependencySteps
        }

        DependencyStepsRunningIgnored {
            WaitingDependencySteps => CheckingMissingDependencies
        }

        DependencyStepsRunning {
            WaitingDependencySteps => WaitingDependencySteps
        }

        DependencyStepsFinishedBroken {
            WaitingDependencySteps => Broken
        }

        DependencyStepsFinishedSuccessfully {
            WaitingDependencySteps => CheckingMissingDependencies
        }

        DependencyStepsFinishedBrokenIgnored {
            WaitingDependencySteps => CheckingMissingDependencies
        }

        MissingDependenciesIgnored {
            CheckingMissingDependencies => CheckingMissingOutputs
        }

        HasMissingDependencies {
            CheckingMissingDependencies => Broken
        }

        NoMissingDependencies {
            CheckingMissingDependencies => CheckingMissingOutputs
        }

        MissingOutputsIgnored {
            CheckingMissingOutputs => CheckingTimestamps
        }

        HasMissingOutputs {
            CheckingMissingOutputs => WaitingToRun
        }

        HasNoMissingOutputs {
            CheckingMissingOutputs => CheckingTimestamps
        }

        TimestampsIgnored {
            CheckingTimestamps => CheckingDependencyContentDigest
        }

        HasNewerDependencies {
            CheckingTimestamps => WaitingToRun
        }

        HasNoNewerDependencies {
            CheckingTimestamps => CheckingDependencyContentDigest
        }

        ContentDigestIgnored {
            CheckingDependencyContentDigest => NoNeedToRun
        }

        ContentDigestChanged {
            CheckingDependencyContentDigest => WaitingToRun
        }

        ContentDigestNotChanged {
            CheckingDependencyContentDigest => NoNeedToRun
        }

        CompletedWithoutRunningStep {
            NoNeedToRun => Done
        }

        ProcessPoolFull {
            WaitingToRun => WaitingToRun
        }

        StartProcess {
            WaitingToRun => Running
        }

        CannotStartProcess {
            WaitingToRun => Broken
        }

        WaitProcess {
            Running => Running
        }

        ProcessTimeout {
            Running => Broken
        }

        ProcessCompletedSuccessfully {
            Running => Done
        }

        ProcessReturnedNonZero {
            Running => Broken
        }

        HasBroken {
            Broken => Broken
        }

        HasDone {
            Done => Done
        }
    }

}
