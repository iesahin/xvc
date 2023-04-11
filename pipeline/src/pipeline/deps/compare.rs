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
use xvc_ecs::{HStore, R1NStore, XvcStore};
use xvc_logging::watch;

use super::digest::DependencyDigestParams;

use super::lines::LinesDep;
use super::regex::RegexDep;
use super::{ParamDep, XvcDependency};

use super::directory::DirectoryDep;
use super::file::FileDep;
use super::generic::GenericDep;
use super::glob::GlobDep;
use super::url::UrlDep;

#[derive(Clone, Debug)]
/// Stored and gathered data to decide the validation of dependencies
pub struct DependencyComparisonParams<'a> {
    pub xvc_root: &'a XvcRoot,
    pub pipeline_rundir: &'a XvcPath,
    pub pmm: &'a XvcPathMetadataMap,
    pub algorithm: &'a HashAlgorithm,
    pub all_dependencies: &'a XvcStore<XvcDependency>,
}

type DigestDiff = Diff<XvcDigests>;

type Ards<T> = Arc<RwLock<DiffStore<T>>>;

/// Result for diff operations
#[derive(Clone, Debug)]
pub struct Diffs {
    pub xvc_dependency_diff: Ards<XvcDependency>,
    pub xvc_digests_diff: Ards<XvcDigests>,
    pub xvc_metadata_diff: Ards<XvcMetadata>,
    pub xvc_path_diff: Ards<XvcPath>,
}

impl Diffs {
    pub fn new() -> Self {
        Self {
            xvc_dependency_diff: Arc::new(RwLock::new(HStore::new())),
            xvc_digests_diff: Arc::new(RwLock::new(HStore::new())),
            xvc_metadata_diff: Arc::new(RwLock::new(HStore::new())),
            xvc_path_diff: Arc::new(RwLock::new(HStore::new())),
        }
    }

    pub fn insert_xvc_dependency_diff(&mut self, entity: XvcEntity, diff: Diff<XvcDependency>) {
        self.xvc_dependency_diff
            .write()
            .unwrap()
            .insert(entity, diff);
    }

    fn insert_xvc_digests_diff(&mut self, entity: XvcEntity, diff: Diff<XvcDigests>) {
        self.xvc_digests_diff.write().unwrap().insert(entity, diff);
    }

    pub fn insert_xvc_metadata_diff(&mut self, entity: XvcEntity, diff: Diff<XvcMetadata>) {
        self.xvc_metadata_diff.write().unwrap().insert(entity, diff);
    }

    pub fn insert_xvc_path_diff(&mut self, entity: XvcEntity, diff: Diff<XvcPath>) {
        self.xvc_path_diff.write().unwrap().insert(entity, diff);
    }

