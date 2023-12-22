use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use globset::Glob;
use globset::GlobSet;
use itertools::Itertools;
use xvc_logging::watch;

use crate::content_to_patterns;
use crate::GlobPattern;
use crate::PatternEffect;
use crate::Result;
use crate::{build_globset, merge_pattern_lists};

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
        let patterns = content_to_patterns(root, None, patterns)
            .into_iter()
            .map(|pat_res_g| pat_res_g.map(|res_g| res_g.unwrap()))
            .collect();
        let mut initialized = Self::empty(&PathBuf::from(root));

        initialized.update(patterns)?;
        Ok(initialized)
    }

    pub fn merge_with(&mut self, other: &IgnoreRules) -> Result<()> {
        assert_eq!(self.root, other.root);

        let mut ignore_patterns = self.ignore_patterns.write()?;
        let other_ignore_patterns = other.ignore_patterns.read()?;

        *ignore_patterns = ignore_patterns
            .iter()
            .chain(other_ignore_patterns.iter())
            .unique()
            .cloned()
            .collect();
        Ok(())
    }

    /// Adds `new_patterns` to the list of patterns and recompiles ignore and
    /// whitelist [GlobSet]s.
    pub fn update(&mut self, new_patterns: Vec<GlobPattern>) -> Result<()> {
        watch!("Before ignore rules update");
        watch!(&self.ignore_set);
        watch!(&self.whitelist_set);
        let (new_ignore_patterns, new_whitelist_patterns): (Vec<_>, Vec<_>) = new_patterns
            .into_iter()
            .partition(|p| matches!(p.effect, PatternEffect::Ignore));
        if !new_ignore_patterns.is_empty() {
            self.ignore_patterns.write()?.extend(new_ignore_patterns);
            let ignore_globs = self
                .ignore_patterns
                .read()?
                .iter()
                .map(|g| g.pattern.clone())
                .collect::<Vec<Glob>>();
            {
                let mut ignore_set = self.ignore_set.write()?;
                *ignore_set = build_globset(ignore_globs)?;
            }
        }

        if !new_whitelist_patterns.is_empty() {
            self.whitelist_patterns
                .write()?
                .extend(new_whitelist_patterns);
            let whitelist_globs = self
                .ignore_patterns
                .read()?
                .iter()
                .map(|g| g.pattern.clone())
                .collect::<Vec<Glob>>();
            {
                let mut whitelist_set = self.whitelist_set.write()?;
                *whitelist_set = build_globset(whitelist_globs)?;
            }
        }
        watch!("After ignore rules update");
        watch!(&self.ignore_set.read());
        watch!(&self.whitelist_set.read());

        Ok(())
    }

    /// Creates a new IgnoreRules object with the specified list of [GlobPattern]s.
    /// It returns [Error::GlobError] if there are malformed globs in any of the files.
    pub fn new(root: &Path, patterns: Vec<GlobPattern>) -> Result<Self> {
        let patterns: Vec<GlobPattern> = patterns.into_iter().unique().collect();
        let (ignore_patterns, whitelist_patterns): (Vec<_>, Vec<_>) = patterns
            .into_iter()
            .partition(|p| matches!(p.effect, PatternEffect::Ignore));

        let ignore_globs: Vec<Glob> = ignore_patterns.iter().map(|p| p.pattern.clone()).collect();

        let ignore_set = build_globset(ignore_globs)?;

        let whitelist_globs: Vec<Glob> = whitelist_patterns
            .iter()
            .map(|p| p.pattern.clone())
            .collect();

        let whitelist_set = build_globset(whitelist_globs)?;

        Ok(IgnoreRules {
            root: root.to_path_buf(),
            ignore_patterns: Arc::new(RwLock::new(ignore_patterns)),
            whitelist_patterns: Arc::new(RwLock::new(whitelist_patterns)),
            whitelist_set: Arc::new(RwLock::new(whitelist_set)),
            ignore_set: Arc::new(RwLock::new(ignore_set)),
        })
    }
}
