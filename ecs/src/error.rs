//! Error codes and messages for Xvc ECS
//!
//! You can use [Error] values for various types of errors.
use log::{debug, error, info, trace, warn};

use std::ffi::OsString;
use std::fmt::Debug;
use std::io;
use std::num::ParseIntError;
use thiserror::Error as ThisError;

use crate::XvcEntity;

#[allow(missing_docs)]
#[derive(ThisError, Debug)]
/// Error type for ECS.
/// We allow missing docs, the names should be self-explanatory.
pub enum Error {
    #[error("Sorry. {0} is not implemented yet")]
    Todo(&'static str),
    #[error("System Time Error: {source}")]
    SystemTimeError {
        #[from]
        source: std::time::SystemTimeError,
    },
    #[error("Cannot restore entity counter from: {path:?}")]
    CannotRestoreEntityCounter { path: OsString },
    #[error("Cannot restore store from: {path:?}")]
    CannotRestoreStoreFromDirectory { path: OsString },

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
    #[error("I/O Error: {source}")]
    IoError {
        #[from]
        source: io::Error,
    },
    #[error("Cannot Parse Integer: {source:?}")]
    CannotParseInteger {
        #[from]
        source: ParseIntError,
    },
    #[error("Missing value for key: {key}")]
    KeyNotFound { key: String },
    #[error("Key is already in the store: {key}")]
    KeyAlreadyFound { store: String, key: String },
    #[error("Multiple keys for value found: {value}")]
    MultipleCorrespondingKeysFound { value: String },
    #[error("Cannot find a related entity: {entity}")]
    NoParentEntityFound { entity: XvcEntity },
    #[error("More than one root entity found in an 1-N relation")]
    MoreThanOneParentFound { entity: usize },
    #[error("Cannot find key in store: {key}")]
    CannotFindKeyInStore { key: String },
    #[error("Internal Store Conversion Error")]
    StoreConversionError,
    #[error("Can initialize {object} only once")]
    CanInitializeOnlyOnce { object: String },
    #[error("Cannot find entity: {entity}")]
    CannotFindEntityInStore { entity: XvcEntity },
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

/// Result type for Xvc ECS functions that may return [Error]
pub type Result<T> = std::result::Result<T, Error>;
