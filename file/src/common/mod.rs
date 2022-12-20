pub mod compare;

use std::fs::{self, Permissions};
use std::{
    fs::Metadata,
    path::{Path, PathBuf},
};

use crate::error::{Error, Result};
use crossbeam_channel::{Receiver, Sender};
use derive_more::{AsRef, Deref, Display, From, FromStr};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use xvc_config::{conf, FromConfigKey};
use xvc_core::types::xvcpath::XvcCachePath;
use xvc_core::util::file::make_symlink;
use xvc_core::{
    all_paths_and_metadata, apply_diff, CacheType, CollectionDigest, ContentDigest, DiffStore,
    MetadataDigest, TextOrBinary, XvcFileType, XvcMetadata, XvcPath, XvcPathMetadataMap, XvcRoot,
};
use xvc_core::{util::file::is_text_file, HashAlgorithm, XvcDigest};
use xvc_logging::{error, info, warn, watch};

use xvc_ecs::{persist, HStore, Storable, XvcEntity, XvcStore};
use xvc_logging::XvcOutputLine;
use xvc_walker::{
    check_ignore, AbsolutePath, Error as XvcWalkerError, Glob, GlobSetBuilder, IgnoreRules,
    MatchResult, PathMetadata,
};

#[derive(Debug, Clone)]
pub struct PathMatch {
    xvc_path: Option<XvcPath>,
    actual_path: Option<PathMetadata>,
    xvc_entity: Option<XvcEntity>,
    actual_digest: Option<XvcDigest>,
}

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
    Default,
    Copy,
)]
pub struct FileTextOrBinary(TextOrBinary);
conf!(FileTextOrBinary, "file.add.text_or_binary");
persist!(FileTextOrBinary, "file-text-or-binary");

