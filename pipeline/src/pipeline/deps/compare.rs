use std::sync::{Arc, RwLock};

use crate::error::{Error, Result};
use crate::XvcEntity;
use anyhow::anyhow;

use subprocess::Exec;
use url::Url;
use xvc_core::types::diff::Diffable;
use xvc_core::util::file::{filter_paths_by_directory, glob_paths, XvcPathMetadataMap};

use xvc_core::{
    AttributeDigest, ContentDigest, Diff, DiffStore, HashAlgorithm, PathCollectionDigest,
    StdoutDigest, TextOrBinary, UrlGetDigest, XvcDigests, XvcMetadata, XvcMetadataDigest, XvcPath,
    XvcRoot,
};
use xvc_ecs::{HStore, R1NStore, Storable, XvcStore};
use xvc_logging::watch;

use super::glob_digest::GlobDigestDep;
use super::lines::LinesDep;
use super::lines_digest::LinesDigestDep;
use super::regex::RegexDep;
use super::regex_digest::RegexDigestDep;
use super::step::StepDep;
use super::{ParamDep, XvcDependency};

use super::file::FileDep;
use super::generic::GenericDep;
use super::glob::GlobDep;
use super::url::UrlDigestDep;

#[derive(Clone, Debug)]
/// Stored and gathered data to decide the validation of dependencies
pub struct DependencyComparisonParams<'a> {
    pub xvc_root: &'a XvcRoot,
    pub pipeline_rundir: &'a XvcPath,
    pub pmm: &'a XvcPathMetadataMap,
    pub algorithm: &'a HashAlgorithm,
    pub all_dependencies: &'a XvcStore<XvcDependency>,
}

///
/// compares two dependencies of the same type
///
/// Decides the dependency type by loading the stored dependency.
/// Calls the respective comparison function for the loaded dependency type.
///
pub fn compare_deps(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: XvcEntity,
) -> Result<Diff<XvcDependency>> {
    let stored = cmp_params
        .all_dependencies
        .get(&stored_dependency_e)
        .ok_or(anyhow!(
            "Stored dependency {:?} not found in all_dependencies",
            stored_dependency_e
        ))?;

    let diff = match stored {
        // Step dependencies are handled differently
        XvcDependency::Step(_) => Diff::Skipped,

        XvcDependency::Generic(generic) => diff_of_dep(compare_deps_generic(cmp_params, generic)?),
        XvcDependency::File(file_dep) => diff_of_dep(compare_deps_file(cmp_params, file_dep)?),
        XvcDependency::Glob(glob_dep) => diff_of_dep(compare_deps_glob(cmp_params, glob_dep)?),
        XvcDependency::UrlDigest(url_dep) => diff_of_dep(compare_deps_url(cmp_params, url_dep)?),
        XvcDependency::Param(param_dep) => diff_of_dep(compare_deps_param(cmp_params, param_dep)?),
        XvcDependency::Regex(regex_dep) => diff_of_dep(compare_deps_regex(cmp_params, regex_dep)?),
        XvcDependency::Lines(lines_dep) => diff_of_dep(compare_deps_lines(cmp_params, lines_dep)?),
        XvcDependency::GlobDigest(dep) => diff_of_dep(compare_deps_glob_digest(cmp_params, dep)?),
        XvcDependency::RegexDigest(dep) => diff_of_dep(compare_deps_regex_digest(cmp_params, dep)?),
        XvcDependency::LinesDigest(dep) => diff_of_dep(compare_deps_lines_digest(cmp_params, dep)?),
    };

    Ok(diff)
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
                (XvcDependency::Generic(r), XvcDependency::Generic(a)) => {
                    diff_of_dep(GenericDep::diff(Some(r), Some(a)))
                }
                (XvcDependency::File(r), XvcDependency::File(a)) => {
                    diff_of_dep(FileDep::diff(Some(r), Some(a)))
                }
                (XvcDependency::Glob(r), XvcDependency::Glob(a)) => {
                    diff_of_dep(GlobDep::diff(Some(a), Some(a)))
                }
                (XvcDependency::Regex(r), XvcDependency::Regex(a)) => {
                    diff_of_dep(RegexDep::diff(Some(r), Some(a)))
                }
                (XvcDependency::Param(r), XvcDependency::Param(a)) => {
                    diff_of_dep(ParamDep::diff(Some(r), Some(a)))
                }
                (XvcDependency::Lines(r), XvcDependency::Lines(a)) => {
                    diff_of_dep(LinesDep::diff(Some(r), Some(a)))
                }
                (XvcDependency::UrlDigest(r), XvcDependency::UrlDigest(a)) => {
                    diff_of_dep(UrlDigestDep::diff(Some(r), Some(a)))
                }
                _ => unreachable!("All dependencies should be of the same type"),
            },
        }
    }
}

