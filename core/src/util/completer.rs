//! Completion helpers for commands and options
use crate::{
    types::xvcroot::{find_root, XvcRootInner},
    Error, Result, XvcPath,
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
    env::current_dir()
        .map_err(Error::from)
        .and_then(|current_dir| {
            load_store_for_completion::<XvcPath>(&current_dir).and_then(|xvc_path_store| {
                // FIXME: This doesn't consider current dir to filter the elements
                let elements = xvc_path_store.filter(|_, xp| xp.starts_with_str(&prefix));
                Ok(elements
                    .iter()
                    .map(|(_, xp)| xp.to_string().into())
                    .collect())
            })
        })
        .unwrap_or_default()
}
