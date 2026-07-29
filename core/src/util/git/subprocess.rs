//! Git operations that run the `git` binary.
//!
//! Every `git` process Xvc spawns goes through [exec_git]. The binary to run comes from the
//! `git.command` configuration option, resolved to an absolute path by
//! [get_absolute_git_command].

use std::{ffi::OsString, path::PathBuf, str::FromStr};

use cached::UnboundCache;
use cached::cached;
use subprocess::Exec;
use xvc_logging::{XvcOutputSender, debug};

use super::paths::XvcGitPaths;
use crate::XvcRoot;
use crate::{Error, Result};

/// Find the absolute path to the git executable to run
///
/// The result is cached per `git_command`, so a `which` lookup happens once per process rather
/// than once per Git operation.
#[cached(
    ty = "UnboundCache<String, String>",
    create = "{ UnboundCache::builder().build().unwrap() }",
    convert = r#"{ git_command.to_string() }"#
)]
pub fn get_absolute_git_command(git_command: &str) -> Result<String> {
    let git_cmd_path = PathBuf::from(git_command);
    let git_cmd = if git_cmd_path.is_absolute() {
        git_command.to_string()
    } else {
        let cmd_path = which::which(git_command)?;
        cmd_path.to_string_lossy().to_string()
    };
    Ok(git_cmd)
}

/// Run a git command with a specific git binary
pub fn exec_git(git_command: &str, xvc_directory: &str, args_str_vec: &[&str]) -> Result<String> {
    let mut args = vec!["-C", xvc_directory];
    args.extend(args_str_vec);
    let args: Vec<OsString> = args
        .iter()
        .map(|s| OsString::from_str(s).unwrap())
        .collect();
    let proc_res = Exec::cmd(git_command).args(&args).capture()?;

    if proc_res.exit_status.success() {
        Ok(proc_res.stdout_str())
    } else {
        Err(Error::GitProcessError {
            stdout: proc_res.stdout_str(),
            stderr: proc_res.stderr_str(),
        })
    }
}

/// Get files tracked by git
///
/// NOTE: Assumptions for this function:
/// - No submodules
pub fn get_git_tracked_files(git_command: &str, xvc_directory: &str) -> Result<Vec<String>> {
    let git_ls_files_out = exec_git(
        git_command,
        xvc_directory,
        // XXX: When core.quotepath is in its default value, all UTF-8 paths are converted to octal
        // strings and we lose the ability to match them. We supply a one off config value to set
        // it to off.
        &["-c", "core.quotepath=off", "ls-files", "--full-name"],
    )?;
    let git_ls_files_out = git_ls_files_out
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    Ok(git_ls_files_out)
}

/// Stash user's staged files to avoid committing them before auto-commit
pub fn stash_user_staged_files(
    output_snd: &XvcOutputSender,
    git_command: &str,
    xvc_directory: &str,
) -> Result<String> {
    // Do we have user staged files?
    let git_diff_staged_out = exec_git(
        git_command,
        xvc_directory,
        &["diff", "--name-only", "--cached"],
    )?;

    // If so stash them
    if !git_diff_staged_out.trim().is_empty() {
        debug!(
            output_snd,
            "Stashing user staged files: {git_diff_staged_out}"
        );
        let stash_out = exec_git(git_command, xvc_directory, &["stash", "push", "--staged"])?;
        debug!(output_snd, "Stashed user staged files: {stash_out}");
    }

    Ok(git_diff_staged_out)
}

/// Unstash user's staged files after auto-commit
pub fn unstash_user_staged_files(
    output_snd: &XvcOutputSender,
    git_command: &str,
    xvc_directory: &str,
) -> Result<()> {
    let res_git_stash_pop = exec_git(git_command, xvc_directory, &["stash", "pop", "--index"])?;
    debug!(
        output_snd,
        "Unstashed user staged files: {res_git_stash_pop}"
    );
    Ok(())
}

