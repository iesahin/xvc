#![warn(missing_docs)]
#![forbid(unsafe_code)]
//! The main dispatching functions for the entire Xvc CLI
pub mod cli;
pub mod error;
pub mod git;
pub mod init;

pub mod api;

pub use api::*;

/// Adds `xvc` as the first elements to `args` and calls [cli::dispatch] after parsing them.
pub fn dispatch(args: Vec<&str>) -> Result<XvcRootOpt> {
    let args_with_binary_name = if !args.is_empty() && args[0] != "xvc" {
        vec!["xvc"].into_iter().chain(args).collect()
    } else {
        args
    };

    let cli_opts = cli::XvcCLI::from_str_slice(&args_with_binary_name)?;

    cli::dispatch(cli_opts)
}
