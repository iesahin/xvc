use std::sync::{Arc, RwLock};

use crate::error::{Error, Result};
use crate::pipeline::StepStateParams;
use crate::XvcEntity;
use anyhow::anyhow;

use subprocess::Exec;
use url::Url;
use xvc_core::types::diff::{self, Diffable};
use xvc_core::util::file::{filter_paths_by_directory, glob_paths, XvcPathMetadataMap};

use xvc_core::{
    AttributeDigest, ContentDigest, Diff, DiffStore, HashAlgorithm, PathCollectionDigest,
    StdoutDigest, TextOrBinary, UrlContentDigest, XvcDigests, XvcMetadata, XvcMetadataDigest,
    XvcPath, XvcRoot,
};
use xvc_ecs::{HStore, R1NStore, Storable, XvcStore};
use xvc_logging::watch;

use super::glob_digest::GlobDep;
use super::lines::LineItemsDep;
use super::lines_digest::LinesDep;
use super::regex::RegexItemsDep;
use super::regex_digest::RegexDep;
use super::step::StepDep;
use super::{ParamDep, XvcDependency};

use super::file::FileDep;
use super::generic::GenericDep;
use super::glob_items::GlobItemsDep;
use super::url::UrlDigestDep;

#[derive(Clone, Debug)]
/// Stored and gathered data to decide the validation of dependencies
pub struct DependencyComparisonParams<'a> {
    pub xvc_root: &'a XvcRoot,
    pub pipeline_rundir: &'a XvcPath,
    pub pmm: &'a XvcPathMetadataMap,
    pub algorithm: &'a HashAlgorithm,
    pub step_dependencies: &'a HStore<XvcDependency>,
}

impl Diffable for XvcDependency {
    type Item = XvcDependency;

    fn diff(record: Option<&XvcDependency>, actual: Option<&XvcDependency>) -> Diff<XvcDependency> {
        match (record, actual) {
            (None, None) => Diff::Skipped,
            (None, Some(actual)) => Diff::RecordMissing {
                actual: actual.clone(),
            },
            (Some(record), None) => Diff::ActualMissing {
                record: record.clone(),
            },
            (Some(record), Some(actual)) => match (record, actual) {
                (XvcDependency::Step(record), XvcDependency::Step(actual)) => {
                    diff_of_dep(StepDep::diff(Some(record), Some(actual)))
                }

                (XvcDependency::Generic(record), XvcDependency::Generic(actual)) => {
                    diff_of_dep(GenericDep::diff(Some(record), Some(actual)))
                }

                (XvcDependency::File(record), XvcDependency::File(actual)) => {
                    diff_of_dep(FileDep::diff(Some(record), Some(actual)))
                }

                (XvcDependency::GlobItems(record), XvcDependency::GlobItems(actual)) => {
                    diff_of_dep(GlobItemsDep::diff(Some(record), Some(actual)))
                }

                (XvcDependency::Glob(record), XvcDependency::Glob(actual)) => {
                    diff_of_dep(GlobDep::diff(Some(record), Some(actual)))
                }

                (XvcDependency::RegexItems(record), XvcDependency::RegexItems(actual)) => {
                    diff_of_dep(RegexItemsDep::diff(Some(record), Some(actual)))
                }

                (XvcDependency::Regex(record), XvcDependency::Regex(actual)) => {
                    diff_of_dep(RegexDep::diff(Some(record), Some(actual)))
                }

                (XvcDependency::Param(record), XvcDependency::Param(actual)) => {
                    diff_of_dep(ParamDep::diff(Some(record), Some(actual)))
                }

                (XvcDependency::LineItems(record), XvcDependency::LineItems(actual)) => {
                    diff_of_dep(LineItemsDep::diff(Some(record), Some(actual)))
                }

                (XvcDependency::UrlDigest(record), XvcDependency::UrlDigest(actual)) => {
                    diff_of_dep(UrlDigestDep::diff(Some(record), Some(actual)))
                }

                (XvcDependency::Lines(record), XvcDependency::Lines(actual)) => {
                    diff_of_dep(LinesDep::diff(Some(record), Some(actual)))
                }
                _ => unreachable!("All dependencies should be of the same type"),
            },
        }
    }

