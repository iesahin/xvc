//! Completion helpers for commands and options
use crate::{
    Result, XvcPath,
    types::xvcroot::{XvcRootInner, find_root},
};
use std::{env, ffi::OsStr, path::Path};

use clap_complete::CompletionCandidate;
use xvc_ecs::{Storable, XvcStore};

/// Return completions for all Git references starting with `current` in the current directory
/// Used in `--from-ref` option.
pub fn git_reference_completer(current: &std::ffi::OsStr) -> Vec<CompletionCandidate> {
    let current = current.to_string_lossy();
    crate::git::gix_list_references(Path::new("."))
        .map(|refs| {
            refs.iter()
                .filter_map(|r| {
                    if r.starts_with(current.as_ref()) {
                        Some(CompletionCandidate::new(r))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Return completions for all Git branches starting with `current` in the current directory
/// Used in `--to-branch` option
pub fn git_branch_completer(current: &std::ffi::OsStr) -> Vec<CompletionCandidate> {
    let current = current.to_string_lossy();
    crate::git::gix_list_branches(Path::new("."))
        .map(|refs| {
            refs.iter()
                .filter_map(|r| {
                    if r.starts_with(current.as_ref()) {
                        Some(CompletionCandidate::new(r))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// A generic function to convert [strum_macros::VariantNames] to [CompletionCandidate] values. It
/// can be used when an enum uses strum to parse string values.
pub fn strum_variants_completer<T: strum::VariantNames>(
    current: &std::ffi::OsStr,
) -> Vec<CompletionCandidate> {
    let current = current.to_string_lossy();
    let variants = T::VARIANTS;
    variants
        .iter()
        .filter_map(|v| {
            if (**v).starts_with(current.as_ref()) {
                Some(CompletionCandidate::new(v))
            } else {
                None
            }
        })
        .collect()
}

/// Returns a store to complete an attribute for a component.
///
/// It doesn't load [XvcRoot] or any configuration files. It just checks the presense of .xvc
/// directory in parent directories and loads a store from there.
///
/// Returns Err(CannotFindXvcRoot) if the root is not found. Actual completers should handle errors
/// to return empty list.
pub fn load_store_for_completion<T: Storable>(current_dir: &Path) -> Result<XvcStore<T>> {
    let xvc_root_path = find_root(current_dir)?;
    let xvc_dir = xvc_root_path.join(XvcRootInner::XVC_DIR);
    let store_root = xvc_dir.join(XvcRootInner::STORE_DIR);
    XvcStore::<T>::load_store(&store_root).map_err(|e| e.into())
}

/// Complete all XvcPath items in the store starting with prefix
pub fn xvc_path_completer(prefix: &OsStr) -> Vec<CompletionCandidate> {
    // FIXME: What should we do for Non-UTF-8 paths?
    let prefix = prefix.to_str().unwrap_or("");
    if let Ok(current_dir) = env::current_dir() {
        load_store_for_completion::<XvcPath>(&current_dir)
            .map(|xvc_path_store| {
                // FIXME: This doesn't consider current dir to filter the elements
                let filtered = xvc_path_store.filter(|_, xp| xp.starts_with_str(prefix));
                filtered
                    .iter()
                    .map(|(_, xp)| xp.to_string().into())
                    .collect()
            })
            .unwrap_or_default()
    } else {
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{HashAlgorithm, RecheckMethod, TextOrBinary};
    use std::fs;
    use std::str::FromStr;
    use xvc_ecs::XvcEntity;
    use xvc_test_helper::create_temp_dir;

    /// The candidate values, sorted so the assertions don't depend on variant declaration order.
    fn values(candidates: Vec<CompletionCandidate>) -> Vec<String> {
        let mut values: Vec<String> = candidates
            .iter()
            .map(|c| c.get_value().to_string_lossy().to_string())
            .collect();
        values.sort();
        values
    }

    #[test]
    fn strum_completer_offers_every_variant_for_an_empty_prefix() {
        assert_eq!(
            values(strum_variants_completer::<RecheckMethod>(OsStr::new(""))),
            ["copy", "hardlink", "reflink", "symlink"]
        );
        assert_eq!(
            values(strum_variants_completer::<TextOrBinary>(OsStr::new(""))),
            ["auto", "binary", "text"]
        );
    }

    #[test]
    fn strum_completer_filters_by_prefix() {
        assert_eq!(
            values(strum_variants_completer::<RecheckMethod>(OsStr::new("s"))),
            ["symlink"]
        );
        assert_eq!(
            values(strum_variants_completer::<TextOrBinary>(OsStr::new("b"))),
            ["binary"]
        );
        assert!(
            values(strum_variants_completer::<RecheckMethod>(OsStr::new(
                "no-such-method"
            )))
            .is_empty()
        );
    }

    /// A candidate the shell inserts is worthless if the command then rejects it. Strum derives
    /// `VARIANTS` and `FromStr` from the same `#[strum(...)]` attributes, but only for the
    /// *primary* serialization -- [HashAlgorithm] for instance offers `b3`, not `blake3`. This
    /// pins that whatever the completer offers is a value the parser accepts.
    #[test]
    fn strum_candidates_are_accepted_by_their_parser() {
        for value in values(strum_variants_completer::<RecheckMethod>(OsStr::new(""))) {
            RecheckMethod::from_str(&value)
                .unwrap_or_else(|e| panic!("RecheckMethod rejects completion {value}: {e}"));
        }
        for value in values(strum_variants_completer::<TextOrBinary>(OsStr::new(""))) {
            TextOrBinary::from_str(&value)
                .unwrap_or_else(|e| panic!("TextOrBinary rejects completion {value}: {e}"));
        }
        for value in values(strum_variants_completer::<HashAlgorithm>(OsStr::new(""))) {
            HashAlgorithm::from_str(&value)
                .unwrap_or_else(|e| panic!("HashAlgorithm rejects completion {value}: {e}"));
        }
    }

    /// Writes a store under `root/.xvc/store`, the way an actual repository keeps it.
    fn init_fake_repo(root: &Path, methods: &[RecheckMethod]) {
        let store_root = root
            .join(XvcRootInner::XVC_DIR)
            .join(XvcRootInner::STORE_DIR);
        fs::create_dir_all(&store_root).unwrap();
        let mut store = XvcStore::<RecheckMethod>::new();
        for (i, method) in methods.iter().enumerate() {
            store.insert(XvcEntity::from((i as u64, 1)), *method);
        }
        store.save(&store_root).unwrap();
    }

    /// Completion runs on every `TAB`, including outside a repository. The completers turn this
    /// error into an empty candidate list rather than printing anything to the shell.
    #[test]
    fn load_store_for_completion_fails_outside_a_repository() {
        let dir = create_temp_dir();
        assert!(load_store_for_completion::<RecheckMethod>(&dir).is_err());
    }

    /// The completers get the shell's working directory, which is usually somewhere below the Xvc
    /// root, so the store has to be found by walking up the parents.
    #[test]
    fn load_store_for_completion_finds_the_store_from_a_child_directory() {
        let root = create_temp_dir();
        init_fake_repo(&root, &[RecheckMethod::Symlink, RecheckMethod::Copy]);

        let child = root.join("dir-1").join("dir-2");
        fs::create_dir_all(&child).unwrap();

        let store = load_store_for_completion::<RecheckMethod>(&child).unwrap();
        let mut loaded: Vec<String> = store.values().map(|m| m.to_string()).collect();
        loaded.sort();
        assert_eq!(loaded, ["copy", "symlink"]);
    }

    /// `.xvc` without the component's store directory is a valid state -- e.g. a repository where
    /// nothing has been tracked yet. That has to complete to nothing, not fail.
    #[test]
    fn load_store_for_completion_returns_an_empty_store_for_an_unwritten_component() {
        let root = create_temp_dir();
        init_fake_repo(&root, &[]);

        let store = load_store_for_completion::<TextOrBinary>(&root).unwrap();
        assert_eq!(store.len(), 0);
    }
}
