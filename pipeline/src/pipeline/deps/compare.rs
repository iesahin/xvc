use crate::error::{Error, Result};
use crate::XvcEntity;
use anyhow::anyhow;
use log::{debug, info};
use xvc_core::util::file::{directory_paths, glob_paths, XvcPathMetadataMap};
use xvc_core::{
    CollectionDigest, ContentDigest, HashAlgorithm, MetadataDigest, TextOrBinary, XvcFileType,
    XvcMetadata, XvcPath, XvcRoot,
};
use xvc_ecs::{R11Store, R1NStore, XvcStore};

use super::digest::{
    dependency_content_digest, paths_collection_digest, paths_metadata_digest,
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
    pub stored_dependency_paths: &'a R1NStore<XvcDependency, XvcPath>,
    pub stored_path_metadata: &'a R11Store<XvcPath, XvcMetadata>,
    pub stored_path_collection_digest: &'a R11Store<XvcPath, CollectionDigest>,
    pub stored_path_metadata_digest: &'a R11Store<XvcPath, MetadataDigest>,
    pub stored_path_content_digest: &'a R11Store<XvcPath, ContentDigest>,
    pub stored_dependency_collection_digest: &'a R11Store<XvcDependency, CollectionDigest>,
    pub stored_dependency_metadata_digest: &'a R11Store<XvcDependency, MetadataDigest>,
    pub stored_dependency_content_digest: &'a R11Store<XvcDependency, ContentDigest>,
    pub text_files: &'a R11Store<XvcPath, TextOrBinary>,
}

/// The change between stored dependency state and the current state
/// TODO: Refactor this using DeltaFields
#[derive(Clone, Debug)]
pub struct XvcDependencyChange {
    /// Changes in path metadata
    /// There may be multiple paths associated with a dependency
    pub updated_metadata: Option<R11Store<XvcPath, XvcMetadata>>,
    /// Change in collection digest
    pub updated_collection_digest: Option<CollectionDigest>,
    /// Change in content digests
    /// There may be multiple paths associated with a dependency
    pub updated_content_digests: Option<R11Store<XvcPath, ContentDigest>>,
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
    stored_dependency_e: &XvcEntity,
) -> Result<XvcDependencyChange> {
    let stored = &cmp_params.all_dependencies[stored_dependency_e];

    match stored {
        // Pipeline and step dependencies are handled differently
        XvcDependency::Pipeline { .. } => Ok(XvcDependencyChange {
            updated_metadata: None,
            updated_collection_digest: None,
            updated_content_digests: None,
        }),
        XvcDependency::Step { .. } => Ok(XvcDependencyChange {
            updated_metadata: None,
            updated_collection_digest: None,
            updated_content_digests: None,
        }),

        XvcDependency::File { path: _ } => {
            compare_deps_single_path(cmp_params, stored_dependency_e)
        }
        XvcDependency::Glob { glob: _ } => compare_deps_glob(cmp_params, stored_dependency_e),
        XvcDependency::Directory { path: _ } => {
            compare_deps_directory(cmp_params, stored_dependency_e)
        }
        XvcDependency::Url { url: _ } => compare_deps_url(cmp_params, stored_dependency_e),
        XvcDependency::Import { url: _, path: _ } => {
            compare_deps_import(cmp_params, stored_dependency_e)
        }
        XvcDependency::Param {
            format: _,
            path: _,
            key: _,
        } => compare_deps_single_path(cmp_params, stored_dependency_e),
        XvcDependency::Regex { path: _, regex: _ } => {
            compare_deps_single_path(cmp_params, stored_dependency_e)
        }
        XvcDependency::Lines {
            path: _,
            begin: _,
            end: _,
        } => compare_deps_single_path(cmp_params, stored_dependency_e),
    }
}