    pub fn insert_attribute_digest_diff<T: AttributeDigest>(
        &mut self,
        entity: XvcEntity,
        diff: Diff<T>,
    ) -> Result<()> {
        let current_diff = self
            .xvc_digests_diff
            .read()?
            .get(&entity)
            .cloned()
            .unwrap_or_else(|| Diff::Identical);
        let merged_diff = match diff.clone() {
            Diff::Identical | Diff::Skipped => current_diff,
            Diff::RecordMissing {
                actual: incoming_attribute_digest,
            } => match current_diff {
                // If current_diff is identical or skipped, we replace it with the incoming
                Diff::Identical | Diff::Skipped => Diff::RecordMissing {
                    actual: XvcDigests::from_attribute_digest(incoming_attribute_digest),
                },

                Diff::RecordMissing {
                    actual: mut current_xvc_digests,
                } => Diff::RecordMissing {
                    actual: {
                        current_xvc_digests.insert(incoming_attribute_digest);
                        current_xvc_digests
                    },
                },
                Diff::ActualMissing { .. } | Diff::Different { .. } => {
                    return Err(anyhow!("Cannot merge {:?} and {:?}", current_diff, diff).into());
                }
            },

            Diff::ActualMissing {
                record: incoming_attribute_digest,
            } => match current_diff {
                Diff::Identical | Diff::Skipped => Diff::ActualMissing {
                    record: {
                        let xvc_digests =
                            XvcDigests::from_attribute_digest(incoming_attribute_digest);
                        xvc_digests
                    },
                },
                Diff::ActualMissing {
                    record: mut current_xvc_digests,
                } => Diff::ActualMissing {
                    record: {
                        current_xvc_digests.insert(incoming_attribute_digest);
                        current_xvc_digests
                    },
                },
                Diff::RecordMissing { .. } | Diff::Different { .. } => {
                    return Err(anyhow!("Cannot merge {:?} and {:?}", current_diff, diff).into());
                }
            },

            Diff::Different {
                record: incoming_record_digest,
                actual: incoming_actual_digest,
            } => match current_diff {
                Diff::Identical | Diff::Skipped => Diff::Different {
                    record: XvcDigests::from_attribute_digest(incoming_record_digest),
                    actual: XvcDigests::from_attribute_digest(incoming_actual_digest),
                },
                Diff::Different {
                    record: mut current_recorded_digests,
                    actual: mut current_actual_digests,
                } => Diff::Different {
                    record: {
                        current_recorded_digests.insert(incoming_record_digest);
                        current_recorded_digests
                    },
                    actual: {
                        current_actual_digests.insert(incoming_actual_digest);
                        current_actual_digests
                    },
                },

                Diff::RecordMissing { .. } | Diff::ActualMissing { .. } => {
                    return Err(anyhow!("Cannot merge {:?} and {:?}", current_diff, diff).into());
                }
            },
        };

        self.xvc_digests_diff
            .write()
            .map(|mut xvc_digests_diff_store| {
                xvc_digests_diff_store.insert(entity, merged_diff);
            });
        Ok(())
    }
}

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
        .ok_or(err!(
            "Stored dependency {:?} not found in all_dependencies",
            stored_dependency_e
        ))?;

    let diff = match stored {
        // Step dependencies are handled differently
        XvcDependency::Step(_) => Diff::Skipped,

        XvcDependency::Generic(generic) => compare_deps_generic(cmp_params, generic),
        XvcDependency::File(file_dep) => compare_deps_file(cmp_params, file_dep),
        XvcDependency::Glob(glob_dep) => compare_deps_glob(cmp_params, glob_dep),
        XvcDependency::Directory(dir_dep) => compare_deps_directory(cmp_params, dir_dep),
        XvcDependency::Url(url_dep) => compare_deps_url(cmp_params, url_dep),
        XvcDependency::Param(param_dep) => compare_deps_param(cmp_params, param_dep),
        XvcDependency::Regex(regex_dep) => compare_deps_regex(cmp_params, regex_dep),
        XvcDependency::Lines(lines_dep) => compare_deps_lines(cmp_params, lines_dep),
    };

    Ok(diff)
}

impl Diffable for XvcDependency {
    type Item = XvcDependency;

    fn diff(record: XvcDependency, actual: XvcDependency) -> Result<Diff<XvcDependency>> {
        match (record, actual) {
            (XvcDependency::Generic(record), XvcDependency::Generic(actual)) => {
                GenericDep::diff(Some(record), Some(actual))
            }
            (XvcDependency::File(record), XvcDependency::File(actual)) => {
                FileDep::diff(Some(record), Some(actual))
            }
            (XvcDependency::Glob(record), XvcDependency::Glob(actual)) => {
                GlobDep::diff(Some(record), Some(actual))
            }
            (XvcDependency::Directory(record), XvcDependency::Directory(actual)) => {
                DirectoryDep::diff(Some(record), Some(actual))
            }
            (XvcDependency::Url(record), XvcDependency::Url(actual)) => {
                UrlDep::diff(Some(record), Some(actual))
            }
            (XvcDependency::Param(record), XvcDependency::Param(actual)) => {
                ParamDep::diff(Some(record), Some(actual))
            }
            (XvcDependency::Regex(record), XvcDependency::Regex(actual)) => {
                RegexDep::diff(Some(record), Some(actual))
            }
            (XvcDependency::Lines(record), XvcDependency::Lines(actual)) => {
                LinesDep::diff(Some(record), Some(actual))
            }
            (XvcDependency::Step(record), XvcDependency::Step(actual)) => {
                unreachable!("Step dependencies are handled differently")
            }
            _ => Err(anyhow!("Cannot diff {:?} and {:?}", record, actual).into()),
        }
    }
}

/// Runs the command and compares the output with the stored dependency
fn compare_deps_generic(
    cmp_params: DependencyComparisonParams,
    rec_generic_dep: &GenericDep,
) -> Result<Diff<GenericDep>> {
    let actual = GenericDep::new(rec_generic_dep.generic_command);
    Ok(GenericDep::diff(
        Some(rec_generic_dep.clone()),
        Some(actual),
    ))
}

/// Compares a dependency path with the actual metadata and content digest found on disk
fn compare_deps_file(
    cmp_params: DependencyComparisonParams,
    rec_file_dep: &FileDep,
) -> Result<Diff<FileDep>> {
    let actual = FileDep::from_pmm(&rec_file_dep.path, cmp_params.pmm)?;

    Ok(FileDep::diff(Some(rec_file_dep), Some(actual)))
}

