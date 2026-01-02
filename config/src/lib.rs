//! Provides a general solution to maintain configuration spanned across different sources.
//!
//!
//! - Default Values
//! - System configuration
//! - User configuration
//! - Public project configuration (tracked by Git)
//! - Private (local) project configuration (not tracked by Git)
//! - Environment variables
//! - Command line options
//!
//!
//! The configuration keys are string.
//! Configuration values can be:
//! - string
//! - bool
//! - int
//! - float
//!
//! Configuration files are in TOML.
//!
//! Options can be nested like `group.name = value`.
//!
//! Each option can be tracked to its source via [XvcConfigOption].
//!
#![warn(missing_docs)]
#![forbid(unsafe_code)]
pub mod config_params;
pub mod configuration;
pub mod error;

pub use config_params::XvcLoadParams;
pub use configuration::blank_optional_config;
pub use configuration::default_config;
pub use configuration::initial_xvc_configuration_file;
pub use configuration::XvcConfiguration;
pub use configuration::XvcOptionalConfiguration;

use directories_next::{BaseDirs, ProjectDirs, UserDirs};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
    str::FromStr,
};
use xvc_walker::AbsolutePath;

use strum_macros::{Display as EnumDisplay, EnumString, IntoStaticStr};

use crate::error::{Error, Result};
use toml::Value as TomlValue;

lazy_static! {
    /// System specific configuration directory.
    /// see [directories_next::ProjectDirs].
    pub static ref SYSTEM_CONFIG_DIRS: Option<ProjectDirs> =
        ProjectDirs::from("com", "emresult", "xvc");

    /// User configuration directories.
    /// See [directories_next::BaseDirs].
    pub static ref USER_CONFIG_DIRS: Option<BaseDirs> = BaseDirs::new();

    /// User directories.
    /// see [directories_next::UserDirs].
    pub static ref USER_DIRS: Option<UserDirs> = UserDirs::new();
}

/// Define the source where an option is obtained
#[derive(
    Debug, Copy, Clone, EnumString, EnumDisplay, IntoStaticStr, Serialize, Deserialize, PartialEq,
)]
#[strum(serialize_all = "lowercase")]
pub enum XvcConfigOptionSource {
    /// Default value defined in source code
    Default,
    /// System-wide configuration value from [SYSTEM_CONFIG_DIRS]
    System,
    /// User's configuration value from [USER_CONFIG_DIRS]
    Global,
    /// Project specific configuration that can be shared
    Project,
    /// Project specific configuration that's not meant to be shared (personal/local)
    Local,
    /// Options obtained from the command line
    CommandLine,
    /// Options from environment variables
    Environment,
    /// Options set while running the software, automatically.
    Runtime,
}

/// The option and its source.
#[derive(Debug, Copy, Clone)]
pub struct XvcConfigOption<T> {
    /// Where did we get this option?
    pub source: XvcConfigOptionSource,
    /// The key and value
    pub option: T,
}

/// Verbosity levels for Xvc CLI
#[derive(Debug, Copy, Clone, EnumString, EnumDisplay, IntoStaticStr)]
pub enum XvcVerbosity {
    /// Do not print anything
    #[strum(serialize = "quiet", serialize = "0")]
    Quiet,
    /// Print default output and errors
    #[strum(serialize = "default", serialize = "error", serialize = "1")]
    Default,
    /// Print default output, warnings and errors
    #[strum(serialize = "warn", serialize = "2")]
    Warn,
    /// Print default output, info, warnings and errors
    #[strum(serialize = "info", serialize = "3")]
    Info,
    /// Print default output, errors, warnings, info and debug output
    #[strum(serialize = "debug", serialize = "4")]
    Debug,
    /// Print default output, errors, warnings, info, debug and tracing output
    #[strum(serialize = "trace", serialize = "5")]
    Trace,
}

