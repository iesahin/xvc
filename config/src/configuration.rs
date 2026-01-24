//! This module defines the core Xvc configuration structures and logic.
//! It includes `XvcConfiguration` for complete settings, `XvcOptionalConfiguration` for partial overrides,
//! and functions for merging configurations from various sources like default values, files, and environment variables.

use std::collections::HashMap;
use std::path::Path;

use crate::Result;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use xvc_walker::AbsolutePath;

/// Core configuration for Xvc.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("CoreConfig(xvc_repo_version: {xvc_repo_version}, verbosity: {verbosity})")]
#[serde(deny_unknown_fields)]
pub struct CoreConfig {
    /// The Xvc repository version.
    pub xvc_repo_version: u8,
    /// The verbosity level for logging.
    pub verbosity: String,
}

/// Git integration configuration for Xvc.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("GitConfig(use_git: {use_git}, command: {command}, auto_commit: {auto_commit}, auto_stage: {auto_stage})")]
#[serde(deny_unknown_fields)]
pub struct GitConfig {
    /// Whether to use Git for version control.
    pub use_git: bool,
    /// The command to execute Git.
    pub command: String,
    /// Whether to automatically commit changes in the .xvc directory.
    pub auto_commit: bool,
    /// Whether to automatically stage changes in the .xvc directory.
    pub auto_stage: bool,
}

/// Cache configuration for Xvc.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("CacheConfig(algorithm: {algorithm})")]
#[serde(deny_unknown_fields)]
pub struct CacheConfig {
    /// The hashing algorithm used for caching.
    pub algorithm: String,
}

/// Configuration for file tracking operations.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("FileTrackConfig(no_commit: {no_commit}, force: {force}, text_or_binary: {text_or_binary}, no_parallel: {no_parallel}, include_git_files: {include_git_files})")]
#[serde(deny_unknown_fields)]
pub struct FileTrackConfig {
    /// Whether to skip committing changes after tracking.
    pub no_commit: bool,
    /// Whether to force tracking even if files are already tracked.
    pub force: bool,
    /// How to treat files: "auto", "text", or "binary".
    pub text_or_binary: String,
    /// Whether to disable parallel operations during tracking.
    pub no_parallel: bool,
    /// Whether to include Git-tracked files in Xvc tracking operations.
    pub include_git_files: bool,
}

/// Configuration for file listing operations.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("FileListConfig(format: {format}, sort: {sort}, show_dot_files: {show_dot_files}, no_summary: {no_summary}, recursive: {recursive}, include_git_files: {include_git_files})")]
#[serde(deny_unknown_fields)]
pub struct FileListConfig {
    /// The format string for displaying file list entries.
    pub format: String,
    /// The sorting order for the file list.
    pub sort: String,
    /// Whether to show dot files (e.g., .gitignore).
    pub show_dot_files: bool,
    /// Whether to suppress the summary row in the file list.
    pub no_summary: bool,
    /// Whether to list files recursively by default.
    pub recursive: bool,
    /// Whether to include Git-tracked files in Xvc listing operations.
    pub include_git_files: bool,
}

/// Configuration for file carry-in operations.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("FileCarryInConfig(force: {force}, no_parallel: {no_parallel})")]
#[serde(deny_unknown_fields)]
pub struct FileCarryInConfig {
    /// Whether to force carry-in even if files are already present in cache.
    pub force: bool,
    /// Whether to disable parallel operations during carry-in.
    pub no_parallel: bool,
}

/// Configuration for file recheck operations.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("FileRecheckConfig(method: {method})")]
#[serde(deny_unknown_fields)]
pub struct FileRecheckConfig {
    /// The method used for rechecking files (e.g., "copy", "hardlink", "symlink", "reflink").
    pub method: String,
}

/// Comprehensive file-related configuration for Xvc.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("FileConfig(track: {track}, list: {list}, carry_in: {carry_in}, recheck: {recheck})")]
#[serde(deny_unknown_fields)]
pub struct FileConfig {
    /// Configuration for `xvc file track`.
    pub track: FileTrackConfig,
    /// Configuration for `xvc file list`.
    pub list: FileListConfig,
    /// Configuration for `xvc file carry-in`.
    #[serde(rename = "carry-in")]
    pub carry_in: FileCarryInConfig,
    /// Configuration for `xvc file recheck`.
    pub recheck: FileRecheckConfig,
}

/// Configuration for pipeline operations.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("PipelineConfig(current_pipeline: {current_pipeline}, default: {default}, default_params_file: {default_params_file}, process_pool_size: {process_pool_size})")]
#[serde(deny_unknown_fields)]
pub struct PipelineConfig {
    /// The name of the currently active pipeline.
    pub current_pipeline: String,
    /// The name of the default pipeline.
    pub default: String,
    /// The default parameters file name for pipelines.
    pub default_params_file: String,
    /// The number of command processes to run concurrently in a pipeline.
    pub process_pool_size: u32,
}

