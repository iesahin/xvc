use crate::error::Error;
use xvc_core::{util::serde::to_json, XvcRoot};
use xvc_ecs::{R1NStore, XvcStore};

use crate::{
    pipeline::XvcStepInvalidate, XvcDependency, XvcOutput, XvcPipeline, XvcStep, XvcStepCommand,
    XvcStepSchema,
};

/// Entry point for `xvc pipeline step show` command.
///
/// Prints the step information for the given step in JSON format.
pub fn cmd_step_show(
    xvc_root: &XvcRoot,
    pipeline_name: &str,
    step_name: String,
) -> Result<(), Error> {
    let (pipeline_e, _) = XvcPipeline::from_name(xvc_root, pipeline_name)?;
    let (step_e, step) = XvcStep::from_name(xvc_root, &pipeline_e, &step_name)?;

    let bs_command = xvc_root.load_store::<XvcStepCommand>()?;
    let command = &bs_command[&step_e];

    let bs_invalidate = xvc_root.load_store::<XvcStepInvalidate>()?;
    let invalidate = bs_invalidate.get(&step_e).cloned().unwrap_or_default();

    let mut deps: XvcStore<XvcDependency> = XvcStore::new();
    xvc_root.with_r1nstore(|rs: &R1NStore<XvcStep, XvcDependency>| {
        for (dep_e, dep) in rs.children_of(&step_e)?.iter() {
            deps.insert(*dep_e, dep.clone());
        }
        Ok(())
    })?;

    let mut outs: XvcStore<XvcOutput> = XvcStore::new();
    xvc_root.with_r1nstore(|rs: &R1NStore<XvcStep, XvcOutput>| {
        for (out_e, out) in rs.children_of(&step_e)?.iter() {
            outs.insert(*out_e, out.clone());
        }
        Ok(())
    })?;

    let ss = XvcStepSchema {
        name: step.name,
        command: command.command.clone(),
        invalidate,
        dependencies: deps.iter().map(|(_, v)| v.clone()).collect(),
        outputs: outs.iter().map(|(_, v)| v.clone()).collect(),
    };

    println!("{}", to_json(&ss)?);

    Ok(())
}
