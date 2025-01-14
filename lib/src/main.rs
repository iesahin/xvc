#![warn(missing_docs)]
//! The entry point for xvc cli
use std::env;

use clap::CommandFactory;

use xvc::{cli::XvcCLI, error::Result};

/// The entry point of the `xvc` cli.
///
/// It parses the command line arguments [xvc::cli::XvcCLI] and calls [xvc::cli::dispatch]
fn main() -> Result<()> {
    let cmd = XvcCLI::command();
    // FIXME: Handle -C option for completions here. Working directory should be considered when using outside of
    // the repository.
    let current_dir = env::current_dir().unwrap();

    if env::var_os("COMPLETE").is_some() {
        return xvc::cli::handle_shell_completion(&cmd, &current_dir);
    }

    let cli_opts = xvc::cli::XvcCLI::from_args_os(std::env::args_os())?;
    xvc::cli::dispatch(cli_opts)?;

    Ok(())
}