/// Configuration for checking ignored files.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("CheckIgnoreConfig(details: {details})")]
#[serde(deny_unknown_fields)]
pub struct CheckIgnoreConfig {
    /// Whether to show detailed information when checking ignored files.
    pub details: bool,
}

/// The top-level Xvc configuration structure.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("XvcConfiguration(core: {core}, git: {git}, cache: {cache}, file: {file}, pipeline: {pipeline}, check_ignore: {check_ignore})")]
#[serde(deny_unknown_fields)]
pub struct XvcConfiguration {
    /// Core Xvc settings.
    pub core: CoreConfig,
    /// Git integration settings.
    pub git: GitConfig,
    /// Cache settings.
    pub cache: CacheConfig,
    /// File-related operation settings.
    pub file: FileConfig,
    /// Pipeline execution settings.
    pub pipeline: PipelineConfig,
    /// Check ignore settings.
    #[serde(rename = "check-ignore")]
    pub check_ignore: CheckIgnoreConfig,
}

/// Optional core configuration for Xvc, used for partial updates.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalCoreConfig(xvc_repo_version: {xvc_repo_version:?}, verbosity: {verbosity:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalCoreConfig {
    /// Optional Xvc repository version.
    pub xvc_repo_version: Option<u8>,
    /// Optional verbosity level for logging.
    pub verbosity: Option<String>,
    /// Optional GUID for the repository.
    /// This is a legacy field and should be migrated to .xvc/guid file.
    pub guid: Option<String>,
}

/// Optional Git integration configuration for Xvc, used for partial updates.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalGitConfig(use_git: {use_git:?}, command: {command:?}, auto_commit: {auto_commit:?}, auto_stage: {auto_stage:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalGitConfig {
    /// Optional setting for whether to use Git for version control.
    pub use_git: Option<bool>,
    /// Optional Git command to execute.
    pub command: Option<String>,
    /// Optional setting for whether to automatically commit changes.
    pub auto_commit: Option<bool>,
    /// Optional setting for whether to automatically stage changes.
    pub auto_stage: Option<bool>,
}

/// Optional cache configuration for Xvc, used for partial updates.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalCacheConfig(algorithm: {algorithm:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalCacheConfig {
    /// Optional hashing algorithm used for caching.
    pub algorithm: Option<String>,
}

/// Optional configuration for file tracking operations, used for partial updates.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalFileTrackConfig(no_commit: {no_commit:?}, force: {force:?}, text_or_binary: {text_or_binary:?}, no_parallel: {no_parallel:?}, include_git_files: {include_git_files:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalFileTrackConfig {
    /// Optional setting for whether to skip committing changes after tracking.
    pub no_commit: Option<bool>,
    /// Optional setting for whether to force tracking.
    pub force: Option<bool>,
    /// Optional setting for how to treat files: "auto", "text", or "binary".
    pub text_or_binary: Option<String>,
    /// Optional setting for whether to disable parallel operations during tracking.
    pub no_parallel: Option<bool>,
    /// Optional setting for whether to include Git-tracked files.
    pub include_git_files: Option<bool>,
}

/// Optional configuration for file listing operations, used for partial updates.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalFileListConfig(format: {format:?}, sort: {sort:?}, show_dot_files: {show_dot_files:?}, no_summary: {no_summary:?}, recursive: {recursive:?}, include_git_files: {include_git_files:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalFileListConfig {
    /// Optional format string for displaying file list entries.
    pub format: Option<String>,
    /// Optional sorting order for the file list.
    pub sort: Option<String>,
    /// Optional setting for whether to show dot files.
    pub show_dot_files: Option<bool>,
    /// Optional setting for whether to suppress the summary row.
    pub no_summary: Option<bool>,
    /// Optional setting for whether to list files recursively.
    pub recursive: Option<bool>,
    /// Optional setting for whether to include Git-tracked files in Xvc listing operations.
    pub include_git_files: Option<bool>,
}

/// Optional configuration for file carry-in operations, used for partial updates.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalFileCarryInConfig(force: {force:?}, no_parallel: {no_parallel:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalFileCarryInConfig {
    /// Optional setting for whether to force carry-in.
    pub force: Option<bool>,
    /// Optional setting for whether to disable parallel operations during carry-in.
    pub no_parallel: Option<bool>,
}

/// Optional configuration for file recheck operations, used for partial updates.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalFileRecheckConfig(method: {method:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalFileRecheckConfig {
    /// Optional recheck method for Xvc.
    pub method: Option<String>,
}