impl FileTextOrBinary {
    pub fn as_inner(&self) -> TextOrBinary {
        self.0
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

pub fn pathbuf_to_xvc_target(
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    xvc_ignore: &IgnoreRules,
    current_dir: &AbsolutePath,
    targets: &Vec<PathBuf>,
) -> Vec<XvcPath> {
    targets
        .into_iter()
        .filter_map(|t| {
            watch!(t);
            watch!(t.is_file());
            watch!(t.is_dir());
            watch!(t.metadata());
            if t.is_file() || t.is_dir() {
                Some(t)
            } else {
                warn!(
                    output_snd,
                    "Unsupported Target Type: {}",
                    t.to_string_lossy()
                );
                None
            }
        })
        .filter(|t| {
            let ignore_result = check_ignore(&xvc_ignore, t);

            match ignore_result {
                MatchResult::Ignore => {
                    warn!(output_snd, "Ignored: {}", t.to_string_lossy());
                    false
                }
                MatchResult::Whitelist => {
                    info!(output_snd, "Whitelisted: {}", t.to_string_lossy());
                    true
                }
                MatchResult::NoMatch => true,
            }
        })
        .map(|t| XvcPath::new(xvc_root, current_dir, &t))
        .filter_map(|res_xp| match res_xp {
            Ok(xp) => Some(xp),
            Err(e) => {
                error!("{}", e);
                None
            }
        })
        .collect()
}

pub fn split_file_directory_targets(
    output_snd: &Sender<XvcOutputLine>,
    xpmm: &XvcPathMetadataMap,
    xvc_targets: &[XvcPath],
) -> (XvcPathMetadataMap, XvcPathMetadataMap) {
    let mut dir_targets = XvcPathMetadataMap::new();
    let mut file_targets = XvcPathMetadataMap::new();

    for xvc_target in xvc_targets {
        if let Some(xmd) = xpmm.get(&xvc_target) {
            match xmd.file_type {
                XvcFileType::Missing => {
                    error!(output_snd, "Target not found: {}", xvc_target);
                }
                XvcFileType::File => {
                    file_targets.insert(xvc_target.clone(), xmd.clone());
                }
                XvcFileType::Directory => {
                    dir_targets.insert(xvc_target.clone(), xmd.clone());
                }
                XvcFileType::Symlink => {
                    error!(output_snd, "Symlinks are not supported: {}", xvc_target)
                }
                XvcFileType::Hardlink => {
                    error!(output_snd, "Hardlinks are not supported: {xvc_target}");
                }
                XvcFileType::Reflink => {
                    error!(output_snd, "Reflinks are not supported: {xvc_target}")
                }
            }
        } else {
            warn!(output_snd, "Ignored or not found: {xvc_target}");
        }
    }

    (file_targets, dir_targets)
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

pub fn targets_from_store(
    xvc_root: &XvcRoot,
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

        return targets_from_store(xvc_root, xvc_root.absolute_path(), &Some(targets));
    }

    let xvc_path_store: XvcStore<XvcPath> = xvc_root.load_store()?;
    if let Some(targets) = targets {
        let xvc_metadata_store: XvcStore<XvcMetadata> = xvc_root.load_store()?;
        let mut glob_matcher = GlobSetBuilder::new();
        targets.iter().for_each(|t| {
            glob_matcher.add(Glob::new(t).expect("Error in glob: {t}"));
        });
        let glob_matcher = glob_matcher.build().map_err(XvcWalkerError::from)?;

        let mut paths =
            xvc_path_store.filter(|_, p| glob_matcher.is_match(&p.as_relative_path().as_str()));
        let metadata = xvc_metadata_store.subset(paths.keys().copied())?;
        // for any directories in the targets, we add all child paths
        let dir_md = metadata.filter(|_, md| md.file_type == XvcFileType::Directory);
        let dir_paths = paths.subset(dir_md.keys().copied())?;
        for (_, dir) in dir_paths.iter() {
            let child_paths = xvc_path_store.filter(|_, p| p.starts_with(dir));
            child_paths.into_iter().for_each(|(k, v)| {
                paths.insert(k, v);
            });
        }
        Ok(paths)
    } else {
        Ok(xvc_path_store.into())
    }
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

    if let Some(targets) = targets {
        let mut glob_matcher = GlobSetBuilder::new();
        targets.iter().for_each(|t| {
            glob_matcher.add(Glob::new(t).expect("Error in glob: {t}"));
        });
        let glob_matcher = glob_matcher.build().map_err(XvcWalkerError::from)?;

        Ok(all_paths
            .into_iter()
            .filter(|(p, _)| glob_matcher.is_match(p.as_str()))
            .collect())
    } else {
        Ok(all_paths)
    }
}

pub fn only_file_targets(
    xvc_path_store: &XvcStore<XvcPath>,
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
        .map(|(xe, xp)| {
            let p = xp.to_absolute_path(xvc_root);
            let xmd = XvcMetadata::from(p.metadata());
            (xp.clone(), xmd)
        })
        .collect()
}

pub fn expand_directory_targets(
    output_snd: &Sender<XvcOutputLine>,
    xpmm: &XvcPathMetadataMap,
    dir_targets: &XvcPathMetadataMap,
) -> (XvcPathMetadataMap, XvcPathMetadataMap) {
    let mut all_dir_targets = XvcPathMetadataMap::new();
    let mut all_file_targets = XvcPathMetadataMap::new();

    for (dir_target, dir_md) in dir_targets {
        for (xvc_path, xvc_md) in xpmm {
            if xvc_path.starts_with(&dir_target) && *xvc_path != *dir_target {
                match xvc_md.file_type {
                    XvcFileType::Directory => {
                        all_dir_targets.insert(xvc_path.clone(), xvc_md.clone());
                    }
                    XvcFileType::File => {
                        all_file_targets.insert(xvc_path.clone(), xvc_md.clone());
                    }
                    _ => {
                        error!(output_snd, "Unsupported Target: {xvc_path}");
                    }
                }
            }
        }
        all_dir_targets.insert(dir_target.clone(), dir_md.clone());
    }
    (all_dir_targets, all_file_targets)
}

pub fn expand_xvc_dir_file_targets(
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    current_dir: &AbsolutePath,
    targets: Vec<PathBuf>,
) -> (XvcPathMetadataMap, XvcPathMetadataMap) {
    let (xpmm, xvc_ignore) = all_paths_and_metadata(xvc_root);
    let xvc_targets =
        pathbuf_to_xvc_target(output_snd, xvc_root, &xvc_ignore, current_dir, &targets);

    let (mut file_targets, given_dir_targets) =
        split_file_directory_targets(output_snd, &xpmm, &xvc_targets);
    // Add all paths under directory targets
    let (mut dir_targets, implicit_file_targets) =
        expand_directory_targets(output_snd, &xpmm, &given_dir_targets);

    dir_targets.extend(given_dir_targets.into_iter());
    file_targets.extend(implicit_file_targets.into_iter());

    (dir_targets, file_targets)
}

pub const PARALLEL_THRESHOLD: usize = 47;

/// Use parallel processing if the number of targets is greater than the threshold
/// or directories are included in the targets.
pub fn decide_no_parallel(from_opts: bool, targets: &[PathBuf]) -> bool {
    from_opts || (targets.iter().all(|t| t.is_file()) && targets.len() < PARALLEL_THRESHOLD)
}

pub fn recheck_from_cache(
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    xvc_path: &XvcPath,
    cache_path: &XvcCachePath,
    cache_type: CacheType,
) -> Result<()> {
    if let Some(parent) = xvc_path.parents().get(0) {
        let parent_dir = parent.to_absolute_path(xvc_root);
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }
    }
    let cache_path = cache_path.to_absolute_path(xvc_root);
    let path = xvc_path.to_absolute_path(xvc_root);
    watch!(path);
    watch!(cache_type);
    match cache_type {
        CacheType::Copy => {
            fs::copy(&cache_path, &path)?;
            info!(output_snd, "[COPY] {} -> {}", cache_path, path);
            let mut perm = path.metadata()?.permissions();
            perm.set_readonly(false);
            fs::set_permissions(&path, perm)?;
        }
        CacheType::Hardlink => {
            fs::hard_link(&cache_path, &path)?;
            info!(output_snd, "[HARDLINK] {} -> {}", cache_path, path);
        }
        CacheType::Symlink => {
            make_symlink(&cache_path, &path)?;
            info!(output_snd, "[SYMLINK] {} -> {}", cache_path, path);
        }
        CacheType::Reflink => {
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
    Ok(())
}

pub fn move_to_cache(
    xvc_root: &XvcRoot,
    xvc_path: &XvcPath,
    cache_path: &XvcCachePath,
) -> Result<()> {
    let path = xvc_path.to_absolute_path(xvc_root);
    let cache_path = cache_path.to_absolute_path(xvc_root);
    let cache_dir = cache_path.parent().ok_or(Error::InternalError {
        message: "Cache path has no parent.".to_string(),
    })?;
    watch!(cache_dir);
    fs::create_dir_all(cache_dir)?;
    watch!(path);
    watch!(cache_path);
    fs::rename(&path, &cache_path).map_err(|source| Error::IoError { source })?;
    let mut perm = cache_path.metadata()?.permissions();
    perm.set_readonly(true);
    fs::set_permissions(&cache_path, perm)?;
    Ok(())
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
    let new_store = apply_diff(&records, diffs, add_new, remove_missing)?;
    xvc_root.save_store(&new_store)?;
    Ok(())
}