    fn diff_superficial(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        match (record, actual) {
            (XvcDependency::Step(record), XvcDependency::Step(actual)) => {
                diff_of_dep(StepDep::diff_superficial(record, actual))
            }

            (XvcDependency::Generic(record), XvcDependency::Generic(actual)) => {
                diff_of_dep(GenericDep::diff_superficial(record, actual))
            }

            (XvcDependency::File(record), XvcDependency::File(actual)) => {
                diff_of_dep(FileDep::diff_superficial(record, actual))
            }

            (XvcDependency::GlobItems(record), XvcDependency::GlobItems(actual)) => {
                diff_of_dep(GlobItemsDep::diff_superficial(record, actual))
            }

            (XvcDependency::Glob(record), XvcDependency::Glob(actual)) => {
                diff_of_dep(GlobDep::diff_superficial(record, actual))
            }

            (XvcDependency::RegexItems(record), XvcDependency::RegexItems(actual)) => {
                diff_of_dep(RegexItemsDep::diff_superficial(record, actual))
            }

            (XvcDependency::Regex(record), XvcDependency::Regex(actual)) => {
                diff_of_dep(RegexDep::diff_superficial(record, actual))
            }

            (XvcDependency::Param(record), XvcDependency::Param(actual)) => {
                diff_of_dep(ParamDep::diff_superficial(record, actual))
            }

            (XvcDependency::LineItems(record), XvcDependency::LineItems(actual)) => {
                diff_of_dep(LineItemsDep::diff_superficial(record, actual))
            }

            (XvcDependency::Lines(record), XvcDependency::Lines(actual)) => {
                diff_of_dep(LinesDep::diff_superficial(record, actual))
            }

            (XvcDependency::UrlDigest(record), XvcDependency::UrlDigest(actual)) => {
                diff_of_dep(UrlDigestDep::diff_superficial(record, actual))
            }

            _ => unreachable!("All dependencies should be of the same type"),
        }
    }

    fn diff_thorough(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        match (record, actual) {
            (XvcDependency::Step(record), XvcDependency::Step(actual)) => {
                diff_of_dep(StepDep::diff_thorough(record, actual))
            }

            (XvcDependency::Generic(record), XvcDependency::Generic(actual)) => {
                let actual = actual.clone().update_output_digest().unwrap();
                diff_of_dep(GenericDep::diff_thorough(record, &actual))
            }

            (XvcDependency::File(record), XvcDependency::File(actual)) => {
                diff_of_dep(FileDep::diff_thorough(record, actual))
            }

            (XvcDependency::GlobItems(record), XvcDependency::GlobItems(actual)) => {
                diff_of_dep(GlobItemsDep::diff_thorough(record, actual))
            }

            (XvcDependency::Glob(record), XvcDependency::Glob(actual)) => {
                diff_of_dep(GlobDep::diff_thorough(record, actual))
            }

            (XvcDependency::RegexItems(record), XvcDependency::RegexItems(actual)) => {
                diff_of_dep(RegexItemsDep::diff_thorough(record, actual))
            }

            (XvcDependency::Regex(record), XvcDependency::Regex(actual)) => {
                diff_of_dep(RegexDep::diff_thorough(record, actual))
            }

            (XvcDependency::Param(record), XvcDependency::Param(actual)) => {
                diff_of_dep(ParamDep::diff_thorough(record, actual))
            }

            (XvcDependency::LineItems(record), XvcDependency::LineItems(actual)) => {
                diff_of_dep(LineItemsDep::diff_thorough(record, actual))
            }

            (XvcDependency::Lines(record), XvcDependency::Lines(actual)) => {
                diff_of_dep(LinesDep::diff_thorough(record, actual))
            }

            (XvcDependency::UrlDigest(record), XvcDependency::UrlDigest(actual)) => {
                diff_of_dep(UrlDigestDep::diff_thorough(record, actual))
            }

            _ => unreachable!("All dependencies should be of the same type"),
        }
    }
}

