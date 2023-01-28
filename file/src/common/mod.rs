pub mod compare;
pub mod gitignore;

use std::fs::{self};
use std::sync::Arc;
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
use xvc_logging::{info, warn, watch};

use xvc_ecs::{persist, HStore, Storable, XvcStore};
use xvc_logging::XvcOutputLine;
use xvc_walker::{AbsolutePath, Error as XvcWalkerError, Glob, GlobSetBuilder};

use self::gitignore::IgnoreOp;

/// Represents whether a file is a text file or not
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
)]
pub struct FileTextOrBinary(TextOrBinary);
conf!(FileTextOrBinary, "file.track.text_or_binary");
persist!(FileTextOrBinary, "file-text-or-binary");

impl FileTextOrBinary {
    pub fn as_inner(&self) -> TextOrBinary {
        self.0
    }
}

impl Default for FileTextOrBinary {
    fn default() -> Self {
        Self(TextOrBinary::default())
    }
}

/// Calculate the digest of a file in `path` with the given `algorithm` after removing line
/// endings if `text_or_binary` is `TextOrBinary::Text`.
pub fn calc_digest(
    path: &Path,
    algorithm: HashAlgorithm,
    text_or_binary: TextOrBinary,
) -> Result<ContentDigest> {
    Ok(ContentDigest::from_path(path, algorithm, text_or_binary)?)
}

pub fn pipe_path_digest(
    receiver: Receiver<(PathBuf, Metadata)>,
    sender: Sender<(PathBuf, ContentDigest)>,
    algorithm: HashAlgorithm,
    text_or_binary: TextOrBinary,
) -> Result<()> {
    while let Ok((p, _)) = receiver.try_recv() {
        let digest = calc_digest(&p, algorithm, text_or_binary);
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
            let is_match = glob_matcher.is_match(str_path);
            is_match
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
            } else {
                if !t.contains('*') {
                    let abs_target = current_dir.join(Path::new(t));
                    if abs_target.is_dir() {
                        glob_matcher
                            .add(Glob::new(&format!("{t}/**")).expect("Error in glob: {t}/**"));
                    } else {
                        glob_matcher.add(Glob::new(t).expect("Error in glob: {t}"));
                    }
                } else {
                    glob_matcher.add(Glob::new(t).expect("Error in glob: {t}"));
                }
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

pub fn recheck_from_cache(
    output_snd: &Sender<XvcOutputLine>,
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
            ignore_writer.send(Some(IgnoreOperation::IgnoreDir {
                dir: parent.clone(),
            }));
        }
    }
    let cache_path = cache_path.to_absolute_path(xvc_root);
    let path = xvc_path.to_absolute_path(xvc_root);
    watch!(path);
    watch!(recheck_method);
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
    ignore_writer.send(Some(IgnoreOperation::IgnoreFile {
        file: xvc_path.clone(),
    }));
    watch!("Before return");
    Ok(())
}

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
        fs::set_permissions(&cache_dir, dir_perm)?;
    }
    fs::rename(&path, &cache_path).map_err(|source| Error::IoError { source })?;
    let mut file_perm = cache_path.metadata()?.permissions();
    file_perm.set_readonly(true);
    fs::set_permissions(&cache_path, file_perm)?;
    let mut dir_perm = cache_dir.metadata()?.permissions();
    dir_perm.set_readonly(true);
    fs::set_permissions(&cache_dir, dir_perm)?;
    Ok(())
}

pub fn move_xvc_path_to_cache(
    xvc_root: &XvcRoot,
    xvc_path: &XvcPath,
    cache_path: &XvcCachePath,
) -> Result<()> {
    let path = xvc_path.to_absolute_path(xvc_root);
    let cache_path = cache_path.to_absolute_path(xvc_root);
    watch!(path);
    watch!(cache_path);
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
