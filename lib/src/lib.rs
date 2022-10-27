#![warn(missing_docs)]
#![forbid(unsafe_code)]
//! The main dispatching functions for the entire XVC CLI
pub mod cli;
pub mod error;
pub mod init;
use config::XvcVerbosity;
use xvc_core::XvcRoot;

pub use xvc_config as config;
pub use xvc_core as core;
pub use xvc_ecs as ecs;
pub use xvc_file as file;
pub use xvc_logging as logging;
pub use xvc_pipeline as pipeline;

pub use xvc_logging::watch;

use crate::error::Result;

use clap::Parser;

/// Adds `xvc` as the first elements to `args` and calls [cli::dispatch] after parsing them.
pub fn dispatch(args: Vec<&str>) -> Result<()> {
    let args_with_binary_name = if !args.is_empty() && args[0] != "xvc" {
        vec!["xvc"].into_iter().chain(args.into_iter()).collect()
    } else {
        args
    };

    let cli_opts = cli::XvcCLI::parse_from(args_with_binary_name);

    cli::dispatch(cli_opts)
}

/// Ensures `xvc` is the first element in `args`, and runs [cli::test_dispatch] after parsing them.
/// It allows to run commands out of xvc directories.
/// For detailed logs, set `verbosity` to [XvcVerbosity::Trace]
pub fn test_dispatch(
    xvc_root_opt: Option<&XvcRoot>,
    args: Vec<&str>,
    verbosity: XvcVerbosity,
) -> Result<String> {
    log::trace!("*********** TEST COMMAND ************");

    watch!(args);

    let args_with_binary_name = if !args.is_empty() && args[0] != "xvc" {
        vec!["xvc"].into_iter().chain(args.into_iter()).collect()
    } else {
        args
    };

    let cli_opts = cli::XvcCLI::parse_from(args_with_binary_name);
    watch!(cli_opts);

    cli::test_dispatch(xvc_root_opt, cli_opts, verbosity)
}
