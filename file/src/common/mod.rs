//! Common operations for xvc file
pub mod compare;
pub mod gitignore;

use std::fs::{self};

use std::{
    fs::Metadata,
    path::{Path, PathBuf},
};

use crate::common::gitignore::IgnoreOperation;
use crate::error::{Error, Result};
use crossbeam_channel::{Receiver, Sender};
use derive_more::{AsRef, Deref, Display, From, FromStr};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use xvc_config::{conf, FromConfigKey};
use xvc_core::types::xvcpath::XvcCachePath;
use xvc_core::util::file::make_symlink;
use xvc_core::HashAlgorithm;
use xvc_core::{
    all_paths_and_metadata, apply_diff, ContentDigest, DiffStore, RecheckMethod, TextOrBinary,
    XvcFileType, XvcMetadata, XvcPath, XvcPathMetadataMap, XvcRoot,
};
use xvc_ecs::ecs::event::EventLog;
use xvc_logging::{error, info, uwr, warn, watch, XvcOutputSender};

use xvc_ecs::{persist, HStore, Storable, XvcStore};

use xvc_walker::{AbsolutePath, Error as XvcWalkerError, Glob, GlobSetBuilder};

use self::gitignore::IgnoreOp;

/// Represents whether a file is a text file or not. We wrap [TextOrBinary] to specify [persist!] and [conf!].
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Hash,
    Display,
    FromStr,
    From,
    AsRef,
    Deref,
    Copy,
    Default,
)]
pub struct FileTextOrBinary(TextOrBinary);
conf!(FileTextOrBinary, "file.track.text_or_binary");
persist!(FileTextOrBinary, "file-text-or-binary");

impl FileTextOrBinary {
    /// Returns the inner TextOrBinary
    pub fn as_inner(&self) -> TextOrBinary {
        self.0
    }
}

/// Receives path and metadata and sends content digests of the sent paths.
pub fn pipe_path_digest(
    receiver: Receiver<(PathBuf, Metadata)>,
    sender: Sender<(PathBuf, ContentDigest)>,
    algorithm: HashAlgorithm,
    text_or_binary: TextOrBinary,
) -> Result<()> {
    while let Ok((p, _)) = receiver.try_recv() {
        let digest = ContentDigest::new(&p, algorithm, text_or_binary);
        match digest {
            Ok(digest) => {
                let _ = sender.send((p, digest));
            }
            Err(err) => {
                log::warn!("{:?}", err);
            }
        }
    }
    Ok(())
}

/// This is to convert targets given in the CLI to XvcPaths. It doesn't walk the
/// file system. It's to be used in `xvc file carry-in` or `xvc file recheck`,
/// where we already track the files in the store.
///
/// Just loads the stores, compiles targets as globs and checks
/// which paths in the store matches. If the matches contain directories, all their
/// children are also selected.
///
/// If `targets` is `None`, all paths in the store are returned.
pub fn load_targets_from_store(
    xvc_root: &XvcRoot,
    current_dir: &AbsolutePath,
    targets: &Option<Vec<String>>,
) -> Result<HStore<XvcPath>> {
    let store: XvcStore<XvcPath> = xvc_root.load_store()?;
    filter_targets_from_store(xvc_root, &store, current_dir, targets)
}

/// Filters the paths in the store by given globs.
///
/// If `targets` is None, returns all paths in the store.
///
/// If `current_dir` is not the root, all targets are prefixed with it.
pub fn filter_targets_from_store(
    xvc_root: &XvcRoot,
    store: &XvcStore<XvcPath>,
    current_dir: &AbsolutePath,
    targets: &Option<Vec<String>>,
) -> Result<HStore<XvcPath>> {
    // If we are not in the root, we add current dir to all targets and recur.
    if *current_dir != *xvc_root.absolute_path() {
        let cwd = current_dir
            .strip_prefix(xvc_root.absolute_path())?
            .to_str()
            .unwrap();
        let targets = match targets {
            Some(targets) => targets.iter().map(|t| format!("{cwd}{t}")).collect(),
            None => vec![cwd.to_string()],
        };

        return filter_targets_from_store(
            xvc_root,
            store,
            xvc_root.absolute_path(),
            &Some(targets),
        );
    }

    watch!(targets);

    let hstore = HStore::<XvcPath>::from(store);
    if let Some(targets) = targets {
        let paths = filter_paths_by_globs(&hstore, targets.as_slice())?;
        watch!(paths);
        Ok(paths)
    } else {
        Ok(hstore)
    }
}

/// Filter a set of paths by a set of globs. The globs are compiled into a
/// GlobSet and paths are checked against the set.
pub fn filter_paths_by_globs(paths: &HStore<XvcPath>, globs: &[String]) -> Result<HStore<XvcPath>> {
    let mut glob_matcher = GlobSetBuilder::new();
    globs.iter().for_each(|t| {
        watch!(t);
        if t.ends_with('/') {
            glob_matcher.add(Glob::new(&format!("{t}**")).expect("Error in glob: {t}**"));
        } else {
            glob_matcher.add(Glob::new(&format!("{t}/**")).expect("Error in glob: {t}/**"));
        }
        glob_matcher.add(Glob::new(t).expect("Error in glob: {t}"));
    });
    let glob_matcher = glob_matcher.build().map_err(XvcWalkerError::from)?;

    let paths = paths
        .filter(|_, p| {
            let str_path = &p.as_relative_path().as_str();

            glob_matcher.is_match(str_path)
        })
        .cloned();

    watch!(paths);
    Ok(paths)
}

