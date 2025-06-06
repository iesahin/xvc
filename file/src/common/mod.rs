//! Common operations for xvc file
pub mod compare;
pub mod gitignore;

use std::collections::{HashMap, HashSet};
use std::fs::{self};

use std::{
    fs::Metadata,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::common::gitignore::IgnoreOperation;
use crate::error::{Error, Result};
use crossbeam_channel::{Receiver, Sender};
use derive_more::{AsRef, Deref, Display, From, FromStr};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use xvc_core::EventLog;
use xvc_core::{
    all_paths_and_metadata, apply_diff, conf, error, get_absolute_git_command,
    get_git_tracked_files, info, persist,
    types::xvcpath::XvcCachePath,
    util::{file::make_symlink, xvcignore::COMMON_IGNORE_PATTERNS},
    uwr, warn, AbsolutePath, ContentDigest, DiffStore, FromConfigKey, Glob, HStore, HashAlgorithm,
    PathSync, RecheckMethod, Storable, TextOrBinary, XvcFileType, XvcMetadata, XvcOutputSender,
    XvcPath, XvcPathMetadataMap, XvcRoot, XvcStore,
};
use xvc_core::{path_metadata_map_from_file_targets, XvcWalkerError};

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
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    current_dir: &AbsolutePath,
    targets: &Option<Vec<String>>,
) -> Result<HStore<XvcPath>> {
    let xvc_path_store: XvcStore<XvcPath> = xvc_root.load_store()?;
    filter_targets_from_store(output_snd, xvc_root, &xvc_path_store, current_dir, targets)
}

/// Filters the paths in the store by given globs.
///
/// If `targets` is None, returns all paths in the store.
///
/// If `current_dir` is not the root, all targets are prefixed with it.
pub fn filter_targets_from_store(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    xvc_path_store: &XvcStore<XvcPath>,
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
            output_snd,
            xvc_root,
            xvc_path_store,
            xvc_root.absolute_path(),
            &Some(targets),
        );
    }

    if let Some(targets) = targets {
        let paths =
            filter_paths_by_globs(output_snd, xvc_root, xvc_path_store, targets.as_slice())?;
        Ok(paths)
    } else {
        Ok(xvc_path_store.into())
    }
}

/// Filter a set of paths by a set of globs. The globs are compiled into a
/// GlobSet and paths are checked against the set.
///
/// If a target ends with /, it's considered a directory and all its children are also selected.
pub fn filter_paths_by_globs(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    paths: &XvcStore<XvcPath>,
    globs: &[String],
) -> Result<HStore<XvcPath>> {
    if globs.is_empty() {
        return Ok(paths.into());
    }

    // Ensure directories end with /
    let globs = globs
        .iter()
        .map(|g| {
            if !g.ends_with('/') && !g.contains('*') {
                let slashed = format!("{g}/");
                // We don't track directories. Instead we look for files that start with the directory.
                if paths.any(|_, p| p.as_str().starts_with(&slashed)) {
                    slashed
                } else {
                    g.clone()
                }
            } else {
                g.clone()
            }
        })
        .collect::<Vec<String>>();

    let mut glob_matcher = build_glob_matcher(output_snd, xvc_root, &globs)?;
    let paths = paths
        .iter()
        .filter_map(|(e, p)| {
            if glob_matcher.is_match(p.as_str()) {
                Some((*e, p.clone()))
            } else {
                None
            }
        })
        .collect();

    Ok(paths)
}

