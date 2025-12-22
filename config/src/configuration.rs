use std::path::Path;

use crate::Result;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use xvc_walker::AbsolutePath;

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("CoreConfig(xvc_repo_version: {xvc_repo_version}, verbosity: {verbosity})")]
#[serde(deny_unknown_fields)]
pub struct CoreConfig {
    pub xvc_repo_version: u8,
    pub verbosity: String,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("GitConfig(use_git: {use_git}, command: {command}, auto_commit: {auto_commit}, auto_stage: {auto_stage})")]
#[serde(deny_unknown_fields)]
pub struct GitConfig {
    pub use_git: bool,
    pub command: String,
    pub auto_commit: bool,
    pub auto_stage: bool,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("CacheConfig(algorithm: {algorithm})")]
#[serde(deny_unknown_fields)]
pub struct CacheConfig {
    pub algorithm: String,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("FileTrackConfig(no_commit: {no_commit}, force: {force}, text_or_binary: {text_or_binary}, no_parallel: {no_parallel}, include_git_files: {include_git_files})")]
#[serde(deny_unknown_fields)]
pub struct FileTrackConfig {
    pub no_commit: bool,
    pub force: bool,
    pub text_or_binary: String,
    pub no_parallel: bool,
    pub include_git_files: bool,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("FileListConfig(format: {format}, sort: {sort}, show_dot_files: {show_dot_files}, no_summary: {no_summary}, recursive: {recursive}, include_git_files: {include_git_files})")]
#[serde(deny_unknown_fields)]
pub struct FileListConfig {
    pub format: String,
    pub sort: String,
    pub show_dot_files: bool,
    pub no_summary: bool,
    pub recursive: bool,
    pub include_git_files: bool,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("FileCarryInConfig(force: {force}, no_parallel: {no_parallel})")]
#[serde(deny_unknown_fields)]
pub struct FileCarryInConfig {
    pub force: bool,
    pub no_parallel: bool,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("FileRecheckConfig(method: {method})")]
#[serde(deny_unknown_fields)]
pub struct FileRecheckConfig {
    pub method: String,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("FileConfig(track: {track}, list: {list}, carry_in: {carry_in}, recheck: {recheck})")]
#[serde(deny_unknown_fields)]
pub struct FileConfig {
    pub track: FileTrackConfig,
    pub list: FileListConfig,
    #[serde(rename = "carry-in")]
    pub carry_in: FileCarryInConfig,
    pub recheck: FileRecheckConfig,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("PipelineConfig(current_pipeline: {current_pipeline}, default: {default}, default_params_file: {default_params_file}, process_pool_size: {process_pool_size})")]
#[serde(deny_unknown_fields)]
pub struct PipelineConfig {
    pub current_pipeline: String,
    pub default: String,
    pub default_params_file: String,
    pub process_pool_size: u32,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("CheckIgnoreConfig(details: {details})")]
