//! The set of paths Xvc owns and manages inside a Git repository.
//!
//! Xvc never stages arbitrary user content. It only ever touches its own metadata directory and
//! the ignore files it maintains. Keeping that set in a single place means the `git status`
//! pre-check and the `git add` that follows it agree by construction, rather than by three
//! copies of the same literals staying in sync by hand.

use gix::bstr::BString;

use crate::{XVC_DIR, XVCIGNORE_FILENAME};

/// Pathspec matching the `.gitignore` files Xvc writes to.
///
/// NOTE: This is a bare Git pathspec, so `*` matches `/` too (`fnmatch` without `FNM_PATHNAME`).
/// It therefore also matches paths like `sub/dir/x.gitignore`, not only `sub/dir/.gitignore`.
pub const GITIGNORE_PATHSPEC: &str = "*.gitignore";

/// Pathspec matching the `.xvcignore` files in the repository.
///
/// See [GITIGNORE_PATHSPEC] for the matching caveat.
pub const XVCIGNORE_PATHSPEC: &str = "*.xvcignore";

/// The paths Xvc owns and manages in Git.
pub struct XvcGitPaths;

impl XvcGitPaths {
    /// The pathspecs to hand to the `git` binary, in the order Xvc has always passed them.
    ///
    /// `xvc_dir` is the absolute path to the `.xvc` directory, as returned by
    /// [`XvcRoot::xvc_dir`][crate::XvcRoot::xvc_dir].
    pub fn subprocess_pathspecs(xvc_dir: &str) -> [&str; 3] {
        [xvc_dir, GITIGNORE_PATHSPEC, XVCIGNORE_PATHSPEC]
    }

    /// The same set as Git pathspecs, for the in-process backend.
    ///
    /// `prefix` is where the Xvc root sits inside the repository, slash-terminated — empty when
    /// the two are the same directory, `"sub/"` when `xvc init` ran in `sub`. Unlike the
    /// subprocess form, these are always relative to the repository root, because
    /// [`gix::Repository::status`] has no working directory to be relative to. The `(top)` magic
    /// says so explicitly.
    ///
    /// # Narrower than [subprocess_pathspecs][Self::subprocess_pathspecs]
    ///
    /// `*.gitignore` is a bare pathspec, where `*` matches `/` as well, so it also selects
    /// `sub/x.gitignore` — a file Git attaches no meaning to and Xvc never writes. The `(glob)`
    /// magic stops `*` from crossing directory separators, and `**/` matches any number of
    /// leading directories, so `**/.gitignore` selects exactly the ignore files themselves.
    pub fn pathspecs(prefix: &str) -> Vec<BString> {
        vec![
            format!(":(top){prefix}{XVC_DIR}").into(),
            format!(":(top,glob){prefix}**/.gitignore").into(),
            format!(":(top,glob){prefix}**/{XVCIGNORE_FILENAME}").into(),
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// The `git status` pre-check and the `git add` that follows must select the same paths, or
    /// Xvc skips a commit it should have made. This pins the set they share.
    #[test]
    fn subprocess_pathspecs_are_the_documented_set() {
        assert_eq!(
            XvcGitPaths::subprocess_pathspecs("/repo/.xvc"),
            ["/repo/.xvc", "*.gitignore", "*.xvcignore"]
        );
    }

    #[test]
    fn pathspecs_at_the_repository_root() {
        assert_eq!(
            XvcGitPaths::pathspecs(""),
            vec![
                BString::from(":(top).xvc"),
                BString::from(":(top,glob)**/.gitignore"),
                BString::from(":(top,glob)**/.xvcignore"),
            ]
        );
    }

    /// When `xvc init` ran in a subdirectory, the pathspecs stay anchored to the repository root
    /// but are scoped to that subdirectory — matching what `git -C <xvc root>` did by having the
    /// working directory scope them.
    #[test]
    fn pathspecs_under_a_subdirectory_xvc_root() {
        assert_eq!(
            XvcGitPaths::pathspecs("sub/"),
            vec![
                BString::from(":(top)sub/.xvc"),
                BString::from(":(top,glob)sub/**/.gitignore"),
                BString::from(":(top,glob)sub/**/.xvcignore"),
            ]
        );
    }
}
