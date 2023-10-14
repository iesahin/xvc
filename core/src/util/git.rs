//! Git operations
use std::path::{Path, PathBuf};

use xvc_walker::{build_ignore_rules, AbsolutePath, IgnoreRules};

use crate::error::Result;
use crate::GIT_DIR;

/// Check whether a path is inside a Git repository.
/// It returns `None` if not, otherwise returns the closest directory with `.git`.
/// It works by checking `.git` directories in parents, until no more parent left.
pub fn inside_git(path: &Path) -> Option<PathBuf> {
    let mut pb = PathBuf::from(path)
        .canonicalize()
        .expect("Cannot canonicalize the path. Possible symlink loop.");
    loop {
        if pb.join(GIT_DIR).is_dir() {
            return Some(pb);
        } else if pb.parent().is_none() {
            return None;
        } else {
            pb.pop();
        }
    }
}

/// Returns [xvc_walker::IgnoreRules] for `.gitignore`
/// It's used to check whether a path is already ignored by Git.
pub fn build_gitignore(git_root: &AbsolutePath) -> Result<IgnoreRules> {
    let initial_rules = IgnoreRules::empty(git_root);

    let rules = build_ignore_rules(initial_rules, git_root, ".gitignore")?;

    Ok(rules)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use test_case::test_case;
    use xvc_logging::watch;
    use xvc_test_helper::*;
    use xvc_walker::check_ignore;
    use xvc_walker::MatchResult as M;

    #[test_case("myfile.txt" , ".gitignore", "/myfile.txt" => matches M::Ignore ; "myfile.txt")]
    #[test_case("mydir/myfile.txt" , "mydir/.gitignore", "myfile.txt" => matches M::Ignore ; "mydir/myfile.txt")]
    #[test_case("mydir/myfile.txt" , ".gitignore", "/mydir/myfile.txt" => matches M::Ignore ; "from root dir")]
    #[test_case("mydir/myfile.txt" , ".gitignore", ""  => matches M::NoMatch ; "non ignore")]
    #[test_case("mydir/myfile.txt" , ".gitignore", "mydir/**" => matches M::Ignore ; "ignore dir star 2")]
    #[test_case("mydir/myfile.txt" , ".gitignore", "mydir/*" => matches M::Ignore ; "ignore dir star")]
    #[test_case("mydir/yourdir/myfile.txt" , "mydir/.gitignore", "yourdir/**" => matches M::Ignore ; "ignore deep dir star 2")]
    #[test_case("mydir/yourdir/myfile.txt" , "mydir/.gitignore", "yourdir/*" => matches M::Ignore ; "ignore deep dir star")]
    #[test_case("mydir/myfile.txt" , "hebelep/.gitignore", "hebelep/myfile.txt" => matches M::NoMatch ; "non ignore from dir")]
    fn test_gitignore(path: &str, gitignore_path: &str, ignore_line: &str) -> M {
        test_logging(log::LevelFilter::Trace);
        let git_root = temp_git_dir();
        let path = git_root.join(PathBuf::from(path));
        let gitignore_path = git_root.join(PathBuf::from(gitignore_path));
        if let Some(ignore_dir) = gitignore_path.parent() {
            fs::create_dir_all(ignore_dir).unwrap();
        }
        fs::write(&gitignore_path, format!("{}\n", ignore_line)).unwrap();

        let gitignore = build_ignore_rules(
            IgnoreRules::empty(&git_root),
            gitignore_path.parent().unwrap(),
            ".gitignore",
        )
        .unwrap();

        watch!(gitignore);

        check_ignore(&gitignore, &path)
    }
}
