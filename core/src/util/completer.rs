//! Completion helpers for commands and options
use std::path::Path;

use clap_complete::CompletionCandidate;

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
