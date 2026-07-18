//! Property-based (fuzz) tests for [`IgnoreRules`], [`Pattern`] and [`Glob`].
//!
//! These tests feed randomly generated patterns, ignore file contents and paths into the
//! matching code and assert that it never panics and that a set of semantic invariants
//! holds. Failing inputs are shrunk and persisted by proptest under
//! `walker/proptest-regressions/`.

use std::path::{Path, PathBuf};

use proptest::prelude::*;
use xvc_walker::{Glob, IgnoreRules, MatchResult, Pattern, Source, glob_match};

const ROOT: &str = "/fuzz-root";

/// Builds ignore rules from `content` (as if read from a global config) and checks
/// `rel_path` against them.
fn check(content: &str, rel_path: &str) -> MatchResult {
    let rules = IgnoreRules::from_global_patterns(Path::new(ROOT), Some(".gitignore"), content);
    rules.check(&PathBuf::from(format!("{ROOT}/{rel_path}")))
}

/// A single path component: a short alphanumeric name.
fn component() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9]{0,5}"
}

/// A relative path with 1 to 4 literal components.
fn components() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec(component(), 1..5)
}

/// A glob-ish string biased towards glob metacharacters.
fn glob_like() -> impl Strategy<Value = String> {
    r#"[a-c0-1*?!/,.{}\[\]\\-]{0,15}"#
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    /// Building rules from arbitrary content and checking arbitrary paths inside the
    /// root never panics.
    #[test]
    fn arbitrary_content_and_path_do_not_panic(content in ".{0,100}", path in ".{0,50}") {
        let _ = check(&content, &path);
    }

    /// `glob_match` never panics on arbitrary pattern/path pairs.
    #[test]
    fn glob_match_does_not_panic(pattern in ".{0,50}", path in ".{0,50}") {
        let _ = glob_match(&pattern, &path);
    }

    /// `Pattern::new` never panics on arbitrary single lines.
    #[test]
    fn pattern_new_does_not_panic(line in ".{0,60}") {
        let _ = Pattern::new(Source::Global, &line);
    }

    /// A literal path used as an ignore pattern always ignores that very path.
    #[test]
    fn literal_path_pattern_ignores_the_path(comps in components()) {
        let path = comps.join("/");
        prop_assert!(
            matches!(check(&path, &path), MatchResult::Ignore),
            "pattern {path:?} must ignore path {path:?}"
        );
    }

    /// A whitelist pattern for the same path always wins over an ignore pattern,
    /// regardless of the order of the lines.
    #[test]
    fn whitelist_overrides_ignore(comps in components(), whitelist_first in any::<bool>()) {
        let path = comps.join("/");
        let content = if whitelist_first {
            format!("!{path}\n{path}")
        } else {
            format!("{path}\n!{path}")
        };
        prop_assert!(
            matches!(check(&content, &path), MatchResult::Whitelist),
            "whitelist for {path:?} must override the ignore line"
        );
    }

    /// `**` ignores every path.
    #[test]
    fn double_star_ignores_everything(comps in components()) {
        let path = comps.join("/");
        prop_assert!(
            matches!(check("**", &path), MatchResult::Ignore),
            "** must ignore {path:?}"
        );
    }

    /// Empty ignore content matches nothing.
    #[test]
    fn empty_content_matches_nothing(comps in components()) {
        let path = comps.join("/");
        prop_assert!(matches!(check("", &path), MatchResult::NoMatch));
    }

    /// A single-component pattern (no slashes) matches anywhere: it ignores a path if
    /// and only if the path's last component equals the pattern.
    #[test]
    fn bare_name_pattern_matches_by_last_component(name in component(), comps in components()) {
        let path = comps.join("/");
        let expected_ignore = comps.last().unwrap() == &name;
        let result = check(&name, &path);
        if expected_ignore {
            prop_assert!(
                matches!(result, MatchResult::Ignore),
                "pattern {name:?} must ignore {path:?}"
            );
        } else {
            prop_assert!(
                matches!(result, MatchResult::NoMatch),
                "pattern {name:?} must not match {path:?} (got {result:?})"
            );
        }
    }

    /// A directory pattern (`name/`) ignores a path if and only if `name` appears as a
    /// non-final component (i.e. the path is inside a directory with that name).
    #[test]
    fn dir_pattern_matches_paths_under_dir(name in component(), comps in components()) {
        let path = comps.join("/");
        let expected_ignore = comps[..comps.len() - 1].contains(&name);
        let result = check(&format!("{name}/"), &path);
        if expected_ignore {
            prop_assert!(
                matches!(result, MatchResult::Ignore),
                "pattern {name:?}/ must ignore {path:?}"
            );
        } else {
            prop_assert!(
                matches!(result, MatchResult::NoMatch),
                "pattern {name:?}/ must not match file path {path:?} (got {result:?})"
            );
        }
    }

    /// The multi-pattern `Glob` matcher is equivalent to the disjunction of
    /// `glob_match` over its patterns.
    #[test]
    fn glob_is_or_of_glob_match(patterns in prop::collection::vec(glob_like(), 0..6), path in glob_like()) {
        let mut glob = Glob::default();
        let mut added = Vec::new();
        for pattern in &patterns {
            if glob.add(pattern) {
                added.push(pattern.clone());
            }
        }
        let expected = added.iter().any(|pattern| glob_match(pattern, &path));
        prop_assert_eq!(
            glob.is_match(&path),
            expected,
            "Glob({:?}) vs glob_match disagree on {:?}",
            added,
            &path
        );
    }

    /// `Glob::new` accepts exactly the patterns `Glob::add` accepts, and `Glob` never
    /// panics while adding or matching arbitrary strings.
    #[test]
    fn glob_add_and_new_agree(pattern in ".{0,30}", path in ".{0,30}") {
        let mut glob = Glob::default();
        let added = glob.add(&pattern);
        let new = Glob::new(&pattern);
        prop_assert_eq!(added, new.is_some());
        let _ = glob.is_match(&path);
        if let Some(mut new) = new {
            prop_assert_eq!(new.is_match(&path), glob.is_match(&path));
        }
    }
}
