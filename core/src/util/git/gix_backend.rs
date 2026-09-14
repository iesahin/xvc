//! Git operations that run in process, via [gix].
//!
//! Xvc's Git operations are being moved here from [`super::subprocess`] one at a time, lowest
//! risk first:
//!
//! 1. [tracked_files] — iterate the index instead of parsing `git ls-files --full-name`. **Done.**
//! 2. [xvc_paths_dirty] — [`gix::Repository::status`] instead of `git status --porcelain`. **Done.**
//! 3. [stage_xvc_paths] — write blobs through the filter pipeline and patch the index. **Done.**
//! 4. [commit_xvc_paths] — build the tree from `HEAD^{tree}` with [`gix::Repository::edit_tree`]
//!    and commit it, so the user's staging area is never touched and no stash is needed. **Done.**
//! 5. [create_and_switch_branch] — `edit_references` instead of `git checkout -b`. **Done.**
//!
//! That covers every Git operation Xvc performs on its own behalf. `git checkout <ref>` (the
//! `--from-ref` flag) is deliberately **not** on the list: it is a full `unpack_trees` two-way
//! merge against the live index and worktree, and gitoxide has no checkout orchestration;
//! reimplementing it risks silently destroying uncommitted user work. It stays on
//! [`super::subprocess::git_checkout_ref`], and so does the stash it needs.

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
/// Writing Git's index has several ways to go silently wrong. Each is guarded, and each guard is
/// explained where it lives: blobs in [convert_changed_paths], stat data in [apply_index_edits],
/// the tree-cache in [write_index], and split indexes in [refuse_split_index].
pub fn stage_xvc_paths(xvc_directory: &Path) -> Result<Vec<String>> {
    let (repo, prefix) = repo_and_prefix(xvc_directory)?;
    let workdir = repo
        .workdir()
        .expect("checked by repo_and_prefix")
        .to_owned();

    let mut index = open_index_or_empty(&repo)?;
    refuse_split_index(&index, "staging Xvc paths")?;

    let changed = changed_xvc_paths(&repo, &index, prefix.as_deref().unwrap_or_default())?;
    if changed.is_empty() {
        return Ok(Vec::new());
    }

    let edits = convert_changed_paths(&repo, &index, &changed)?;
    apply_index_edits(&mut index, &workdir, &edits)?;
    write_index(&repo, &mut index)?;

    let mut staged: Vec<String> = changed.iter().map(ToString::to_string).collect();
    staged.sort();
    Ok(staged)
}

