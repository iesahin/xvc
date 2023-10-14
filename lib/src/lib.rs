#![warn(missing_docs)]
#![forbid(unsafe_code)]
//! The main dispatching functions for the entire XVC CLI
pub mod cli;
pub mod error;
pub mod init;

pub use xvc_config as config;
pub use xvc_core as core;
pub use xvc_ecs as ecs;
pub use xvc_file as file;
pub use xvc_logging as logging;
pub use xvc_pipeline as pipeline;

pub use xvc_logging::watch;

pub use crate::error::Result;

/// Adds `xvc` as the first elements to `args` and calls [cli::dispatch] after parsing them.
pub fn dispatch(args: Vec<&str>) -> Result<()> {
    let args_with_binary_name = if !args.is_empty() && args[0] != "xvc" {
        vec!["xvc"].into_iter().chain(args).collect()
    } else {
        args
    };

    let cli_opts = cli::XvcCLI::from_str_slice(&args_with_binary_name)?;

    cli::dispatch(cli_opts)
}
