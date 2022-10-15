use crate::error::Result;
use xvc_config::FromConfigKey;
use xvc_core::XvcRoot;

use crate::pipeline::the_grand_pipeline_loop;
use crate::XvcPipeline;

pub fn cmd_run(xvc_root: &XvcRoot, name: Option<String>) -> Result<()> {
    let conf = xvc_root.config();
    let pipeline_name = match name {
        Some(name) => name,
        None => XvcPipeline::from_conf(conf).name,
    };
    the_grand_pipeline_loop(xvc_root, pipeline_name)
}
