#![allow(clippy::enum_variant_names)]

use derive_more::Display;

use crate::error::{Error, Result};
use crate::{
    cmd_step_dependency, cmd_step_new, cmd_step_output, cmd_step_show, cmd_step_update, XvcPipeline,
};
use clap::Parser;
use sad_machine::state_machine;
use serde::{Deserialize, Serialize};
use xvc_core::XvcRoot;
use xvc_ecs::{persist, XvcEntity};
use xvc_logging::XvcOutputSender;

use super::api::step_list::cmd_step_list;
use super::api::step_remove::cmd_step_remove;
use super::XvcStepInvalidate;

/// Step creation, dependency, output commands
#[derive(Debug, Clone, Parser)]
#[command(name = "step")]
pub struct StepCLI {
    /// Step subcommand
    #[command(subcommand)]
    pub subcommand: StepSubCommand,
}

/// Step management subcommands
#[derive(Debug, Clone, Parser)]
#[command()]
pub enum StepSubCommand {
    /// List steps in a pipeline
    #[command()]
    List {
        /// Show only the names, otherwise print commands as well.
        #[arg(long)]
        names_only: bool,
    },

    /// Add a new step
    #[command()]
    New {
        /// Name of the new step
        #[arg(long, short)]
        step_name: String,

        /// Step command to run
        #[arg(long, short)]
        command: String,

        /// When to run the command. One of always, never, by_dependencies (default).
        /// This is used to freeze or invalidate a step manually.
        #[arg(long)]
        when: Option<XvcStepInvalidate>,
    },

    /// Remove a step from a pipeline
    #[command()]
    Remove {
        /// Name of the step to remove
        #[arg(long, short)]
        step_name: String,
    },

    /// Update a step's command or when options.
    #[command(about = "Update step options")]
    Update {
        /// Name of the step to update. The step should already be defined.
        #[arg(long, short)]
        step_name: String,

        /// Step command to run
        #[arg(long, short)]
        command: Option<String>,

        /// When to run the command. One of always, never, by_dependencies (default).
        /// This is used to freeze or invalidate a step manually.
        #[arg(long)]
        when: Option<XvcStepInvalidate>,
    },