#[serde(deny_unknown_fields)]
pub struct CheckIgnoreConfig {
    pub details: bool,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[display("XvcConfiguration(core: {core}, git: {git}, cache: {cache}, file: {file}, pipeline: {pipeline}, check_ignore: {check_ignore})")]
#[serde(deny_unknown_fields)]
pub struct XvcConfiguration {
    pub core: CoreConfig,
    pub git: GitConfig,
    pub cache: CacheConfig,
    pub file: FileConfig,
    pub pipeline: PipelineConfig,
    #[serde(rename = "check-ignore")]
    pub check_ignore: CheckIgnoreConfig,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalCoreConfig(xvc_repo_version: {xvc_repo_version:?}, verbosity: {verbosity:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalCoreConfig {
    pub xvc_repo_version: Option<u8>,
    pub verbosity: Option<String>,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalGitConfig(use_git: {use_git:?}, command: {command:?}, auto_commit: {auto_commit:?}, auto_stage: {auto_stage:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalGitConfig {
    pub use_git: Option<bool>,
    pub command: Option<String>,
    pub auto_commit: Option<bool>,
    pub auto_stage: Option<bool>,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalCacheConfig(algorithm: {algorithm:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalCacheConfig {
    pub algorithm: Option<String>,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalFileTrackConfig(no_commit: {no_commit:?}, force: {force:?}, text_or_binary: {text_or_binary:?}, no_parallel: {no_parallel:?}, include_git_files: {include_git_files:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalFileTrackConfig {
    pub no_commit: Option<bool>,
    pub force: Option<bool>,
    pub text_or_binary: Option<String>,
    pub no_parallel: Option<bool>,
    pub include_git_files: Option<bool>,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalFileListConfig(format: {format:?}, sort: {sort:?}, show_dot_files: {show_dot_files:?}, no_summary: {no_summary:?}, recursive: {recursive:?}, include_git_files: {include_git_files:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalFileListConfig {
    pub format: Option<String>,
    pub sort: Option<String>,
    pub show_dot_files: Option<bool>,
    pub no_summary: Option<bool>,
    pub recursive: Option<bool>,
    pub include_git_files: Option<bool>,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalFileCarryInConfig(force: {force:?}, no_parallel: {no_parallel:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalFileCarryInConfig {
    pub force: Option<bool>,
    pub no_parallel: Option<bool>,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalFileRecheckConfig(method: {method:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalFileRecheckConfig {
    pub method: Option<String>,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalFileConfig(track: {track:?}, list: {list:?}, carry_in: {carry_in:?}, recheck: {recheck:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalFileConfig {
    pub track: Option<OptionalFileTrackConfig>,
    pub list: Option<OptionalFileListConfig>,
    #[serde(rename = "carry-in")]
    pub carry_in: Option<OptionalFileCarryInConfig>,
    pub recheck: Option<OptionalFileRecheckConfig>,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalPipelineConfig(current_pipeline: {current_pipeline:?}, default: {default:?}, default_params_file: {default_params_file:?}, process_pool_size: {process_pool_size:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalPipelineConfig {
    pub current_pipeline: Option<String>,
    pub default: Option<String>,
    pub default_params_file: Option<String>,
    pub process_pool_size: Option<u32>,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("OptionalCheckIgnoreConfig(details: {details:?})")]
#[serde(deny_unknown_fields)]
pub struct OptionalCheckIgnoreConfig {
    pub details: Option<bool>,
}

#[derive(Display, Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[display("XvcOptionalConfiguration(core: {core:?}, git: {git:?}, cache: {cache:?}, file: {file:?}, pipeline: {pipeline:?}, check_ignore: {check_ignore:?})")]
#[serde(deny_unknown_fields)]
pub struct XvcOptionalConfiguration {
    pub core: Option<OptionalCoreConfig>,
    pub git: Option<OptionalGitConfig>,
    pub cache: Option<OptionalCacheConfig>,
    pub file: Option<OptionalFileConfig>,
    pub pipeline: Option<OptionalPipelineConfig>,
    pub check_ignore: Option<OptionalCheckIgnoreConfig>,
}

impl XvcOptionalConfiguration {
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

    pub fn from_env() -> Self {
        let mut config = Self {
            core: None,
            git: None,
            cache: None,
            file: None,
            pipeline: None,
            check_ignore: None,
        };

        let prefix = "XVC_";
        for (key, value) in std::env::vars() {
            if !key.starts_with(prefix) {
                continue;
            }

            let key_upper = key[prefix.len()..].to_string();
            match key_upper.as_str() {
                // core
                "CORE_XVC_REPO_VERSION" => {
                    if let Ok(val) = value.parse::<u8>() {
                        config
                            .core
                            .get_or_insert_with(Default::default)
                            .xvc_repo_version = Some(val);
                    }
                }
                "CORE_VERBOSITY" => {
                    config.core.get_or_insert_with(Default::default).verbosity = Some(value);
                }
                // git
                "GIT_USE_GIT" => {
                    if let Some(val) = Self::parse_bool(&value) {
                        config.git.get_or_insert_with(Default::default).use_git = Some(val);
                    }
                }
                "GIT_COMMAND" => {
                    config.git.get_or_insert_with(Default::default).command = Some(value);
                }
                "GIT_AUTO_COMMIT" => {
                    if let Some(val) = Self::parse_bool(&value) {
                        config.git.get_or_insert_with(Default::default).auto_commit = Some(val);
                    }
                }
                "GIT_AUTO_STAGE" => {
                    if let Some(val) = Self::parse_bool(&value) {
                        config.git.get_or_insert_with(Default::default).auto_stage = Some(val);
                    }
                }
                // cache
                "CACHE_ALGORITHM" => {
                    config.cache.get_or_insert_with(Default::default).algorithm = Some(value);
                }
                // file.track
                "FILE_TRACK_NO_COMMIT" => {
                    if let Some(val) = Self::parse_bool(&value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .track
                            .get_or_insert_with(Default::default)
                            .no_commit = Some(val);
                    }
                }
                "FILE_TRACK_FORCE" => {
                    if let Some(val) = Self::parse_bool(&value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .track
                            .get_or_insert_with(Default::default)
                            .force = Some(val);
                    }
                }
                "FILE_TRACK_TEXT_OR_BINARY" => {
                    config
                        .file
                        .get_or_insert_with(Default::default)
                        .track
                        .get_or_insert_with(Default::default)
                        .text_or_binary = Some(value);
                }
                "FILE_TRACK_NO_PARALLEL" => {
                    if let Some(val) = Self::parse_bool(&value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .track
                            .get_or_insert_with(Default::default)
                            .no_parallel = Some(val);
                    }
                }
                "FILE_TRACK_INCLUDE_GIT_FILES" => {
                    if let Some(val) = Self::parse_bool(&value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .track
                            .get_or_insert_with(Default::default)
                            .include_git_files = Some(val);
                    }
                }
                // file.list
                "FILE_LIST_FORMAT" => {
                    config
                        .file
                        .get_or_insert_with(Default::default)
                        .list
                        .get_or_insert_with(Default::default)
                        .format = Some(value);
                }
                "FILE_LIST_SORT" => {
                    config
                        .file
                        .get_or_insert_with(Default::default)
                        .list
                        .get_or_insert_with(Default::default)
                        .sort = Some(value);
                }
                "FILE_LIST_SHOW_DOT_FILES" => {
                    if let Some(val) = Self::parse_bool(&value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .list
                            .get_or_insert_with(Default::default)
                            .show_dot_files = Some(val);
                    }
                }
                "FILE_LIST_NO_SUMMARY" => {
                    if let Some(val) = Self::parse_bool(&value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .list
                            .get_or_insert_with(Default::default)
                            .no_summary = Some(val);
                    }
                }
                "FILE_LIST_RECURSIVE" => {
                    if let Some(val) = Self::parse_bool(&value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .list
                            .get_or_insert_with(Default::default)
                            .recursive = Some(val);
                    }
                }
                "FILE_LIST_INCLUDE_GIT_FILES" => {
                    if let Some(val) = Self::parse_bool(&value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .list
                            .get_or_insert_with(Default::default)
                            .include_git_files = Some(val);
                    }
                }
                // file.carry_in
                "FILE_CARRY_IN_FORCE" => {
                    if let Some(val) = Self::parse_bool(&value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .carry_in
                            .get_or_insert_with(Default::default)
                            .force = Some(val);
                    }
                }
                "FILE_CARRY_IN_NO_PARALLEL" => {
                    if let Some(val) = Self::parse_bool(&value) {
                        config
                            .file
                            .get_or_insert_with(Default::default)
                            .carry_in
                            .get_or_insert_with(Default::default)
                            .no_parallel = Some(val);
                    }
                }
                // file.recheck
                "FILE_RECHECK_METHOD" => {
                    config
                        .file
                        .get_or_insert_with(Default::default)
                        .recheck
                        .get_or_insert_with(Default::default)
                        .method = Some(value);
                }
                // pipeline
                "PIPELINE_CURRENT_PIPELINE" => {
                    config
                        .pipeline
                        .get_or_insert_with(Default::default)
                        .current_pipeline = Some(value);
                }
                "PIPELINE_DEFAULT" => {
                    config.pipeline.get_or_insert_with(Default::default).default = Some(value);
                }
                "PIPELINE_DEFAULT_PARAMS_FILE" => {
                    config
                        .pipeline
                        .get_or_insert_with(Default::default)
                        .default_params_file = Some(value);
                }
                "PIPELINE_PROCESS_POOL_SIZE" => {
                    if let Ok(val) = value.parse::<u32>() {
                        config
                            .pipeline
                            .get_or_insert_with(Default::default)
                            .process_pool_size = Some(val);
                    }
                }
                // check_ignore
                "CHECK_IGNORE_DETAILS" => {
                    if let Some(val) = Self::parse_bool(&value) {
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
    /// Create a new blank config params
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

    /// Update include_system_config value
    pub fn include_system_config(mut self, include_system_config: bool) -> Self {
        self.include_system_config = include_system_config;
        self
    }

    /// Update include_user_config value
    pub fn include_user_config(mut self, include_user_config: bool) -> Self {
        self.include_user_config = include_user_config;
        self
    }

    /// Update project config path
    pub fn project_config_path(mut self, project_config_path: Option<AbsolutePath>) -> Self {
        self.project_config_path = project_config_path;
        self
    }

    /// Update local config path
    pub fn local_config_path(mut self, local_config_path: Option<AbsolutePath>) -> Self {
        self.local_config_path = local_config_path;
        self
    }

    /// Whether to include enviroment variables in the configuration
    pub fn include_environment_config(mut self, include_environment_config: bool) -> Self {
        self.include_environment_config = include_environment_config;
        self
    }

    /// Command line config from key=value definitions
    pub fn command_line_config(mut self, command_line_config: Option<Vec<String>>) -> Self {
        self.command_line_config = command_line_config;
        self
    }
}

/// Returns the default configuration that can be modified with system, user,
/// repository configurations
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

/// Returns the default configuration that can be modified with system, user,
/// repository configurations
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

pub fn initial_xvc_config(
    default_config: XvcConfiguration,
    user_options: XvcOptionalConfiguration,
) -> Result<String> {
    let mut config = default_config;
    if let Some(user_core) = user_options.core {
        if let Some(verbosity) = user_core.verbosity {
            config.core.verbosity = verbosity;
        }
    }
    if let Some(user_git) = user_options.git {
        if let Some(use_git) = user_git.use_git {
            config.git.use_git = use_git;
        }
        if let Some(command) = user_git.command {
            config.git.command = command;
        }
        if let Some(auto_commit) = user_git.auto_commit {
            config.git.auto_commit = auto_commit;
        }
        if let Some(auto_stage) = user_git.auto_stage {
            config.git.auto_stage = auto_stage;
        }
    }
    if let Some(user_cache) = user_options.cache {
        if let Some(algorithm) = user_cache.algorithm {
            config.cache.algorithm = algorithm;
        }
    }
    if let Some(user_file) = user_options.file {
        if let Some(user_track) = user_file.track {
            if let Some(no_commit) = user_track.no_commit {
                config.file.track.no_commit = no_commit;
            }
            if let Some(force) = user_track.force {
                config.file.track.force = force;
            }
            if let Some(text_or_binary) = user_track.text_or_binary {
                config.file.track.text_or_binary = text_or_binary;
            }
            if let Some(no_parallel) = user_track.no_parallel {
                config.file.track.no_parallel = no_parallel;
            }
            if let Some(include_git_files) = user_track.include_git_files {
                config.file.track.include_git_files = include_git_files;
            }
        }
        if let Some(user_list) = user_file.list {
            if let Some(format) = user_list.format {
                config.file.list.format = format;
            }
            if let Some(sort) = user_list.sort {
                config.file.list.sort = sort;
            }
            if let Some(show_dot_files) = user_list.show_dot_files {
                config.file.list.show_dot_files = show_dot_files;
            }
            if let Some(no_summary) = user_list.no_summary {
                config.file.list.no_summary = no_summary;
            }
            if let Some(recursive) = user_list.recursive {
                config.file.list.recursive = recursive;
            }
            if let Some(include_git_files) = user_list.include_git_files {
                config.file.list.include_git_files = include_git_files;
            }
        }
        if let Some(user_carry_in) = user_file.carry_in {
            if let Some(force) = user_carry_in.force {
                config.file.carry_in.force = force;
            }
            if let Some(no_parallel) = user_carry_in.no_parallel {
                config.file.carry_in.no_parallel = no_parallel;
            }
        }
        if let Some(user_recheck) = user_file.recheck {
            if let Some(method) = user_recheck.method {
                config.file.recheck.method = method;
            }
        }
    }
    if let Some(user_pipeline) = user_options.pipeline {
        if let Some(current_pipeline) = user_pipeline.current_pipeline {
            config.pipeline.current_pipeline = current_pipeline;
        }
        if let Some(default) = user_pipeline.default {
            config.pipeline.default = default;
        }
        if let Some(default_params_file) = user_pipeline.default_params_file {
            config.pipeline.default_params_file = default_params_file;
        }
        if let Some(process_pool_size) = user_pipeline.process_pool_size {
            config.pipeline.process_pool_size = process_pool_size;
        }
    }
    if let Some(user_check_ignore) = user_options.check_ignore {
        if let Some(details) = user_check_ignore.details {
            config.check_ignore.details = details;
        }
    }
    Ok(format!(
        r##"
[core]
xvc_repo_version = "{xvc_repo_version}"
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
#   MB, GB and TB to represent sizes larged than 1MB.
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

/// Returns a blank optional configuration with all fields set to None.
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
    pub fn from_file(file_path: &Path) -> Result<Self> {
        let s =
            std::fs::read_to_string(&file_path).map_err(|e| crate::Error::IoError { source: e })?;
        let c: XvcConfiguration =
            toml::from_str(&s).map_err(|e| crate::Error::TomlDeserializationError { source: e })?;
        Ok(c)
    }

    pub fn merge_with_optional(&self, opt_config: XvcOptionalConfiguration) -> Self {
        merge_configs(&self, &opt_config)
    }
}
