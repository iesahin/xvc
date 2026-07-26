//! Git operations that run in process, via [gix].
//!
//! This module is a placeholder. Xvc's Git operations currently all run the `git` binary (see
//! [`super::subprocess`]); they are being moved here one at a time, lowest risk first:
//!
//! 1. `tracked_files` — iterate the index instead of parsing `git ls-files --full-name`.
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
