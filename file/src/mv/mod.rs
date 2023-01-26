use std::fs;
use std::path::Path;

use crate::common::compare::{
    diff_content_digest, diff_xvc_path_metadata, make_file_content_digest_diff_handler,
};
use crate::common::gitignore::make_ignore_handler;
use crate::common::{
    filter_targets_from_store, load_targets_from_store, xvc_path_metadata_map_from_disk,
    FileTextOrBinary,
};
use crate::copy::{
    check_if_destination_is_a_directory, check_if_sources_have_changed, get_source_path_metadata,
    recheck_destination,
};
use crate::recheck::{make_recheck_handler, RecheckOperation};
use crate::{recheck, Result};
use anyhow::anyhow;
use clap::Parser;
use crossbeam_channel::Sender;
use itertools::Itertools;
use xvc_config::FromConfigKey;
use xvc_core::{CacheType, ContentDigest, XvcFileType, XvcMetadata, XvcPath, XvcRoot};
use xvc_ecs::{HStore, R11Store, XvcEntity, XvcStore};
use xvc_logging::{debug, error, info, watch, XvcOutputLine};

/// CLI for `xvc file copy`.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case", author, version)]
pub struct MoveCLI {
    /// How the destination should be rechecked: One of copy, symlink, hardlink, reflink.
    ///
    /// Note: Reflink uses copy if the underlying file system doesn't support it.
    #[arg(long, alias = "as")]
    pub cache_type: Option<CacheType>,

    /// Do not recheck the destination files
    /// This is useful when you want to copy only records, without updating the
    /// workspace.
    #[arg(long)]
    pub no_recheck: bool,

    /// Source file, glob or directory within the workspace.
    ///
    /// If the source ends with a slash, it's considered a directory and all
    /// files in that directory are copied.
    ///
    /// If there are multiple source files, the destination must be a directory.
    #[arg()]
    pub source: String,

    /// Location we move file(s) to within the workspace.
    ///
    /// If this ends with a slash, it's considered a directory and
    /// created if it doesn't exist.
    ///
    /// If the number of source files is more than one, the destination must be a directory.
    #[arg()]
    pub destination: String,
}

pub fn get_move_source_dest_store(
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    stored_xvc_path_store: &XvcStore<XvcPath>,
    stored_metadata_store: &XvcStore<XvcMetadata>,
    source_xvc_paths: &HStore<XvcPath>,
    source_metadata: &HStore<XvcMetadata>,
    source: &str,
    destination: &str,
) -> Result<HStore<XvcPath>> {
    // Create targets in the store
    // If destination is a directory, check if exists and create if not.
    // If destination is a file, check if exists and return error if it does and
    // force is not set.
    if destination.ends_with('/') {
        let dir_path = XvcPath::new(
            &xvc_root,
            &xvc_root,
            Path::new(destination.strip_suffix('/').unwrap()),
        )?;

        check_if_destination_is_a_directory(
            &dir_path,
            stored_xvc_path_store,
            stored_metadata_store,
        )?;

        check_if_sources_have_changed(
            output_snd,
            xvc_root,
            stored_xvc_path_store,
            stored_metadata_store,
            source_xvc_paths,
            source_metadata,
        )?;

        let mut source_dest_store = HStore::new();

        let mut error_paths = vec![];

        for (source_xe, source_path) in source_xvc_paths.iter() {
            let dest_path = dir_path.join(source_path).unwrap();

            match stored_xvc_path_store.entities_for(&dest_path) {
                Some(v) => {
                    error_paths.push(dest_path);
                }
                None => {
                    source_dest_store.insert(*source_xe, dest_path);
                }
            }
        }

        if error_paths.len() > 0 {
            Err(anyhow!(
                "Destination files already exist. Operation cancelled. Delete them first: {}",
                error_paths.iter().map(|xp| xp.to_string()).join("\n")
            )
            .into())
        } else {
            Ok(source_dest_store)
        }
    } else {
        // Destination doesn't end with '/'
        if source_xvc_paths.len() > 1 {
            return Err(
                anyhow!("Destination must be a directory if multiple sources are given").into(),
            );
        }

        check_if_sources_have_changed(
            output_snd,
            xvc_root,
            stored_xvc_path_store,
            stored_metadata_store,
            source_xvc_paths,
            source_metadata,
        )?;

        let current_dir = xvc_root.config().current_dir()?;
        let source_xe = source_xvc_paths.keys().next().unwrap();

        let mut source_dest_store = HStore::<XvcPath>::with_capacity(1);
        let dest_path = XvcPath::new(&xvc_root, current_dir, Path::new(destination))?;

        match stored_xvc_path_store.entity_by_value(&dest_path) {
            Some(_) => Err(anyhow!(
                "Destination file {} already exists. Delete it first.",
                dest_path
            )
            .into()),
            None => {
                source_dest_store.insert(*source_xe, dest_path);
                Ok(source_dest_store)
            }
        }
    }
}

