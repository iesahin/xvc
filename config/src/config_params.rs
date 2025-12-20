use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreConfig {
    pub guid: String,
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
pub struct XvcConfig {
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
    pub guid: Option<String>,
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
pub struct XvcOptionalConfig {
    pub core: Option<OptionalCoreConfig>,
    pub git: Option<OptionalGitConfig>,
    pub cache: Option<OptionalCacheConfig>,
    pub file: Option<OptionalFileConfig>,
    pub pipeline: Option<OptionalPipelineConfig>,
    #[serde(rename = "check-ignore")]
    pub check_ignore: Option<OptionalCheckIgnoreConfig>,
}