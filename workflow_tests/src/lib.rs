#![warn(missing_docs)]
#![forbid(unsafe_code)]
//! The main dispatching functions for the entire XVC CLI
use std::path::Path;

use config::XvcVerbosity;

pub use xvc_config as config;
pub use xvc_core as core;
pub use xvc_ecs as ecs;
pub use xvc_file as file;
pub use xvc_logging as logging;
pub use xvc_pipeline as pipeline;

pub use xvc_logging::watch;

use xvc::error::Result;

/// Run xvc in another process with `args` in `xvc_root_opt`.
///
/// See [xvc::cli::test_dispatch].
#[cfg(test)]
pub fn test_dispatch(
    xvc_root_opt: Option<&Path>,
    args: Vec<&str>,
    verbosity: XvcVerbosity,
) -> Result<String> {
    log::trace!("*********** TEST COMMAND ************");

    watch!(args);

    xvc::cli::test_dispatch(xvc_root_opt, &args, verbosity)
}
