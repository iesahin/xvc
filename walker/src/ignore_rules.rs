//! Ignore patterns for a directory and its child directories.
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use globset::GlobSet;
use itertools::Itertools;
use crate::{build_globset, content_to_patterns, GlobPattern, PatternEffect, Result};

use xvc_logging::watch;
use crate::PEAK_ALLOC;
/// Complete set of ignore rules for a directory and its child directories.
#[derive(Debug, Clone)]
pub struct IgnoreRules {
    /// The root of the ignore rules.
    /// Typically this is the root directory of Git or Xvc repository.
    pub root: PathBuf,

    /// All ignore patterns collected from ignore files or specified in code.
    pub ignore_patterns: Arc<RwLock<Vec<GlobPattern>>>,

    /// All whitelist patterns collected from ignore files or specified in code
    pub whitelist_patterns: Arc<RwLock<Vec<GlobPattern>>>,

    /// Compiled [GlobSet] for whitelisted paths.
    pub whitelist_set: Arc<RwLock<GlobSet>>,

    /// Compiled [GlobSet] for ignored paths.
    pub ignore_set: Arc<RwLock<GlobSet>>,
}

impl IgnoreRules {
    /// An empty set of ignore rules that neither ignores nor whitelists any path.
    pub fn empty(dir: &Path) -> Self {
        IgnoreRules {
            root: PathBuf::from(dir),
            ignore_patterns: Arc::new(RwLock::new(Vec::<GlobPattern>::new())),
            whitelist_patterns: Arc::new(RwLock::new(Vec::<GlobPattern>::new())),
            ignore_set: Arc::new(RwLock::new(GlobSet::empty())),
            whitelist_set: Arc::new(RwLock::new(GlobSet::empty())),
        }
    }

    /// Compiles patterns as [Source::Global] and initializes the elements.
    pub fn try_from_patterns(root: &Path, patterns: &str) -> Result<Self> {

    watch!(PEAK_ALLOC.current_usage_as_mb());
        let patterns = content_to_patterns(root, None, patterns)
            .into_iter()
            .map(|pat_res_g| pat_res_g.map(|res_g| res_g.unwrap()))
            .collect();
    watch!(PEAK_ALLOC.current_usage_as_mb());
        let mut initialized = Self::empty(&PathBuf::from(root));

    watch!(PEAK_ALLOC.current_usage_as_mb());
        initialized.update(patterns)?;
        Ok(initialized)
    }

    /// Adds `new_patterns` to the list of patterns and recompiles ignore and
    /// whitelist [GlobSet]s.
    pub fn update(&mut self, new_patterns: Vec<GlobPattern>) -> Result<()> {
    watch!(PEAK_ALLOC.current_usage_as_mb());
        let (new_ignore_patterns, new_whitelist_patterns): (Vec<_>, Vec<_>) = new_patterns
            .into_iter()
            .partition(|p| matches!(p.effect, PatternEffect::Ignore));
    watch!(PEAK_ALLOC.current_usage_as_mb());
        self.update_ignore(&new_ignore_patterns)?;
    watch!(PEAK_ALLOC.current_usage_as_mb());
        self.update_whitelist(&new_whitelist_patterns)?;
    watch!(PEAK_ALLOC.current_usage_as_mb());
        Ok(())
    }

    /// Merge with other ignore rules, extending this one's patterns and rebuilding glob sets.
    pub fn merge_with(&mut self, other: &IgnoreRules) -> Result<()> {
        assert_eq!(self.root, other.root);
    watch!(PEAK_ALLOC.current_usage_as_mb());

        self.update_ignore(&other.ignore_patterns.read().unwrap())?;
    watch!(PEAK_ALLOC.current_usage_as_mb());
        self.update_whitelist(&other.whitelist_patterns.read().unwrap())?;
    watch!(PEAK_ALLOC.current_usage_as_mb());
        Ok(())
    }

    fn update_whitelist(&mut self, new_whitelist_patterns: &[GlobPattern]) -> Result<()> {
    watch!(PEAK_ALLOC.current_usage_as_mb());
        assert!(new_whitelist_patterns
            .iter()
            .all(|p| matches!(p.effect, PatternEffect::Whitelist)));
    watch!(PEAK_ALLOC.current_usage_as_mb());
        {
            let mut whitelist_patterns = self.whitelist_patterns.write()?;
    watch!(PEAK_ALLOC.current_usage_as_mb());

            *whitelist_patterns = whitelist_patterns
                .iter()
                .chain(new_whitelist_patterns.iter())
                .unique()
                .cloned()
                .collect();
        }
    watch!(PEAK_ALLOC.current_usage_as_mb());

        {
    watch!(PEAK_ALLOC.current_usage_as_mb());
            let whitelist_globs = self
                .whitelist_patterns
                .read()?
                .iter()
                .map(|g| g.pattern.clone())
                .collect();
    watch!(PEAK_ALLOC.current_usage_as_mb());
            let whitelist_set = build_globset(whitelist_globs)?;
    watch!(PEAK_ALLOC.current_usage_as_mb());
            *self.whitelist_set.write()? = whitelist_set;
        }

    watch!(PEAK_ALLOC.current_usage_as_mb());
        Ok(())
    }
    fn update_ignore(&mut self, new_ignore_patterns: &[GlobPattern]) -> Result<()> {
    watch!(PEAK_ALLOC.current_usage_as_mb());
        assert!(new_ignore_patterns
            .iter()
            .all(|p| matches!(p.effect, PatternEffect::Ignore)));
    watch!(PEAK_ALLOC.current_usage_as_mb());
        {
            let mut ignore_patterns = self.ignore_patterns.write()?;
    watch!(PEAK_ALLOC.current_usage_as_mb());

            *ignore_patterns = ignore_patterns
                .iter()
                .chain(new_ignore_patterns.iter())
                .unique()
                .cloned()
                .collect();
        }
    watch!(PEAK_ALLOC.current_usage_as_mb());

        {
            let ignore_globs = self
                .ignore_patterns
                .read()?
                .iter()
                .map(|g| g.pattern.clone())
                .collect();
            let ignore_set = build_globset(ignore_globs)?;
            *self.ignore_set.write()? = ignore_set;
    watch!(PEAK_ALLOC.current_usage_as_mb());
        }
    watch!(PEAK_ALLOC.current_usage_as_mb());

        Ok(())
    }
}
