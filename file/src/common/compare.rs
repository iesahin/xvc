//! File comparison utilities.
use crate::error::Error;
use crate::Result;
use anyhow::anyhow;
use crossbeam_channel::{Receiver, Sender};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use std::collections::HashSet;
use std::path::PathBuf;
use std::thread::{self, JoinHandle};

use xvc_config::FromConfigKey;
use xvc_core::types::xvcdigest::{content_digest::ContentDigest, DIGEST_LENGTH};
use xvc_ecs::{Error as EcsError, SharedXStore};

use xvc_core::{
    diff_store, Diff, DiffStore, DiffStore2, HashAlgorithm, RecheckMethod, XvcDigest, XvcFileType,
    XvcMetadata, XvcPath, XvcPathMetadataMap, XvcRoot,
};

use xvc_ecs::{HStore, XvcEntity, XvcStore};
use xvc_logging::{debug, error, panic, watch, XvcOutputSender};

use super::FileTextOrBinary;

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
        .map(|(xe, xp)| (*xe, pmm[xp]))
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

/// For each command, we have a single requested_recheck_method.
/// We build an actual store by repeating it for all entities we have.
pub fn diff_recheck_method(
    stored_recheck_method_store: &XvcStore<RecheckMethod>,
    requested_recheck_method: RecheckMethod,
    entities: &HashSet<XvcEntity>,
) -> DiffStore<RecheckMethod> {
    let requested_recheck_method_store: HStore<RecheckMethod> =
        HStore::from_iter(entities.iter().map(|x| (*x, requested_recheck_method)));

    diff_store(
        stored_recheck_method_store,
        &requested_recheck_method_store,
        Some(entities),
    )
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

    diff_store(
        stored_text_or_binary_store,
        &requested_text_or_binary_store,
        Some(entities),
    )
}

