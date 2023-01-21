use std::path::Path;

use crate::common::{filter_targets_from_store, load_targets_from_store, FileTextOrBinary};
use crate::Result;
use anyhow::anyhow;
use clap::Parser;
use crossbeam_channel::Sender;
use xvc_core::{CacheType, ContentDigest, XvcFileType, XvcMetadata, XvcPath, XvcRoot};
use xvc_ecs::{HStore, R11Store, XvcEntity, XvcStore};
use xvc_logging::{debug, error, XvcOutputLine};

/// CLI for `xvc file copy`.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case", author, version)]
pub struct CopyCLI {
    /// How the targets should be rechecked: One of copy, symlink, hardlink, reflink.
    ///
    /// Note: Reflink uses copy if the underlying file system doesn't support it.
    #[arg(long, alias = "as")]
    pub cache_type: Option<CacheType>,

    /// Don't use parallelism
    #[arg(long)]
    pub no_parallel: bool,

    /// Force even if target exists.
    #[arg(long)]
    pub force: bool,

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

pub(crate) fn cmd_copy(
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    opts: CopyCLI,
) -> Result<()> {
    // Get all files to copy

    let source_targets = if opts.source.ends_with("/") {
        let mut source = opts.source.to_string();
        source.push('*');
        vec![source]
    } else {
        vec![opts.source.to_string()]
    };

    let current_dir = xvc_root.config().current_dir()?;
    let all_metadata = xvc_root.load_store::<XvcMetadata>()?;
    let all_xvc_paths = xvc_root.load_store::<XvcPath>()?;
    let all_sources =
        filter_targets_from_store(xvc_root, &all_xvc_paths, current_dir, &Some(source_targets))?;
    let source_metadata = all_metadata.subset(all_sources.keys().copied())?;
    let source_metadata_files = source_metadata.filter(|xe, md| md.is_file());

    if source_metadata_files.len() > 1 && !opts.destination.ends_with("/") {
        return Err(anyhow!("Target must be a directory if multiple sources are given").into());
    }

    let source_xvc_path_files = all_sources.subset(source_metadata_files.keys().copied())?;

    // Create targets in the store
    // If target is a directory, check if exists and create if not.
    // If target is a file, check if exists and return error if it does and
    // force is not set.
    let mut source_dest_store = if opts.destination.ends_with('/') {
        let dir_path = XvcPath::new(
            &xvc_root,
            &xvc_root,
            Path::new(opts.destination.strip_suffix('/').unwrap()),
        )?;

        let current_dir_entity = match all_xvc_paths.entities_for(&dir_path) {
            Some(v) => Some(v[0]),
            None => None,
        };

        let current_dir_metadata = current_dir_entity.and_then(|e| all_metadata.get(&e));

        if let Some(current_dir_metadata) = current_dir_metadata {
            if !current_dir_metadata.is_dir() {
                return Err(anyhow!(
                        "Destination is not recorded as a directory. Please move or delete the destination first."
                    )
                    .into());
            }
        }

        let mut source_dest_store = HStore::new();

        for (source_xe, source_path) in source_xvc_path_files.iter() {
            let dest_path = dir_path.join(source_path).unwrap();

            match all_xvc_paths.entities_for(&dest_path) {
                Some(v) => {
                    if !opts.force {
                        error!(
                            output_snd,
                            "Target file {} already exists. Use --force to overwrite.", dest_path
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
        if source_xvc_path_files.len() > 1 {
            return Err(
                anyhow!("Destination must be a directory if multiple sources are given").into(),
            );
        }

        let source_xe = source_xvc_path_files.keys().next().unwrap();

        let store = xvc_root.load_r11store::<XvcPath, XvcMetadata>()?;
        let mut source_dest_store = HStore::<(XvcEntity, XvcPath)>::with_capacity(1);
        let dest_path = XvcPath::new(&xvc_root, &current_dir, Path::new(&opts.destination))?;

        match store.left.entities_for(&dest_path) {
            Some(dest_xe) => {
                if !opts.force {
                    return Err(anyhow!(
                        "Target file {} already exists. Use --force to overwrite.",
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

    xvc_root.with_r11store_mut(|store: &mut R11Store<XvcPath, XvcMetadata>| {
        for (source_xe, (dest_xe, dest_path)) in source_dest_store.iter() {
            let source_md = source_metadata.get(source_xe).unwrap();
            store.left.insert(*dest_xe, dest_path.clone());
            // If we recheck, we'll update the metadata with the actual
            // file metadata below.
            store.right.insert(*dest_xe, source_md.clone());

            // Create destination parent directory records if they don't exist
            for parent in dest_path.parents() {
                let parent_entities = store.left.entities_for(&parent);
                if parent_entities.is_none() || parent_entities.unwrap().len() == 0 {
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

        xvc_root.with_store_mut(|cache_type_store: &mut XvcStore<CacheType>| {
            for (source_xe, (dest_xe, _)) in source_dest_store.iter() {
                if let Some(cache_type) = opts.cache_type {
                    cache_type_store.insert(*dest_xe, cache_type);
                } else {
                    let source_cache_type = cache_type_store.get(source_xe).unwrap();
                    cache_type_store.insert(*dest_xe, *source_cache_type);
                }
            }
            Ok(())
        })?;

        Ok(())
    })?;

    // Recheck target files

    if !opts.no_recheck {}

    Ok(())
}
