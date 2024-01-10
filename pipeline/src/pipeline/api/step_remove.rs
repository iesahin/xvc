use xvc_core::XvcRoot;
use xvc_ecs::{R1NStore, XvcStore};
use xvc_logging::{info, XvcOutputSender};

use crate::{
    pipeline::XvcStepInvalidate, Result, XvcDependency, XvcOutput, XvcPipeline, XvcStep,
    XvcStepCommand,
};

/// Remove a step from a pipeline
pub fn cmd_step_remove(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    pipeline_name: &str,
    step_name: String,
) -> Result<()> {
    let (pipeline_e, _) = XvcPipeline::from_name(xvc_root, pipeline_name)?;
    let (step_e, step) = XvcStep::from_name(xvc_root, &pipeline_e, &step_name)?;

    // We'll need this while removing dependencies
    let pipeline_steps = xvc_root
        .load_r1nstore::<XvcPipeline, XvcStep>()?
        .children_of(&pipeline_e)?;

    // Remove dependencies
    xvc_root.with_r1nstore_mut::<XvcStep, XvcDependency>(
        |rs: &mut R1NStore<XvcStep, XvcDependency>| {
            let deps = rs.children_of(&step_e)?;
            for (dep_e, dep) in deps.iter() {
                info!(output_snd, "Removing dep: {}", dep);
                rs.remove_child(*dep_e)?;
            }

            // Remove any dependencies on this step
            for (pipeline_step_e, pipeline_step) in pipeline_steps.iter() {
                let deps = rs.children_of(pipeline_step_e)?;
                for (dep_e, dep) in deps.iter() {
                    if matches!(dep, XvcDependency::Step ( dep ) if dep.name == step.name) {
                        info!(output_snd, "Removing dep {} from {}", dep, pipeline_step);
                        rs.remove_child(*dep_e)?;
                    }
                }
            }
            Ok(())
        },
    )?;

    // Remove outputs
    xvc_root.with_r1nstore_mut::<XvcStep, XvcOutput>(|rs: &mut R1NStore<XvcStep, XvcOutput>| {
        let outputs = rs.children_of(&step_e)?;
        for (output_e, output) in outputs.iter() {
            info!(output_snd, "Removing output: {}", output);
            rs.remove_child(*output_e)?;
        }
        Ok(())
    })?;
    // Remove step

    xvc_root.with_store_mut(|bs: &mut XvcStore<XvcStepCommand>| {
        bs.remove(step_e);
        Ok(())
    })?;

    xvc_root.with_store_mut(|bs: &mut XvcStore<XvcStepInvalidate>| {
        bs.remove(step_e);
        Ok(())
    })?;

    xvc_root.with_r1nstore_mut::<XvcPipeline, XvcStep>(
        |rs: &mut R1NStore<XvcPipeline, XvcStep>| {
            info!(output_snd, "Removing step: {}", step);
            rs.remove_child(step_e)?;
            Ok(())
        },
    )?;

    Ok(())
}
