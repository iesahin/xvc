//! Git operations that run in process, via [gix].
//!
//! Xvc's Git operations are being moved here from [`super::subprocess`] one at a time, lowest
//! risk first:
//!
//! 1. [tracked_files] — iterate the index instead of parsing `git ls-files --full-name`. **Done.**
//! 2. `xvc_paths_dirty` — [`gix::Repository::status`] instead of `git status --porcelain`.
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
    let repo = gix::discover(xvc_directory).map_err(|e| Error::GixError {
        cause: e.to_string(),
    })?;

    let workdir = repo.workdir().ok_or_else(|| Error::GixError {
        cause: format!(
            "{} is inside a bare Git repository, which tracks no worktree files",
            xvc_directory.display()
        ),
    })?;

    // Both sides are canonicalized before comparison: `gix` reports the worktree as configured,
    // which may traverse symlinks differently than the path Xvc was given.
    let prefix = subdirectory_prefix(&workdir.canonicalize()?, &xvc_directory.canonicalize()?)?;

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
