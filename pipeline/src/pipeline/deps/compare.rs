use std::sync::{Arc, RwLock};

use crate::error::{Error, Result};
use crate::XvcEntity;
use anyhow::anyhow;

use subprocess::Exec;
use url::Url;
use xvc_core::types::diff::Diffable;
use xvc_core::util::file::{filter_paths_by_directory, glob_paths, XvcPathMetadataMap};

use xvc_core::{
    AttributeDigest, CollectionDigest, ContentDigest, Diff, DiffStore, HashAlgorithm, StdoutDigest,
    TextOrBinary, UrlGetDigest, UrlHeadDigest, XvcDigests, XvcMetadata, XvcPath, XvcRoot,
};
use xvc_ecs::{HStore, R1NStore, XvcStore};
use xvc_logging::watch;

use super::digest::DependencyDigestParams;

use super::XvcDependency;

#[derive(Clone, Debug)]
/// Stored and gathered data to decide the validation of dependencies
pub struct DependencyComparisonParams<'a> {
    pub xvc_root: &'a XvcRoot,
    pub pipeline_rundir: &'a XvcPath,
    pub pmm: &'a XvcPathMetadataMap,
    pub algorithm: &'a HashAlgorithm,
    pub all_dependencies: &'a XvcStore<XvcDependency>,
    pub dependency_paths: &'a R1NStore<XvcDependency, XvcPath>,
    pub xvc_path_store: &'a XvcStore<XvcPath>,
    pub xvc_metadata_store: &'a XvcStore<XvcMetadata>,
    pub xvc_digests_store: &'a XvcStore<XvcDigests>,
    pub text_files: &'a XvcStore<TextOrBinary>,
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
/// TODO: This can probably be avoided (or simplified) using a common trait for comparison across
/// all dependency delta types.
pub fn compare_deps(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: XvcEntity,
    collected_diffs: &mut Diffs,
) -> Result<Diff<XvcDependency>> {
    let stored = &cmp_params.all_dependencies[&stored_dependency_e];
    watch!(collected_diffs);
    match stored {
        // Step dependencies are handled differently
        XvcDependency::Step { .. } => Ok(Diff::Skipped),

        dep @ XvcDependency::Generic { .. } => {
            compare_deps_generic(cmp_params, stored_dependency_e, dep)
        }
        XvcDependency::File { path } => {
            compare_deps_single_path(cmp_params, stored_dependency_e, path, collected_diffs)
        }
        XvcDependency::Glob { glob } => {
            compare_deps_glob(cmp_params, stored_dependency_e, glob, collected_diffs)
        }
        XvcDependency::Directory { path } => {
            compare_deps_directory(cmp_params, stored_dependency_e, path, collected_diffs)
        }
        XvcDependency::Url { url } => {
            compare_deps_url(cmp_params, stored_dependency_e, url, collected_diffs)
        }
        XvcDependency::Param {
            path,
            format: _,
            key: _,
        } => compare_deps_single_path(cmp_params, stored_dependency_e, path, collected_diffs),
        XvcDependency::Regex { path, regex: _ } => {
            compare_deps_single_path(cmp_params, stored_dependency_e, path, collected_diffs)
        }
        XvcDependency::Lines {
            path,
            begin: _,
            end: _,
        } => compare_deps_single_path(cmp_params, stored_dependency_e, path, collected_diffs),
    }
}

