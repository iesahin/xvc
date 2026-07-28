//! Git operations that run in process, via [gix].
//!
//! Xvc's Git operations are being moved here from [`super::subprocess`] one at a time, lowest
//! risk first:
//!
//! 1. [tracked_files] — iterate the index instead of parsing `git ls-files --full-name`. **Done.**
//! 2. [xvc_paths_dirty] — [`gix::Repository::status`] instead of `git status --porcelain`. **Done.**
//! 3. [stage_xvc_paths] — write blobs through the filter pipeline and patch the index. **Done.**
//! 4. `commit_xvc_paths` — build the tree from `HEAD^{tree}` with [`gix::Repository::edit_tree`]
//!    and commit it, so the user's staging area is never touched and no stash is needed.
//! 5. `create_and_switch_branch` — `edit_references` instead of `git checkout -b`.
//!
//! `git checkout <ref>` (the `--from-ref` flag) is deliberately **not** on that list. It is a
//! full `unpack_trees` two-way merge against the live index and worktree, and gitoxide has no
//! checkout orchestration; reimplementing it risks silently destroying uncommitted user work.
//! It stays on [`super::subprocess::git_checkout_ref`], and so does the stash it needs.

use std::path::Path;

use gix::bstr::BString;

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

/// Stage every changed path under the Xvc-owned path set, and return the paths staged.
///
/// This replaces `git add .xvc *.gitignore *.xvcignore`. It writes the index and nothing else — no
/// ref moves, no commit — and only touches entries Xvc owns, so anything the user has staged is
/// left exactly as it was.
///
/// The return value replaces the old `git add --verbose` output parsing: an empty `Vec` means
/// there was nothing to stage.
///
/// # Why this is the delicate one
///
/// Writing Git's index has several ways to go silently wrong, each guarded below:
///
/// - Blobs come from [`gix::filter::Pipeline::worktree_file_to_object`], not from hashing the file
///   bytes. It applies `.gitattributes` clean filters and `core.autocrlf`, so the object matches
///   what `git add` would have written. Hashing directly produces a different blob under
///   `core.autocrlf=true`, and the file then reads as permanently modified.
/// - The tree-cache extension is dropped before writing. `gix` serializes it as-is rather than
///   invalidating it, so a stale-but-valid cache would make a *later* `git commit` — the user's,
///   not Xvc's — capture outdated subtree content (gitoxide#2421).
/// - Split-index repositories are refused rather than silently flattened, since rewriting the
///   index would drop the `link` extension that holds the shared state.
/// - Stat data comes from [`gix::index::fs::Metadata`], which reads `st_ctime`. `std::fs::Metadata`
///   reports inode *birth* time instead, which Git disagrees with, making every entry look racy
///   and forcing `git status` to re-hash the whole tree on each run.
pub fn stage_xvc_paths(xvc_directory: &Path) -> Result<Vec<String>> {
    let (repo, prefix) = repo_and_prefix(xvc_directory)?;
    let workdir = repo
        .workdir()
        .expect("checked by repo_and_prefix")
        .to_owned();

    // Not `open_index`, which errors when `.git/index` does not exist yet — the state a fresh
    // `git init` is in, and what `xvc init` meets. An absent index is an empty one.
    let mut index = match repo.try_index().map_err(|e| Error::GixIndexError {
        cause: e.to_string(),
    })? {
        // Two derefs: the shared index is an `Arc<FileSnapshot<File>>`.
        Some(index) => (**index).clone(),
        None => gix::index::File::from_state(
            gix::index::State::new(repo.object_hash()),
            repo.index_path(),
        ),
    };

    if index.link().is_some() {
        return Err(Error::GitBackendUnsupported {
            operation: "staging Xvc paths".into(),
            reason: "the repository uses a split index".into(),
        });
    }

    let changed = changed_xvc_paths(&repo, &index, prefix.as_deref().unwrap_or_default())?;
    if changed.is_empty() {
        return Ok(Vec::new());
    }

    let (mut pipeline, _) = repo.filter_pipeline(None).map_err(|e| Error::GixError {
        cause: e.to_string(),
    })?;

    let mut staged = Vec::with_capacity(changed.len());
    let mut removed = Vec::new();
    let mut upserts = Vec::new();

    for rela_path in &changed {
        let converted = pipeline
            .worktree_file_to_object(rela_path.as_ref(), &index)
            .map_err(|e| Error::GixIndexError {
                cause: e.to_string(),
            })?;

        match converted {
            Some((id, kind, _)) => upserts.push((rela_path.clone(), id, kind)),
            // `None` means the file is gone from the worktree, so the entry should be too.
            None => removed.push(rela_path.clone()),
        }
        staged.push(rela_path.to_string());
    }

    for (rela_path, id, kind) in upserts {
        let rela_bstr: &gix::bstr::BStr = rela_path.as_ref();
        let absolute = workdir.join(gix::path::from_bstr(rela_bstr));
        let metadata = gix::index::fs::Metadata::from_path_no_follow(&absolute)?;
        let stat =
            gix::index::entry::Stat::from_fs(&metadata).map_err(|e| Error::GixIndexError {
                cause: e.to_string(),
            })?;
        let mode = gix::index::entry::Mode::from(gix::objs::tree::EntryMode::from(kind));

        match index
            .entry_mut_by_path_and_stage(rela_path.as_ref(), gix::index::entry::Stage::Unconflicted)
        {
            Some(entry) => {
                entry.id = id;
                entry.stat = stat;
                entry.mode = mode;
            }
            None => index.dangerously_push_entry(
                stat,
                id,
                // Stage 0. PATH_LEN is computed at serialization time and must not be set here.
                gix::index::entry::Flags::empty(),
                mode,
                rela_path.as_ref(),
            ),
        }
    }

    if !removed.is_empty() {
        index.remove_entries(|_, path, _| removed.iter().any(|removed| removed == path));
    }

    // Mandatory after `dangerously_push_entry`, which appends without regard for ordering.
    index.sort_entries();
    // See the tree-cache note above. Without this, a later `git commit` can capture stale content.
    index.remove_tree();
    // Rewriting the index drops resolve-undo either way; taking it explicitly documents the loss.
    index.remove_resolve_undo();

    let skip_hash = repo
        .config_snapshot()
        .boolean("index.skipHash")
        .unwrap_or(false);
    index
        .write(gix::index::write::Options {
            extensions: Default::default(),
            skip_hash,
        })
        .map_err(|e| Error::GixIndexError {
            cause: e.to_string(),
        })?;

    staged.sort();
    Ok(staged)
}

