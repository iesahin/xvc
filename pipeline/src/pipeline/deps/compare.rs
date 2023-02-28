use std::cmp;

use crate::error::{Error, Result};
use crate::XvcEntity;
use serde::__private::de::Content;
use url::Url;
use xvc_core::types::diff::Diffable;
use xvc_file;
use anyhow::anyhow;
use log::{debug, info};
use subprocess::Exec;
use xvc_core::types::digest_kind::KindfulDigest;
use xvc_core::types::xvcdigest::DigestKind;
use xvc_core::util::file::{filter_paths_by_directory, glob_paths, XvcPathMetadataMap};
use xvc_core::util::store;
use xvc_core::{
    CollectionDigest, ContentDigest, Diff, HashAlgorithm, MetadataDigest, TextOrBinary, XvcDigest,
    XvcDigests, XvcFileType, XvcMetadata, XvcPath, XvcRoot, StdoutDigest, UrlGetDigest, UrlHeadDigest, AttributeDigest, XvcMetadataDigest,
};
use xvc_ecs::{R11Store, R1NStore, XvcStore};
use xvc_file::compare::diff_xvc_path_metadata;

use super::digest::{
    dependency_content_digest, paths_collection_digest, actual_paths_metadata_digest,
    xvc_path_content_digest, DependencyDigestParams,
};

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
    stored_dependency_e: &XvcEntity,
) -> Result<DigestDiff> {
    let stored = &cmp_params.all_dependencies[stored_dependency_e];

    match stored {
        // Step dependencies are handled differently
        XvcDependency::Step { .. } => Ok(XvcDependencyDiff {
            metadata_diff: None,
            updated_collection_digest: None,
            updated_content_digests: None,
        }),

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
        XvcDependency::Import { url, path } => {
            compare_deps_import(cmp_params, stored_dependency_e, url, path)
        }
        XvcDependency::Param {
            path,
            format: _,
            key: _,
        } => compare_deps_single_path(cmp_params, stored_dependency_e, path),
        XvcDependency::Regex { path, regex } => {
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
) -> Result<DigestDiff> {

    let command_output = Exec::shell(generic_command).capture()?;
    let stdout = String::from_utf8(command_output.stdout)?;
    let stderr = String::from_utf8(command_output.stderr)?;
    let return_code = command_output.exit_code;
    if stderr.len() > 0 {
        return Err(Error::ProcessError { stdout, stderr });
    }

    let actual = StdoutDigest::new(&stdout, algorithm).into();

    if let Some(xvc_digests) = cmp_params.xvc_digests.get(&stored_dependency_e) {
        if let Some(record) =  digests.get(actual.attribute()) {
        if record == actual {
            Ok(DigestDiff::Identical)
        } else {
            Ok(DigestDiff::Different { record, actual })
        }
    } else {
        Ok(DigestDiff::RecordMissing { actual })
    }
} else {
    Ok(DigestDiff::RecordMissing { actual })
}
}

/// Compares a dependency path with the actual metadata and content digest found on disk
///
/// It loads the dependency, extracts the path and calls [compare_path] with it.
fn compare_deps_single_path(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: &XvcEntity,
    xvc_path: &XvcPath,
) -> Result<DigestDiff> {
    if let Some(stored) = cmp_params.all_dependencies.get(stored_dependency_e) {
       if !matches!(stored, XvcDependency::File { path }) {
            if path != xvc_path {
                return Err(anyhow!("Dependency record is different from called path. Please report."));
            }
        }
        let actual_xvc_metadata = cmp_params.pmm.get(xvc_path).cloned();
        let xe_path = cmp_params.xvc_path_store.entity_by_value(xvc_path).expect("Missing XvcPath Entity");
        let stored_xvc_metadata = cmp_params.xvc_metadata_store.get(&xe_path).cloned().expect("Missing XvcMetadata");
        let diff_xvc_metadata = XvcMetadata::diff(actual_xvc_metadata, Some(stored_xvc_metadata));

        match diff_xvc_metadata {
            // If there is no change in metadata, we don't check further
            Diff::Identical | Diff::Skipped => Ok(Diff::Identical),
            Diff::RecordMissing { actual } => {
                let text_or_binary = cmp_params.text_files.get(&xe_path).unwrap_or_default();
                let actual = ContentDigest::new(&xvc_path.to_path(cmp_params.xvc_root), cmp_params.algorithm, text_or_binary)?;
                Ok(Diff::RecordMissing { actual })
            },
            Diff::ActualMissing { record } =>  {
                let record = cmp_params.xvc_digests_store.get(&xe_path).cloned();
                Ok(Diff::ActualMissing { record })
            }
            Diff::Different { record, actual } => {
                let record = cmp_params.xvc_digests_store.get(&xe_path).cloned();
                let text_or_binary = cmp_params.text_files.get(&xe_path).unwrap_or_default();
                let actual = ContentDigest::new(&xvc_path.to_path(cmp_params.xvc_root), cmp_params.algorithm, text_or_binary)?;
                if record == actual {
                    Ok(Diff::Identical)
                } else {
                    Ok(Diff::Different { record, actual })
                }
        }
    }

   } else {
    Err(anyhow!("No such stored XvcDependency"))
   }
}

fn compare_deps_url(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: &XvcEntity,
    url: &Url,
) -> Result<DigestDiff> {
    if let Some(stored) = cmp_params.all_dependencies.get(stored_dependency_e) {
       if !matches!(stored, XvcDependency::Url { url: recorded_url }) {
            if url != recorded_url {
                return Err(anyhow!("Dependency record is different from called url. {recorded_url} != {url}."));
            }
        }

        let actual = UrlHeadDigest::new(url, cmp_params.algorithm)?;
        let recorded_xvc_digests= cmp_params.xvc_digests_store.get(stored_dependency_e).unwrap_or_default();
        let record: Option<UrlHeadDigest> = recorded_xvc_digests.get<UrlHeadDigest>().cloned().into();

        let head_diff = UrlHeadDigest::diff(record, actual);

        match head_diff {
            Diff::Identical | Diff::Skipped => Ok(Diff::Identical),
            Diff::RecordMissing { actual } => {
                let actual_get_diff = UrlGetDigest::new(url, cmp_params.algorithm)?;
                actual.insert(actual_get_diff.attribute(), actual);
                Ok(Diff::RecordMissing { actual })
            }
            Diff::ActualMissing { record } => {
                let record_get_diff = recorded_xvc_digests.get<UrlGetDigest>().cloned().into();
                record.insert(record_get_diff.attribute(), record);
                Ok(Diff::ActualMissing { record })
            },
            Diff::Different { record, actual } => {
                if record == actual {
                    /// TODO: We may want to force download here with a flag
                    Ok(Diff::Identical)
                } else {
                    let actual_get_diff = UrlGetDigest::new(url, cmp_params.algorithm)?;
                    actual.insert(actual_get_diff.attribute(), actual);
                    let record_get_diff = recorded_xvc_digests.get<UrlGetDigest>().cloned().into();
                    record.insert(record_get_diff.attribute(), record);
                    Ok(Diff::Different { record, actual })
                }
            }
        }
} else {
    Err(anyhow!("No such stored XvcDependency"))
}
}

fn compare_deps_directory(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: &XvcEntity,
    directory: &XvcPath,
) -> Result<DigestDiff> {
    if let Some(stored) = cmp_params.all_dependencies.get(stored_dependency_e) {
       if !matches!(stored, XvcDependency::File { path }) {
            if path != xvc_path {
                return Err(anyhow!("Dependency directory is different from called path. {stored} != {directory}"));
            }
        }

      let pmm = filter_paths_by_directory(cmp_params.pmm, directory);
      compare_deps_multiple_paths(cmp_params, stored_dependency_e, &pmm)

     } else {
         Err(anyhow!("No such stored XvcDependency"))
    }
}

fn compare_deps_multiple_paths(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: &XvcEntity,
    paths: &XvcPathMetadataMap,
) -> Result<HStore<DigestDiff>> {
    let xvc_root = cmp_params.xvc_root;
    let algorithm = cmp_params.algorithm;
    let pmm = cmp_params.pmm;
    let pipeline_rundir = cmp_params.pipeline_rundir;
    let dep_digest_params = DependencyDigestParams {
        xvc_root,
        algorithm,
        pipeline_rundir,
        pmm,
    };
    let actual_collection_digest = CollectionDigest::new(paths, algorithm)?;
    let stored_collection_digest = stored_digests.get(actual_collection_digest.attribute()).cloned().into();

    let collection_digest_diff = CollectionDigest::diff(stored_collection_digest, actual_collection_digest);

    let xvc_metadata_diffs = paths.iter().map(|(path, md)| {
        let actual_xvc_metadata_digest = XvcMetadataDigest::new(md)?;
        let xe = cmp_params.xvc_path_store.entity_by_value(path).ok_or(anyhow!("Cannot find XvcEntity for path {}.", path))?;
        let recorded_xvc_metadata_digest = cmp_params.xvc_digests_store.get(&xe).map(|xvc_digests| xvc_digests.get<XvcMetadataDigest>().unwrap_or(None)).cloned();
        let xvc_metadata_diff = XvcMetadataDigest::diff(recorded_xvc_metadata_digest, Some(actual_xvc_metadata_digest));
        Ok((xe, xvc_metadata_diff))
    }).collect::<DiffStore<XvcMetadataDigest>>();

    // If neither collection, nor the metadata of any path has changed, we can skip the rest of the comparison
    if collection_digest_diff == Diff::Identical && xvc_metadata_diffs.values().all(|x| x == Diff::Identical) {
        Ok(Diff::Identical)
    } else {
        let content_digest_diffs = xvc_metadata_diffs.iter().map(|(xe, xvc_metadata_diff)| {
            let path = xvc_path_store.get(xe).ok_or(anyhow!("Cannot find XvcPath for XvcEntity {}.", xe))?;
            let content_digest_diff = match xvc_metadata_diff {
                Diff::Identical => Diff::Identical,
                Diff::Skipped => Diff::Skipped,
                Diff::RecordMissing { actual } => {
                    let text_or_binary = cmp_params.text_files.get(&xe).unwrap_or_default();
                    let actual_content_digest = ContentDigest::new(path, algorithm, text_or_binary)?;
                    actual.insert(actual_content_digest.attribute(), actual_content_digest);
                    Diff::RecordMissing { actual }
                }
                Diff::ActualMissing { record } => {
                    let recorded_content_digest = cmp_params.xvc_digests_store.get(&xe).map(|xvc_digests| xvc_digests.get<ContentDigest>().unwrap_or(None)).cloned();
                    record.insert(recorded_content_digest.attribute(), recorded_content_digest);
                    Diff::ActualMissing { record }
                }
                Diff::Different { record, actual } => {
                    let text_or_binary = cmp_params.text_files.get(&xe).unwrap_or_default();
                    let actual_content_digest = ContentDigest::new(path, algorithm, text_or_binary)?;
                    let recorded_content_digest = cmp_params.xvc_digests_store.get(&xe).map(|xvc_digests| xvc_digests.get<ContentDigest>().unwrap_or(None)).cloned();
                    actual.insert(actual_content_digest.attribute(), actual_content_digest);
                    record.insert(recorded_content_digest.attribute(), recorded_content_digest);
                    if record == actual {
                        Diff::Identical
                    } else {
                        Diff::Different { record, actual }
                    }
                }
            };
            (xe, content_digest_diff)
        }).collect::<HStore<Diff<ContentDigest>>>();

        let collection_content_digest = content_digest_diff.iter().sorted().map(|(_, content_digest_diff)| {
            match content_digest_diff {

            }
        });

        // Calculate digests for the collection
        let mut digest_diff_store = DiffStore<XvcDigests>::new();
        let mut collection_diffs: XvcDigests = collection_digest_diff.into();


    }


    Ok(multipath_change)
}

/// Compares two globs, one stored and one current.
///
/// Uses [compare_deps_multiple_paths] after extracting the paths with [glob_paths]
fn compare_deps_glob(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: &XvcEntity,
    glob: &str,
) -> Result<XvcDependencyDiff> {
    let stored = &cmp_params.all_dependencies[stored_dependency_e];
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
