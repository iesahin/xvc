use std::path::{Path, PathBuf};

use globset::Glob;
use globset::GlobSet;
use itertools::Itertools;

use crate::build_globset;
use crate::content_to_patterns;
use crate::GlobPattern;
use crate::PatternEffect;
use crate::Result;

/// Complete set of ignore rules for a directory and its child directories.
#[derive(Debug, Clone)]
pub struct IgnoreRules {
    /// The root of the ignore rules.
    /// Typically this is the root directory of Git or Xvc repository.
    pub root: PathBuf,
    /// All patterns collected from ignore files or specified in code.
    pub patterns: Vec<GlobPattern>,
    /// Compiled [GlobSet] for whitelisted paths.
    pub whitelist_set: GlobSet,
    /// Compiled [GlobSet] for ignored paths.
    pub ignore_set: GlobSet,
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
