//! Error codes and messages for Xvc Walker
use log::{debug, error, info, trace, warn};
use std::hash::Hash;
use thiserror::Error as ThisError;

#[allow(missing_docs)]
#[derive(ThisError, Debug)]
/// Error type for Xvc Walker
pub enum Error {
    #[error("General Xvc Walker Error: {source}")]
    AnyhowError {
        #[from]
        source: anyhow::Error,
    },
    #[error("Crossbeam Send Error for Type: {t:?} {cause:?}")]
    CrossbeamSendError { t: String, cause: String },

    #[error("Ignore rules poisoned")]
    LockPoisonError { t: String, cause: String },

    #[error("Glob error: {source}")]
    GlobError {
        #[from]
        source: globset::Error,
    },

    #[error("File System Notify Error: {source:?}")]
    NotifyError {
        #[from]
        source: notify::Error,
    },

    #[error("I/O Error: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("Cannot Merge Empty Ignore Rules")]
    CannotMergeEmptyIgnoreRules,
}

impl Hash for Error {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{}", self).hash(state);
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        format!("{}", self).eq(&format!("{}", other))
    }
}

impl<T> From<crossbeam_channel::SendError<T>> for Error
where
    T: std::fmt::Debug,
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

/// Result type for xvc-walker that may also return [Error]
pub type Result<T> = std::result::Result<T, Error>;