/// Optional comprehensive file-related configuration for Xvc, used for partial updates.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalFileConfig(track: {track:?}, list: {list:?}, carry_in: {carry_in:?}, recheck: {recheck:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalFileConfig {
    /// Optional configuration for `xvc file track`.
    pub track: Option<OptionalFileTrackConfig>,
    /// Optional configuration for `xvc file list`.
    pub list: Option<OptionalFileListConfig>,
    /// Optional configuration for `xvc file carry-in`.
    #[serde(rename = "carry-in")]
    pub carry_in: Option<OptionalFileCarryInConfig>,
    /// Optional configuration for `xvc file recheck`.
    pub recheck: Option<OptionalFileRecheckConfig>,
}

/// Optional configuration for pipeline operations, used for partial updates.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalPipelineConfig(current_pipeline: {current_pipeline:?}, default: {default:?}, default_params_file: {default_params_file:?}, process_pool_size: {process_pool_size:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalPipelineConfig {
    /// Optional name of the currently active pipeline.
    pub current_pipeline: Option<String>,
    /// Optional name of the default pipeline.
    pub default: Option<String>,
    /// Optional default parameters file name for pipelines.
    pub default_params_file: Option<String>,
    /// Optional number of command processes to run concurrently.
    pub process_pool_size: Option<u32>,
}

/// Optional configuration for checking ignored files, used for partial updates.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalCheckIgnoreConfig(details: {details:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalCheckIgnoreConfig {
    /// Optional setting for whether to show detailed information when checking ignored files.
    pub details: Option<bool>,
}

/// The top-level optional Xvc configuration structure, used for partial updates.
#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("XvcOptionalConfiguration(core: {core:?}, git: {git:?}, cache: {cache:?}, file: {file:?}, pipeline: {pipeline:?}, check_ignore: {check_ignore:?})")]
#[serde(deny_unknown_fields)]
pub struct XvcOptionalConfiguration {
    /// Optional core Xvc settings.
    pub core: Option<OptionalCoreConfig>,
    /// Optional Git integration settings.
    pub git: Option<OptionalGitConfig>,
    /// Optional cache settings.
    pub cache: Option<OptionalCacheConfig>,
    /// Optional file-related operation settings.
    pub file: Option<OptionalFileConfig>,
    /// Optional pipeline execution settings.
    pub pipeline: Option<OptionalPipelineConfig>,
    /// Optional check ignore settings.
    #[serde(rename = "check-ignore")]
    pub check_ignore: Option<OptionalCheckIgnoreConfig>,
}

impl XvcOptionalConfiguration {
    /// Creates an `XvcOptionalConfiguration` by reading and parsing a TOML file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the TOML configuration file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `XvcOptionalConfiguration` if successful, or an error.
    pub fn from_file(file_path: &Path) -> Result<Self> {
        let s =
            std::fs::read_to_string(&file_path).map_err(|e| crate::Error::IoError { source: e })?;
        let c: XvcOptionalConfiguration =
            toml::from_str(&s).map_err(|e| crate::Error::TomlDeserializationError { source: e })?;
        Ok(c)
    }

    fn parse_bool(s: &str) -> Option<bool> {
        match s {
            "1" | "TRUE" | "True" | "true" => Some(true),
            "0" | "FALSE" | "False" | "false" => Some(false),
            _ => None,
        }
    }

