//! A libnotify based file system notification module that considers ignore rules.
//!
//! This module uses [notify] crate to watch file system events.
//! It filters relevant events, and also ignores the events from ignored paths.
//! It defines [PathEvent] as a simple version of [notify::EventKind].
//! It defines [PathEventHandler] that handles events from [notify::EventHandler].
use crate::{
    check_ignore,
    error::{Error, Result},
    IgnoreRules, MatchResult,
};
use notify::{Event, EventHandler, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::Metadata;
use std::path::PathBuf;

use crossbeam_channel::{bounded, Receiver, Sender};
use log::{debug, warn};

/// An walker-relevant event for changes in a directory.
/// It packs newer [std::fs::Metadata] if there is.
#[derive(Debug)]
pub enum PathEvent {
    /// Emitted when a new `path` is created with `metadata`.
    Create {
        /// The created path
        path: PathBuf,
        /// The new metadata
        metadata: Metadata,
    },
    /// Emitted after a new write to `path`.
    Update {
        /// Updated path
        path: PathBuf,
        /// New metadata
        metadata: Metadata,
    },
    /// Emitted when [PathBuf] is deleted.
    Delete {
        /// Deleted path
        path: PathBuf,
    },
}

/// A struct that handles [notify::Event]s considering also [IgnoreRules]
struct PathEventHandler {
    sender: Sender<PathEvent>,
    ignore_rules: IgnoreRules,
}

impl EventHandler for PathEventHandler {
    fn handle_event(&mut self, event: notify::Result<Event>) {
        if let Ok(event) = event {
            match event.kind {
                notify::EventKind::Create(_) => self.create_event(event.paths[0].clone()),
                notify::EventKind::Modify(mk) => match mk {
                    notify::event::ModifyKind::Any => todo!(),
                    notify::event::ModifyKind::Data(_) => self.write_event(event.paths[0].clone()),
                    notify::event::ModifyKind::Metadata(_) => {
                        self.write_event(event.paths[0].clone())
                    }
                    notify::event::ModifyKind::Name(rk) => match rk {
                        notify::event::RenameMode::Any => {}
                        notify::event::RenameMode::To => self.create_event(event.paths[0].clone()),
                        notify::event::RenameMode::From => {
                            self.remove_event(event.paths[0].clone())
                        }
                        notify::event::RenameMode::Both => {
                            self.rename_event(event.paths[0].clone(), event.paths[1].clone())
                        }
                        notify::event::RenameMode::Other => {}
                    },
                    notify::event::ModifyKind::Other => {}
                },
                notify::EventKind::Remove(_) => self.remove_event(event.paths[0].clone()),
                notify::EventKind::Any => {}
                notify::EventKind::Access(_) => {}
                notify::EventKind::Other => {}
            }
        } else {
            debug!("{:?}", event);
        }
    }
}

impl PathEventHandler {
    fn write_event(&mut self, path: PathBuf) {
        match check_ignore(&self.ignore_rules, &path) {
            MatchResult::Whitelist | MatchResult::NoMatch => {
                self.sender
                    .send(PathEvent::Create {
                        path: path.clone(),
                        metadata: path.metadata().map_err(Error::from).unwrap(),
                    })
                    .unwrap_or_else(|e| warn!("{}", e));
            }
            MatchResult::Ignore => {
                debug!("FS Notification Ignored: {}", path.to_string_lossy());
            }
        }
    }

    fn create_event(&mut self, path: PathBuf) {
        match check_ignore(&self.ignore_rules, &path) {
            MatchResult::Whitelist | MatchResult::NoMatch => {
                self.sender
                    .send(PathEvent::Create {
                        path: path.clone(),
                        metadata: path.metadata().map_err(Error::from).unwrap(),
                    })
                    .unwrap_or_else(|e| {
                        Error::from(e).warn();
                    });
            }
            MatchResult::Ignore => {
                debug!("FS Notification Ignored: {}", path.to_string_lossy());
            }
        }
    }

    fn remove_event(&mut self, path: PathBuf) {
        match check_ignore(&self.ignore_rules, &path) {
            MatchResult::Whitelist | MatchResult::NoMatch => {
                self.sender
                    .send(PathEvent::Delete { path })
                    .unwrap_or_else(|e| warn!("{}", e));
            }
            MatchResult::Ignore => {
                debug!("FS Notification Ignored: {}", path.to_string_lossy());
            }
        }
    }

    fn rename_event(&mut self, from: PathBuf, to: PathBuf) {
        self.remove_event(from);
        self.create_event(to);
    }
}

/// Create a [notify::RecommendedWatcher] and a [crossbeam_channel::Receiver] to receive
/// [PathEvent]s. It creates the channel and [PathEventHandler] with its [Sender], then returns the
/// [Receiver] for consumption.
///
/// Paths ignored by `ignore_rules` do not emit any events.
pub fn make_watcher(
    ignore_rules: IgnoreRules,
) -> Result<(RecommendedWatcher, Receiver<PathEvent>)> {
    let (sender, receiver) = bounded(10000);
    let root = ignore_rules.root.clone();
    let mut watcher = notify::recommended_watcher(PathEventHandler {
        ignore_rules,
        sender,
    })?;
    watcher.watch(&root, RecursiveMode::Recursive)?;
    Ok((watcher, receiver))
}