/// Compares a dependency path with the actual metadata and content digest found on disk
///
/// It loads the dependency, extracts the path and calls [compare_path] with it.
fn compare_deps_single_path(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: &XvcEntity,
) -> Result<XvcDependencyChange> {
    let stored = &cmp_params.all_dependencies[stored_dependency_e];
    let path: XvcPath = stored
        .xvc_path()
        .ok_or(Error::XvcDependencyComparisonError)?;

    let path_comparison = compare_path(&cmp_params, stored_dependency_e, &path)?;

    match path_comparison.updated_metadata {
        // If there is no change in metadata, we don't check further
        None => Ok(path_comparison),
        Some(ref path_metadata_change) => {
            let changed_paths = &path_metadata_change.left;
            assert!(changed_paths.len() == 1);
            let (path_e, path) = changed_paths.iter().next().unwrap();
            let xvc_root = cmp_params.xvc_root;
            let algorithm = cmp_params.algorithm;
            let pipeline_rundir = cmp_params.pipeline_rundir;
            // Check stored content digests
            let stored_path_content_digests = cmp_params.stored_path_content_digest;
            let content_digest = stored_path_content_digests.right.get(path_e);
            let pmm = cmp_params.pmm;
            let dep_digest_params = DependencyDigestParams {
                xvc_root,
                algorithm,
                pipeline_rundir,
                pmm,
            };
            let current_content_digest = dependency_content_digest(&dep_digest_params, stored)?;
            match (content_digest, current_content_digest) {
                // Nothing was recorded and nothing has changed, do nothing
                (None, ContentDigest(None)) => {
                    debug!(
                        "Nothing was recorded and nothing has changed. There may be a bug for {:?}",
                        cmp_params
                    );
                    Ok(path_comparison)
                }
                // Nothing was recorded but we have some digest now
                (None, ContentDigest(Some(current_content_digest))) => {
                    let mut content_change = R11Store::<XvcPath, ContentDigest>::new();
                    content_change.insert(path_e, path.clone(), current_content_digest.into());
                    Ok(XvcDependencyChange {
                        updated_metadata: Some(path_metadata_change.clone()),
                        updated_content_digests: Some(content_change),
                        updated_collection_digest: path_comparison.updated_collection_digest,
                    })
                }

                (Some(_), ContentDigest(None)) => Err(anyhow!(
                    "ContentDigest cannot be calculated. This shouldn't happen."
                )
                .into()),
                // We have a recorded digest and we have a digest now, comparing them
                (Some(content_digest), ContentDigest(Some(_))) => {
                    if *content_digest == current_content_digest {
                        // If no changes in content, we only send the metadata changes
                        // back
                        Ok(path_comparison.clone())
                    } else {
                        let mut content_change = R11Store::<XvcPath, ContentDigest>::new();
                        content_change.insert(
                            path_e,
                            path.clone(),
                            // unwrap is fine here as we know it's Some(_)
                            current_content_digest,
                        );
                        Ok(XvcDependencyChange {
                            updated_metadata: Some(path_metadata_change.clone()),
                            updated_content_digests: Some(content_change),
                            updated_collection_digest: path_comparison.updated_collection_digest,
                        })
                    }
                }
            }
        }
    }
}