    /// Creates an `XvcOptionalConfiguration` from a HashMap of string key-value pairs.
    /// This is typically used to parse environment variables or command-line arguments.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix to filter keys from the HashMap (e.g., "XVC_").
    /// * `values` - A reference to the HashMap containing configuration values.
    ///
    /// # Returns
    ///
    /// A new `XvcOptionalConfiguration` instance.
    pub fn from_hash_map(prefix: &str, values: &HashMap<String, String>) -> Self {
        let mut config = Self {
            core: None,
            git: None,
            cache: None,
            file: None,
            pipeline: None,
            check_ignore: None,
        };

        for (key, value) in values.iter() {
            if !key.starts_with(prefix) {
                continue;
            }

            let key_str = &key[prefix.len()..].to_lowercase();
            match key_str.as_str() {
                // core
                "core.xvc_repo_version" => {
                    if let Ok(val) = value.parse::<u8>() {
                        config
                            .core
                            .get_or_insert_with(Default::default)
                            .xvc_repo_version = Some(val);
                    }
                }
                "core.verbosity" => {
                    config.core.get_or_insert_with(Default::default).verbosity =
                        Some(value.to_string());
                }
                // git
                "git.use_git" => {
                    if let Some(val) = Self::parse_bool(value) {
                        config.git.get_or_insert_with(Default::default).use_git = Some(val);
                    }
                }
                "git.command" => {
                    config.git.get_or_insert_with(Default::default).command =
                        Some(value.to_string());
                }
                "git.auto_commit" => {
                    if let Some(val) = Self::parse_bool(value) {
                        config.git.get_or_insert_with(Default::default).auto_commit = Some(val);
                    }
                }
                "git.auto_stage" => {
                    if let Some(val) = Self::parse_bool(value) {
                        config.git.get_or_insert_with(Default::default).auto_stage = Some(val);
                    }
                }
                // cache
                "cache.algorithm" => {
                    config.cache.get_or_insert_with(Default::default).algorithm =
                        Some(value.to_string());
                }
                // file.track
                "file.track.no_commit" => {
                    if let Some(val) = Self::parse_bool(value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .track
                            .get_or_insert_with(Default::default)
                            .no_commit = Some(val);
                    }
                }
                "file.track.force" => {
                    if let Some(val) = Self::parse_bool(value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .track
                            .get_or_insert_with(Default::default)
                            .force = Some(val);
                    }
                }
                "file.track.text_or_binary" => {
                    config
                        .file
                        .get_or_insert_with(Default::default)
                        .track
                        .get_or_insert_with(Default::default)
                        .text_or_binary = Some(value.to_string());
                }
                "file.track.no_parallel" => {
                    if let Some(val) = Self::parse_bool(value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .track
                            .get_or_insert_with(Default::default)
                            .no_parallel = Some(val);
                    }
                }
                "file.track.include_git_files" => {
                    if let Some(val) = Self::parse_bool(value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .track
                            .get_or_insert_with(Default::default)
                            .include_git_files = Some(val);
                    }
                }
                // file.list
                "file.list.format" => {
                    config
                        .file
                        .get_or_insert_with(Default::default)
                        .list
                        .get_or_insert_with(Default::default)
                        .format = Some(value.to_string());
                }
                "file.list.sort" => {
                    config
                        .file
                        .get_or_insert_with(Default::default)
                        .list
                        .get_or_insert_with(Default::default)
                        .sort = Some(value.to_string());
                }
                "file.list.show_dot_files" => {
                    if let Some(val) = Self::parse_bool(value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .list
                            .get_or_insert_with(Default::default)
                            .show_dot_files = Some(val);
                    }
                }
                "file.list.no_summary" => {
                    if let Some(val) = Self::parse_bool(value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .list
                            .get_or_insert_with(Default::default)
                            .no_summary = Some(val);
                    }
                }
                "file.list.recursive" => {
                    if let Some(val) = Self::parse_bool(value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .list
                            .get_or_insert_with(Default::default)
                            .recursive = Some(val);
                    }
                }
                "file.list.include_git_files" => {
                    if let Some(val) = Self::parse_bool(value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .list
                            .get_or_insert_with(Default::default)
                            .include_git_files = Some(val);
                    }
                }
                // file.carry_in
                "file.carry_in.force" => {
                    if let Some(val) = Self::parse_bool(value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .carry_in
                            .get_or_insert_with(Default::default)
                            .force = Some(val);
                    }
                }
                "file.carry_in.no_parallel" => {
                    if let Some(val) = Self::parse_bool(value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .carry_in
                            .get_or_insert_with(Default::default)
                            .no_parallel = Some(val);
                    }
                }
                // file.recheck
                "file.recheck.method" => {
                    config
                        .file
                        .get_or_insert_with(Default::default)
                        .recheck
                        .get_or_insert_with(Default::default)
                        .method = Some(value.to_string());
                }
                // pipeline
                "pipeline.current_pipeline" => {
                    config
                        .pipeline
                        .get_or_insert_with(Default::default)
                        .current_pipeline = Some(value.to_string());
                }
                "pipeline.default" => {
                    config.pipeline.get_or_insert_with(Default::default).default =
                        Some(value.to_string());
                }
                "pipeline.default_params_file" => {
                    config
                        .pipeline
                        .get_or_insert_with(Default::default)
                        .default_params_file = Some(value.to_string());
                }
                "pipeline.process_pool_size" => {
                    if let Ok(val) = value.parse::<u32>() {
                        config
                            .pipeline
                            .get_or_insert_with(Default::default)
                            .process_pool_size = Some(val);
                    }
                }
                // check_ignore
                "check_ignore.details" => {
                    if let Some(val) = Self::parse_bool(value) {
                        config
                            .check_ignore
                            .get_or_insert_with(Default::default)
                            .details = Some(val);
                    }
                }
                _ => {} // Ignore unknown keys
            }
        }
        config
    }

    /// Creates an `XvcOptionalConfiguration` by reading environment variables
    /// prefixed with "XVC_".
    ///
    /// # Returns
    ///
    /// A new `XvcOptionalConfiguration` instance populated from environment variables.
    pub fn from_env() -> Self {
        let env_vars: HashMap<String, String> = std::env::vars().collect();
        Self::from_hash_map("XVC_", &env_vars)
    }
}

