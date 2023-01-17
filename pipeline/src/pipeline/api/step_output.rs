use crate::error::Result;
use log::info;
use std::path::PathBuf;
use xvc_core::{XvcPath, XvcRoot};
use xvc_ecs::R1NStore;

use crate::{XvcMetricsFormat, XvcOutput, XvcPipeline, XvcStep};

/// Entry point for `xvc pipeline step output` command.
///
/// It loads the pipeline and the step with the given names. Then records
/// `files`, `metrics` and `images` as [XvcOutput]s of the step.
///
/// TODO: This can be split like [crate::pipeline::api::step_dependency::XvcDependencyList].
pub fn cmd_step_output(
    xvc_root: &XvcRoot,
    pipeline_name: &str,
    step_name: String,
    files: Option<Vec<String>>,
    metrics: Option<Vec<String>>,
    images: Option<Vec<String>>,
) -> Result<()> {
    let (pipeline_e, _) = XvcPipeline::from_name(xvc_root, pipeline_name)?;
    let (step_e, step) = XvcStep::from_name(xvc_root, &pipeline_e, &step_name)?;
    let current_dir = xvc_root.config().current_dir()?;
    let mut outputs: Vec<XvcOutput> = Vec::new();

    if let Some(output_files) = files {
        for file in output_files {
            let pathbuf = PathBuf::from(file);
            let path = XvcPath::new(xvc_root, current_dir, &pathbuf)?;
            outputs.push(XvcOutput::File { path });
        }
    }

    if let Some(metrics) = metrics {
        for metric in metrics {
            let pathbuf = PathBuf::from(metric);
            let format = XvcMetricsFormat::from_path(&pathbuf);
            let path = XvcPath::new(xvc_root, current_dir, &pathbuf)?;
            outputs.push(XvcOutput::Metric { format, path });
        }
    }

    if let Some(images) = images {
        for image in images {
            let pathbuf = PathBuf::from(image);
            let path = XvcPath::new(xvc_root, current_dir, &pathbuf)?;
            outputs.push(XvcOutput::Image { path });
        }
    }

    xvc_root.with_r1nstore_mut(|rs: &mut R1NStore<XvcStep, XvcOutput>| {
        for o in &outputs {
            rs.insert(step_e, step.clone(), xvc_root.new_entity(), o.clone());
            info!("Adding {:?}", step.clone());
        }
        Ok(())
    })?;

    Ok(())
}
