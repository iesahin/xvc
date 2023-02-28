//! Core file operations
use cached::proc_macro::cached;
use cached::UnboundCache;
use glob::Pattern as GlobPattern;
use regex::Regex;

use std::collections::HashMap;
use std::fs::{self, Metadata};
use std::io::{self, Read};
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
#[cfg(windows)]
use std::os::windows::fs as windows_fs;

use std::path::{Path, PathBuf};
use xvc_logging::watch;
use xvc_walker::{IgnoreRules, PathMetadata, WalkOptions};

use crate::error::Error as XvcError;
use crate::error::Result as XvcResult;
use crate::CHANNEL_BOUND;
use crossbeam_channel::{bounded, Receiver, Sender};

use crate::types::{xvcpath::XvcPath, xvcroot::XvcRoot};
use crate::XvcMetadata;

use super::xvcignore::walk_parallel;
/// A hashmap to store [XvcMetadata] for [XvcPath]
pub type XvcPathMetadataMap = HashMap<XvcPath, XvcMetadata>;

/// A parallel directory walker.
/// It starts from `start_dir` and sends [PathMetadata] by traversing all child directories.
/// It uses [xvc_walker::walk_parallel] after building an empty [IgnoreRules].
/// NOTE:
///     This function doesn't ignore any files under `start_dir`.
///     It doesn't check any ignore files.
///     It even returns `.git` and `.xvc` directory contents.
pub fn path_metadata_channel(
    sender: Sender<XvcResult<PathMetadata>>,
    start_dir: &Path,
) -> XvcResult<()> {
    let initial_rules = IgnoreRules::empty(start_dir);
    let walk_options = WalkOptions {
        ignore_filename: None,
        include_dirs: true,
    };
    let (w_sender, w_receiver) = bounded(CHANNEL_BOUND);
    let (ignore_sender, _ignore_receiver) = bounded(CHANNEL_BOUND);
    xvc_walker::walk_parallel(
        initial_rules,
        start_dir,
        walk_options,
        w_sender,
        ignore_sender,
    )?;
    for pm in w_receiver {
        sender.send(Ok(pm?))?;
    }

    Ok(())
}

/// Clears errors in `receiver` by ignoring them.
/// TODO: `sender` can be of `Sender<PathMetadata>`
pub fn pipe_filter_path_errors(
    receiver: Receiver<XvcResult<PathMetadata>>,
    sender: Sender<(PathBuf, Metadata)>,
) -> XvcResult<()> {
    while let Ok(Ok(pm)) = receiver.try_recv() {
        let _ = sender.send((pm.path, pm.metadata));
    }
    Ok(())
}

/// A convenience function to return all paths in an Xvc repository.
/// This is meant to be called once in the beginning and the result is carried around instead of hitting the disk every time we look for the changes.
/// It returns the generated [XvcPathMetadataMap] and [IgnoreRules] that is created during
/// traversal.
/// NOTE:
///     This function only returns a snapshot of the repository.
///     If you want to handle events after this initial snapshot, see [xvc_walker::notify::make_watcher].
pub fn all_paths_and_metadata(xvc_root: &XvcRoot) -> (XvcPathMetadataMap, IgnoreRules) {
    walk_parallel(xvc_root, true).unwrap()
}

/// Returns a compiled [glob::Pattern] by prepending it with `pipeline_rundir`.
#[cached(
    type = "UnboundCache<String, glob::Pattern>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{:?}{}", pipeline_rundir, glob) }"#,
    result = true
)]
pub fn compiled_glob(pipeline_rundir: &Path, glob: &str) -> XvcResult<glob::Pattern> {
    GlobPattern::new(&pipeline_rundir.join(glob).to_string_lossy())
        .map_err(|source| XvcError::GlobPatternError { source })
}

/// Returns a compiled [Regex] from `path`.
#[cached(result = true)]
pub fn compiled_regex(pat: String) -> XvcResult<Regex> {
    Regex::new(&pat).map_err(|source| XvcError::RegexError { source })
}

/// Returns a subset of `pmm` ([XvcPathMetadataMap]) that are child paths of `directory`.
pub fn filter_paths_by_directory(
    pmm: &XvcPathMetadataMap,
    directory: &XvcPath,
) -> XvcPathMetadataMap {
    let paths = pmm
        .iter()
        .filter_map(|(p, md)| {
            if p.starts_with(directory) {
                Some((p.clone(), *md))
            } else {
                None
            }
        })
        .collect::<XvcPathMetadataMap>();
    paths
}

