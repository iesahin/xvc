//! Listing Git references and branches, in process via [gix].
//!
//! These feed the shell completers in [`crate::util::completer`] and have never used the `git`
//! binary.

use std::path::Path;

use crate::{Error, Result};

/// Return all tags and branches from a repository using Gix
///
/// TODO: We can add prefix listing if there is a performance issue for large repos here
pub fn gix_list_references(repo_path: &Path) -> Result<Vec<String>> {
    // We use map error because gix::discover::Error is a large struct
    let repo = gix::discover(repo_path).map_err(|e| Error::GixError {
        cause: e.to_string(),
    })?;
    let mut refs = Vec::new();

    let ref_platform = repo.references()?;
    ref_platform.all().map(|all| {
        all.for_each(|reference| {
            if let Ok(reference) = reference {
                if let Some((_, name)) = reference.name().category_and_short_name() {
                    refs.push(name.to_string());
                }
            }
        });
        Ok(refs)
    })?
}

/// List local branches in a Git repository
pub fn gix_list_branches(repo_path: &Path) -> Result<Vec<String>> {
    // We use map error because gix::discover::Error is a large struct
    let repo = gix::discover(repo_path).map_err(|e| Error::GixError {
        cause: e.to_string(),
    })?;
    let mut refs = Vec::new();

    let ref_platform = repo.references()?;
    ref_platform.local_branches().map(|all| {
        all.for_each(|reference| {
            if let Ok(reference) = reference {
                if let Some((_, name)) = reference.name().category_and_short_name() {
                    refs.push(name.to_string());
                }
            }
        });
        Ok(refs)
    })?
}
