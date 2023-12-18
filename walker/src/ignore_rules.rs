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
        let mut initialized = Self::empty(&PathBuf::from(root));

        initialized.update(patterns)?;
        Ok(initialized)
    }

    /// Adds `new_patterns` to the list of patterns and recompiles ignore and
    /// whitelist [GlobSet]s.
    pub fn update(&mut self, new_patterns: Vec<GlobPattern>) -> Result<()> {
        let (new_ignore_patterns, new_whitelist_patterns): (Vec<_>, Vec<_>) = new_patterns
            .into_iter()
            .partition(|p| matches!(p.effect, PatternEffect::Ignore));
        let (mut current_ignore_patterns, mut current_whitelist_patterns): (Vec<_>, Vec<_>) = self
            .patterns
            .clone()
            .into_iter()
            .partition(|p| matches!(p.effect, PatternEffect::Ignore));

        if !new_ignore_patterns.is_empty() {
            current_ignore_patterns.extend(new_ignore_patterns);
            let current_ignore_globs = current_ignore_patterns
                .iter()
                .map(|p| p.pattern.clone())
                .collect();
            self.ignore_set = build_globset(current_ignore_globs)?
        }

        if !new_whitelist_patterns.is_empty() {
            current_whitelist_patterns.extend(new_whitelist_patterns);

            let current_whitelist_globs: Vec<Glob> = current_whitelist_patterns
                .iter()
                .map(|p| p.pattern.clone())
                .collect();

            self.whitelist_set = build_globset(current_whitelist_globs)?
        }

        Ok(())
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

        let ignore_set = build_globset(ignore_patterns)?;

        let whitelist_patterns: Vec<Glob> = patterns
            .iter()
            .filter(|p| p.effect == PatternEffect::Whitelist)
            .map(|p| p.pattern.clone())
            .collect();

        let whitelist_set = build_globset(whitelist_patterns)?;

        Ok(IgnoreRules {
            root: root.to_path_buf(),
            patterns,
            whitelist_set,
            ignore_set,
        })
    }
}
