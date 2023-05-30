#![allow(clippy::enum_variant_names)]

use crate::error::{Error, Result};
use crate::XvcPipeline;
use sad_machine::state_machine;
use serde::{Deserialize, Serialize};
use xvc_core::XvcRoot;
use xvc_ecs::{persist, XvcEntity};

/// A step (stage) in a pipeline.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct XvcStep {
    /// Name of the step
    pub name: String,
}

persist!(XvcStep, "xvc-step");

impl XvcStep {
    /// Search for a step with the given name in the given pipeline.
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

    /// Search for a step with the given entity in the given pipeline.
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

// TODO: Link to the Documentation after it's written: https://github.com/iesahin/xvc/issues/202
// ```mermaid
// stateDiagram-v2
//     [*] --> Begin
//     Begin --> NoNeedToRun: RunNever
//     Begin --> WaitingDependencySteps: RunConditional
//     WaitingDependencySteps --> CheckingMissingDependencies: DependencyStepsFinishedBrokenIgnored
//     WaitingDependencySteps --> WaitingDependencySteps: DependencyStepsRunning
//     WaitingDependencySteps --> CheckingMissingDependencies: DependencyStepsFinishedSuccessfully
//     WaitingDependencySteps --> Broken: DependencyStepsFinishedBroken
//     CheckingMissingDependencies --> CheckingMissingOutputs: MissingDependenciesIgnored
//     CheckingMissingDependencies --> Broken: HasMissingDependencies
//     CheckingMissingDependencies --> CheckingMissingOutputs: NoMissingDependencies
//     CheckingMissingOutputs --> CheckingSuperficialDiffs: MissingOutputsIgnored
//     CheckingMissingOutputs --> CheckingSuperficialDiffs: HasMissingOutputs
//     CheckingSuperficialDiffs --> WaitingToRun: DiffsIgnored
//     CheckingSuperficialDiffs --> NoNeedToRun: HasNoNewerDependencies
//     CheckingSuperficialDiffs --> CheckingThoroughDiffs: HasNewerDependencies
//     CheckingThoroughDiffs --> NoNeedToRun: ThoroughDiffsNotChanged
//     CheckingThoroughDiffs --> WaitingToRun: ThoroughDiffsChanged
//     NoNeedToRun --> Done: CompletedWithoutRunningStep
//     WaitingToRun --> WaitingToRun: ProcessPoolFull
//     WaitingToRun --> Running: StartProcess
//     WaitingToRun --> Broken: CannotStartProcess
//     Running --> Running: WaitProcess
//     Running --> Broken: ProcessTimeout
//     Running --> Done: ProcessCompletedSuccessfully
//     Running --> Broken: ProcessReturnedNonZero
//     Broken --> Broken: HasBroken
//     Done --> Done: HasDone
//     Done --> [*]
//     Broken --> [*]
// ```

state_machine! {
    XvcStepState {
        InitialStates { Begin }

        RunNever {
            Begin => NoNeedToRun
        }

        RunConditional {
            Begin => WaitingDependencySteps
        }


        DependencyStepsFinishedBrokenIgnored {
            WaitingDependencySteps => CheckingMissingDependencies
        }


        DependencyStepsRunning {
            WaitingDependencySteps => WaitingDependencySteps
        }


        DependencyStepsFinishedSuccessfully {
            WaitingDependencySteps => CheckingMissingDependencies
        }


        DependencyStepsFinishedBroken {
            WaitingDependencySteps => Broken
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
            CheckingMissingOutputs => CheckingSuperficialDiffs
        }

        HasMissingOutputs {
            CheckingMissingOutputs => CheckingSuperficialDiffs
        }

        HasNoMissingOutputs {
            CheckingMissingOutputs => CheckingSuperficialDiffs
        }

            SuperficialDiffsIgnored {
                CheckingSuperficialDiffs => CheckingThoroughDiffs
            }

            SuperficialDiffsNotChanged {
                CheckingSuperficialDiffs => NoNeedToRun
            }


            SuperficialDiffsChanged {
                 CheckingSuperficialDiffs => CheckingThoroughDiffs
             }
//     CheckingThoroughDiffs --> NoNeedToRun: ThoroughDiffsNotChanged
        ThoroughDiffsNotChanged {
            CheckingThoroughDiffs => NoNeedToRun
        }

//     CheckingThoroughDiffs --> WaitingToRun: ThoroughDiffsChanged
        ThoroughDiffsChanged {
            CheckingThoroughDiffs => WaitingToRun
        }
//     NoNeedToRun --> Done: CompletedWithoutRunningStep
        CompletedWithoutRunningStep {
            NoNeedToRun => Done
        }

//     WaitingToRun --> WaitingToRun: ProcessPoolFull

        ProcessPoolFull {
            WaitingToRun => WaitingToRun
        }

//     WaitingToRun --> Running: StartProcess
        StartProcess {
            WaitingToRun => Running
        }

//     WaitingToRun --> Broken: CannotStartProcess
        CannotStartProcess {
            WaitingToRun => Broken
        }

//     Running --> Running: WaitProcess
        WaitProcess {
            Running => Running
        }

//     Running --> Broken: ProcessTimeout
        ProcessTimeout {
            Running => Broken
        }

//     Running --> Done: ProcessCompletedSuccessfully
        ProcessCompletedSuccessfully {
            Running => Done
        }

//     Running --> Broken: ProcessReturnedNonZero
        ProcessReturnedNonZero {
            Running => Broken
        }

        KeepBroken {
            Broken => Broken
        }

        KeepDone {
            Done => Done
        }
    }

}
