use crate::error::Result;
use xvc_core::XvcRoot;
use xvc_ecs::{R11Store, XvcStore};

use crate::{pipeline::XvcStepInvalidate, XvcPipeline, XvcStep, XvcStepCommand};

/// Entry point for `xvc pipeline step update` command.
/// Updates the command and invalidation strategy (`when` to run) of the
/// given step.
pub fn cmd_step_update(
    xvc_root: &XvcRoot,
    pipeline_name: &str,
    step_name: String,
    command: Option<String>,
    changed: Option<XvcStepInvalidate>,
) -> Result<()> {
    let (pipeline_e, _) = XvcPipeline::from_name(xvc_root, pipeline_name)?;
    let (step_e, step) = XvcStep::from_name(xvc_root, &pipeline_e, &step_name)?;

    let changed = changed.unwrap_or(XvcStepInvalidate::ByDependencies);

    xvc_root.with_r11store_mut(|rs: &mut R11Store<XvcStep, XvcStepInvalidate>| {
        rs.insert(&step_e, step.clone(), changed);
        Ok(())
    })?;

    if let Some(command) = command {
        xvc_root.with_store_mut(|bs: &mut XvcStore<XvcStepCommand>| {
            let step_command = XvcStepCommand {
                command: command.clone(),
            };
            bs.insert(step_e, step_command);
            Ok(())
        })?;
    };

    Ok(())
}
