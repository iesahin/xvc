//! Completion helpers for shells

use std::io;

use crate::{Result, cli::XvcCLI};
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
    write_nushell_completions(&mut io::stdout());
}

/// Writes the static Nushell completion script for the whole CLI to `writer`.
fn write_nushell_completions(writer: &mut impl io::Write) {
    let mut cmd = XvcCLI::command();
    generate(Nushell, &mut cmd, "xvc", writer);
}

#[cfg(test)]
mod test {
    use super::*;
    use clap::Command;

    fn nushell_script() -> String {
        let mut script = Vec::new();
        write_nushell_completions(&mut script);
        String::from_utf8(script).expect("Nushell completions must be UTF-8")
    }

    /// Nushell has no dynamic completion protocol, so the whole command tree is written out
    /// statically (see `book/src/ref/xvc-completions.md`). This walks the same tree clap
    /// generates from and checks each command reached the script.
    #[test]
    fn nushell_script_declares_every_command() {
        let script = nushell_script();
        assert!(script.starts_with("module completions {"));
        assert!(script.contains("export extern xvc ["));

        fn assert_declared(script: &str, path: &str, cmd: &Command) {
            for sub in cmd.get_subcommands() {
                let sub_path = format!("{path} {}", sub.get_name());
                assert!(
                    script.contains(&format!("export extern \"{sub_path}\" [")),
                    "`{sub_path}` is missing from the Nushell completions"
                );
                assert_declared(script, &sub_path, sub);
            }
        }

        assert_declared(&script, "xvc", &XvcCLI::command());
    }

    /// The options of a command have to be written next to it, otherwise the script completes
    /// command names only.
    #[test]
    fn nushell_script_declares_options_with_their_commands() {
        let script = nushell_script();

        assert!(script.contains("--recheck-method"));
        assert!(script.contains("--pipeline-name"));
        assert!(script.contains("--storage"));
        // Short flags are attached to their long form, as `--long(-s)`.
        assert!(script.contains("--verbose(-v)"));
    }

    /// The script is `use`d as a Nushell module, so it has to close the module it opens.
    #[test]
    fn nushell_script_exports_its_module() {
        let script = nushell_script();
        assert!(script.trim_end().ends_with("use completions *"));
    }
}
