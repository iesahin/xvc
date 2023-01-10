//! Error and Result types for the pipelines crate
use log::{debug, error, info, trace, warn};

use std::ffi::OsString;
use std::fmt::Debug;
use std::io;
use std::path::PathBuf;
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
    //
    // #[error("Cannot find {xvc_path} in cache: {cache_path}")]
    // CannotFindFileInCache {
    //     xvc_path: String,
    //     cache_path: String,
    // },
    // #[error("File not found: {path}")]
    // FileNotFound { path: PathBuf },
    // #[error("Internal Error: {message}")]
    // InternalError { message: String },
    // #[error("FS Extra error: {source}")]
    // FileSystemExtraError {
    //     #[from]
    //     source: fs_extra::error::Error,
    // },
    // #[error("Jwalk Error: {source}")]
    // JwalkError {
    //     #[from]
    //     source: jwalk::Error,
    // },
    // #[error("Conflicting Command Line Flags: {flags:?}")]
    // ConflictingFlags { flags: &'static [String] },
    // #[error("File System Walk Error: {error}")]
    // FSWalkerError { error: String },
    #[error("Cannot infer format from file extension: {extension:?}")]
    CannotInferFormatFromExtension { extension: OsString },
    #[error("Format specification for input (stdin) required.")]
    FormatSpecificationRequired,
    #[error("Process Error - stdout: {stdout}\nstderr: {stderr}")]
    ProcessError { stdout: String, stderr: String },
    // #[error("Process Exec Error: {source}")]
    // ProcessExecError {
    //     #[from]
    //     source: subprocess::PopenError,
    // },
    // #[error("Cannot find XVC Root: {path}")]
    // CannotFindXvcRoot { path: PathBuf },
    //
    // #[error("Thread Error")]
    // ThreadError,
    //
    // #[error("Cannot nest XVC repositories: {path}")]
    // CannotNestXvcRepositories { path: PathBuf },
    //
    // #[error("Cannot set step to both frozen and run_always")]
    // CannotSetToBothFrozenAndRunAlways,
    // #[error("Regex Error: {source}")]
    // RegexError {
    //     #[from]
    //     source: regex::Error,
    // },
    // #[error("Command Line Parsing Error: {source}")]
    // ClapError {
    //     #[from]
    //     source: ClapError,
    // },
    #[error("Invalid regular expression: {regex}")]
    InvalidRegexFormat { regex: String },
    //
    #[error("Invalid lines definition: {line}")]
    InvalidLinesFormat { line: String },
    //
    #[error("Step {step} not found in pipeline")]
    StepNotFoundInPipeline { step: String },
    // #[error("System Time Error: {source}")]
    // SystemTimeError {
    //     #[from]
    //     source: std::time::SystemTimeError,
    // },
    // #[error("[E1002] MsgPack Serialization Error: {source}")]
    // MsgPackDecodeError {
    //     #[from]
    //     source: rmp_serde::decode::Error,
    // },
    // #[error("[E1003] MsgPack Serialization Error: {source}")]
    // MsgPackEncodeError {
    //     #[from]
    //     source: rmp_serde::encode::Error,
    // },
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
    // //// ****** Data Errors ******
    // #[error("Unsupported Target Type: {path}")]
    // UnsupportedTargetType { path: String },
    // #[error("Target is ignored, please unignore in .xvcignore: {path}")]
    // TargetIgnored { path: String },
    //
    // //// ****** Pipeline Errors ******
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
    // #[error("Cannot convert enum type from string: {cause_key}")]
    // EnumTypeConversionError { cause_key: String },
    // #[error("Unicode/UTF-8 Error: {cause:?}")]
    // UnicodeError { cause: OsString },
    //
    // #[error("Path must be file, not symlink or directory")]
    // RequiresAFile { path: PathBuf },
    //
    // #[error("Path is not in XVC Repository: {path:?}")]
    // PathNotInXvcRepository { path: OsString },
    //
    #[error("Path not found in Path Metadata Map: {path:?}")]
    PathNotFoundInPathMetadataMap { path: OsString },
    //
    // #[error("Path has no parent: {path:?}")]
    // PathHasNoParent { path: OsString },
    //
    // #[error("Path has no filename: {path:?}")]
    // PathHasNoFilename { path: OsString },
    //
    #[error("Path has no modification time: {path:?}")]
    PathHasNoModificationTime { path: OsString },
    //
    // #[error("Cannot Parse Integer: {source:?}")]
    // CannotParseInteger {
    //     #[from]
    //     source: ParseIntError,
    // },
    //
    // #[error("Config source for level {config_source:?} not found at {path:?}")]
    // ConfigurationForSourceNotFound {
    //     config_source: String,
    //     path: OsString,
    // },
    //
    // #[error("Config value type mismatch: {key} ")]
    // MismatchedValueType { key: String },
    //
    // #[error("Config key not found: {key}")]
    // ConfigKeyNotFound { key: String },
    //
    // #[error("Cannot Determine System Configuration Path")]
    // CannotDetermineSystemConfigurationPath,
    //
    // #[error("Cannot Determine User Configuration Path")]
    // CannotDetermineUserConfigurationPath,
    //
    // #[error("No .xvcignore patterns found. There may be a problem in your setup")]
    // RequiresXvcIgnore,
    //
    // // #[error("XvcIgnore Error: {source}")]
    // // XvcIgnoreError {
    // //     #[from]
    // //     source: ignore::Error,
    // // },
    #[error("Internal Error: XvcDependencyComparisonError in Pipelines")]
    XvcDependencyComparisonError,
    //
    // #[error("Glob Error: {source}")]
    // GlobError {
    //     #[from]
    //     source: glob::GlobError,
    // },
    //
    // #[error("Glob Pattern Error: {source}")]
    // GlobPatternError {
    //     #[from]
    //     source: glob::PatternError,
    // },
    //
    // #[error("Enum Parsing Error")]
    // StrumError {
    //     #[from]
    //     source: strum::ParseError,
    // },
    //
    // #[error("Missing key: {key}")]
    // RequiresKey { key: String },
    //
    #[error("Missing value for key: {key}")]
    KeyNotFound { key: String },
    //
    #[error("Missing value for key: {key} in {path}")]
    KeyNotFoundInDocument { key: String, path: PathBuf },
    //
    // #[error("Parameter file not found: {path}")]
    // ParamFileNotFound { path: PathBuf },
    //
    #[error("Invalid Parameter Format: {param} ")]
    InvalidParameterFormat { param: String },

    #[error("Unsupported param file format: {path:?} ")]
    UnsupportedParamFileFormat { path: OsString },
    //
    // #[error("Path strip prefix error: {source}")]
    // StringPrefixError {
    //     #[from]
    //     source: std::path::StripPrefixError,
    // },
    //
    #[error("Crossbeam Send Error for Type: {t:?} {cause:?}")]
    CrossbeamSendError { t: String, cause: String },
    //
    // #[error("Only files or directories can be added: {path:?} ")]
    // OnlyFilesAndDirectoriesCanBeAdded { path: OsString },
    //
    // #[error("This directory already belongs to an XVC repository {path:?}")]
    // DirectoryContainsXVCAlready { path: OsString },
    //
    // #[error("This directory is not in a Git Repository {path:?}")]
    // PathNotInGitRepository { path: OsString },
    //
    // #[error("Cannot find a related entity: {entity}")]
    // NoParentEntityFound { entity: usize },
    //
    // #[error("More than one root entity found in an 1-N relation")]
    // MoreThanOneParentFound { entity: usize },
    //
    // #[error("Cannot find key in store: {key}")]
    // CannotFindKeyInStore { key: usize },
    //
    // #[error("Cannot find xvc meta file in {path:?}")]
    // CannotFindXvcMetaFile { path: OsString },
    //
    #[error("Cannot find Pipeline: {name}")]
    CannotFindPipeline { name: String },
    //
    // #[error("Cannot find Step: {name}")]
    // CannotFindStep { name: String },
    //
    // #[error("Internal Store Conversion Error")]
    // StoreConversionError,
    //
    // #[error("Can initialize {object} only once")]
    // CanInitializeOnlyOnce { object: String },
    //
    // #[error("Relative Path Conversion Error: {source}")]
    // RelativePathError {
    //     #[from]
    //     source: relative_path::FromPathError,
    // },
    //
    // #[error("Error in Key/Value Store: {source}")]
    // SledError {
    //     #[from]
    //     source: SledError,
    // },
    // // #[error("the data for key `{0}` is not available")]
    // // Redaction(String),
    // // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // // InvalidHeader { expected: String, found: String },
    // // #[error("unknown data store error")]
    // // Unknown
    // #[error("ScanDir Error: {source}")]
    // ScanDirError {
    //     #[from]
    //     source: ScanDirError,
    // },
    // #[error("ScanDir Error: {sources:?}")]
    // ScanDirErrors { sources: Vec<ScanDirError> },
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
//
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
