//! Home for `xvc file copy` command.
//!
//! It contains [`CopyCLI`] to handle command line options and [`cmd_copy`] to execute the command.
use std::path::Path;

use crate::common::compare::{diff_content_digest, diff_xvc_path_metadata};
use crate::common::gitignore::make_ignore_handler;
use crate::common::{filter_targets_from_store, xvc_path_metadata_map_from_disk, FileTextOrBinary};
use crate::recheck::{make_recheck_handler, RecheckOperation};
use crate::Result;
use anyhow::anyhow;
use clap::Parser;

use xvc_core::{ContentDigest, Diff, RecheckMethod, XvcFileType, XvcMetadata, XvcPath, XvcRoot};
use xvc_ecs::{HStore, R11Store, XvcEntity, XvcStore};
use xvc_logging::{debug, error, watch, XvcOutputSender};

/// CLI for `xvc file copy`.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case", author, version)]
pub struct CopyCLI {
    /// How the targets should be rechecked: One of copy, symlink, hardlink, reflink.
    ///
    /// Note: Reflink uses copy if the underlying file system doesn't support it.
    #[arg(long, alias = "as")]
    pub recheck_method: Option<RecheckMethod>,

    /// Force even if target exists.
    #[arg(long)]
    pub force: bool,

    /// Do not recheck the destination files
    /// This is useful when you want to copy only records, without updating the
    /// workspace.
    #[arg(long)]
    pub no_recheck: bool,

    /// When copying multiple files, by default whole path is copied to the destination. This
    /// option sets the destination to be created with the file name only.
    #[arg(long)]
    pub name_only: bool,

    /// Source file, glob or directory within the workspace.
    ///
    /// If the source ends with a slash, it's considered a directory and all
    /// files in that directory are copied.
    ///
    /// If the number of source files is more than one, the destination must be a directory.
    #[arg()]
    pub source: String,

    /// Location we copy file(s) to within the workspace.
    ///
    /// If the target ends with a slash, it's considered a directory and
    /// created if it doesn't exist.
    ///
    /// If the number of source files is more than one, the destination must be a directory.
    #[arg()]
    pub destination: String,
}

pub(crate) fn get_source_path_metadata(
    xvc_root: &XvcRoot,
    stored_xvc_path_store: &XvcStore<XvcPath>,
    stored_xvc_metadata_store: &XvcStore<XvcMetadata>,
    source: &str,
    destination: &str,
) -> Result<(HStore<XvcPath>, HStore<XvcMetadata>)> {
    let source_targets = if source.ends_with('/') {
        let mut source = source.to_string();
        source.push('*');
        vec![source]
    } else {
        vec![source.to_string()]
    };

    let current_dir = xvc_root.config().current_dir()?;
    let all_sources = filter_targets_from_store(
        xvc_root,
        stored_xvc_path_store,
        current_dir,
        &Some(source_targets),
    )?;
    let source_metadata = stored_xvc_metadata_store.subset(all_sources.keys().copied())?;
    let source_metadata_files = source_metadata.filter(|_xe, md| md.is_file()).cloned();

    if source_metadata_files.len() > 1 && !destination.ends_with('/') {
        return Err(anyhow!("Target must be a directory if multiple sources are given").into());
    }

    let source_xvc_path_files = all_sources.subset(source_metadata_files.keys().copied())?;

    Ok((source_xvc_path_files, source_metadata_files))
}

pub(crate) fn check_if_destination_is_a_directory(
    dir_path: &XvcPath,
    stored_xvc_path_store: &XvcStore<XvcPath>,
    stored_metadata_store: &XvcStore<XvcMetadata>,
) -> Result<()> {
    let current_dir_entity = stored_xvc_path_store.entities_for(dir_path).map(|v| v[0]);

    let current_dir_metadata = current_dir_entity.and_then(|e| stored_metadata_store.get(&e));

    if let Some(current_dir_metadata) = current_dir_metadata {
        if !current_dir_metadata.is_dir() {
            return Err(anyhow!(
                        "Destination is not recorded as a directory. Please move or delete the destination first."
                    )
                    .into());
        }
    }
    Ok(())
}

