use crate::error::Result;
use comfy_table::Table;

use xvc_core::XvcRoot;
use xvc_ecs::R11Store;
use xvc_logging::{output, XvcOutputSender};

use crate::{XvcPipeline, XvcPipelineRunDir};

/// Entry point for `xvc pipeline list` command.
/// Lists all pipelines and their run directories.
pub fn cmd_list(output_snd: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<()> {
    Ok(
        xvc_root.with_r11store(|rs: &R11Store<XvcPipeline, XvcPipelineRunDir>| {
            let mut table = Table::new();
            table.set_header(vec!["Name", "Run Dir"]);
            let all = &rs.left;
            for (e, pipeline) in all.iter() {
                let rundir_str = if let Some((_, rd)) = rs.left_to_right(e) {
                    rd.run_dir.to_string()
                } else {
                    "".to_owned()
                };
                table.add_row(vec![&pipeline.name, &rundir_str]);
            }
            output!(output_snd, "{}", table);
            Ok(())
        })?,
    )
}
