//! Locating the Git repository and reading its ignore rules.
//!
//! Neither of these runs a `git` process: [inside_git] walks parent directories looking for
//! `.git`, and [build_gitignore] parses `.gitignore` files with [xvc_walker].

use std::path::{Path, PathBuf};

use crate::GIT_DIR;
use crate::Result;
use xvc_walker::{AbsolutePath, IgnoreRules, build_ignore_patterns};

use crate::util::xvcignore::COMMON_IGNORE_PATTERNS;

/// Where the Git repository containing a path is, if there is one.
///
/// Returned by [inside_git].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitRoot {
    /// A `.git` directory was found. Holds the directory that contains it, i.e. the repository
    /// root.
    Found(PathBuf),
    /// Neither the path nor any of its ancestors contains a `.git` directory.
    NotFound,
}

/// Check whether a path is inside a Git repository.
///
/// Returns [GitRoot::Found] with the closest ancestor directory containing `.git`, or
/// [GitRoot::NotFound]. It works by checking `.git` directories in parents, until no more parent
/// left.
pub fn inside_git(path: &Path) -> GitRoot {
    let mut pb = PathBuf::from(path)
        .canonicalize()
        .expect("Cannot canonicalize the path. Possible symlink loop.");
    loop {
        if pb.join(GIT_DIR).is_dir() {
            return GitRoot::Found(pb);
        } else if pb.parent().is_none() {
            return GitRoot::NotFound;
        } else {
            pb.pop();
        }
    }
}

/// Returns [xvc_walker::IgnoreRules] for `.gitignore`
/// It's used to check whether a path is already ignored by Git.
pub fn build_gitignore(git_root: &AbsolutePath) -> Result<IgnoreRules> {
    let rules = build_ignore_patterns(
        COMMON_IGNORE_PATTERNS,
        git_root,
        ".gitignore".to_owned().as_ref(),
    )?;

    Ok(rules)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use test_case::test_case;
    use xvc_test_helper::*;
    use xvc_walker::MatchResult as M;

    #[test_case("myfile.txt" , ".gitignore", "/myfile.txt" => matches M::Ignore ; "myfile.txt")]
    #[test_case("mydir/myfile.txt" , "mydir/.gitignore", "myfile.txt" => matches M::Ignore ; "mydir/myfile.txt")]
    #[test_case("mydir/myfile.txt" , ".gitignore", "/mydir/myfile.txt" => matches M::Ignore ; "from root dir")]
    #[test_case("mydir/myfile.txt" , ".gitignore", ""  => matches M::NoMatch ; "non ignore")]
    #[test_case("mydir/myfile.txt" , ".gitignore", "mydir/**" => matches M::Ignore ; "ignore dir star 2")]
    #[test_case("mydir/myfile.txt" , ".gitignore", "mydir/*" => matches M::Ignore ; "ignore dir star")]
    #[test_case("mydir/yourdir/myfile.txt" , "mydir/.gitignore", "yourdir/*" => matches M::Ignore ; "ignore deep dir star")]
    #[test_case("mydir/yourdir/myfile.txt" , "mydir/.gitignore", "yourdir/**" => matches M::Ignore ; "ignore deep dir star 2")]
    #[test_case("mydir/myfile.txt" , "another-dir/.gitignore", "another-dir/myfile.txt" => matches M::NoMatch ; "non ignore from dir")]
    fn test_gitignore(path: &str, gitignore_path: &str, ignore_line: &str) -> M {
        test_logging(log::LevelFilter::Trace);
        let git_root = temp_git_dir();
        let path = git_root.join(PathBuf::from(path));
        let gitignore_path = git_root.join(PathBuf::from(gitignore_path));
        if let Some(ignore_dir) = gitignore_path.parent() {
            fs::create_dir_all(ignore_dir).unwrap();
        }
        fs::write(&gitignore_path, format!("{}\n", ignore_line)).unwrap();

        let gitignore = build_ignore_patterns("", &git_root, ".gitignore").unwrap();

        gitignore.check(&path)
    }
}
