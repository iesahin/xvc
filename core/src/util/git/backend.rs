//! Choosing between the in-process backend and the `git` binary.
//!
//! Up to this point each Git operation moved to [`super::gix_backend`] wholesale, because each was
//! a drop-in replacement that needed no abstraction. Committing is the first one that cannot be:
//! `gix` runs no hooks and signs no commits, so for some repositories the `git` binary is not a
//! legacy path but the only correct one. Both implementations have to coexist in one binary, and
//! something has to pick.
//!
//! [GitBackend] is that seam. It is written in terms of what Xvc does — commit the paths it owns —
//! rather than mirroring the `git` command line, so the two implementations are free to reach the
//! same end by different means. They do: [SubprocessBackend] stages into the index and commits it,
//! stashing the user's staged files out of the way first, while [GixBackend] builds the tree from
//! `HEAD^{tree}` and never touches the staging area at all.

use std::path::{Path, PathBuf};

use xvc_logging::{XvcOutputSender, debug, info};

use super::{capabilities, gix_backend, subprocess};
use crate::{Error, Result, XvcRoot};

/// The value of the `git.backend` configuration option.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitBackendKind {
    /// Run in process where possible, and use the `git` binary where it is not.
    Auto,
    /// Always run in process, and fail rather than fall back.
    Gix,
    /// Always run the `git` binary.
    Subprocess,
}

impl GitBackendKind {
    /// Parse the `git.backend` configuration value.
    ///
    /// An unrecognized value is an error rather than a silent fall back to the default: a typo in
    /// `git.backend` should say so, not quietly pick a backend the user did not ask for.
    pub fn from_config_value(value: &str) -> Result<Self> {
        match value {
            "auto" => Ok(Self::Auto),
            "gix" => Ok(Self::Gix),
            "subprocess" => Ok(Self::Subprocess),
            other => Err(Error::GitBackendUnsupported {
                operation: "selecting a Git backend".into(),
                reason: format!(
                    "git.backend is set to {other:?}, which is not one of \"auto\", \"gix\" or \"subprocess\""
                ),
            }),
        }
    }
}

/// The Git operations Xvc performs, in terms of what Xvc means by them.
pub trait GitBackend {
    /// The files Git tracks under the Xvc root, relative to it.
    fn tracked_files(&self) -> Result<Vec<String>>;

    /// Whether anything under the Xvc-owned path set differs from `HEAD`.
    fn xvc_paths_dirty(&self) -> Result<bool>;

    /// Stage the Xvc-owned paths, returning the paths staged.
    fn stage_xvc_paths(&self, output_snd: &XvcOutputSender) -> Result<Vec<String>>;

    /// Stage *and* commit the Xvc-owned paths, returning the new commit id.
    ///
    /// `Ok(None)` means there was nothing to commit.
    ///
    /// This is deliberately one operation rather than a `stage` followed by a `commit`. In
    /// [GixBackend] the tree is built from `HEAD^{tree}`, so there is no intermediate staged state
    /// to expose; splitting it would force that backend to invent one, and inventing one is
    /// exactly the coupling to the index that makes the subprocess backend need a stash.
    fn commit_xvc_paths(
        &self,
        output_snd: &XvcOutputSender,
        message: &str,
    ) -> Result<Option<String>>;

    /// Create `branch` and switch to it, as `git checkout -b` does.
    fn create_and_switch_branch(&self, output_snd: &XvcOutputSender, branch: &str) -> Result<()>;
}

/// Runs the `git` binary for everything.
pub struct SubprocessBackend {
    git_command: String,
    xvc_root_str: String,
    xvc_dir_str: String,
}

impl SubprocessBackend {
    /// Resolve the `git` binary named by `git.command` and bind it to an Xvc root.
    pub fn new(xvc_root: &XvcRoot) -> Result<Self> {
        let git_command = subprocess::get_absolute_git_command(&xvc_root.config().git.command)?;
        Ok(Self {
            git_command,
            xvc_root_str: path_to_string(xvc_root.as_path())?,
            xvc_dir_str: path_to_string(xvc_root.xvc_dir())?,
        })
    }
}

impl GitBackend for SubprocessBackend {
    fn tracked_files(&self) -> Result<Vec<String>> {
        subprocess::get_git_tracked_files(&self.git_command, &self.xvc_root_str)
    }

    fn xvc_paths_dirty(&self) -> Result<bool> {
        let mut args = vec!["status", "--porcelain"];
        args.extend(super::paths::XvcGitPaths::subprocess_pathspecs(
            &self.xvc_dir_str,
        ));
        let output = subprocess::exec_git(&self.git_command, &self.xvc_root_str, &args)?;
        Ok(!output.trim().is_empty())
    }

