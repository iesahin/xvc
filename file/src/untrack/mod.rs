use std::collections::{HashMap, HashSet};
use std::fs::{self, create_dir_all};
use std::path::PathBuf;

use crate::common::gitignore::make_ignore_handler;
use crate::common::{
    filter_targets_from_store, load_targets_from_store, recheck_from_cache, FileTextOrBinary,
};
use crate::recheck::{make_recheck_handler, RecheckOperation};
use crate::{recheck, Result};
use clap::Parser;
use crossbeam_channel::Sender;
use derive_more::From;
use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use xvc_core::types::recheckmethod;
use xvc_core::{ContentDigest, RecheckMethod, XvcCachePath, XvcMetadata, XvcPath, XvcRoot};
use xvc_ecs::ecs::event::EventLog;
use xvc_ecs::{HStore, XvcEntity, XvcStore};
use xvc_logging::{debug, error, output, panic, uwo, uwr, watch};

/// Remove files from tracking and possibly delete them
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, From, Parser)]
#[command(rename_all = "kebab-case")]
pub struct UntrackCLI {
    /// Restore all versions to a directory before deleting the cache files.
    #[arg(long)]
    restore_versions: Option<String>,

    /// Delete all files also from given storages
    #[arg(long)]
    delete_from_storages: Option<Vec<String>>,

    /// Files/directories to untrack
    #[arg()]
    targets: Vec<String>,
}