fn compare_deps_url(
    cmp_params: DependencyComparisonParams,
    url_dep: &UrlDep,
) -> Result<Diff<UrlDep>> {
    let actual = UrlDep::new(url_dep.url).update_headers()?;
    Ok(UrlDep::diff(Some(url_dep), Some(actual)))
}

fn compare_deps_directory(
    cmp_params: DependencyComparisonParams,
    record: &DirectoryDep,
) -> Result<Diff<DirectoryDep>> {
    let actual = DirectoryDep::from_pmm(cmp_params.xvc_root, record.path, cmp_params.pmm);

    actual.calculate_changed_paths_digests(cmp_params.xvc_root, &record)?;

    match DirectoryDep::diff_superficial(record, actual)? {
        Diff::Different { record, actual } => Ok(DirectoryDep::diff_thorough(record, actual)),
        diff => Ok(diff),
    }
}

fn compare_deps_param(
    cmp_params: DependencyComparisonParams,
    param_dep: &ParamDep,
) -> Result<Diff<ParamDep>> {
    let actual = ParamDep::from_pmm(param_dep.param_name, cmp_params.pmm)?;

    Ok(ParamDep::diff(Some(param_dep), Some(actual)))
}

fn compare_deps_regex(
    cmp_params: DependencyComparisonParams,
    regex_dep: &RegexDep,
) -> Result<Diff<RegexDep>> {
    let actual = RegexDep::from_pmm(regex_dep.param_name, cmp_params.pmm)?;

    Ok(RegexDep::diff(Some(regex_dep), Some(actual)))
}

fn compare_deps_lines(
    cmp_params: DependencyComparisonParams,
    lines_dep: &LinesDep,
) -> Result<Diff<LinesDep>> {
    let actual = LinesDep::from_pmm(lines_dep.param_name, cmp_params.pmm)?;
    Ok(LinesDep::diff(Some(lines_dep), Some(actual)))
}

