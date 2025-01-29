use std::{env, ffi::OsStr};

use clap_complete::CompletionCandidate;
use xvc_core::util::completer::load_store_for_completion;

use crate::{error::Error, XvcPipeline, XvcStep};

/// Return all pipeline names starting with `prefix`
pub fn pipeline_name_completer(prefix: &OsStr) -> Vec<CompletionCandidate> {
    // This must be safe as we don't allow Non-UTF-8 strings for storage identifiers
    let prefix = prefix.to_str().unwrap_or("");
    env::current_dir()
        .map_err(Error::from)
        .map(|current_dir| {
            load_store_for_completion::<XvcPipeline>(&current_dir)
                .map(|xvc_pipeline_store| {
                    xvc_pipeline_store
                        .filter(|_, xp| xp.name.starts_with(prefix))
                        .iter()
                        .map(|(_, xp)| xp.name.clone().into())
                        .collect()
                })
                .unwrap_or_default()
        })
        .unwrap_or_default()
}

/// Return all step names starting with `prefix`
pub fn step_name_completer(prefix: &OsStr) -> Vec<CompletionCandidate> {
    // This must be safe as we don't allow Non-UTF-8 strings for storage identifiers
    let prefix = prefix.to_str().unwrap_or("");
    env::current_dir()
        .map_err(Error::from)
        .map(|current_dir| {
            load_store_for_completion::<XvcStep>(&current_dir)
                .map(|xvc_pipeline_store| {
                    xvc_pipeline_store
                        .filter(|_, xp| xp.name.starts_with(prefix))
                        .iter()
                        .map(|(_, xp)| xp.name.clone().into())
                        .collect()
                })
                .unwrap_or_default()
        })
        .unwrap_or_default()
}
