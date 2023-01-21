//! Initialize an Xvc repository
use crate::error::{Error, Result};
use clap::Parser;
use log::{info, warn};
use std::env;
use std::fs;
use std::path::PathBuf;
use xvc_config::XvcConfigInitParams;
use xvc_core::default_project_config;
use xvc_core::types::xvcroot::init_xvc_root;
use xvc_core::util::git::inside_git;
use xvc_core::XvcRoot;
use xvc_logging::watch;
use xvc_pipeline;
use xvc_walker::AbsolutePath;

/// Initialize an XVC repository
#[derive(Debug, Clone, Parser)]
#[command(author, version)]
pub struct InitCLI {
    #[arg(long)]
    /// Path to the directory to be intialized. (default: current directory)
    pub path: Option<PathBuf>,
    #[arg(long)]
    /// Don't require Git
    pub no_git: bool,
    #[arg(long)]
    /// Create the repository even if already initialized.
    /// Overwrites the current .xvc directory
    /// Resets all data and guid, etc.
    pub force: bool,
}

/// Creates `.xvc` directory and all related data structures
///
/// It initializes core data structures.
/// Runs [xvc_pipeline::init] and [xvc_file::init] for crate-wise initialization.
///
/// # Arguments
///
/// - `xvc_root_opt`: Optional [xvc_core::XvcRoot]
/// It's an error to reinit inside an Xvc repository (with `Some(xvc_root)`) normally.
/// It's possible to force reinit with `opts.force` though.
///
/// - `opts`: [Command line options][InitCLI] to `xvc init`
pub fn run(xvc_root_opt: Option<&XvcRoot>, opts: InitCLI) -> Result<XvcRoot> {
    let path = opts
        .clone()
        .path
        .unwrap_or_else(|| env::current_dir().unwrap());
    watch!(&path);
    // Check whether we are inside a repository
    match xvc_root_opt {
        Some(xvc_root) => {
            if opts.force {
                warn!(
                    "Removing previous installation: {:?}",
                    xvc_root.xvc_dir().as_os_str()
                );
                fs::remove_dir_all(xvc_root.xvc_dir())?;
            } else {
                return Err(Error::DirectoryContainsXVCAlready {
                    path: xvc_root.absolute_path().as_os_str().to_os_string(),
                });
            }
        }
        None => {
            info!("No previous repository found in {:?}", path);
        }
    }

    let in_git = inside_git(&path);

    match in_git {
        None => {
            if !opts.no_git {
                return Err(Error::PathNotInGitRepository {
                    path: path.into_os_string(),
                });
            }
        }
        Some(git_root) => {
            info!("Git repository found in: {:?}", git_root);
        }
    }
    let default_configuration = default_project_config(!opts.no_git);
    let config_opts = XvcConfigInitParams {
        default_configuration,
        current_dir: AbsolutePath::from(&path),
        include_system_config: true,
        include_user_config: true,
        project_config_path: None,
        local_config_path: None,
        include_environment_config: true,
        command_line_config: None,
    };
    watch!(config_opts);

    let xvc_root = init_xvc_root(&path, config_opts)?;
    watch!(xvc_root);
    xvc_pipeline::init(&xvc_root)?;
    xvc_file::init(&xvc_root)?;
    Ok(xvc_root)
}
