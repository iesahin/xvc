#![warn(missing_docs)]
//! The entry point for xvc cli
use clap::Parser;
use xvc::{error::Result, watch};

/// The entry point of the `xvc` cli.
///
/// It parses the command line arguments [xvc::cli::XvcCLI] and calls [xvc::cli::dispatch]
///
fn main() -> Result<()> {
    let cli_opts = xvc::cli::XvcCLI::parse();
    watch!(cli_opts);
    xvc::cli::dispatch(cli_opts)
}
