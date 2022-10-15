use std::{fs, io, path::Path};

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::Debug;

use crate::XvcEntity;

use super::{sorted_files, timestamp};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
#[serde(bound = "T: Serialize, for<'lt> T: Deserialize<'lt>")]
pub enum Event<T>
where
    T: Serialize + for<'lt> Deserialize<'lt> + Clone + Debug,
{
    Add { entity: XvcEntity, value: T },
    Remove { entity: XvcEntity },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
#[serde(bound = "T: Serialize, for<'lt> T: Deserialize<'lt>")]
pub struct EventLog<T>(Vec<Event<T>>)
where
    T: Serialize + for<'lt> Deserialize<'lt> + Clone + Debug;

impl<T> EventLog<T>
where
    T: Serialize + for<'lt> Deserialize<'lt> + Clone + Debug,
{
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn to_json(&self) -> Result<JsonValue> {
        serde_json::to_value(&self.0).map_err(|e| Error::JsonError { source: e }.warn())
    }

    pub fn from_json(json_str: &str) -> Result<Self> {
        serde_json::from_str(json_str).map_err(|e| Error::JsonError { source: e }.warn())
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        match fs::read_to_string(path) {
            Ok(contents) => Self::from_json(&contents),
            Err(err) => Err(Error::IoError { source: err }),
        }
    }

    pub fn to_file(&self, path: &Path) -> Result<()> {
        let json_str = self.to_json()?.to_string();
        fs::write(path, json_str).map_err(|source| Error::IoError { source })
    }

    pub fn from_dir(dir: &Path) -> Result<Self> {
        let files = sorted_files(dir)?;
        let merged = files
            .iter()
            .map(|f| {
                Self::from_file(f)
                    .expect(&format!("Error reading event log: {}", f.to_string_lossy()))
            })
            .fold(Self::new(), |mut merged, new| {
                merged.0.extend(new.0.into_iter());
                merged
            });
        Ok(merged)
    }

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

    pub fn to_msgpack(&self) -> Result<Vec<u8>> {
        let mut value = Vec::new();
        match self.serialize(&mut rmp_serde::Serializer::new(&mut value)) {
            Ok(_) => Ok(value),
            Err(source) => Err(Error::MsgPackEncodeError { source }.warn()),
        }
    }

    pub fn from_msgpack(msgpack_val: &[u8]) -> Result<Self> {
        let cursor = io::Cursor::new(msgpack_val);
        let mut deser = rmp_serde::decode::Deserializer::new(cursor);
        let val = Deserialize::deserialize(&mut deser);
        match val {
            Ok(md) => Ok(md),
            Err(source) => Err(Error::MsgPackDecodeError { source }.warn()),
        }
    }

    pub fn add(&mut self, entity: XvcEntity, value: T) {
        self.0.push(Event::Add { entity, value });
    }

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
