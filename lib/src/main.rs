#![warn(missing_docs)]
//! The entry point for xvc cli
use xvc::error::Result;

/// The entry point of the `xvc` cli.
///
/// It parses the command line arguments [xvc::cli::XvcCLI] and calls [xvc::cli::dispatch]
fn main() -> Result<()> {
    let cli_opts = xvc::cli::XvcCLI::from_args_os(std::env::args_os())?;
    xvc::cli::dispatch(cli_opts)
}