/// Runs the command and compares the output with the stored dependency
fn compare_deps_generic(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: XvcEntity,
    dependency: &XvcDependency,
) -> Result<Diff<XvcDependency>> {
    if let XvcDependency::Generic {
        generic_command,
        output_digest,
    } = dependency
    {
        let command_output = Exec::shell(generic_command).capture()?;
        let stdout = String::from_utf8(command_output.stdout)?;
        let stderr = String::from_utf8(command_output.stderr)?;
        let algorithm = *cmp_params.algorithm;
        let return_code = command_output.exit_status;
        if stderr.len() > 0 || !return_code.success() {
            return Err(Error::ProcessError { stdout, stderr });
        }

        let actual = StdoutDigest::new(&stdout, algorithm).into();
        let record = cmp_params.all_dependencies.get(&stored_dependency_e);

        match (record, actual) {
            (
                Some(
                    record @ XvcDependency::Generic {
                        output_digest: Some(recorded_digest),
                        generic_command: recorded_generic_command,
                    },
                ),
                Some(actual_digest),
            ) => {
                assert!(recorded_generic_command == generic_command);
                if *recorded_digest == actual_digest {
                    Ok(Diff::Identical)
                } else {
                    Ok(Diff::Different {
                        record: record.clone(),
                        actual: XvcDependency::Generic {
                            output_digest: Some(actual_digest),
                            generic_command: generic_command.clone(),
                        },
                    })
                }
            }

            (
                Some(XvcDependency::Generic {
                    output_digest: None,
                    generic_command: recorded_generic_command,
                }),
                Some(actual_digest),
            ) => {
                assert!(recorded_generic_command == generic_command);
                Ok(Diff::RecordMissing {
                    actual: XvcDependency::Generic {
                        output_digest: Some(actual_digest),
                        generic_command: generic_command.clone(),
                    },
                })
            }

            (None, Some(actual_digest)) => Ok(Diff::RecordMissing {
                actual: XvcDependency::Generic {
                    output_digest: Some(actual_digest),
                    generic_command: generic_command.clone(),
                },
            }),

            (
                Some(
                    record @ XvcDependency::Generic {
                        output_digest: Some(recorded_digest),
                        ..
                    },
                ),
                None,
            ) => Ok(Diff::ActualMissing {
                record: record.clone(),
            }),

            (None, None) => unreachable!(),
            _ => unreachable!(),
        }
    } else {
        unreachable!();
    }
}

/// Compares a dependency path with the actual metadata and content digest found on disk
///
/// It loads the dependency, extracts the path and calls [compare_path] with it.
fn compare_deps_single_path(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: XvcEntity,
    xvc_path: &XvcPath,
    collected_diffs: &mut Diffs,
) -> Result<()> {
    if let Some(stored) = cmp_params.all_dependencies.get(&stored_dependency_e) {
        if matches!(
            stored,
            XvcDependency::File {
                path: _recorded_path
            }
        ) {
            let actual_xvc_metadata = cmp_params.pmm.get(xvc_path).cloned();
            let oxe_path = cmp_params.xvc_path_store.entity_by_value(xvc_path);

            let stored_xvc_metadata =
                oxe_path.and_then(|xe_path| cmp_params.xvc_metadata_store.get(&xe_path).cloned());
            let diff_xvc_metadata = XvcMetadata::diff(stored_xvc_metadata, actual_xvc_metadata);

            let xe_path = if let Some(xe_path) = oxe_path {
                collected_diffs.insert_xvc_path_diff(xe_path, Diff::Identical);
                xe_path
            } else {
                let xe_path = cmp_params.xvc_root.new_entity();
                collected_diffs.insert_xvc_path_diff(
                    xe_path,
                    Diff::RecordMissing {
                        actual: xvc_path.clone(),
                    },
                );
                xe_path
            };

            collected_diffs.insert_xvc_metadata_diff(xe_path, diff_xvc_metadata.clone());

            let diff = match diff_xvc_metadata.clone() {
                // If there is no change in metadata, we don't check further
                Diff::Identical | Diff::Skipped => Diff::Identical,
                Diff::RecordMissing { .. } => {
                    let text_or_binary = cmp_params
                        .text_files
                        .get(&xe_path)
                        .copied()
                        .unwrap_or_default();
                    let absolute_path = xvc_path.to_absolute_path(&cmp_params.xvc_root);
                    let actual =
                        ContentDigest::new(&absolute_path, *cmp_params.algorithm, text_or_binary)?;
                    Diff::RecordMissing { actual }
                }
                Diff::ActualMissing { .. } => {
                    let record = cmp_params
                        .xvc_digests_store
                        .get(&xe_path)
                        .and_then(|xd| xd.get::<ContentDigest>())
                        .ok_or_else(|| {
                            Error::from(xvc_ecs::Error::CannotFindEntityInStore { entity: xe_path })
                        })?;
                    Diff::ActualMissing { record }
                }
                Diff::Different { .. } => {
                    let record = cmp_params
                        .xvc_digests_store
                        .get(&xe_path)
                        .and_then(|xd| xd.get::<ContentDigest>())
                        .ok_or_else(|| {
                            Error::from(xvc_ecs::Error::CannotFindEntityInStore { entity: xe_path })
                        })?;
                    let text_or_binary = cmp_params
                        .text_files
                        .get(&xe_path)
                        .copied()
                        .unwrap_or_default();
                    let absolute_path = xvc_path.to_absolute_path(&cmp_params.xvc_root);
                    let actual =
                        ContentDigest::new(&absolute_path, *cmp_params.algorithm, text_or_binary)?;
                    if record == actual {
                        Diff::Identical
                    } else {
                        Diff::Different { record, actual }
                    }
                }
            };
            // We collect both path and dependency digest diffs
            collected_diffs.insert_attribute_digest_diff(xe_path, diff.clone());
            collected_diffs.insert_attribute_digest_diff(stored_dependency_e, diff);
            Ok(())
        } else {
            Err(anyhow!("Dependency record is different from called path.").into())
        }
    } else {
        Err(anyhow!("No such stored XvcDependency").into())
    }
}

