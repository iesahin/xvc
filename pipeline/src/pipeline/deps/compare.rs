

use crate::error::{Error, Result};
use crate::XvcEntity;
use anyhow::anyhow;


use subprocess::Exec;
use url::Url;
use xvc_core::types::diff::Diffable;
use xvc_core::util::file::{filter_paths_by_directory, glob_paths, XvcPathMetadataMap};
use xvc_core::{
    CollectionDigest, ContentDigest, Diff, DiffStore, HashAlgorithm, StdoutDigest,
    TextOrBinary, UrlGetDigest, UrlHeadDigest, XvcDigests, XvcMetadata, XvcMetadataDigest, XvcPath,
    XvcRoot,
};
use xvc_ecs::{HStore, R1NStore, XvcStore};



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
) -> Result<HStore<DigestDiff>> {
    let stored = &cmp_params.all_dependencies[&stored_dependency_e];

    match stored {
        // Step dependencies are handled differently
        XvcDependency::Step { .. } => Ok(HStore::new()),

        XvcDependency::Generic { generic_command } => {
            compare_deps_generic(cmp_params, stored_dependency_e, generic_command)
        }
        XvcDependency::File { path } => {
            compare_deps_single_path(cmp_params, stored_dependency_e, path)
        }
        XvcDependency::Glob { glob } => compare_deps_glob(cmp_params, stored_dependency_e, glob),
        XvcDependency::Directory { path } => {
            compare_deps_directory(cmp_params, stored_dependency_e, path)
        }
        XvcDependency::Url { url } => compare_deps_url(cmp_params, stored_dependency_e, url),
        XvcDependency::Param {
            path,
            format: _,
            key: _,
        } => compare_deps_single_path(cmp_params, stored_dependency_e, path),
        XvcDependency::Regex { path, regex: _ } => {
            compare_deps_single_path(cmp_params, stored_dependency_e, path)
        }
        XvcDependency::Lines {
            path,
            begin: _,
            end: _,
        } => compare_deps_single_path(cmp_params, stored_dependency_e, path),
    }
}

/// Runs the command and compares the output with the stored dependency
fn compare_deps_generic(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: XvcEntity,
    generic_command: &str,
) -> Result<HStore<DigestDiff>> {
    let command_output = Exec::shell(generic_command).capture()?;
    let stdout = String::from_utf8(command_output.stdout)?;
    let stderr = String::from_utf8(command_output.stderr)?;
    let algorithm = *cmp_params.algorithm;
    let return_code = command_output.exit_status;
    if stderr.len() > 0 || !return_code.success() {
        return Err(Error::ProcessError { stdout, stderr });
    }

    let actual = StdoutDigest::new(&stdout, algorithm).into();

    if let Some(xvc_digests) = cmp_params.xvc_digests_store.get(&stored_dependency_e) {
        if let Some(record) = xvc_digests.get::<StdoutDigest>() {
            if record == actual {
                Ok((stored_dependency_e, DigestDiff::Identical).into())
            } else {
                let record = record.into();
                let actual = actual.into();
                Ok((
                    stored_dependency_e,
                    DigestDiff::Different { record, actual },
                )
                    .into())
            }
        } else {
            let actual = actual.into();
            Ok((stored_dependency_e, DigestDiff::RecordMissing { actual }).into())
        }
    } else {
        let actual = actual.into();
        Ok((stored_dependency_e, DigestDiff::RecordMissing { actual }).into())
    }
}

/// Compares a dependency path with the actual metadata and content digest found on disk
///
/// It loads the dependency, extracts the path and calls [compare_path] with it.
fn compare_deps_single_path(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: XvcEntity,
    xvc_path: &XvcPath,
) -> Result<HStore<DigestDiff>> {
    if let Some(stored) = cmp_params.all_dependencies.get(&stored_dependency_e) {
        if !matches!(stored, XvcDependency::File { path: _ }) {
            return Err(
                anyhow!("Dependency record is different from called path. Please report.").into(),
            );
        }
        let actual_xvc_metadata = cmp_params.pmm.get(xvc_path).cloned();
        let xe_path = cmp_params
            .xvc_path_store
            .entity_by_value(xvc_path)
            .expect("Missing XvcPath Entity");
        let stored_xvc_metadata = cmp_params
            .xvc_metadata_store
            .get(&xe_path)
            .cloned()
            .expect("Missing XvcMetadata");
        let diff_xvc_metadata = XvcMetadata::diff(actual_xvc_metadata, Some(stored_xvc_metadata));

        let diff = match diff_xvc_metadata {
            // If there is no change in metadata, we don't check further
            Diff::Identical | Diff::Skipped => Diff::Identical,
            Diff::RecordMissing { actual: _ } => {
                let text_or_binary = cmp_params
                    .text_files
                    .get(&xe_path)
                    .copied()
                    .unwrap_or_default();
                let absolute_path = xvc_path.to_absolute_path(&cmp_params.xvc_root);
                let actual =
                    ContentDigest::new(&absolute_path, *cmp_params.algorithm, text_or_binary)?
                        .into();
                Diff::RecordMissing { actual }
            }
            Diff::ActualMissing { record: _ } => {
                let record = cmp_params
                    .xvc_digests_store
                    .get(&xe_path)
                    .cloned()
                    .ok_or_else(|| {
                        Error::from(xvc_ecs::Error::CannotFindEntityInStore { entity: xe_path })
                    })?
                    .into();
                Diff::ActualMissing { record }
            }
            Diff::Different { record: _, actual: _ } => {
                let record = cmp_params
                    .xvc_digests_store
                    .get(&xe_path)
                    .cloned()
                    .ok_or_else(|| {
                        Error::from(xvc_ecs::Error::CannotFindEntityInStore { entity: xe_path })
                    })?
                    .into();
                let text_or_binary = cmp_params
                    .text_files
                    .get(&xe_path)
                    .copied()
                    .unwrap_or_default();
                let absolute_path = xvc_path.to_absolute_path(&cmp_params.xvc_root);
                let actual =
                    ContentDigest::new(&absolute_path, *cmp_params.algorithm, text_or_binary)?
                        .into();
                if record == actual {
                    Diff::Identical
                } else {
                    Diff::Different { record, actual }
                }
            }
        };
        Ok((stored_dependency_e, diff).into())
    } else {
        Err(anyhow!("No such stored XvcDependency").into())
    }
}