/// Converts targets to a map of XvcPaths and their metadata. It walks the file
/// system with [`all_paths_and_metadata`]. This is aimed towards `xvc file
/// track`, `xvc file hash` and similar commands where we work with the existing
/// files.
///
/// This walks all the repository. It doesn't try to optimize the walk by
/// selecting targets first, because,
/// - This is a premature optimization.
/// - We need to consider ignore files and this requires to start a walk from
///   the root.
///
/// If some day we need to optimize first walking the ignores, then walking the
/// directories in the targets, I'd be glad that this is used in very large
/// repositories.

pub fn targets_from_disk(
    xvc_root: &XvcRoot,
    current_dir: &AbsolutePath,
    targets: &Option<Vec<String>>,
) -> Result<XvcPathMetadataMap> {
    watch!(current_dir);
    watch!(xvc_root.absolute_path());
    // If we are not in the root, we add current dir to all targets and recur.
    if *current_dir != *xvc_root.absolute_path() {
        let cwd = current_dir
            .strip_prefix(xvc_root.absolute_path())?
            .to_str()
            .unwrap();
        let targets = match targets {
            Some(targets) => targets.iter().map(|t| format!("{cwd}{t}")).collect(),
            None => vec![cwd.to_string()],
        };

        return targets_from_disk(xvc_root, xvc_root.absolute_path(), &Some(targets));
    }
    let (all_paths, _) = all_paths_and_metadata(xvc_root);

    watch!(all_paths);

    if let Some(targets) = targets {
        let mut glob_matcher = GlobSetBuilder::new();
        targets.iter().for_each(|t| {
            if t.ends_with('/') {
                glob_matcher.add(Glob::new(&format!("{t}**")).expect("Error in glob: {t}**"));
            } else if !t.contains('*') {
                let abs_target = current_dir.join(Path::new(t));
                if abs_target.is_dir() {
                    glob_matcher.add(Glob::new(&format!("{t}/**")).expect("Error in glob: {t}/**"));
                } else {
                    glob_matcher.add(Glob::new(t).expect("Error in glob: {t}"));
                }
            } else {
                glob_matcher.add(Glob::new(t).expect("Error in glob: {t}"));
            }
        });
        let glob_matcher = glob_matcher.build().map_err(XvcWalkerError::from)?;
        watch!(glob_matcher);
        Ok(all_paths
            .into_iter()
            .filter(|(p, _)| glob_matcher.is_match(p.as_str()))
            .collect())
    } else {
        Ok(all_paths)
    }
}

/// Selects only the files in `targets` by matching them with the metadata in `xvc_metadata_store`.
pub fn only_file_targets(
    xvc_metadata_store: &XvcStore<XvcMetadata>,
    targets: &HStore<XvcPath>,
) -> Result<HStore<XvcPath>> {
    let target_metadata = xvc_metadata_store.subset(targets.keys().copied())?;

    assert! {
        target_metadata.len() == targets.len(),
        "The number of targets and the number of target metadata should be the same."
    }

    let target_files = targets.subset(
        target_metadata
            .filter(|_, xmd| xmd.file_type == XvcFileType::File)
            .keys()
            .copied(),
    )?;

    Ok(target_files)
}

/// Return the metadata of targets. This is used in various functions to get the
/// changed files in repository. When you want to get all files and their
/// metadata, it may be better to use [all_paths_and_metadata].
pub fn xvc_path_metadata_map_from_disk(
    xvc_root: &XvcRoot,
    targets: &HStore<XvcPath>,
) -> XvcPathMetadataMap {
    targets
        .par_iter()
        .map(|(_, xp)| {
            let p = xp.to_absolute_path(xvc_root);
            let xmd = XvcMetadata::from(p.metadata());
            (xp.clone(), xmd)
        })
        .collect()
}

