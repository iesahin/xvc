//! Error handling for Xvc library
use log::{debug, error, info, trace, warn};

use std::env::VarError;
use std::ffi::OsString;
use std::fmt::Debug;
use std::io;
use std::num::ParseIntError;
use std::string::FromUtf8Error;
use thiserror::Error as ThisError;

use assert_cmd;
use assert_fs;

#[allow(missing_docs)]
#[derive(ThisError, Debug)]
/// Error codes and messages for main Xvc CLI
///
/// Uses [thiserror] for [From] and [Display conversions.
pub enum Error {
    #[error("Sorry. {0} is not implemented yet")]
    Todo(&'static str),

    #[error("Error in Output Channel")]
    OutputError,

    #[error("General Xvc Error: {source}")]
    AnyhowError {
        #[from]
        source: anyhow::Error,
    },
    #[error("Core Error: {source}")]
    CoreError {
        #[from]
        source: xvc_core::error::Error,
    },
    #[error("Pipeline Error: {source}")]
    PipelineError {
        #[from]
        source: xvc_pipeline::error::Error,
    },
    #[error("File Error: {source}")]
    FileError {
        #[from]
        source: xvc_file::error::Error,
    },
    #[error("Ecs Error: {source}")]
    EcsError {
        #[from]
        source: xvc_ecs::error::Error,
    },
    #[error("Walker Error: {source}")]
    WalkerError {
        #[from]
        source: xvc_walker::error::Error,
    },

    #[error("Configuration Error: {source}")]
    ConfigError {
        #[from]
        source: xvc_config::error::Error,
    },

    #[error("Storage Error: {source}")]
    StorageError {
        #[from]
        source: xvc_storage::Error,
    },

    #[error("Environment Variable Error: {source}")]
    VarError {
        #[from]
        source: VarError,
    },

    #[error("Process Error - stdout: {stdout}\nstderr: {stderr}")]
    ProcessError { stdout: String, stderr: String },
    #[error("Process Exec Error: {source}")]
    ProcessExecError {
        #[from]
        source: subprocess::PopenError,
    },
    #[error("[E1004] Json Serialization Error: {source}")]
    JsonError {
        #[from]
        source: serde_json::Error,
    },
    #[error("Yaml Error: {source}")]
    YamlError {
        #[from]
        source: serde_yaml::Error,
    },

    #[error("This command requires Xvc repository. Please use xvc init first.")]
    RequiresXvcRepository,

    #[error("I/O Error: {source}")]
    IoError {
        #[from]
        source: io::Error,
    },
    #[error("Path is not in Xvc Repository: {path:?}")]
    PathNotInXvcRepository { path: OsString },
    #[error("Path has no parent: {path:?}")]
    PathHasNoParent { path: OsString },
    #[error("This directory already belongs to an Xvc repository {path:?}")]
    DirectoryContainsXvcAlready { path: OsString },
    #[error("This directory is not in a Git Repository {path:?}")]
    PathNotInGitRepository { path: OsString },
    #[error("Cannot Parse Integer: {source:?}")]
    CannotParseInteger {
        #[from]
        source: ParseIntError,
    },
    #[error("Cannot Find Executable: {source}")]
    WhichError {
        #[from]
        source: which::Error,
    },

    #[error("Git Process Error: \nSTDOUT: {stdout}\nSTDERR: {stderr}")]
    GitProcessError { stdout: String, stderr: String },

    #[error("Fixture Error: {source}")]
    FixtureError {
        #[from]
        source: assert_fs::fixture::FixtureError,
    },

    #[error("Cargo Error: {source}")]
    CargoError {
        #[from]
        source: assert_cmd::cargo::CargoError,
    },

    #[cfg(test)]
    #[error("FS Extra Error: {source}")]
    FsExtraError {
        #[from]
        source: fs_extra::error::Error,
    },

    #[error("Cannot convert to Utf-8")]
    FromUtf8Error {
        #[from]
        source: FromUtf8Error,
    },
}

impl Error {
    /// Emit debug message for Error
    pub fn debug(self) -> Self {
        debug!("{}", self);
        self
    }
    /// Emit trace message for Error
    pub fn trace(self) -> Self {
        trace!("{}", self);
        self
    }
    /// Emit warning message for Error
    pub fn warn(self) -> Self {
        warn!("{}", self);
        self
    }
    /// Emit error message for Error
    pub fn error(self) -> Self {
        error!("{}", self);
        self
    }
    /// Emit info message for Error
    pub fn info(self) -> Self {
        info!("{}", self);
        self
    }
    /// Panics for Error
    pub fn panic(self) -> Self {
        panic!("{}", self);
    }
}

/// The result type for main Xvc library
///
/// Almost all functions in this crate return something of this type.
pub type Result<T> = std::result::Result<T, Error>;
