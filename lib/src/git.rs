//! Git operations for Xvc repository commands
use std::{ffi::OsString, path::PathBuf, str::FromStr};

use subprocess::Exec;
use xvc_core::XvcRoot;
use xvc_logging::{debug, watch, XvcOutputSender};

use crate::{Error, Result};

/// Find the absolute path to the git executable to run
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
    watch!(args);
    let proc_res = Exec::cmd(git_command).args(&args).capture()?;

    match proc_res.exit_status {
        subprocess::ExitStatus::Exited(0) => Ok(proc_res.stdout_str()),
        subprocess::ExitStatus::Exited(_) => Err(Error::GitProcessError {
            stdout: proc_res.stdout_str(),
            stderr: proc_res.stderr_str(),
        }),
        subprocess::ExitStatus::Signaled(_)
        | subprocess::ExitStatus::Other(_)
        | subprocess::ExitStatus::Undetermined => Err(Error::GitProcessError {
            stdout: proc_res.stdout_str(),
            stderr: proc_res.stderr_str(),
        }),
    }
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

    watch!(git_diff_staged_out);

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
    from_ref: String,
) -> Result<()> {
    let xvc_directory = xvc_root.as_path().to_str().unwrap();
    let git_command_option = xvc_root.config().get_str("git.command")?.option;
    let git_command = get_absolute_git_command(&git_command_option)?;

    let git_diff_staged_out = stash_user_staged_files(output_snd, &git_command, xvc_directory)?;
    exec_git(&git_command, xvc_directory, &["checkout", &from_ref])?;

    if !git_diff_staged_out.trim().is_empty() {
        debug!("Unstashing user staged files: {git_diff_staged_out}");
        unstash_user_staged_files(output_snd, &git_command, xvc_directory)?;
    }
    Ok(())
}

/// This receives `xvc_root` ownership because as a final operation, it must drop the root to
/// record the last entity counter before commit.
pub fn handle_git_automation(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    to_branch: Option<&str>,
    xvc_cmd: &str,
) -> Result<()> {
    let xvc_root_dir = xvc_root.as_path().to_path_buf();
    let xvc_root_str = xvc_root_dir.to_str().unwrap();
    let use_git = xvc_root.config().get_bool("git.use_git")?.option;
    let auto_commit = xvc_root.config().get_bool("git.auto_commit")?.option;
    let auto_stage = xvc_root.config().get_bool("git.auto_stage")?.option;
    let git_command_str = xvc_root.config().get_str("git.command")?.option;
    let git_command = get_absolute_git_command(&git_command_str)?;
    let xvc_dir = xvc_root.xvc_dir().clone();
    let xvc_dir_str = xvc_dir.to_str().unwrap();

    if use_git {
        if auto_commit {
            git_auto_commit(
                output_snd,
                &git_command,
                xvc_root_str,
                xvc_dir_str,
                xvc_cmd,
                to_branch,
            )?;
        } else if auto_stage {
            git_auto_stage(output_snd, &git_command, xvc_root_str, xvc_dir_str)?;
        }
    }

    Ok(())
}

/// Commit `.xvc` directory after Xvc operations
pub fn git_auto_commit(
    output_snd: &XvcOutputSender,
    git_command: &str,
    xvc_root_str: &str,
    xvc_dir_str: &str,
    xvc_cmd: &str,
    to_branch: Option<&str>,
) -> Result<()> {
    debug!(output_snd, "Using Git: {git_command}");

    let git_diff_staged_out = stash_user_staged_files(output_snd, git_command, xvc_root_str)?;

    if let Some(branch) = to_branch {
        debug!(output_snd, "Checking out branch {branch}");
        exec_git(git_command, xvc_root_str, &["checkout", "-b", branch])?;
    }

    // Add and commit `.xvc`
    match exec_git(
        git_command,
        xvc_root_str,
        // We check the output of the git add command to see if there were any files added.
        // "--verbose" is required to get the output we need.
        &[
            "add",
            "--verbose",
            xvc_dir_str,
            "*.gitignore",
            "*.xvcignore",
        ],
    ) {
        Ok(git_add_output) => {
            watch!(git_add_output);
            if git_add_output.trim().is_empty() {
                debug!(output_snd, "No files to commit");
                return Ok(());
            } else {
                match exec_git(
                    git_command,
                    xvc_root_str,
                    &[
                        "commit",
                        "-m",
                        &format!("Xvc auto-commit after '{xvc_cmd}'"),
                    ],
                ) {
                    Ok(res_git_commit) => {
                        debug!(output_snd, "Committing .xvc/ to git: {res_git_commit}");
                    }
                    Err(e) => {
                        debug!(output_snd, "Error committing .xvc/ to git: {e}");
                        return Err(e);
                    }
                }
            }
        }
        Err(e) => {
            debug!(output_snd, "Error adding .xvc/ to git: {e}");
            return Err(e);
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
    Ok(())
}

/// runs `git add .xvc *.gitignore *.xvcignore` to stage the files after Xvc operations
pub fn git_auto_stage(
    output_snd: &XvcOutputSender,
    git_command: &str,
    xvc_root_str: &str,
    xvc_dir_str: &str,
) -> Result<()> {
    let res_git_add = exec_git(
        git_command,
        xvc_root_str,
        &["add", &xvc_dir_str, "*.gitignore", "*.xvcignore"],
    )?;
    debug!(output_snd, "Staging .xvc/ to git: {res_git_add}");
    Ok(())
}