/// Returns all _non-ignored_ paths described with `glob` under `root_dir`
#[cached(
    type = "UnboundCache<String, XvcPathMetadataMap>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{}{}", root_dir, glob) }"#,
    result = true
)]
pub fn glob_paths(
    xvc_root: &XvcRoot,
    pmm: &XvcPathMetadataMap,
    root_dir: &XvcPath,
    glob: &str,
) -> XvcResult<XvcPathMetadataMap> {
    glob_paths_nocache(xvc_root, pmm, root_dir, glob)
}

/// Returns a subset of `pmm`.
/// Paths are under `root_dir` and defined by `glob`.
/// `xvc_root` is required to convert [XvcPath] elements to absolute paths.
pub fn glob_paths_nocache(
    xvc_root: &XvcRoot,
    pmm: &XvcPathMetadataMap,
    root_dir: &XvcPath,
    glob: &str,
) -> XvcResult<XvcPathMetadataMap> {
    let abs_root_dir = root_dir.to_absolute_path(xvc_root);
    let g = compiled_glob(&abs_root_dir, glob)?;
    let mut matches = XvcPathMetadataMap::new();
    for (p, md) in pmm.iter() {
        if g.matches_path(&p.to_absolute_path(xvc_root)) {
            matches.insert(p.clone(), *md);
        }
    }
    watch!(&matches);
    Ok(matches)
}

/// Checks whether `glob` includes `path`.
/// Note that, if the `path` is ignored, this fn always returns false
///
/// WARNING: Assumes xvc_ignore doesn't change during the run.
///          It caches the results by pipeline_rundir, glob and path as keys.
#[cached(
    type = "UnboundCache<String, bool>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{:?}##{}##{:?}", pipeline_rundir, glob, path) }"#,
    result = true
)]
pub fn glob_includes(
    xvc_root: &XvcRoot,
    pmm: &XvcPathMetadataMap,
    pipeline_rundir: &XvcPath,
    glob: &str,
    path: &XvcPath,
) -> XvcResult<bool> {
    if pmm.contains_key(path) {
        let abs_pipeline_rundir = pipeline_rundir.to_absolute_path(xvc_root);
        let g = compiled_glob(&abs_pipeline_rundir, glob)?;
        Ok(g.matches_path(&path.to_absolute_path(xvc_root)))
    } else {
        Ok(false)
    }
}

/// Checks whether path is under directory by checking first if it's in the `pmm` keys
#[cached(
    type = "UnboundCache<String, bool>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{:?}##{:?}", directory, path) }"#,
    result = true
)]
pub fn dir_includes(
    pmm: &XvcPathMetadataMap,
    directory: &XvcPath,
    path: &XvcPath,
) -> XvcResult<bool> {
    if pmm.contains_key(path) {
        // Makes a prefix comparison to see whether dir includes the path
        let rel_path = path.relative_pathbuf();
        let rel_dir = directory.relative_pathbuf();
        Ok(rel_path.starts_with(rel_dir))
    } else {
        Ok(false)
    }
}

/// Checks whether a file in `path` is a text file by loading the first 8000
/// bytes (or whole file) and checks if it contains 0 (NUL).
/// The technique is used also by Git.
pub fn is_text_file(path: &Path) -> XvcResult<bool> {
    const BLOCK_SIZE: usize = 8000;
    let mut buffer = [0; BLOCK_SIZE];
    let mut file = fs::File::open(path)?;
    let read_bytes = file.read(&mut buffer[..])?;
    if read_bytes == 0 {
        // empty files are considered text
        Ok(true)
    } else if buffer[0..read_bytes].contains(&0) {
        Ok(false)
    } else {
        Ok(true)
    }
}

#[cfg(unix)]
/// Creates a symlink from target to original
pub fn make_symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, target: Q) -> io::Result<()> {
    unix_fs::symlink(original, target)
}

#[cfg(windows)]
/// Creates a file symlink from target to original
pub fn make_symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, target: Q) -> io::Result<()> {
    windows_fs::symlink_file(original, target)
}