    /// Add a dependency to a step
    #[command()]
    Dependency {
        /// Name of the step to add the dependency to
        #[arg(long, short, aliases = &["for", "to"])]
        step_name: String,

        /// Add a generic command output as a dependency. Can be used multiple times.
        /// Please delimit the command with ' ' to avoid shell expansion.
        #[arg(long = "generic")]
        generics: Option<Vec<String>>,

        /// Add a URL dependency to the step. Can be used multiple times.
        #[arg(long = "url")]
        urls: Option<Vec<String>>,

        /// Add a file dependency to the step. Can be used multiple times.
        #[arg(long = "file")]
        files: Option<Vec<String>>,

        /// Add a step dependency to a step. Can be used multiple times.
        /// Steps are referred with their names.
        #[arg(long = "step")]
        steps: Option<Vec<String>>,

        /// Add a glob items dependency to the step.
        ///
        /// You can depend on multiple files and directories with this dependency.
        ///
        /// The difference between this and the glob option is that this option keeps track of all
        /// matching files, but glob only keeps track of the matched files' digest. When you want
        /// to use ${XVC_GLOB_ITEMS}, ${XVC_ADDED_GLOB_ITEMS}, or ${XVC_REMOVED_GLOB_ITEMS}
        /// environment variables in the step command, use the glob-items dependency. Otherwise,
        /// you can use the glob option to save disk space.
        #[arg(long = "glob_items", aliases=&["glob-items", "glob-i"])]
        glob_items: Option<Vec<String>>,

        /// Add a glob dependency to the step. Can be used multiple times.
        ///
        /// You can depend on multiple files and directories with this dependency.
        ///
        /// The difference between this and the glob-items option is that the glob-items option
        /// keeps track of all matching files individually, but this option only keeps track of the
        /// matched files' digest. This dependency uses considerably less disk space.
        #[arg(long = "glob", aliases=&["globs"])]
        globs: Option<Vec<String>>,

        /// Add a parameter dependency to the step in the form filename.yaml::model.units . Can be used multiple times.
        #[arg(long = "param", aliases = &["params"])]
        params: Option<Vec<String>>,

        /// Add a regex dependency in the form filename.txt:/^regex/ . Can be used multiple times.
        ///
        /// The difference between this and the regex option is that the regex-items option keeps
        /// track of all matching lines, but regex only keeps track of the matched lines' digest.
        /// When you want to use ${XVC_REGEX_ITEMS}, ${XVC_ADDED_REGEX_ITEMS},
        /// ${XVC_REMOVED_REGEX_ITEMS} environment variables in the step command, use the regex
        /// option. Otherwise, you can use the regex-digest option to save disk space.
        #[arg(
            long = "regex_items",
            aliases = &["regex-items", "regexp_items", "regexp-items"],
        )]
        regex_items: Option<Vec<String>>,

        /// Add a regex dependency in the form filename.txt:/^regex/ . Can be used multiple times.
        ///
        /// The difference between this and the regex option is that the regex option keeps track
        /// of all matching lines that can be used in the step command. This option only keeps
        /// track of the matched lines' digest.
        #[arg(
            long = "regex",
            aliases = &["regexp"],
        )]
        regexes: Option<Vec<String>>,

        /// Add a line dependency in the form filename.txt::123-234
        ///
        /// The difference between this and the lines option is that the line-items option keeps
        /// track of all matching lines that can be used in the step command. This option only
        /// keeps track of the matched lines' digest. When you want to use ${XVC_ALL_LINE_ITEMS},
        /// ${XVC_ADDED_LINE_ITEMS}, ${XVC_CHANGED_LINE_ITEMS} options in the step command, use the
        /// line option. Otherwise, you can use the lines option to save disk space.
        #[arg(
            long = "line_items",
            aliases = &["line-items", "line-i"],
        )]
        line_items: Option<Vec<String>>,

        /// Add a line digest dependency in the form filename.txt::123-234
        ///
        /// The difference between this and the line-items dependency is that the line option keeps
        /// track of all matching lines that can be used in the step command. This option only
        /// keeps track of the matched lines' digest. If you don't need individual lines to be
        /// kept, use this option to save space.
        #[arg(
            long = "lines",
            aliases = &["line"],
        )]
        lines: Option<Vec<String>>,
    },

    /// Add an output to a step
    #[command()]
    Output {
        /// Name of the step to add the output to
        #[arg(long, short)]
        step_name: String,

        /// Add a file output to the step. Can be used multiple times.
        #[arg(long = "output-file")]
        files: Option<Vec<String>>,

        /// Add a metric output to the step. Can be used multiple times.
        #[arg(long = "output-metric")]
        metrics: Option<Vec<String>>,

        /// Add an image output to the step. Can be used multiple times.
        #[arg(long = "output-image")]
        images: Option<Vec<String>>,
    },

    /// Print step configuration
    #[command()]
    Show {
        /// Name of the step to show
        #[arg(long, short)]
        step_name: String,
    },
}

/// Dispatch `xvc pipeline step` subcommands.
pub fn handle_step_cli(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    pipeline_name: &str,
    command: StepCLI,
) -> Result<()> {
    match command.subcommand {
        StepSubCommand::List {
            names_only: only_names,
        } => cmd_step_list(output_snd, xvc_root, pipeline_name, only_names),

        StepSubCommand::New {
            step_name,
            command,
            when: changed,
        } => cmd_step_new(xvc_root, pipeline_name, step_name, command, changed),

        StepSubCommand::Remove { step_name } => {
            cmd_step_remove(output_snd, xvc_root, pipeline_name, step_name)
        }

        StepSubCommand::Update {
            step_name,
            command,
            when: changed,
        } => cmd_step_update(xvc_root, pipeline_name, step_name, command, changed),

        dep_opts @ StepSubCommand::Dependency { .. } => {
            cmd_step_dependency(output_snd, xvc_root, pipeline_name, dep_opts)
        }

        StepSubCommand::Output {
            step_name,
            files,
            metrics,
            images,
        } => cmd_step_output(xvc_root, pipeline_name, step_name, files, metrics, images),
        StepSubCommand::Show { step_name } => cmd_step_show(xvc_root, pipeline_name, step_name),
    }
}

/// A step (stage) in a pipeline.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd, Display)]
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

