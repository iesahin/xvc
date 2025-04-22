use std::path::PathBuf;

use crate::error::Result;
use clap::Parser;
use xvc_core::error::Error as CoreError;
use xvc_core::R11Store;
use xvc_core::{XvcEcsError, XvcPath, XvcRoot};

use crate::{XvcPipeline, XvcPipelineRunDir};

/// Rename, change dir or set a pipeline as default
#[derive(Debug, Clone, Parser)]
#[command(name = "update")]
pub struct UpdateCLI {
    /// Rename the pipeline to
    #[arg(long)]
    rename: Option<String>,

    /// Set the working directory
    #[arg(long, value_hint=clap::ValueHint::DirPath)]
    workdir: Option<PathBuf>,

    /// Set this pipeline as default
    #[arg(long, help = "set this pipeline default")]
    set_default: bool,
}

/// Entry point for `xvc pipeline update` command.
/// Can rename the pipeline, change the working directory or set the pipeline as
/// default.
pub fn cmd_update(xvc_root: &XvcRoot, pipeline_name: &str, opts: UpdateCLI) -> Result<()> {
    let rename = opts.rename;
    let workdir = opts.workdir;
    let default = opts.set_default;
    Ok(
        xvc_root.with_r11store_mut(|rs: &mut R11Store<XvcPipeline, XvcPipelineRunDir>| {
            let name = pipeline_name.to_owned();
            let pipeline_subset_store = rs.left.filter(|_, p| p.name == name);
            if pipeline_subset_store.is_empty() {
                Err(XvcEcsError::KeyNotFound { key: name }.into())
            } else if pipeline_subset_store.len() > 1 {
                Err(XvcEcsError::MultipleCorrespondingKeysFound { value: name }.into())
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
                        pipeline.name.clone_from(newname);
                    }

                    rs.left.update(*pipeline_e, pipeline);
                }
                Ok(())
            }
        })?,
    )
}
