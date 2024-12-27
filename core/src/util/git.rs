//! Git operations for Xvc repositories
use std::{ffi::OsString, path::PathBuf, str::FromStr};

use crate::XvcRoot;
use subprocess::Exec;
use xvc_logging::{debug, watch, XvcOutputSender};

use crate::{Error, Result};
use std::path::Path;

use xvc_walker::{build_ignore_patterns, AbsolutePath, IgnoreRules};

use crate::GIT_DIR;

use super::xvcignore::COMMON_IGNORE_PATTERNS;
/// Check whether a path is inside a Git repository.
/// It returns `None` if not, otherwise returns the closest directory with `.git`.
/// It works by checking `.git` directories in parents, until no more parent left.
pub fn inside_git(path: &Path) -> Option<PathBuf> {
    let mut pb = PathBuf::from(path)
        .canonicalize()
        .expect("Cannot canonicalize the path. Possible symlink loop.");
    loop {
        if pb.join(GIT_DIR).is_dir() {
            return Some(pb);
        } else if pb.parent().is_none() {
            return None;
        } else {
            pb.pop();
        }
    }
}

/// Returns [xvc_walker::IgnoreRules] for `.gitignore`
/// It's used to check whether a path is already ignored by Git.
pub fn build_gitignore(git_root: &AbsolutePath) -> Result<IgnoreRules> {
    let rules = build_ignore_patterns(
        COMMON_IGNORE_PATTERNS,
        git_root,
        ".gitignore".to_owned().as_ref(),
    )?;

    Ok(rules)
}

/// Find the absolute path to the git executable to run
/// TODO: This must be cached. It makes a which request every time a command runs
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
        &["add", xvc_dir_str, "*.gitignore", "*.xvcignore"],
    )?;
    debug!(output_snd, "Staging .xvc/ to git: {res_git_add}");
    Ok(())
}

/// Run `git check-ignore` to check if a path is ignored by Git
pub fn git_ignored(git_command: &str, xvc_root_str: &str, path: &str) -> Result<bool> {
    let command_res = exec_git(git_command, xvc_root_str, &["check-ignore", path])?;

    if command_res.trim().is_empty() {
        Ok(false)
    } else {
        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use test_case::test_case;
    use xvc_logging::watch;
    use xvc_test_helper::*;
    use xvc_walker::MatchResult as M;

    #[test_case("myfile.txt" , ".gitignore", "/myfile.txt" => matches M::Ignore ; "myfile.txt")]
    #[test_case("mydir/myfile.txt" , "mydir/.gitignore", "myfile.txt" => matches M::Ignore ; "mydir/myfile.txt")]
    #[test_case("mydir/myfile.txt" , ".gitignore", "/mydir/myfile.txt" => matches M::Ignore ; "from root dir")]
    #[test_case("mydir/myfile.txt" , ".gitignore", ""  => matches M::NoMatch ; "non ignore")]
    #[test_case("mydir/myfile.txt" , ".gitignore", "mydir/**" => matches M::Ignore ; "ignore dir star 2")]
    #[test_case("mydir/myfile.txt" , ".gitignore", "mydir/*" => matches M::Ignore ; "ignore dir star")]
    #[test_case("mydir/yourdir/myfile.txt" , "mydir/.gitignore", "yourdir/*" => matches M::Ignore ; "ignore deep dir star")]
    #[test_case("mydir/yourdir/myfile.txt" , "mydir/.gitignore", "yourdir/**" => matches M::Ignore ; "ignore deep dir star 2")]
    #[test_case("mydir/myfile.txt" , "another-dir/.gitignore", "another-dir/myfile.txt" => matches M::NoMatch ; "non ignore from dir")]
    fn test_gitignore(path: &str, gitignore_path: &str, ignore_line: &str) -> M {
        test_logging(log::LevelFilter::Trace);
        let git_root = temp_git_dir();
        let path = git_root.join(PathBuf::from(path));
        let gitignore_path = git_root.join(PathBuf::from(gitignore_path));
        if let Some(ignore_dir) = gitignore_path.parent() {
            fs::create_dir_all(ignore_dir).unwrap();
        }
        fs::write(&gitignore_path, format!("{}\n", ignore_line)).unwrap();

        let gitignore = build_ignore_patterns("", &git_root, ".gitignore").unwrap();

        gitignore.check(&path)
    }
}