/// Builds a glob matcher based on the provided directory and glob patterns.
///
/// # Arguments
///
/// * `output_snd`: A sender for output messages.
/// * `dir`: The directory to which the glob patterns will be applied.
/// * `globs`: A slice of glob patterns as strings.
///
/// # Returns
///
/// * `Result<Glob>`: A `Result` that contains the `Glob` matcher if successful, or an error if not.
///
/// # Errors
///
/// This function will return an error if any of the glob patterns are invalid.
///
pub fn build_glob_matcher(
    output_snd: &XvcOutputSender,
    dir: &Path,
    globs: &[String],
) -> Result<Glob> {
    let mut glob_matcher = Glob::default();
    globs.iter().for_each(|t| {
        if t.ends_with('/') {
            if !glob_matcher.add(&format!("{t}**")) {
                error!(output_snd, "Error in glob: {t}");
            }
        } else if !t.contains('*') {
            let abs_target = dir.join(Path::new(t));
            if abs_target.is_dir() {
                if !glob_matcher.add(&format!("{t}/**")) {
                    error!(output_snd, "Error in glob: {t}")
                }
            } else if !glob_matcher.add(t) {
                error!(output_snd, "Error in glob: {t}")
            }
        } else if !glob_matcher.add(t) {
            error!(output_snd, "Error in glob: {t}")
        }
    });
    Ok(glob_matcher)
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
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    current_dir: &AbsolutePath,
    targets: &Option<Vec<String>>,
    filter_git_paths: bool,
) -> Result<XvcPathMetadataMap> {
    // If we are not in the root, we add current dir to all targets and recur.
    if *current_dir != *xvc_root.absolute_path() {
        let cwd = current_dir
            .strip_prefix(xvc_root.absolute_path())?
            .to_str()
            .unwrap();

        let cwd = if cwd.ends_with('/') {
            cwd.to_owned()
        } else {
            format!("{cwd}/")
        };

        let targets = match targets {
            Some(targets) => targets.iter().map(|t| format!("{cwd}{t}")).collect(),
            None => vec![cwd.to_string()],
        };
        return targets_from_disk(
            output_snd,
            xvc_root,
            xvc_root.absolute_path(),
            &Some(targets),
            filter_git_paths,
        );
    }

    let has_globs_or_dirs = targets
        .as_ref()
        .map(|targets| {
            targets.iter().any(|t| {
                t.contains('*') || t.ends_with('/') || t.contains('/') || PathBuf::from(t).is_dir()
            })
        })
        // None means all paths
        .unwrap_or(true);
    // If there are no globs/directories in the targets, no need to retrieve all the paths
    // here.

    let all_paths = if has_globs_or_dirs {
        all_paths_and_metadata(xvc_root).0
    } else {
        // FIXME: Move this to a function
        let (pmm, _) = path_metadata_map_from_file_targets(
            output_snd,
            COMMON_IGNORE_PATTERNS,
            xvc_root,
            // This should be ok as we checked empty condition on has_globs_or_dirs
            targets.clone().unwrap(),
            &xvc_core::walker::WalkOptions::xvcignore(),
        )?;
        let mut xpmm = HashMap::new();

        pmm.into_iter().for_each(|pm| {
            let md: XvcMetadata = XvcMetadata::from(pm.metadata);
            let rxp = XvcPath::new(xvc_root, xvc_root.absolute_path(), &pm.path);
            match rxp {
                Ok(xvc_path) => {
                    xpmm.insert(xvc_path, md);
                }
                Err(e) => {
                    e.warn();
                }
            }
        });
        xpmm
    };

    // Return false when the path is a git path

    let git_files: HashSet<String> = if filter_git_paths {
        let git_command_str = xvc_root.config().get_str("git.command")?.option;
        let git_command = get_absolute_git_command(&git_command_str)?;
        get_git_tracked_files(
            &git_command,
            xvc_root
                .absolute_path()
                .to_str()
                .expect("xvc_root must have a path"),
        )?
        .into_iter()
        .collect()
    } else {
        HashSet::new()
    };

    let mut git_path_filter: Box<dyn FnMut(&XvcPath) -> bool> = if filter_git_paths {
        Box::new(|p: &XvcPath| {
            let path_str = p.as_str();
            let path_str = path_str
                .strip_prefix(
                    xvc_root
                        .absolute_path()
                        .to_str()
                        .expect("xvc_root must have a path"),
                )
                .unwrap_or(path_str);
            !git_files.contains(path_str)
        })
    } else {
        Box::new(|_p: &XvcPath| true)
    };

    if let Some(targets) = targets {
        // FIXME: Is this a bug? When targets is empty, we can return all files.
        // Targets should be None to return all paths but what about we pass Some([])?

        if targets.is_empty() {
            return Ok(XvcPathMetadataMap::new());
        }

        let mut glob_matcher = build_glob_matcher(output_snd, xvc_root, targets)?;
        Ok(all_paths
            .into_iter()
            .filter(|(p, _)| git_path_filter(p))
            .filter(|(p, _)| glob_matcher.is_match(p.as_str()))
            .collect())
    } else {
        Ok(all_paths
            .into_iter()
            .filter(|(p, _)| git_path_filter(p))
            .collect())
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
    if let Some(parent) = xvc_path.parents().first() {
        let parent_dir = parent.to_absolute_path(xvc_root);
        if !parent_dir.exists() {
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

    match recheck_method {
        RecheckMethod::Copy => {
            copy_file(output_snd, cache_path, path)?;
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
            reflink(output_snd, cache_path, path)?;
        }
    }
    uwr!(
        ignore_writer.send(Some(IgnoreOperation::IgnoreFile {
            file: xvc_path.clone(),
        })),
        output_snd
    );
    Ok(())
}

#[cfg(feature = "reflink")]
fn reflink(
    output_snd: &XvcOutputSender,
    cache_path: AbsolutePath,
    path: AbsolutePath,
) -> Result<()> {
    match reflink::reflink(&cache_path, &path) {
        Ok(_) => {
            info!(output_snd, "[REFLINK] {} -> {}", cache_path, path);
            Ok(())
        }
        Err(e) => {
            warn!(
                output_snd,
                "File system doesn't support reflink. {e}. Copying instead."
            );
            copy_file(output_snd, cache_path, path)
        }
    }
}

fn copy_file(
    output_snd: &XvcOutputSender,
    cache_path: AbsolutePath,
    path: AbsolutePath,
) -> Result<()> {
    fs::copy(&cache_path, &path)?;
    set_writable(&path)?;
    info!(output_snd, "[COPY] {} -> {}", cache_path, path);
    Ok(())
}

#[cfg(not(unix))]
pub fn set_writable(path: &Path) -> Result<()> {
    let mut perm = path.metadata()?.permissions();
    perm.set_readonly(false);
    fs::set_permissions(path, perm)?;
    Ok(())
}

#[cfg(not(unix))]
pub fn set_readonly(path: &Path) -> Result<()> {
    let mut perm = path.metadata()?.permissions();
    perm.set_readonly(true);
    fs::set_permissions(path, perm)?;
    Ok(())
}

/// Set a path to user writable on unix systems.
#[cfg(unix)]
pub fn set_writable(path: &Path) -> Result<()> {
    let mut permissions = path.metadata()?.permissions();
    let mode = permissions.mode();
    let new_mode = mode | 0o200;
    permissions.set_mode(new_mode);
    fs::set_permissions(path, permissions)?;
    Ok(())
}

/// Set a path to readonly on unix systems.
#[cfg(unix)]
pub fn set_readonly(path: &Path) -> Result<()> {
    let mut permissions = path.metadata()?.permissions();
    let mode = permissions.mode();
    let new_mode = mode & !0o200;
    permissions.set_mode(new_mode);
    fs::set_permissions(path, permissions)?;
    Ok(())
}

#[cfg(not(feature = "reflink"))]
fn reflink(
    output_snd: &XvcOutputSender,
    cache_path: AbsolutePath,
    path: AbsolutePath,
) -> Result<()> {
    warn!(
        output_snd,
        "Xvc isn't compiled with reflink support. Copying the file."
    );
    copy_file(output_snd, cache_path, path)
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
                xvc_core::Event::Add { entity: _, value } => {
                    let xcp = uwr!(XvcCachePath::new(xp, value), output_snd
                 );

                    Some(xcp)
                }
                xvc_core::Event::Remove { entity } => {
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
///
/// The [PathSync] struct is used to lock the paths during the operation, so that no two threads
/// try to accessl to the same path at the same time.
// TODO: Remove this when we set unix permissions in platform dependent fashion
#[allow(clippy::permissions_set_readonly_false)]
pub fn move_to_cache(
    path: &AbsolutePath,
    cache_path: &AbsolutePath,
    path_sync: &PathSync,
) -> Result<()> {
    let cache_dir = cache_path.parent().ok_or(Error::InternalError {
        message: "Cache path has no parent.".to_string(),
    })?;
    // We don't lock the path_sync here because we don't want to block other threads.
    path_sync
        .with_sync_abs_path(path, |path| {
            path_sync.with_sync_abs_path(cache_path, |cache_path| {
                if !cache_dir.exists() {
                    fs::create_dir_all(cache_dir)?;
                }
                // Set to writable
                let mut dir_perm = cache_dir.metadata()?.permissions();
                dir_perm.set_readonly(false);
                fs::set_permissions(cache_dir, dir_perm)?;

                fs::rename(path, cache_path)
                    .map_err(|source| XvcWalkerError::IoError { source })?;
                let mut file_perm = cache_path.metadata()?.permissions();
                file_perm.set_readonly(true);
                fs::set_permissions(cache_path, file_perm.clone())?;
                let mut dir_perm = cache_dir.metadata()?.permissions();
                dir_perm.set_readonly(true);
                fs::set_permissions(cache_dir, dir_perm)?;
                Ok(())
            })
        })
        .map_err(|e| e.into())
}

/// Move an xvc_path to the cache path.
/// Uses [move_to_cache]
pub fn move_xvc_path_to_cache(
    xvc_root: &XvcRoot,
    xvc_path: &XvcPath,
    cache_path: &XvcCachePath,
    path_sync: &PathSync,
) -> Result<()> {
    let path = xvc_path.to_absolute_path(xvc_root);
    let cache_path = cache_path.to_absolute_path(xvc_root);
    move_to_cache(&path, &cache_path, path_sync)
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
