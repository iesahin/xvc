use crate::error::{Error, Result};
use xvc_core::XvcRoot;
use xvc_ecs::{R11Store, R1NStore, XvcStore};

use crate::{pipeline::XvcStepInvalidate, XvcPipeline, XvcStep, XvcStepCommand};

/// Creates a new step
pub fn cmd_step_new(
    xvc_root: &XvcRoot,
    pipeline_name: &str,
    step_name: String,
    command: String,
    changed: Option<XvcStepInvalidate>,
) -> Result<()> {
    let (pipeline_e, pipeline) = XvcPipeline::from_name(xvc_root, pipeline_name)?;
    let (step_e, step) = match XvcStep::from_name(xvc_root, &pipeline_e, &step_name) {
        Err(Error::StepNotFoundInPipeline { .. }) => {
            Ok((xvc_root.new_entity(), XvcStep { name: step_name }))
        }
        Ok(_) => Err(Error::StepAlreadyFoundInPipeline {
            step_name,
            pipeline_name: pipeline_name.to_string(),
        }),
        Err(err) => Err(err),
    }?;

    let changed = changed.unwrap_or_default();

    xvc_root.with_r11store_mut(|rs: &mut R11Store<XvcStep, XvcStepInvalidate>| {
        rs.insert(&step_e, step.clone(), changed);
        Ok(())
    })?;

    xvc_root.with_store_mut(|bs: &mut XvcStore<XvcStepCommand>| {
        let step_command = XvcStepCommand {
            command: command.clone(),
        };
        bs.insert(step_e, step_command);
        Ok(())
    })?;

    xvc_root.with_r1nstore_mut(|rs: &mut R1NStore<XvcPipeline, XvcStep>| {
        rs.insert(pipeline_e, pipeline.clone(), step_e, step.clone());
        Ok(())
    })?;

    Ok(())
}