/// Commit every changed path under the Xvc-owned path set, and return the new commit id.
///
/// This replaces the `git add` + `git commit` pair in
/// [`super::subprocess::git_auto_commit`]. `Ok(None)` means there was nothing to commit.
///
/// # Why the stash is gone
///
/// The subprocess version has to stash the user's staged files first, because `git commit` commits
/// the *whole index* — it has no way to commit a subset. That stash is the riskiest thing Xvc does:
/// it runs on every command, and a crash between `stash push` and `stash pop` strands the user's
/// staged work in a stash they never created.
///
/// Building the tree with [`gix::Repository::edit_tree`] removes the reason for it. The new tree
/// starts from `HEAD^{tree}` and only the Xvc-owned paths are edited into it, so whatever the user
/// has staged is not in the commit and never had to be moved out of the way. The index is patched
/// afterwards, entry by entry, for the same reason.
///
/// # Order of operations
///
/// The commit is written before the index, deliberately. Neither order is crash-atomic — `git`
/// isn't either — so the question is only which half-finished state is nicer to be left in.
///
/// Writing the index first and then failing to move the ref would leave the index claiming content
/// that `HEAD` does not have: `git status` shows all of `.xvc/` as staged, with nothing to
/// distinguish it from a real staged change. Committing first leaves `index` old, `HEAD` new and
/// the worktree new — a staged-revert and unstaged-reapply of the same content, which is confusing
/// to read but loses nothing and is undone by `git reset`.
pub fn commit_xvc_paths(xvc_directory: &Path, message: &str) -> Result<Option<String>> {
    let (repo, prefix) = repo_and_prefix(xvc_directory)?;
    let workdir = repo
        .workdir()
        .expect("checked by repo_and_prefix")
        .to_owned();

    let mut index = open_index_or_empty(&repo)?;
    refuse_split_index(&index, "committing Xvc paths")?;

    let changed = changed_xvc_paths(&repo, &index, prefix.as_deref().unwrap_or_default())?;
    if changed.is_empty() {
        return Ok(None);
    }

    // `_or_empty` so that a repository without commits — a fresh `git init`, which is what
    // `xvc init` meets — builds its first tree from the empty tree rather than erroring.
    let head_tree = repo
        .head_tree_id_or_empty()
        .map_err(|e| Error::GixCommitError {
            cause: e.to_string(),
        })?
        .detach();

    let edits = convert_changed_paths(&repo, &index, &changed)?;

    let mut editor = repo
        .edit_tree(head_tree)
        .map_err(|e| Error::GixCommitError {
            cause: e.to_string(),
        })?;
    for (rela_path, id, kind) in &edits.upserts {
        editor
            .upsert(rela_path, *kind, *id)
            .map_err(|e| Error::GixCommitError {
                cause: e.to_string(),
            })?;
    }
    for rela_path in &edits.removals {
        // `remove_leaf`, not `remove`: the paths come from a `Files` status walk so they are always
        // blobs, and if one somehow named a directory this drops nothing rather than a whole
        // subtree. Trees left empty by a removal are pruned by `write` below.
        editor
            .remove_leaf(rela_path)
            .map_err(|e| Error::GixCommitError {
                cause: e.to_string(),
            })?;
    }
    let new_tree = editor
        .write()
        .map_err(|e| Error::GixCommitError {
            cause: e.to_string(),
        })?
        .detach();

    if new_tree == head_tree {
        // Every changed path turned out to have the content `HEAD` already has, so `status`
        // reported a stat change and nothing more — a file rewritten with identical bytes, say.
        //
        // The subprocess version gets this wrong: `git add --verbose` prints a line for such a
        // path, the non-empty output is read as "something to commit", and the `git commit` that
        // follows fails with "nothing to commit" — an error Xvc then returns, failing a command
        // that did nothing wrong.
        //
        // The index is still written, even though there is no commit. Doing so refreshes the stat
        // data, so the next command's status walk sees a clean tree instead of rediscovering the
        // same non-change every time. It cannot stage anything visible: the entries being written
        // hold exactly the content `HEAD` has.
        apply_index_edits(&mut index, &workdir, &edits)?;
        write_index(&repo, &mut index)?;
        return Ok(None);
    }

    // Empty for an unborn `HEAD`, which is the "initial commit" case.
    let parents: Vec<gix::ObjectId> = repo
        .head()
        .map_err(|e| Error::GixCommitError {
            cause: e.to_string(),
        })?
        .id()
        .map(|id| id.detach())
        .into_iter()
        .collect();

    // Writes the ref and the reflog, dereferencing `HEAD` to the branch it points at.
    let commit_id = repo
        .commit("HEAD", message, new_tree, parents)
        .map_err(|e| match e {
            // `gix` reports this as a bare "author or committer is missing", which does not say
            // what to do about it. `git` names the configuration keys, so name them here too.
            // Under `git.backend = "auto"` this is caught before we get here and the `git` binary
            // handles the commit; reaching this means the in-process backend was asked for
            // explicitly.
            gix::commit::Error::AuthorMissing | gix::commit::Error::CommitterMissing => {
                Error::GixCommitError {
                    cause: "no Git identity is configured. Set it with \
                            `git config --global user.email \"you@example.com\"` and \
                            `git config --global user.name \"Your Name\"`, or set \
                            `git.backend = \"auto\"` to let the Git command handle commits"
                        .to_string(),
                }
            }
            e => Error::GixCommitError {
                cause: e.to_string(),
            },
        })?;

    apply_index_edits(&mut index, &workdir, &edits)?;
    write_index(&repo, &mut index)?;

    Ok(Some(commit_id.detach().to_string()))
}

