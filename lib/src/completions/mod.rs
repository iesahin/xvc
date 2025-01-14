//! Shell agnostic dynamic completion functionality
use crate::{Error, Result};

use clap::Command;
use clap_complete::CompletionCandidate;

use std::env;
use std::ffi::OsString;
use std::iter;
use std::path::Path;

/// Handles shell-agnostic completions when COMPLETE environment variable is present
///
/// This function processes the command-line arguments to ensure that aliases
/// are expanded before passing them to `clap_complete`. It then attempts to
/// run the completion using the `clap_complete` crate.
///
/// # Arguments
///
/// * `app` - The command for which to handle shell completion.
/// * `cwd` - The current working directory.
///
/// # Panics
///
/// This function will panic if it is called without the `COMPLETE` environment
/// variable set.
pub fn handle_shell_completion(app: &Command, cwd: &Path) -> Result<()> {
    let mut args = vec![];
    // Take the first two arguments as is, they must be passed to clap_complete
    // without any changes. They are usually "xvc --".
    args.extend(env::args_os().take(2));

    // Make sure aliases are expanded before passing them to clap_complete. We
    // skip the first two args ("jj" and "--") for alias resolution, then we
    // stitch the args back together, like clap_complete expects them.
    let orig_args = env::args_os().skip(2);
    if orig_args.len() > 0 {
        let arg_index: Option<usize> = env::var("_CLAP_COMPLETE_INDEX")
            .ok()
            .and_then(|s| s.parse().ok());
        let resolved_aliases = if let Some(index) = arg_index {
            // As of clap_complete 4.5.38, zsh completion script doesn't pad an
            // empty arg at the complete position. If the args doesn't include a
            // command name, the default command would be expanded at that
            // position. Therefore, no other command names would be suggested.
            let pad_len = usize::saturating_sub(index + 1, orig_args.len());
            let padded_args = orig_args.chain(iter::repeat(OsString::new()).take(pad_len));
            convert_args_to_string(app, padded_args)?
        } else {
            convert_args_to_string(app, orig_args)?
        };
        args.extend(resolved_aliases.into_iter().map(OsString::from));
    }

    let ran_completion = clap_complete::CompleteEnv::with_factory(|| {
        app.clone()
            // for completing aliases
            .allow_external_subcommands(true)
    })
    .try_complete(args.iter(), Some(cwd))?;

    assert!(
        ran_completion,
        "This function should not be called without the COMPLETE variable set."
    );

    Ok(())
}

fn convert_args_to_string(
    app: &Command,
    args_os: impl IntoIterator<Item = OsString>,
) -> Result<Vec<String>> {
    let mut string_args: Vec<String> = vec![];
    for arg_os in args_os {
        if let Some(string_arg) = arg_os.to_str() {
            string_args.push(string_arg.to_owned());
        } else {
            return Err(Error::NonUtf8Argument(arg_os));
        }
    }

    resolve_default_command(app, string_args)
}

fn resolve_default_command(app: &Command, mut string_args: Vec<String>) -> Result<Vec<String>> {
    const PRIORITY_FLAGS: &[&str] = &["--help", "-h", "--version", "-V"];

    let has_priority_flag = string_args
        .iter()
        .any(|arg| PRIORITY_FLAGS.contains(&arg.as_str()));
    if has_priority_flag {
        return Ok(string_args);
    }

    let app_clone = app
        .clone()
        .allow_external_subcommands(true)
        .ignore_errors(true);
    let matches = app_clone.try_get_matches_from(&string_args).ok();

    if let Some(matches) = matches {
        if matches.subcommand_name().is_none() {
            string_args.push("-h".into());
            return Ok(string_args);
        }
    }
    Ok(string_args)
}

/// Return completions for all Git references starting with `current` in the current directory
/// Used in `--from-ref` option.
pub fn git_reference_completer(current: &std::ffi::OsStr) -> Vec<CompletionCandidate> {
    let current = current.to_string_lossy();
    xvc_core::git::gix_list_references(Path::new("."))
        .map(|refs| {
            refs.iter()
                .filter_map(|r| {
                    if r.starts_with(current.as_ref()) {
                        Some(CompletionCandidate::new(r))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Return completions for all Git branches starting with `current` in the current directory
/// Used in `--to-branch` option
pub fn git_branch_completer(current: &std::ffi::OsStr) -> Vec<CompletionCandidate> {
    let current = current.to_string_lossy();
    xvc_core::git::gix_list_branches(Path::new("."))
        .map(|refs| {
            refs.iter()
                .filter_map(|r| {
                    if r.starts_with(current.as_ref()) {
                        Some(CompletionCandidate::new(r))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}
