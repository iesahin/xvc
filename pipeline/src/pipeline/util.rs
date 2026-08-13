use std::{env, ffi::OsStr};

use clap_complete::CompletionCandidate;
use xvc_core::util::completer::load_store_for_completion;

use crate::{XvcPipeline, XvcStep, error::Error};

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::pipeline::XvcStepInvalidate;
    use crate::pipeline::api::dag::XvcPipelineDagFormat;
    use crate::pipeline::schema::XvcSchemaSerializationFormat;
    use std::str::FromStr;
    use xvc_core::util::completer::strum_variants_completer;

    fn values(candidates: Vec<CompletionCandidate>) -> Vec<String> {
        let mut values: Vec<String> = candidates
            .iter()
            .map(|c| c.get_value().to_string_lossy().to_string())
            .collect();
        values.sort();
        values
    }

    /// The enum-valued pipeline options (`--when`, `xvc pipeline dag --format`,
    /// `xvc pipeline export --format`) complete through
    /// [strum_variants_completer][xvc_core::util::completer::strum_variants_completer], so the
    /// candidate lists are whatever strum derives. These pin them to the values the commands
    /// document.
    #[test]
    fn enum_valued_options_complete_to_their_documented_values() {
        assert_eq!(
            values(strum_variants_completer::<XvcStepInvalidate>(OsStr::new(
                ""
            ))),
            ["always", "by_dependencies", "never"]
        );
        assert_eq!(
            values(strum_variants_completer::<XvcPipelineDagFormat>(
                OsStr::new("")
            )),
            ["graphviz", "mermaid"]
        );
        assert_eq!(
            values(strum_variants_completer::<XvcSchemaSerializationFormat>(
                OsStr::new("")
            )),
            ["json", "kdl", "yaml"]
        );
    }

    /// A completed value the command then rejects is worse than no completion at all.
    #[test]
    fn enum_candidates_are_accepted_by_their_parser() {
        for value in values(strum_variants_completer::<XvcStepInvalidate>(OsStr::new(
            "",
        ))) {
            XvcStepInvalidate::from_str(&value)
                .unwrap_or_else(|e| panic!("XvcStepInvalidate rejects completion {value}: {e}"));
        }
        for value in values(strum_variants_completer::<XvcPipelineDagFormat>(
            OsStr::new(""),
        )) {
            XvcPipelineDagFormat::from_str(&value)
                .unwrap_or_else(|e| panic!("XvcPipelineDagFormat rejects completion {value}: {e}"));
        }
        for value in values(strum_variants_completer::<XvcSchemaSerializationFormat>(
            OsStr::new(""),
        )) {
            XvcSchemaSerializationFormat::from_str(&value).unwrap_or_else(|e| {
                panic!("XvcSchemaSerializationFormat rejects completion {value}: {e}")
            });
        }
    }
}
