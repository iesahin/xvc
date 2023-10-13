//! Error messages for Xvc core crate
use log::{debug, error, info, trace, warn};
use xvc_config::error::Error as XvcConfigError;
use xvc_ecs::error::Error as XvcEcsError;
use xvc_walker::error::Error as XvcWalkerError;

use std::ffi::OsString;
use std::fmt::Debug;
use std::io;
use std::path::PathBuf;
use std::sync::PoisonError;
use thiserror::Error as ThisError;

#[allow(missing_docs)]
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Sorry. {0} is not implemented yet")]
    Todo(&'static str),

    #[error("General Xvc Core Error: {msg}")]
    GeneralError { msg: String },

    #[error("General Xvc Core Error: {source}")]
    AnyhowError {
        #[from]
        source: anyhow::Error,
    },

    #[error("ECS Error: {source}")]
    EcsError {
        #[from]
        source: XvcEcsError,
    },
    #[error("Walker Error: {source}")]
    WalkerError {
        #[from]
        source: XvcWalkerError,
    },

    #[error("Config Error: {source}")]
    ConfigError {
        #[from]
        source: XvcConfigError,
    },

    #[error("File System Walk Error: {error}")]
    FSWalkerError { error: String },
    #[error("Cannot find XVC Root: {path}")]
    CannotFindXvcRoot { path: PathBuf },
    #[error("Cannot nest XVC repositories: {path}")]
    CannotNestXvcRepositories { path: PathBuf },
    #[error("Regex Error: {source}")]
    RegexError {
        #[from]
        source: regex::Error,
    },
    #[error("System Time Error: {source}")]
    SystemTimeError {
        #[from]
        source: std::time::SystemTimeError,
    },
    #[error("[E1002] MsgPack Serialization Error: {source}")]
    MsgPackDecodeError {
        #[from]
        source: rmp_serde::decode::Error,
    },
    #[error("[E1003] MsgPack Serialization Error: {source}")]
    MsgPackEncodeError {
        #[from]
        source: rmp_serde::encode::Error,
    },
    #[error("[E1004] Json Serialization Error: {source}")]
    JsonError {
        #[from]
        source: serde_json::Error,
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
    #[error("Unicode/UTF-8 Error: {cause:?}")]
    UnicodeError { cause: OsString },
    #[error("Glob Error: {source}")]
    GlobError {
        #[from]
        source: glob::GlobError,
    },
    //
    #[error("Glob Pattern Error: {source}")]
    GlobPatternError {
        #[from]
        source: glob::PatternError,
    },
    #[error("Path strip prefix error: {source}")]
    StringPrefixError {
        #[from]
        source: std::path::StripPrefixError,
    },

    #[error("Request Error: {source}")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },

    #[error("Crossbeam Send Error for Type: {t:?} {cause:?}")]
    CrossbeamSendError { t: String, cause: String },
    #[error("Relative Path Conversion Error: {source}")]
    RelativePathError {
        #[from]
        source: relative_path::FromPathError,
    },
    #[error("Glob error: {source}")]
    GlobSetError {
        #[from]
        source: xvc_walker::globset::Error,
    },

    #[error("Cannot find parent path")]
    CannotFindParentPath { path: PathBuf },

    #[error("Poison Error: {cause:?}")]
    PoisonError { cause: String },
}

impl<T> From<crossbeam_channel::SendError<T>> for Error
where
    T: Debug,
{
    fn from(e: crossbeam_channel::SendError<T>) -> Self {
        Error::CrossbeamSendError {
            t: format!("{:#?}", e.0),
            cause: e.to_string(),
        }
    }
}

impl From<Box<dyn std::any::Any + Send>> for Error {
    fn from(e: Box<dyn std::any::Any + Send>) -> Self {
        Error::GeneralError {
            msg: format!("{:?}", e),
        }
    }
}

impl<T: Debug> From<PoisonError<T>> for Error {
    fn from(e: PoisonError<T>) -> Self {
        Error::PoisonError {
            cause: e.to_string(),
        }
    }
}

impl<T: Debug> From<&PoisonError<T>> for Error {
    fn from(e: &PoisonError<T>) -> Self {
        Error::PoisonError {
            cause: e.to_string(),
        }
    }
}

impl Error {
    /// log error to debug
    pub fn debug(self) -> Self {
        debug!("{}", self);
        self
    }
    /// log error to trace
    pub fn trace(self) -> Self {
        trace!("{}", self);
        self
    }
    /// log error to warning
    pub fn warn(self) -> Self {
        warn!("{}", self);
        self
    }
    /// log error to error
    pub fn error(self) -> Self {
        error!("{}", self);
        self
    }
    /// log error to info
    pub fn info(self) -> Self {
        info!("{}", self);
        self
    }
    /// panic with this error
    pub fn panic(self) -> Self {
        panic!("{}", self);
    }
}

/// The result type that may return a xvc::core::Error
pub type Result<T> = std::result::Result<T, Error>;
