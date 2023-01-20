use std::path::Path;

use crate::common::{targets_from_store, FileTextOrBinary};
use crate::Result;
use anyhow::anyhow;
use clap::Parser;
use crossbeam_channel::Sender;
use xvc_core::{ContentDigest, XvcMetadata, XvcPath, XvcRoot};
use xvc_ecs::{HStore, R11Store, XvcEntity, XvcStore};
use xvc_logging::{error, XvcOutputLine};

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
    let all_sources = targets_from_store(xvc_root, current_dir, &Some(source_targets))?;
    let all_metadata = xvc_root.load_store::<XvcMetadata>()?;
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
    
    let mut source_dest_store = HStore::<(XvcEntity, XvcPath)>::new();

    xvc_root.with_r11store_mut(|store: &mut R11Store<XvcPath, XvcMetadata>| {
        // If target is a directory, we get new targets by appending the sources to the target
        if opts.destination.ends_with('/') {
            let dir_path = XvcPath::new(
                &xvc_root,
                &xvc_root,
                Path::new(opts.destination.strip_suffix('/').unwrap()),
            )?;

            let current_dir_entity = match store.left.entities_for(&dir_path) {
                Some(v) => Some(v[0]),
                None => None,
            };

            let current_dir_metadata = current_dir_entity.and_then(|e| store.right.get(&e));

            if let Some(current_dir_metadata) = current_dir_metadata {
                if !current_dir_metadata.is_dir() {
                    return Err(anyhow!(
                        "Destination is not recorded as a directory. Please move or delete the destination first."
                    )
                    .into());
                }
            }

            for (source_xe, source_path) in source_xvc_path_files.iter() {
                let dest_path = dir_path.join(source_path).unwrap();


                match store.left.entities_for(&dest_path) {
                    Some(v) => {
                        if !opts.force {
                            error!(output_snd, "Target file {} already exists. Use --force to overwrite.", dest_path);
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
        } else { // Destination doesn't end with '/'
            if source_xvc_path_files.len() > 1 {
                return Err(anyhow!("Destination must be a directory if multiple sources are given").into());
            }

            let source_xe = source_xvc_path_files.keys().next().unwrap();

            let dest_path = XvcPath::new(
                &xvc_root,
                &current_dir,
                Path::new(&opts.destination),
            )?;

            match store.left.entities_for(&dest_path) {
                    Some(dest_xe) => if !opts.force {
                            return Err(anyhow!("Target file {} already exists. Use --force to overwrite.", dest_path).into());
        } else {
                            source_dest_store.insert(*source_xe, (dest_xe[0], dest_path));
                        }
                    None => {
                        source_dest_store.insert(*source_xe, (xvc_root.new_entity(), dest_path));
                    }
                }
        };
          // Copy XvcPath and XvcMetadata to destination
                for (source_xe, (dest_xe, dest_path)) in source_dest_store.iter() {
                    let source_md = source_metadata.get(source_xe).unwrap();
                    store.left.insert(*dest_xe, dest_path.clone());
                    // If we recheck, we'll update the metadata with the actual
                    // file metadata below.
                    store.right.insert(*dest_xe, source_md.clone());

                }

            // Copy XvcDigest to destination

                xvc_root.with_store_mut(|content_digest_store: &mut XvcStore<ContentDigest>| {
                    for (source_xe, (dest_xe, _)) in source_dest_store.iter() {
                            content_digest_store.get(source_xe).and_then(|cd| {
                                content_digest_store.insert(*dest_xe, *cd);
                            });
                    }
                    Ok(())
                })?;


                xvc_root.with_store_mut(|text_or_binary_store: &mut XvcStore<FileTextOrBinary>| {
                    for (source_xe, (dest_xe, _)) in source_dest_store.iter() {
                            let text_or_binary = text_or_binary_store.get(source_xe).and_then(|tob| {
                                text_or_binary_store.insert(*dest_xe, *tob);
                            });
                    }
                    Ok(())
                })?;

                xvc_root.with_store_mut(|cache_type_store: &mut XvcStore<ContentDigest>| {
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

    if !opts.no_recheck {
        
    }

    Ok(())
}
