//! The home of `xvc file move` command.
//!
//! It contains [`MoveCLI`] to define command line for the command [`cmd_move`] as the entry point.
use std::fs;
use std::path::Path;

use crate::copy::{
    check_if_destination_is_a_directory, check_if_sources_have_changed, get_source_path_metadata,
    recheck_destination,
};

use crate::Result;
use anyhow::anyhow;
use clap::Parser;

use itertools::Itertools;
use xvc_config::FromConfigKey;
use xvc_core::{RecheckMethod, XvcFileType, XvcMetadata, XvcPath, XvcRoot};
use xvc_ecs::{HStore, XvcEntity, XvcStore};
use xvc_logging::{info, uwr, watch, XvcOutputSender};

/// CLI for `xvc file copy`.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case", author, version)]
pub struct MoveCLI {
    /// How the destination should be rechecked: One of copy, symlink, hardlink, reflink.
    ///
    /// Note: Reflink uses copy if the underlying file system doesn't support it.
    #[arg(long, alias = "as")]
    pub recheck_method: Option<RecheckMethod>,

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

/// Return movable [`XvcPath`] entities.
/// Unlike [`get_copy_source_dest_store`], this function doesn't create any new entities. The move sources should
/// already be recorded.
/// `stored_xvc_path_store` and `stored_xvc_metadata_store` are results of `load_targets_from_store`, and
/// `source_xvc_paths` and `source_xvc_metadata` are loaded from `targets_from_disk`.
pub fn get_move_source_dest_store(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    stored_xvc_path_store: &XvcStore<XvcPath>,
    stored_xvc_metadata_store: &XvcStore<XvcMetadata>,
    source_xvc_paths: &HStore<XvcPath>,
    source_xvc_metadata: &HStore<XvcMetadata>,
    destination: &str,
) -> Result<HStore<XvcPath>> {
    // Create targets in the store
    // If destination is a directory, check if exists and create if not.
    // If destination is a file, check if exists and return error if it does and
    // force is not set.
    if destination.ends_with('/') {
        let dir_path = XvcPath::new(
            xvc_root,
            xvc_root,
            Path::new(destination.strip_suffix('/').unwrap()),
        )?;

        check_if_destination_is_a_directory(
            &dir_path,
            stored_xvc_path_store,
            stored_xvc_metadata_store,
        )?;

        check_if_sources_have_changed(
            output_snd,
            xvc_root,
            stored_xvc_path_store,
            stored_xvc_metadata_store,
            source_xvc_paths,
            source_xvc_metadata,
        )?;

        let mut source_dest_store = HStore::new();

        let mut error_paths = vec![];

        for (source_xe, source_path) in source_xvc_paths.iter() {
            let dest_path = dir_path.join(source_path).unwrap();

            match stored_xvc_path_store.entities_for(&dest_path) {
                Some(_v) => {
                    error_paths.push(dest_path);
                }
                None => {
                    source_dest_store.insert(*source_xe, dest_path);
                }
            }
        }

        if !error_paths.is_empty() {
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
            stored_xvc_metadata_store,
            source_xvc_paths,
            source_xvc_metadata,
        )?;

        let current_dir = xvc_root.config().current_dir()?;
        let source_xe = source_xvc_paths.keys().next().unwrap();

        let mut source_dest_store = HStore::<XvcPath>::with_capacity(1);
        let dest_path = XvcPath::new(xvc_root, current_dir, Path::new(destination))?;

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
    output_snd: &XvcOutputSender,
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
        &opts.destination,
    )?;

    xvc_root.with_store_mut(|xvc_path_store: &mut XvcStore<XvcPath>| {
        xvc_root.with_store_mut(|xvc_metadata_store: &mut XvcStore<XvcMetadata>| {
            for (source_xe, dest_path) in source_dest_store.iter() {
                xvc_path_store.update(*source_xe, dest_path.clone());
                // Create destination parent directory records if they don't exist
                for parent in dest_path.parents() {
                    let parent_entities = xvc_path_store.entities_for(&parent);
                    if parent_entities.is_none() || parent_entities.unwrap().is_empty() {
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
    xvc_root.with_store_mut(|recheck_method_store: &mut XvcStore<RecheckMethod>| {
        for (source_xe, dest_path) in source_dest_store.iter() {
            let source_path = stored_xvc_path_store.get(source_xe).unwrap();
            let source_recheck_method = recheck_method_store
                .get(source_xe)
                .copied()
                .unwrap_or_else(|| RecheckMethod::from_conf(xvc_root.config()));

            let dest_recheck_method = if let Some(given_recheck_method) = opts.recheck_method {
                given_recheck_method
            } else {
                source_recheck_method
            };

            if dest_recheck_method != source_recheck_method {
                recheck_method_store.update(*source_xe, dest_recheck_method);
            }
            watch!(source_recheck_method);
            watch!(dest_recheck_method);
            match (source_recheck_method, dest_recheck_method) {
                // If both are copy, move the file
                (RecheckMethod::Copy, RecheckMethod::Copy) => {
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
                    if source_path.exists() {
                        uwr!(fs::remove_file(&source_path), output_snd);
                    }
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
