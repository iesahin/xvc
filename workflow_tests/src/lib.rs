#![forbid(unsafe_code)]

pub use xvc_config as config;
pub use xvc_core as core;
pub use xvc_ecs as ecs;
pub use xvc_file as file;
pub use xvc_logging as logging;
pub use xvc_pipeline as pipeline;

pub use xvc_logging::watch;

use xvc::{cli::XvcCLI, error::Result};

/// Run xvc in another process with `args` in `xvc_root_opt`.
///
/// See [xvc::cli::test_dispatch].
pub fn dispatch(cli_opts: XvcCLI) -> Result<()> {
    xvc::cli::dispatch(cli_opts)
}