pub fn cmd_untrack(
    output_snd: &Sender<xvc_logging::XvcOutputLine>,
    xvc_root: &XvcRoot,
    opts: UntrackCLI,
) -> Result<()> {
    // Load targets from store
    let current_dir = xvc_root.config().current_dir()?;
    let all_paths = xvc_root.load_store()?;
    let all_content_digests = xvc_root.load_store()?;
    let untrack_targets =
        filter_targets_from_store(xvc_root, &all_paths, current_dir, &Some(opts.targets))?;

    // Get cache paths for each

    let mut all_cache_paths: HStore<Vec<XvcCachePath>> = HStore::new();

    // Find all cache paths
    // We have 1-1 relationship between content digests and paths.
    // So, in order to get earlier versions, we check the event log.
    for (xe, xp) in all_paths.iter() {
        let path_digest_events: EventLog<ContentDigest> =
            all_content_digests.all_event_log_for_entity(*xe)?;
        let cache_paths = path_digest_events
            .iter()
            .filter_map(|cd_event| match cd_event {
                xvc_ecs::ecs::event::Event::Add { entity, value } => {
                    let xcp = uwr!(XvcCachePath::new(xp, value), output_snd
                 );

                    Some(xcp)
                }
                xvc_ecs::ecs::event::Event::Remove { entity } => {
                    // We don't delete ContentDigests of available XvcPaths.
                    // This is an error.
                    error!(
                    output_snd,
                    "There shouldn't be a remove event for content digest of {xp}. Please report this. {}",
                    entity
                );
                    None
                }
            })
            .collect();
        all_cache_paths.insert(*xe, cache_paths);
    }

    // Get inverted index

    let mut entities_for_cache_path: HashMap<XvcCachePath, HashSet<XvcEntity>> = HashMap::new();

    for (xe, cache_paths) in all_cache_paths.iter() {
        for cp in cache_paths {
            if !entities_for_cache_path.contains_key(cp) {
                entities_for_cache_path.insert(cp.clone(), HashSet::new());
            }
            let mut entity_set = entities_for_cache_path.get_mut(cp).unwrap();
            entity_set.insert(*xe);
        }
    }

    // Recheck untrack targets with RecheckMethod::Copy, the links will be broken after deletion.

    let (ignore_snd, ignore_thread) = make_ignore_handler(output_snd, xvc_root)?;
    let (recheck_op_snd, recheck_thread) = make_recheck_handler(output_snd, xvc_root, &ignore_snd)?;

    for (xe, xvc_path) in untrack_targets.iter() {
        let abs_path = xvc_path.to_absolute_path(xvc_root);
        if !abs_path.symlink_metadata().unwrap().is_file() {
            let content_digest = all_content_digests[&xe];
            let recheck_method = RecheckMethod::Copy;
            uwr!(
                recheck_op_snd.send(Some(RecheckOperation::Recheck {
                    xvc_path: xvc_path.clone(),
                    content_digest,
                    recheck_method,
                })),
                output_snd
            );
        } else {
            debug!(output_snd, "Path is already a file: {xvc_path}");
        }
    }

    uwr!(recheck_op_snd.send(None), output_snd);
    uwr!(recheck_thread.join(), output_snd);
    uwr!(ignore_snd.send(None), output_snd);
    uwr!(ignore_thread.join(), output_snd);

    // Find cache paths of the targets

    let untrack_cache_paths: HStore<Vec<XvcCachePath>> =
        all_cache_paths.subset(untrack_targets.keys().copied())?;
    let untrack_entities: HashSet<XvcEntity> = HashSet::from_iter(untrack_targets.keys().copied());

    // We can restore versions here
    if let Some(restore_dir) = opts.restore_versions {
        const VERSION_ID_LEN: usize = 15;
        let abs_restore_dir = current_dir.join(restore_dir);
        watch!(abs_restore_dir);
        if abs_restore_dir.is_file() {
            panic!(
                output_snd,
                "There is already a file at {}. Aborting.", abs_restore_dir
            );
        }

        if !abs_restore_dir.exists() {
            fs::create_dir_all(&abs_restore_dir);
        }

        untrack_cache_paths.par_iter().for_each(|(xe, vec_cp)| {
            let xvc_path = all_paths[xe].clone();
            let destination_path = xvc_path.to_absolute_path(&abs_restore_dir);
            let destination_dir = destination_path.parent().unwrap();
            let stem = xvc_path
                .as_relative_path()
                .file_stem()
                .unwrap_or_else(|| "");
            let extension = xvc_path
                .as_relative_path()
                .extension()
                .unwrap_or_else(|| "");

            if !destination_dir.exists() {
                uwr!(fs::create_dir_all(&destination_dir), output_snd);
            }

            // TODO: Parallelize this if we have a lot of versions
            vec_cp.iter().sorted().for_each(|xcp| {
                let from = xcp.to_absolute_path(xvc_root);
                // The suffix will be like b3-123-456-789
                let xcp_suffix = xcp.digest_string(VERSION_ID_LEN);
                // The file name will be like {restore_dir}/{xvc_path_parent}/{xvc_path_stem}-{cache_prefix}.{extension}
                let to = PathBuf::from(format!(
                    "{destination_dir}/{stem}-{xcp_suffix}.{extension}",
                    destination_dir = destination_dir.to_str().unwrap()
                ));
                uwr!(fs::copy(&from, &to), output_snd);
                output!(output_snd, "[COPY] {from} -> {}", to.to_str().unwrap());
            });
        })
    }

    let mut deletable_paths = Vec::<XvcCachePath>::new();
    // Report the differences if found
    for (xe, vec_cp) in untrack_cache_paths {
        for cp in vec_cp {
            let entities_pointing_to_cp =
                HashSet::from_iter(entities_for_cache_path[&cp].iter().copied());
            let mut deletable = true;
            entities_pointing_to_cp
                .difference(&untrack_entities)
                .for_each(|other_xe| {
                    let this_xp = all_paths.get(&xe).unwrap();
                    let other_xp = all_paths.get(other_xe).unwrap();
                    output!(
                        output_snd,
                        "Not deleting {} (for {}) because it's also used by {}",
                        cp,
                        this_xp,
                        other_xp
                    );
                    deletable = false;
                });

            if deletable {
                deletable_paths.push(cp);
            }
        }
    }

    // Remove all targets from store
    // We use nested stores to make this transactional
    xvc_root.with_store_mut(|xp_store: &mut XvcStore<XvcPath>| {
        xvc_root.with_store_mut(|xmd_store: &mut XvcStore<XvcMetadata>| {
            xvc_root.with_store_mut(|cache_type_store: &mut XvcStore<RecheckMethod>| {
                xvc_root.with_store_mut(
                    |text_or_binary_store: &mut XvcStore<FileTextOrBinary>| {
                        xvc_root.with_store_mut(
                            |content_digest_store: &mut XvcStore<ContentDigest>| {
                                for xe in &untrack_entities {
                                    content_digest_store.remove(*xe);
                                }
                                Ok(())
                            },
                        )?;
                        for xe in &untrack_entities {
                            text_or_binary_store.remove(*xe);
                        }
                        Ok(())
                    },
                )?;
                for xe in &untrack_entities {
                    cache_type_store.remove(*xe);
                }
                Ok(())
            })?;
            for xe in &untrack_entities {
                xmd_store.remove(*xe);
            }
            Ok(())
        })?;
        for xe in &untrack_entities {
            xp_store.remove(*xe);
        }
        Ok(())
    })?;
    Ok(())
}
