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

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::git::subprocess::{get_absolute_git_command, get_git_tracked_files};
    use std::fs;
    use std::process::Command;
    use xvc_test_helper::temp_git_dir;

    /// Commit a set of repo-relative paths, and return the repository root.
    fn repo_with(paths: &[&str]) -> std::path::PathBuf {
        let root = temp_git_dir();
        for path in paths {
            let full = root.join(path);
            fs::create_dir_all(full.parent().unwrap()).unwrap();
            fs::write(&full, format!("content of {path}\n")).unwrap();
        }
        for args in [
            vec!["add", "-A"],
            vec![
                "-c",
                "user.email=t@example.com",
                "-c",
                "user.name=Test",
                "commit",
                "-m",
                "fixture",
            ],
        ] {
            let status = Command::new("git")
                .arg("-C")
                .arg(&root)
                .args(&args)
                .status()
                .unwrap();
            assert!(status.success(), "git {args:?} failed");
        }
        root
    }

    fn sorted(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    /// At the repository root the two backends must agree exactly. This is the property that
    /// makes swapping the implementation safe; the differential comparison is what catches
    /// silent divergence from Git.
    #[test]
    fn matches_the_subprocess_backend_at_the_repository_root() {
        let root = repo_with(&["root.txt", "sub/in_sub.txt", "sub/deep/deeper.txt"]);
        let git = get_absolute_git_command("git").unwrap();

        let subprocess = sorted(get_git_tracked_files(&git, root.to_str().unwrap()).unwrap());
        let gix = sorted(tracked_files(&root).unwrap());

        assert_eq!(gix, subprocess);
        assert_eq!(
            gix,
            vec!["root.txt", "sub/deep/deeper.txt", "sub/in_sub.txt"]
        );
    }

    /// When the Xvc root is a subdirectory of the Git repository, paths come back relative to the
    /// Xvc root. `git ls-files --full-name` reports them relative to the *repository* root
    /// instead, which no longer matches the `XvcPath`s the caller compares them against.
    #[test]
    fn returns_paths_relative_to_the_given_directory() {
        let root = repo_with(&["root.txt", "sub/in_sub.txt", "sub/deep/deeper.txt"]);

        assert_eq!(
            sorted(tracked_files(&root.join("sub")).unwrap()),
            vec!["deep/deeper.txt", "in_sub.txt"]
        );
        assert_eq!(
            tracked_files(&root.join("sub/deep")).unwrap(),
            vec!["deeper.txt"]
        );
    }

    /// A repository with no commits and nothing staged has an empty (or absent) index.
    #[test]
    fn empty_repository_has_no_tracked_files() {
        let root = temp_git_dir();
        assert_eq!(tracked_files(&root).unwrap(), Vec::<String>::new());
    }

    /// Non-ASCII paths, which `git ls-files` octal-escapes unless `core.quotepath=off` is passed.
    /// Both backends get these right; this pins that the index path needs no such workaround.
    #[test]
    fn handles_non_ascii_paths() {
        let root = repo_with(&["ünïcödé.txt", "yeni klasör/veri.txt"]);
        let git = get_absolute_git_command("git").unwrap();

        let expected = vec!["yeni klasör/veri.txt", "ünïcödé.txt"];
        assert_eq!(sorted(tracked_files(&root).unwrap()), expected);
        assert_eq!(
            sorted(get_git_tracked_files(&git, root.to_str().unwrap()).unwrap()),
            expected
        );
    }

    /// Git C-quotes paths containing control characters no matter what `core.quotepath` is set
    /// to, so `ls-files` reports a newline in a filename as the 15-character literal
    /// `"two\nlines.txt"`. That never matches an `XvcPath`, so such a file silently escaped the
    /// caller's filter. This is the one case where the two backends genuinely disagree.
    #[cfg(unix)]
    #[test]
    fn unquotes_paths_that_ls_files_c_quotes() {
        let root = repo_with(&["two\nlines.txt"]);
        let git = get_absolute_git_command("git").unwrap();

        assert_eq!(tracked_files(&root).unwrap(), vec!["two\nlines.txt"]);
        assert_eq!(
            get_git_tracked_files(&git, root.to_str().unwrap()).unwrap(),
            vec![r#""two\nlines.txt""#],
            "if ls-files stops quoting these, this backend is no longer the only correct one"
        );
    }
}
