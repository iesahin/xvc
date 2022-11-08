//! Error types for the config crate.
use log::{debug, error, info, trace, warn};

use std::ffi::OsString;
use std::fmt::Debug;
use std::io;
use thiserror::Error as ThisError;

/// Error enum that uses [ThisError].
/// The messages should be self-explanatory, so skipping the docs.
#[allow(missing_docs)]
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Sorry. {0} is not implemented yet")]
    Todo(&'static str),
    #[error("Regex Error: {source}")]
    RegexError {
        #[from]
        source: regex::Error,
    },
    #[error("TOML Serialization Error: {source}")]
    TomlSerializationError {
        #[from]
        source: toml::ser::Error,
    },

    #[error("TOML Deserialization Error: {source}")]
    TomlDeserializationError {
        #[from]
        source: toml::de::Error,
    },

    #[error("Yaml Error: {source}")]
    YamlError {
        #[from]
        source: serde_yaml::Error,
    },
    #[error("Encountered NULL value in YAML map")]
    YamlNullValueForKey { key: String },
    #[error("I/O Error: {source}")]
    IoError {
        #[from]
        source: io::Error,
    },
    #[error("Cannot convert enum type from string: {cause_key}")]
    EnumTypeConversionError { cause_key: String },
    #[error("Config source for level {config_source:?} not found at {path:?}")]
    ConfigurationForSourceNotFound {
        config_source: String,
        path: OsString,
    },

    #[error("Config value type mismatch: {key} ")]
    MismatchedValueType { key: String },
    #[error("Config key not found: {key}")]
    ConfigKeyNotFound { key: String },
    #[error("Cannot Determine System Configuration Path")]
    CannotDetermineSystemConfigurationPath,

    #[error("Cannot Determine User Configuration Path")]
    CannotDetermineUserConfigurationPath,
    #[error("Enum Parsing Error")]
    StrumError {
        #[from]
        source: strum::ParseError,
    },
}

impl Error {
    /// print [DEBUG] message for [Error]
    pub fn debug(self) -> Self {
        debug!("{}", self);
        self
    }
    /// print [TRACE] message for [Error]
    pub fn trace(self) -> Self {
        trace!("{}", self);
        self
    }
    /// print [WARN] message for [Error]
    pub fn warn(self) -> Self {
        warn!("{}", self);
        self
    }
    /// print [ERROR] message for [Error]
    pub fn error(self) -> Self {
        error!("{}", self);
        self
    }
    /// print [INFO] message for [Error]
    pub fn info(self) -> Self {
        info!("{}", self);
        self
    }
    /// print [PANIC] message for [Error] and exit!
    pub fn panic(self) -> Self {
        panic!("{}", self);
    }
}

/// Result type for xvc-config crate
pub type Result<T> = std::result::Result<T, Error>;