fn compare_deps_url(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: XvcEntity,
    url: &Url,
    collected_diffs: &mut Diffs,
) -> Result<()> {
    if let Some(stored) = cmp_params.all_dependencies.get(&stored_dependency_e) {
        if !matches!(stored, XvcDependency::Url { url: _ }) {
            return Err(anyhow!("Dependency record is different from called url.").into());
        }

        let actual = UrlHeadDigest::new(url, *cmp_params.algorithm)?;
        let record_xvc_digests = cmp_params.xvc_digests_store.get(&stored_dependency_e);
        let record = record_xvc_digests.and_then(|s| s.get::<UrlHeadDigest>());

        let head_diff = UrlHeadDigest::diff(record, Some(actual));
        collected_diffs.insert_attribute_digest_diff(stored_dependency_e, head_diff.clone());

        let get_diff = match head_diff {
            Diff::Identical | Diff::Skipped => Diff::<UrlGetDigest>::Identical,
            Diff::RecordMissing { .. } => {
                let actual_get_diff = UrlGetDigest::new(url, *cmp_params.algorithm)?;
                Diff::RecordMissing {
                    actual: actual_get_diff,
                }
            }
            Diff::ActualMissing { .. } => {
                let record = record_xvc_digests
                    .and_then(|rec| rec.get::<UrlGetDigest>())
                    .ok_or_else(|| xvc_ecs::Error::CannotFindEntityInStore {
                        entity: stored_dependency_e,
                    })?;
                Diff::ActualMissing { record }
            }

            Diff::Different { record, actual } => {
                if record == actual {
                    // TODO: We may want to force download here with a flag
                    Diff::Identical
                } else {
                    let actual_get_diff = UrlGetDigest::new(url, *cmp_params.algorithm)?;
                    let record = record_xvc_digests
                        .and_then(|rec| rec.get::<UrlGetDigest>())
                        .ok_or_else(|| xvc_ecs::Error::CannotFindEntityInStore {
                            entity: stored_dependency_e,
                        })?;
                    Diff::Different {
                        record,
                        actual: actual_get_diff,
                    }
                }
            }
        };
        collected_diffs.insert_attribute_digest_diff(stored_dependency_e, get_diff)?;
        Ok(())
    } else {
        Err(anyhow!("No such stored XvcDependency").into())
    }
}

fn compare_deps_directory(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: XvcEntity,
    directory: &XvcPath,
    collected_diffs: &mut Diffs,
) -> Result<()> {
    if let Some(stored) = cmp_params.all_dependencies.get(&stored_dependency_e) {
        if !matches!(stored, XvcDependency::Directory { path: _ }) {
            return Err(anyhow!("Dependency directory is different from called path.").into());
        }

        let paths = filter_paths_by_directory(cmp_params.pmm, directory);
        compare_deps_multiple_paths(cmp_params, stored_dependency_e, &paths, collected_diffs)
    } else {
        Err(anyhow!("No such stored XvcDependency").into())
    }
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
    let actual_collection_digest = CollectionDigest::new(paths.keys().cloned(), *algorithm)?;
    let stored_xvc_digests = cmp_params.xvc_digests_store.get(&stored_dependency_e);
    let stored_collection_digest = stored_xvc_digests.and_then(|s| s.get::<CollectionDigest>());

    let collection_digest_diff =
        CollectionDigest::diff(stored_collection_digest, Some(actual_collection_digest));

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
    stored_dependency_e: XvcEntity,
    _glob: &str,
    collected_diffs: &mut Diffs,
) -> Result<()> {
    let stored = &cmp_params.all_dependencies[&stored_dependency_e];
    if let XvcDependency::Glob { glob } = stored {
        let glob_pmm = glob_paths(
            cmp_params.xvc_root,
            cmp_params.pmm,
            cmp_params.pipeline_rundir,
            glob,
        )?;
        compare_deps_multiple_paths(cmp_params, stored_dependency_e, &glob_pmm, collected_diffs)
    } else {
        Err(Error::XvcDependencyComparisonError)
    }
}
