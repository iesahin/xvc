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
//     Begin --> DoneWithoutRunning: RunNever
//     Begin --> WaitingDependencySteps: RunConditional
//     WaitingDependencySteps --> WaitingDependencySteps: DependencyStepsRunning
//     WaitingDependencySteps --> Broken: DependencyStepsFinishedBroken
//     WaitingDependencySteps --> CheckingOutputs: DependencyStepsFinishedBrokenIgnored
//     WaitingDependencySteps --> CheckingOutputs: DependencyStepsFinishedSuccessfully
//     CheckingOutputs --> CheckingSuperficialDiffs: OutputsIgnored
//     CheckingOutputs --> CheckingSuperficialDiffs: CheckedOutputs
//     CheckingSuperficialDiffs --> CheckingThoroughDiffs: SuperficialDiffsIgnored
//     CheckingSuperficialDiffs --> ComparingDiffsAndOutputs: SuperficialDiffsNotChanged
//     CheckingSuperficialDiffs --> CheckingThoroughDiffs: SuperficialDiffsChanged
//     CheckingSuperficialDiffs --> Broken: HasMissingDependencies
//     CheckingThoroughDiffs --> ComparingDiffsAndOutputs: ThoroughDiffsNotChanged
//     CheckingThoroughDiffs --> ComparingDiffsAndOutputs: ThoroughDiffsChanged
//     ComparingDiffsAndOutputs --> WaitingToRun: DiffsHasChanged
//     ComparingDiffsAndOutputs --> DoneWithoutRunning: DiffsHasNotChanged
//     DoneWithoutRunning --> Done: CompletedWithoutRunningStep
//     WaitingToRun --> WaitingToRun: ProcessPoolFull
//     WaitingToRun --> Running: StartProcess
//     WaitingToRun --> Broken: CannotStartProcess
//     Running --> Running: WaitProcess
//     Running --> Broken: ProcessTimeout
//     Running --> Done: ProcessCompletedSuccessfully
//     Running --> Broken: ProcessReturnedNonZero
//     Broken --> Broken: KeepBroken
//     Done --> Done: KeepDone
//     Broken --> [*]
//     Done --> [*]
// ```

state_machine! {
    XvcStepState {
        InitialStates { Begin }

        RunNever {
            Begin => DoneWithoutRunning
        }

        RunConditional {
            Begin => WaitingDependencySteps
        }

        DependencyStepsFinishedBrokenIgnored {
            WaitingDependencySteps => CheckingOutputs
        }


        DependencyStepsRunning {
            WaitingDependencySteps => WaitingDependencySteps
        }

        DependencyStepsFinishedSuccessfully {
            WaitingDependencySteps => CheckingOutputs
        }

        DependencyStepsFinishedBroken {
            WaitingDependencySteps => Broken
        }

        OutputsIgnored {
            CheckingOutputs => CheckingSuperficialDiffs
        }

        CheckedOutputs {
            CheckingOutputs => CheckingSuperficialDiffs
        }

        SuperficialDiffsIgnored {
           CheckingSuperficialDiffs => CheckingThoroughDiffs
        }

        SuperficialDiffsNotChanged {
           CheckingSuperficialDiffs => ComparingDiffsAndOutputs
        }

        SuperficialDiffsChanged {
           CheckingSuperficialDiffs => CheckingThoroughDiffs
        }

        HasMissingDependencies {
            CheckingSuperficialDiffs => Broken
        }

        ThoroughDiffsNotChanged {
            CheckingThoroughDiffs => ComparingDiffsAndOutputs
        }

        ThoroughDiffsChanged {
            CheckingThoroughDiffs => ComparingDiffsAndOutputs
        }

        RunAlways {
            ComparingDiffsAndOutputs => WaitingToRun
        }

        DiffsHasChanged {
            ComparingDiffsAndOutputs => WaitingToRun
        }

        DiffsHasNotChanged {
            ComparingDiffsAndOutputs => DoneWithoutRunning
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
            Running => DoneByRunning
        }

        ProcessReturnedNonZero {
            Running => Broken
        }

        KeepBroken {
            Broken => Broken
        }

        KeepDone {
            DoneByRunning => DoneByRunning
        }

        KeepDone {
            DoneWithoutRunning => DoneWithoutRunning
        }
    }

}
