//! Get the Xvc root directory for the current project
use crate::error::Result;
use crate::types::xvcroot::XvcRoot;
use clap::Parser;

use relative_path::RelativePath;
use xvc_logging::{output, watch, XvcOutputSender};

#[derive(Debug, Parser)]
#[command(name = "root")]
/// Get the XVC root directory for the current project
pub struct RootCLI {
    #[arg(long)]
    /// Show absolute path instead of relative
    absolute: bool,
}

/// Entry point for xvc root
///
/// # Arguments
///
/// - `output`: Buffer to write the result
/// - `xvc_root`: The root of the current project
/// - `opts`: [CLI options][RootCLI]
///
/// # Errors and Panics
///
/// - It returns an error when `output` can't be written by `writeln!`
///   This probably leads to panic! in caller.
/// - It returns an error when `xvc_root` cannot be converted to absolute path
///   This should never happen if `xvc_root` properly constructed.
pub fn run(output_snd: &XvcOutputSender, xvc_root: &XvcRoot, opts: RootCLI) -> Result<()> {
    if opts.absolute {
        output!("{}", xvc_root.absolute_path().to_string_lossy());
    } else {
        let current_dir = xvc_root.config().current_dir.option.to_path_buf();

        let rel_dir = RelativePath::new(&current_dir.to_string_lossy()).relative(
            RelativePath::new(&xvc_root.absolute_path().to_string_lossy()),
        );
        watch!(rel_dir);
        if rel_dir == "" {
            output!(output_snd, ".");
        } else {
            output!(output_snd, "{}", rel_dir);
        }
    }
    Ok(())
}