pub(crate) fn check_if_sources_have_changed(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    stored_xvc_path_store: &XvcStore<XvcPath>,
    stored_metadata_store: &XvcStore<XvcMetadata>,
    source_xvc_paths: &HStore<XvcPath>,
    _source_metadata: &HStore<XvcMetadata>,
) -> Result<()> {
    // We don't parallelize the diff operation because we abort all operations if there is a single changed file.
    let pmm = xvc_path_metadata_map_from_disk(xvc_root, source_xvc_paths);
    let xvc_path_metadata_diff =
        diff_xvc_path_metadata(xvc_root, stored_xvc_path_store, stored_metadata_store, &pmm);
    let stored_content_digest_store = xvc_root.load_store::<ContentDigest>()?;
    let stored_text_or_binary_store = xvc_root.load_store::<FileTextOrBinary>()?;
    let content_digest_diff = diff_content_digest(
        output_snd,
        xvc_root,
        stored_xvc_path_store,
        stored_metadata_store,
        &stored_content_digest_store,
        &stored_text_or_binary_store,
        &xvc_path_metadata_diff.0,
        &xvc_path_metadata_diff.1,
        None,
        None,
        true,
    );
    let changed_path_entities = content_digest_diff
        .iter()
        .filter_map(|(e, diff)| {
            // We only care about changed files, not missing ones
            if matches!(diff, Diff::<ContentDigest>::Different { .. }) {
                Some((*e, stored_xvc_path_store.get(e).cloned().unwrap()))
            } else {
                None
            }
        })
        .collect::<HStore<XvcPath>>();

    if !changed_path_entities.is_empty() {
        Err(anyhow!(format!(
            "Sources have changed, please carry-in or recheck following files before copying:\n{}",
            changed_path_entities
                .values()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        ))
        .into())
    } else {
        Ok(())
    }
}

/// Build a store to match destination path entities with source path entities.
/// It creates the destination entities with [`XvcRoot::new_entity()`] if they are not already found.
/// `stored_xvc_path_store` and `stored_xvc_metadata_store` are loaded with `XvcRoot::load_store`, and
/// `source_xvc_paths` and `source_xvc_metadata` are the results of [`targets_from_disk`].
#[allow(clippy::too_many_arguments)]
pub fn get_copy_source_dest_store(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    stored_xvc_path_store: &XvcStore<XvcPath>,
    stored_xvc_metadata_store: &XvcStore<XvcMetadata>,
    source_xvc_paths: &HStore<XvcPath>,
    source_xvc_metadata: &HStore<XvcMetadata>,
    destination: &str,
    name_only: bool,
    force: bool,
) -> Result<HStore<(XvcEntity, XvcPath)>> {
    // Create targets in the store
    // If destination is a directory, check if exists and create if not.
    // If destination is a file, check if exists and return error if it does and
    // force is not set.
    let source_dest_store = if destination.ends_with('/') {
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

        for (source_xe, source_path) in source_xvc_paths.iter() {
            let dest_path = if name_only {
                dir_path.join_file_name(source_path)?
            } else {
                dir_path.join(source_path)?
            };

            match stored_xvc_path_store.entities_for(&dest_path) {
                Some(v) => {
                    if !force {
                        error!(
                            output_snd,
                            "Destination file {} already exists. Use --force to overwrite.",
                            dest_path
                        );
                        continue;
                    } else {
                        source_dest_store.insert(*source_xe, (v[0], dest_path));
                    }
                }
                None => {
                    source_dest_store.insert(*source_xe, (xvc_root.new_entity(), dest_path));
                }
            }
        }
        source_dest_store
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

        let mut source_dest_store = HStore::<(XvcEntity, XvcPath)>::with_capacity(1);
        let dest_path = XvcPath::new(xvc_root, current_dir, Path::new(destination))?;

        match stored_xvc_path_store.entities_for(&dest_path) {
            Some(dest_xe) => {
                if !force {
                    return Err(anyhow!(
                        "Destination file {} already exists. Use --force to overwrite.",
                        dest_path
                    )
                    .into());
                } else {
                    source_dest_store.insert(*source_xe, (dest_xe[0], dest_path));
                }
            }
            None => {
                source_dest_store.insert(*source_xe, (xvc_root.new_entity(), dest_path));
            }
        }
        source_dest_store
    };

    Ok(source_dest_store)
}

pub(crate) fn recheck_destination(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    destination_entities: &[XvcEntity],
) -> Result<()> {
    watch!(destination_entities);
    let (ignore_writer, ignore_thread) = make_ignore_handler(output_snd, xvc_root)?;
    let (recheck_handler, recheck_thread) =
        make_recheck_handler(output_snd, xvc_root, &ignore_writer)?;
    // We reload to get the latest paths
    // Interleaving might prevent this.
    let stored_xvc_path_store = xvc_root.load_store::<XvcPath>()?;
    let mut recheck_paths = stored_xvc_path_store.subset(destination_entities.iter().copied())?;
    let stored_content_digests = xvc_root.load_store::<ContentDigest>()?;
    let stored_recheck_methods = xvc_root.load_store::<RecheckMethod>()?;

    recheck_paths.drain().for_each(|(xe, xvc_path)| {
        let content_digest = stored_content_digests.get(&xe).unwrap();
        let recheck_method = stored_recheck_methods.get(&xe).unwrap();
        recheck_handler
            .send(Some(RecheckOperation::Recheck {
                xvc_path,
                content_digest: *content_digest,
                recheck_method: *recheck_method,
            }))
            .unwrap();
    });

    // Send None to signal end of operations and break the loops.
    recheck_handler.send(None).unwrap();
    recheck_thread.join().unwrap();
    ignore_writer.send(None).unwrap();
    ignore_thread.join().unwrap();

    Ok(())
}