/// Create `branch` and switch to it, as `git checkout -b <branch>` does.
///
/// This replaces `git checkout -b`. Called just before [`commit_xvc_paths`], at a point where the
/// worktree and index hold exactly what the command produced — creating a branch touches neither.
/// It is two reference writes and nothing else.
///
/// # `deref: false` is load-bearing
///
/// `HEAD` is itself a symbolic reference, ordinarily pointing at `refs/heads/<current branch>`.
/// [`gix::Repository::commit`] writes to `"HEAD"` with `deref: true`, deliberately, so a commit
/// lands on the current branch rather than detaching `HEAD`. Doing the same here would be wrong in
/// the opposite direction: it would resolve *through* `HEAD` and update whatever branch it
/// currently points at — silently repointing the user's actual branch to look like the new one,
/// instead of moving `HEAD` itself. `deref: false` applies the edit to `HEAD` literally.
///
/// # Unborn `HEAD`
///
/// On a fresh `git init`, `HEAD` exists but the branch it names does not — there is no commit yet
/// to create it at. Real `git checkout -b` in that state does not create the branch ref either; it
/// only repoints `HEAD`'s symbolic target, and the branch ref comes into existence on the first
/// commit. This does the same: the branch-creation edit is skipped when [`Head::id`](gix::Head::id)
/// is `None`, and [`commit_xvc_paths`]'s own `deref: true` write to `"HEAD"` creates
/// `refs/heads/<branch>` when it next runs — exactly as it already does for a first commit on
/// whatever branch `HEAD` already named.
///
/// # `PreviousValue::MustNotExist` is not what its name suggests
///
/// The obvious way to refuse an existing branch is `expected: PreviousValue::MustNotExist` on the
/// branch-creation edit. It does not refuse an existing branch — `gix_ref`'s own transaction code
/// only rejects it when the existing ref's value *differs* from the one being written, and silently
/// treats a matching value as success. A branch created at the same commit `HEAD` is already on —
/// unremarkable, since `git branch <name>` with no start point does exactly that — would pass
/// straight through and switch onto it. So existence is checked explicitly, before either edit is
/// built, independent of what the branch would have pointed at.
///
/// # `HEAD`'s reflog does not gain an entry here
///
/// `gix_ref` does not write a reflog entry for a symbolic-target change at all: its own comment
/// calls this "a special hack", since a reflog line needs an old and a new object id and a symbolic
/// target has neither. Only the branch's own reflog entry (`branch: Created from HEAD`, an
/// object-target change) is written by this function. Real `git checkout -b` does log the switch on
/// `HEAD`; in Xvc's flow the difference is short-lived, since [`commit_xvc_paths`] runs immediately
/// after and adds its own `HEAD` reflog entry for the commit that lands on the new branch.
pub fn create_and_switch_branch(xvc_directory: &Path, branch: &str) -> Result<()> {
    let repo = gix::discover(xvc_directory).map_err(|e| Error::GixError {
        cause: e.to_string(),
    })?;

    let branch_ref: gix::refs::FullName = format!("refs/heads/{branch}").try_into().map_err(
        |e: gix::validate::reference::name::Error| Error::GixReferenceEditError {
            cause: e.to_string(),
        },
    )?;

    if repo
        .try_find_reference(&branch_ref)
        .map_err(|e| Error::GixReferenceEditError {
            cause: e.to_string(),
        })?
        .is_some()
    {
        return Err(Error::GixReferenceEditError {
            cause: format!("a branch named '{branch}' already exists"),
        });
    }

    let head = repo.head().map_err(|e| Error::GixReferenceEditError {
        cause: e.to_string(),
    })?;

    let mut edits = Vec::with_capacity(2);

    // Skipped on an unborn HEAD: there is no commit yet for the branch to point at.
    if let Some(head_id) = head.id() {
        edits.push(gix::refs::transaction::RefEdit {
            change: gix::refs::transaction::Change::Update {
                log: gix::refs::transaction::LogChange {
                    mode: gix::refs::transaction::RefLog::AndReference,
                    force_create_reflog: false,
                    message: "branch: Created from HEAD".into(),
                },
                // The existence check above is what actually refuses an existing branch; see the
                // doc comment. This still guards the narrow race between that check and this write.
                expected: gix::refs::transaction::PreviousValue::MustNotExist,
                new: gix::refs::Target::Object(head_id.detach()),
            },
            name: branch_ref.clone(),
            deref: false,
        });
    }

    edits.push(gix::refs::transaction::RefEdit {
        change: gix::refs::transaction::Change::Update {
            // No reflog entry results from this; see the doc comment.
            log: gix::refs::transaction::LogChange::default(),
            // Not `MustExist`: on an unborn HEAD the ref exists but names a branch with no commit
            // yet, which is a legitimate starting point, not a condition to refuse.
            expected: gix::refs::transaction::PreviousValue::Any,
            new: gix::refs::Target::Symbolic(branch_ref),
        },
        name: "HEAD"
            .try_into()
            .expect("\"HEAD\" is a valid reference name"),
        deref: false,
    });

    // Both writes in one transaction, so a crash between them cannot leave the branch created but
    // HEAD still pointing at the old one — the transaction fully applies or fully does not.
    repo.edit_references(edits)
        .map_err(|e| Error::GixReferenceEditError {
            cause: e.to_string(),
        })?;

    Ok(())
}

/// What a set of changed paths implies for the index and for the tree being built.
struct IndexEdits {
    /// Paths present in the worktree, with the blob and file mode to record for them.
    upserts: Vec<(BString, gix::ObjectId, gix::objs::tree::EntryKind)>,
    /// Paths gone from the worktree, which should leave the index and the tree.
    removals: Vec<BString>,
}

