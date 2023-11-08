use crate::error::{Error, Result};
use clap::Parser;
use xvc_config::FromConfigKey;
use xvc_core::XvcRoot;

use crate::XvcPipeline;

/// Delete a pipeline
#[derive(Debug, Clone, Parser)]
#[command(name = "delete")]
pub struct DeleteCLI {
    /// Name or GUID of the pipeline to be deleted
    #[arg(long, short)]
    pipeline_name: String,
}

/// Entry point for `xvc pipeline delete` command.
/// It deletes the pipeline with the given name.
/// It is not possible to delete the default pipeline.
pub fn cmd_delete(xvc_root: &XvcRoot, opts: DeleteCLI) -> Result<()> {
    let name = opts.pipeline_name;
    let conf = xvc_root.config();
    let default_pipeline = XvcPipeline::from_conf(conf);
    if name == default_pipeline.name {
        return Err(Error::CannotDeleteDefaultPipeline {
            name: default_pipeline.name,
        });
    }

    let mut pipeline_s = xvc_root.load_store::<XvcPipeline>()?;

    if pipeline_s.len() <= 1 {
        return Err(Error::CannotDeleteLastPipeline);
    }

    let vec_e = pipeline_s.filter(|_, p| p.name == name);
    vec_e.iter().for_each(|(e, _)| {
        pipeline_s.remove(*e);
    });
    xvc_root.save_store(&pipeline_s)?;
    Ok(())
}
