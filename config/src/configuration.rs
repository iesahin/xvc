use crate::Result;
use serde::{Deserialize, Serialize};
use xvc_walker::AbsolutePath;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreConfig {
    pub xvc_repo_version: u8,
    pub verbosity: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GitConfig {
    pub use_git: bool,
    pub command: String,
    pub auto_commit: bool,
    pub auto_stage: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CacheConfig {
    pub algorithm: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileTrackConfig {
    pub no_commit: bool,
    pub force: bool,
    pub text_or_binary: String,
    pub no_parallel: bool,
    pub include_git_files: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileListConfig {
    pub format: String,
    pub sort: String,
    pub show_dot_files: bool,
    pub no_summary: bool,
    pub recursive: bool,
    pub include_git_files: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileCarryInConfig {
    pub force: bool,
    pub no_parallel: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileRecheckConfig {
    pub method: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileConfig {
    pub track: FileTrackConfig,
    pub list: FileListConfig,
    #[serde(rename = "carry-in")]
    pub carry_in: FileCarryInConfig,
    pub recheck: FileRecheckConfig,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineConfig {
    pub current_pipeline: String,
    pub default: String,
    pub default_params_file: String,
    pub process_pool_size: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckIgnoreConfig {
    pub details: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalCoreConfig {
    pub xvc_repo_version: Option<u8>,
    pub verbosity: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalGitConfig {
    pub use_git: Option<bool>,
    pub command: Option<String>,
    pub auto_commit: Option<bool>,
    pub auto_stage: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalCacheConfig {
    pub algorithm: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalFileTrackConfig {
    pub no_commit: Option<bool>,
    pub force: Option<bool>,
    pub text_or_binary: Option<String>,
    pub no_parallel: Option<bool>,
    pub include_git_files: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalFileListConfig {
    pub format: Option<String>,
    pub sort: Option<String>,
    pub show_dot_files: Option<bool>,
    pub no_summary: Option<bool>,
    pub recursive: Option<bool>,
    pub include_git_files: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalFileCarryInConfig {
    pub force: Option<bool>,
    pub no_parallel: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalFileRecheckConfig {
    pub method: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalFileConfig {
    pub track: Option<OptionalFileTrackConfig>,
    pub list: Option<OptionalFileListConfig>,
    #[serde(rename = "carry-in")]
    pub carry_in: Option<OptionalFileCarryInConfig>,
    pub recheck: Option<OptionalFileRecheckConfig>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalPipelineConfig {
    pub current_pipeline: Option<String>,
    pub default: Option<String>,
    pub default_params_file: Option<String>,
    pub process_pool_size: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalCheckIgnoreConfig {
    pub details: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct XvcOptionalConfiguration {
    pub core: Option<OptionalCoreConfig>,
    pub git: Option<OptionalGitConfig>,
    pub cache: Option<OptionalCacheConfig>,
    pub file: Option<OptionalFileConfig>,
    pub pipeline: Option<OptionalPipelineConfig>,
    #[serde(rename = "check-ignore")]
    pub check_ignore: Option<OptionalCheckIgnoreConfig>,
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
    optional_config: &XvcOptionalConfiguration,
) -> XvcConfiguration {
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
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = "{guid}"
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
        guid = config.core.guid,
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