///
/// compares two dependencies of the same type
///
/// Decides the dependency type by loading the stored dependency.
/// Calls the respective comparison function for the loaded dependency type.
///
pub fn thorough_compare_dependency(
    cmp_params: &StepStateParams,
    stored_dependency_e: XvcEntity,
) -> Result<Diff<XvcDependency>> {
    let stored = if cmp_params.all_steps.contains_key(&stored_dependency_e) {
        let step = cmp_params.all_steps[&stored_dependency_e].clone();
        Ok(XvcDependency::Step(StepDep {
            name: step.name.clone(),
        }))
    } else {
        cmp_params
            .recorded_dependencies
            .children
            .get(&stored_dependency_e)
            .cloned()
            .ok_or(anyhow!(
                "Stored dependency {:?} not found in step dependencies",
                stored_dependency_e
            ))
    }?;

    let diff = match stored {
        // Step dependencies are handled differently
        XvcDependency::Step(_) => Diff::Skipped,
        XvcDependency::Generic(generic) => {
            watch!(generic);
            diff_of_dep(thorough_compare_generic(cmp_params, &generic)?)
        }
        XvcDependency::File(file_dep) => diff_of_dep(thorough_compare_file(cmp_params, &file_dep)?),
        XvcDependency::GlobItems(glob_dep) => {
            diff_of_dep(thorough_compare_glob(cmp_params, &glob_dep)?)
        }
        XvcDependency::UrlDigest(url_dep) => diff_of_dep(thorough_compare_url(&url_dep)?),
        XvcDependency::Param(param_dep) => diff_of_dep(thorough_compare_param(&param_dep)?),
        XvcDependency::RegexItems(regex_dep) => diff_of_dep(thorough_compare_regex(&regex_dep)?),
        XvcDependency::LineItems(lines_dep) => diff_of_dep(thorough_compare_lines(&lines_dep)?),
        XvcDependency::Glob(dep) => diff_of_dep(thorough_compare_glob_digest(cmp_params, &dep)?),
        XvcDependency::Regex(dep) => diff_of_dep(thorough_compare_regex_digest(cmp_params, &dep)?),
        XvcDependency::Lines(dep) => diff_of_dep(thorough_compare_lines_digest(cmp_params, &dep)?),
    };

    Ok(diff)
}

/// Runs the command and compares the output with the stored dependency
fn thorough_compare_generic(
    cmp_params: &StepStateParams,
    record: &GenericDep,
) -> Result<Diff<GenericDep>> {
    let mut actual = GenericDep::new(record.generic_command.clone());
    actual = actual.update_output_digest()?;
    watch!(record);
    watch!(actual);
    Ok(GenericDep::diff_thorough(record, &actual))
}

/// Compares a dependency path with the actual metadata and content digest found on disk
fn thorough_compare_file(cmp_params: &StepStateParams, record: &FileDep) -> Result<Diff<FileDep>> {
    let actual = FileDep::from_pmm(&record.path, cmp_params.pmm.read().as_ref()?)?;
    let actual = actual.calculate_content_digest(
        cmp_params.xvc_root,
        cmp_params.algorithm,
        TextOrBinary::Auto,
    )?;
    watch!(actual);
    Ok(FileDep::diff_thorough(record, &actual))
}

fn thorough_compare_url(record: &UrlDigestDep) -> Result<Diff<UrlDigestDep>> {
    let actual = UrlDigestDep::new(record.url.clone()).update_content_digest()?;
    Ok(UrlDigestDep::diff_thorough(record, &actual))
}

fn thorough_compare_param(record: &ParamDep) -> Result<Diff<ParamDep>> {
    let actual = ParamDep::new(&record.path, Some(record.format), record.key.clone())?;

    Ok(ParamDep::diff(Some(record), Some(&actual)))
}

fn thorough_compare_regex(record: &RegexItemsDep) -> Result<Diff<RegexItemsDep>> {
    let actual = RegexItemsDep::new(record.path.clone(), record.regex.clone());

    Ok(RegexItemsDep::diff(Some(record), Some(&actual)))
}

fn thorough_compare_lines(record: &LineItemsDep) -> Result<Diff<LineItemsDep>> {
    let actual = LineItemsDep::new(record.path.clone(), record.begin, record.end);
    Ok(LineItemsDep::diff(Some(record), Some(&actual)))
}

/// Compares two globs, one stored and one current.
fn thorough_compare_glob(
    cmp_params: &StepStateParams,
    record: &GlobItemsDep,
) -> Result<Diff<GlobItemsDep>> {
    let mut actual = GlobItemsDep::from_pmm(
        cmp_params.xvc_root,
        cmp_params.pipeline_rundir,
        record.glob.clone(),
        cmp_params.pmm.read().as_ref()?,
    )?
    .update_changed_paths_digests(record, cmp_params.xvc_root, cmp_params.algorithm)?;

    Ok(GlobItemsDep::diff(Some(record), Some(&actual)))
}

