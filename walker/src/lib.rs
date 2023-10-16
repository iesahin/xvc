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
pub mod notify;

pub use abspath::AbsolutePath;
pub use error::{Error, Result};
use itertools::Itertools;
pub use std::hash::Hash;
use xvc_logging::watch;

use crossbeam_channel::Sender;
use log::warn;
// use glob::{MatchOptions, Pattern, PatternError};
pub use globset::{self, Glob, GlobSet, GlobSetBuilder};
use std::{
    ffi::OsString,
    fmt::Debug,
    fs::{self, Metadata},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context};

/// Combine a path and its metadata in a single struct
#[derive(Debug, Clone)]
pub struct PathMetadata {
    /// path
    pub path: PathBuf,
    /// metadata
    pub metadata: Metadata,
}

/// Show whether a path matches to a glob rule
#[derive(Debug, Clone)]
pub enum MatchResult {
    /// There is no match between glob(s) and path
    NoMatch,
    /// Path matches to ignored glob(s)
    Ignore,
    /// Path matches to whitelisted glob(s)
    Whitelist,
}

/// Is the pattern matches anywhere or only relative to a directory?
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PatternRelativity {
    /// Match the path regardless of the directory prefix
    Anywhere,
    /// Match the path if it only starts with `directory`
    RelativeTo {
        /// The directory that the pattern must have as prefix to be considered a match
        directory: String,
    },
}

/// Is the path only a directory, or could it be directory or file?
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PathKind {
    /// Path matches to directory or file
    Any,
    /// Path matches only to directory
    Directory,
}

/// Is this pattern a ignore or whitelist patter?
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PatternEffect {
    /// This is an ignore pattern
    Ignore,
    /// This is a whitelist pattern
    Whitelist,
}

/// Do we get this pattern from a file (.gitignore, .xvcignore, ...) or specify it directly in
/// code?
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Source {
    /// Pattern is obtained from file
    File {
        /// Path of the pattern file
        path: PathBuf,
        /// (1-based) line number the pattern retrieved
        line: usize,
    },
    /// Pattern is globally defined in code
    Global,
}

/// Pattern is generic and could be an instance of String, Glob, Regex or any other object.
/// The type is evolved by compiling.
/// A pattern can start its life as `Pattern<String>` and can be compiled into `Pattern<Glob>` or
/// `Pattern<Regex>`.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Pattern<T>
where
    T: PartialEq + Hash,
{
    /// The pattern type
    pattern: T,
    /// The original string that defines the pattern
    original: String,
    /// Where did we get this pattern?
    source: Source,
    /// Is this ignore or whitelist pattern?
    effect: PatternEffect,
    /// Does it have an implied prefix?
    relativity: PatternRelativity,
    /// Is the path a directory or anything?
    path_kind: PathKind,
}

impl<T: PartialEq + Hash> Pattern<T> {
    /// Runs a function (like `compile`) on `pattern` to get a new pattern.
    pub fn map<U, F>(self, f: F) -> Pattern<U>
    where
        U: PartialEq + Hash,
        F: FnOnce(T) -> U,
    {
        Pattern::<U> {
            pattern: f(self.pattern),
            original: self.original,
            source: self.source,
            effect: self.effect,
            relativity: self.relativity,
            path_kind: self.path_kind,
        }
    }
}

impl<T: PartialEq + Hash> Pattern<Result<T>> {
    /// Convert from `Pattern<Result<T>>` to `Result<Pattern<Ok>>` to get the result from
    /// [Self::map]
    fn transpose(self) -> Result<Pattern<T>> {
        match self.pattern {
            Ok(p) => Ok(Pattern::<T> {
                pattern: p,
                original: self.original,
                source: self.source,
                effect: self.effect,
                relativity: self.relativity,
                path_kind: self.path_kind,
            }),
            Err(e) => Err(e),
        }
    }
}

