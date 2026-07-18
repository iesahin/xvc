//! Multi-pattern glob matcher.
//!
//! `fast-glob` 1.0 removed the `Glob` struct that earlier versions (0.3) provided for
//! multi-pattern matching. This module reimplements it on top of [`fast_glob::glob_match`]
//! with the same API (`new`, `add`, `is_match`), so downstream crates keep working
//! unchanged.
//!
//! The 0.3 implementation joined all added patterns into a single brace group
//! (`{pattern1,pattern2,...}`) and matched with brace expansion. Since `glob_match`
//! supports brace expansion natively now, matching each pattern separately is
//! equivalent: the matcher returns true if any of the added patterns match.

use fast_glob::glob_match;

/// Maximum brace nesting depth supported by `fast_glob::glob_match`.
/// Patterns nested deeper than this fail to match, so they are rejected in [`Glob::add`].
const MAX_BRACE_NESTING: usize = 10;

/// `Glob` represents a glob pattern matcher with support for multi-pattern matching.
#[derive(Debug, Clone, Default)]
pub struct Glob {
    patterns: Vec<String>,
}

impl Glob {
    /// Creates a new `Glob` instance from a given glob pattern.
    ///
    /// Returns `Some(Glob)` if the pattern is valid, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use xvc_walker::Glob;
    ///
    /// let glob = Glob::new("*.txt");
    /// assert!(glob.is_some());
    /// ```
    pub fn new(glob: &str) -> Option<Self> {
        let mut value = Glob::default();
        if value.add(glob) { Some(value) } else { None }
    }

    /// Adds a new glob pattern to match against.
    ///
    /// Returns `true` if the pattern was successfully added, `false` if the pattern is
    /// invalid (unbalanced braces or brackets, or braces nested deeper than 10 levels).
    ///
    /// # Example
    ///
    /// ```
    /// use xvc_walker::Glob;
    ///
    /// let mut glob = Glob::default();
    /// assert!(glob.add("*.txt"));
    /// assert!(!glob.add("{unbalanced"));
    /// ```
    pub fn add(&mut self, glob: &str) -> bool {
        if is_valid_glob(glob) {
            self.patterns.push(glob.to_string());
            true
        } else {
            false
        }
    }

    /// Checks if any of the glob patterns matches the given path.
    ///
    /// Returns `true` if a match is found, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use xvc_walker::Glob;
    ///
    /// let mut glob = Glob::new("*.txt").unwrap();
    /// assert!(glob.is_match("file.txt"));
    /// ```
    pub fn is_match(&mut self, path: &str) -> bool {
        self.patterns
            .iter()
            .any(|pattern| glob_match(pattern, path))
    }

    /// Returns the patterns added to this matcher.
    pub fn patterns(&self) -> &[String] {
        &self.patterns
    }
}

/// Validates brace and bracket structure of a glob pattern, mirroring the checks the
/// `Glob` struct in fast-glob 0.3 performed in `Pattern::parse`: braces must be
/// balanced, brackets must be closed, and nesting must not exceed
/// [`MAX_BRACE_NESTING`]. Escaped characters and characters inside brackets are not
/// structural.
fn is_valid_glob(glob: &str) -> bool {
    let glob = glob.as_bytes();
    let mut depth = 0usize;
    let mut in_brackets = false;
    let mut current = 0usize;

    while current < glob.len() {
        match glob[current] {
            b'\\' => current += 1,
            b']' if in_brackets => in_brackets = false,
            b'[' if !in_brackets => in_brackets = true,
            b'}' if !in_brackets && depth > 0 => depth -= 1,
            b'{' if !in_brackets => {
                depth += 1;
                if depth > MAX_BRACE_NESTING {
                    return false;
                }
            }
            _ => {}
        }
        current += 1;
    }

    depth == 0 && !in_brackets
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("*.txt" => true; "simple star")]
    #[test_case("**/*.txt" => true; "globstar")]
    #[test_case("{a,b}/*.txt" => true; "braces")]
    #[test_case("[ab].txt" => true; "brackets")]
    #[test_case("a{b" => false; "unbalanced open brace")]
    #[test_case("a[b" => false; "unclosed bracket")]
    #[test_case("a}b" => true; "stray close brace is literal")]
    #[test_case("a[{]b}" => true; "brace inside brackets is not structural")]
    #[test_case("a[{]b{" => false; "brace after brackets is structural")]
    #[test_case(r"a\{b" => true; "escaped brace")]
    #[test_case("{{{{{{{{{{a}}}}}}}}}}" => true; "ten levels of nesting")]
    #[test_case("{{{{{{{{{{{a}}}}}}}}}}}" => false; "eleven levels of nesting")]
    fn test_is_valid_glob(glob: &str) -> bool {
        is_valid_glob(glob)
    }

    #[test]
    fn test_new_and_is_match() {
        let mut glob = Glob::new("*.txt").unwrap();
        assert!(glob.is_match("file.txt"));
        assert!(!glob.is_match("dir/file.txt"));
        assert!(!glob.is_match("file.bin"));

        assert!(Glob::new("{unbalanced").is_none());
    }

    #[test]
    fn test_multi_pattern_match() {
        let mut glob = Glob::default();
        assert!(glob.add("*.txt"));
        assert!(glob.add("dir/**"));
        assert!(glob.add("data-????.bin"));

        assert!(glob.is_match("file.txt"));
        assert!(glob.is_match("dir/sub/file.bin"));
        assert!(glob.is_match("data-0001.bin"));
        assert!(!glob.is_match("other.bin"));
    }

    #[test]
    fn test_empty_glob_matches_nothing() {
        let mut glob = Glob::default();
        assert!(!glob.is_match("file.txt"));
        assert!(!glob.is_match(""));
    }

    #[test]
    fn test_invalid_add_does_not_change_matcher() {
        let mut glob = Glob::new("*.txt").unwrap();
        assert!(!glob.add("{a,b"));
        assert!(glob.is_match("file.txt"));
        assert!(!glob.is_match("a"));
        assert_eq!(glob.patterns().len(), 1);
    }

    #[test]
    fn test_brace_expansion_in_patterns() {
        let mut glob = Glob::new("some/**/{the,crazy}/?*.{png,txt}").unwrap();
        assert!(glob.is_match("some/a/bigger/path/to/the/crazy/needle.txt"));
        assert!(!glob.is_match("some/a/bigger/path/to/the/calm/needle.txt"));
    }
}