/// How should we initialize the configuration?
///
/// It's possible to ignore certain sources by supplying `None` to their values here.
#[derive(Debug, Clone)]
pub struct XvcConfigParams {
    /// The default configuration for the project.
    /// It should contain all default values as a TOML document.
    /// Xvc produces this in [xvc_core::default_configuration].
    pub default_configuration: String,
    /// The directory where the application runs.
    /// This can be set by various Options.
    /// It affects how paths are handled in general.
    pub current_dir: AbsolutePath,
    /// Should we include system configuration?
    /// If `true`, it's read from [SYSTEM_CONFIG_DIRS].
    pub include_system_config: bool,
    /// Should the user's (home) config be included.
    /// If `true`, it's read from [USER_CONFIG_DIRS].
    pub include_user_config: bool,
    /// Where should we load the project's (public) configuration?
    /// It's loaded in [XvcRootInner::new]
    /// TODO: Add a option to ignore this
    pub project_config_path: Option<AbsolutePath>,
    /// Where should we load the project's (private) configuration?
    /// It's loaded in [XvcRootInner::new]
    /// TODO: Add a option to ignore this
    pub local_config_path: Option<AbsolutePath>,
    /// Should we include configuration from the environment.
    /// If `true`, look for all variables in the form
    ///
    /// `XVC_group.key=value`
    ///
    /// from the environment and put them into the configuration.
    pub include_environment_config: bool,
    /// Command line configuration
    pub command_line_config: Option<Vec<String>>,
}

impl XvcConfigParams {
    /// Creates a new `XvcConfigParams` instance with default settings.
    ///
    /// # Arguments
    ///
    /// * `default_configuration` - The default configuration as a TOML string.
    /// * `current_dir` - The absolute path of the current working directory.
    ///
    /// # Returns
    ///
    /// A new `XvcConfigParams` instance with default settings.
    pub fn new(default_configuration: String, current_dir: AbsolutePath) -> Self {
        Self {
            default_configuration,
            current_dir,
            include_system_config: true,
            include_user_config: true,
            project_config_path: None,
            local_config_path: None,
            include_environment_config: true,
            command_line_config: None,
        }
    }

    /// Updates the `include_system_config` value.
    ///
    /// # Arguments
    ///
    /// * `include_system_config` - Whether to include system configuration.
    ///
    /// # Returns
    ///
    /// The modified `XvcConfigParams` instance.
    pub fn include_system_config(mut self, include_system_config: bool) -> Self {
        self.include_system_config = include_system_config;
        self
    }

    /// Updates the `include_user_config` value.
    ///
    /// # Arguments
    ///
    /// * `include_user_config` - Whether to include user configuration.
    ///
    /// # Returns
    ///
    /// The modified `XvcConfigParams` instance.
    pub fn include_user_config(mut self, include_user_config: bool) -> Self {
        self.include_user_config = include_user_config;
        self
    }

    /// Updates the `project_config_path` value.
    ///
    /// # Arguments
    ///
    /// * `project_config_path` - The optional absolute path to the project's public configuration.
    ///
    /// # Returns
    ///
    /// The modified `XvcConfigParams` instance.
    pub fn project_config_path(mut self, project_config_path: Option<AbsolutePath>) -> Self {
        self.project_config_path = project_config_path;
        self
    }

    /// Updates the `local_config_path` value.
    ///
    /// # Arguments
    ///
    /// * `local_config_path` - The optional absolute path to the project's private configuration.
    ///
    /// # Returns
    ///
    /// The modified `XvcConfigParams` instance.
    pub fn local_config_path(mut self, local_config_path: Option<AbsolutePath>) -> Self {
        self.local_config_path = local_config_path;
        self
    }

    /// Whether to include environment variables in the configuration.
    ///
    /// # Arguments
    ///
    /// * `include_environment_config` - Whether to include environment variables.
    ///
    /// # Returns
    ///
    /// The modified `XvcConfigParams` instance.
    pub fn include_environment_config(mut self, include_environment_config: bool) -> Self {
        self.include_environment_config = include_environment_config;
        self
    }

    /// Sets the command line configuration from key=value definitions.
    ///
    /// # Arguments
    ///
    /// * `command_line_config` - An optional vector of strings representing command-line configurations.
    ///
    /// # Returns
    ///
    /// The modified `XvcConfigParams` instance.
    pub fn command_line_config(mut self, command_line_config: Option<Vec<String>>) -> Self {
        self.command_line_config = command_line_config;
        self
    }
}

