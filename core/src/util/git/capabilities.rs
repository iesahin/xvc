//! What the in-process backend cannot do for a given repository.
//!
//! [`gix`] does not run hooks and does not sign commits, and neither omission announces itself. A
//! user whose `pre-commit` hook runs a formatter would get unformatted commits; a user under a
//! signed-commit policy would get an unsigned commit that is accepted locally and rejected on
//! push. Both look like Xvc quietly changing behavior, with nothing in the output to explain it.
//!
//! So the conditions are detected rather than left to the user to notice, and Xvc falls back to
//! running the `git` binary when any of them holds. The cost is one configuration read and four
//! `stat` calls, once per command.

use std::path::{Path, PathBuf};

/// The hooks `git commit` runs, and `gix` does not.
const COMMIT_HOOKS: [&str; 4] = [
    "pre-commit",
    "prepare-commit-msg",
    "commit-msg",
    "post-commit",
];

/// Why the in-process backend cannot be used for this repository, or `None` if it can.
///
/// The returned string is a sentence fragment naming the condition, suitable for logging as
/// "using the Git command because {reason}".
pub fn gix_unsupported(repo: &gix::Repository, index: &gix::index::File) -> Option<String> {
    let config = repo.config_snapshot();

    // `gix` builds its commits with `extra_headers: Default::default()`, so there is no `gpgsig`
    // header to write — signing is not merely unimplemented, there is nowhere to put a signature.
    if config.boolean("commit.gpgsign").unwrap_or(false) {
        return Some("commit.gpgsign is enabled".to_string());
    }

    let hooks = hooks_dir(repo);
    for hook in COMMIT_HOOKS {
        let path = hooks.join(hook);
        if is_executable_hook(&path) {
            return Some(format!("the {hook} hook is installed"));
        }
    }

    // Rewriting a split index would drop the `link` extension that holds the shared state, which
    // is destructive rather than merely lossy. `gix_backend` refuses this too; catching it here
    // turns the refusal into a fallback.
    if index.link().is_some() {
        return Some("the repository uses a split index".to_string());
    }

    // `gix` hard-errors without `user.name`/`user.email`, where `git` first tries to synthesize
    // `user@hostname` and only refuses if it cannot build a plausible address. Deferring to `git`
    // keeps whichever of those two the user has always seen on this machine, instead of Xvc
    // starting to fail where it used to work. `gix` also offers a generic fallback identity, but
    // it writes "no name configured <noEmailAvailable@example.com>" into the user's history, which
    // is worse than either outcome.
    if repo.author().is_none() || repo.committer().is_none() {
        return Some("no Git identity is configured (user.name and user.email)".to_string());
    }

    None
}

/// Whether `path` is a hook Git would run.
///
/// The `.sample` files every `git init` writes must not count — they are the same names with a
/// suffix, so matching on the exact name already excludes them. On unix Git additionally requires
/// the executable bit, which is what stops a stray non-executable file from disabling the
/// in-process backend.
fn is_executable_hook(path: &Path) -> bool {
    let metadata = match std::fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return false,
    };

    if !metadata.is_file() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o111 != 0
    }
    // Windows has no executable bit, and Git runs any hook file it finds there.
    #[cfg(not(unix))]
    {
        true
    }
}

/// Where Git would look for hooks: `core.hooksPath` if set, `.git/hooks` otherwise.
fn hooks_dir(repo: &gix::Repository) -> PathBuf {
    repo.config_snapshot()
        .trusted_path("core.hooksPath")
        .transpose()
        .ok()
        .flatten()
        .map(|path| path.into_owned())
        .unwrap_or_else(|| repo.common_dir().join("hooks"))
}