/// Runs the command and compares the output with the stored dependency
fn compare_deps_generic(
    cmp_params: DependencyComparisonParams,
    rec_generic_dep: &GenericDep,
) -> Result<Diff<GenericDep>> {
    let actual = GenericDep::new(rec_generic_dep.generic_command);
    Ok(GenericDep::diff(Some(rec_generic_dep), Some(&actual)))
}

/// Compares a dependency path with the actual metadata and content digest found on disk
fn compare_deps_file(
    cmp_params: DependencyComparisonParams,
    record: &FileDep,
) -> Result<Diff<FileDep>> {
    let actual = FileDep::from_pmm(&record.path, cmp_params.pmm);

    Ok(FileDep::diff(Some(record), Some(&actual)))
}

fn compare_deps_url(
    cmp_params: DependencyComparisonParams,
    record: &UrlDigestDep,
) -> Result<Diff<UrlDigestDep>> {
    let actual = UrlDigestDep::new(record.url).update_headers()?;
    Ok(UrlDigestDep::diff(Some(record), Some(&actual)))
}

fn compare_deps_param(
    cmp_params: DependencyComparisonParams,
    record: &ParamDep,
) -> Result<Diff<ParamDep>> {
    let actual = ParamDep::new(&record.path, Some(record.format), record.key)?;

    Ok(ParamDep::diff(Some(record), Some(&actual)))
}

fn compare_deps_regex(
    cmp_params: DependencyComparisonParams,
    record: &RegexDep,
) -> Result<Diff<RegexDep>> {
    let actual = RegexDep::new(record.path.clone(), record.regex);

    Ok(RegexDep::diff(Some(record), Some(&actual)))
}

fn compare_deps_lines(
    cmp_params: DependencyComparisonParams,
    record: &LinesDep,
) -> Result<Diff<LinesDep>> {
    let actual = LinesDep::new(record.path, record.begin, record.end);
    Ok(LinesDep::diff(Some(record), Some(&actual)))
}

/// Compares two globs, one stored and one current.
fn compare_deps_glob(
    cmp_params: DependencyComparisonParams,
    record: &GlobDep,
) -> Result<Diff<GlobDep>> {
    let mut actual = GlobDep::from_pmm(
        cmp_params.xvc_root,
        cmp_params.pipeline_rundir,
        record.glob,
        cmp_params.pmm,
    )?
    .update_changed_paths_digests(record, cmp_params.xvc_root, *cmp_params.algorithm)?;

    Ok(GlobDep::diff(Some(record), Some(&actual)))
}

fn compare_deps_glob_digest(
    cmp_params: DependencyComparisonParams,
    record: &GlobDigestDep,
) -> Result<Diff<GlobDigestDep>> {
    let actual = GlobDigestDep::new(record.glob).update_collection_digests(cmp_params.pmm)?;
    match GlobDigestDep::diff_superficial(record, &actual) {
        Diff::Different { record, actual } => {
            let actual = actual.update_content_digest(cmp_params.xvc_root, cmp_params.pmm)?;
            Ok(GlobDigestDep::diff_thorough(&record, &actual))
        }
        Diff::RecordMissing { actual } => {
            let actual = actual.update_content_digest(cmp_params.xvc_root, cmp_params.pmm)?;
            Ok(GlobDigestDep::diff_thorough(record, &actual))
        }
        diff => Ok(diff),
    }
}

fn compare_deps_regex_digest(
    cmp_params: DependencyComparisonParams,
    record: &RegexDigestDep,
) -> Result<Diff<RegexDigestDep>> {
    let actual = RegexDigestDep::new(record.path, record.regex)
        .update_metadata(cmp_params.pmm.get(&record.path).cloned());
    // Shortcircuit if the metadata is identical
    match RegexDigestDep::diff_superficial(record, &actual) {
        Diff::Different { record, actual } => {
            let actual = actual.update_digest(cmp_params.xvc_root, cmp_params.algorithm);
            Ok(RegexDigestDep::diff_thorough(&record, &actual))
        }
        Diff::RecordMissing { actual } => {
            let actual = actual.update_digest(cmp_params.xvc_root, cmp_params.algorithm);
            Ok(Diff::RecordMissing { actual })
        }
        diff => Ok(diff),
    }
}

fn compare_deps_lines_digest(
    cmp_params: DependencyComparisonParams,
    record: &LinesDigestDep,
) -> Result<Diff<LinesDigestDep>> {
    let actual = LinesDigestDep::new(record.path, record.begin, record.end)
        .update_metadata(cmp_params.pmm.get(&record.path).cloned());

    // Shortcircuit if the metadata is identical
    match LinesDigestDep::diff_superficial(record, &actual) {
        Diff::Different { record, actual } => {
            let actual = actual.update_digest(cmp_params.xvc_root, *cmp_params.algorithm);
            Ok(LinesDigestDep::diff_thorough(&record, &actual))
        }
        Diff::RecordMissing { actual } => {
            let actual = actual.update_digest(cmp_params.xvc_root, *cmp_params.algorithm);
            Ok(Diff::RecordMissing { actual })
        }
        diff => Ok(diff),
    }
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