fn thorough_compare_glob_digest(
    cmp_params: &StepStateParams,
    record: &GlobDep,
) -> Result<Diff<GlobDep>> {
    let actual = GlobDep::new(record.glob.clone())
        .update_collection_digests(cmp_params.pmm.read().as_ref()?)?;
    match GlobDep::diff_superficial(record, &actual) {
        Diff::Different { record, actual } => {
            let actual = actual
                .update_content_digest(cmp_params.xvc_root, cmp_params.pmm.read().as_ref()?)?;
            Ok(GlobDep::diff_thorough(&record, &actual))
        }
        Diff::RecordMissing { actual } => {
            let actual = actual
                .update_content_digest(cmp_params.xvc_root, cmp_params.pmm.read().as_ref()?)?;
            Ok(GlobDep::diff_thorough(record, &actual))
        }
        diff => Ok(diff),
    }
}

fn thorough_compare_regex_digest(
    cmp_params: &StepStateParams,
    record: &RegexDep,
) -> Result<Diff<RegexDep>> {
    let actual = RegexDep::new(record.path.clone(), record.regex.clone())
        .update_metadata(cmp_params.pmm.read().as_ref()?.get(&record.path).cloned());
    // Shortcircuit if the metadata is identical
    match RegexDep::diff_superficial(record, &actual) {
        Diff::Different { record, actual } => {
            let actual = actual.update_digest(cmp_params.xvc_root, cmp_params.algorithm);
            Ok(RegexDep::diff_thorough(&record, &actual))
        }
        Diff::RecordMissing { actual } => {
            let actual = actual.update_digest(cmp_params.xvc_root, cmp_params.algorithm);
            Ok(Diff::RecordMissing { actual })
        }
        diff => Ok(diff),
    }
}

fn thorough_compare_lines_digest(
    cmp_params: &StepStateParams,
    record: &LinesDep,
) -> Result<Diff<LinesDep>> {
    let actual = LinesDep::new(record.path.clone(), record.begin, record.end)
        .update_metadata(cmp_params.pmm.read().as_ref()?.get(&record.path).cloned());

    // Shortcircuit if the metadata is identical
    match LinesDep::diff_superficial(record, &actual) {
        Diff::Different { record, actual } => {
            let actual = actual.update_digest(cmp_params.xvc_root, cmp_params.algorithm);
            Ok(LinesDep::diff_thorough(&record, &actual))
        }
        Diff::RecordMissing { actual } => {
            let actual = actual.update_digest(cmp_params.xvc_root, cmp_params.algorithm);
            Ok(Diff::RecordMissing { actual })
        }
        diff => Ok(diff),
    }
}

pub fn superficial_compare_dependency(
    cmp_params: &StepStateParams,
    stored_dependency_e: XvcEntity,
) -> Result<Diff<XvcDependency>> {
    // If the dependency is a step, we reify it here
    // Otherwise we search the dependencies for its key
    let stored = if cmp_params.all_steps.contains_key(&stored_dependency_e) {
        let step = cmp_params.all_steps[&stored_dependency_e].clone();
        Ok(XvcDependency::Step(StepDep { name: step.name }))
    } else {
        cmp_params
            .recorded_dependencies
            .children
            .get(&stored_dependency_e)
            .cloned()
            .ok_or(anyhow!(
                "Stored dependency {:?} not found in step dependencies",
                stored_dependency_e
            ))
    }?;

    let diff = match &stored {
        // Step dependencies are handled differently
        XvcDependency::Step(_) => Diff::Skipped,
        XvcDependency::Generic(generic) => {
            watch!(generic);
            diff_of_dep(superficial_compare_generic(cmp_params, generic)?)
        }
        XvcDependency::File(file_dep) => {
            diff_of_dep(superficial_compare_file(cmp_params, file_dep)?)
        }
        XvcDependency::GlobItems(glob_dep) => {
            diff_of_dep(superficial_compare_glob(cmp_params, glob_dep)?)
        }
        XvcDependency::UrlDigest(url_dep) => diff_of_dep(superficial_compare_url(url_dep)?),
        XvcDependency::Param(param_dep) => diff_of_dep(superficial_compare_param(param_dep)?),
        XvcDependency::RegexItems(regex_dep) => diff_of_dep(superficial_compare_regex(regex_dep)?),
        XvcDependency::LineItems(lines_dep) => diff_of_dep(superficial_compare_lines(lines_dep)?),
        XvcDependency::Glob(dep) => diff_of_dep(superficial_compare_glob_digest(cmp_params, dep)?),
        XvcDependency::Regex(dep) => {
            diff_of_dep(superficial_compare_regex_digest(cmp_params, dep)?)
        }
        XvcDependency::Lines(dep) => {
            diff_of_dep(superficial_compare_lines_digest(cmp_params, dep)?)
        }
    };

    Ok(diff)
}