/// One of the concrete types that can represent a pattern.
type GlobPattern = Pattern<Glob>;

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
            ignore_filename: Some(".gitignore".to_owned()),
            include_dirs: true,
        }
    }

    /// Instantiate a Xvc repository walker that uses `.xvcignore` as ignore file name and includes
    /// directories in results.
    pub fn xvcignore() -> Self {
        Self {
            ignore_filename: Some(".xvcignore".to_owned()),
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

/// Complete set of ignore rules for a directory and its child directories.
#[derive(Debug, Clone)]
pub struct IgnoreRules {
    /// The root of the ignore rules.
    /// Typically this is the root directory of Git or Xvc repository.
    root: PathBuf,
    /// All patterns collected from ignore files or specified in code.
    patterns: Vec<GlobPattern>,
    /// Compiled [GlobSet] for whitelisted paths.
    whitelist_set: GlobSet,
    /// Compiled [GlobSet] for ignored paths.
    ignore_set: GlobSet,
}

impl IgnoreRules {
    /// An empty set of ignore rules that neither ignores nor whitelists any path.
    pub fn empty(dir: &Path) -> Self {
        IgnoreRules {
            root: PathBuf::from(dir),
            patterns: Vec::<GlobPattern>::new(),
            ignore_set: GlobSet::empty(),
            whitelist_set: GlobSet::empty(),
        }
    }

    /// Compiles patterns as [Source::Global] and initializes the elements.
    pub fn try_from_patterns(root: &Path, patterns: &str) -> Result<Self> {
        let patterns = content_to_patterns(root, None, patterns)
            .into_iter()
            .map(|pat_res_g| pat_res_g.map(|res_g| res_g.unwrap()))
            .collect();
        let empty = Self::empty(&PathBuf::from(root));

        let initialized = empty.update(patterns)?;
        Ok(initialized)
    }

    /// Consumes `self`, adds `new_patterns` to the list of patterns and recompiles ignore and
    /// whitelist [GlobSet]s.
    pub fn update(self, new_patterns: Vec<GlobPattern>) -> Result<Self> {
        let patterns: Vec<GlobPattern> = self
            .patterns
            .into_iter()
            .chain(new_patterns.iter().cloned())
            .unique()
            .collect();

        let ignore_set = if new_patterns
            .iter()
            .any(|p| p.effect == PatternEffect::Ignore)
        {
            let all_ignore_patterns: Vec<Glob> = patterns
                .iter()
                .filter(|p| p.effect == PatternEffect::Ignore)
                .map(|p| p.pattern.clone())
                .collect();

            build_globset(&all_ignore_patterns)?
        } else {
            self.ignore_set
        };

        let whitelist_set = if new_patterns
            .iter()
            .any(|p| p.effect == PatternEffect::Whitelist)
        {
            let all_whitelist_patterns: Vec<Glob> = patterns
                .iter()
                .filter(|p| p.effect == PatternEffect::Whitelist)
                .map(|p| p.pattern.clone())
                .collect();

            build_globset(&all_whitelist_patterns)?
        } else {
            self.whitelist_set
        };

        Ok(IgnoreRules {
            root: self.root,
            patterns,
            whitelist_set,
            ignore_set,
        })
    }

    /// Creates a new IgnoreRules object with the specified list of [GlobPattern]s.
    /// It returns [Error::GlobError] if there are malformed globs in any of the files.
    pub fn new(root: &Path, patterns: Vec<GlobPattern>) -> Result<Self> {
        let patterns: Vec<GlobPattern> = patterns.into_iter().unique().collect();
        let ignore_patterns: Vec<Glob> = patterns
            .iter()
            .filter(|p| p.effect == PatternEffect::Ignore)
            .map(|p| p.pattern.clone())
            .collect();

        let ignore_set = build_globset(&ignore_patterns)?;

        let whitelist_patterns: Vec<Glob> = patterns
            .iter()
            .filter(|p| p.effect == PatternEffect::Whitelist)
            .map(|p| p.pattern.clone())
            .collect();

        let whitelist_set = build_globset(&whitelist_patterns)?;

        Ok(IgnoreRules {
            root: root.to_path_buf(),
            patterns,
            whitelist_set,
            ignore_set,
        })
    }
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
                    "Error getting metadata {:?} {}",
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

/// Walk all child paths under `dir` and send non-ignored paths to `path_sender`.
/// Newly found ignore rules are sent through `ignore_sender`.
/// The ignore file name (`.xvcignore`, `.gitignore`, `.ignore`, ...) is set by `walk_options`.
///
/// It lists elements of a file, then creates a new crossbeam scope for each child directory and
/// calls itself recursively. It may not be feasible for small directories to create threads.
pub fn walk_parallel(
    ignore_rules: IgnoreRules,
    dir: &Path,
    walk_options: WalkOptions,
    path_sender: Sender<Result<PathMetadata>>,
    ignore_sender: Sender<Result<IgnoreRules>>,
) -> Result<()> {
    let child_paths: Vec<PathMetadata> = directory_list(dir)?
        .into_iter()
        .filter_map(|pm_res| match pm_res {
            Ok(pm) => Some(pm),
            Err(e) => {
                path_sender
                    .send(Err(e))
                    .expect("Channel error in walk_parallel");
                None
            }
        })
        .collect();

    let dir_with_ignores = if let Some(ignore_filename) = walk_options.ignore_filename.clone() {
        let ignore_filename = OsString::from(ignore_filename);
        if let Some(ignore_path_metadata) = child_paths
            .iter()
            .find(|pm| pm.path.file_name() == Some(&ignore_filename))
        {
            let ignore_path = dir.join(&ignore_path_metadata.path);
            let new_patterns = clear_glob_errors(
                &path_sender,
                patterns_from_file(&ignore_rules.root, &ignore_path)?,
            );
            watch!(new_patterns);
            let ignore_rules = ignore_rules.update(new_patterns)?;
            watch!(ignore_rules);
            ignore_sender.send(Ok(ignore_rules.clone()))?;
            ignore_rules
        } else {
            ignore_rules
        }
    } else {
        ignore_rules
    };

    let mut child_dirs = Vec::<PathMetadata>::new();
    watch!(child_paths);

    for child_path in child_paths {
        match check_ignore(&dir_with_ignores, child_path.path.as_ref()) {
            MatchResult::NoMatch | MatchResult::Whitelist => {
                watch!(child_path.path);
                if child_path.metadata.is_dir() {
                    if walk_options.include_dirs {
                        path_sender.send(Ok(child_path.clone()))?;
                    }
                    child_dirs.push(child_path);
                } else {
                    path_sender.send(Ok(child_path.clone()))?;
                }
            }
            // We can return anyhow! error here to notice the user that the path is ignored
            MatchResult::Ignore => {
                watch!(child_path.path);
            }
        }
    }

    crossbeam::scope(|s| {
        for child_dir in child_dirs {
            let dwi = dir_with_ignores.clone();
            let walk_options = walk_options.clone();
            let path_sender = path_sender.clone();
            let ignore_sender = ignore_sender.clone();
            s.spawn(move |_| {
                watch!(dwi);
                watch!(walk_options);
                watch!(path_sender);
                watch!(ignore_sender);
                walk_parallel(
                    dwi,
                    &child_dir.path,
                    walk_options,
                    path_sender,
                    ignore_sender,
                )
            });
        }
    })
    .expect("Error in crossbeam scope in walk_parallel");

    watch!("End of walk_parallel");

    Ok(())
}

/// Walk `dir` with `walk_options`, with the given _initial_ `ignore_rules`.
/// Note that ignore rules are expanded with the rules given in the `ignore_filename` in
/// `walk_options`.
/// The result is added to given `res_paths` to reduce the number of memory inits for vec.
///
/// It collects all [`PathMetadata`] of the child paths.
/// Filters paths with the rules found in child directories and the given `ignore_rules`.
pub fn walk_serial(
    ignore_rules: IgnoreRules,
    dir: &Path,
    walk_options: &WalkOptions,
    res_paths: &mut Vec<Result<PathMetadata>>,
) -> Result<IgnoreRules> {
    let child_paths: Vec<PathMetadata> = directory_list(dir)?
        .into_iter()
        .filter_map(|pm_res| match pm_res {
            Ok(pm) => Some(pm),
            Err(e) => {
                res_paths.push(Err(e));
                None
            }
        })
        .collect();

    let dir_with_ignores = if let Some(ignore_filename) = walk_options.ignore_filename.clone() {
        let ignore_filename = OsString::from(ignore_filename);
        if let Some(ignore_path_metadata) = child_paths
            .iter()
            .find(|pm| pm.path.file_name() == Some(&ignore_filename))
        {
            let ignore_path = dir.join(&ignore_path_metadata.path);
            let new_patterns: Vec<GlobPattern> =
                patterns_from_file(&ignore_rules.root, &ignore_path)?
                    .into_iter()
                    .filter_map(|res_p| match res_p.pattern {
                        Ok(_) => Some(res_p.map(|p| p.unwrap())),
                        Err(e) => {
                            res_paths.push(Err(e));
                            None
                        }
                    })
                    .collect();

            ignore_rules.update(new_patterns)?
        } else {
            ignore_rules
        }
    } else {
        ignore_rules
    };

    let mut child_dirs = Vec::<PathMetadata>::new();

    for child_path in child_paths {
        match check_ignore(&dir_with_ignores, child_path.path.as_ref()) {
            MatchResult::NoMatch | MatchResult::Whitelist => {
                if child_path.metadata.is_dir() {
                    if walk_options.include_dirs {
                        res_paths.push(Ok(child_path.clone()));
                    }
                    child_dirs.push(child_path);
                } else {
                    res_paths.push(Ok(child_path.clone()));
                }
            }
            // We can return anyhow! error here to notice the user that the path is ignored
            MatchResult::Ignore => {}
        }
    }

    let mut child_ignores = vec![dir_with_ignores.clone()];

    for child_dir in child_dirs {
        let child_ignore = walk_serial(
            dir_with_ignores.clone(),
            &child_dir.path,
            walk_options,
            res_paths,
        )?;
        child_ignores.push(child_ignore);
    }

    let merged = merge_ignores(&child_ignores)?;

    Ok(merged)
}

/// merge ignore rules in a single set of ignore rules.
///
/// - if the list is empty, it's an error.
/// - the `root` of the result is the `root` of the first element.
/// - it collects all rules in a single `Vec<GlobPattern>` and recompiles `whitelist` and `ignore`
/// globsets.
pub fn merge_ignores(ignore_rules: &Vec<IgnoreRules>) -> Result<IgnoreRules> {
    if ignore_rules.is_empty() {
        Err(Error::CannotMergeEmptyIgnoreRules)
    } else if ignore_rules.len() == 1 {
        Ok(ignore_rules[0].clone())
    } else {
        let merged = Vec::<GlobPattern>::new();
        let root = ignore_rules[0].root.clone();
        let patterns = ignore_rules
            .iter()
            .fold(merged, |mut merged, ir| {
                merged.extend(ir.patterns.clone());
                merged
            })
            .into_iter()
            .unique()
            .collect();
        IgnoreRules::new(&root, patterns)
    }
}

/// Just build the ignore rules with the given directory
pub fn build_ignore_rules(
    given: IgnoreRules,
    dir: &Path,
    ignore_filename: &str,
) -> Result<IgnoreRules> {
    let elements = dir
        .read_dir()
        .map_err(|e| anyhow!("Error reading directory: {:?}, {:?}", dir, e))?;

    let mut child_dirs = Vec::<PathBuf>::new();
    let ignore_fn = OsString::from(ignore_filename);
    let ignore_root = given.root.clone();
    let mut ignore_rules = given;
    let mut new_patterns: Option<Vec<GlobPattern>> = None;

    for entry in elements {
        match entry {
            Ok(entry) => {
                if entry.path().is_dir() {
                    child_dirs.push(entry.path());
                }
                if entry.file_name() == ignore_fn && entry.path().exists() {
                    let ignore_path = entry.path();
                    new_patterns = Some(
                        patterns_from_file(&ignore_root, &ignore_path)?
                            .into_iter()
                            .filter_map(|p| match p.transpose() {
                                Ok(p) => Some(p),
                                Err(e) => {
                                    warn!("{:?}", e);
                                    None
                                }
                            })
                            .collect(),
                    );
                }
            }
            Err(e) => {
                warn!("{}", e);
            }
        }
    }

    if let Some(new_patterns) = new_patterns {
        ignore_rules = ignore_rules.update(new_patterns)?;
    }

    for child_dir in child_dirs {
        match check_ignore(&ignore_rules, &child_dir) {
            MatchResult::NoMatch | MatchResult::Whitelist => {
                ignore_rules = build_ignore_rules(ignore_rules, &child_dir, ignore_filename)?;
            }
            MatchResult::Ignore => {}
        }
    }

    Ok(ignore_rules)
}

fn clear_glob_errors(
    sender: &Sender<Result<PathMetadata>>,
    new_patterns: Vec<Pattern<Result<Glob>>>,
) -> Vec<Pattern<Glob>> {
    let new_glob_patterns: Vec<Pattern<Glob>> = new_patterns
        .into_iter()
        .filter_map(|p| match p.transpose() {
            Ok(p) => Some(p),
            Err(e) => {
                sender
                    .send(Err(Error::from(anyhow!("Error in glob pattern: {:?}", e))))
                    .expect("Error in channel");
                None
            }
        })
        .collect();
    new_glob_patterns
}

fn transform_pattern_for_glob(pattern: Pattern<String>) -> Pattern<String> {
    let anything_anywhere = |p| format!("**/{p}");
    let anything_relative = |p, directory| format!("{directory}/**/{p}");
    let directory_anywhere = |p| format!("**{p}/**");
    let directory_relative = |p, directory| format!("{directory}/**/{p}/**");

    let transformed_pattern = match (&pattern.path_kind, &pattern.relativity) {
        (PathKind::Any, PatternRelativity::Anywhere) => anything_anywhere(pattern.pattern),
        (PathKind::Any, PatternRelativity::RelativeTo { directory }) => {
            anything_relative(pattern.pattern, directory)
        }
        (PathKind::Directory, PatternRelativity::Anywhere) => directory_anywhere(pattern.pattern),
        (PathKind::Directory, PatternRelativity::RelativeTo { directory }) => {
            directory_relative(pattern.pattern, directory)
        }
    };

    Pattern {
        pattern: transformed_pattern,
        ..pattern
    }
}

fn build_globset(patterns: &[Glob]) -> Result<GlobSet> {
    let mut gs_builder = GlobSetBuilder::new();

    for p in patterns {
        gs_builder.add(p.clone());
    }
    gs_builder
        .build()
        .map_err(|e| anyhow!("Error building glob set: {:?}", e).into())
}

fn patterns_from_file(
    ignore_root: &Path,
    ignore_path: &Path,
) -> Result<Vec<Pattern<Result<Glob>>>> {
    watch!(ignore_root);
    watch!(ignore_path);
    let content = fs::read_to_string(ignore_path).with_context(|| {
        format!(
            "Cannot read file: {:?}\n
        If the file is present, it may be an encoding issue. Please check if it's UTF-8 encoded.",
            ignore_path
        )
    })?;
    watch!(&content);
    Ok(content_to_patterns(
        ignore_root,
        Some(ignore_path),
        &content,
    ))
}

/// convert a set of rules in `content` to glob patterns.
/// patterns may come from `source`.
/// the root directory of all search is in `ignore_root`.
pub fn content_to_patterns(
    ignore_root: &Path,
    source: Option<&Path>,
    content: &str,
) -> Vec<Pattern<Result<Glob>>> {
    let patterns: Vec<Pattern<Result<Glob>>> = content
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
        .map(|(line, source)| build_pattern(source, line))
        .map(transform_pattern_for_glob)
        .map(|pc| pc.map(|s| Glob::new(&s).map_err(Error::from)))
        .collect();

    patterns
}

fn build_pattern(source: Source, original: &str) -> Pattern<String> {
    let current_dir = match &source {
        Source::Global => "".to_string(),
        Source::File { path, .. } => {
            let path = path
                .parent()
                .expect("Pattern source file doesn't have parent")
                .to_string_lossy()
                .to_string();
            if path.starts_with('/') {
                path
            } else {
                format!("/{path}")
            }
        }
    };

    // if Pattern starts with ! it's whitelist, if ends with / it's dir only, if it contains
    // non final slash, it should be considered under the current dir only, otherwise it
    // matches

    let begin_exclamation = original.starts_with('!');
    let mut line = if begin_exclamation || original.starts_with(r"\!") {
        original[1..].to_owned()
    } else {
        original.to_owned()
    };

    // TODO: We should handle filenames with trailing spaces better, with regex match and removing
    // the \\ from the name
    if !line.ends_with("\\ ") {
        line = line.trim_end().to_string();
    }

    let end_slash = line.ends_with('/');
    if end_slash {
        line = line[..line.len() - 1].to_string()
    }

    let begin_slash = line.starts_with('/');
    let non_final_slash = if !line.is_empty() {
        line[..line.len() - 1].chars().any(|c| c == '/')
    } else {
        false
    };

    if begin_slash {
        line = line[1..].to_string();
    }

    let current_dir = if current_dir.ends_with('/') {
        &current_dir[..current_dir.len() - 1]
    } else {
        &current_dir
    };

    let effect = if begin_exclamation {
        PatternEffect::Whitelist
    } else {
        PatternEffect::Ignore
    };

    let path_kind = if end_slash {
        PathKind::Directory
    } else {
        PathKind::Any
    };

    let relativity = if non_final_slash {
        PatternRelativity::RelativeTo {
            directory: current_dir.to_owned(),
        }
    } else {
        PatternRelativity::Anywhere
    };

    Pattern::<String> {
        pattern: line,
        original: original.to_owned(),
        source,
        effect,
        relativity,
        path_kind,
    }
}

/// Check whether `path` is whitelisted or ignored with `ignore_rules`
pub fn check_ignore(ignore_rules: &IgnoreRules, path: &Path) -> MatchResult {
    let is_abs = path.is_absolute();
    // strip_prefix eats the final slash, and ends_with behave differently than str, so we work
    // around here
    let path_str = path.to_string_lossy();
    let final_slash = path_str.ends_with('/');

    let path = if is_abs {
        if final_slash {
            format!(
                "/{}/",
                path.strip_prefix(&ignore_rules.root)
                    .expect("path must be within root")
                    .to_string_lossy()
            )
        } else {
            format!(
                "/{}",
                path.strip_prefix(&ignore_rules.root)
                    .expect("path must be within root")
                    .to_string_lossy()
            )
        }
    } else {
        path_str.to_string()
    };

    if ignore_rules.whitelist_set.is_match(&path) {
        MatchResult::Whitelist
    } else if ignore_rules.ignore_set.is_match(&path) {
        MatchResult::Ignore
    } else {
        MatchResult::NoMatch
    }
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
        let pat = build_pattern(Source::Global, line);
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
        let pat = build_pattern(source, line);
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
        let pat = build_pattern(source, line);
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

    fn create_patterns(root: &str, dir: Option<&str>, patterns: &str) -> Vec<Pattern<Glob>> {
        content_to_patterns(Path::new(root), dir.map(Path::new), patterns)
            .into_iter()
            .map(|pat_res_g| pat_res_g.map(|res_g| res_g.unwrap()))
            .collect()
    }

    fn new_dir_with_ignores(
        root: &str,
        dir: Option<&str>,
        initial_patterns: &str,
    ) -> Result<IgnoreRules> {
        let patterns = create_patterns(root, dir, initial_patterns);
        let empty = IgnoreRules::empty(&PathBuf::from(root));

        let initialized = empty.update(patterns).unwrap();
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
        let pattern = build_pattern(source, pattern);
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
        let pattern = build_pattern(source, pattern);
        pattern.pattern
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

        check_ignore(&dwi, &path)
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
