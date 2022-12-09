//! [Error] codes and messages for xvc-file crate
use log::{debug, error, info, trace, warn};

use std::fmt::Debug;
use std::io;
use std::path::PathBuf;
use thiserror::Error as ThisError;

/// Error messages for xvc-file
#[allow(missing_docs)]
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Sorry. {0} is not implemented yet")]
    Todo(&'static str),
    #[error("General Xvc File Error. {source}")]
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
        source: xvc_walker::error::Error,
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

impl Error {
    pub fn debug(self) -> Self {
        debug!("{}", self);
        self
    }
    pub fn trace(self) -> Self {
        trace!("{}", self);
        self
    }
    pub fn warn(self) -> Self {
        warn!("{}", self);
        self
    }
    pub fn error(self) -> Self {
        error!("{}", self);
        self
    }
    pub fn info(self) -> Self {
        info!("{}", self);
        self
    }
    pub fn panic(self) -> Self {
        panic!("{}", self);
    }
}

pub type Result<T> = std::result::Result<T, Error>;
