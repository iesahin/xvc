//! Pattern describes a single line in an ignore file and its semantics
//! It is used to match a path with the given pattern
use crate::sync;
pub use error::{Error, Result};
pub use ignore_rules::IgnoreRules;
pub use std::hash::Hash;
pub use sync::{PathSync, PathSyncSingleton};

pub use crate::notify::{make_watcher, PathEvent, RecommendedWatcher};

// use glob::{MatchOptions, Pattern, PatternError};
pub use fast_glob::Glob;
use std::{fmt::Debug, path::PathBuf};

use crate::error;
use crate::ignore_rules;

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

/// Is this pattern a ignore or whitelist pattern?
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
    /// Pattern is globally defined in code
    Global,

    /// Pattern is obtained from file
    File {
        /// Path of the pattern file
        path: PathBuf,
        /// (1-based) line number the pattern retrieved
        line: usize,
    },

    /// Pattern is from CLI
    CommandLine { current_dir: PathBuf },
}

/// Pattern is generic and could be an instance of String, Glob, Regex or any other object.
/// The type is evolved by compiling.
/// A pattern can start its life as `Pattern<String>` and can be compiled into `Pattern<Glob>` or
/// `Pattern<Regex>`.
#[derive(Debug)]
pub struct Pattern {
    /// The pattern type
    pub glob: String,
    /// The original string that defines the pattern
    pub original: String,
    /// Where did we get this pattern?
    pub source: Source,
    /// Is this ignore or whitelist pattern?
    pub effect: PatternEffect,
    /// Does it have an implied prefix?
    pub relativity: PatternRelativity,
    /// Is the path a directory or anything?
    pub path_kind: PathKind,
}

impl Pattern {
    pub fn new(source: Source, original: &str) -> Self {
        let original = original.to_owned();
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
            Source::CommandLine { current_dir } => current_dir.to_string_lossy().to_string(),
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

        let glob = transform_pattern_for_glob(&line, relativity.clone(), path_kind.clone());

        Pattern {
            glob,
            original,
            source,
            effect,
            relativity,
            path_kind,
        }
    }
}

fn transform_pattern_for_glob(
    original: &str,
    relativity: PatternRelativity,
    path_kind: PathKind,
) -> String {
    let anything_anywhere = |p| format!("**/{p}");
    let anything_relative = |p, directory| format!("{directory}/**/{p}");
    let directory_anywhere = |p| format!("**/{p}/**");
    let directory_relative = |p, directory| format!("{directory}/**/{p}/**");

    let transformed_pattern = match (path_kind, relativity) {
        (PathKind::Any, PatternRelativity::Anywhere) => anything_anywhere(original),
        (PathKind::Any, PatternRelativity::RelativeTo { directory }) => {
            anything_relative(original, directory)
        }
        (PathKind::Directory, PatternRelativity::Anywhere) => directory_anywhere(original),
        (PathKind::Directory, PatternRelativity::RelativeTo { directory }) => {
            directory_relative(original, directory)
        }
    };

    transformed_pattern
}

pub fn build_pattern_list(patterns: Vec<String>, source: Source) -> Vec<Pattern> {
    patterns
        .iter()
        .map(|p| Pattern::new(source.clone(), p))
        .collect()
}