/// Compare the record and the actual metadata and content digest of a path
///
///
fn compare_path(
    cmp_params: &DependencyComparisonParams,
    stored_dependency_e: &XvcEntity,
    path: &XvcPath,
) -> Result<XvcDependencyChange> {
    let xvc_root = cmp_params.xvc_root;
    let pmm = cmp_params.pmm;
    let current_md = pmm.get(path);
    let (path_e, o_md) = match cmp_params.stored_path_metadata.entity_by_left(path) {
        None => {
            // There is no previous path information, let's create an entry for the new md
            let path_e = xvc_root.new_entity();
            let md = match current_md {
                // There is no current path either, we create something that exists in records only
                None => XvcMetadata {
                    file_type: xvc_core::XvcFileType::RecordOnly,
                    size: None,
                    modified: None,
                },
                Some(md) => *md,
            };
            (path_e, Some(md))
        }
        // There is some previous path info
        Some(stored_path_e) => match cmp_params
            .stored_path_metadata
            .left_to_right(stored_dependency_e)
        {
            None => {
                // There is no previous metadata information for the path
                let md = match current_md {
                    // There is no current path either, we create something that exists in records only
                    None => XvcMetadata {
                        file_type: xvc_core::XvcFileType::RecordOnly,
                        size: None,
                        modified: None,
                    },
                    Some(md) => *md,
                };
                (stored_path_e, Some(md))
            }
            Some((_, stored_metadata)) => {
                // We found stored metadata, let's check if it's changed
                let o_md = match current_md {
                    None => Some(XvcMetadata {
                        file_type: xvc_core::XvcFileType::RecordOnly,
                        size: None,
                        modified: None,
                    }),
                    Some(md) => {
                        // We always invalidate RecordOnly files
                        if stored_metadata.file_type == XvcFileType::RecordOnly {
                            Some(*md)
                        } else if md.file_type == stored_metadata.file_type
                            && md.size == stored_metadata.size
                            && md.modified == stored_metadata.modified
                        {
                            None
                        } else {
                            Some(*md)
                        }
                    }
                };
                (stored_path_e, o_md)
            }
        },
    };

    match o_md {
        None => Ok(XvcDependencyChange {
            updated_metadata: None,
            updated_collection_digest: None,
            updated_content_digests: None,
        }),
        Some(md) => {
            let mut path_metadata_change = R11Store::<XvcPath, XvcMetadata>::new();
            path_metadata_change.insert(&path_e, path.clone(), md);
            Ok(XvcDependencyChange {
                updated_metadata: Some(path_metadata_change),
                updated_collection_digest: None,
                updated_content_digests: None,
            })
        }
    }
}

fn compare_deps_import(
    _cmp_params: DependencyComparisonParams,
    _stored_dependency_e: &XvcEntity,
) -> Result<XvcDependencyChange> {
    todo!()
}

fn compare_deps_url(
    _cmp_params: DependencyComparisonParams,
    _stored_dependency_e: &XvcEntity,
) -> Result<XvcDependencyChange> {
    todo!()
}

fn compare_deps_directory(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: &XvcEntity,
) -> Result<XvcDependencyChange> {
    let stored = &cmp_params.all_dependencies[stored_dependency_e];
    if let XvcDependency::Directory { path } = stored {
        let pmm = directory_paths(cmp_params.pmm, path);
        compare_deps_multiple_paths(cmp_params, stored_dependency_e, &pmm)
    } else {
        Err(Error::XvcDependencyComparisonError)
    }
}

