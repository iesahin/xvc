//! Records a [store][crate::XvcStore] event.
//! It's used for journaling the operations.
//! Journaling is used to keep separate files for operations and replay, merge on start.
use std::{fs, io, path::Path};

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::Debug;

use crate::XvcEntity;

use super::{sorted_files, timestamp};

/// Records add and remove operations of a serializable component `T`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
#[serde(bound = "T: Serialize, for<'lt> T: Deserialize<'lt>")]
pub enum Event<T>
where
    T: Serialize + for<'lt> Deserialize<'lt> + Clone + Debug,
{
    /// Add operation is used to record a component `T` to an [`XvcStore<T>`]
    Add {
        /// The unique key
        entity: XvcEntity,
        /// The serializable component
        value: T,
    },
    /// Remove operation is used when a value is deleted or updated.
    /// In an update, a subsequent [Event::Add] is added.
    Remove {
        /// The key for the component.
        /// Unlike [Event::Add] this doesn't need `T`.
        entity: XvcEntity,
    },
}

/// A series of [Event] values.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
#[serde(bound = "T: Serialize, for<'lt> T: Deserialize<'lt>")]
pub struct EventLog<T>(Vec<Event<T>>)
where
    T: Serialize + for<'lt> Deserialize<'lt> + Clone + Debug;

impl<T> Default for EventLog<T>
where
    T: Serialize + for<'lt> Deserialize<'lt> + Clone + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> EventLog<T>
where
    T: Serialize + for<'lt> Deserialize<'lt> + Clone + Debug,
{
    /// Create an empty event log
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Create an event log already initialized
    pub fn from_events(events: Vec<Event<T>>) -> Self {
        Self(events)
    }

    /// Convert the log to a Json array containing ([XvcEntity], `T`) values.
    /// This is used to record [XvcStore] to files.
    pub fn to_json(&self) -> Result<JsonValue> {
        serde_json::to_value(&self.0).map_err(|e| Error::JsonError { source: e }.warn())
    }

    /// Load the logs from a Json file.
    pub fn from_json(json_str: &str) -> Result<Self> {
        serde_json::from_str(json_str).map_err(|e| Error::JsonError { source: e }.warn())
    }

    /// Loads the event log from a Json file.
    pub fn from_file(path: &Path) -> Result<Self> {
        match fs::read_to_string(path) {
            Ok(contents) => Self::from_json(&contents),
            Err(err) => Err(Error::IoError { source: err }),
        }
    }

    /// Records the event log to a Json file.
    pub fn to_file(&self, path: &Path) -> Result<()> {
        let json_str = self.to_json()?.to_string();
        fs::write(path, json_str).map_err(|source| Error::IoError { source })
    }

    /// Loads a set of Json files from a directory after sorting them.
    /// File contents are merged in a single event log.
    pub fn from_dir(dir: &Path) -> Result<Self> {
        let files = sorted_files(dir)?;
        let merged = files
            .iter()
            .map(|f| {
                Self::from_file(f)
                    .unwrap_or_else(|_| panic!("Error reading event log: {}", f.to_string_lossy()))
            })
            .fold(Self::new(), |mut merged, new| {
                merged.0.extend(new.0);
                merged
            });
        Ok(merged)
    }

    /// Records an event log to a single file in the given directory.
    /// The file name uses [timestamp] to make this file as the last file in a sorted list.
    pub fn to_dir(&self, dir: &Path) -> Result<()> {
        if !self.is_empty() {
            if !dir.exists() {
                fs::create_dir_all(dir)?;
            }
            let path = dir.join(format!("{}.json", timestamp()));
            let json_str = self.to_json()?.to_string();
            fs::write(path, json_str).map_err(|source| Error::IoError { source })
        } else {
            Ok(())
        }
    }

    /// Converts the event log to a [MessagePack](https://msgpack.org/index.html) list
    pub fn to_msgpack(&self) -> Result<Vec<u8>> {
        let mut value = Vec::new();
        match self.serialize(&mut rmp_serde::Serializer::new(&mut value)) {
            Ok(_) => Ok(value),
            Err(source) => Err(Error::MsgPackEncodeError { source }.warn()),
        }
    }

    /// Converts the event log from a [MessagePack](https://msgpack.org/index.html) list
    pub fn from_msgpack(msgpack_val: &[u8]) -> Result<Self> {
        let cursor = io::Cursor::new(msgpack_val);
        let mut deser = rmp_serde::decode::Deserializer::new(cursor);
        let val = Deserialize::deserialize(&mut deser);
        match val {
            Ok(md) => Ok(md),
            Err(source) => Err(Error::MsgPackDecodeError { source }.warn()),
        }
    }

    /// Appends an [Event::Add] event to the log
    pub fn add(&mut self, entity: XvcEntity, value: T) {
        self.0.push(Event::Add { entity, value });
    }

    /// Appends an [Event::Remove] event to the log.
    ///
    /// Note that this doesn't remove anything from the log.
    /// Stores that use event log are supposed to remove the element from their maps after this.
    pub fn remove(&mut self, entity: XvcEntity) {
        self.0.push(Event::Remove { entity });
    }
}

use std::ops::Deref;

impl<T> Deref for EventLog<T>
where
    T: Serialize + for<'lt> Deserialize<'lt> + Clone + Debug,
{
    type Target = Vec<Event<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