/// Entry point for `xvc file copy` command.
/// Copies a file (and its records) to a new location in the repository
pub fn cmd_copy(output_snd: &XvcOutputSender, xvc_root: &XvcRoot, opts: CopyCLI) -> Result<()> {
    // Get all files to copy

    let stored_metadata_store = xvc_root.load_store::<XvcMetadata>()?;
    let stored_xvc_path_store = xvc_root.load_store::<XvcPath>()?;
    let (source_xvc_paths, source_metadata) = get_source_path_metadata(
        xvc_root,
        &stored_xvc_path_store,
        &stored_metadata_store,
        &opts.source,
        &opts.destination,
    )?;

    watch!(source_xvc_paths);
    watch!(source_metadata);

    let source_dest_store = get_copy_source_dest_store(
        output_snd,
        xvc_root,
        &stored_xvc_path_store,
        &stored_metadata_store,
        &source_xvc_paths,
        &source_metadata,
        &opts.destination,
        opts.name_only,
        opts.force,
    )?;

    watch!(source_dest_store);

    xvc_root.with_r11store_mut(|store: &mut R11Store<XvcPath, XvcMetadata>| {
        for (source_xe, (dest_xe, dest_path)) in source_dest_store.iter() {
            let source_md = stored_metadata_store.get(source_xe).unwrap();
            store.left.insert(*dest_xe, dest_path.clone());
            // If we recheck, we'll update the metadata with the actual
            // file metadata below.
            store.right.insert(*dest_xe, *source_md);

            // Create destination parent directory records if they don't exist
            for parent in dest_path.parents() {
                let parent_entities = store.left.entities_for(&parent);
                if parent_entities.is_none() || parent_entities.unwrap().is_empty() {
                    let parent_entity = xvc_root.new_entity();
                    store.left.insert(parent_entity, parent.clone());
                    store.right.insert(
                        parent_entity,
                        XvcMetadata {
                            file_type: XvcFileType::Directory,
                            ..Default::default()
                        },
                    );
                }
            }
        }

        // Copy XvcDigest to destination

        xvc_root.with_store_mut(|content_digest_store: &mut XvcStore<ContentDigest>| {
            for (source_xe, (dest_xe, _)) in source_dest_store.iter() {
                let cd = content_digest_store.get(source_xe);
                match cd {
                    Some(cd) => {
                        content_digest_store.insert(*dest_xe, *cd);
                    }
                    None => {
                        debug!(output_snd, "No content digest found for {}", source_xe);
                    }
                }
            }
            Ok(())
        })?;

        xvc_root.with_store_mut(|text_or_binary_store: &mut XvcStore<FileTextOrBinary>| {
            for (source_xe, (dest_xe, _)) in source_dest_store.iter() {
                let tob = text_or_binary_store.get(source_xe);
                match tob {
                    Some(tob) => {
                        text_or_binary_store.insert(*dest_xe, *tob);
                    }
                    None => {
                        debug!(output_snd, "No text or binary found for {}", source_xe);
                    }
                }
            }
            Ok(())
        })?;

        xvc_root.with_store_mut(|recheck_method_store: &mut XvcStore<RecheckMethod>| {
            for (source_xe, (dest_xe, _)) in source_dest_store.iter() {
                if let Some(recheck_method) = opts.recheck_method {
                    recheck_method_store.insert(*dest_xe, recheck_method);
                } else {
                    let source_recheck_method = recheck_method_store.get(source_xe).unwrap();
                    recheck_method_store.insert(*dest_xe, *source_recheck_method);
                }
            }
            Ok(())
        })?;

        Ok(())
    })?;

    // Recheck destination files
    // TODO: We can interleave this operation with the copy operation above
    // to speed things up. This looks premature optimization now.

    if !opts.no_recheck {
        recheck_destination(
            output_snd,
            xvc_root,
            source_dest_store
                .iter()
                .map(|(_, (dest_xe, _))| *dest_xe)
                .collect::<Vec<XvcEntity>>()
                .as_slice(),
        )?;
    }

    Ok(())
}