fn compare_deps_url(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: XvcEntity,
    url: &Url,
) -> Result<HStore<DigestDiff>> {
    if let Some(stored) = cmp_params.all_dependencies.get(&stored_dependency_e) {
        if !matches!(stored, XvcDependency::Url { url: _ }) {
            return Err(anyhow!("Dependency record is different from called url.").into());
        }

        let actual = UrlHeadDigest::new(url, *cmp_params.algorithm)
            .map_err(|e| e.warn())
            .ok();
        let record_xvc_digests = cmp_params.xvc_digests_store.get(&stored_dependency_e);
        let record = record_xvc_digests.and_then(|s| s.get::<UrlHeadDigest>());

        let head_diff = UrlHeadDigest::diff(record, actual);

        let diff = match head_diff {
            Diff::Identical | Diff::Skipped => Ok(Diff::Identical),
            Diff::RecordMissing { actual } => {
                let actual_get_diff = UrlGetDigest::new(url, *cmp_params.algorithm)?;
                let mut actual_xvc_digests = XvcDigests::new();
                actual_xvc_digests.insert(actual);
                actual_xvc_digests.insert(actual_get_diff);
                Ok(Diff::RecordMissing {
                    actual: actual_xvc_digests,
                })
            }
            Diff::ActualMissing {
                record: _record_head_digest,
            } => Ok(Diff::ActualMissing {
                record: record_xvc_digests.cloned().unwrap(),
            }),
            Diff::Different { record, actual } => {
                if record == actual {
                    // TODO: We may want to force download here with a flag
                    Ok(Diff::Identical)
                } else {
                    let actual_get_diff = UrlGetDigest::new(url, *cmp_params.algorithm)?;
                    let mut actual_xvc_digests = XvcDigests::new();
                    actual_xvc_digests.insert(actual);
                    actual_xvc_digests.insert(actual_get_diff);
                    Ok(Diff::Different {
                        record: record_xvc_digests.cloned().unwrap(),
                        actual: actual_xvc_digests,
                    })
                }
            }
        };

        diff.map(|diff| (stored_dependency_e, diff).into())
    } else {
        Err(anyhow!("No such stored XvcDependency").into())
    }
}

fn compare_deps_directory(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: XvcEntity,
    directory: &XvcPath,
) -> Result<HStore<DigestDiff>> {
    if let Some(stored) = cmp_params.all_dependencies.get(&stored_dependency_e) {
        if !matches!(stored, XvcDependency::File { path: _ }) {
            return Err(anyhow!("Dependency directory is different from called path.").into());
        }

        let pmm = filter_paths_by_directory(cmp_params.pmm, directory);
        compare_deps_multiple_paths(cmp_params, stored_dependency_e, &pmm)
    } else {
        Err(anyhow!("No such stored XvcDependency").into())
    }
}

