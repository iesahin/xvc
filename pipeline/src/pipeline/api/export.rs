use crate::error::{Error, Result};
use crossbeam_channel::Sender;
use std::{fs, path::PathBuf};
use xvc_config::FromConfigKey;
use xvc_core::{
    util::serde::{to_json, to_yaml},
    XvcPath, XvcRoot,
};
use xvc_ecs::{HStore, R11Store, R1NStore, XvcEntity, XvcStore};
use xvc_logging::XvcOutputLine;

use crate::{
    pipeline::{schema::XvcSchemaSerializationFormat, XvcStepInvalidate},
    XvcDependency, XvcOutput, XvcPipeline, XvcPipelineRunDir, XvcPipelineSchema, XvcStep,
    XvcStepCommand, XvcStepSchema,
};

pub fn cmd_export(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    name: Option<String>,
    file: Option<PathBuf>,
    format: Option<XvcSchemaSerializationFormat>,
) -> Result<()> {
    let conf = xvc_root.config();
    let name = match name {
        Some(name) => name,
        None => XvcPipeline::from_conf(conf).name,
    };

    let mut p_res: Result<(XvcEntity, XvcPipeline)> =
        Err(Error::CannotFindPipeline { name: name.clone() });

    xvc_root.with_store(|bs: &XvcStore<XvcPipeline>| {
        let name = name.clone();
        if let Some((e, p)) = bs.iter().find(|(_, p)| p.name == name) {
            p_res = Ok((*e, p.clone()));
        }
        Ok(())
    })?;

    let (pipeline_e, pipeline) = p_res?;

    let mut workdir_res: Option<XvcPipelineRunDir> = None;

    xvc_root.with_r11store(|rs: &R11Store<XvcPipeline, XvcPipelineRunDir>| {
        if let Some((_, rd)) = rs.left_to_right(&pipeline_e) {
            workdir_res = Some(rd.clone());
        }
        Ok(())
    })?;

    let mut steps: XvcStore<XvcStep> = XvcStore::new();

    xvc_root.with_r1nstore(|rs: &R1NStore<XvcPipeline, XvcStep>| {
        steps = rs.children_of(&pipeline_e)?;
        Ok(())
    })?;

    let commands: XvcStore<XvcStepCommand> = xvc_root
        .load_store::<XvcStepCommand>()?
        .subset(steps.keys().cloned())?;

    let step_invalidate: XvcStore<XvcStepInvalidate> = xvc_root
        .load_store::<XvcStepInvalidate>()?
        .subset(steps.keys().cloned())?;

    let mut deps: HStore<XvcStore<XvcDependency>> = HStore::new();

    xvc_root.with_r1nstore(|rs: &R1NStore<XvcStep, XvcDependency>| {
        for step_e in steps.keys() {
            deps.insert(*step_e, rs.children_of(step_e)?);
        }
        Ok(())
    })?;

    let mut outs: HStore<XvcStore<XvcOutput>> = HStore::new();

    xvc_root.with_r1nstore(|rs: &R1NStore<XvcStep, XvcOutput>| {
        for step_e in steps.keys() {
            outs.insert(*step_e, rs.children_of(step_e)?);
        }
        Ok(())
    })?;

    // Generate the output

    let mut step_schemas = Vec::<XvcStepSchema>::with_capacity(steps.len());
    for (e, s) in steps.iter() {
        let ss = XvcStepSchema {
            name: s.name.clone(),
            command: commands[e].command.clone(),
            invalidate: step_invalidate.get(e).cloned().unwrap_or_default(),
            dependencies: deps[e].values().cloned().collect(),
            outputs: outs[e].values().cloned().collect(),
        };
        step_schemas.push(ss);
    }

    let workdir = match workdir_res {
        Some(wd) => wd.run_dir,
        None => XvcPath::root_path()?,
    };

    let pipeline_schema = XvcPipelineSchema {
        version: 1,
        name: pipeline.name,
        workdir,
        steps: step_schemas,
    };

    let output_format = match format {
        None => match file.as_deref() {
            None => XvcSchemaSerializationFormat::Json,
            Some(path) => XvcSchemaSerializationFormat::from_path(path)?,
        },
        Some(format) => format,
    };

    let output = match output_format {
        XvcSchemaSerializationFormat::Json => {
            let value = to_json(&pipeline_schema)?;
            serde_json::to_string_pretty(&value)?
        }
        XvcSchemaSerializationFormat::Yaml => to_yaml(&pipeline_schema)?,
    };
    match file {
        Some(path) => fs::write(path, output).map_err(|e| e.into()),
        None => Ok(output_snd.send(format!("{}", output).into()).unwrap()),
    }
}