    fn stage_xvc_paths(&self, output_snd: &XvcOutputSender) -> Result<Vec<String>> {
        subprocess::git_auto_stage(
            output_snd,
            &self.git_command,
            &self.xvc_root_str,
            &self.xvc_dir_str,
        )
    }

    fn commit_xvc_paths(
        &self,
        output_snd: &XvcOutputSender,
        message: &str,
    ) -> Result<Option<String>> {
        subprocess::git_auto_commit(
            output_snd,
            &self.git_command,
            &self.xvc_root_str,
            &self.xvc_dir_str,
            message,
        )
    }

    fn create_and_switch_branch(&self, output_snd: &XvcOutputSender, branch: &str) -> Result<()> {
        debug!(output_snd, "Checking out branch {branch}");
        subprocess::exec_git(
            &self.git_command,
            &self.xvc_root_str,
            &["checkout", "-b", branch],
        )?;
        Ok(())
    }
}

/// Runs Git operations in process with [`gix`].
pub struct GixBackend {
    xvc_root_dir: PathBuf,
}

impl GixBackend {
    /// Bind the in-process backend to an Xvc root.
    pub fn new(xvc_root: &XvcRoot) -> Result<Self> {
        Ok(Self {
            xvc_root_dir: xvc_root.as_path().to_path_buf(),
        })
    }
}

impl GitBackend for GixBackend {
    fn tracked_files(&self) -> Result<Vec<String>> {
        gix_backend::tracked_files(&self.xvc_root_dir)
    }

    fn xvc_paths_dirty(&self) -> Result<bool> {
        gix_backend::xvc_paths_dirty(&self.xvc_root_dir)
    }

    fn stage_xvc_paths(&self, output_snd: &XvcOutputSender) -> Result<Vec<String>> {
        let staged = gix_backend::stage_xvc_paths(&self.xvc_root_dir)?;
        debug!(output_snd, "Staged {} paths to git", staged.len());
        Ok(staged)
    }

    fn commit_xvc_paths(
        &self,
        output_snd: &XvcOutputSender,
        message: &str,
    ) -> Result<Option<String>> {
        let commit = gix_backend::commit_xvc_paths(&self.xvc_root_dir, message)?;
        match &commit {
            Some(id) => debug!(output_snd, "Committed .xvc/ to git: {id}"),
            None => debug!(output_snd, "No files to commit"),
        }
        Ok(commit)
    }

    fn create_and_switch_branch(&self, output_snd: &XvcOutputSender, branch: &str) -> Result<()> {
        gix_backend::create_and_switch_branch(&self.xvc_root_dir, branch)?;
        debug!(output_snd, "Created and switched to branch {branch}");
        Ok(())
    }
}

/// Pick the backend to use for `xvc_root`, honoring `git.backend`.
///
/// Under `"auto"` this opens the repository to ask [`capabilities::gix_unsupported`] whether
/// anything about it rules the in-process backend out. Failing to open it at all is also an
/// answer: the `git` binary may still cope with a repository `gix` cannot read, so that falls back
/// rather than failing.
pub fn select_backend(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
) -> Result<Box<dyn GitBackend>> {
    let kind = GitBackendKind::from_config_value(&xvc_root.config().git.backend)?;

    match kind {
        GitBackendKind::Subprocess => Ok(Box::new(SubprocessBackend::new(xvc_root)?)),
        GitBackendKind::Gix => Ok(Box::new(GixBackend::new(xvc_root)?)),
        GitBackendKind::Auto => match gix_fallback_reason(xvc_root.as_path()) {
            Some(reason) => {
                // `info`, not `debug`: this changes which code path runs the user's commit, and
                // "why is Xvc still spawning git?" should be answerable without a debug build.
                info!(output_snd, "Using the Git command because {reason}.");
                Ok(Box::new(SubprocessBackend::new(xvc_root)?))
            }
            None => Ok(Box::new(GixBackend::new(xvc_root)?)),
        },
    }
}

/// Why `"auto"` should use the `git` binary for this repository, or `None` to run in process.
fn gix_fallback_reason(xvc_root_dir: &Path) -> Option<String> {
    let repo = gix::discover(xvc_root_dir).ok()?;
    let index = repo.index_or_empty().ok()?;
    capabilities::gix_unsupported(&repo, &index)
}

/// Xvc roots come from the filesystem, so a non-UTF-8 one is possible even though the subprocess
/// backend has never been able to handle it.
fn path_to_string(path: &Path) -> Result<String> {
    path.to_str()
        .map(ToString::to_string)
        .ok_or_else(|| Error::GitBackendUnsupported {
            operation: "running the Git command".into(),
            reason: format!("the path {} is not valid UTF-8", path.display()),
        })
}
