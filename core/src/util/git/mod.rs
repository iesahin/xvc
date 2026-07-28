//! Git operations for Xvc repositories
//!
//! Xvc keeps its metadata in `.xvc/` and hides tracked files from Git with `.gitignore` entries.
//! Both need to reach Git, which this module handles.
//!
//! The work is split by *how* it reaches Git:
//!
//! - [subprocess] runs the `git` binary.
//! - [gix_backend] does the same work in process with [gix]. Operations move there one at a time.
//! - [paths] holds the path set Xvc owns, shared by both so they cannot drift apart.
//! - [ignore] locates the repository and reads `.gitignore` rules — no Git needed either way.
//! - [refs] lists references and branches for the shell completers, already using [gix].
//!
//! [handle_git_automation] is the entry point that decides which of those runs, so it lives here
//! rather than in either backend. Callers should use the re-exports below rather than reaching
//! into the submodules, so that moving an operation between backends stays invisible to them.

pub mod gix_backend;
pub mod ignore;
pub mod paths;
pub mod refs;
pub mod subprocess;

pub use gix_backend::{stage_xvc_paths, tracked_files, xvc_paths_dirty};
pub use ignore::{GitRoot, build_gitignore, inside_git};
pub use paths::{GITIGNORE_PATHSPEC, XVCIGNORE_PATHSPEC, XvcGitPaths};
pub use refs::{gix_list_branches, gix_list_references};
pub use subprocess::{
    exec_git, get_absolute_git_command, get_git_tracked_files, git_auto_commit, git_auto_stage,
    git_checkout_ref, stash_user_staged_files, unstash_user_staged_files,
};

use xvc_logging::{XvcOutputSender, debug};

use crate::{Result, XvcRoot};

/// Commit or stage Xvc's own files after a command, according to the `git.*` configuration.
///
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
    let git_config = xvc_root.config().git.clone();
    let use_git = git_config.use_git;
    let auto_commit = git_config.auto_commit;
    let auto_stage = git_config.auto_stage;
    let git_command_str = git_config.command.clone();
    let git_command = get_absolute_git_command(&git_command_str)?;
    let xvc_dir = xvc_root.xvc_dir().clone();
    let xvc_dir_str = xvc_dir.to_str().unwrap();

    if use_git {
        // Check if there are any changes in the relevant paths before proceeding.
        //
        // This fails open: if the status check itself errors, carry on and let the commit or stage
        // below report the problem. The commit path re-checks anyway, so the cost of a false
        // positive here is one wasted `git add`, whereas a false negative would silently drop a
        // commit Xvc owed.
        match xvc_paths_dirty(&xvc_root_dir) {
            Ok(false) => {
                debug!(
                    output_snd,
                    "No changes detected in Xvc files, skipping Git operations."
                );
                return Ok(());
            }
            Ok(true) => {}
            Err(e) => {
                debug!(output_snd, "Error checking git status: {e}");
            }
        }

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
            let staged = stage_xvc_paths(&xvc_root_dir)?;
            if staged.is_empty() {
                debug!(output_snd, "No files to stage");
            } else {
                debug!(output_snd, "Staged {} paths to git", staged.len());
            }
        }
    }

    Ok(())
}