/// Returns the default Xvc configuration.
///
/// This configuration serves as the base, which can then be overridden
/// by system, user, and repository-specific configurations.
///
/// # Returns
///
/// A `XvcConfiguration` struct populated with default values.
pub fn default_config() -> XvcConfiguration {
    XvcConfiguration {
        core: CoreConfig {
            xvc_repo_version: 2,
            verbosity: "error".to_string(),
        },
        git: GitConfig {
            use_git: true,
            command: "git".to_string(),
            auto_commit: true,
            auto_stage: false,
        },
        cache: CacheConfig {
            algorithm: "blake3".to_string(),
        },
        file: FileConfig {
            track: FileTrackConfig {
                no_commit: false,
                force: false,
                text_or_binary: "auto".to_string(),
                no_parallel: false,
                include_git_files: false,
            },
            list: FileListConfig {
                format: "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}".to_string(),
                sort: "name-desc".to_string(),
                show_dot_files: false,
                no_summary: false,
                recursive: false,
                include_git_files: false,
            },
            carry_in: FileCarryInConfig {
                force: false,
                no_parallel: false,
            },
            recheck: FileRecheckConfig {
                method: "copy".to_string(),
            },
        },
        pipeline: PipelineConfig {
            current_pipeline: "default".to_string(),
            default: "default".to_string(),
            default_params_file: "params.yaml".to_string(),
            process_pool_size: 4,
        },
        check_ignore: CheckIgnoreConfig { details: false },
    }
}

/// Merges an optional configuration into a base Xvc configuration.
///
/// This function applies values from `opt_config` to `config`, overwriting
/// any existing values in `config` if they are present in `opt_config`.
///
/// # Arguments
///
/// * `config` - A reference to the base `XvcConfiguration`.
/// * `opt_config` - A reference to the optional `XvcOptionalConfiguration` to merge.
///
/// # Returns
///
/// A new `XvcConfiguration` with merged values.
pub fn merge_configs(
    config: &XvcConfiguration,
    opt_config: &XvcOptionalConfiguration,
) -> XvcConfiguration {
    let core = CoreConfig {
        xvc_repo_version: opt_config
            .core
            .clone()
            .and_then(|c| c.xvc_repo_version)
            .unwrap_or(config.core.xvc_repo_version),
        verbosity: opt_config
            .core
            .clone()
            .and_then(|c| c.verbosity)
            .unwrap_or(config.core.verbosity.clone()),
    };

    let git = GitConfig {
        use_git: opt_config
            .git
            .clone()
            .and_then(|g| g.use_git)
            .unwrap_or(config.git.use_git),
        command: opt_config
            .git
            .clone()
            .and_then(|g| g.command)
            .unwrap_or(config.git.command.clone()),
        auto_commit: opt_config
            .git
            .clone()
            .and_then(|g| g.auto_commit)
            .unwrap_or(config.git.auto_commit),
        auto_stage: opt_config
            .git
            .clone()
            .and_then(|g| g.auto_stage)
            .unwrap_or(config.git.auto_stage),
    };

    let cache = CacheConfig {
        algorithm: opt_config
            .cache
            .clone()
            .and_then(|g| g.algorithm)
            .unwrap_or(config.cache.algorithm.clone()),
    };

    let opt_track = opt_config.file.clone().and_then(|f| f.track);
    let track = FileTrackConfig {
        no_commit: opt_track
            .clone()
            .and_then(|t| t.no_commit)
            .unwrap_or(config.file.track.no_commit),
        force: opt_track
            .clone()
            .and_then(|t| t.force)
            .unwrap_or(config.file.track.force),
        text_or_binary: opt_track
            .clone()
            .and_then(|t| t.text_or_binary)
            .unwrap_or(config.file.track.text_or_binary.clone()),
        no_parallel: opt_track
            .clone()
            .and_then(|t| t.no_parallel)
            .unwrap_or(config.file.track.no_parallel),
        include_git_files: opt_track
            .and_then(|t| t.include_git_files)
            .unwrap_or(config.file.track.include_git_files),
    };

    let opt_list = opt_config.file.clone().and_then(|f| f.list);
    let list = FileListConfig {
        format: opt_list
            .clone()
            .and_then(|l| l.format)
            .unwrap_or(config.file.list.format.clone()),
        sort: opt_list
            .clone()
            .and_then(|l| l.sort)
            .unwrap_or(config.file.list.sort.clone()),
        show_dot_files: opt_list
            .clone()
            .and_then(|l| l.show_dot_files)
            .unwrap_or(config.file.list.show_dot_files),
        no_summary: opt_list
            .clone()
            .and_then(|l| l.no_summary)
            .unwrap_or(config.file.list.no_summary),
        recursive: opt_list
            .clone()
            .and_then(|l| l.recursive)
            .unwrap_or(config.file.list.recursive),
        include_git_files: opt_list
            .and_then(|l| l.include_git_files)
            .unwrap_or(config.file.list.include_git_files),
    };

    let opt_carry_in = opt_config.file.clone().and_then(|f| f.carry_in);
    let carry_in = FileCarryInConfig {
        force: opt_carry_in
            .clone()
            .and_then(|c| c.force)
            .unwrap_or(config.file.carry_in.force),
        no_parallel: opt_carry_in
            .and_then(|c| c.no_parallel)
            .unwrap_or(config.file.carry_in.no_parallel),
    };

    let opt_recheck = opt_config.file.clone().and_then(|f| f.recheck);
    let recheck = FileRecheckConfig {
        method: opt_recheck
            .and_then(|r| r.method)
            .unwrap_or(config.file.recheck.method.clone()),
    };

    let file = FileConfig {
        track,
        list,
        carry_in,
        recheck,
    };

    let pipeline = PipelineConfig {
        current_pipeline: opt_config
            .pipeline
            .clone()
            .and_then(|p| p.current_pipeline)
            .unwrap_or(config.pipeline.current_pipeline.clone()),
        default: opt_config
            .pipeline
            .clone()
            .and_then(|p| p.default)
            .unwrap_or(config.pipeline.default.clone()),
        default_params_file: opt_config
            .pipeline
            .clone()
            .and_then(|p| p.default_params_file)
            .unwrap_or(config.pipeline.default_params_file.clone()),
        process_pool_size: opt_config
            .pipeline
            .clone()
            .and_then(|p| p.process_pool_size)
            .unwrap_or(config.pipeline.process_pool_size),
    };

    let check_ignore = CheckIgnoreConfig {
        details: opt_config
            .check_ignore
            .clone()
            .and_then(|c| c.details)
            .unwrap_or(config.check_ignore.details),
    };

    XvcConfiguration {
        core,
        git,
        cache,
        file,
        pipeline,
        check_ignore,
    }
}

