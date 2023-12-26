//! Core file operations
use cached::proc_macro::cached;
use cached::UnboundCache;
use glob::Pattern as GlobPattern;
use regex::Regex;

use std::collections::HashMap;
use std::fs::{self, Metadata};
use std::io::{self, Read};
use std::ops::Index;
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
#[cfg(windows)]
use std::os::windows::fs as windows_fs;

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock, RwLockWriteGuard};
use std::thread::{self, JoinHandle};
use xvc_logging::{error, uwo, uwr, watch, XvcOutputLine, XvcOutputSender};
use xvc_walker::{
    build_ignore_rules, make_watcher, IgnoreRules, MatchResult, PathEvent, PathMetadata,
    RecommendedWatcher, WalkOptions,
};

use crate::error::Error;
use crate::error::Result;
use crate::util::xvcignore::COMMON_IGNORE_PATTERNS;
use crate::{XvcFileType, CHANNEL_BOUND, XVCIGNORE_FILENAME};
use crossbeam_channel::{bounded, select, Receiver, RecvError, Sender};
use xvc_walker::check_ignore;

use crate::types::{xvcpath::XvcPath, xvcroot::XvcRoot};
use crate::XvcMetadata;

use super::xvcignore::walk_parallel;
/// A hashmap to store [XvcMetadata] for [XvcPath]
pub type XvcPathMetadataMap = HashMap<XvcPath, XvcMetadata>;

#[derive(Debug)]
pub struct XvcPathMetadataProvider {
    /// The root directory to start walking from
    xvc_root: XvcRoot,
    path_map: Arc<RwLock<XvcPathMetadataMap>>,
    kill_switch_sender: Sender<bool>,
    background_thread: Arc<Mutex<JoinHandle<Result<()>>>>,
    output_sender: XvcOutputSender,
    ignore_rules: IgnoreRules,
}

