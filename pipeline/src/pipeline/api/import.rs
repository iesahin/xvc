use crate::error::{Error, Result};
use log::warn;
use std::{fs, io::BufRead, path::PathBuf};
use xvc_core::XvcRoot;
use xvc_ecs::{R11Store, R1NStore};

use crate::{
    pipeline::{schema::XvcSchemaSerializationFormat, XvcStepInvalidate},
    XvcDependency, XvcOutput, XvcPipeline, XvcPipelineRunDir, XvcPipelineSchema, XvcStep,
    XvcStepCommand,
};

/// Entry point for `xvc pipeline import` command.
/// Reads a pipeline definition in JSON or YAML formats and creates/updates it.
/// If `name` is None, uses the pipeline name from the file.
/// If `file` is None, reads from stdin.
/// If `format` is None, uses the file extension to determine the format.
/// If `overwrite` is true, overwrites the pipeline if it already exists.
pub fn cmd_import<R: BufRead>(
    input: R,
    xvc_root: &XvcRoot,
    pipeline_name: String,
    file: Option<PathBuf>,
    format: Option<XvcSchemaSerializationFormat>,
    overwrite: bool,
) -> Result<()> {
    let (content, format) = match file {
        None => {
            if let Some(format) = format {
                let mut buf = String::new();
                for line in input.lines() {
                    buf.push_str(
                        &(line.unwrap_or_else(|e| {
                            Error::from(e).warn();
                            "".to_string()
                        })),
                    );
                    buf.push('\n');
                }
                Ok((buf, format))
            } else {
                Err(Error::FormatSpecificationRequired)
            }
        }
        Some(path) => {
            // explicit format overrides the file extension
            let format = match format {
                Some(format) => format,
                None => XvcSchemaSerializationFormat::from_path(&path)?,
            };
            let content = fs::read_to_string(&path)?;
            Ok((content, format))
        }
    }?;

    let schema: XvcPipelineSchema = match format {
        XvcSchemaSerializationFormat::Json => serde_json::from_str(&content)?,
        XvcSchemaSerializationFormat::Yaml => serde_yaml::from_str(&content)?,
    };

    assert!(schema.version == 1);

    if let Ok((pipeline_e, pipeline)) = XvcPipeline::from_name(xvc_root, &pipeline_name) {
        if !overwrite {
            return Err(Error::PipelineAlreadyFound {
                name: pipeline.name,
            });
        } else {
            // Delete the pipeline entity
            let mut pipeline_s = xvc_root.load_store::<XvcPipeline>()?;
            pipeline_s.remove(pipeline_e);
            // We don't delete steps or other entities here. They can be removed by some `fsck` command.
            xvc_root.save_store(&pipeline_s)?;
            warn!("Deleting the older pipeline!");
        }
    }

    let pipeline_e = xvc_root.new_entity();
    let pipeline = XvcPipeline {
        name: pipeline_name,
    };
    xvc_root.with_r11store_mut(|rs: &mut R11Store<XvcPipeline, XvcPipelineRunDir>| {
        let run_dir = XvcPipelineRunDir {
            run_dir: schema.workdir.clone(),
        };
        rs.insert(&pipeline_e, pipeline.clone(), run_dir);
        Ok(())
    })?;

    for step_schema in schema.steps {
        let step_e = xvc_root.new_entity();
        let step = XvcStep {
            name: step_schema.name,
        };
        xvc_root.with_r1nstore_mut(|rs: &mut R1NStore<XvcPipeline, XvcStep>| {
            rs.insert(pipeline_e, pipeline.clone(), step_e, step.clone());
            Ok(())
        })?;

        xvc_root.with_r11store_mut(|rs: &mut R11Store<XvcStep, XvcStepCommand>| {
            let step_command = XvcStepCommand {
                command: step_schema.command.clone(),
            };
            rs.insert(&step_e, step.clone(), step_command);
            Ok(())
        })?;

        xvc_root.with_r11store_mut(|rs: &mut R11Store<XvcStep, XvcStepInvalidate>| {
            rs.insert(&step_e, step.clone(), step_schema.invalidate);
            Ok(())
        })?;

        xvc_root.with_r1nstore_mut(|rs: &mut R1NStore<XvcStep, XvcDependency>| {
            for dep in step_schema.dependencies.clone() {
                let dep_e = xvc_root.new_entity();
                rs.insert(step_e, step.clone(), dep_e, dep);
            }
            Ok(())
        })?;

        xvc_root.with_r1nstore_mut(|rs: &mut R1NStore<XvcStep, XvcOutput>| {
            for out in step_schema.outputs.clone() {
                let out_e = xvc_root.new_entity();
                rs.insert(step_e, step.clone(), out_e, out);
            }
            Ok(())
        })?;
    }
    Ok(())
}
