//! Error messages for xvc-storage
use log::{debug, error, info, trace, warn};

use std::fmt::Debug;
use std::io;
use thiserror::Error as ThisError;

use crate::StorageIdentifier;

/// Error messages for xvc-storage
#[derive(ThisError, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Sorry. {0} is not implemented yet")]
    Todo(&'static str),

    #[error("General Xvc Remote Error: {source}")]
    AnyhowError {
        #[from]
        source: anyhow::Error,
    },

    #[error("Xvc ECS Error: {source}")]
    EcsError {
        #[from]
        source: xvc_ecs::error::Error,
    },

    #[error("Xvc Core Error: {source}")]
    CoreError {
        #[from]
        source: xvc_core::error::Error,
    },

    #[error("Xvc Config Error: {source}")]
    ConfigError {
        #[from]
        source: xvc_config::error::Error,
    },

    #[error("Walker Error: {source}")]
    WalkerError {
        #[from]
        source: xvc_walker::error::Error,
    },
    #[error("I/O Error: {source}")]
    IoError {
        #[from]
        source: io::Error,
    },
    #[error("Crossbeam Send Error for Type: {t:?} {cause:?}")]
    CrossbeamSendError { t: String, cause: String },

    #[error("Uuid Error: {source:?}")]
    UuidError {
        #[from]
        source: uuid::Error,
    },

    #[error("No Guid found for Xvc Repository")]
    NoRepositoryGuidFound,

    #[error("Cannot find remote with identifier: {identifier}")]
    CannotFindRemoteWithIdentifier { identifier: StorageIdentifier },

    #[error("Process Exec Error: {source}")]
    ProcessExecError {
        #[from]
        source: subprocess::PopenError,
    },

    #[error("Process Error.\nSTDOUT:\n{stdout}\nSTDERR:\n{stderr}")]
    ProcessError { stdout: String, stderr: String },

    #[error("Cannot Find Executable: {source}")]
    WhichError {
        #[from]
        source: which::Error,
    },

    #[cfg(any(feature = "s3", feature = "minio"))]
    #[error("Cloud Credentials Error: {source}")]
    CloudCredentialsError {
        #[from]
        source: s3::creds::error::CredentialsError,
    },
    #[cfg(any(feature = "s3", feature = "minio"))]
    #[error("S3 Error: {source}")]
    S3Error {
        #[from]
        source: s3::error::S3Error,
    },

    #[error("Environment Variable Error: {source}")]
    VarError {
        #[from]
        source: std::env::VarError,
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

/// Result type for xvc-storage crate
pub type Result<T> = std::result::Result<T, Error>;