/// Turn worktree files into blobs, splitting the changed paths into what to write and what to drop.
///
/// Blobs come from [`gix::filter::Pipeline::worktree_file_to_object`] rather than from hashing the
/// file's bytes, so `.gitattributes` clean filters and `core.autocrlf` apply and the object matches
/// what `git add` would have written. Hashing directly produces a different blob under
/// `core.autocrlf=true`, and the file then reads as permanently modified in `git status`.
fn convert_changed_paths(
    repo: &gix::Repository,
    index: &gix::index::File,
    changed: &[BString],
) -> Result<IndexEdits> {
    let (mut pipeline, _) = repo.filter_pipeline(None).map_err(|e| Error::GixError {
        cause: e.to_string(),
    })?;

    let mut edits = IndexEdits {
        upserts: Vec::with_capacity(changed.len()),
        removals: Vec::new(),
    };

    for rela_path in changed {
        let converted = pipeline
            .worktree_file_to_object(rela_path.as_ref(), index)
            .map_err(|e| Error::GixIndexError {
                cause: e.to_string(),
            })?;

        match converted {
            Some((id, kind, _)) => edits.upserts.push((rela_path.clone(), id, kind)),
            // `None` means the file is gone from the worktree, so the entry should be too.
            None => edits.removals.push(rela_path.clone()),
        }
    }

    Ok(edits)
}

/// Patch the index so the Xvc-owned entries match `edits`, leaving every other entry alone.
fn apply_index_edits(
    index: &mut gix::index::File,
    workdir: &Path,
    edits: &IndexEdits,
) -> Result<()> {
    for (rela_path, id, kind) in &edits.upserts {
        let rela_bstr: &gix::bstr::BStr = rela_path.as_ref();
        let absolute = workdir.join(gix::path::from_bstr(rela_bstr));
        // Not `std::fs::Metadata`: its `created()` reports the inode's *birth* time, while Git
        // records `st_ctime`. Recording the wrong one makes every entry look racy, so `git status`
        // re-hashes the whole tree on each run.
        let metadata = gix::index::fs::Metadata::from_path_no_follow(&absolute)?;
        let stat =
            gix::index::entry::Stat::from_fs(&metadata).map_err(|e| Error::GixIndexError {
                cause: e.to_string(),
            })?;
        let mode = gix::index::entry::Mode::from(gix::objs::tree::EntryMode::from(*kind));

        match index
            .entry_mut_by_path_and_stage(rela_path.as_ref(), gix::index::entry::Stage::Unconflicted)
        {
            Some(entry) => {
                entry.id = *id;
                entry.stat = stat;
                entry.mode = mode;
            }
            None => index.dangerously_push_entry(
                stat,
                *id,
                // Stage 0. PATH_LEN is computed at serialization time and must not be set here.
                gix::index::entry::Flags::empty(),
                mode,
                rela_path.as_ref(),
            ),
        }
    }

    if !edits.removals.is_empty() {
        index.remove_entries(|_, path, _| edits.removals.iter().any(|removed| removed == path));
    }

    Ok(())
}

/// Write the index back to `.git/index`, dropping the extensions a read-modify-write invalidates.
fn write_index(repo: &gix::Repository, index: &mut gix::index::File) -> Result<()> {
    // Mandatory after `dangerously_push_entry`, which appends without regard for ordering.
    index.sort_entries();
    // `gix` serializes the tree extension as-is rather than invalidating it, so a stale-but-valid
    // tree-cache would make a *later* `git commit` — the user's, not Xvc's — skip a directory it
    // should have re-read and capture outdated subtree content (gitoxide#2421).
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

    Ok(())
}

/// Read `.git/index`, treating an absent one as empty.
///
/// Not [`gix::Repository::open_index`], which errors when `.git/index` does not exist yet — the
/// state a fresh `git init` is in, and what `xvc init` meets.
fn open_index_or_empty(repo: &gix::Repository) -> Result<gix::index::File> {
    let index = match repo.try_index().map_err(|e| Error::GixIndexError {
        cause: e.to_string(),
    })? {
        // Two derefs: the shared index is an `Arc<FileSnapshot<File>>`.
        Some(index) => (**index).clone(),
        None => gix::index::File::from_state(
            gix::index::State::new(repo.object_hash()),
            repo.index_path(),
        ),
    };
    Ok(index)
}

/// Refuse to rewrite a split index, whose shared state lives in the `link` extension that a
/// read-modify-write would drop.
fn refuse_split_index(index: &gix::index::File, operation: &str) -> Result<()> {
    if index.link().is_some() {
        return Err(Error::GitBackendUnsupported {
            operation: operation.into(),
            reason: "the repository uses a split index".into(),
        });
    }
    Ok(())
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
