//! Core file operationscorefil

use std::collections::HashMap;

use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use xvc_logging::{error, uwr, watch, XvcOutputSender};
use xvc_walker::{build_ignore_rules, make_watcher, IgnoreRules, MatchResult, PathEvent};

use crate::error::Error;
use crate::error::Result;
use crate::util::xvcignore::COMMON_IGNORE_PATTERNS;
use crate::{XvcFileType, XVCIGNORE_FILENAME};
use crossbeam_channel::{bounded, RecvError, Select, Sender};
use xvc_walker::check_ignore;

use crate::types::{xvcpath::XvcPath, xvcroot::XvcRoot};
use crate::XvcMetadata;

use super::XvcPathMetadataMap;

/// A cached path metadata provider.
/// It starts from `xvc_root` and caches [XvcMetadata] when the paths are requested.
#[derive(Debug)]
pub struct XvcPathMetadataProvider {
    /// The root directory to start walking from
    xvc_root: XvcRoot,
    path_map: Arc<RwLock<XvcPathMetadataMap>>,
    kill_signal_sender: Arc<Sender<bool>>,
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

        let (watcher, event_receiver) = make_watcher(ignore_rules.clone())?;
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
            // This is not used but to keep the watcher within the thread lifetime
            let watcher = watcher;
            watch!(watcher);

            let handle_fs_event = |fs_event, pmm: Arc<RwLock<XvcPathMetadataMap>>| match fs_event {
                PathEvent::Create { path, metadata } => {
                    let xvc_path = XvcPath::new(&xvc_root, &xvc_root, &path).unwrap();
                    let xvc_md = XvcMetadata::from(metadata);
                    watch!("Creating {} with {}", xvc_path, xvc_md);
                    let mut pmm = pmm.write().unwrap();
                    pmm.insert(xvc_path, xvc_md);
                }
                PathEvent::Update { path, metadata } => {
                    let xvc_path = XvcPath::new(&xvc_root, &xvc_root, &path).unwrap();
                    let xvc_md = XvcMetadata::from(metadata);
                    watch!("Updating {} with {}", xvc_path, xvc_md);
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
                    watch!("Deleting {}", xvc_path);
                    let mut pmm = pmm.write().unwrap();
                    pmm.insert(xvc_path, xvc_md);
                }
            };

            let mut sel = Select::new();
            let fs_event_index = sel.recv(&fs_receiver);
            watch!(fs_event_index);
            let kill_signal_index = sel.recv(&kill_signal_receiver);
            watch!(kill_signal_index);

            loop {
                watch!("pmp background updater ticks");
                if let Ok(selection) = sel.select_timeout(Duration::from_millis(100)) {
                    let index = selection.index();
                    watch!(index);
                    if index == fs_event_index {
                        let fs_event = selection.recv(&fs_receiver);
                        watch!(fs_event);
                        match fs_event {
                            Ok(Some(fs_event)) => {
                                let pmm = path_map.clone();
                                handle_fs_event(fs_event, pmm);
                            }
                            Ok(None) => return Ok(()),
                            Err(e) => {
                                // If the channel is disconnected, return Ok.
                                if e == RecvError {
                                    return Ok(());
                                } else {
                                    error!("Error in fs_receiver: {:?}", e);
                                    return Err(
                                        anyhow::anyhow!("Error in fs_receiver: {:?}", e).into()
                                    );
                                }
                            }
                        }
                        continue;
                    } else if index == kill_signal_index {
                        let _ = selection.recv(&kill_signal_receiver);
                        return Ok(());
                    } else {
                        return Err((anyhow::anyhow!("Unknown selection index: {}", index)).into());
                    }
                }
            }
        })));
        watch!(background_thread);

        Ok(Self {
            xvc_root,
            path_map,
            kill_signal_sender: Arc::new(kill_signal_sender),
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
        let md = pm.get(path).cloned();
        watch!(&md);
        md
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
        let md = path.symlink_metadata();
        watch!(&md);
        self.path_map
            .write()
            .unwrap()
            .insert(xvc_path.clone(), XvcMetadata::from(md));
        Ok(())
    }

    /// Stop updating the paths by killing the background thread
    pub fn stop(&self) -> Result<()> {
        watch!(self.background_thread);
        self.kill_signal_sender
            .clone()
            .send(true)
            .map_err(Error::from)?;
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
                            let md = entry.symlink_metadata();
                            watch!(&md);
                            self.path_map
                                .write()
                                .unwrap()
                                .insert(xvc_path, XvcMetadata::from(md));
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

    /// Return all paths from the disk specified with glob
    pub fn glob_paths(&self, glob: &str) -> Result<XvcPathMetadataMap> {
        watch!(glob);
        self.update_with_glob(glob)?;
        let mut matches = XvcPathMetadataMap::new();
        let pattern = glob::Pattern::new(glob)?;
        for (p, md) in self.path_map.read().unwrap().iter() {
            watch!(p);
            if pattern.matches(p.as_str()) {
                watch!("matched", p);
                if !md.is_missing() {
                    matches.insert(p.clone(), *md);
                }
            }
        }
        Ok(matches)
    }

    /// Return a snapshot of the current path metadata map.
    /// This is a clone of the internal map and is not updated. Intended to be used in testing.
    pub fn current_path_metadata_map_clone(&self) -> Result<XvcPathMetadataMap> {
        Ok(self.path_map.read()?.clone())
    }
}

impl Drop for XvcPathMetadataProvider {
    /// Stop the background thread when quit
    fn drop(&mut self) {
        // Ignore if the channel is closed
        let _ = self.stop();
    }
}
