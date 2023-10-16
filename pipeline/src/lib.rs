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
    new::cmd_new, run::cmd_run, step_new::cmd_step_new, step_output::cmd_step_output,
    step_show::cmd_step_show, step_update::cmd_step_update, update::cmd_update,
};

use clap::Parser;

use pipeline::api::step_dependency::XvcDependencyList;
pub use pipeline::deps;
use pipeline::schema::XvcSchemaSerializationFormat;

use serde::{Deserialize, Serialize};
use std::io::BufRead;
use std::path::PathBuf;
use std::str::FromStr;
use xvc_config::{conf, FromConfigKey, UpdateFromXvcConfig, XvcConfig};
use xvc_ecs::XvcStore;
use xvc_logging::{watch, XvcOutputSender};

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
#[allow(clippy::large_enum_variant)]
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
        #[arg(long, short)]
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

impl UpdateFromXvcConfig for PipelineCLI {
    fn update_from_conf(self, conf: &XvcConfig) -> xvc_config::error::Result<Box<Self>> {
        let default_pipeline = XvcPipeline::from_conf(conf);
        let name = Some(self.name.clone().unwrap_or(default_pipeline.name));
        watch!(name);
        Ok(Box::new(Self {
            name,
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
    let pipeline_name = command.name.unwrap();
    match command.subcommand {
        PipelineSubCommand::Run { name } => {
            let pipeline_name = name.unwrap_or(pipeline_name);
            cmd_run(output_snd, xvc_root, pipeline_name)
        }

        PipelineSubCommand::New { name, workdir } => cmd_new(xvc_root, &name, workdir),
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
            let pipeline_name = name.unwrap_or(pipeline_name);
            cmd_export(output_snd, xvc_root, pipeline_name, file, format)
        }
        PipelineSubCommand::Dag { name, file, format } => {
            let pipeline_name = name.unwrap_or(pipeline_name);
            cmd_dag(output_snd, xvc_root, pipeline_name, file, format)
        }
        PipelineSubCommand::Import {
            name,
            file,
            format,
            overwrite,
        } => {
            let pipeline_name = name.unwrap_or(pipeline_name);
            cmd_import(input, xvc_root, pipeline_name, file, format, overwrite)
        }
        PipelineSubCommand::Step(step_cli) => {
            handle_step_cli(output_snd, xvc_root, &pipeline_name, step_cli)
        }
    }
}

/// Dispatch `xvc pipeline step` subcommands.
pub fn handle_step_cli(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    pipeline_name: &str,
    command: StepCLI,
) -> Result<()> {
    match command.subcommand {
        StepSubCommand::New {
            step_name,
            command,
            when: changed,
        } => cmd_step_new(xvc_root, pipeline_name, step_name, command, changed),
        StepSubCommand::Update {
            step_name,
            command,
            when: changed,
        } => cmd_step_update(xvc_root, pipeline_name, step_name, command, changed),

        StepSubCommand::Dependency {
            step_name,
            generics,
            urls,
            files,
            glob_items,
            globs,
            params,
            steps,
            regex_items,
            regexes,
            line_items,
            lines,
        } => XvcDependencyList::new(output_snd, xvc_root, pipeline_name, &step_name)?
            .files(files)?
            .glob_items(glob_items)?
            .globs(globs)?
            .params(params)?
            .steps(steps)?
            .generic_commands(generics)?
            .regexes(regexes)?
            .regex_items(regex_items)?
            .lines(lines)?
            .line_items(line_items)?
            .urls(urls)?
            .record(),
        StepSubCommand::Output {
            step_name,
            files,
            metrics,
            images,
        } => cmd_step_output(xvc_root, pipeline_name, step_name, files, metrics, images),
        StepSubCommand::Show { step_name } => cmd_step_show(xvc_root, pipeline_name, step_name),
    }
}