impl From<u8> for XvcVerbosity {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Quiet,
            1 => Self::Default,
            2 => Self::Warn,
            3 => Self::Info,
            4 => Self::Debug,
            _ => Self::Trace,
        }
    }
}

/// A configuration value with its source
#[derive(Debug, Clone)]
pub struct XvcConfigValue {
    /// Where did we get this value?
    pub source: XvcConfigOptionSource,
    /// The value itself
    pub value: TomlValue,
}

impl XvcConfigValue {
    /// Create a new XvcConfigValue
    pub fn new(source: XvcConfigOptionSource, value: TomlValue) -> Self {
        Self { source, value }
    }
}

/// A set of options defined as a TOML document from a single [XvcConfigOptionSource]
#[derive(Debug, Clone)]
pub struct XvcConfigMap {
    /// Where does these option come from?
    pub source: XvcConfigOptionSource,
    /// The key-value map for the options
    pub map: HashMap<String, TomlValue>,
}

/// Keeps track of all Xvc configuration.
///
/// It's created by [XvcRoot] using the options from [XvcConfigInitParams].
/// Keeps the current directory, that can also be set manually from the command line.
/// It loads several config maps (one for each [XvcConfigOptionSource]) and cascadingly merges them to get an actual configuration.
#[derive(Debug, Clone)]
pub struct XvcConfig {
    /// Current directory. It can be set with xvc -C option
    pub current_dir: AbsolutePath,
    /// System configuration from the system directories
    system_config: XvcOptionalConfiguration,
    /// User's configuration value from [USER_CONFIG_DIRS]
    user_config: XvcOptionalConfiguration,
    /// Project specific configuration that can be shared
    project_config: XvcOptionalConfiguration,
    /// Project specific configuration that's not meant to be shared (personal/local)
    local_config: XvcOptionalConfiguration,
    /// Options obtained from the command line
    command_line_config: XvcOptionalConfiguration,
    /// Options from environment variables
    environment_config: XvcOptionalConfiguration,
    /// Options set while running the software, automatically.
    runtime_config: XvcOptionalConfiguration,
    /// The current configuration map, updated cascadingly
    the_config: XvcConfiguration,
}

impl fmt::Display for XvcConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\nCurrent Configuration")?;
        writeln!(f, "current_dir: {}", self.current_dir)?;
        writeln!(f, "{}", &self.the_config)?;
        writeln!(f)
    }
}

impl XvcConfig {
    /// Loads the default configuration from `p`.
    ///
    /// The configuration must be a valid TOML document.
    pub fn new_v2(config_init_params: &XvcLoadParams) -> Result<Self> {
        let default_conf = default_config();

        let system_config = if config_init_params.include_system_config {
            Self::system_config_file()
                .and_then(|path| Self::load_optional_config_from_file(&path))
                .unwrap_or(blank_optional_config())
        } else {
            blank_optional_config()
        };

        let user_config = if config_init_params.include_user_config {
            Self::user_config_file()
                .and_then(|path| Self::load_optional_config_from_file(&path))
                .unwrap_or(blank_optional_config())
        } else {
            blank_optional_config()
        };

        let project_config = if config_init_params.include_project_config {
            if let Some(ref config_path) = config_init_params.project_config_path {
                Self::load_optional_config_from_file(config_path)?
            } else {
                blank_optional_config()
            }
        } else {
            blank_optional_config()
        };

        let local_config = if config_init_params.include_local_config {
            if let Some(ref config_path) = config_init_params.local_config_path {
                Self::load_optional_config_from_file(config_path)?
            } else {
                blank_optional_config()
            }
        } else {
            blank_optional_config()
        };

        let environment_config = if config_init_params.include_environment_config {
            XvcOptionalConfiguration::from_env()
        } else {
            blank_optional_config()
        };

        let command_line_config =
            Self::load_command_line_config(&config_init_params.command_line_config)?;

        let runtime_config = blank_optional_config();

        let the_config = default_conf
            .merge_with_optional(&system_config)
            .merge_with_optional(&user_config)
            .merge_with_optional(&project_config)
            .merge_with_optional(&local_config)
            .merge_with_optional(&environment_config)
            .merge_with_optional(&command_line_config)
            .merge_with_optional(&runtime_config);

        Ok(XvcConfig {
            current_dir: config_init_params.current_dir.clone(),
            system_config,
            user_config,
            project_config,
            local_config,
            command_line_config,
            environment_config,
            runtime_config,
            the_config,
        })
    }