/// Initializes the Xvc configuration file from the given [XvcConfiguration]
///
/// Please use [merge_configs] before this to get [config] from possibly
/// user supplied [XvcOptionalConfiguration]. See [init_xvc_root] function for a use of this.
///
/// # Arguments
///
/// * `config` - The `XvcConfiguration` to convert to a TOML file.
///
/// # Returns
///
/// A `Result` containing a TOML-formatted string of the merged configuration if successful, or an error.
pub fn initial_xvc_configuration_file(config: &XvcConfiguration) -> Result<String> {
    Ok(format!(
        r##"
[core]
xvc_repo_version = {xvc_repo_version}
# Default verbosity level.
# One of "error", "warn", "info"
verbosity = "{verbosity}"

[git]
# Automate git operations.
# Turning this off leads Xvc to behave as if it's not in a Git repository.
# Not recommended unless you're really not using Git
use_git = {use_git}
# Command to run Git process.
# You can set this to an absolute path to specify an executable
# If set to a non-absolute path, the executable will be searched in $PATH.
command = "{git_command}"

# Commit changes in .xvc/ directory after commands.
# You can set this to false if you want to commit manually.
auto_commit = {auto_commit}

# Stage changes in .xvc/ directory without committing.
# auto_commit implies auto_stage.
# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
auto_stage = {auto_stage}

[cache]
# The hash algorithm used for the cache.
# It may take blake3, blake2, sha2 or sha3 as values.
# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# The cache path is produced by prepending algorithm name to the cache.
# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
algorithm = "{cache_algorithm}"

[file]

[file.track]

# Don't move file content to cache after xvc file track
no_commit = {file_track_no_commit}
# Force to track files even if they are already tracked.
force = {file_track_force}

# Xvc calculates file content digest differently for text and binary files.
# This option controls whether to treat files as text or binary.
# It may take auto, text or binary as values.
# Auto check each file individually and treat it as text if it's text.
text_or_binary = "{file_track_text_or_binary}"

# Don't use parallelism in track operations.
# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
no_parallel = {file_track_no_parallel}

# Track files that are tracked by Git. 
include_git_files = {file_track_include_git_files}

[file.list]

# Format for `xvc file list` rows. You can reorder or remove columns.
# The following are the keys for each row:
# - {{acd64}}:  actual content digest. All 64 digits from the workspace file's content.
# - {{acd8}}:  actual content digest. First 8 digits the file content digest.
# - {{aft}}:  actual file type. Whether the entry is a file (F), directory (D),
#   symlink (S), hardlink (H) or reflink (R).
# - {{asz}}:  actual size. The size of the workspace file in bytes. It uses MB,
#   GB and TB to represent sizes larger than 1MB.
# - {{ats}}:  actual timestamp. The timestamp of the workspace file.
# - {{cst}}:  cache status. One of "=", ">", "<", "X", or "?" to show
#   whether the file timestamp is the same as the cached timestamp, newer,
#   older, not cached or not tracked.
# - {{name}}: The name of the file or directory.
# - {{rcd64}}:  recorded content digest. All 64 digits.
# - {{rcd8}}:  recorded content digest. First 8 digits.
# - {{rrm}}:  recorded recheck method. Whether the entry is linked to the workspace
#   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# - {{rsz}}:  recorded size. The size of the cached content in bytes. It uses
#   MB, GB and TB to represent sizes larger than 1MB.
# - {{rts}}:  recorded timestamp. The timestamp of the cached content.
#
# There are no escape sequences in the format string.
# If you want to add a tab, type it to the string.
# If you want to add a literal double curly brace, open an issue.
format = "{file_list_format}"

# Default sort order for `xvc file list`.
# Valid values are
# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
sort = "{file_list_sort}"

# Show dot files like .gitignore
show_dot_files = {file_list_show_dot_files}

# Do not show a summary for as the final row for `xvc file list`.
no_summary = {file_list_no_summary}

# List files recursively always.
recursive = {file_list_recursive}

# List files tracked by Git. 
include_git_files = {file_list_include_git_files}

[file.carry-in]
# Carry-in the files to cache always, even if they are already present.
force = {file_carry_in_force}

# Don't use parallel move/copy in carry-in
no_parallel = {file_carry_in_no_parallel}

[file.recheck]
# The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# The default is copy to make sure the options is portable.
# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# Note that hardlink and symlink are read-only as they link the files in cache.
method = "{file_recheck_method}"

[pipeline]
# Name of the current pipeline to run
current_pipeline = "{pipeline_current_pipeline}"
# Name of the default pipeline
default = "{pipeline_default}"
# Name of the default params file name
default_params_file = "{pipeline_default_params_file}"
# Number of command processes to run concurrently
process_pool_size = {pipeline_process_pool_size}
 
[check-ignore]
# Show details by default
details = {check_ignore_details}

"##,
        xvc_repo_version = config.core.xvc_repo_version,
        verbosity = config.core.verbosity,
        use_git = config.git.use_git,
        git_command = config.git.command,
        auto_commit = config.git.auto_commit,
        auto_stage = config.git.auto_stage,
        cache_algorithm = config.cache.algorithm,
        file_track_no_commit = config.file.track.no_commit,
        file_track_force = config.file.track.force,
        file_track_text_or_binary = config.file.track.text_or_binary,
        file_track_no_parallel = config.file.track.no_parallel,
        file_track_include_git_files = config.file.track.include_git_files,
        file_list_format = config.file.list.format,
        file_list_sort = config.file.list.sort,
        file_list_show_dot_files = config.file.list.show_dot_files,
        file_list_no_summary = config.file.list.no_summary,
        file_list_recursive = config.file.list.recursive,
        file_list_include_git_files = config.file.list.include_git_files,
        file_carry_in_force = config.file.carry_in.force,
        file_carry_in_no_parallel = config.file.carry_in.no_parallel,
        file_recheck_method = config.file.recheck.method,
        pipeline_current_pipeline = config.pipeline.current_pipeline,
        pipeline_default = config.pipeline.default,
        pipeline_default_params_file = config.pipeline.default_params_file,
        pipeline_process_pool_size = config.pipeline.process_pool_size,
        check_ignore_details = config.check_ignore.details,
    ))
}

