//! Pipeline management commands and data structures
//!
//! This contains CLI structs for `xvc pipeline` subcommands, [`init`] function to
//! run during `xvc init` for pipeline related initialization, [`cmd_pipeline`]
//! and [`handle_step_cli`] functions to dispatch the options to subcommands.
#![warn(missing_docs)]
#![forbid(unsafe_code)]
pub mod error;
mod pipeline;

pub use crate::pipeline::api::{
    dag::cmd_dag, delete::cmd_delete, export::cmd_export, import::cmd_import, list::cmd_list,
    new::cmd_new, run::cmd_run, step_dependency::cmd_step_dependency, step_new::cmd_step_new,
    step_output::cmd_step_output, step_show::cmd_step_show, step_update::cmd_step_update,
    update::cmd_update,
};

use clap::Parser;

use pipeline::api::dag::DagCLI;
use pipeline::api::delete::DeleteCLI;
use pipeline::api::export::ExportCLI;
use pipeline::api::import::ImportCLI;
use pipeline::api::new::NewCLI;
use pipeline::api::update::UpdateCLI;
pub use pipeline::deps;

use pipeline::step::handle_step_cli;
use pipeline::step::StepCLI;
use serde::{Deserialize, Serialize};
use std::io::BufRead;
use std::str::FromStr;
use xvc_config::{conf, FromConfigKey, UpdateFromXvcConfig, XvcConfig};
use xvc_ecs::XvcStore;
use xvc_logging::XvcOutputSender;

use xvc_core::XvcPath;
use xvc_core::XvcRoot;
use xvc_ecs::{self, persist, XvcEntity};

use crate::error::{Error, Result};
pub use crate::pipeline::command::CommandProcess;
pub use crate::pipeline::command::XvcStepCommand;
pub use crate::pipeline::deps::{param::XvcParamFormat, XvcDependency};
pub use crate::pipeline::outs::XvcMetricsFormat;
pub use crate::pipeline::outs::XvcOutput;
pub use crate::pipeline::schema::XvcPipelineSchema;
pub use crate::pipeline::schema::XvcStepSchema;
pub use crate::pipeline::step::XvcStep;
use crate::pipeline::XvcStepInvalidate;

pub use crate::pipeline::api::run::RunCLI;

/// Pipeline management commands
#[derive(Debug, Parser, Clone)]
#[command(name = "pipeline")]
pub struct PipelineCLI {
    /// Name of the pipeline this command applies to
    #[arg(long, short)]
    pub pipeline_name: Option<String>,
    /// Subcommand to run
    #[command(subcommand)]
    pub subcommand: PipelineSubCommand,
}

/// Pipeline management subcommands
#[derive(Debug, Clone, Parser)]
#[command()]
#[allow(clippy::large_enum_variant)]
pub enum PipelineSubCommand {
    /// Create a new pipeline
    New(NewCLI),

    /// Update the name and other attributes of a pipeline
    Update(UpdateCLI),

    /// Delete a pipeline
    Delete(DeleteCLI),

    /// Run a pipeline
    Run(RunCLI),

    /// List all pipelines
    List,

    /// Generate a dot or mermaid diagram for the pipeline
    Dag(DagCLI),

    /// Export the pipeline to a YAML or JSON file to edit
    Export(ExportCLI),

    /// Import the pipeline from a file
    Import(ImportCLI),

    /// Step creation, dependency, output commands
    #[command()]
    Step(StepCLI),
}

impl UpdateFromXvcConfig for PipelineCLI {
    fn update_from_conf(self, conf: &XvcConfig) -> xvc_config::error::Result<Box<Self>> {
        let default_pipeline = XvcPipeline::from_conf(conf);
        let name = Some(self.pipeline_name.clone().unwrap_or(default_pipeline.name));
        Ok(Box::new(Self {
            pipeline_name: name,
            subcommand: self.subcommand.clone(),
        }))
    }
}

