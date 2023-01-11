//! Pipeline management commands and data structures
//!
//! This contains CLI structs for `xvc pipeline` subcommands, [`init`] function to
//! run during `xvc init` for pipeline related initialization, [`run`] function
//! to dispatch the options to subcommands.
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

use crossbeam_channel::Sender;
use pipeline::deps;
use pipeline::schema::XvcSchemaSerializationFormat;

use serde::{Deserialize, Serialize};
use std::io::BufRead;
use std::path::PathBuf;
use std::str::FromStr;
use xvc_config::{conf, FromConfigKey, UpdateFromXvcConfig, XvcConfig};
use xvc_ecs::XvcStore;
use xvc_logging::XvcOutputLine;

use xvc_core::XvcPath;
use xvc_core::XvcRoot;
use xvc_ecs::{self, persist, XvcEntity};

use crate::pipeline::api::dag::XvcPipelineDagFormat;

use crate::error::{Error, Result};
pub use crate::pipeline::command::XvcStepCommand;
pub use crate::pipeline::deps::{param::XvcParamFormat, XvcDependency};
pub use crate::pipeline::outs::XvcMetricsFormat;
pub use crate::pipeline::outs::XvcOutput;
pub use crate::pipeline::schema::XvcPipelineSchema;
pub use crate::pipeline::schema::XvcStepSchema;
pub use crate::pipeline::step::XvcStep;
use crate::pipeline::XvcStepInvalidate;

/// Pipeline management commands
#[derive(Debug, Parser)]
#[command(name = "pipeline")]
pub struct PipelineCLI {
    /// Name of the pipeline this command applies
    #[arg(long, short)]
    pub name: Option<String>,
    /// Subcommand to run
    #[command(subcommand)]
    pub subcommand: PipelineSubCommand,
}

/// Pipeline management subcommands
#[derive(Debug, Clone, Parser)]
#[command()]
pub enum PipelineSubCommand {
    /// Create a new pipeline
    #[command()]
    New {
        /// Name of the pipeline this command applies to
        #[arg(long, short)]
        name: String,

        /// Default working directory
        #[arg(short, long)]
        workdir: Option<PathBuf>,

        /// Set this pipeline as default
        #[arg(long)]
        set_default: bool,
    },

    /// Rename, change dir or set a pipeline as default
    #[command()]
    Update {
        /// Name of the pipeline this command applies to
        #[arg(long, short)]
        name: Option<String>,

        /// Rename the pipeline to
        #[arg(long)]
        rename: Option<String>,

        /// Set the working directory
        #[arg(long)]
        workdir: Option<PathBuf>,

        /// Set this pipeline as default
        #[arg(long, help = "set this pipeline default")]
        set_default: bool,
    },

    /// Delete a pipeline
    #[command(about = "Delete a pipeline")]
    Delete {
        /// Name or GUID of the pipeline to be deleted
        #[arg(long, short)]
        name: String,
    },

    /// Run a pipeline
    #[command(about = "Run a pipeline")]
    Run {
        /// Name of the pipeline to run
        #[arg(long, short)]
        name: Option<String>,
    },

    /// List all pipelines
    #[command()]
    List,

    /// Generate a dot or mermaid diagram for the pipeline
    #[command()]
    Dag {
        /// Name of the pipeline to generate the diagram
        #[arg(long, short)]
        name: Option<String>,

        /// Output file. Writes to stdout if not set.
        #[arg(long)]
        file: Option<PathBuf>,

        /// Format for graph. Either dot or mermaid.
        #[arg(long, default_value = "dot")]
        format: XvcPipelineDagFormat,
    },

    /// Export the pipeline to a YAML or JSON file to edit
    #[command()]
    Export {
        /// Name of the pipeline to export
        #[arg(long, short)]
        name: Option<String>,

        /// File to write the pipeline. Writes to stdout if not set.
        #[arg(long)]
        file: Option<PathBuf>,

        /// Output format. One of json or yaml. If not set, the format is
        /// guessed from the file extension. If the file extension is not set,
        /// json is used as default.
        #[arg(long)]
        format: Option<XvcSchemaSerializationFormat>,
    },

    /// Import the pipeline from a file
    #[command()]
    Import {
        /// Name of the pipeline to import.
        /// If not set, the name from the file is used.
        #[arg(long, short)]
        name: Option<String>,

        /// File to read the pipeline. Use stdin if not specified.
        #[arg(long)]
        file: Option<PathBuf>,

        /// Input format. One of json or yaml. If not set, the format is
        /// guessed from the file extension. If the file extension is not set,
        /// json is used as default.        
        #[arg(long)]
        format: Option<XvcSchemaSerializationFormat>,

        /// Overwrite the pipeline even if the name already exists
        #[arg(long)]
        overwrite: bool,
    },

    /// Step creation, dependency, output commands
    #[command()]
    Step(StepCLI),
}

#[derive(Debug, Clone, Parser)]
#[command(name = "step", about = "Step management commands")]
pub struct StepCLI {
    #[command(subcommand)]
    pub subcommand: StepSubCommand,
}

#[derive(Debug, Clone, Parser)]
#[command(about = "Step management commands")]
pub enum StepSubCommand {
    #[command(about = "Add a new step")]
    New {
        #[arg(long, short, help = "Name of the step")]
        step_name: String,
        #[arg(long, short, help = "Command to run the step")]
        command: Option<String>,
        #[arg(long, help = "When to run the command")]
        changed: Option<XvcStepInvalidate>,
    },

