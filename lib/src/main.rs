#![warn(missing_docs)]
//! The entry point for xvc cli
use std::env;

use clap::CommandFactory;

use xvc::{cli::XvcCLI, error::Result, Error};

/// The entry point of the `xvc` cli.
///
/// It parses the command line arguments [xvc::cli::XvcCLI] and calls [xvc::cli::dispatch]
fn main() -> Result<()> {
    if env::var_os("COMPLETE").is_some() {
        // FIXME: Handle -C option for completions here. Working directory should be considered when using outside of
        // the repository. It requires to get the context in completions. We currently don't have that.
        // See https://github.com/clap-rs/clap/discussions/5708
        let current_dir = env::current_dir().ok();
        let ran_completion = clap_complete::CompleteEnv::with_factory(XvcCLI::command)
            .try_complete(env::args_os(), current_dir.as_deref())
            .map_err(Error::from)?;

        if ran_completion {
            return Ok(());
        } else {
            eprintln!("Something is broken with completions. Please undefine COMPLETE environment variable (if there is one) and report this.");
            return Err(Error::CompletionError);
        }
    }

    let cli_opts = xvc::cli::XvcCLI::from_args_os(std::env::args_os())?;
    xvc::cli::dispatch(cli_opts)?;

    Ok(())
}