/// Checkout a git branch or tag before running an Xvc command
pub fn git_checkout_ref(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    from_ref: &str,
) -> Result<()> {
    let xvc_directory = xvc_root.as_path().to_str().unwrap();
    let git_command_option = xvc_root.config().git.command.clone();
    let git_command = get_absolute_git_command(&git_command_option)?;

    let git_diff_staged_out = stash_user_staged_files(output_snd, &git_command, xvc_directory)?;
    exec_git(&git_command, xvc_directory, &["checkout", from_ref])?;

    if !git_diff_staged_out.trim().is_empty() {
        debug!("Unstashing user staged files: {git_diff_staged_out}");
        unstash_user_staged_files(output_snd, &git_command, xvc_directory)?;
    }
    Ok(())
}

/// Commit `.xvc` directory after Xvc operations
///
/// Returns the new commit's id, or `None` when there was nothing to commit.
///
/// # The stash
///
/// `git commit` commits the whole index; there is no way to ask it for a subset. So anything the
/// user has staged has to be moved out of the way first and put back afterwards, which is what
/// [stash_user_staged_files] and [unstash_user_staged_files] are for.
///
/// This is the riskiest thing Xvc does, and it runs on every command: a crash between the push and
/// the pop leaves the user's staged work in a stash they never created. The in-process backend
/// does not need it — see [`super::gix_backend::commit_xvc_paths`] — so this path exists for the
/// repositories that cannot use it.
pub fn git_auto_commit(
    output_snd: &XvcOutputSender,
    git_command: &str,
    xvc_root_str: &str,
    xvc_dir_str: &str,
    message: &str,
) -> Result<Option<String>> {
    debug!(output_snd, "Using Git: {git_command}");

    let git_diff_staged_out = stash_user_staged_files(output_snd, git_command, xvc_root_str)?;

    // Add and commit `.xvc`
    let mut commit_result = Ok(None);
    // We check the output of the git add command to see if there were any files added.
    // "--verbose" is required to get the output we need.
    let mut add_args = vec!["add", "--verbose"];
    add_args.extend(XvcGitPaths::subprocess_pathspecs(xvc_dir_str));
    match exec_git(git_command, xvc_root_str, &add_args) {
        Ok(git_add_output) => {
            if git_add_output.trim().is_empty() {
                debug!(output_snd, "No files to commit");
            } else {
                match exec_git(git_command, xvc_root_str, &["commit", "-m", message]) {
                    Ok(res_git_commit) => {
                        debug!(output_snd, "Committing .xvc/ to git: {res_git_commit}");
                        commit_result = exec_git(git_command, xvc_root_str, &["rev-parse", "HEAD"])
                            .map(|id| Some(id.trim().to_string()));
                    }
                    Err(e) => {
                        debug!(output_snd, "Error committing .xvc/ to git: {e}");
                        commit_result = Err(e);
                    }
                }
            }
        }
        Err(e) => {
            debug!(output_snd, "Error adding .xvc/ to git: {e}");
            commit_result = Err(e);
        }
    }

    // Pop the stash if there were files we stashed

    if !git_diff_staged_out.trim().is_empty() {
        debug!(
            output_snd,
            "Unstashing user staged files: {git_diff_staged_out}"
        );
        unstash_user_staged_files(output_snd, git_command, xvc_root_str)?;
    }

    commit_result
}

/// runs `git add .xvc *.gitignore *.xvcignore` to stage the files after Xvc operations
///
/// Returns the paths staged, read back from `git add --verbose`. Git prints one `add '<path>'` line
/// per file, and quotes the path the same way `ls-files` does — so a path with a control character
/// in it comes back C-quoted. The in-process backend reads the paths from the index instead and has
/// no such problem; here the list is only ever logged, so the difference does not propagate.
pub fn git_auto_stage(
    output_snd: &XvcOutputSender,
    git_command: &str,
    xvc_root_str: &str,
    xvc_dir_str: &str,
) -> Result<Vec<String>> {
    let mut add_args = vec!["add", "--verbose"];
    add_args.extend(XvcGitPaths::subprocess_pathspecs(xvc_dir_str));
    let res_git_add = exec_git(git_command, xvc_root_str, &add_args)?;
    debug!(output_snd, "Staging .xvc/ to git: {res_git_add}");

    let mut staged: Vec<String> = res_git_add
        .lines()
        .filter_map(|line| {
            line.strip_prefix("add '")
                .and_then(|rest| rest.strip_suffix('\''))
                .map(ToString::to_string)
        })
        .collect();
    staged.sort();
    Ok(staged)
}