/// A pipeline is a collection of steps that are run in a specific order.
/// This struct defines the name of it.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct XvcPipeline {
    /// The name of the pipeline, that's also the unique ID
    pub name: String,
}

impl FromStr for XvcPipeline {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self {
            name: s.to_string(),
        })
    }
}

persist!(XvcPipeline, "xvc-pipeline");
conf!(XvcPipeline, "pipeline.default");

/// A pipeline run directory where the pipeline is run.
/// It should be within the workspace to be portable across systems.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct XvcPipelineRunDir {
    /// The directory to run the command relative to xvc_root
    pub run_dir: XvcPath,
}

persist!(XvcPipelineRunDir, "xvc-pipeline-run-dir");

impl XvcPipeline {
    /// Load a pipeline by name.
    ///
    /// Returns the entity and the pipeline if found. Otherwise returns [Error::NoPipelinesFound].
    pub fn from_name(xvc_root: &XvcRoot, name: &str) -> Result<(XvcEntity, Self)> {
        let all = xvc_root.load_store::<XvcPipeline>()?;
        match all.iter().find(|(_, p)| p.name == name) {
            None => Err(Error::NoPipelinesFound {
                name: name.to_string(),
            }),
            Some((e, pipeline)) => Ok((*e, pipeline.to_owned())),
        }
    }
}

/// Initialize pipeline stores and save them.
///
/// This is to run during `xvc init`.
pub fn init(xvc_root: &XvcRoot) -> Result<()> {
    let conf = xvc_root.config();
    let mut pipeline_store = XvcStore::<XvcPipeline>::new();
    // If there is a system config for default pipeline name, adhere to it
    let initial_name = if let Ok(config_opt) = conf.get_str("pipeline.default") {
        config_opt.option
    } else {
        "default".to_string()
    };

    pipeline_store.insert(xvc_root.new_entity(), XvcPipeline { name: initial_name });

    xvc_root.save_store(&pipeline_store)?;
    xvc_root.save_store(&XvcStore::<XvcPipelineRunDir>::new())?;

    xvc_root.save_store(&XvcStore::<XvcStep>::new())?;
    xvc_root.save_store(&XvcStore::<XvcStepCommand>::new())?;
    xvc_root.save_store(&XvcStore::<XvcDependency>::new())?;
    xvc_root.save_store(&XvcStore::<XvcOutput>::new())?;
    xvc_root.save_store(&XvcStore::<XvcStepInvalidate>::new())?;

    Ok(())
}

/// Run `xvc pipeline` command.
/// This is the entry point for the pipeline subcommand.
/// It dispatches to the subcommands using [PipelineCLI] argument.
pub fn cmd_pipeline<R: BufRead>(
    input: R,
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    command: PipelineCLI,
) -> Result<()> {
    let conf = xvc_root.config();
    let command = command.update_from_conf(conf)?;
    // This should already be filled from the conf if not given
    let pipeline_name = command.pipeline_name.unwrap();
    match command.subcommand {
        PipelineSubCommand::Run(opts) => cmd_run(output_snd, xvc_root, opts),
        PipelineSubCommand::New(opts) => cmd_new(xvc_root, opts),
        PipelineSubCommand::Update(opts) => cmd_update(xvc_root, opts),
        PipelineSubCommand::List => cmd_list(output_snd, xvc_root),
        PipelineSubCommand::Delete(opts) => cmd_delete(xvc_root, opts),
        PipelineSubCommand::Export(opts) => cmd_export(output_snd, xvc_root, opts),
        PipelineSubCommand::Dag(opts) => cmd_dag(output_snd, xvc_root, opts),
        PipelineSubCommand::Import(opts) => cmd_import(input, xvc_root, opts),
        PipelineSubCommand::Step(step_cli) => {
            handle_step_cli(output_snd, xvc_root, &pipeline_name, step_cli)
        }
    }
}

