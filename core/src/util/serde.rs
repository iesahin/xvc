//! Xvc serialization utilities
use std::io::Cursor;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_yaml;
use toml;

use crate::error::{Error, Result};

/// Utility function to serialize to Msgpack
pub fn to_msgpack<T>(data: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut value = Vec::new();
    match data.serialize(&mut rmp_serde::Serializer::new(&mut value)) {
        Ok(_) => Ok(value),
        Err(source) => Err(Error::MsgPackEncodeError { source }.warn()),
    }
}

/// Utility function to serialize to Json
pub fn to_json<T>(data: &T) -> Result<JsonValue>
where
    T: Serialize,
{
    serde_json::to_value(data).map_err(|e| Error::JsonError { source: e }.warn())
}

/// Utility function to serialize to Toml
pub fn to_toml<T>(data: &T) -> Result<String>
where
    T: Serialize,
{
    toml::ser::to_string_pretty(&data).map_err(|source| Error::TomlSerializationError { source })
}

/// Utility function to serialize to Yaml
pub fn to_yaml<T>(data: &T) -> Result<String>
where
    T: Serialize,
{
    serde_yaml::to_string(data).map_err(|source| Error::YamlError { source })
}

/// Utility function to convert a Msgpack byte slice to anything
pub fn from_msgpack<'de, T>(msgpack_val: &[u8]) -> Result<T>
where
    T: Deserialize<'de>,
{
    let cursor = Cursor::new(msgpack_val);
    let mut deser = rmp_serde::decode::Deserializer::new(cursor);
    let val = Deserialize::deserialize(&mut deser);
    match val {
        Ok(t) => Ok(t),
        Err(source) => Err(Error::MsgPackDecodeError { source }.warn()),
    }
}
