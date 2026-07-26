//! Git operations for Xvc repositories
//!
//! Xvc keeps its metadata in `.xvc/` and hides tracked files from Git with `.gitignore` entries.
//! Both need to reach Git, which this module handles.
//!
//! The work is split by *how* it reaches Git:
//!
//! - [subprocess] runs the `git` binary. Everything lives here today.
//! - [gix_backend] does the same work in process with [gix]. Operations move here one at a time.
//! - [paths] holds the path set Xvc owns, shared by both so they cannot drift apart.
//! - [ignore] locates the repository and reads `.gitignore` rules — no Git needed either way.
//! - [refs] lists references and branches for the shell completers, already using [gix].
//!
//! Callers should use the re-exports below rather than reaching into the submodules, so that
//! moving an operation between backends stays invisible to them.

pub mod gix_backend;
pub mod ignore;
pub mod paths;
pub mod refs;
pub mod subprocess;

pub use ignore::{GitRoot, build_gitignore, inside_git};
pub use paths::{GITIGNORE_PATHSPEC, XVCIGNORE_PATHSPEC, XvcGitPaths};
pub use refs::{gix_list_branches, gix_list_references};
pub use subprocess::{
    exec_git, get_absolute_git_command, get_git_tracked_files, git_auto_commit, git_auto_stage,
    git_checkout_ref, handle_git_automation, stash_user_staged_files, unstash_user_staged_files,
};
