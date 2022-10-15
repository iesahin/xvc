use crate::{Error, Result};
use crossbeam_channel::{bounded, Sender};
use dashmap::DashMap;
use log::{info, warn};
use rayon::iter::{FromParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::{BTreeMap, HashMap};
use std::fs::Metadata;
use xvc_config::FromConfigKey;
use xvc_core::types::xvcdigest::{CollectionDigest, ContentDigest, MetadataDigest, DIGEST_LENGTH};
use xvc_core::util::file::path_metadata_channel;

use xvc_core::{
    CacheType, Error as CoreError, HashAlgorithm, Result as CoreResult, XvcDigest, XvcMetadata,
    XvcPath, XvcPathMetadataMap, XvcRoot,
};
use xvc_core::{XvcFileType, CHANNEL_BOUND};
use xvc_ecs::{Error as EcsError, HStore, R11Store, XvcEntity, XvcStore};
use xvc_logging::watch;
use xvc_walker::{check_ignore, IgnoreRules, MatchResult, PathMetadata};

use crate::track::DataTextOrBinary;

#[derive(Debug)]
pub struct PathComparisonParams {
    pub xvc_path_store: XvcStore<XvcPath>,
    pub xvc_path_imap: BTreeMap<XvcPath, XvcEntity>,
    pub xvc_metadata_store: XvcStore<XvcMetadata>,
    pub content_digest_store: XvcStore<ContentDigest>,
    pub metadata_digest_store: XvcStore<MetadataDigest>,
    pub collection_digest_store: XvcStore<CollectionDigest>,
    pub cache_type_store: XvcStore<CacheType>,
    pub text_or_binary_store: XvcStore<DataTextOrBinary>,
    pub algorithm: HashAlgorithm,
}

impl PathComparisonParams {
    pub fn init(xvc_root: &XvcRoot) -> Result<Self> {
        let conf = xvc_root.config();
        let algorithm = HashAlgorithm::from_conf(conf);
        let xvc_path_store = xvc_root.load_store::<XvcPath>()?;
        let xvc_path_imap = xvc_path_store.index_map()?;
        let xvc_metadata_store = xvc_root.load_store::<XvcMetadata>()?;
        let metadata_digest_store = xvc_root.load_store::<MetadataDigest>()?;
        let collection_digest_store = xvc_root.load_store::<CollectionDigest>()?;
        let content_digest_store = xvc_root.load_store::<ContentDigest>()?;
        let cache_type_store = xvc_root.load_store::<CacheType>()?;
        let text_or_binary_store = xvc_root.load_store::<DataTextOrBinary>()?;

        Ok(Self {
            algorithm,
            xvc_path_store,
            xvc_path_imap,
            xvc_metadata_store,
            cache_type_store,
            collection_digest_store,
            content_digest_store,
            metadata_digest_store,
            text_or_binary_store,
        })
    }
}

/// Shows which information is identical, missing or different in diff calculations.
/// If we have a new path, it means RecordMissing.
/// If we have a record, but don't have the path on file system, it's ActualMissing.
/// If both are present but differ, it's the Difference.
/// Otherwise they are identical.
/// In some operations, some fields are not compared.
/// In this case, they get the value Skipped.
#[derive(Debug, PartialEq, Eq)]
pub enum DeltaField<T> {
    Identical,
    RecordMissing { actual: T },
    ActualMissing { record: T },
    Different { record: T, actual: T },
    Skipped,
}

/// Possible changes that could occur on a file path
#[derive(Debug)]
pub struct FileDelta {
    pub delta_md: DeltaField<XvcMetadata>,
    pub delta_content_digest: DeltaField<ContentDigest>,
    pub delta_metadata_digest: DeltaField<MetadataDigest>,
    pub delta_cache_type: DeltaField<CacheType>,
    pub delta_text_or_binary: DeltaField<DataTextOrBinary>,
}

impl FileDelta {
    /// Returns whether the denoted path has changed, by checking whether fields are None
    pub fn shows_change(&self) -> bool {
        !(self.delta_md == DeltaField::Identical
            && self.delta_cache_type == DeltaField::Identical
            && self.delta_metadata_digest == DeltaField::Identical
            && self.delta_content_digest == DeltaField::Identical
            && self.delta_text_or_binary == DeltaField::Identical)
    }
}

pub type FileDeltaStore = HStore<FileDelta>;

/// Possible changes that could occur on a directory
#[derive(Debug)]
pub struct DirectoryDelta {
    /// Changes in recorded [XvcMetadata]
    pub delta_xvc_metadata: DeltaField<XvcMetadata>,
    /// This is calculated using the sorted collection of [XvcPath] names a directory contains.
    pub delta_collection_digest: DeltaField<CollectionDigest>,
    /// The difference in the merged metadata digests.
    pub delta_metadata_digest: DeltaField<MetadataDigest>,
    /// The difference in the merged content digests.
    pub delta_content_digest: DeltaField<ContentDigest>,
}

impl DirectoryDelta {
    /// Returns whether the denoted path has changed, by checking whether fields are None
    pub fn shows_change(&self) -> bool {
        !(self.delta_collection_digest == DeltaField::Identical
            && self.delta_collection_digest == DeltaField::Identical
            && self.delta_collection_digest == DeltaField::Identical)
    }
}

pub type DirectoryDeltaStore = HStore<DirectoryDelta>;

pub fn update_path_comparison_params_with_actual_info(
    mut pcp: PathComparisonParams,
    file_delta_store: &FileDeltaStore,
) -> PathComparisonParams {
    for (xe, fd) in file_delta_store.map.iter() {
        match fd.delta_md {
            DeltaField::RecordMissing { actual } => {
                pcp.xvc_metadata_store.insert(*xe, actual);
            }
            DeltaField::Different { actual, .. } => {
                pcp.xvc_metadata_store.insert(*xe, actual);
            }
            _ => {}
        }

        match fd.delta_metadata_digest {
            DeltaField::RecordMissing { actual } => {
                pcp.metadata_digest_store.insert(*xe, actual);
            }
            DeltaField::Different { actual, .. } => {
                pcp.metadata_digest_store.insert(*xe, actual);
            }
            _ => {}
        }

        match fd.delta_content_digest {
            DeltaField::RecordMissing { actual } => {
                pcp.content_digest_store.insert(*xe, actual);
            }
            DeltaField::Different { actual, .. } => {
                pcp.content_digest_store.insert(*xe, actual);
            }
            _ => {}
        }
        match fd.delta_cache_type {
            DeltaField::RecordMissing { actual } => {
                pcp.cache_type_store.insert(*xe, actual);
            }
            DeltaField::Different { actual, .. } => {
                pcp.cache_type_store.insert(*xe, actual);
            }
            _ => {}
        }
        match fd.delta_text_or_binary {
            DeltaField::RecordMissing { actual } => {
                pcp.text_or_binary_store.insert(*xe, actual);
            }
            DeltaField::Different { actual, .. } => {
                pcp.text_or_binary_store.insert(*xe, actual);
            }
            _ => {}
        }
    }

    pcp
}

/// Find the changes in `targets` assuming they point to directories.
/// A directory is considered changed when its non-ignored child paths or their metadata has changed.
///
/// ```mermaid
/// flowchart TD
/// find_childs[Find the childs of each directory]
/// add_records[Add content and metadata digests of each child]
/// update_records[Update content and metadata digests with file_delta_store]
/// merge_child_list[Merge child path names in a to_string]
/// merge_content_digest[Merge content digests in a single byte vector]
/// merge_metadata_digest[Merge metadata digests in a single byte vector]
/// calculate_digests[Calculate digests from merged strings and byte vectors]
/// decide_digests[If digests are different from the records, send a DeltaField::Different record]
///
/// find_childs --> add_records
/// add_records --> update_records
/// update_records --> merge_child_list
/// merge_child_list --> merge_content_digest
/// merge_content_digest --> merge_metadata_digest
/// merge_metadata_digest --> calculate_digests
/// calculate_digests --> decide_digests
///
/// ```
///
/// This one calculates the differences in Depth-First fashion.
///
/// It adds the given directories to a stack.
///
/// For each directory in the stack, gets the list of child paths.
/// If childs contains a directory, it pushes the directory to stack.
///
///
/// ## Params
///
/// - `updated_path_comparison_params`: The _updated_ PathComparisonParams.
/// This function requires _pcp_ to be updated with
/// [update_path_comparison_params_with_actual_info].
/// - `targets`: the list of all targets that we're checking the changes
///              These may include files as well.
///
///
pub fn find_dir_changes_serial(
    xvc_root: &XvcRoot,
    updated_path_comparison_params: &PathComparisonParams,
    targets: &XvcPathMetadataMap,
) -> Result<DirectoryDeltaStore> {
    let pcp = updated_path_comparison_params;

    let mut dir_delta_store = DirectoryDeltaStore::new();

    let mut dir_stack = Vec::<XvcPath>::new();

    dir_stack.extend(targets.iter().map(|(xp, xm)| xp.clone()));

    while let Some(the_dir) = dir_stack.pop() {
        let dir_xe = pcp
            .xvc_path_store
            .entities_for(&the_dir)
            .unwrap()
            .get(0)
            .unwrap();

        let x_md = pcp
            .xvc_metadata_store
            .get(&dir_xe)
            .cloned()
            .unwrap_or_else(|| {
                let actual_md = the_dir.to_absolute_path(xvc_root).to_path_buf().metadata();
                let xvc_md = XvcMetadata::from(actual_md);
                xvc_md
            });

        if x_md.file_type == XvcFileType::Directory {
            // find child paths
            let mut vec_child_paths =
                Vec::from_iter(pcp.xvc_path_store.iter().filter_map(|(xe, xp)| {
                    if xp.starts_with(&the_dir) && (*xe != *dir_xe) {
                        Some(*xe)
                    } else {
                        None
                    }
                }));
            vec_child_paths.sort();
            let child_metadata: HStore<XvcMetadata> = vec_child_paths
                .iter()
                .filter_map(|xe| pcp.xvc_metadata_store.get(xe).map(|xmd| (*xe, *xmd)))
                .collect();

            let child_dirs: Vec<XvcEntity> = child_metadata
                .iter()
                .filter_map(|(xe, xmd)| {
                    if xmd.file_type == XvcFileType::Directory {
                        Some(*xe)
                    } else {
                        None
                    }
                })
                .collect();

            // if we have directories to calculate, let's dive to them first.
            if !child_dirs.is_empty() {
                if !child_dirs.iter().all(|xe| dir_delta_store.contains_key(xe)) {
                    dir_stack.push(the_dir);
                    child_dirs
                        .iter()
                        .for_each(|xe| dir_stack.push(pcp.xvc_path_store[xe].clone()));
                    continue;
                }
            }

            let child_paths_string = vec_child_paths.iter().fold(String::new(), |mut paths, xe| {
                let xvc_path_str = pcp.xvc_path_store[xe].to_string();
                paths.push_str(&xvc_path_str);
                paths
            });

            let child_paths_bytes = child_paths_string.as_bytes();
            let child_metadata_digest_bytes = vec_child_paths
                .iter()
                .map(|xe| {
                    if let Some(dd) = dir_delta_store.get(xe) {
                        match dd.delta_metadata_digest {
                            DeltaField::Different { actual, .. } => actual,
                            DeltaField::RecordMissing { actual } => actual,
                            DeltaField::Identical => pcp.metadata_digest_store[xe],
                            DeltaField::ActualMissing { record } => MetadataDigest(None),
                            DeltaField::Skipped => pcp.metadata_digest_store[xe],
                        }
                    } else {
                        *pcp.metadata_digest_store
                            .get(xe)
                            .unwrap_or_else(|| &MetadataDigest(None))
                    }
                })
                .fold(
                    Vec::<u8>::with_capacity(vec_child_paths.len() * DIGEST_LENGTH),
                    |mut bytes, metadata_digest| {
                        let digest_bytes = match metadata_digest.0 {
                            Some(xvc_digest) => xvc_digest.digest,
                            None => [0; DIGEST_LENGTH],
                        };

                        bytes.extend(digest_bytes);
                        bytes
                    },
                );

            let child_content_digest_bytes = vec_child_paths
                .iter()
                .map(|xe| {
                    if let Some(dd) = dir_delta_store.get(xe) {
                        match dd.delta_content_digest {
                            DeltaField::Different { actual, .. } => actual,
                            DeltaField::RecordMissing { actual } => actual,
                            DeltaField::Identical => pcp.content_digest_store[xe],
                            DeltaField::ActualMissing { record } => ContentDigest(None),
                            DeltaField::Skipped => pcp.content_digest_store[xe],
                        }
                    } else {
                        *pcp.content_digest_store
                            .get(xe)
                            .unwrap_or_else(|| &ContentDigest(None))
                    }
                })
                .fold(
                    Vec::<u8>::with_capacity(vec_child_paths.len() * DIGEST_LENGTH),
                    |mut bytes, content_digest| {
                        let digest_bytes = match content_digest.0 {
                            Some(xvc_digest) => xvc_digest.digest,
                            None => [0; DIGEST_LENGTH],
                        };

                        bytes.extend(digest_bytes);
                        bytes
                    },
                );

            let new_collection_digest = CollectionDigest(Some(XvcDigest::from_bytes(
                child_paths_bytes,
                &pcp.algorithm,
            )));
            let new_metadata_digest = MetadataDigest(Some(XvcDigest::from_bytes(
                &child_metadata_digest_bytes,
                &pcp.algorithm,
            )));
            let new_content_digest = ContentDigest(Some(XvcDigest::from_bytes(
                &child_content_digest_bytes,
                &pcp.algorithm,
            )));

            let delta_collection_digest = match pcp.collection_digest_store.get(&dir_xe) {
                None => DeltaField::RecordMissing {
                    actual: new_collection_digest,
                },
                Some(prev_collection_digest) => {
                    if *prev_collection_digest == new_collection_digest {
                        DeltaField::Identical
                    } else {
                        DeltaField::Different {
                            record: *prev_collection_digest,
                            actual: new_collection_digest,
                        }
                    }
                }
            };

            let delta_metadata_digest = match pcp.metadata_digest_store.get(&dir_xe) {
                None => DeltaField::RecordMissing {
                    actual: new_metadata_digest,
                },
                Some(prev_metadata_digest) => {
                    if *prev_metadata_digest == new_metadata_digest {
                        DeltaField::Identical
                    } else {
                        DeltaField::Different {
                            record: *prev_metadata_digest,
                            actual: new_metadata_digest,
                        }
                    }
                }
            };

            let delta_content_digest = match pcp.content_digest_store.get(&dir_xe) {
                None => DeltaField::RecordMissing {
                    actual: new_content_digest,
                },
                Some(prev_content_digest) => {
                    if *prev_content_digest == new_content_digest {
                        DeltaField::Identical
                    } else {
                        DeltaField::Different {
                            record: *prev_content_digest,
                            actual: new_content_digest,
                        }
                    }
                }
            };

            let delta_xvc_metadata = match pcp.xvc_metadata_store.get(&dir_xe) {
                None => DeltaField::RecordMissing { actual: x_md },
                Some(prev_md) => {
                    if x_md == *prev_md {
                        DeltaField::Identical
                    } else {
                        DeltaField::Different {
                            record: *prev_md,
                            actual: x_md,
                        }
                    }
                }
            };

            let dir_delta = DirectoryDelta {
                delta_xvc_metadata,
                delta_collection_digest,
                delta_metadata_digest,
                delta_content_digest,
            };

            dir_delta_store.insert(*dir_xe, dir_delta);
        }
    }

    Ok(dir_delta_store)
}

pub fn find_file_changes_serial(
    xvc_root: &XvcRoot,
    path_comparison_params: &PathComparisonParams,
    cache_type: &CacheType,
    text_or_binary: &DataTextOrBinary,
    targets: &XvcPathMetadataMap,
) -> Result<FileDeltaStore> {
    let mut xvc_entity_vec = Vec::<XvcEntity>::new();
    let mut diff_check_vec = Vec::<(XvcEntity, CoreResult<Metadata>)>::new();

    let xvc_path_imap = &path_comparison_params.xvc_path_store.index_map()?;
    let xvc_metadata_store = &path_comparison_params.xvc_metadata_store;

    let mut path_delta_store = HStore::<FileDelta>::new();

    for (xvc_path, actual_md) in targets {
        let xvc_entity = xvc_path_imap[xvc_path];
        let res_diff = xvc_file_diff(
            xvc_root,
            path_comparison_params,
            &xvc_entity,
            &actual_md,
            text_or_binary,
            cache_type,
        );
        if let Ok(diff) = res_diff {
            path_delta_store.insert(xvc_entity, diff);
        }
    }

    Ok(path_delta_store)
}

pub fn find_file_changes_parallel(
    xvc_root: &XvcRoot,
    path_comparison_params: &PathComparisonParams,
    cache_type: &CacheType,
    text_or_binary: &DataTextOrBinary,
    targets: &XvcPathMetadataMap,
) -> Result<FileDeltaStore> {
    let mut xvc_entity_vec = Vec::<XvcEntity>::new();
    let mut diff_check_vec = Vec::<(XvcEntity, CoreResult<Metadata>)>::new();

    let xvc_path_imap = &path_comparison_params.xvc_path_store.index_map()?;
    let xvc_metadata_store = &path_comparison_params.xvc_metadata_store;

    let path_delta_store = HStore::<FileDelta>::from_par_iter(targets.par_iter().filter_map(
        |(xvc_path, actual_md)| {
            let xvc_entity = xvc_path_imap[xvc_path];
            let res_diff = xvc_file_diff(
                xvc_root,
                path_comparison_params,
                &xvc_entity,
                &actual_md,
                text_or_binary,
                cache_type,
            );
            if let Ok(diff) = res_diff {
                Some((xvc_entity, diff))
            } else {
                None
            }
        },
    ));

    Ok(path_delta_store)
}

/// Find differences between the actual paths and metadata, and records.
///
/// ```mermaid
/// graph LR
/// Actual --> ActualXvcPath
/// Records --> RecordedXvcPath
/// ActualXvcPath --> [Comparator]
/// RecordedXvcPath --> [Comparator]
/// ```
///
/// For given targets, it finds the actual metadata and sends to a channel.
/// It also finds the records for these targets and sends them to a channel.
///

// pub fn find_file_changes_parallel(
//     xvc_root: &XvcRoot,
//     path_comparison_params: &PathComparisonParams,
//     cache_type: &CacheType,
//     text_or_binary: &DataTextOrBinary,
//     targets: &XvcPathMetadataMap,
// ) -> Result<FileDeltaStore> {
//     let (xvc_entity_snd, xvc_entity_rec) = bounded::<XvcEntity>(CHANNEL_BOUND);
//     let (diff_check_snd, diff_check_rec) =
//         bounded::<(XvcEntity, CoreResult<Metadata>)>(CHANNEL_BOUND);
//
//     let xvc_path_imap = &path_comparison_params.xvc_path_imap;
//     let xvc_metadata_store = &path_comparison_params.xvc_path_metadata_store.right;
//
//
//     for target in targets {
//         let x_e = xvc_path_imap[target];
//         let x_md = xvc_metadata_store.get(&x_e).cloned().unwrap_or_else(|| {
//             let actual_md = target.to_absolute_path(xvc_root).to_path_buf().metadata();
//             let xvc_md = XvcMetadata::from(actual_md);
//             xvc_md
//         });
//
//
//         if x_md.file_type == XvcFileType::Directory {
//             for (child_path, child_path_e) in xvc_path_imap {
//                 if child_path.starts_with(target) {
//                     let x_md = xvc_metadata_store[&child_path_e];
//                     if x_md.file_type != XvcFileType::Directory {
//                         xvc_entity_snd.send(*child_path_e).unwrap();
//                     }
//                 }
//             }
//         } else {
//             xvc_entity_snd.send(x_e)?;
//         }
//     }
//
//     drop(xvc_entity_snd);
//
//     let path_delta_store = crossbeam::scope(|s| {
//         s.spawn(|_| {
//             while let Ok(entity) = xvc_entity_rec.recv() {
//                 if let Some(xvc_path) = path_comparison_params
//                     .xvc_path_metadata_store
//                     .left
//                     .get(&entity)
//                 {
//                     let abs_path = xvc_path.to_absolute_path(xvc_root).to_path_buf();
//                     let res_md = abs_path.metadata();
//                     diff_check_snd
//                         .send((entity, res_md.map_err(|e| CoreError::from(e))))
//                         .unwrap();
//                 } else {
//                     warn!("Cannot find record for: {}", entity);
//                 }
//             }
//             drop(diff_check_snd);
//         });
//
//         let path_delta_store = s
//             .spawn(|_| {
//                 let mut path_delta_store = HStore::<FileDelta>::new();
//                 while let Ok((xvc_entity, res_md)) = diff_check_rec.recv() {
//                     let res_diff = xvc_file_diff(
//                         xvc_root,
//                         path_comparison_params,
//                         &xvc_entity,
//                         &res_md,
//                         text_or_binary,
//                         cache_type,
//                     );
//                     if let Ok(diff) = res_diff {
//                         if diff.shows_change() {
//                             path_delta_store.insert(xvc_entity.clone(), diff);
//                         }
//                     }
//                 }
//                 path_delta_store
//             })
//             .join()
//             .unwrap();
//
//         path_delta_store
//     })
//     .expect("Failed to spawn threads");
//
//     Ok(path_delta_store)
// }
//
fn filter_ignored_inner(
    pm_res: CoreResult<PathMetadata>,
    non_ignored_snd: &Sender<PathMetadata>,
    xvc_ignore: &IgnoreRules,
) -> Result<()> {
    match pm_res {
        Err(e) => {
            warn!("{}", e);
        }

        Ok(pm) => match check_ignore(xvc_ignore, &pm.path) {
            MatchResult::Ignore => {
                info!(
                    "{}",
                    Error::TargetIgnored {
                        path: pm.path.to_string_lossy().to_string()
                    }
                );
            }
            MatchResult::NoMatch | MatchResult::Whitelist => {
                non_ignored_snd.send(pm).unwrap();
            }
        },
    }
    Ok(())
}

fn xvc_file_diff(
    xvc_root: &XvcRoot,
    path_comparison_params: &PathComparisonParams,
    xvc_entity: &XvcEntity,
    actual_md: &XvcMetadata,
    text_or_binary: &DataTextOrBinary,
    cache_type: &CacheType,
) -> Result<FileDelta> {
    let pcp = path_comparison_params;
    // start from the easier comparisons
    let recorded_tob = pcp.text_or_binary_store.get(xvc_entity);

    let delta_text_or_binary = match recorded_tob {
        None => DeltaField::RecordMissing {
            actual: text_or_binary.clone(),
        },
        Some(prev_tob) => {
            if *prev_tob == *text_or_binary {
                DeltaField::Identical
            } else {
                DeltaField::Different {
                    record: *prev_tob,
                    actual: *text_or_binary,
                }
            }
        }
    };

    let recorded_cache_type = pcp.cache_type_store.get(xvc_entity);

    let delta_cache_type = match recorded_cache_type {
        None => DeltaField::RecordMissing {
            actual: *cache_type,
        },
        Some(prev_cache_type) => {
            if *prev_cache_type == *cache_type {
                DeltaField::Identical
            } else {
                DeltaField::Different {
                    actual: *cache_type,
                    record: *prev_cache_type,
                }
            }
        }
    };

    let xvc_path = pcp.xvc_path_store[xvc_entity].clone();

    let recorded_md = pcp.xvc_metadata_store.get(xvc_entity);

    let delta_md = match recorded_md {
        None => {
            if actual_md.file_type == XvcFileType::RecordOnly {
                return Err(Error::FileNotFound {
                    path: xvc_path.to_absolute_path(xvc_root).to_path_buf(),
                });
            } else {
                DeltaField::RecordMissing {
                    actual: actual_md.clone(),
                }
            }
        }

        Some(prev_md) => {
            if actual_md.file_type == XvcFileType::RecordOnly {
                DeltaField::<XvcMetadata>::ActualMissing { record: *prev_md }
            } else {
                if *prev_md == *actual_md {
                    DeltaField::<XvcMetadata>::Identical
                } else {
                    DeltaField::<XvcMetadata>::Different {
                        record: *prev_md,
                        actual: *actual_md,
                    }
                }
            }
        }
    };

    let delta_metadata_digest: DeltaField<MetadataDigest> = match delta_md {
        DeltaField::Identical => DeltaField::Identical,
        DeltaField::Skipped => DeltaField::Skipped,
        DeltaField::RecordMissing { actual } => DeltaField::RecordMissing {
            actual: actual.digest()?,
        },
        DeltaField::ActualMissing { record } => {
            if let Some(record) = pcp.metadata_digest_store.get(xvc_entity) {
                DeltaField::ActualMissing {
                    record: record.clone(),
                }
            } else {
                return Err(EcsError::CannotFindKeyInStore {
                    key: usize::from(*xvc_entity),
                }
                .into());
            }
        }
        DeltaField::Different { record, actual } => {
            if let Some(record) = pcp.metadata_digest_store.get(xvc_entity) {
                DeltaField::Different {
                    record: record.clone(),
                    actual: actual.digest()?,
                }
            } else {
                return Err(EcsError::CannotFindKeyInStore {
                    key: usize::from(*xvc_entity),
                }
                .into());
            }
        }
    };

    let delta_content_digest: DeltaField<ContentDigest> = match delta_md {
        DeltaField::Identical => DeltaField::Identical,
        DeltaField::Skipped => DeltaField::Skipped,
        DeltaField::RecordMissing { actual } => DeltaField::RecordMissing {
            actual: xvc_path.digest(xvc_root, pcp.algorithm, text_or_binary)?,
        },
        DeltaField::ActualMissing { record } => {
            if let Some(record) = pcp.content_digest_store.get(xvc_entity) {
                DeltaField::ActualMissing {
                    record: record.clone(),
                }
            } else {
                return Err(EcsError::CannotFindKeyInStore {
                    key: usize::from(*xvc_entity),
                }
                .into());
            }
        }
        // We check whether the content actually changed here
        DeltaField::Different { record, actual } => {
            if let Some(record) = pcp.content_digest_store.get(xvc_entity) {
                let record = record.clone();
                let actual = xvc_path.digest(xvc_root, pcp.algorithm, text_or_binary)?;
                if record == actual {
                    DeltaField::Identical
                } else {
                    DeltaField::Different {
                        record: record.clone(),
                        actual: xvc_path.digest(xvc_root, pcp.algorithm, text_or_binary)?,
                    }
                }
            } else {
                return Err(EcsError::CannotFindKeyInStore {
                    key: usize::from(*xvc_entity),
                }
                .into());
            }
        }
    };

    Ok(FileDelta {
        delta_md,
        delta_metadata_digest,
        delta_content_digest,
        delta_cache_type,
        delta_text_or_binary,
    })
}
