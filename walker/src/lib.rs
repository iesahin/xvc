//! Xvc walker traverses directory trees with ignore rules.
//!
//! Ignore rules are similar to [.gitignore](https://git-scm.com/docs/gitignore) and child
//! directories are not traversed if ignored.
//!
//! [walk_parallel] function is the most useful element in this module.
//! It walks and sends [PathMetadata] through a channel, also updating the ignore rules and sending
//! them.
#![warn(missing_docs)]
#![forbid(unsafe_code)]
pub mod abspath;
pub mod error;
pub mod ignore_rules;
pub mod notify;
pub mod pattern;
pub mod sync;
pub mod walk_parallel;
pub mod walk_serial;

pub use pattern::MatchResult;
pub use pattern::PathKind;
pub use pattern::Pattern;
pub use pattern::PatternEffect;
pub use pattern::PatternRelativity;
pub use pattern::Source;

pub use walk_parallel::walk_parallel;
pub use walk_serial::walk_serial;

pub use abspath::AbsolutePath;
pub use error::{Error, Result};

pub use ignore_rules::IgnoreRules;
pub use ignore_rules::SharedIgnoreRules;

pub use notify::make_watcher;
use std::ffi::OsStr;
pub use std::hash::Hash;
pub use sync::{PathSync, PathSyncSingleton};
use xvc_logging::warn;

pub use notify::PathEvent;
pub use notify::RecommendedWatcher;

use xvc_logging::watch;

// use glob::{MatchOptions, Pattern, PatternError};
pub use fast_glob::Glob;
use std::{
    ffi::OsString,
    fmt::Debug,
    fs::{self, Metadata},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context};

static MAX_THREADS_PARALLEL_WALK: usize = 8;

/// Combine a path and its metadata in a single struct
#[derive(Debug, Clone)]
pub struct PathMetadata {
    /// path
    pub path: PathBuf,
    /// metadata
    pub metadata: Metadata,
}

/// What's the ignore file name and should we add directories to the result?
#[derive(Debug, Clone)]
pub struct WalkOptions {
    /// The ignore filename (`.gitignore`, `.xvcignore`, `.ignore`, etc.) or `None` for not
    /// ignoring anything.
    pub ignore_filename: Option<String>,
    /// Should the results include directories themselves?
    /// Note that they are always traversed, but may not be listed if we're only interested in
    /// actual files.
    pub include_dirs: bool,
}

impl WalkOptions {
    /// Instantiate a Git repository walker that uses `.gitignore` as ignore file name and includes
    /// directories in results.
    pub fn gitignore() -> Self {
        Self {
            ignore_filename: Some(".gitignore".into()),
            include_dirs: true,
        }
    }

    /// Instantiate a Xvc repository walker that uses `.xvcignore` as ignore file name and includes
    /// directories in results.
    pub fn xvcignore() -> Self {
        Self {
            ignore_filename: Some(".xvcignore".into()),
            include_dirs: true,
        }
    }

    /// Return options with `include_dirs` turned off.
    /// `WalkOptions::xvcignore().without_dirs()` specifies a `xvcignore` walker that only lists
    /// files.
    pub fn without_dirs(self) -> Self {
        Self {
            ignore_filename: self.ignore_filename,
            include_dirs: false,
        }
    }
    /// Return the same option with `include_dirs` turned on.
    pub fn with_dirs(self) -> Self {
        Self {
            ignore_filename: self.ignore_filename,
            include_dirs: true,
        }
    }
}

