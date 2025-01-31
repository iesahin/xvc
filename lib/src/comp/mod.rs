//! Completion helpers for shells

use std::io;

use crate::{cli::XvcCLI, Result};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use clap_complete_nushell::Nushell;

/// Completion helper commands
#[derive(Debug, Clone, Parser)]
#[command(author, version)]
pub struct CompCLI {
    /// Subcommand to run
    #[command(subcommand)]
    pub subcommand: CompSubCommand,
}

/// Completion helper subcommands
#[derive(Debug, Clone, Parser)]
#[command()]
pub enum CompSubCommand {
    // TODO: We can parameterize this and use for other shells as well.
    #[command()]
    GenerateNushell,
}

pub fn run(opts: CompCLI) -> Result<()> {
    match opts.subcommand {
        CompSubCommand::GenerateNushell => generate_nushell(),
    }

    Ok(())
}

fn generate_nushell() {
    let mut cmd = XvcCLI::command();
    generate(Nushell, &mut cmd, "xvc", &mut io::stdout());
}
