#![warn(missing_docs)]
#![forbid(unsafe_code)]
//! The main dispatching functions for the entire XVC CLI
use config::XvcVerbosity;
use xvc_core::XvcRoot;

pub use xvc_config as config;
pub use xvc_core as core;
pub use xvc_ecs as ecs;
pub use xvc_file as file;
pub use xvc_logging as logging;
pub use xvc_pipeline as pipeline;

pub use xvc_logging::watch;

use xvc::error::Result;

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

    let cli_opts = xvc::cli::XvcCLI::from_str_slice(&args_with_binary_name)?;
    watch!(cli_opts);

    xvc::cli::test_dispatch(xvc_root_opt, cli_opts, verbosity)
}
