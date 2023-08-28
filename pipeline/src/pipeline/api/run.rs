use crate::error::Result;

use xvc_core::XvcRoot;
use xvc_logging::{XvcOutputSender};

use crate::pipeline::the_grand_pipeline_loop;


/// Entry point for `xvc pipeline run` command.
///
/// It loads an [`XvcPipeline`] with the name and runs [`the_grand_pipeline_loop`]
/// with it.
pub fn cmd_run(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    pipeline_name: String,
) -> Result<()> {
    the_grand_pipeline_loop(output_snd, xvc_root, pipeline_name)
}