/// Compare paths in `paths` with their stored values with respect to changes in
/// - CollectionDigest (of all paths)
/// - XvcMetadata for each path
/// - ContentDigest for the changed paths.
///
/// If a path is not found in the stored paths (in cmp_params.xvc_path_store), it is considered missing and an entity is created for it.
/// This entity is not recorded to that store, but is used to record the diff in the collected_diffs.
pub fn compare_deps_multiple_paths(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: XvcEntity,
    paths: &XvcPathMetadataMap,
    collected_diffs: &mut Diffs,
) -> Result<()> {
    let xvc_root = cmp_params.xvc_root;
    let algorithm = cmp_params.algorithm;
    let pmm = cmp_params.pmm;
    let pipeline_rundir = cmp_params.pipeline_rundir;
    let _dep_digest_params = DependencyDigestParams {
        xvc_root,
        algorithm,
        pipeline_rundir,
        pmm,
    };
    let actual_collection_digest = PathCollectionDigest::new(paths.keys().cloned(), *algorithm)?;
    let stored_xvc_digests = cmp_params.xvc_digests_store.get(&stored_dependency_e);
    let stored_collection_digest = stored_xvc_digests.and_then(|s| s.get::<PathCollectionDigest>());

    let collection_digest_diff =
        PathCollectionDigest::diff(stored_collection_digest, Some(actual_collection_digest));

    collected_diffs
        .insert_attribute_digest_diff(stored_dependency_e, collection_digest_diff.clone())?;

    // If collection digest is changed, we have changes in the paths
    match collection_digest_diff {
        Diff::Skipped | Diff::Identical => {}
        Diff::RecordMissing { .. } | Diff::ActualMissing { .. } | Diff::Different { .. } => {
            paths.iter().for_each(|(xvc_path, xvc_md)| {
                match cmp_params.xvc_path_store.entity_by_value(xvc_path) {
                    Some(xvc_path_e) => {
                        collected_diffs.insert_xvc_path_diff(xvc_path_e, Diff::Skipped);
                        let actual_xvc_metadata = xvc_md.clone();
                        let recorded_xvc_metadata =
                            cmp_params.xvc_metadata_store.get(&xvc_path_e).cloned();
                        let xvc_metadata_diff =
                            XvcMetadata::diff(recorded_xvc_metadata, Some(actual_xvc_metadata));
                        collected_diffs.insert_xvc_metadata_diff(xvc_path_e, xvc_metadata_diff);
                    }
                    None => {
                        let new_xvc_path_e = xvc_root.new_entity();
                        collected_diffs.insert_xvc_path_diff(
                            new_xvc_path_e,
                            Diff::RecordMissing {
                                actual: xvc_path.clone(),
                            },
                        );
                        let actual_xvc_metadata = xvc_md.clone();
                        let xvc_metadata_diff = XvcMetadata::diff(None, Some(actual_xvc_metadata));
                        collected_diffs.insert_xvc_metadata_diff(new_xvc_path_e, xvc_metadata_diff);
                    }
                }
            });
        }
    }

    // If any of the paths are changed, we need to compare the content digest
    let changed_entities = collected_diffs
        .xvc_metadata_diff
        .read()
        .map(|xvc_metadata_diffs| {
            xvc_metadata_diffs
                .iter()
                .filter_map(|(e, d)| if d.changed() { Some(*e) } else { None })
                .collect::<Vec<_>>()
        })?;

    let changed_paths: Vec<(XvcEntity, XvcPath)> = changed_entities
        .iter()
        .map(|xe| -> (XvcEntity, XvcPath) {
            let (xe, path) = collected_diffs
                .xvc_path_diff
                .read()
                .map(
                    |store| match store.get(&xe).expect("Entity should exist in the store") {
                        Diff::RecordMissing { actual } => (xe, actual.clone()),
                        Diff::ActualMissing { record } => (xe, record.clone()),
                        Diff::Skipped => cmp_params
                            .xvc_path_store
                            .get(&xe)
                            .map(|p| (xe, p.clone()))
                            .expect("Entity should exist in the store"),
                        Diff::Different { record: _, actual } => (xe, actual.clone()),
                        Diff::Identical => cmp_params
                            .xvc_path_store
                            .get(&xe)
                            .map(|p| (xe, p.clone()))
                            .expect("Entity should exist in the store"),
                    },
                )
                .unwrap();
            (*xe, path)
        })
        .collect();

    let mut content_digest_diffs: HStore<Diff<ContentDigest>> =
        HStore::with_capacity(changed_paths.len());

    collected_diffs
        .xvc_metadata_diff
        .read()
        .and_then(|xvc_md_diff_store| {
            changed_paths.iter().for_each(|(xe, xp)| {
                let xmd = xvc_md_diff_store.get(xe);

                let content_digest_diff = match xmd {
                    Some(Diff::Identical) => Diff::<ContentDigest>::Identical,

                    Some(Diff::Skipped) => Diff::<ContentDigest>::Skipped,

                    Some(Diff::RecordMissing { .. }) | None => {
                        let text_or_binary =
                            cmp_params.text_files.get(&xe).copied().unwrap_or_default();
                        let actual_content_digest = ContentDigest::new(
                            &xp.to_absolute_path(&xvc_root),
                            *algorithm,
                            text_or_binary,
                        )
                        .unwrap();
                        Diff::RecordMissing {
                            actual: actual_content_digest,
                        }
                    }

                    Some(Diff::ActualMissing { record: _ }) => {
                        let recorded_content_digest = cmp_params
                            .xvc_digests_store
                            .get(&xe)
                            .and_then(|xvc_digests| xvc_digests.get::<ContentDigest>())
                            .unwrap();
                        Diff::ActualMissing {
                            record: recorded_content_digest,
                        }
                    }

                    Some(Diff::Different { .. }) => {
                        let text_or_binary =
                            cmp_params.text_files.get(&xe).copied().unwrap_or_default();
                        let actual_content_digest = ContentDigest::new(
                            &xp.to_absolute_path(xvc_root),
                            *algorithm,
                            text_or_binary,
                        )
                        .map_err(|e| e.panic())
                        .ok();

                        let recorded_content_digest = cmp_params
                            .xvc_digests_store
                            .get(&xe)
                            .and_then(|xvc_digests| xvc_digests.get::<ContentDigest>());

                        ContentDigest::diff(recorded_content_digest, actual_content_digest)
                    }
                };

                content_digest_diffs.insert(*xe, content_digest_diff);
            });
            Ok(())
        })?;

    content_digest_diffs.drain().for_each(|(xe, diff)| {
        collected_diffs
            .insert_attribute_digest_diff(xe, diff)
            .unwrap()
    });
    Ok(())
}

/// Compares two globs, one stored and one current.
///
/// Uses [compare_deps_multiple_paths] after extracting the paths with [glob_paths]
fn compare_deps_glob(
    cmp_params: DependencyComparisonParams,
    record: &GlobDep,
) -> Result<Diff<GlobDep>> {
    let actual = GlobDep::from_pmm(
        cmp_params.xvc_root,
        cmp_params.pipeline_rundir,
        record.glob,
        cmp_params.pmm,
    );

    actual.calculate_changed_paths_digests(cmp_params.xvc_root, &record)?;

    match GlobDep::diff_superficial(record, actual)? {
        Diff::Different { record, actual } => Ok(GlobDep::diff_thorough(record, actual)),
        diff => Ok(diff),
    }
}
