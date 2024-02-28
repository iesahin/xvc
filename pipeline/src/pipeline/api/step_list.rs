use crate::{
    error::Result, pipeline::XvcStepInvalidate, Error, XvcPipeline, XvcStep, XvcStepCommand,
};
use itertools::Itertools;
use xvc_core::XvcRoot;
use xvc_ecs::{HStore, R1NStore};
use xvc_logging::{output, watch, XvcOutputSender};

pub fn cmd_step_list(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    pipeline_name: &str,
    names_only: bool,
) -> Result<()> {
    let (pipeline_e, _) = XvcPipeline::from_name(xvc_root, pipeline_name)?;

    xvc_root
        .with_r1nstore(|rs: &R1NStore<XvcPipeline, XvcStep>| {
            let steps: HStore<XvcStep> = rs.children_of(&pipeline_e)?;
            watch!(steps);

            if names_only {
                for (_, step) in steps.into_iter().sorted() {
                    output!(output_snd, "{}", step);
                }
            } else {
                let bs_command = xvc_root.load_store::<XvcStepCommand>()?;
                let bs_invalidate = xvc_root.load_store::<XvcStepInvalidate>()?;

                for (step_e, step) in steps.into_iter().sorted() {
                    let command = &bs_command[&step_e];
                    let invalidate = bs_invalidate.get(&step_e).cloned().unwrap_or_default();
                    output!(output_snd, "{}: {} ({})", step, command, invalidate);
                }
            }

            Ok(())
        })
        .map_err(Error::from)?;

    Ok(())
}