fn compare_deps_multiple_paths(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: XvcEntity,
    paths: &XvcPathMetadataMap,
) -> Result<HStore<DigestDiff>> {
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
    let actual_collection_digest = CollectionDigest::new(paths, *algorithm)
        .map_err(|e| e.warn())
        .ok();
    let stored_xvc_digests = cmp_params.xvc_digests_store.get(&stored_dependency_e);
    let stored_collection_digest = stored_xvc_digests.and_then(|s| s.get::<CollectionDigest>());

    let collection_digest_diff =
        CollectionDigest::diff(stored_collection_digest, actual_collection_digest);

    let mut xvc_metadata_diffs = paths
        .iter()
        .map(|(path, md)| {
            let actual_xvc_metadata_digest = XvcMetadataDigest::new(md).unwrap();
            let xe = cmp_params
                .xvc_path_store
                .entity_by_value(path)
                .ok_or(anyhow!("Cannot find XvcEntity for path {}.", path))
                .unwrap();
            let recorded_xvc_metadata_digest = cmp_params
                .xvc_digests_store
                .get(&xe)
                .and_then(|xvc_digests| xvc_digests.get::<XvcMetadataDigest>());

            let xvc_metadata_diff = XvcMetadataDigest::diff(
                recorded_xvc_metadata_digest,
                Some(actual_xvc_metadata_digest),
            );
            (xe, xvc_metadata_diff)
        })
        .collect::<DiffStore<XvcMetadataDigest>>();

    // If neither collection, nor the metadata of any path has changed, we can skip the rest of the comparison
    if collection_digest_diff == Diff::Identical
        && xvc_metadata_diffs.values().all(|x| *x == Diff::Identical)
    {
        // The return map will contain Skipped values for all paths.
        let xvc_digests = xvc_metadata_diffs
            .drain()
            .map(|(xe, _xmd)| (xe, Diff::Skipped))
            .collect::<HStore<DigestDiff>>();
        Ok(xvc_digests)
    } else {
        let mut digest_diffs = xvc_metadata_diffs
            .iter()
            .map(|(xe, xvc_metadata_diff)| {
                let path = (cmp_params
                    .xvc_path_store
                    .get(xe)
                    .ok_or(anyhow!("Cannot find XvcPath for XvcEntity {}.", xe)))
                .unwrap();
                let content_digest_diff = match xvc_metadata_diff.clone() {
                    Diff::Identical => Diff::Identical,
                    Diff::Skipped => Diff::Skipped,
                    Diff::RecordMissing { actual } => {
                        let text_or_binary =
                            cmp_params.text_files.get(&xe).copied().unwrap_or_default();
                        let actual_content_digest = ContentDigest::new(
                            &path.to_absolute_path(&xvc_root),
                            *algorithm,
                            text_or_binary,
                        )
                        .unwrap();
                        let mut actual: XvcDigests = actual.into();
                        actual.insert(actual_content_digest);
                        Diff::RecordMissing { actual }
                    }
                    Diff::ActualMissing { record } => {
                        let recorded_content_digest = cmp_params
                            .xvc_digests_store
                            .get(&xe)
                            .and_then(|xvc_digests| xvc_digests.get::<ContentDigest>())
                            .unwrap();
                        let mut record: XvcDigests = record.into();
                        record.insert(recorded_content_digest);
                        Diff::ActualMissing { record }
                    }
                    Diff::Different { .. } => {
                        let text_or_binary =
                            cmp_params.text_files.get(&xe).copied().unwrap_or_default();
                        let actual_content_digest = ContentDigest::new(
                            &path.to_absolute_path(xvc_root),
                            *algorithm,
                            text_or_binary,
                        )
                        .map_err(|e| e.panic())
                        .unwrap();
                        let recorded_content_digest = cmp_params
                            .xvc_digests_store
                            .get(&xe)
                            .and_then(|xvc_digests| xvc_digests.get::<ContentDigest>());
                        match ContentDigest::diff(
                            recorded_content_digest,
                            Some(actual_content_digest),
                        ) {
                            Diff::Identical => Diff::Identical,
                            Diff::Skipped => Diff::Skipped,
                            Diff::RecordMissing { actual } => Diff::RecordMissing {
                                actual: actual.into(),
                            },
                            Diff::ActualMissing { record } => Diff::ActualMissing {
                                record: record.into(),
                            },
                            Diff::Different { record, actual } => {
                                let record: XvcDigests = record.into();
                                let actual: XvcDigests = actual.into();
                                Diff::Different { record, actual }
                            }
                        }
                    }
                };
                (*xe, content_digest_diff)
            })
            .collect::<HStore<Diff<XvcDigests>>>();

        assert!(!digest_diffs.contains_key(&stored_dependency_e));
        let dep_xvc_digests = match collection_digest_diff {
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
        };
        digest_diffs.insert(stored_dependency_e, dep_xvc_digests);
        Ok(digest_diffs)
    }
}

/// Compares two globs, one stored and one current.
///
/// Uses [compare_deps_multiple_paths] after extracting the paths with [glob_paths]
fn compare_deps_glob(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: XvcEntity,
    _glob: &str,
) -> Result<HStore<DigestDiff>> {
    let stored = &cmp_params.all_dependencies[&stored_dependency_e];
    if let XvcDependency::Glob { glob } = stored {
        let glob_pmm = glob_paths(
            cmp_params.xvc_root,
            cmp_params.pmm,
            cmp_params.pipeline_rundir,
            glob,
        )?;
        compare_deps_multiple_paths(cmp_params, stored_dependency_e, &glob_pmm)
    } else {
        Err(Error::XvcDependencyComparisonError)
    }
}
