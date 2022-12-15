use crate::{error, Error, Result};
use crossbeam_channel::{bounded, Sender};
use dashmap::DashMap;
use log::{info, warn};
use rayon::iter::{FromParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::Metadata;
use std::path;
use xvc_config::FromConfigKey;
use xvc_core::types::xvcdigest::{CollectionDigest, ContentDigest, MetadataDigest, DIGEST_LENGTH};
use xvc_core::util::file::path_metadata_channel;
use xvc_ecs::Error as EcsError;

use xvc_core::{
    diff_store, CacheType, Diff, DiffStore, DiffStore2, DiffStore3, Error as CoreError,
    HashAlgorithm, Result as CoreResult, XvcDigest, XvcMetadata, XvcPath, XvcPathMetadataMap,
    XvcRoot,
};
use xvc_core::{XvcFileType, CHANNEL_BOUND};
use xvc_ecs::{HStore, R11Store, Storable, XvcEntity, XvcStore};
use xvc_logging::{uwo, uwr, watch, XvcOutputLine};
use xvc_walker::{check_ignore, IgnoreRules, MatchResult, PathMetadata};

use super::FileTextOrBinary;

#[derive(Debug)]
pub struct PathComparisonParams {
    pub xvc_path_store: XvcStore<XvcPath>,
    pub xvc_path_imap: BTreeMap<XvcPath, XvcEntity>,
    pub xvc_metadata_store: XvcStore<XvcMetadata>,
    pub content_digest_store: XvcStore<ContentDigest>,
    pub metadata_digest_store: XvcStore<MetadataDigest>,
    pub collection_digest_store: XvcStore<CollectionDigest>,
    pub cache_type_store: XvcStore<CacheType>,
    pub text_or_binary_store: XvcStore<FileTextOrBinary>,
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
        let text_or_binary_store = xvc_root.load_store::<FileTextOrBinary>()?;

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

// pub fn update_path_comparison_params_with_actual_info(
//     mut pcp: PathComparisonParams,
//     file_delta_store: &FileDeltaStore,
// ) -> PathComparisonParams {
//     for (xe, fd) in file_delta_store.map.iter() {
//         match fd.delta_md {
//             Diff::RecordMissing { actual } => {
//                 pcp.xvc_metadata_store.insert(*xe, actual);
//             }
//             Diff::Different { actual, .. } => {
//                 pcp.xvc_metadata_store.insert(*xe, actual);
//             }
//             _ => {}
//         }
//
//         match fd.delta_metadata_digest {
//             Diff::RecordMissing { actual } => {
//                 pcp.metadata_digest_store.insert(*xe, actual);
//             }
//             Diff::Different { actual, .. } => {
//                 pcp.metadata_digest_store.insert(*xe, actual);
//             }
//             _ => {}
//         }
//
//         match fd.delta_content_digest {
//             Diff::RecordMissing { actual } => {
//                 pcp.content_digest_store.insert(*xe, actual);
//             }
//             Diff::Different { actual, .. } => {
//                 pcp.content_digest_store.insert(*xe, actual);
//             }
//             _ => {}
//         }
//         match fd.delta_cache_type {
//             Diff::RecordMissing { actual } => {
//                 pcp.cache_type_store.insert(*xe, actual);
//             }
//             Diff::Different { actual, .. } => {
//                 pcp.cache_type_store.insert(*xe, actual);
//             }
//             _ => {}
//         }
//         match fd.delta_text_or_binary {
//             Diff::RecordMissing { actual } => {
//                 pcp.text_or_binary_store.insert(*xe, actual);
//             }
//             Diff::Different { actual, .. } => {
//                 pcp.text_or_binary_store.insert(*xe, actual);
//             }
//             _ => {}
//         }
//     }
//
//     pcp
// }
//
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
// pub fn find_dir_changes_serial(
//     os: Sender<XvcOutputLine>,
//     xvc_root: &XvcRoot,
//     updated_path_comparison_params: &PathComparisonParams,
//     actual_xvc_path_store: &HStore<XvcPath>,
//     actual_xvc_metadata_store: &HStore<XvcMetadata>,
// ) -> Result<DirectoryDeltaStore> {
//     let pcp = updated_path_comparison_params;
//
//     let path_subset: HashSet<XvcEntity> = HashSet::from_iter(actual_xvc_path_store.keys().copied());
//
//     let mut dir_delta_store = DirectoryDeltaStore::new();
//
//     let mut dir_stack = Vec::<XvcPath>::new();
//
//     dir_stack.extend(targets.iter().map(|(xp, xm)| xp.clone()));
//
//     while let Some(the_dir) = dir_stack.pop() {
//         let dir_xe = pcp
//             .xvc_path_store
//             .entities_for(&the_dir)
//             .unwrap()
//             .get(0)
//             .unwrap();
//
//         let x_md = pcp
//             .xvc_metadata_store
//             .get(&dir_xe)
//             .cloned()
//             .unwrap_or_else(|| {
//                 let actual_md = the_dir.to_absolute_path(xvc_root).to_path_buf().metadata();
//                 let xvc_md = XvcMetadata::from(actual_md);
//                 xvc_md
//             });
//
//         if x_md.file_type == XvcFileType::Directory {
//             // find child paths
//             let mut vec_child_paths =
//                 Vec::from_iter(pcp.xvc_path_store.iter().filter_map(|(xe, xp)| {
//                     if xp.starts_with(&the_dir) && (*xe != *dir_xe) {
//                         Some(*xe)
//                     } else {
//                         None
//                     }
//                 }));
//             vec_child_paths.sort();
//             let child_metadata: HStore<XvcMetadata> = vec_child_paths
//                 .iter()
//                 .filter_map(|xe| pcp.xvc_metadata_store.get(xe).map(|xmd| (*xe, *xmd)))
//                 .collect();
//
//             let child_dirs: Vec<XvcEntity> = child_metadata
//                 .iter()
//                 .filter_map(|(xe, xmd)| {
//                     if xmd.file_type == XvcFileType::Directory {
//                         Some(*xe)
//                     } else {
//                         None
//                     }
//                 })
//                 .collect();
//
//             // if we have directories to calculate, let's dive to them first.
//             if !child_dirs.is_empty() {
//                 if !child_dirs.iter().all(|xe| dir_delta_store.contains_key(xe)) {
//                     dir_stack.push(the_dir);
//                     child_dirs
//                         .iter()
//                         .for_each(|xe| dir_stack.push(pcp.xvc_path_store[xe].clone()));
//                     continue;
//                 }
//             }
//
//             let child_paths_string = vec_child_paths.iter().fold(String::new(), |mut paths, xe| {
//                 let xvc_path_str = pcp.xvc_path_store[xe].to_string();
//                 paths.push_str(&xvc_path_str);
//                 paths
//             });
//
//             let child_paths_bytes = child_paths_string.as_bytes();
//             let child_metadata_digest_bytes = vec_child_paths
//                 .iter()
//                 .map(|xe| {
//                     if let Some(dd) = dir_delta_store.get(xe) {
//                         match dd.delta_metadata_digest {
//                             Diff::Different { actual, .. } => actual,
//                             Diff::RecordMissing { actual } => actual,
//                             Diff::Identical => pcp.metadata_digest_store[xe],
//                             Diff::ActualMissing { record } => MetadataDigest(None),
//                             Diff::Skipped => pcp.metadata_digest_store[xe],
//                         }
//                     } else {
//                         *pcp.metadata_digest_store
//                             .get(xe)
//                             .unwrap_or_else(|| &MetadataDigest(None))
//                     }
//                 })
//                 .fold(
//                     Vec::<u8>::with_capacity(vec_child_paths.len() * DIGEST_LENGTH),
//                     |mut bytes, metadata_digest| {
//                         let digest_bytes = match metadata_digest.0 {
//                             Some(xvc_digest) => xvc_digest.digest,
//                             None => [0; DIGEST_LENGTH],
//                         };
//
//                         bytes.extend(digest_bytes);
//                         bytes
//                     },
//                 );
//
//             let child_content_digest_bytes = vec_child_paths
//                 .iter()
//                 .map(|xe| {
//                     if let Some(dd) = dir_delta_store.get(xe) {
//                         match dd.delta_content_digest {
//                             Diff::Different { actual, .. } => actual,
//                             Diff::RecordMissing { actual } => actual,
//                             Diff::Identical => pcp.content_digest_store[xe],
//                             Diff::ActualMissing { record } => ContentDigest(None),
//                             Diff::Skipped => pcp.content_digest_store[xe],
//                         }
//                     } else {
//                         *pcp.content_digest_store
//                             .get(xe)
//                             .unwrap_or_else(|| &ContentDigest(None))
//                     }
//                 })
//                 .fold(
//                     Vec::<u8>::with_capacity(vec_child_paths.len() * DIGEST_LENGTH),
//                     |mut bytes, content_digest| {
//                         let digest_bytes = match content_digest.0 {
//                             Some(xvc_digest) => xvc_digest.digest,
//                             None => [0; DIGEST_LENGTH],
//                         };
//
//                         bytes.extend(digest_bytes);
//                         bytes
//                     },
//                 );
//
//             let new_collection_digest = CollectionDigest(Some(XvcDigest::from_bytes(
//                 child_paths_bytes,
//                 &pcp.algorithm,
//             )));
//             let new_metadata_digest = MetadataDigest(Some(XvcDigest::from_bytes(
//                 &child_metadata_digest_bytes,
//                 &pcp.algorithm,
//             )));
//             let new_content_digest = ContentDigest(Some(XvcDigest::from_bytes(
//                 &child_content_digest_bytes,
//                 &pcp.algorithm,
//             )));
//
//             let delta_collection_digest = match pcp.collection_digest_store.get(&dir_xe) {
//                 None => Diff::RecordMissing {
//                     actual: new_collection_digest,
//                 },
//                 Some(prev_collection_digest) => {
//                     if *prev_collection_digest == new_collection_digest {
//                         Diff::Identical
//                     } else {
//                         Diff::Different {
//                             record: *prev_collection_digest,
//                             actual: new_collection_digest,
//                         }
//                     }
//                 }
//             };
//
//             let delta_metadata_digest = match pcp.metadata_digest_store.get(&dir_xe) {
//                 None => Diff::RecordMissing {
//                     actual: new_metadata_digest,
//                 },
//                 Some(prev_metadata_digest) => {
//                     if *prev_metadata_digest == new_metadata_digest {
//                         Diff::Identical
//                     } else {
//                         Diff::Different {
//                             record: *prev_metadata_digest,
//                             actual: new_metadata_digest,
//                         }
//                     }
//                 }
//             };
//
//             let delta_content_digest = match pcp.content_digest_store.get(&dir_xe) {
//                 None => Diff::RecordMissing {
//                     actual: new_content_digest,
//                 },
//                 Some(prev_content_digest) => {
//                     if *prev_content_digest == new_content_digest {
//                         Diff::Identical
//                     } else {
//                         Diff::Different {
//                             record: *prev_content_digest,
//                             actual: new_content_digest,
//                         }
//                     }
//                 }
//             };
//
//             let xvc_metadata_diff_store = diff_store(
//                 &pcp.xvc_metadata_store,
//                 xvc_metadata_actual_store,
//                 Some(path_subset),
//             );
//
//             let delta_xvc_metadata = match pcp.xvc_metadata_store.get(&dir_xe) {
//                 None => Diff::RecordMissing { actual: x_md },
//                 Some(prev_md) => {
//                     if x_md == *prev_md {
//                         Diff::Identical
//                     } else {
//                         Diff::Different {
//                             record: *prev_md,
//                             actual: x_md,
//                         }
//                     }
//                 }
//             };
//
//             let dir_delta = DirectoryDelta {
//                 delta_xvc_metadata,
//                 delta_collection_digest,
//                 delta_metadata_digest,
//                 delta_content_digest,
//             };
//
//             dir_delta_store.insert(*dir_xe, dir_delta);
//         }
//     }
//
//     Ok(dir_delta_store)
// }
//
/// Compare the records and the actual info from `pmm` to find the differences
/// in paths.
/// This is used to detect changes between actual paths and our records.
/// New entities are created for those paths missing from the records.
pub fn diff_xvc_path_metadata(
    xvc_root: &XvcRoot,
    stored_xvc_path_store: &XvcStore<XvcPath>,
    stored_xvc_metadata_store: &XvcStore<XvcMetadata>,
    pmm: &XvcPathMetadataMap,
) -> DiffStore2<XvcPath, XvcMetadata> {
    let actual_xvc_path_store: HStore<XvcPath> = HStore::from_storable(
        pmm.keys().cloned(),
        stored_xvc_path_store,
        xvc_root.entity_generator(),
    );

    let entities: HashSet<XvcEntity> = actual_xvc_path_store.keys().copied().collect();

    let actual_xvc_metadata_store: HStore<XvcMetadata> = actual_xvc_path_store
        .iter()
        .map(|(xe, xp)| (*xe, pmm[xp].clone()))
        .collect();

    let xvc_path_diff = diff_store(
        stored_xvc_path_store,
        &actual_xvc_path_store,
        Some(&entities),
    );

    let xvc_metadata_diff = diff_store(
        stored_xvc_metadata_store,
        &actual_xvc_metadata_store,
        Some(&entities),
    );

    DiffStore2(xvc_path_diff, xvc_metadata_diff)
}

/// For each command, we have a single requested_cache_type.
/// We build an actual store by repeating it for all entities we have.
pub fn diff_cache_type(
    stored_cache_type_store: &XvcStore<CacheType>,
    requested_cache_type: &CacheType,
    entities: &HashSet<XvcEntity>,
) -> DiffStore<CacheType> {
    let requested_cache_type_store: HStore<CacheType> =
        HStore::from_iter(entities.iter().map(|x| (*x, *requested_cache_type)));

    let cache_store_diff = diff_store(
        stored_cache_type_store,
        &requested_cache_type_store,
        Some(entities),
    );

    cache_store_diff
}

/// For each command, we have a single requested_text_or_binary.
/// We build an actual store by repeating it for all entities we have.
/// This is used to find when the user wants to change recheck method.
pub fn diff_text_or_binary(
    stored_text_or_binary_store: &XvcStore<FileTextOrBinary>,
    requested_text_or_binary: &FileTextOrBinary,
    entities: &HashSet<XvcEntity>,
) -> DiffStore<FileTextOrBinary> {
    let requested_text_or_binary_store: HStore<FileTextOrBinary> = entities
        .iter()
        .map(|x| (*x, *requested_text_or_binary))
        .collect();
    let text_or_binary_diff = diff_store(
        &stored_text_or_binary_store,
        &requested_text_or_binary_store,
        Some(entities),
    );
    text_or_binary_diff
}

/// Check whether content digests of files in `prerequisite_diffs` have changed.
///
/// This is used to identify the files that requires attention in several
/// commands, like recheck or carry-in.
pub fn diff_content_digest(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    stored_xvc_path_store: &XvcStore<XvcPath>,
    stored_content_digest_store: &XvcStore<ContentDigest>,
    stored_text_or_binary_store: &XvcStore<FileTextOrBinary>,
    prerequisite_diffs: &DiffStore3<XvcPath, XvcMetadata, FileTextOrBinary>,
    requested_text_or_binary: &Option<FileTextOrBinary>,
    requested_hash_algorithm: &Option<HashAlgorithm>,
    parallel: bool,
) -> DiffStore<ContentDigest> {
    let xvc_path_diff_store = prerequisite_diffs.0;
    let xvc_metadata_diff_store = prerequisite_diffs.1;
    let text_or_binary_diff_store = prerequisite_diffs.2;
    let entities: HashSet<XvcEntity> = xvc_path_diff_store.keys().copied().collect();
    let algorithm: HashAlgorithm =
        requested_hash_algorithm.unwrap_or_else(|| HashAlgorithm::from_conf(xvc_root.config()));

    let the_closure = |xe| -> Result<(XvcEntity, Diff<ContentDigest>)> {
        let xvc_path_diff = xvc_path_diff_store.get(xe).expect("xvc_path_diff.get(xe)");
        let xvc_metadata_diff = xvc_metadata_diff_store
            .get(xe)
            .expect("xvc_metadata_diff.get(xe)");

        if prerequisite_diffs.get_diff3(*xe).changed() {
            let stored_content_digest = stored_content_digest_store.get(xe);
            let text_or_binary = requested_text_or_binary.unwrap_or_else(|| {
                stored_text_or_binary_store
                    .get(xe)
                    .copied()
                    .unwrap_or_else(|| FileTextOrBinary::from_conf(xvc_root.config()))
            });
            let diff_content_digest = match xvc_path_diff {
                // We calculate the diff even the path is identical.
                // This is because the metadata or the `text_or_binary` has
                // changed.
                // Actually, this is the most common branch, as we don't expect
                // the entity paths to change that often.
                Diff::Identical | Diff::Skipped => {
                    let xvc_path = stored_xvc_path_store
                        .get(xe)
                        .expect("stored_xvc_path_store.get(xe)");
                    let path = xvc_path.to_absolute_path(xvc_root);
                    let actual = ContentDigest::from_path(&path, algorithm, text_or_binary.0)?;
                    match stored_content_digest {
                        Some(record) => {
                            if actual != *record {
                                Diff::Different {
                                    record: *record,
                                    actual,
                                }
                            } else {
                                Diff::Identical
                            }
                        }
                        None => Diff::RecordMissing { actual },
                    }
                }
                // The path is not recorded before.
                Diff::RecordMissing { actual } => {
                    let path = actual.to_absolute_path(xvc_root);
                    let actual = ContentDigest::from_path(&path, algorithm, text_or_binary.0)?;
                    match stored_content_digest {
                        Some(record) => {
                            if actual != *record {
                                Diff::Different {
                                    record: *record,
                                    actual,
                                }
                            } else {
                                Diff::Identical
                            }
                        }
                        None => Diff::RecordMissing { actual },
                    }
                }
                // The path is changed. This can happen after a move
                // operation, for example.
                Diff::Different { record, actual } => {
                    let path = actual.to_absolute_path(xvc_root);
                    let actual = ContentDigest::from_path(&path, algorithm, text_or_binary.0)?;
                    match stored_content_digest {
                        Some(record) => {
                            if actual != *record {
                                Diff::Different {
                                    record: *record,
                                    actual,
                                }
                            } else {
                                Diff::Identical
                            }
                        }
                        None => Diff::RecordMissing { actual },
                    }
                }
                // We have a record, but the path on disk is missing.
                // We can't calculate the digest. We'll use the recorded
                // one.
                Diff::ActualMissing { record } => {
                    match stored_content_digest {
                        Some(record) => Diff::ActualMissing { record: *record },
                        // if the both actual and the record are
                        // missing, they are identical in their inexistance.
                        // how can a man without hands clap?
                        None => Diff::Identical,
                    }
                }
            };

            Ok((*xe, diff_content_digest))
        } else {
            Ok((*xe, Diff::Skipped))
        }
    };

    if parallel {
        entities
            .par_iter()
            .map(uwr!(the_closure, output_snd))
            .collect()
    } else {
        entities.iter().map(uwr!(the_closure, output_snd)).collect()
    }
}

/// This is used to detect changes in path collections, e.g., directories or
/// globs.
/// When a collection list changes, for example a file added to a directory, we
/// recalculate the collection digest to see if the collection has changed.
pub fn diff_dir_collection_digest(
    xvc_root: &XvcRoot,
    stored_collection_digest: Option<&CollectionDigest>,
    stored_xvc_path_store: &XvcStore<XvcPath>,
    path_diffs: &DiffStore<XvcPath>,
    sorted_entities: &[XvcEntity],
) -> Result<Diff<CollectionDigest>> {
    let xvc_path_diff = path_diffs.subset(sorted_entities.iter().copied())?;
    let stored_xvc_paths = stored_xvc_path_store.subset(sorted_entities.iter().copied())?;
    let collection_strings = Vec::<String>::new();

    for xe in sorted_entities {
        let xvc_path_diff = xvc_path_diff.get(xe).expect("xvc_path_diff.get(xe)");
        match xvc_path_diff {
            Diff::Identical | Diff::Skipped => {
                let path = stored_xvc_paths.get(xe).expect("stored_xvc_paths.get(xe)");
                collection_strings.push(path.to_string());
            }
            Diff::RecordMissing { actual } => {
                collection_strings.push(actual.to_string());
            }
            Diff::Different { record, actual } => {
                collection_strings.push(actual.to_string());
            }
            Diff::ActualMissing { record } => {
                // We can do some weird things here, like adding a prefix or
                // reversing the string to change the collection result. I think it's better to use an empty
                // entity string to describe the situation.

                // This is to make sure the collection digest is different when
                // all records are missing.
                collection_strings.push(xe.to_string());
            }
        }
    }

    let joined = collection_strings.join("\n");
    let actual: CollectionDigest = XvcDigest::from_content(&joined, HashAlgorithm::Blake3).into();

    let dg = match stored_collection_digest {
        Some(record) => {
            if actual != *record {
                Diff::Different {
                    record: *record,
                    actual,
                }
            } else {
                Diff::Identical
            }
        }
        None => Diff::RecordMissing { actual },
    };

    Ok(dg)
}

/// This is to detect metadata changes in collections, e.g., directories or
/// globs. When timestamp, size or similar metadata changes, the result changes.
/// It can be used to detect changes in directories, globs, or other collections
/// that use [XvcMetadata] to keep individual items' metadata.
pub fn diff_dir_metadata_digest(
    xvc_root: &XvcRoot,
    stored_metadata_digest: Option<&MetadataDigest>,
    stored_xvc_metadata_store: &XvcStore<XvcMetadata>,
    metadata_diffs: &DiffStore<XvcMetadata>,
    sorted_entities: &[XvcEntity],
) -> Result<Diff<MetadataDigest>> {
    let xvc_metadata_diff = metadata_diffs.subset(sorted_entities.iter().copied())?;
    let metadata_digest_bytes = Vec::<u8>::with_capacity(sorted_entities.len() * DIGEST_LENGTH);

    for xe in sorted_entities {
        let xvc_metadata_diff = xvc_metadata_diff
            .get(xe)
            .ok_or(EcsError::CannotFindEntityInStore { entity: *xe })?;
        match xvc_metadata_diff {
            Diff::Identical | Diff::Skipped => {
                let metadata = stored_xvc_metadata_store
                    .get(xe)
                    .ok_or(xvc_ecs::error::Error::CannotFindKeyInStore { key: (*xe).into() })?;
                metadata_digest_bytes.extend(metadata.digest()?.0.unwrap().as_bytes());
            }
            Diff::RecordMissing { actual } => {
                metadata_digest_bytes.extend(actual.digest()?.0.unwrap().as_bytes());
            }
            Diff::Different { record, actual } => {
                metadata_digest_bytes.extend(actual.digest()?.0.unwrap().as_bytes());
            }
            Diff::ActualMissing { record } => {
                // This is to make sure the metadata digest is different when
                // all records are missing or their order has changed.
                let entity_bytes: usize = (*xe).into();
                let mut entity_bytes_as_digest = Vec::from([0u8; DIGEST_LENGTH]);
                entity_bytes_as_digest.copy_from_slice(&entity_bytes.to_le_bytes());
                metadata_digest_bytes.extend(
                    XvcDigest::from_bytes(&entity_bytes_as_digest, HashAlgorithm::AsIs).digest,
                );
            }
        }
    }

    // We always use Blake3 to keep the metadata digest consistent.
    let actual = MetadataDigest::from(XvcDigest::from_bytes(
        &metadata_digest_bytes,
        HashAlgorithm::Blake3,
    ));

    let digest = match stored_metadata_digest {
        Some(record) => {
            if actual != *record {
                Diff::Different {
                    record: *record,
                    actual,
                }
            } else {
                Diff::Identical
            }
        }
        None => Diff::RecordMissing { actual },
    };

    Ok(digest)
}

/// This is used to detect content changes in elements of path collections,
/// e.g., directories or globs. When the content of these elements change, their
/// content digests also change. We collect them together and calculate their
/// digest to detect changes in the collection.
pub fn diff_dir_content_digest(
    xvc_root: &XvcRoot,
    stored_content_digest: Option<&ContentDigest>,
    stored_xvc_content_store: &XvcStore<ContentDigest>,
    content_diffs: &DiffStore<ContentDigest>,
    sorted_entities: &[XvcEntity],
) -> Result<Diff<ContentDigest>> {
    let xvc_content_diff = content_diffs.subset(sorted_entities.iter().copied())?;
    let content_digest_bytes = Vec::<u8>::with_capacity(sorted_entities.len() * DIGEST_LENGTH);

    for xe in sorted_entities {
        let xvc_content_diff = xvc_content_diff
            .get(xe)
            .ok_or(EcsError::CannotFindEntityInStore { entity: *xe })?;
        match xvc_content_diff {
            Diff::Identical | Diff::Skipped => {
                let content = stored_xvc_content_store
                    .get(xe)
                    .ok_or(xvc_ecs::error::Error::CannotFindEntityInStore { entity: *xe })?;
                content_digest_bytes.extend(content.0.expect("digest").digest);
            }
            Diff::RecordMissing { actual } => {
                content_digest_bytes.extend(actual.0.expect("digest").digest);
            }
            Diff::Different { record, actual } => {
                content_digest_bytes.extend(actual.0.expect("digest").digest);
            }
            Diff::ActualMissing { record } => {
                // This is to make sure the content digest is different when
                // all records are missing or their order has changed.
                let entity_bytes: usize = (*xe).into();
                let mut entity_bytes_as_digest = Vec::from([0u8; DIGEST_LENGTH]);
                entity_bytes_as_digest.copy_from_slice(&entity_bytes.to_le_bytes());
                content_digest_bytes.extend(
                    &XvcDigest::from_bytes(&entity_bytes_as_digest, HashAlgorithm::AsIs).digest,
                );
            }
        }
    }

    // We always use Blake3 to keep the content digest consistent.
    let actual = ContentDigest::from(XvcDigest::from_bytes(
        &content_digest_bytes,
        HashAlgorithm::Blake3,
    ));

    let digest = match stored_content_digest {
        Some(record) => {
            if actual != *record {
                Diff::Different {
                    record: *record,
                    actual,
                }
            } else {
                Diff::Identical
            }
        }
        None => Diff::RecordMissing { actual },
    };

    Ok(digest)
}
