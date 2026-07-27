//! Git operations that run in process, via [gix].
//!
//! Xvc's Git operations are being moved here from [`super::subprocess`] one at a time, lowest
//! risk first:
//!
//! 1. [tracked_files] — iterate the index instead of parsing `git ls-files --full-name`. **Done.**
//! 2. [xvc_paths_dirty] — [`gix::Repository::status`] instead of `git status --porcelain`. **Done.**
//! 3. `stage_xvc_paths` — write blobs through the filter pipeline and patch the index.
//! 4. `commit_xvc_paths` — build the tree from `HEAD^{tree}` with [`gix::Repository::edit_tree`]
//!    and commit it, so the user's staging area is never touched and no stash is needed.
//! 5. `create_and_switch_branch` — `edit_references` instead of `git checkout -b`.
//!
//! `git checkout <ref>` (the `--from-ref` flag) is deliberately **not** on that list. It is a
//! full `unpack_trees` two-way merge against the live index and worktree, and gitoxide has no
//! checkout orchestration; reimplementing it risks silently destroying uncommitted user work.
//! It stays on [`super::subprocess::git_checkout_ref`], and so does the stash it needs.

use std::path::Path;

use super::paths::XvcGitPaths;
use crate::{Error, Result};

/// List the files Git tracks under `xvc_directory`, as paths relative to `xvc_directory`.
///
/// This replaces `git ls-files --full-name`, which Xvc used to shell out for. Index entries are
/// raw bytes, so reading them directly avoids the quoting `ls-files` applies to its output:
///
/// - Non-ASCII paths. The subprocess version passed `-c core.quotepath=off` to stop Git
///   octal-escaping them (`"\303\274n..."`); there is nothing to escape here, so the workaround
///   is gone.
/// - Paths containing control characters, such as a newline in a filename. Git C-quotes these
///   *regardless* of `core.quotepath`, yielding the literal `"two\nlines.txt"` — quotes, escape
///   and all. That never matches an [`crate::XvcPath`], so those files silently escaped the
///   caller's filter. Index entries are unquoted, so they now match.
/// - Non-UTF-8 paths. With `core.quotepath=off` Git emits the raw bytes, which the subprocess
///   crate lossily replaces with U+FFFD. They are skipped here instead, since the caller compares
///   against `String`s. Either way such a path goes unfiltered; skipping just avoids inventing a
///   path that does not exist.
///
/// # Relative to what
///
/// Index paths are relative to the *repository* root, which is not necessarily `xvc_directory` —
/// `xvc init` can run in a subdirectory of a Git repository. Callers compare these against
/// [`crate::XvcPath`]s, which are relative to the Xvc root, so entries outside `xvc_directory`
/// are dropped and the rest are re-based onto it.
pub fn tracked_files(xvc_directory: &Path) -> Result<Vec<String>> {
    let (repo, prefix) = repo_and_prefix(xvc_directory)?;

    let index = repo.index_or_empty().map_err(|e| Error::GixIndexError {
        cause: e.to_string(),
    })?;

    let files = index
        .entries()
        .iter()
        // Conflicted paths appear once per stage. `git ls-files` prints them all; we keep only
        // stage 0, so a path is reported at most once.
        .filter(|entry| entry.stage() == gix::index::entry::Stage::Unconflicted)
        .filter_map(|entry| {
            let path = std::str::from_utf8(entry.path(&index)).ok()?;
            match prefix.as_deref() {
                None => Some(path.to_string()),
                Some(prefix) => path.strip_prefix(prefix).map(ToString::to_string),
            }
        })
        .collect();

    Ok(files)
}

/// Whether anything under the Xvc-owned path set differs from `HEAD` — staged, unstaged or
/// untracked.
///
/// This replaces the `git status --porcelain <xvc paths>` pre-check that decides whether
/// [`super::handle_git_automation`] has any reason to commit or stage.
///
/// The pathspecs are narrower than the ones the subprocess pre-check used; see
/// [`XvcGitPaths::pathspecs`] for what changed and why.
pub fn xvc_paths_dirty(xvc_directory: &Path) -> Result<bool> {
    let (repo, prefix) = repo_and_prefix(xvc_directory)?;

    let mut changes = repo
        .status(gix::progress::Discard)
        .map_err(|e| Error::GixError {
            cause: e.to_string(),
        })?
        // `Collapsed`, the default, would be cheaper — it reports a wholly new `.xvc/store/` as a
        // single directory, which is enough for a boolean. `Files` is used anyway so that this
        // check and the commit that follows it walk the tree the same way and cannot disagree
        // about whether there is anything to do.
        .untracked_files(gix::status::UntrackedFiles::Files)
        .index_worktree_submodules(None)
        .into_iter(XvcGitPaths::pathspecs(
            prefix.as_deref().unwrap_or_default(),
        ))
        .map_err(|e| Error::GixError {
            cause: e.to_string(),
        })?;

    // Only the first item matters. Dropping the iterator early interrupts and joins the producer
    // threads, so there is no need to drain it.
    match changes.next() {
        None => Ok(false),
        Some(Ok(_)) => Ok(true),
        Some(Err(e)) => Err(Error::GixError {
            cause: e.to_string(),
        }),
    }
}

/// Open the repository containing `xvc_directory`, and work out where that directory sits inside
/// it.
///
/// Both paths are canonicalized before comparison: `gix` reports the worktree as configured, which
/// may traverse symlinks differently than the path Xvc was given.
fn repo_and_prefix(xvc_directory: &Path) -> Result<(gix::Repository, Option<String>)> {
    let repo = gix::discover(xvc_directory).map_err(|e| Error::GixError {
        cause: e.to_string(),
    })?;

    let workdir = repo
        .workdir()
        .ok_or_else(|| Error::GixError {
            cause: format!(
                "{} is inside a bare Git repository, which has no worktree",
                xvc_directory.display()
            ),
        })?
        .canonicalize()?;

    let prefix = subdirectory_prefix(&workdir, &xvc_directory.canonicalize()?)?;
    Ok((repo, prefix))
}

/// The slash-separated path from `root` down to `dir`, with a trailing slash, or `None` when they
/// are the same directory.
///
/// Index paths are slash-separated on every platform, so the components are joined with `/`
/// rather than the host separator.
fn subdirectory_prefix(root: &Path, dir: &Path) -> Result<Option<String>> {
    let relative = dir.strip_prefix(root)?;
    let joined = relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");

    Ok((!joined.is_empty()).then(|| format!("{joined}/")))
}

/// Compile-time guard for the `tree-editor` feature in `core/Cargo.toml`.
///
/// [`gix::Repository::edit_tree`] is gated behind it and is not enabled by any of `gix`'s default
/// features. This function is never called; it exists so that dropping the feature fails the
/// build here, with this explanation, rather than in the middle of step 4 above.
#[cfg(test)]
#[allow(dead_code)]
fn assert_tree_editor_feature_enabled(repo: &gix::Repository, tree: gix::ObjectId) {
    let _ = repo.edit_tree(tree);
}