    /// Return the current configuration
    pub fn config(&self) -> &XvcConfiguration {
        &self.the_config
    }

    /// Return the system configuration file path for Xvc
    /// FIXME: Return Absolute Path
    pub fn system_config_file() -> Result<PathBuf> {
        Ok(SYSTEM_CONFIG_DIRS
            .to_owned()
            .ok_or(Error::CannotDetermineSystemConfigurationPath)?
            .config_dir()
            .to_path_buf())
    }

    /// Return the user configuration file path for Xvc
    pub fn user_config_file() -> Result<PathBuf> {
        Ok(USER_CONFIG_DIRS
            .to_owned()
            .ok_or(Error::CannotDetermineUserConfigurationPath)?
            .config_dir()
            .join("xvc"))
    }

    /// Load an [XvcOptionalConfiguration] from a file or returns a blank config if the file is not found
    pub fn load_optional_config_from_file(path: &Path) -> Result<XvcOptionalConfiguration> {
        if path.exists() {
            let opt_config = XvcOptionalConfiguration::from_file(path)?;
            Ok(opt_config)
        } else {
            Ok(blank_optional_config())
        }
    }

    /// Loads configuration from command line arguments.
    /// Parses a vector of key-value strings into an [XvcOptionalConfiguration].
    pub fn load_command_line_config(
        cli_opt_vector: &Option<Vec<String>>,
    ) -> Result<XvcOptionalConfiguration> {
        if let Some(cli_opts) = cli_opt_vector {
            let cli_opts_hm = Self::parse_key_value_vector(cli_opts);
            Ok(XvcOptionalConfiguration::from_hash_map("", &cli_opts_hm))
        } else {
            Ok(XvcOptionalConfiguration::default())
        }
    }

    /// Parses a vector of strings, and returns a `HashMap<String, String>`.
    fn parse_key_value_vector(cli_opts: &Vec<String>) -> HashMap<String, String> {
        cli_opts
            .into_iter()
            .map(|str| {
                let elements: Vec<&str> = str.split('=').collect();
                let key = elements[0].trim().to_owned();
                let value = elements[1].trim().to_owned();
                (key, value)
            })
            .collect()
    }

    /// Where do we run the command?
    ///
    /// This can be modified by options in the command line, so it's not always equal to [std::env::current_dir()]
    pub fn current_dir(&self) -> Result<&AbsolutePath> {
        let pb = &self.current_dir;
        Ok(pb)
    }

    /// The current verbosity level.
    /// Set with `core.verbosity` option.
    pub fn verbosity(&self) -> XvcVerbosity {
        let verbosity_str = &self.the_config.core.verbosity;
        match XvcVerbosity::from_str(verbosity_str) {
            Ok(v) => v,
            Err(source) => {
                Error::StrumError { source }.warn();
                XvcVerbosity::Default
            }
        }
    }

    /// Find where a configuration value is defined, by checking configuration layers from highest priority to lowest.
    pub fn find_value_source(&self, key: &str) -> Option<XvcConfigOptionSource> {
        let layers = [
            (XvcConfigOptionSource::Runtime, &self.runtime_config),
            (
                XvcConfigOptionSource::CommandLine,
                &self.command_line_config,
            ),
            (XvcConfigOptionSource::Environment, &self.environment_config),
            (XvcConfigOptionSource::Local, &self.local_config),
            (XvcConfigOptionSource::Project, &self.project_config),
            (XvcConfigOptionSource::Global, &self.user_config), // enum variant is Global
            (XvcConfigOptionSource::System, &self.system_config),
        ];

        for (source, config) in &layers {
            if self.key_exists_in_optional_config(config, key) {
                return Some(*source);
            }
        }

        if self.is_valid_key(key) {
            Some(XvcConfigOptionSource::Default)
        } else {
            None
        }
    }