/// Build the ignore rules with the given directory
pub fn build_ignore_patterns(
    given: &str,
    ignore_root: &Path,
    ignore_filename: &str,
) -> Result<IgnoreRules> {
    watch!(ignore_filename);
    watch!(ignore_root);

    let ignore_rules = IgnoreRules::from_global_patterns(ignore_root, Some(ignore_filename), given);

    let dirs_under = |p: &Path| -> Vec<PathBuf> {
        p.read_dir()
            .unwrap()
            .filter_map(|p| {
                if let Ok(p) = p {
                    if p.path().is_dir() {
                        Some(p.path())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .filter_map(|p| match ignore_rules.check(&p) {
                MatchResult::NoMatch | MatchResult::Whitelist => Some(p),
                MatchResult::Ignore => None,
            })
            .collect()
    };

    let mut dir_stack: Vec<PathBuf> = vec![ignore_root.to_path_buf()];

    let ignore_fn = ignore_rules.ignore_filename.as_deref().unwrap();

    while let Some(dir) = dir_stack.pop() {
        watch!(dir);
        let ignore_filename = dir.join(ignore_fn);
        watch!(ignore_filename);
        if ignore_filename.is_file() {
            let ignore_content = fs::read_to_string(&ignore_filename)?;
            let new_patterns =
                content_to_patterns(ignore_root, Some(&ignore_filename), &ignore_content);
            ignore_rules.add_patterns(new_patterns)?;
        }
        let mut new_dirs = dirs_under(&dir);
        watch!(new_dirs);
        dir_stack.append(&mut new_dirs);
        watch!(dir_stack);
    }

    Ok(ignore_rules)
}

/// convert a set of rules in `content` to glob patterns.
/// patterns may come from `source`.
/// the root directory of all search is in `ignore_root`.
pub fn content_to_patterns(
    ignore_root: &Path,
    source: Option<&Path>,
    content: &str,
) -> Vec<Pattern> {
    let patterns: Vec<Pattern> = content
        .lines()
        .enumerate()
        // A line starting with # serves as a comment. Put a backslash ("\") in front of the first hash for patterns that begin with a hash.
        .filter(|(_, line)| !(line.trim().is_empty() || line.starts_with('#')))
        // Trailing spaces are ignored unless they are quoted with backslash ("\").
        .map(|(i, line)| {
            if !line.ends_with("\\ ") {
                (i, line.trim_end())
            } else {
                (i, line)
            }
        })
        // if source file is not given, set the source Global
        .map(|(i, line)| {
            (
                line,
                match source {
                    Some(p) => Source::File {
                        path: p
                            .strip_prefix(ignore_root)
                            .expect("path must be within ignore_root")
                            .to_path_buf(),
                        line: (i + 1),
                    },
                    None => Source::Global,
                },
            )
        })
        .map(|(line, source)| Pattern::new(source, line))
        .collect();

    patterns
}

pub fn update_ignore_rules(dir: &Path, ignore_rules: &IgnoreRules) -> Result<()> {
    if let Some(ref ignore_filename) = ignore_rules.ignore_filename {
        let ignore_root = &ignore_rules.root;
        let ignore_path = dir.join(ignore_filename);
        if ignore_path.is_file() {
            let new_patterns: Vec<Pattern> = {
                let content = fs::read_to_string(&ignore_path)?;
                content_to_patterns(ignore_root, Some(ignore_path).as_deref(), &content)
            };

            ignore_rules.add_patterns(new_patterns)?;
        }
    }
    Ok(())
}
/// Return all childs of a directory regardless of any ignore rules
/// If there is an error to obtain the metadata, error is added to the element instead
pub fn directory_list(dir: &Path) -> Result<Vec<Result<PathMetadata>>> {
    let elements = dir
        .read_dir()
        .map_err(|e| anyhow!("Error reading directory: {:?}, {:?}", dir, e))?;
    let mut child_paths = Vec::<Result<PathMetadata>>::new();

    for entry in elements {
        match entry {
            Err(err) => child_paths.push(Err(Error::from(anyhow!(
                "Error reading entry in dir {:?} {:?}",
                dir,
                err
            )))),
            Ok(entry) => match entry.metadata() {
                Err(err) => child_paths.push(Err(Error::from(anyhow!(
                    "Error getting metadata {:?} {:?}",
                    entry,
                    err
                )))),
                Ok(md) => {
                    child_paths.push(Ok(PathMetadata {
                        path: entry.path(),
                        metadata: md.clone(),
                    }));
                }
            },
        }
    }
    Ok(child_paths)
}

#[cfg(test)]
mod tests {

    use super::*;

    use log::LevelFilter;
    use test_case::test_case;

    use crate::error::Result;
    use crate::AbsolutePath;
    use xvc_test_helper::*;

    #[test_case("!mydir/*/file" => matches PatternEffect::Whitelist ; "t1159938339")]
    #[test_case("!mydir/myfile" => matches PatternEffect::Whitelist ; "t1302522194")]
    #[test_case("!myfile" => matches PatternEffect::Whitelist ; "t3599739725")]
    #[test_case("!myfile/" => matches PatternEffect::Whitelist ; "t389990097")]
    #[test_case("/my/file" => matches PatternEffect::Ignore ; "t3310011546")]
    #[test_case("mydir/*" => matches PatternEffect::Ignore ; "t1461510927")]
    #[test_case("mydir/file" => matches PatternEffect::Ignore; "t4096563949")]
    #[test_case("myfile" => matches PatternEffect::Ignore; "t4042406621")]
    #[test_case("myfile*" => matches PatternEffect::Ignore ; "t3367706249")]
    #[test_case("myfile/" => matches PatternEffect::Ignore ; "t1204466627")]
    fn test_pattern_effect(line: &str) -> PatternEffect {
        let pat = Pattern::new(Source::Global, line);
        pat.effect
    }

    #[test_case("", "!mydir/*/file" => matches PatternRelativity::RelativeTo { directory } if directory.is_empty() ; "t500415168")]
    #[test_case("", "!mydir/myfile" => matches PatternRelativity::RelativeTo {directory} if directory.is_empty() ; "t1158125354")]
    #[test_case("dir/", "!mydir/*/file" => matches PatternRelativity::RelativeTo { directory } if directory == "/dir" ; "t3052699971")]
    #[test_case("dir/", "!mydir/myfile" => matches PatternRelativity::RelativeTo {directory} if directory == "/dir" ; "t885029019")]
    #[test_case("", "!myfile" => matches PatternRelativity::Anywhere; "t3101661374")]
    #[test_case("", "!myfile/" => matches PatternRelativity::Anywhere ; "t3954695505")]
    #[test_case("", "/my/file" => matches PatternRelativity::RelativeTo { directory } if directory.is_empty() ; "t1154256567")]
    #[test_case("", "mydir/*" => matches PatternRelativity::RelativeTo { directory } if directory.is_empty() ; "t865348822")]
    #[test_case("", "mydir/file" => matches PatternRelativity::RelativeTo { directory } if directory.is_empty() ; "t809589695")]
    #[test_case("root/", "/my/file" => matches PatternRelativity::RelativeTo { directory } if directory == "/root" ; "t7154256567")]
    #[test_case("root/", "mydir/*" => matches PatternRelativity::RelativeTo { directory } if directory == "/root" ; "t765348822")]
    #[test_case("root/", "mydir/file" => matches PatternRelativity::RelativeTo { directory } if directory == "/root" ; "t709589695")]
    #[test_case("", "myfile" => matches PatternRelativity::Anywhere; "t949952742")]
    #[test_case("", "myfile*" => matches PatternRelativity::Anywhere ; "t2212007572")]
    #[test_case("", "myfile/" => matches PatternRelativity::Anywhere; "t900104620")]
    fn test_pattern_relativity(dir: &str, line: &str) -> PatternRelativity {
        let source = Source::File {
            path: PathBuf::from(dir).join(".gitignore"),
            line: 1,
        };
        let pat = Pattern::new(source, line);
        pat.relativity
    }

    #[test_case("", "!mydir/*/file" => matches PathKind::Any ; "t4069397926")]
    #[test_case("", "!mydir/myfile" => matches PathKind::Any ; "t206435934")]
    #[test_case("", "!myfile" => matches PathKind::Any ; "t4262638148")]
    #[test_case("", "!myfile/" => matches PathKind::Directory ; "t214237847")]
    #[test_case("", "/my/file" => matches PathKind::Any ; "t187692643")]
    #[test_case("", "mydir/*" => matches PathKind::Any ; "t1159784957")]
    #[test_case("", "mydir/file" => matches PathKind::Any ; "t2011171465")]
    #[test_case("", "myfile" => matches PathKind::Any ; "t167946945")]
    #[test_case("", "myfile*" => matches PathKind::Any ; "t3091563211")]
    #[test_case("", "myfile/" => matches PathKind::Directory ; "t1443554623")]
    fn test_path_kind(dir: &str, line: &str) -> PathKind {
        let source = Source::File {
            path: PathBuf::from(dir).join(".gitignore"),
            line: 1,
        };
        let pat = Pattern::new(source, line);
        pat.path_kind
    }

    #[test_case("" => 0)]
    #[test_case("myfile" => 1)]
    #[test_case("mydir/myfile" => 1)]
    #[test_case("mydir/myfile\n!myfile" => 2)]
    #[test_case("mydir/myfile\n/another" => 2)]
    #[test_case("mydir/myfile\n\n\nanother" => 2)]
    #[test_case("#comment\nmydir/myfile\n\n\nanother" => 2)]
    #[test_case("#mydir/myfile" => 0)]
    fn test_content_to_patterns_count(contents: &str) -> usize {
        let patterns = content_to_patterns(Path::new(""), None, contents);
        patterns.len()
    }

    fn create_patterns(root: &str, dir: Option<&str>, patterns: &str) -> Vec<Pattern> {
        content_to_patterns(Path::new(root), dir.map(Path::new), patterns)
    }

    fn new_dir_with_ignores(
        root: &str,
        dir: Option<&str>,
        initial_patterns: &str,
    ) -> Result<IgnoreRules> {
        let patterns = create_patterns(root, dir, initial_patterns);
        let initialized = IgnoreRules::empty(&PathBuf::from(root), None);

        initialized.add_patterns(patterns)?;
        Ok(initialized)
    }

    #[test_case(".", "" ; "empty_dwi")]
    #[test_case("dir", "myfile")]
    #[test_case("dir", "mydir/myfile")]
    #[test_case("dir", "mydir/myfile\n!myfile")]
    #[test_case("dir", "mydir/myfile\n/another")]
    #[test_case("dir", "mydir/myfile\n\n\nanother")]
    #[test_case("dir", "#comment\nmydir/myfile\n\n\nanother")]
    #[test_case("dir", "#mydir/myfile" ; "single ignored lined")]
    fn test_dir_with_ignores(dir: &str, contents: &str) {
        new_dir_with_ignores(dir, None, contents).unwrap();
    }

    #[test_case("/dir", "/mydir/myfile/" => matches PatternRelativity::RelativeTo { directory } if directory == "/dir" ; "t868594159")]
    #[test_case("/dir", "mydir" => matches PatternRelativity::Anywhere ; "t4030766779")]
    #[test_case("/dir/", "mydir/myfile" => matches PatternRelativity::RelativeTo { directory } if directory == "/dir" ; "t2043231107")]
    #[test_case("dir", "myfile" => matches PatternRelativity::Anywhere; "t871610344" )]
    #[test_case("dir/", "mydir/myfile" => matches PatternRelativity::RelativeTo { directory } if directory == "/dir" ; "t21398102")]
    #[test_case("dir/", "myfile" => matches PatternRelativity::Anywhere ; "t1846637197")]
    #[test_case("dir//", "/mydir/myfile" => matches PatternRelativity::RelativeTo { directory } if directory == "/dir" ; "t2556287848")]
    fn test_path_relativity(dir: &str, pattern: &str) -> PatternRelativity {
        let source = Source::File {
            path: PathBuf::from(format!("{dir}/.gitignore")),
            line: 1,
        };
        let pattern = Pattern::new(source, pattern);
        pattern.relativity
    }

    #[test_case("", "myfile" => "myfile" ; "t1142345310")]
    #[test_case("", "/myfile" => "myfile" ; "t1427001291")]
    #[test_case("", "myfile/" => "myfile" ; "t789151905")]
    #[test_case("", "mydir/myfile" => "mydir/myfile" ; "t21199018162")]
    #[test_case("", "myfile.*" => "myfile.*" ; "t31199018162")]
    #[test_case("", "mydir/**.*" => "mydir/**.*" ; "t41199018162")]
    #[test_case("dir", "myfile" => "myfile" ; "t1242345310")]
    #[test_case("dir", "/myfile" => "myfile" ; "t3427001291")]
    #[test_case("dir", "myfile/" => "myfile" ; "t759151905")]
    #[test_case("dir", "mydir/myfile" => "mydir/myfile" ; "t21199018562")]
    #[test_case("dir", "/my/file.*" => "my/file.*" ; "t61199018162")]
    #[test_case("dir", "/mydir/**.*" => "mydir/**.*" ; "t47199018162")]
    fn test_pattern_line(dir: &str, pattern: &str) -> String {
        let source = Source::File {
            path: PathBuf::from(format!("{dir}.gitignore")),
            line: 1,
        };
        let pattern = Pattern::new(source, pattern);
        pattern.glob
    }

    // Blank file tests
    #[test_case("", "#mydir/myfile", ""  => matches MatchResult::NoMatch ; "t01")]
    #[test_case("", "", ""  => matches MatchResult::NoMatch ; "t02" )]
    #[test_case("", "\n\n  \n", ""  => matches MatchResult::NoMatch; "t03"  )]
    #[test_case("", "dir-0001", ""  => matches MatchResult::NoMatch ; "t04" )]
    #[test_case("", "dir-0001/file-0001.bin", ""  => matches MatchResult::NoMatch ; "t05" )]
    #[test_case("", "dir-0001/*", ""  => matches MatchResult::NoMatch ; "t06" )]
    #[test_case("", "dir-0001/**", ""  => matches MatchResult::NoMatch ; "t07" )]
    #[test_case("", "dir-0001/dir-0001**", ""  => matches MatchResult::NoMatch ; "t08" )]
    #[test_case("", "dir-0001/dir-00*", ""  => matches MatchResult::NoMatch ; "t09" )]
    #[test_case("", "dir-00**/", ""  => matches MatchResult::NoMatch ; "t10" )]
    #[test_case("", "dir-00**/*/file-0001.bin", ""  => matches MatchResult::NoMatch ; "t11" )]
    #[test_case("", "dir-00**/*/*.bin", ""  => matches MatchResult::NoMatch ; "t12" )]
    #[test_case("", "dir-00**/", ""  => matches MatchResult::NoMatch ; "t13" )]
    #[test_case("", "#mydir/myfile", ""  => matches MatchResult::NoMatch ; "t148864489901")]
    // No Match Tests
    #[test_case("", "", "dir-0001/file-0002.bin"  => matches MatchResult::NoMatch ; "t172475356002" )]
    #[test_case("", "\n\n  \n", "dir-0001/file-0002.bin" => matches MatchResult::NoMatch; "t8688937603"  )]
    #[test_case("", "dir-0001", "dir-0001/file-0002.bin" => matches MatchResult::NoMatch ; "t132833780304" )]
    #[test_case("", "dir-0001/file-0001.bin", "dir-0001/file-0002.bin" => matches MatchResult::NoMatch ; "t173193800505" )]
    #[test_case("", "dir-0001/dir-0001**", "dir-0001/file-0002.bin" => matches MatchResult::NoMatch ; "t318664043308" )]
    #[test_case("", "dir-0001/dir-00*", "dir-0001/file-0002.bin" => matches MatchResult::NoMatch ; "t269908728009" )]
    #[test_case("", "dir-00**/*/file-0001.bin", "dir-0001/file-0002.bin" => matches MatchResult::NoMatch ; "t142240004811" )]
    #[test_case("", "dir-00**/*/*.bin", "dir-0001/file-0002.bin" => matches MatchResult::NoMatch ; "t414921892712" )]
    #[test_case("", "dir-00**/", "dir-0001/file-0002.bin" => matches MatchResult::Ignore; "t256322548613" )]
    // Ignore tests
    #[test_case("", "dir-0001/file-0001.bin", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t3378553489" )]
    #[test_case("", "dir-0001/file-0001.*", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t3449646229" )]
    #[test_case("", "dir-0001/*.bin", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t1232001745" )]
    #[test_case("", "dir-0001/*", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t2291655464" )]
    #[test_case("", "dir-0001/**/*.bin", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t355659763" )]
    #[test_case("", "dir-0001/**", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t1888678340" )]
    #[test_case("", "dir-000?/file-0001.bin", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t1603222532" )]
    #[test_case("", "dir-000?/*.bin", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t2528090273" )]
    #[test_case("", "dir-*/*", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t3141482339" )]
    // Whitelist Tests
    #[test_case("", "!dir-0001", "dir-0001/file-0002.bin" => matches MatchResult::NoMatch ; "t2963495371" )]
    #[test_case("", "!dir-0001/file-0001.bin", "dir-0001/file-0002.bin" => matches MatchResult::NoMatch ; "t3935333051" )]
    #[test_case("", "!dir-0001/dir-0001**", "dir-0001/file-0002.bin" => matches MatchResult::NoMatch ; "t3536143628" )]
    #[test_case("", "!dir-0001/dir-00*", "dir-0001/file-0002.bin" => matches MatchResult::NoMatch ; "t4079058836" )]
    #[test_case("", "!dir-00**/", "dir-0001/file-0002.bin" => matches MatchResult::Whitelist ; "t3713155445" )]
    #[test_case("", "!dir-00**/*/file-0001.bin", "dir-0001/file-0002.bin" => matches MatchResult::NoMatch ; "t1434153118" )]
    #[test_case("", "!dir-00**/*/*.bin", "dir-0001/file-0002.bin" => matches MatchResult::NoMatch ; "t1650195998" )]
    #[test_case("", "!dir-0001/file-0001.bin", "dir-0001/file-0001.bin" => matches MatchResult::Whitelist ; "t1569068369" )]
    #[test_case("", "!dir-0001/file-0001.*", "dir-0001/file-0001.bin" => matches MatchResult::Whitelist ; "t2919165396" )]
    #[test_case("", "!dir-0001/*.bin", "dir-0001/file-0001.bin" => matches MatchResult::Whitelist ; "t2682012728" )]
    #[test_case("", "!dir-0001/*", "dir-0001/file-0001.bin" => matches MatchResult::Whitelist ; "t4009543743" )]
    #[test_case("", "!dir-0001/**/*.bin", "dir-0001/file-0001.bin" => matches MatchResult::Whitelist ; "t3333689486" )]
    #[test_case("", "!dir-0001/**", "dir-0001/file-0001.bin" => matches MatchResult::Whitelist ; "t4259364613" )]
    #[test_case("", "!dir-000?/file-0001.bin", "dir-0001/file-0001.bin" => matches MatchResult::Whitelist ; "t3424909626" )]
    #[test_case("", "!dir-000?/*.bin", "dir-0001/file-0001.bin" => matches MatchResult::Whitelist ; "t3741545053" )]
    #[test_case("", "!dir-*/*", "dir-0001/file-0001.bin" => matches MatchResult::Whitelist ; "t1793504005" )]
    // Ignore in child dir
    #[test_case("dir-0001", "/file-0001.bin", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t1295565113" )]
    #[test_case("dir-0001", "/file-0001.*", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t4048655621" )]
    #[test_case("dir-0001", "/*.bin", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t2580936986" )]
    #[test_case("dir-0001", "/*", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t109602877" )]
    #[test_case("dir-0001", "/**/*.bin", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t112292599" )]
    #[test_case("dir-0001", "/**", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t1323958164" )]
    #[test_case("dir-0001", "/file-0001.bin", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t4225367752" )]
    #[test_case("dir-0001", "/*.bin", "dir-0001/file-0001.bin" => matches MatchResult::Ignore ; "t3478922394" )]
    // NoMatch in child_dir
    #[test_case("dir-0002", "/file-0001.bin", "dir-0001/file-0001.bin" => matches MatchResult::NoMatch ; "t345532514" )]
    #[test_case("dir-0002", "/file-0001.*", "dir-0001/file-0001.bin" => matches MatchResult::NoMatch ; "t1313276210" )]
    #[test_case("dir-0002", "/*.bin", "dir-0001/file-0001.bin" => matches MatchResult::NoMatch ; "t657078396" )]
    #[test_case("dir-0002", "/*", "dir-0001/file-0001.bin" => matches MatchResult::NoMatch ; "t2456576806" )]
    #[test_case("dir-0002", "/**/*.bin", "dir-0001/file-0001.bin" => matches MatchResult::NoMatch ; "t2629832143" )]
    #[test_case("dir-0002", "/**", "dir-0001/file-0001.bin" => matches MatchResult::NoMatch ; "t2090580478" )]
    #[test_case("dir-0002", "/file-0001.bin", "dir-0001/file-0001.bin" => matches MatchResult::NoMatch ; "t1588943529" )]
    #[test_case("dir-0002", "/*.bin", "dir-0001/file-0001.bin" => matches MatchResult::NoMatch ; "t371313784" )]
    fn test_match_result(dir: &str, contents: &str, path: &str) -> MatchResult {
        test_logging(LevelFilter::Trace);

        let root = create_directory_hierarchy(false).unwrap();
        let source_file = format!("{root}/{dir}/.gitignore");
        let path = root.as_ref().join(path).to_owned();
        let dwi =
            new_dir_with_ignores(root.to_str().unwrap(), Some(&source_file), contents).unwrap();

        dwi.check(&path)
    }

    // TODO: Patterns shouldn't have / prefix, but an appropriate PathKind
    #[test_case(true => matches Ok(_); "this is to refresh the dir for each test run")]
    // This builds a directory hierarchy to run the tests
    fn create_directory_hierarchy(force: bool) -> Result<AbsolutePath> {
        let temp_dir: PathBuf = seeded_temp_dir("xvc-walker", Some(20220615));

        if force && temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }

        if !temp_dir.exists() {
            // in parallel tests, sometimes this fail
            fs::create_dir(&temp_dir)?;
            create_directory_tree(&temp_dir, 10, 10, 1000, None)?;
            // root/dir1 may have another tree
            let level_1 = &temp_dir.join("dir-0001");
            create_directory_tree(level_1, 10, 10, 1000, None)?;
            // and another level
            let level_2 = &level_1.join("dir-0001");
            create_directory_tree(level_2, 10, 10, 1000, None)?;
        }

        Ok(AbsolutePath::from(temp_dir))
    }
}
