//! The set of paths Xvc owns and manages inside a Git repository.
//!
//! Xvc never stages arbitrary user content. It only ever touches its own metadata directory and
//! the ignore files it maintains. Keeping that set in a single place means the `git status`
//! pre-check and the `git add` that follows it agree by construction, rather than by three
//! copies of the same literals staying in sync by hand.

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
}