    /// Helper function to check if a specific key exists and has a `Some` value within an [XvcOptionalConfiguration] instance.
    /// It traverses the optional configuration structure based on the `key`'s dot-separated parts.
    fn key_exists_in_optional_config(&self, config: &XvcOptionalConfiguration, key: &str) -> bool {
        let parts: Vec<&str> = key.split('.').collect();
        match parts.as_slice() {
            // core
            ["core", "xvc_repo_version"] => config
                .core
                .as_ref()
                .is_some_and(|c| c.xvc_repo_version.is_some()),
            ["core", "verbosity"] => config.core.as_ref().is_some_and(|c| c.verbosity.is_some()),
            // git
            ["git", "use_git"] => config.git.as_ref().is_some_and(|c| c.use_git.is_some()),
            ["git", "command"] => config.git.as_ref().is_some_and(|c| c.command.is_some()),
            ["git", "auto_commit"] => config.git.as_ref().is_some_and(|c| c.auto_commit.is_some()),
            ["git", "auto_stage"] => config.git.as_ref().is_some_and(|c| c.auto_stage.is_some()),
            // cache
            ["cache", "algorithm"] => config.cache.as_ref().is_some_and(|c| c.algorithm.is_some()),
            // file.track
            ["file", "track", "no_commit"] => config
                .file
                .as_ref()
                .and_then(|f| f.track.as_ref())
                .is_some_and(|t| t.no_commit.is_some()),
            ["file", "track", "force"] => config
                .file
                .as_ref()
                .and_then(|f| f.track.as_ref())
                .is_some_and(|t| t.force.is_some()),
            ["file", "track", "text_or_binary"] => config
                .file
                .as_ref()
                .and_then(|f| f.track.as_ref())
                .is_some_and(|t| t.text_or_binary.is_some()),
            ["file", "track", "no_parallel"] => config
                .file
                .as_ref()
                .and_then(|f| f.track.as_ref())
                .is_some_and(|t| t.no_parallel.is_some()),
            ["file", "track", "include_git_files"] => config
                .file
                .as_ref()
                .and_then(|f| f.track.as_ref())
                .is_some_and(|t| t.include_git_files.is_some()),
            // file.list
            ["file", "list", "format"] => config
                .file
                .as_ref()
                .and_then(|f| f.list.as_ref())
                .is_some_and(|l| l.format.is_some()),
            ["file", "list", "sort"] => config
                .file
                .as_ref()
                .and_then(|f| f.list.as_ref())
                .is_some_and(|l| l.sort.is_some()),
            ["file", "list", "show_dot_files"] => config
                .file
                .as_ref()
                .and_then(|f| f.list.as_ref())
                .is_some_and(|l| l.show_dot_files.is_some()),
            ["file", "list", "no_summary"] => config
                .file
                .as_ref()
                .and_then(|f| f.list.as_ref())
                .is_some_and(|l| l.no_summary.is_some()),
            ["file", "list", "recursive"] => config
                .file
                .as_ref()
                .and_then(|f| f.list.as_ref())
                .is_some_and(|l| l.recursive.is_some()),
            ["file", "list", "include_git_files"] => config
                .file
                .as_ref()
                .and_then(|f| f.list.as_ref())
                .is_some_and(|l| l.include_git_files.is_some()),
            // file.carry-in
            ["file", "carry-in", "force"] => config
                .file
                .as_ref()
                .and_then(|f| f.carry_in.as_ref())
                .is_some_and(|c| c.force.is_some()),
            ["file", "carry-in", "no_parallel"] => config
                .file
                .as_ref()
                .and_then(|f| f.carry_in.as_ref())
                .is_some_and(|c| c.no_parallel.is_some()),
            // file.recheck
            ["file", "recheck", "method"] => config
                .file
                .as_ref()
                .and_then(|f| f.recheck.as_ref())
                .is_some_and(|r| r.method.is_some()),
            // pipeline
            ["pipeline", "current_pipeline"] => config
                .pipeline
                .as_ref()
                .is_some_and(|p| p.current_pipeline.is_some()),
            ["pipeline", "default"] => config
                .pipeline
                .as_ref()
                .is_some_and(|p| p.default.is_some()),
            ["pipeline", "default_params_file"] => config
                .pipeline
                .as_ref()
                .is_some_and(|p| p.default_params_file.is_some()),
            ["pipeline", "process_pool_size"] => config
                .pipeline
                .as_ref()
                .is_some_and(|p| p.process_pool_size.is_some()),
            // check-ignore
            ["check-ignore", "details"] => config
                .check_ignore
                .as_ref()
                .is_some_and(|c| c.details.is_some()),
            _ => false,
        }
    }