/// Compare the content of a file with the stored content digest.
///
/// The file is defined by the entity `xe`. The comparison is done only when the path (`xvc_path_diff`) or the metadata
/// (`xvc_metadata_diff`) of the file has changed.
#[allow(clippy::too_many_arguments)]
pub fn diff_file_content_digest(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    xe: XvcEntity,
    xvc_path_diff: &Diff<XvcPath>,
    xvc_metadata_diff: &Diff<XvcMetadata>,
    stored_xvc_path_store: &XvcStore<XvcPath>,
    stored_content_digest_store: &XvcStore<ContentDigest>,
    algorithm: HashAlgorithm,
    text_or_binary: FileTextOrBinary,
) -> Result<(XvcEntity, Diff<ContentDigest>)> {
    let anything_changed = xvc_path_diff.changed() || xvc_metadata_diff.changed();

    if anything_changed {
        let stored_content_digest = stored_content_digest_store.get(&xe);

        let path_from_store = || -> Result<PathBuf> {
            let xvc_path = stored_xvc_path_store
                .get(&xe)
                .ok_or(EcsError::CannotFindEntityInStore { entity: xe })?;
            let path = xvc_path.to_absolute_path(xvc_root).to_path_buf();
            Ok(path)
        };
        let compare_with_stored_digest = |actual| -> Diff<ContentDigest> {
            watch!(stored_content_digest);
            watch!(actual);
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
            Diff::Identical | Diff::Skipped => {
                match xvc_metadata_diff {
                    // text_or_binary should have changed.
                    Diff::Skipped | Diff::Identical => {
                        let path = path_from_store()?;
                        let actual = ContentDigest::new(&path, algorithm, text_or_binary.0)?;
                        compare_with_stored_digest(actual)
                    }
                    Diff::RecordMissing { .. } => {
                        panic!(
                            output_snd,
                            "We have path but no metadata for entity {xe}. This shouldn't happen."
                        );
                    }
                    Diff::ActualMissing { .. } => Diff::ActualMissing {
                        record: *stored_content_digest.unwrap(),
                    },
                    // Either the metadata has changed, or the file is deleted.
                    Diff::Different { actual, .. } => match actual.file_type {
                        xvc_core::XvcFileType::Missing => Diff::ActualMissing {
                            record: *stored_content_digest.unwrap(),
                        },
                        xvc_core::XvcFileType::File => {
                            let path = path_from_store()?;
                            let actual = ContentDigest::new(&path, algorithm, text_or_binary.0)?;
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
                watch!(actual);
                let path = actual.to_absolute_path(xvc_root);
                watch!(path);
                let actual_digest = ContentDigest::new(&path, algorithm, text_or_binary.0)?;
                watch!(actual_digest);
                let res = compare_with_stored_digest(actual_digest);
                watch!(res);
                res
            }
            // The path is changed. This can happen after a move
            // operation, for example.
            Diff::Different { actual, .. } => {
                let path = actual.to_absolute_path(xvc_root);
                let actual = ContentDigest::new(&path, algorithm, text_or_binary.0)?;
                compare_with_stored_digest(actual)
            }
            // We have a record, but the path on disk is missing.
            // We can't calculate the digest. We'll use the recorded
            // one.
            Diff::ActualMissing { .. } => {
                match stored_content_digest {
                    Some(record) => Diff::ActualMissing { record: *record },
                    // if the both actual and the record are
                    // missing, they are identical in their inexistence.
                    // how can a man without hands clap?
                    None => Diff::Identical,
                }
            }
        };

        Ok((xe, diff_content_digest))
    } else {
        Ok((xe, Diff::Skipped))
    }
}

/// Used to signal diff channels to calculate diffs of the requested entity.
pub struct DiffRequest {
    xe: XvcEntity,
}

type FileContentDigestDiffHandlers = (
    Sender<Option<DiffRequest>>,
    Receiver<Option<Diff<ContentDigest>>>,
    JoinHandle<()>,
);

/// This is a channel version of [diff_file_content_digest]. It creates a thread that listens to requests
/// diff_request channel and sends the calculated diffs to the diff_result channel.
///
/// The thread will exit when the other ends of channel is dropped or when the diff_request_rcv gets a None.
/// It sends a None to the diff_result_snd when it exits.
#[allow(clippy::too_many_arguments)]
pub fn make_file_content_digest_diff_handler(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    stored_xvc_path_store: &SharedXStore<XvcPath>,
    stored_xvc_metadata_store: &SharedXStore<XvcMetadata>,
    stored_content_digest_store: &SharedXStore<ContentDigest>,
    stored_text_or_binary_store: &SharedXStore<FileTextOrBinary>,
    requested_text_or_binary: Option<FileTextOrBinary>,
    requested_hash_algorithm: Option<HashAlgorithm>,
) -> Result<FileContentDigestDiffHandlers> {
    let algorithm: HashAlgorithm =
        requested_hash_algorithm.unwrap_or_else(|| HashAlgorithm::from_conf(xvc_root.config()));

    let (diff_request_snd, diff_request_rcv) =
        crossbeam_channel::bounded::<Option<DiffRequest>>(crate::CHANNEL_CAPACITY);
    let (diff_result_snd, diff_result_rcv) = crossbeam_channel::bounded(crate::CHANNEL_CAPACITY);

    let output_snd = output_snd.clone();
    let xvc_root = xvc_root.clone();
    let stored_xvc_path_store = stored_xvc_path_store.clone();
    let stored_xvc_metadata_store = stored_xvc_metadata_store.clone();
    let stored_content_digest_store = stored_content_digest_store.clone();
    let stored_text_or_binary_store = stored_text_or_binary_store.clone();

    let handle = thread::spawn(move || {
        while let Ok(Some(diff_request)) = diff_request_rcv.recv() {
            let stored_xvc_path_store = stored_xvc_path_store.read().unwrap();
            let stored_xvc_metadata_store = stored_xvc_metadata_store.read().unwrap();
            let stored_content_digest_store = stored_content_digest_store.read().unwrap();
            let stored_text_or_binary_store = stored_text_or_binary_store.read().unwrap();
            let xe = diff_request.xe;
            let xvc_path = stored_xvc_path_store.get(&xe).unwrap();
            let xvc_metadata = stored_xvc_metadata_store.get(&xe).unwrap();
            if xvc_metadata.is_file() {
                let stored_content_digest = stored_content_digest_store.get(&xe);
                let text_or_binary = requested_text_or_binary.unwrap_or_else(|| {
                    stored_text_or_binary_store
                        .get(&xe)
                        .cloned()
                        .unwrap_or_default()
                });
                let path = xvc_path.to_absolute_path(&xvc_root);

                if path.is_file() {
                    let actual_content_digest_res =
                        ContentDigest::new(&path, algorithm, text_or_binary.as_inner());
                    match (actual_content_digest_res, stored_content_digest) {
                        (Ok(actual), Some(stored)) => {
                            if actual == *stored {
                                diff_result_snd.send(Some(Diff::Identical)).unwrap();
                            } else {
                                diff_result_snd
                                    .send(Some(Diff::Different {
                                        actual,
                                        record: *stored,
                                    }))
                                    .unwrap();
                            }
                        }
                        (Err(e), _) => {
                            debug!(
                                output_snd,
                                "Failed to calculate content digest of {:?}: {}", path, e
                            );
                        }
                        (Ok(actual), None) => {
                            diff_result_snd
                                .send(Some(Diff::RecordMissing { actual }))
                                .unwrap();
                        }
                    }
                } else if let Some(stored_content_digest) = stored_content_digest {
                    diff_result_snd
                        .send(Some(Diff::ActualMissing {
                            record: *stored_content_digest,
                        }))
                        .unwrap();
                } else {
                    diff_result_snd.send(Some(Diff::Identical)).unwrap();
                }
            }
        }

        // Send None to indicate that the thread is exiting.
        diff_result_snd.send(None).unwrap();
    });

    Ok((diff_request_snd, diff_result_rcv, handle))
}

/// Check whether content digests of files and directories in xvc_path_store has
/// changed.
///
/// This is used to identify the files that requires attention in several
/// commands, like recheck or carry-in.
#[allow(clippy::too_many_arguments)]
pub fn diff_content_digest(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    stored_xvc_path_store: &XvcStore<XvcPath>,
    stored_xvc_metadata_store: &XvcStore<XvcMetadata>,
    stored_content_digest_store: &XvcStore<ContentDigest>,
    stored_text_or_binary_store: &XvcStore<FileTextOrBinary>,
    xvc_path_diff_store: &DiffStore<XvcPath>,
    xvc_metadata_diff_store: &DiffStore<XvcMetadata>,
    requested_text_or_binary: Option<FileTextOrBinary>,
    requested_hash_algorithm: Option<HashAlgorithm>,
    parallel: bool,
) -> DiffStore<ContentDigest> {
    let entities: HashSet<XvcEntity> = xvc_path_diff_store.keys().copied().collect();
    let algorithm: HashAlgorithm =
        requested_hash_algorithm.unwrap_or_else(|| HashAlgorithm::from_conf(xvc_root.config()));

    let diff_file = |xe| -> Result<(XvcEntity, Diff<ContentDigest>)> {
        let xvc_path_diff = xvc_path_diff_store
            .get(&xe)
            .unwrap_or(&Diff::<XvcPath>::Skipped);
        let xvc_metadata_diff = xvc_metadata_diff_store
            .get(&xe)
            .unwrap_or(&Diff::<XvcMetadata>::Skipped);

        let text_or_binary = requested_text_or_binary.unwrap_or_else(|| {
            stored_text_or_binary_store
                .get(&xe)
                .copied()
                .unwrap_or_else(|| FileTextOrBinary::from_conf(xvc_root.config()))
        });

        diff_file_content_digest(
            output_snd,
            xvc_root,
            xe,
            xvc_path_diff,
            xvc_metadata_diff,
            stored_xvc_path_store,
            stored_content_digest_store,
            algorithm,
            text_or_binary,
        )
    };

    let diff_dir = |xe, file_content_digest_store: &DiffStore<ContentDigest>| {
        let from_store = |xe| stored_xvc_path_store.get(xe).unwrap();
        let the_dir = match xvc_path_diff_store.get(xe) {
            None | Some(Diff::Identical) | Some(Diff::Skipped) => from_store(xe),
            Some(Diff::RecordMissing { actual }) => actual,
            Some(Diff::ActualMissing { record }) => record,
            Some(Diff::Different { actual, .. }) => actual,
        };

        let child_path_entities = entities
            .iter()
            .filter_map(|xe| {
                let xvc_path = match xvc_path_diff_store.get(xe) {
                    None | Some(Diff::Identical) | Some(Diff::Skipped) => from_store(xe),
                    Some(Diff::RecordMissing { actual }) => actual,
                    Some(Diff::ActualMissing { record }) => record,
                    Some(Diff::Different { actual, .. }) => actual,
                };

                if xvc_path.starts_with(the_dir) {
                    Some(*xe)
                } else {
                    None
                }
            })
            .sorted()
            .collect::<Vec<XvcEntity>>();

        diff_dir_content_digest(
            stored_content_digest_store.get(xe),
            stored_content_digest_store,
            file_content_digest_store,
            &child_path_entities,
        )
    };

    let file_type = |xe| {
        stored_xvc_metadata_store
            .get(&xe)
            .map(|xmd| Ok(xmd.file_type))
            .unwrap_or_else(|| match xvc_metadata_diff_store.get(&xe) {
                None | Some(Diff::Identical) | Some(Diff::Skipped) => Err(anyhow!(
                    "Cannot determine file type for path {} (entity {})",
                    stored_xvc_path_store.get(&xe).unwrap(),
                    xe
                )),
                Some(Diff::RecordMissing { actual }) => Ok(actual.file_type),
                Some(Diff::ActualMissing { record }) => Ok(record.file_type),
                Some(Diff::Different { record, actual }) => match actual.file_type {
                    XvcFileType::Missing => Ok(record.file_type),
                    _ => Ok(actual.file_type),
                },
            })
    };

    let file_entities = entities
        .iter()
        .filter(|xe| {
            file_type(**xe)
                .map(|ft| ft == XvcFileType::File)
                .unwrap_or(false)
        })
        .copied()
        .collect::<HashSet<XvcEntity>>();

    let dir_entities = entities
        .iter()
        .filter(|xe| {
            file_type(**xe)
                .map(|ft| ft == XvcFileType::Directory)
                .unwrap_or(false)
        })
        .copied()
        .collect::<HashSet<XvcEntity>>();

    entities
        .difference(&file_entities)
        .copied()
        .collect::<HashSet<_>>()
        .difference(&dir_entities)
        .for_each(|xe| {
            let ep = stored_xvc_path_store
                .get(xe)
                .map(|xp| xp.to_string())
                .unwrap_or_else(|| format!("{:?}", xvc_path_diff_store.get(xe).unwrap()));
            error!(
                output_snd,
                "Skipping {} because it is neither a file nor a directory", ep
            );
        });

    let (file_content_digest_diff_store, dir_content_digest_diff_store) = if parallel {
        let file_content_digest_diff_store = file_entities
            .par_iter()
            .filter_map(|xe| match diff_file(*xe) {
                Ok((_, diff)) => Some((*xe, diff)),
                Err(e) => {
                    error!(output_snd, "{}", e);
                    None
                }
            })
            .collect::<DiffStore<ContentDigest>>();

        let dir_content_digest_diff_store = dir_entities
            .par_iter()
            .filter_map(|e| match diff_dir(e, &file_content_digest_diff_store) {
                Ok(d) => Some((*e, d)),
                Err(e) => {
                    error!(output_snd, "{}", e);
                    None
                }
            })
            .collect::<DiffStore<ContentDigest>>();

        (
            file_content_digest_diff_store,
            dir_content_digest_diff_store,
        )
    } else {
        let file_content_digest_diff_store = file_entities
            .iter()
            .filter_map(|xe| match diff_file(*xe) {
                Ok((_, diff)) => Some((*xe, diff)),
                Err(e) => {
                    error!(output_snd, "{}", e);
                    None
                }
            })
            .collect::<DiffStore<ContentDigest>>();

        let dir_content_digest_diff_store = dir_entities
            .iter()
            .filter_map(|e| match diff_dir(e, &file_content_digest_diff_store) {
                Ok(d) => Some((*e, d)),
                Err(e) => {
                    error!(output_snd, "{}", e);
                    None
                }
            })
            .collect::<DiffStore<ContentDigest>>();

        (
            file_content_digest_diff_store,
            dir_content_digest_diff_store,
        )
    };

    let mut diff_store = DiffStore::with_capacity(
        file_content_digest_diff_store.len() + dir_content_digest_diff_store.len(),
    );

    diff_store.extend(file_content_digest_diff_store);
    diff_store.extend(dir_content_digest_diff_store);
    diff_store
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
            .ok_or(EcsError::CannotFindKeyInStore {
                key: xe.to_string(),
            })?;
        match xvc_content_diff {
            Diff::Identical | Diff::Skipped => {
                let content = stored_xvc_content_store.get(xe).ok_or(
                    xvc_ecs::error::Error::CannotFindKeyInStore {
                        key: xe.to_string(),
                    },
                )?;
                content_digest_bytes.extend(content.digest().digest);
            }
            Diff::RecordMissing { actual } => {
                content_digest_bytes.extend(actual.digest().digest);
            }
            Diff::Different { actual, .. } => {
                content_digest_bytes.extend(actual.digest().digest);
            }
            Diff::ActualMissing { .. } => {
                // This is to make sure the content digest is different when
                // all records are missing or their order has changed.
                let entity_bytes: u128 = (*xe).into();
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
