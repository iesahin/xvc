use crate::{
    check_ignore,
    error::{Error, Result},
    IgnoreRules, MatchResult,
};
use notify::{Event, EventHandler, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::Metadata;
use std::path::PathBuf;
use xvc_logging::watch;

use crossbeam_channel::{bounded, Receiver, Sender};
use log::{debug, warn};

#[derive(Debug)]
pub enum PathEvent {
    Create { path: PathBuf, metadata: Metadata },
    Update { path: PathBuf, metadata: Metadata },
    Delete { path: PathBuf },
}

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

pub fn make_watcher(
    ignore_rules: IgnoreRules,
) -> Result<(RecommendedWatcher, Receiver<PathEvent>)> {
    let (sender, receiver) = bounded(10000);
    let root = ignore_rules.root.clone();
    watch!(ignore_rules);
    let mut watcher = notify::recommended_watcher(PathEventHandler {
        ignore_rules,
        sender,
    })?;
    watcher.watch(&root, RecursiveMode::Recursive)?;
    watch!(watcher);
    watch!(receiver);
    Ok((watcher, receiver))
}