/// Returns a blank `XvcOptionalConfiguration` with all fields set to `None`.
///
/// This is useful as a starting point when no optional configuration overrides are provided.
///
/// # Returns
///
/// A new `XvcOptionalConfiguration` instance with no values set.
pub fn blank_optional_config() -> XvcOptionalConfiguration {
    XvcOptionalConfiguration {
        core: None,
        git: None,
        cache: None,
        file: None,
        pipeline: None,
        check_ignore: None,
    }
}

impl XvcConfiguration {
    /// Creates an `XvcConfiguration` by reading and parsing a TOML file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the TOML configuration file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `XvcConfiguration` if successful, or an error.
    pub fn from_file(file_path: &Path) -> Result<Self> {
        let s =
            std::fs::read_to_string(&file_path).map_err(|e| crate::Error::IoError { source: e })?;
        let c: XvcConfiguration =
            toml::from_str(&s).map_err(|e| crate::Error::TomlDeserializationError { source: e })?;
        Ok(c)
    }

    /// Merges an optional configuration into the current configuration.
    ///
    /// # Arguments
    ///
    /// * `opt_config` - A reference to the `XvcOptionalConfiguration` to merge.
    ///
    /// # Returns
    ///
    /// A new `XvcConfiguration` with the merged values.
    pub fn merge_with_optional(&self, opt_config: &XvcOptionalConfiguration) -> Self {
        merge_configs(self, opt_config)
    }
}