/// Runs the command and compares the output with the stored dependency
fn superficial_compare_generic(
    cmp_params: &StepStateParams,
    record: &GenericDep,
) -> Result<Diff<GenericDep>> {
    let actual = GenericDep::new(record.generic_command.clone());
    let actual = actual.update_output_digest()?;
    Ok(GenericDep::diff_superficial(record, &actual))
}

/// Compares a dependency path with the actual metadata and content digest found on disk
fn superficial_compare_file(
    cmp_params: &StepStateParams,
    record: &FileDep,
) -> Result<Diff<FileDep>> {
    let actual = FileDep::from_pmm(&record.path, cmp_params.pmm.read().as_ref()?)?;
    watch!(actual);
    Ok(FileDep::diff_superficial(record, &actual))
}

fn superficial_compare_url(record: &UrlDigestDep) -> Result<Diff<UrlDigestDep>> {
    let actual = UrlDigestDep::new(record.url.clone()).update_headers()?;
    watch!(actual);
    Ok(UrlDigestDep::diff_superficial(record, &actual))
}

fn superficial_compare_param(record: &ParamDep) -> Result<Diff<ParamDep>> {
    let actual = ParamDep::new(&record.path, Some(record.format), record.key.clone())?;
    Ok(ParamDep::diff_superficial(record, &actual))
}

fn superficial_compare_regex(record: &RegexItemsDep) -> Result<Diff<RegexItemsDep>> {
    let actual = RegexItemsDep::new(record.path.clone(), record.regex.clone());
    Ok(RegexItemsDep::diff_superficial(record, &actual))
}

fn superficial_compare_lines(record: &LineItemsDep) -> Result<Diff<LineItemsDep>> {
    let actual = LineItemsDep::new(record.path.clone(), record.begin, record.end);
    Ok(LineItemsDep::diff_superficial(record, &actual))
}

/// Compares two globs, one stored and one current.
fn superficial_compare_glob(
    cmp_params: &StepStateParams,
    record: &GlobItemsDep,
) -> Result<Diff<GlobItemsDep>> {
    let mut actual = GlobItemsDep::from_pmm(
        cmp_params.xvc_root,
        cmp_params.pipeline_rundir,
        record.glob.clone(),
        cmp_params.pmm.read().as_ref()?,
    )?
    .update_changed_paths_digests(record, cmp_params.xvc_root, cmp_params.algorithm)?;

    Ok(GlobItemsDep::diff_superficial(record, &actual))
}

fn superficial_compare_glob_digest(
    cmp_params: &StepStateParams,
    record: &GlobDep,
) -> Result<Diff<GlobDep>> {
    let actual = GlobDep::new(record.glob.clone())
        .update_collection_digests(cmp_params.pmm.read().as_ref()?)?;
    Ok(GlobDep::diff_superficial(record, &actual))
}

fn superficial_compare_regex_digest(
    cmp_params: &StepStateParams,
    record: &RegexDep,
) -> Result<Diff<RegexDep>> {
    let actual = RegexDep::new(record.path.clone(), record.regex.clone())
        .update_metadata(cmp_params.pmm.read().as_ref()?.get(&record.path).cloned());
    Ok(RegexDep::diff_superficial(record, &actual))
}

fn superficial_compare_lines_digest(
    cmp_params: &StepStateParams,
    record: &LinesDep,
) -> Result<Diff<LinesDep>> {
    let actual = LinesDep::new(record.path.clone(), record.begin, record.end)
        .update_metadata(cmp_params.pmm.read().as_ref()?.get(&record.path).cloned());

    Ok(LinesDep::diff_superficial(record, &actual))
}
fn diff_of_dep<T>(dep: Diff<T>) -> Diff<XvcDependency>
where
    T: Storable + Into<XvcDependency>,
{
    match dep {
        Diff::Identical => Diff::Identical,
        Diff::RecordMissing { actual } => Diff::RecordMissing {
            actual: actual.into(),
        },
        Diff::ActualMissing { record } => Diff::ActualMissing {
            record: record.into(),
        },
        Diff::Different { record, actual } => Diff::Different {
            record: record.into(),
            actual: actual.into(),
        },
        Diff::Skipped => Diff::Skipped,
    }
}
