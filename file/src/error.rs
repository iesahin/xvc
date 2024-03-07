//! [Error] codes and messages for xvc-file crate
use log::{debug, error, info, trace, warn};

use std::fmt::Debug;
use std::io;
use std::path::PathBuf;
use std::time::SystemTimeError;
use thiserror::Error as ThisError;

/// Error messages for xvc-file
#[allow(missing_docs)]
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Sorry. {0} is not implemented yet")]
    Todo(&'static str),
    #[error("{source}")]
    AnyhowError {
        #[from]
        source: anyhow::Error,
    },
    #[error("Cannot find {xvc_path} in cache: {cache_path}")]
    CannotFindFileInCache {
        xvc_path: String,
        cache_path: String,
    },
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },
    #[error("Internal Error: {message}")]
    InternalError { message: String },
    #[error("Walker Error: {source}")]
    WalkerError {
        #[from]
        source: xvc_walker::Error,
    },
    #[error("Ecs Error: {source}")]
    EcsError {
        #[from]
        source: xvc_ecs::error::Error,
    },
    #[error("Storage Error: {source}")]
    StorageError {
        #[from]
        source: xvc_storage::error::Error,
    },
    #[error("Target is ignored, please unignore in .xvcignore: {path}")]
    TargetIgnored { path: String },

    #[error("[E2004] Requires xvc repository.")]
    RequiresXvcRepository,

    #[error("Xvc Core Error: {source}")]
    XvcCoreError {
        #[from]
        source: xvc_core::error::Error,
    },
    #[error("Xvc Config Error: {source}")]
    XvcConfigError {
        #[from]
        source: xvc_config::error::Error,
    },
    #[error("I/O Error: {source}")]
    IoError {
        #[from]
        source: io::Error,
    },
    #[error("Enum Parsing Error")]
    StrumError {
        #[from]
        source: strum::ParseError,
    },
    #[error("Crossbeam Send Error for Type: {t:?} {cause:?}")]
    CrossbeamSendError { t: String, cause: String },

    #[error("Strip Prefix Error")]
    StripPrefixError {
        #[from]
        source: std::path::StripPrefixError,
    },
    #[error("Relative Path Strip Prefix Error: {:?}", e)]
    RelativeStripPrefixError { e: relative_path::StripPrefixError },

    #[error("System time error")]
    SystemTimeError {
        #[from]
        source: SystemTimeError,
    },
    #[error("Xvc does not support content digest for symlink: {path}")]
    ContentDigestNotSupported { path: PathBuf },

    #[error("Poisoned Locks: {t} {cause}")]
    LockPoisonError { t: String, cause: String },

    #[error("Multiple files found to share")]
    MultipleFilesToShare,

    #[error("No files found to share")]
    NoFilesToShare,

    #[error("Error parsing the duration")]
    DurationError {
        #[from]
        source: humantime::DurationError,
    },
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

impl<T> From<std::sync::PoisonError<T>> for Error
where
    T: std::fmt::Debug,
{
    fn from(e: std::sync::PoisonError<T>) -> Self {
        Error::LockPoisonError {
            t: format!("{:#?}", e),
            cause: e.to_string(),
        }
    }
}

impl Error {
    /// Write error message to stderr using [log::debug] and return the error
    pub fn debug(self) -> Self {
        debug!("{}", self);
        self
    }
    /// Write error message to stderr using [log::trace] and return the error
    pub fn trace(self) -> Self {
        trace!("{}", self);
        self
    }

    /// Write error message to stderr using [log::warn] and return the error
    pub fn warn(self) -> Self {
        warn!("{}", self);
        self
    }
    /// Write error message to stderr using [log::error] and return the error
    pub fn error(self) -> Self {
        error!("{}", self);
        self
    }
    /// Write error message to stderr using [log::info] and return the error
    pub fn info(self) -> Self {
        info!("{}", self);
        self
    }
    /// Write error message to stderr using [panic!] and quit.
    pub fn panic(self) -> Self {
        panic!("{}", self);
    }
}

/// Result type for xvc-file crate
pub type Result<T> = std::result::Result<T, Error>;