impl XvcPathMetadataProvider {
    /// Create a new PathMetadataProvider
    pub fn new(output_sender: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<Self> {
        let initial_rules = IgnoreRules::try_from_patterns(xvc_root, COMMON_IGNORE_PATTERNS)?;
        let ignore_rules = build_ignore_rules(initial_rules, xvc_root, XVCIGNORE_FILENAME)?;
        watch!(ignore_rules);
        let path_map = Arc::new(RwLock::new(HashMap::new()));

        let (_watcher, event_receiver) = make_watcher(ignore_rules.clone())?;
        let (kill_signal_sender, kill_signal_receiver) = bounded(1);

        let xvc_root = xvc_root.clone();
        let xvc_root_clone = xvc_root.clone();
        let path_map_clone = path_map.clone();
        let output_sender = output_sender.clone();
        let event_receiver_clone = event_receiver.clone();

        let background_thread = Arc::new(Mutex::new(thread::spawn(move || {
            let path_map = path_map_clone;
            let fs_receiver = event_receiver_clone;
            let xvc_root = xvc_root_clone;

            let handle_fs_event = |fs_event, pmm: Arc<RwLock<XvcPathMetadataMap>>| match fs_event {
                PathEvent::Create { path, metadata } => {
                    let xvc_path = XvcPath::new(&xvc_root, &xvc_root, &path).unwrap();
                    let xvc_md = XvcMetadata::from(metadata);
                    let mut pmm = pmm.write().unwrap();
                    pmm.insert(xvc_path, xvc_md);
                }
                PathEvent::Update { path, metadata } => {
                    let xvc_path = XvcPath::new(&xvc_root, &xvc_root, &path).unwrap();
                    let xvc_md = XvcMetadata::from(metadata);
                    let mut pmm = pmm.write().unwrap();
                    pmm.insert(xvc_path, xvc_md);
                }
                PathEvent::Delete { path } => {
                    let xvc_path = XvcPath::new(&xvc_root, &xvc_root, &path).unwrap();
                    let xvc_md = XvcMetadata {
                        file_type: XvcFileType::Missing,
                        size: None,
                        modified: None,
                    };
                    let mut pmm = pmm.write().unwrap();
                    pmm.insert(xvc_path, xvc_md);
                }
            };

            loop {
                select! {
                    recv(fs_receiver) -> fs_event => match fs_event {
                        Ok(Some(fs_event)) => {
                            let pmm = path_map.clone();
                            handle_fs_event(fs_event, pmm);
                        }
                        Ok(None) => {
                            return Ok(())
                        }
                        Err(e) => {
                            // If the channel is disconnected, return Ok.
                            if e == RecvError {
                                return Ok(())
                            } else {
                                error!("Error in fs_receiver: {:?}", e);
                                return Err(anyhow::anyhow!("Error in fs_receiver: {:?}", e).into())
                            }
                        }
                    },

                    recv(kill_signal_receiver) -> kill_signal => {
                        if let Ok(true) = kill_signal {
                            return Ok(());
                        }
                    },
                }
            }
        })));
        watch!(background_thread);

        Ok(Self {
            xvc_root,
            path_map,
            kill_switch_sender: kill_signal_sender,
            background_thread,
            output_sender,
            ignore_rules,
        })
    }

    /// Returns the [XvcMetadata] for a given [XvcPath].
    pub fn get(&self, path: &XvcPath) -> Option<XvcMetadata> {
        watch!(path);
        if !self.path_map.read().unwrap().contains_key(path) {
            uwr!(self.update_metadata(path), self.output_sender);
        }
        let pm = self.path_map.clone();
        let pm = uwr!(pm.read(), self.output_sender);
        pm.get(path).cloned()
    }

    /// Returns true if the path is present in the repository.
    pub fn path_present(&self, path: &XvcPath) -> bool {
        if !self.path_map.read().unwrap().contains_key(path) {
            uwr!(self.update_metadata(path), self.output_sender);
        }
        let pm = self.path_map.clone();
        let pm = uwr!(pm.read(), self.output_sender);
        if let Some(md) = pm.get(path) {
            !md.is_missing()
        } else {
            false
        }
    }

    fn update_metadata(&self, xvc_path: &XvcPath) -> Result<()> {
        watch!(xvc_path);
        let path = xvc_path.to_absolute_path(&self.xvc_root);
        watch!(path);
        let md = path.symlink_metadata()?;
        watch!(&md);
        self.path_map
            .write()
            .unwrap()
            .insert(xvc_path.clone(), md.into());
        Ok(())
    }

    /// Stop updating the paths by killing the background thread
    pub fn stop(&self) -> Result<()> {
        watch!(self.background_thread);
        self.kill_switch_sender.send(true).map_err(Error::from)?;
        watch!(self.background_thread);
        Ok(())
    }

    fn update_with_glob(&self, glob: &str) -> Result<()> {
        watch!(glob);
        for entry in glob::glob(glob)? {
            match entry {
                Ok(entry) => {
                    if matches!(
                        check_ignore(&self.ignore_rules, &entry),
                        MatchResult::Ignore
                    ) {
                        continue;
                    } else {
                        let xvc_path = XvcPath::new(&self.xvc_root, &self.xvc_root, &entry)?;
                        watch!(xvc_path);
                        if self.path_map.read().unwrap().contains_key(&xvc_path) {
                            continue;
                        } else {
                            let md = entry.symlink_metadata()?;
                            watch!(&md);
                            self.path_map.write().unwrap().insert(xvc_path, md.into());
                        }
                    }
                }
                Err(e) => {
                    error!(self.output_sender, "Error while globbing: {:?}", e);
                }
            }
        }
        Ok(())
    }

    pub fn glob_paths(&self, glob: &str) -> Result<XvcPathMetadataMap> {
        self.update_with_glob(glob)?;
        let mut matches = XvcPathMetadataMap::new();
        for (p, md) in self.path_map.read().unwrap().iter() {
            watch!(p);
            if glob::Pattern::new(glob)?.matches_path(&p.to_absolute_path(&self.xvc_root)) {
                watch!("matched: {p}");
                matches.insert(p.clone(), *md);
            }
        }
        Ok(matches)
    }
}

/// A parallel directory walker.
/// It starts from `start_dir` and sends [PathMetadata] by traversing all child directories.
/// It uses [xvc_walker::walk_parallel] after building an empty [IgnoreRules].
/// NOTE:
///     This function doesn't ignore any files under `start_dir`.
///     It doesn't check any ignore files.
///     It even returns `.git` and `.xvc` directory contents.
pub fn path_metadata_channel(sender: Sender<Result<PathMetadata>>, start_dir: &Path) -> Result<()> {
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
    receiver: Receiver<Result<PathMetadata>>,
    sender: Sender<(PathBuf, Metadata)>,
) -> Result<()> {
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
pub fn compiled_glob(pipeline_rundir: &Path, glob: &str) -> Result<glob::Pattern> {
    GlobPattern::new(&pipeline_rundir.join(glob).to_string_lossy())
        .map_err(|source| Error::GlobPatternError { source })
}

/// Returns a compiled [Regex] from `path`.
#[cached(result = true)]
pub fn compiled_regex(pat: String) -> Result<Regex> {
    Regex::new(&pat).map_err(|source| Error::RegexError { source })
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
    pmp: &XvcPathMetadataProvider,
    root_dir: &XvcPath,
    glob: &str,
) -> Result<XvcPathMetadataMap> {
    let full_glob = format!("{}{}", root_dir, glob);
    watch!(full_glob);
    pmp.glob_paths(&full_glob)
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
    pmp: &XvcPathMetadataProvider,
    pipeline_rundir: &XvcPath,
    glob: &str,
    path: &XvcPath,
) -> Result<bool> {
    if pmp.path_present(path) {
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
pub fn dir_includes(pmm: &XvcPathMetadataMap, directory: &XvcPath, path: &XvcPath) -> Result<bool> {
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
pub fn is_text_file(path: &Path) -> Result<bool> {
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