/// The repository-relative paths under the Xvc-owned set that differ from `HEAD`.
fn changed_xvc_paths(
    repo: &gix::Repository,
    index: &gix::index::File,
    prefix: &str,
) -> Result<Vec<BString>> {
    let changes = repo
        .status(gix::progress::Discard)
        .map_err(|e| Error::GixError {
            cause: e.to_string(),
        })?
        // Not the default `Collapsed`, which reports a wholly new `.xvc/store/` as one directory
        // entry. Staging needs each file, and a directory path would be converted to a removal.
        .untracked_files(gix::status::UntrackedFiles::Files)
        .index_worktree_submodules(None)
        // Compare against the very index that is about to be mutated, rather than letting
        // `status` re-read it from disk, so the two cannot see different states.
        .index(gix::worktree::IndexPersistedOrInMemory::InMemory(
            index.clone(),
        ))
        .into_iter(XvcGitPaths::pathspecs(prefix))
        .map_err(|e| Error::GixError {
            cause: e.to_string(),
        })?;

    let mut paths = Vec::new();
    for change in changes {
        let change = change.map_err(|e| Error::GixError {
            cause: e.to_string(),
        })?;
        let path = change.location().to_owned();
        if !paths.contains(&path) {
            paths.push(path);
        }
    }
    Ok(paths)
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
