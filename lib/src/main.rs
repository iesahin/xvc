#![warn(missing_docs)]
//! The entry point for xvc cli
use std::io;

use clap::{Command, CommandFactory};
use clap_complete::{generate, generator, Shell};
use rand::{self, Rng};

use xvc::{cli::XvcCLI, error::Result};

/// The entry point of the `xvc` cli.
///
/// It parses the command line arguments [xvc::cli::XvcCLI] and calls [xvc::cli::dispatch]
fn main() -> Result<()> {
    // println!("{:#?}", std::env::args().nth(1).as_deref());
    // FIXME: Clean up
    // if let Some("check-ignore") = std::env::args().nth(1).as_deref() {
    //     println!("now");
    //     dynamic_completion(&mut XvcCLI::command());
    //     return Ok(());
    // }

    clap_complete::CompleteEnv::with_factory(XvcCLI::command).complete();

    let cli_opts = xvc::cli::XvcCLI::from_args_os(std::env::args_os())?;
    xvc::cli::dispatch(cli_opts)?;

    Ok(())
}

fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn dynamic_completion(cmd: &mut Command) {
    let generator = Shell::from_env().unwrap_or(Shell::Bash);
    // Get current word being completed
    let comp_line = std::env::var("COMP_LINE").unwrap_or_default();
    let comp_point = std::env::var("COMP_POINT")
        .unwrap_or_default()
        .parse::<usize>()
        .unwrap_or(0);

    println!("{}", comp_line);
    println!("{}", comp_point);
    // Generate dynamic suggestions here
    // let suggestions = get_dynamic_suggestions(&comp_line, comp_point);
    // // Add dynamic suggestions to command cmd. arg
    // cmd.clone()
    //     .arg(clap::Arg::new("xxx").value_parser(suggestions));
    // // Generate completion script
    // generate(generator, cmd, "myapp", &mut io::stdout());
}

fn get_dynamic_suggestions(_comp_line: &str, _comp_point: usize) -> Vec<String> {
    vec![generate_random_string(10)]
}
