//! Error and Result types for the pipelines crate
use log::{debug, error, info, trace, warn};

use std::ffi::OsString;
use std::fmt::Debug;
use std::io;
use std::num::TryFromIntError;
use std::path::PathBuf;
use std::sync::PoisonError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
/// Error messages for pipelines crate
#[allow(missing_docs)]
pub enum Error {
    #[error("Sorry. {0} is not implemented yet")]
    Todo(&'static str),

    #[error("General Xvc Pipelines Error: {source}")]
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

    #[error("Cannot infer format from file extension: {extension:?}")]
    CannotInferFormatFromExtension { extension: OsString },
    #[error("Format specification for input (stdin) required.")]
    FormatSpecificationRequired,
    #[error("Process Error - stdout: {stdout}\nstderr: {stderr}")]
    ProcessError { stdout: String, stderr: String },
    #[error("Process Exec Error: {source}")]
    ProcessExecError {
        #[from]
        source: subprocess::PopenError,
    },
    #[error("Invalid regular expression: {regex}")]
    InvalidRegexFormat { regex: String },
    //
    #[error("Invalid lines definition: {line}")]
    InvalidLinesFormat { line: String },
    //
    #[error("Step {step} not found in pipeline")]
    StepNotFoundInPipeline { step: String },
    #[error("[E1004] Json Serialization Error: {source}")]
    JsonError {
        #[from]
        source: serde_json::Error,
    },
    #[error("Encountered NULL value in JSON map")]
    JsonNullValueForKey { key: String },
    //
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
    #[error("[E2001] Step with name '{step_name}' already found in {pipeline_name}")]
    StepAlreadyFoundInPipeline {
        step_name: String,
        pipeline_name: String,
    },
    #[error("[E2002] Stage with name already found")]
    StepRequiresName,
    #[error("[E2003] The command xvc {command} requires subcommand.")]
    RequiresSubCommand { command: String },
    #[error("[E2004] Requires xvc repository.")]
    RequiresXvcRepository,
    #[error("Pipeline {name} already found")]
    PipelineAlreadyFound { name: String },
    #[error("Pipeline {name} is not found")]
    NoPipelinesFound { name: String },
    #[error("Pipeline Steps Contain Cycle")]
    PipelineStepsContainCycle { pipeline: String, step: String },
    #[error("Cannot delete last pipeline")]
    CannotDeleteLastPipeline,
    #[error("Cannot delete default pipeline: {name}")]
    CannotDeleteDefaultPipeline { name: String },
    //
    #[error("Pipeline cannot depend to itself")]
    PipelineCannotDependToItself,

    #[error("Step cannot depend to itself")]
    StepCannotDependToItself,

    #[error("Internal Error: Content Digest for Pipeline Dependencies is not available. ")]
    NoContentDigestForPipelines,

    #[error("Internal Error: Content Digest for Step Dependencies is not available. ")]
    NoContentDigestForSteps,
    //
    #[error("I/O Error: {source}")]
    IoError {
        #[from]
        source: io::Error,
    },

    #[error("Unicode/UTF-8 Error: {source:?}")]
    UnicodeError {
        #[from]
        source: std::string::FromUtf8Error,
    },
    #[error("Poison Error: {cause:?}")]
    PoisonError { cause: String },
    #[error("Path not found: {path:?}")]
    PathNotFound { path: OsString },
    #[error("Path has no modification time: {path:?}")]
    PathHasNoModificationTime { path: OsString },
    #[error("Internal Error: XvcDependencyComparisonError in Pipelines")]
    XvcDependencyComparisonError,
    #[error("Missing value for key: {key}")]
    KeyNotFound { key: String },
    //
    #[error("Missing value for key: {key} in {path}")]
    KeyNotFoundInDocument { key: String, path: PathBuf },

    #[error("Pattern Error: {source}")]
    PatternError {
        #[from]
        source: glob::PatternError,
    },

    #[error("URL Request Error: {source}")]
    UrlRequestError {
        #[from]
        source: reqwest::Error,
    },

    #[error("Invalid Parameter Format: {param} ")]
    InvalidParameterFormat { param: String },

    #[error("Unsupported param file format: {path:?} ")]
    UnsupportedParamFileFormat { path: OsString },

    #[error("Crossbeam Send Error for Type: {t:?} {cause:?}")]
    CrossbeamSendError { t: String, cause: String },
    #[error("Crossbeam Recv Error: {source}")]
    CrossbeamRecvError {
        #[from]
        source: crossbeam_channel::RecvError,
    },

    #[error("Cannot find Pipeline: {name}")]
    CannotFindPipeline { name: String },

    #[error("Cannot parse url: {source}")]
    CannotParseUrl {
        #[from]
        source: url::ParseError,
    },

    #[error("Try Receive Error: {source}")]
    TryReceiveError {
        #[from]
        source: crossbeam_channel::TryRecvError,
    },

    #[error("Cannot cast from: {source}")]
    TryFromIntError {
        #[from]
        source: TryFromIntError,
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
    /// Log the error and return it
    pub fn debug(self) -> Self {
        debug!("{}", self);
        self
    }
    /// Log the error and return it
    pub fn trace(self) -> Self {
        trace!("{}", self);
        self
    }
    /// Log the error and return it
    pub fn warn(self) -> Self {
        warn!("{}", self);
        self
    }
    /// Log the error and return it
    pub fn error(self) -> Self {
        error!("{}", self);
        self
    }
    /// Log the error and return it
    pub fn info(self) -> Self {
        info!("{}", self);
        self
    }
    /// Panic with the error
    pub fn panic(self) -> Self {
        panic!("{}", self);
    }
}

/// The result type for xvc pipeline crate
pub type Result<T> = std::result::Result<T, Error>;