    #[command(about = "Update step options")]
    Update {
        #[arg(long, short, help = "Name of the step (that must already be added)")]
        step_name: String,
        #[arg(long, short, help = "Command to run the step")]
        command: Option<String>,
        #[arg(long, help = "When to run the command")]
        changed: Option<XvcStepInvalidate>,
    },

    #[command(about = "Add a dependency to a step in the pipeline")]
    Dependency {
        #[arg(long, short, help = "Name of the step")]
        step_name: String,
        #[arg(
            long = "file",
            help = "Add a file dependency to the step. Can be used multiple times."
        )]
        files: Option<Vec<String>>,
        #[arg(long = "step", help = "Add explicit step dependencies to run")]
        steps: Option<Vec<String>>,
        #[arg(long = "pipeline", help = "Add explicit pipeline dependencies to run")]
        pipelines: Option<Vec<String>>,
        #[arg(
            long = "directory",
            help = "Add a directory dependency to the step. Can be used multiple times."
        )]
        directories: Option<Vec<String>>,
        #[arg(
            long = "glob",
            help = "Add a glob dependency to the step. Can be used multiple times."
        )]
        globs: Option<Vec<String>>,
        #[arg(
            long = "param",
            help = "Add a parameter dependency to the step in the form filename.yaml::model.units . Can be used multiple times."
        )]
        params: Option<Vec<String>>,
        #[arg(
            long = "regex",
            aliases = &["regexp"],
            help = "Add a regex dependency in the form filename.txt:/^regex/"
        )]
        regexps: Option<Vec<String>>,
        #[arg(
            long = "line",
            aliases = &["lines"],
            help = "Add a line dependency in the form filename.txt::123-234"
        )]
        lines: Option<Vec<String>>,
    },

    #[command(about = "Add an output to a step in the pipeline")]
    Output {
        #[arg(long, short, help = "Name of the step")]
        step_name: String,
        #[arg(
            long = "output-file",
            help = "Add a file output to the step. Can be used multiple times."
        )]
        files: Option<Vec<String>>,
        #[arg(
            long = "output-metric",
            help = "Add a metrics output to the step. Can be used multiple times."
        )]
        metrics: Option<Vec<String>>,
        #[arg(
            long = "output-image",
            help = "Add an image output to the step. Can be used multiple times."
        )]
        images: Option<Vec<String>>,
    },

    #[command(about = "Print step configuration")]
    Show {
        #[arg(long, short, help = "Name of the step")]
        step_name: String,
    },
}

impl UpdateFromXvcConfig for PipelineCLI {
    fn update_from_conf(self, conf: &XvcConfig) -> xvc_config::error::Result<Box<Self>> {
        let default_pipeline = XvcPipeline::from_conf(conf);
        let name = Some(self.name.clone().unwrap_or(default_pipeline.name));
        Ok(Box::new(Self {
            name,
            subcommand: self.subcommand.clone(),
        }))
    }
}

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

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct XvcPipelineRunDir {
    /// The directory to run the command relative to xvc_root
    pub run_dir: XvcPath,
}

persist!(XvcPipelineRunDir, "xvc-pipeline-run-dir");

impl XvcPipeline {
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

pub fn run<R: BufRead>(
    input: R,
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    command: PipelineCLI,
) -> Result<()> {
    let conf = xvc_root.config();
    let command = command.update_from_conf(conf)?;
    // This should already be filled from the conf if not given
    let pipeline_name = command.name.unwrap();
    match command.subcommand {
        PipelineSubCommand::Run { name } => cmd_run(xvc_root, name),

        PipelineSubCommand::New {
            name,
            workdir,
            set_default,
        } => cmd_new(xvc_root, &name, workdir, set_default),
        PipelineSubCommand::Update {
            name,
            rename,
            workdir,
            set_default,
        } => cmd_update(
            xvc_root,
            name.unwrap_or(pipeline_name),
            rename,
            workdir,
            set_default,
        ),
        PipelineSubCommand::List => cmd_list(output_snd, xvc_root),
        PipelineSubCommand::Delete { name } => cmd_delete(xvc_root, name),
        PipelineSubCommand::Export { name, file, format } => {
            cmd_export(output_snd, xvc_root, name, file, format)
        }
        PipelineSubCommand::Dag { name, file, format } => {
            cmd_dag(output_snd, xvc_root, name, file, format)
        }
        PipelineSubCommand::Import {
            name,
            file,
            format,
            overwrite,
        } => cmd_import(input, xvc_root, name, file, format, overwrite),
        PipelineSubCommand::Step(step_cli) => handle_step_cli(xvc_root, &pipeline_name, step_cli),
    }
}

pub fn handle_step_cli(xvc_root: &XvcRoot, pipeline_name: &str, command: StepCLI) -> Result<()> {
    match command.subcommand {
        StepSubCommand::New {
            step_name,
            command,
            changed,
        } => cmd_step_new(xvc_root, pipeline_name, step_name, command, changed),
        StepSubCommand::Update {
            step_name,
            command,
            changed,
        } => cmd_step_update(xvc_root, pipeline_name, step_name, command, changed),

        StepSubCommand::Dependency {
            step_name,
            files,
            directories,
            globs,
            params,
            steps,
            pipelines,
            regexps,
            lines,
        } => cmd_step_dependency(
            xvc_root,
            pipeline_name,
            step_name,
            files,
            directories,
            globs,
            params,
            steps,
            pipelines,
            regexps,
            lines,
        ),
        StepSubCommand::Output {
            step_name,
            files,
            metrics,
            images,
        } => cmd_step_output(xvc_root, pipeline_name, step_name, files, metrics, images),
        StepSubCommand::Show { step_name } => cmd_step_show(xvc_root, pipeline_name, step_name),
    }
}