fn compare_deps_multiple_paths(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: &XvcEntity,
    paths: &XvcPathMetadataMap,
) -> Result<XvcDependencyChange> {
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
    let collection_digest = paths_collection_digest(&dep_digest_params, paths)?;
    let paths_metadata_digest = paths_metadata_digest(&dep_digest_params, paths)?;
    let stored_dependency_metadata_digest = cmp_params
        .stored_dependency_metadata_digest
        .left_to_right(stored_dependency_e);
    let stored_dependency_collection_digest = cmp_params
        .stored_dependency_collection_digest
        .left_to_right(stored_dependency_e);
    let stored_path_content_digest = cmp_params.stored_path_content_digest;
    let stored_path_metadata = cmp_params.stored_path_metadata;
    let path_entity_index = stored_path_metadata.left.index_map()?;

    let mut multipath_change = match stored_dependency_collection_digest {
        // We didn't have any collection digest, this may mean we don't have any
        // content digests either. We may be keeping the content digests from an earlier caching
        // though, so we are not sure if no content digests were calculated for these paths.
        None => XvcDependencyChange {
            updated_collection_digest: Some(collection_digest),
            updated_content_digests: None,
            updated_metadata: None,
        },

        Some((_, stored_dependency_collection_digest)) => {
            match stored_dependency_collection_digest {
                // We have an associated digest but it doesn't contain collection digest
                CollectionDigest(None) => XvcDependencyChange {
                    updated_collection_digest: Some(collection_digest),
                    updated_content_digests: None,
                    updated_metadata: None,
                },
                // We have a digest and it contains a collection digest
                CollectionDigest(Some(_)) => {
                    if *stored_dependency_collection_digest == collection_digest {
                        XvcDependencyChange {
                            updated_collection_digest: None,
                            updated_content_digests: None,
                            updated_metadata: None,
                        }
                    } else {
                        XvcDependencyChange {
                            updated_collection_digest: Some(collection_digest),
                            updated_content_digests: None,
                            updated_metadata: None,
                        }
                    }
                }
            }
        }
    };

    let changed_path_store = || {
        let mut changed_paths = R11Store::<XvcPath, XvcMetadata>::new();
        for (path, md) in pmm {
            match path_entity_index.get(path) {
                None => {
                    info!("Adding to tracked paths: {}", path);
                    changed_paths.insert(&xvc_root.new_entity(), path.clone(), *md);
                }
                Some(entity) => match stored_path_metadata.left_to_right(entity) {
                    None => {
                        changed_paths.insert(entity, path.clone(), *md);
                    }

                    Some((_, stored_md)) => {
                        if stored_md != md {
                            changed_paths.insert(entity, path.clone(), *md);
                        }
                    }
                },
            }
        }
        if changed_paths.left.is_empty() {
            None
        } else {
            Some(changed_paths)
        }
    };

    // if the collection has changed, we consider all metadata invalid and changed. otherwise check the individual metadata.
    multipath_change.updated_metadata = match multipath_change.updated_collection_digest {
        Some(_) => {
            // TODO: Do we really need this?
            info!("Collection elements has changed, recalculating all metadata");
            let mut changed_paths = R11Store::<XvcPath, XvcMetadata>::new();
            for (path, md) in pmm.iter() {
                if md.file_type == XvcFileType::File {
                    match path_entity_index.get(path) {
                        None => {
                            info!("Adding to tracked paths: {}", path);
                            changed_paths.insert(&xvc_root.new_entity(), path.clone(), *md);
                        }
                        Some(entity) => changed_paths.insert(entity, path.clone(), *md),
                    }
                } else {
                    info!(
                        "Skipping metadata tracking for {} for having type: {}",
                        path, md.file_type
                    );
                }
            }
            Some(changed_paths)
        }
        None => match stored_dependency_metadata_digest {
            // We don't have a metadata digest before
            None => changed_path_store(),
            Some((_, metadata_digest)) => {
                if *metadata_digest == paths_metadata_digest {
                    None
                } else {
                    changed_path_store()
                }
            }
        },
    };

    // If path_metadata_change is not None, we should check the contents to decide if the content has changed.

    multipath_change.updated_content_digests = match multipath_change.updated_metadata {
        None => None,
        Some(ref path_md_store) => {
            // for each path with changed metadata, we check whether the content has changed
            let mut changed_digests = R11Store::<XvcPath, ContentDigest>::new();
            for (path_e, path) in path_md_store.left.iter() {
                let dep_digest_params = DependencyDigestParams {
                    xvc_root,
                    algorithm,
                    pipeline_rundir,
                    pmm,
                };
                let current_digest = xvc_path_content_digest(&dep_digest_params, path)?;
                match stored_path_content_digest.left_to_right(path_e) {
                    None => {
                        // We don't use metadata digest for individual files as the metadata itself is easier to compare than the digest
                        changed_digests.insert(path_e, path.clone(), current_digest);
                    }
                    Some((_, content_digest)) => match content_digest {
                        ContentDigest(None) => {
                            changed_digests.insert(path_e, path.clone(), current_digest);
                        }
                        ContentDigest(Some(_)) => {
                            if *content_digest != current_digest {
                                changed_digests.insert(path_e, path.clone(), current_digest);
                            }
                        }
                    },
                }
            }

            if changed_digests.left.is_empty() {
                None
            } else {
                Some(changed_digests)
            }
        }
    };

    Ok(multipath_change)
}

/// Compares two globs, one stored and one current.
///
/// Uses [compare_deps_multiple_paths] after extracting the paths with [glob_paths]
fn compare_deps_glob(
    cmp_params: DependencyComparisonParams,
    stored_dependency_e: &XvcEntity,
) -> Result<XvcDependencyChange> {
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