/// Copies / links `cache_path` to `xvc_path` with `recheck_method`.
/// WARNING: If `xvc_path` is already present, it will be deleted first.
/// It also sends an ignore operation to `ignore_writer`.
pub fn recheck_from_cache(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    xvc_path: &XvcPath,
    cache_path: &XvcCachePath,
    recheck_method: RecheckMethod,
    ignore_writer: &Sender<IgnoreOp>,
) -> Result<()> {
    if let Some(parent) = xvc_path.parents().get(0) {
        let parent_dir = parent.to_absolute_path(xvc_root);
        watch!(parent_dir);
        if !parent_dir.exists() {
            watch!(&parent_dir);
            fs::create_dir_all(parent_dir)?;
            uwr!(
                ignore_writer.send(Some(IgnoreOperation::IgnoreDir {
                    dir: parent.clone(),
                })),
                output_snd
            );
        }
    }
    let cache_path = cache_path.to_absolute_path(xvc_root);
    let path = xvc_path.to_absolute_path(xvc_root);
    // If the file already exists, we delete it.
    if path.exists() {
        fs::remove_file(&path)?;
    }

    watch!(path);
    watch!(recheck_method);

    // TODO: Remove this when we set unix permissions in platform dependent fashion
    #[allow(clippy::permissions_set_readonly_false)]
    match recheck_method {
        RecheckMethod::Copy => {
            watch!("Before copy");
            fs::copy(&cache_path, &path)?;
            info!(output_snd, "[COPY] {} -> {}", cache_path, path);
            let mut perm = path.metadata()?.permissions();
            watch!(&perm);
            perm.set_readonly(false);
            watch!(&perm);
            fs::set_permissions(&path, perm)?;
        }
        RecheckMethod::Hardlink => {
            fs::hard_link(&cache_path, &path)?;
            info!(output_snd, "[HARDLINK] {} -> {}", cache_path, path);
        }
        RecheckMethod::Symlink => {
            make_symlink(&cache_path, &path)?;
            info!(output_snd, "[SYMLINK] {} -> {}", cache_path, path);
        }
        RecheckMethod::Reflink => {
            match reflink::reflink_or_copy(&cache_path, &path) {
                Ok(None) => {
                    info!(output_snd, "[REFLINK] {} -> {}", cache_path, path);
                }
                Ok(Some(_)) => {
                    warn!(
                        output_snd,
                        "File system doesn't support reflink. Copying instead."
                    );
                    info!(output_snd, "[COPY] {} -> {}", cache_path, path);
                    let mut perm = path.metadata()?.permissions();
                    perm.set_readonly(false);
                    fs::set_permissions(&path, perm)?;
                }
                Err(source) => {
                    Error::IoError { source }.error();
                }
            };
        }
    }
    uwr!(
        ignore_writer.send(Some(IgnoreOperation::IgnoreFile {
            file: xvc_path.clone(),
        })),
        output_snd
    );
    watch!("Return recheck_from_cache");
    Ok(())
}

/// All cache paths for all xvc paths.
/// There are extracted from the event logs.
pub fn cache_paths_for_xvc_paths(
    output_snd: &XvcOutputSender,
    all_paths: &XvcStore<XvcPath>,
    all_content_digests: &XvcStore<ContentDigest>,
) -> Result<HStore<Vec<XvcCachePath>>> {
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
                xvc_ecs::ecs::event::Event::Add { entity: _, value } => {
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

    Ok(all_cache_paths)
}

/// Moves the `path` to `cache_path`.
///
/// It creates the cache directory and sets the cache file read only.
///
/// It overwrites the cache file if it already exists.
// TODO: Remove this when we set unix permissions in platform dependent fashion
#[allow(clippy::permissions_set_readonly_false)]
pub fn move_to_cache(path: &AbsolutePath, cache_path: &AbsolutePath) -> Result<()> {
    let cache_dir = cache_path.parent().ok_or(Error::InternalError {
        message: "Cache path has no parent.".to_string(),
    })?;
    watch!(cache_dir);
    if !cache_dir.exists() {
        fs::create_dir_all(cache_dir)?;
    } else {
        // Set to writable
        let mut dir_perm = cache_dir.metadata()?.permissions();
        dir_perm.set_readonly(false);
        fs::set_permissions(cache_dir, dir_perm)?;
    }
    fs::rename(path, cache_path).map_err(|source| Error::IoError { source })?;
    let mut file_perm = cache_path.metadata()?.permissions();
    file_perm.set_readonly(true);
    fs::set_permissions(cache_path, file_perm)?;
    let mut dir_perm = cache_dir.metadata()?.permissions();
    dir_perm.set_readonly(true);
    fs::set_permissions(cache_dir, dir_perm)?;
    Ok(())
}

/// Move an xvc_path to the cache path.
/// Uses [move_to_cache]
pub fn move_xvc_path_to_cache(
    xvc_root: &XvcRoot,
    xvc_path: &XvcPath,
    cache_path: &XvcCachePath,
) -> Result<()> {
    let path = xvc_path.to_absolute_path(xvc_root);
    let cache_path = cache_path.to_absolute_path(xvc_root);
    move_to_cache(&path, &cache_path)
}

/// Record store records checking their Diff.
/// It loads the store and creates a new store by [apply_diff], then saves it.
/// TODO: This may be optimized for in place update when stores get larger.
pub fn update_store_records<T>(
    xvc_root: &XvcRoot,
    diffs: &DiffStore<T>,
    add_new: bool,
    remove_missing: bool,
) -> Result<()>
where
    T: Storable,
{
    let records = xvc_root.load_store::<T>()?;
    watch!(records.len());
    let new_store = apply_diff(&records, diffs, add_new, remove_missing)?;
    watch!(new_store.len());
    xvc_root.save_store(&new_store)?;
    Ok(())
}
