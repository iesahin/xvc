use std::path::PathBuf;

use crate::error::Result;
use clap::Parser;
use xvc_core::{XvcPath, XvcRoot};
use xvc_core::error::Error as EcsError;
use xvc_core::R11Store;

use crate::{XvcPipeline, XvcPipelineRunDir};

/// Create a new pipeline
#[derive(Debug, Clone, Parser)]
#[command(name = "new")]
pub struct NewCLI {
    /// Default working directory
    #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
    workdir: Option<PathBuf>,
}

/// Entry point for `xvc pipeline new` command.
/// It creates a new pipeline with the given name.
/// If `workdir` is None, uses the default workdir.
pub fn cmd_new(xvc_root: &XvcRoot, pipeline_name: &str, opts: NewCLI) -> Result<()> {
    Ok(
        xvc_root.with_r11store_mut(|rs: &mut R11Store<XvcPipeline, XvcPipelineRunDir>| {
            let name = pipeline_name.to_string();
            if rs.left.iter().any(|(_, p)| p.name == name) {
                Err(EcsError::KeyAlreadyFound {
                    key: name,
                    store: "xvc-pipeline".into(),
                }
                .into())
            } else {
                let pipeline = XvcPipeline { name };
                let p_e = xvc_root.new_entity();
                rs.left.insert(p_e, pipeline);
                if let Some(wd) = &opts.workdir {
                    let conf = xvc_root.config();
                    let current_dir = conf.current_dir()?;
                    let run_dir = XvcPipelineRunDir {
                        run_dir: XvcPath::new(xvc_root, current_dir, wd)?,
                    };
                    rs.right.insert(p_e, run_dir);
                }
                Ok(())
            }
        })?,
    )
}
