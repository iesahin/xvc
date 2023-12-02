use crate::{error::Result, XvcPipeline};

use clap::Parser;
use xvc_config::FromConfigKey;
use xvc_core::XvcRoot;
use xvc_logging::XvcOutputSender;

use crate::pipeline::the_grand_pipeline_loop;

/// Run a pipeline
#[derive(Debug, Clone, Parser)]
#[command(name = "run")]
pub struct RunCLI {
    /// Name of the pipeline to run
    #[arg(long, short)]
    pipeline_name: Option<String>,
}

/// Entry point for `xvc pipeline run` command.
///
/// It loads an [`XvcPipeline`] with the name and runs [`the_grand_pipeline_loop`]
/// with it.
pub fn cmd_run(output_snd: &XvcOutputSender, xvc_root: &XvcRoot, opts: RunCLI) -> Result<()> {
    let config = xvc_root.config();
    let default_pipeline = XvcPipeline::from_conf(config);
    let pipeline_name = opts.pipeline_name.unwrap_or(default_pipeline.name);
    the_grand_pipeline_loop(output_snd, xvc_root, pipeline_name)
}