    /// Checks if a given key string (e.g., "core.verbosity") corresponds to a valid, known configuration path.
    /// This ensures that only recognized keys are processed or reported as having a default source.
    fn is_valid_key(&self, key: &str) -> bool {
        let parts: Vec<&str> = key.split('.').collect();
        matches!(
            parts.as_slice(),
            // core
            ["core", "xvc_repo_version"] |
            ["core", "verbosity"] |
            // git
            ["git", "use_git"] |
            ["git", "command"] |
            ["git", "auto_commit"] |
            ["git", "auto_stage"] |
            // cache
            ["cache", "algorithm"] |
            // file.track
            ["file", "track", "no_commit"] |
            ["file", "track", "force"] |
            ["file", "track", "text_or_binary"] |
            ["file", "track", "no_parallel"] |
            ["file", "track", "include_git_files"] |
            // file.list
            ["file", "list", "format"] |
            ["file", "list", "sort"] |
            ["file", "list", "show_dot_files"] |
            ["file", "list", "no_summary"] |
            ["file", "list", "recursive"] |
            ["file", "list", "include_git_files"] |
            // file.carry-in
            ["file", "carry-in", "force"] |
            ["file", "carry-in", "no_parallel"] |
            // file.recheck
            ["file", "recheck", "method"] |
            // pipeline
            ["pipeline", "current_pipeline"] |
            ["pipeline", "default"] |
            ["pipeline", "default_params_file"] |
            ["pipeline", "process_pool_size"] |
            // check-ignore
            ["check-ignore", "details"]
        )
    }
}

/// Trait to update CLI options with defaults from configuration.
///
/// When a CLI struct like [xvc_pipeline::PipelineCLI] implements this trait, it reads the configuration and updates values not set in the command line accordingly.
pub trait FromConfig {
    /// Update the implementing struct from the configuration.
    /// Reading the relevant keys and values of the config is in implementor's responsibility.
    ///
    /// This is used to abstract away CLI structs and crate options.
    fn from_config(conf: &XvcConfiguration) -> Result<Box<Self>>;
}

/// A trait for updating an existing configuration struct with values from a complete `XvcConfiguration`.
///
/// This allows for merging configuration values into an already instantiated struct,
/// often used for CLI options where some fields might already be set by the user.
pub trait UpdateFromConfig {
    /// Updates the implementing struct with values from the provided `XvcConfiguration`.
    ///
    /// The implementation is responsible for reading the relevant keys and values
    /// from `conf` and applying them to `self`.
    ///
    /// # Arguments
    ///
    /// * `self` - The instance of the struct to be updated.
    /// * `conf` - A reference to the `XvcConfiguration` containing the values to apply.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Box`ed instance of the updated struct if successful, or an error.
    fn update_from_config(self, conf: &XvcConfiguration) -> Result<Box<Self>>;
}
