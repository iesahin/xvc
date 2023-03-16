use crate::error::Result;
use xvc_config::FromConfigKey;
use xvc_core::XvcRoot;
use xvc_logging::{watch, XvcOutputSender};

use crate::pipeline::the_grand_pipeline_loop;
use crate::XvcPipeline;

/// Entry point for `xvc pipeline run` command.
///
/// It loads an [`XvcPipeline`] with the name and runs [`the_grand_pipeline_loop`]
/// with it.
pub fn cmd_run(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    name: Option<String>,
) -> Result<()> {
    let conf = xvc_root.config();
    let pipeline_name = match name {
        Some(name) => name,
        None => XvcPipeline::from_conf(conf).name,
    };
    watch!(pipeline_name);
    the_grand_pipeline_loop(output_snd, xvc_root, pipeline_name)
}
