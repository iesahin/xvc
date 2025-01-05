//! Print shell completions to be sourced in shells.
use crate::{cli::XvcCLI, error::Result};
use clap::{CommandFactory, Parser};

use clap_complete::{generate, Shell};
use xvc_logging::{output, XvcOutputSender};

/// Print shell completions to be sourced in shells.
#[derive(Debug, Parser, Clone)]
#[command(name = "completions")]
pub struct CompletionsCLI {
    /// The shell the completions will be generated for.
    /// One of https://docs.rs/clap_complete/latest/clap_complete/aot/enum.Shell.html
    /// Tries to identify from environment when omitted.
    shell: Option<Shell>,
}

/// Print out aliases for long commands.
/// These can be sourced in `~/.zsh_aliases`, `~/.bash_aliases` etc. like `$(xvc aliases)`.
pub fn run(output_snd: &XvcOutputSender, opts: CompletionsCLI) -> Result<()> {
    let shell = opts.shell.unwrap_or_else(|| {
        Shell::from_env().expect(
            "Could not
determine shell from environment. Please specify.",
        )
    });

    let mut cmd = XvcCLI::command();
    // Create a writable buffer to store the completions
    let mut buffer: Vec<u8> = Vec::new();
    generate(shell, &mut cmd, "xvc", &mut buffer);
    let completions = String::from_utf8(buffer).expect("Completion script should be UTF-8");
    output!(output_snd, "{}", completions);
    Ok(())
}
