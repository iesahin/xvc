//! Ignore patterns for a directory and its child directories.
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use crate::{Result, Source};

use crate::pattern::{MatchResult, Pattern};

use xvc_logging::watch;

/// Complete set of ignore rules for a directory and its child directories.
#[derive(Debug, Clone)]
pub struct IgnoreRules {
    /// The root of the ignore rules.
    /// Typically this is the root directory of Git or Xvc repository.
    pub root: PathBuf,

    /// All ignore patterns collected from ignore files or specified in code.
    // FIXME: This should be RwLock instead but as [fast_glob::Glob.is_match] requires &mut self,
    // we use a simpler approach. 
    pub ignore_patterns: Arc<Mutex<Vec<Pattern>>>,

    /// All whitelist patterns collected from ignore files or specified in code
    pub whitelist_patterns: Arc<Mutex<Vec<Pattern>>>,

}

impl IgnoreRules {
    /// An empty set of ignore rules that neither ignores nor whitelists any path.
    pub fn empty(dir: &Path) -> Self {
        IgnoreRules {
            root: PathBuf::from(dir),
            ignore_patterns: Arc::new(Mutex::new(Vec::<Pattern>::new())),
            whitelist_patterns: Arc::new(Mutex::new(Vec::<Pattern>::new())),
        }
    }

    pub fn from_global_patterns(ignore_root: &Path, given: &str) -> Self {

    let mut given_patterns = Vec::<Pattern>::new();
    // Add given patterns to ignore_patterns
    for line in given.lines() {
        let pattern = Pattern::new(Source::Global, line);
        given_patterns.push(pattern);
    }
        IgnoreRules::from_patterns(given_patterns, ignore_root)
        }


/// Constructs a new `IgnoreRules` instance from a vector of patterns and a root path.
///
/// This function separates the patterns into ignore patterns and whitelist patterns
/// based on their `PatternEffect`. It then stores these patterns and the root path
/// in a new `IgnoreRules` instance.
///
/// # Arguments
///
/// * `patterns` - A vector of `Pattern` instances to be used for creating the `IgnoreRules`.
/// * `ignore_root` - A reference to the root path for the ignore rules.
///
/// # Returns
///
/// A new `IgnoreRules` instance containing the given patterns and root path.
    pub fn from_patterns(mut patterns: Vec<Pattern>, ignore_root: &Path) -> Self {
        let mut ignore_patterns = Vec::new();
        let mut whitelist_patterns = Vec::new();
        patterns.drain(0..patterns.len()).for_each(|pattern| {
            match pattern.effect {
            crate::PatternEffect::Ignore => ignore_patterns.push(pattern),
            crate::PatternEffect::Whitelist => whitelist_patterns.push(pattern),
            }
        });
        IgnoreRules {
            root: PathBuf::from(ignore_root),
            ignore_patterns: Arc::new(Mutex::new(ignore_patterns)),
            whitelist_patterns: Arc::new(Mutex::new(whitelist_patterns)),
        }
    }


    
/// Checks if a given path matches any of the whitelist or ignore patterns.
///
/// The function first checks if the path matches any of the whitelist patterns.
/// If a match is found, it returns `MatchResult::Whitelist`.
///
/// If the path does not match any of the whitelist patterns, the function then checks
/// if the path matches any of the ignore patterns. If a match is found, it returns
/// `MatchResult::Ignore`.
///
/// If the path does not match any of the whitelist or ignore patterns, the function
/// returns `MatchResult::NoMatch`.
///
/// # Arguments
///
/// * `path` - A reference to the path to check.
///
/// # Returns
///
/// * `MatchResult::Whitelist` if the path matches a whitelist pattern.
/// * `MatchResult::Ignore` if the path matches an ignore pattern.
/// * `MatchResult::NoMatch` if the path does not match any pattern.
    pub fn check(&self, path: &Path) -> MatchResult {
    let is_abs = path.is_absolute();
    watch!(is_abs);
    // strip_prefix eats the final slash, and ends_with behave differently than str, so we work
    // around here
    let path_str = path.to_string_lossy();
    watch!(path_str);
    let final_slash = path_str.ends_with('/');
    watch!(final_slash);
    
    let path = if is_abs {

        if final_slash {
            format!(
                "/{}/",
                path.strip_prefix(&self.root)
                    .expect("path must be within root")
                    .to_string_lossy()
            )
        } else {
            format!(
                "/{}",
                path.strip_prefix(&self.root)
                    .expect("path must be within root")
                    .to_string_lossy()
            )
        }
    } else {
        path_str.to_string()
    };


        { 
            let mut whitelist_patterns = self.whitelist_patterns.lock().unwrap();
                       
            for pattern in whitelist_patterns.iter_mut() {
                let glob = &mut pattern.glob;
                if glob.is_match(&path) {
                    return MatchResult::Whitelist;
                }
            }
        }

        { 
            let mut ignore_patterns = self.ignore_patterns.lock().unwrap();
            for pattern in ignore_patterns.iter_mut() {
                if pattern.glob.is_match(&path) {
                    return MatchResult::Ignore;
                }
            }
        }

        MatchResult::NoMatch
    }

/// Merges the ignore and whitelist patterns of another `IgnoreRules` instance into this one.
///
/// This function locks the ignore and whitelist patterns of both `IgnoreRules` instances,
/// drains the patterns from the other instance, and pushes them into this instance.
/// The other instance is left empty after this operation.
///
/// # Arguments
///
/// * `other` - A reference to the other `IgnoreRules` instance to merge with.
///
/// # Returns
///
/// * `Ok(())` if the merge operation was successful.
/// * `Err` if the merge operation failed.
///
/// # Panics
///
/// This function will panic if the roots of the two `IgnoreRules` instances are not equal.
    pub fn merge_with(&self, other: &IgnoreRules) -> Result<()> {
        assert_eq!(self.root, other.root);

        { 
            let mut ignore_patterns = self.ignore_patterns.lock().unwrap();
            let mut other_ignore_patterns = other.ignore_patterns.lock().unwrap();
            let len = other_ignore_patterns.len();
            other_ignore_patterns.drain(0..len).for_each(|p| ignore_patterns.push(p.into()));
        }

        { 
            let mut whitelist_patterns = self.whitelist_patterns.lock().unwrap();
            let mut other_whitelist_patterns = other.whitelist_patterns.lock().unwrap();
            let len = other_whitelist_patterns.len();
            other_whitelist_patterns.drain(0..len).for_each(|p| whitelist_patterns.push(p.into()));
                    
        }

        Ok(())
    }

    pub  fn add_patterns(&self, patterns: Vec<Pattern>) -> Result<()> {
        let other = IgnoreRules::from_patterns(patterns, &self.root);
        self.merge_with(&other)
        }
    }
