use crate::error::Error;
use crate::Result;
use crossbeam_channel::Sender;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

use xvc_config::FromConfigKey;
use xvc_core::types::xvcdigest::{CollectionDigest, ContentDigest, MetadataDigest, DIGEST_LENGTH};
use xvc_ecs::Error as EcsError;

use xvc_core::{
    diff_store, CacheType, Diff, DiffStore, DiffStore2, DiffStore3, HashAlgorithm, XvcDigest,
    XvcMetadata, XvcPath, XvcPathMetadataMap, XvcRoot,
};

use xvc_ecs::{HStore, XvcEntity, XvcStore};
use xvc_logging::{panic, uwr, watch, XvcOutputLine};

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
    watch!(pmm);
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
    requested_cache_type: CacheType,
    entities: &HashSet<XvcEntity>,
) -> DiffStore<CacheType> {
    let requested_cache_type_store: HStore<CacheType> =
        HStore::from_iter(entities.iter().map(|x| (*x, requested_cache_type)));

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
    requested_text_or_binary: FileTextOrBinary,
    entities: &HashSet<XvcEntity>,
) -> DiffStore<FileTextOrBinary> {
    let requested_text_or_binary_store: HStore<FileTextOrBinary> = entities
        .iter()
        .map(|x| (*x, requested_text_or_binary))
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
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    stored_xvc_path_store: &XvcStore<XvcPath>,
    stored_content_digest_store: &XvcStore<ContentDigest>,
    stored_text_or_binary_store: &XvcStore<FileTextOrBinary>,
    prerequisite_diffs: &DiffStore3<XvcPath, XvcMetadata, FileTextOrBinary>,
    requested_text_or_binary: Option<FileTextOrBinary>,
    requested_hash_algorithm: Option<HashAlgorithm>,
    parallel: bool,
) -> DiffStore<ContentDigest> {
    let xvc_path_diff_store = &prerequisite_diffs.0;
    let xvc_metadata_diff_store = &prerequisite_diffs.1;
    let text_or_binary_diff_store = &prerequisite_diffs.2;
    let entities: HashSet<XvcEntity> = xvc_path_diff_store.keys().copied().collect();
    let algorithm: HashAlgorithm =
        requested_hash_algorithm.unwrap_or_else(|| HashAlgorithm::from_conf(xvc_root.config()));

    let the_closure = |xe| -> Result<(XvcEntity, Diff<ContentDigest>)> {
        let xvc_path_diff = xvc_path_diff_store
            .get(xe)
            .ok_or_else(|| EcsError::CannotFindEntityInStore { entity: *xe })?;
        let xvc_metadata_diff = xvc_metadata_diff_store
            .get(xe)
            .ok_or_else(|| EcsError::CannotFindEntityInStore { entity: *xe })?;
        if prerequisite_diffs.get_diff3(*xe).changed() {
            let stored_content_digest = stored_content_digest_store.get(xe);
            let text_or_binary = requested_text_or_binary.unwrap_or_else(|| {
                stored_text_or_binary_store
                    .get(xe)
                    .copied()
                    .unwrap_or_else(|| FileTextOrBinary::from_conf(xvc_root.config()))
            });

            let path_from_store = || -> Result<PathBuf> {
                let xvc_path = stored_xvc_path_store
                    .get(xe)
                    .ok_or_else(|| EcsError::CannotFindEntityInStore { entity: *xe })?;
                let path = xvc_path.to_absolute_path(xvc_root).to_path_buf();
                Ok(path)
            };
            let compare_with_stored_digest = |actual| -> Diff<ContentDigest> {
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
            };

            watch!(xvc_path_diff);
            let diff_content_digest = match xvc_path_diff {
                // We calculate the diff even the path is identical.
                // This is because the metadata or the `text_or_binary` has
                // changed.
                Diff::Identical | Diff::Skipped => {
                    match xvc_metadata_diff {
                        // text_or_binary should have changed.
                        Diff::Skipped | Diff::Identical => {
                            let path = path_from_store()?;
                            let actual =
                                ContentDigest::from_path(&path, algorithm, text_or_binary.0)?;
                            compare_with_stored_digest(actual)
                        }
                        Diff::RecordMissing { .. } => {
                            panic!(output_snd, "We have path but no metadata for entity {xe}. This shouldn't happen.");
                        }
                        Diff::ActualMissing { .. } => Diff::ActualMissing {
                            record: stored_content_digest.unwrap().clone(),
                        },
                        // Either the metadata has changed, or the file is deleted.
                        Diff::Different { actual, .. } => match actual.file_type {
                            xvc_core::XvcFileType::Missing => Diff::ActualMissing {
                                record: stored_content_digest.unwrap().clone(),
                            },
                            xvc_core::XvcFileType::File => {
                                let path = path_from_store()?;
                                let actual =
                                    ContentDigest::from_path(&path, algorithm, text_or_binary.0)?;
                                compare_with_stored_digest(actual)
                            }
                            xvc_core::XvcFileType::Reflink
                            | xvc_core::XvcFileType::Hardlink
                            | xvc_core::XvcFileType::Directory
                            | xvc_core::XvcFileType::Symlink => {
                                let path = path_from_store()?;
                                return Err(Error::ContentDigestNotSupported { path });
                            }
                        },
                    }
                }
                // The path is not recorded before.
                Diff::RecordMissing { actual } => {
                    let path = actual.to_absolute_path(xvc_root);
                    let actual = ContentDigest::from_path(&path, algorithm, text_or_binary.0)?;
                    compare_with_stored_digest(actual)
                }
                // The path is changed. This can happen after a move
                // operation, for example.
                Diff::Different { record, actual } => {
                    let path = actual.to_absolute_path(xvc_root);
                    let actual = ContentDigest::from_path(&path, algorithm, text_or_binary.0)?;
                    compare_with_stored_digest(actual)
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
            .map(|e| uwr!(the_closure(e), output_snd))
            .collect()
    } else {
        entities
            .iter()
            .map(|e| uwr!(the_closure(e), output_snd))
            .collect()
    }
}

/// This is used to detect changes in path collections, e.g., directories or
/// globs.
/// When a collection list changes, for example a file added to a directory, we
/// recalculate the collection digest to see if the collection has changed.
pub fn diff_dir_collection_digest(
    stored_collection_digest: Option<&CollectionDigest>,
    stored_xvc_path_store: &XvcStore<XvcPath>,
    path_diffs: &DiffStore<XvcPath>,
    sorted_entities: &[XvcEntity],
) -> Result<Diff<CollectionDigest>> {
    let xvc_path_diff = path_diffs.subset(sorted_entities.iter().copied())?;
    let stored_xvc_paths = stored_xvc_path_store.subset(sorted_entities.iter().copied())?;
    let mut collection_strings = Vec::<String>::new();

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
    stored_metadata_digest: Option<&MetadataDigest>,
    stored_xvc_metadata_store: &XvcStore<XvcMetadata>,
    metadata_diffs: &DiffStore<XvcMetadata>,
    sorted_entities: &[XvcEntity],
) -> Result<Diff<MetadataDigest>> {
    let xvc_metadata_diff = metadata_diffs.subset(sorted_entities.iter().copied())?;
    let mut metadata_digest_bytes = Vec::<u8>::with_capacity(sorted_entities.len() * DIGEST_LENGTH);

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
    stored_content_digest: Option<&ContentDigest>,
    stored_xvc_content_store: &XvcStore<ContentDigest>,
    content_diffs: &DiffStore<ContentDigest>,
    sorted_entities: &[XvcEntity],
) -> Result<Diff<ContentDigest>> {
    let xvc_content_diff = content_diffs.subset(sorted_entities.iter().copied())?;
    let mut content_digest_bytes = Vec::<u8>::with_capacity(sorted_entities.len() * DIGEST_LENGTH);

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
