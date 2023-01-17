use std::path::PathBuf;

use crate::error::Result;
use xvc_core::error::Error as CoreError;
use xvc_core::{XvcPath, XvcRoot};
use xvc_ecs::error::Error as EcsError;
use xvc_ecs::R11Store;

use crate::{XvcPipeline, XvcPipelineRunDir};

/// Entry point for `xvc pipeline update` command.
/// Can rename the pipeline, change the working directory or set the pipeline as
/// default.
pub fn cmd_update(
    xvc_root: &XvcRoot,
    name: String,
    rename: Option<String>,
    workdir: Option<PathBuf>,
    default: bool,
) -> Result<()> {
    Ok(
        xvc_root.with_r11store_mut(|rs: &mut R11Store<XvcPipeline, XvcPipelineRunDir>| {
            let name = name.to_owned();
            let pipeline_subset_store = rs.left.filter(|_, p| p.name == name);
            if pipeline_subset_store.is_empty() {
                Err(EcsError::KeyNotFound { key: name }.into())
            } else if pipeline_subset_store.len() > 1 {
                Err(EcsError::MultipleCorrespondingKeysFound { value: name }.into())
            } else {
                if let Some((pipeline_e, pipeline)) = pipeline_subset_store.first() {
                    let mut pipeline = pipeline.clone();
                    if default {
                        // TODO: Implement setting the default pipeline
                        return Err(CoreError::Todo("Setting default pipeline").error());
                    }
                    if let Some(wd) = &workdir {
                        let current_dir = xvc_root.config().current_dir()?;
                        rs.right.insert(
                            *pipeline_e,
                            XvcPipelineRunDir {
                                run_dir: XvcPath::new(xvc_root, current_dir, wd)?,
                            },
                        );
                    }
                    if let Some(newname) = &rename {
                        pipeline.name = newname.to_owned();
                    }

                    rs.left.update(*pipeline_e, pipeline);
                }
                Ok(())
            }
        })?,
    )
}