/// Entry point for `xvc file move`
pub(crate) fn cmd_move(
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    opts: MoveCLI,
) -> Result<()> {
    // Get all files to move
    let stored_metadata_store = xvc_root.load_store::<XvcMetadata>()?;
    let stored_xvc_path_store = xvc_root.load_store::<XvcPath>()?;
    let (source_xvc_paths, source_metadata) = get_source_path_metadata(
        xvc_root,
        &stored_xvc_path_store,
        &stored_metadata_store,
        &opts.source,
        &opts.destination,
    )?;

    let source_dest_store = get_move_source_dest_store(
        output_snd,
        xvc_root,
        &stored_xvc_path_store,
        &stored_metadata_store,
        &source_xvc_paths,
        &source_metadata,
        &opts.source,
        &opts.destination,
    )?;

    xvc_root.with_store_mut(|xvc_path_store: &mut XvcStore<XvcPath>| {
        xvc_root.with_store_mut(|xvc_metadata_store: &mut XvcStore<XvcMetadata>| {
            for (source_xe, dest_path) in source_dest_store.iter() {
                xvc_path_store.update(*source_xe, dest_path.clone());
                // Create destination parent directory records if they don't exist
                for parent in dest_path.parents() {
                    let parent_entities = xvc_path_store.entities_for(&parent);
                    if parent_entities.is_none() || parent_entities.unwrap().len() == 0 {
                        let parent_entity = xvc_root.new_entity();
                        xvc_path_store.insert(parent_entity, parent.clone());
                        xvc_metadata_store.insert(
                            parent_entity,
                            XvcMetadata {
                                file_type: XvcFileType::Directory,
                                ..Default::default()
                            },
                        );
                    }
                }
            }
            Ok(())
        })?;
        Ok(())
    })?;

    let mut recheck_entities = Vec::<XvcEntity>::new();
    watch!(source_dest_store);
    xvc_root.with_store_mut(|cache_type_store: &mut XvcStore<CacheType>| {
        for (source_xe, dest_path) in source_dest_store.iter() {
            let source_path = stored_xvc_path_store.get(source_xe).unwrap();
            let source_cache_type = cache_type_store
                .get(source_xe)
                .copied()
                .unwrap_or_else(|| CacheType::from_conf(&xvc_root.config()));

            let dest_cache_type = if let Some(given_cache_type) = opts.cache_type {
                given_cache_type
            } else {
                source_cache_type
            };

            if dest_cache_type != source_cache_type {
                cache_type_store.update(*source_xe, dest_cache_type);
            }
            watch!(source_cache_type);
            watch!(dest_cache_type);
            match (source_cache_type, dest_cache_type) {
                // If both are copy, move the file
                (CacheType::Copy, CacheType::Copy) => {
                    let source_path = source_path.to_absolute_path(xvc_root);
                    let dest_path = dest_path.to_absolute_path(xvc_root);
                    if source_path != dest_path {
                        // If no-recheck is given, this effectively works like a delete.
                        if opts.no_recheck {
                            fs::remove_file(&source_path)?;
                        } else {
                            let parent = dest_path.parent().unwrap();
                            if !parent.exists() {
                                fs::create_dir_all(parent)?;
                            }
                            fs::rename(&source_path, &dest_path)?;
                        }
                    } else {
                        info!(
                            output_snd,
                            "Source and destination are the same. Skipping move for {}",
                            source_path
                        );
                    }
                }
                // For others just delete the source and recheck.
                // Moving symlinks relatively etc. is too much complexity for little gain.
                _ => {
                    let source_path = source_path.to_absolute_path(xvc_root);
                    watch!(source_path);
                    fs::remove_file(&source_path)?;
                    recheck_entities.push(*source_xe);
                    watch!(recheck_entities);
                }
            }
        }
        Ok(())
    })?;

    watch!(recheck_entities);

    if !opts.no_recheck {
        recheck_destination(output_snd, xvc_root, &recheck_entities)?;
    }

    Ok(())
}
