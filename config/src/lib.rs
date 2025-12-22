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

use directories_next::{BaseDirs, ProjectDirs, UserDirs};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt, fs,
    path::{Path, PathBuf},
    str::FromStr,
};
use xvc_logging::{debug, watch};
use xvc_walker::AbsolutePath;

use strum_macros::{Display as EnumDisplay, EnumString, IntoStaticStr};

use crate::{
    configuration::{
        blank_optional_config, default_config, XvcConfiguration, XvcOptionalConfiguration,
    },
    error::{Error, Result},
};
use toml::Value as TomlValue;

fn load_environment_config() -> XvcOptionalConfiguration {
    XvcOptionalConfiguration::from_env()
}
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
#[derive(Debug, Copy, Clone, EnumString, EnumDisplay, IntoStaticStr, Serialize, Deserialize)]
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
    /// Default configuration we set in the executable
    default_config: XvcConfiguration,
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
    pub the_config: XvcConfiguration,
    /// The init params used to create this config
    pub load_params: XvcLoadParams,
}

impl fmt::Display for XvcConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\nCurrent Configuration")?;
        writeln!(f, "current_dir: {}", self.current_dir)?;
        writeln!(f, "{}", &self.the_config);
        writeln!(f)
    }
}

impl XvcConfig {
    /// Loads the default configuration from `p`.
    ///
    /// The configuration must be a valid TOML document.
    fn new(config_init_params: &XvcLoadParams) -> Result<Self> {
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
            if let Some(config_path) = config_init_params.project_config_path {
                Self::load_optional_config_from_file(&config_path)?
            } else {
                blank_optional_config()
            }
        } else {
            blank_optional_config()
        };

        let local_config = if config_init_params.include_local_config {
            if let Some(config_path) = config_init_params.local_config_path {
                Self::load_optional_config_from_file(&config_path)?
            } else {
                blank_optional_config()
            }
        } else {
            blank_optional_config()
        };

        let environment_config = if config_init_params.include_environment_config {
            load_environment_config()
        } else {
            blank_optional_config()
        };

        let command_line_config = load_command_line_config(&config_init_params.command_line_config);

        let runtime_config = blank_optional_config();

        let the_config = default_conf
            .merge_with_optional(system_config)
            .merge_with_optional(user_config)
            .merge_with_optional(project_config)
            .merge_with_optional(local_config)
            .merge_with_optional(environment_config)
            .merge_with_optional(command_line_config)
            .merge_with_optional(runtime_config);

        Ok(XvcConfig {
            current_dir: config_init_params.current_dir,
            default_config: default_conf,
            system_config,
            user_config,
            project_config,
            local_config,
            command_line_config,
            environment_config,
            runtime_config,
            the_config,
            load_params: config_init_params.clone(),
        })
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

    /// Load all keys from the environment that starts with `XVC_` and build a hash map with them.
    ///
    /// The resulting hash map has `key: value` elements for environment variables in the form `XVC_key=value`.
    fn env_map() -> Result<HashMap<String, TomlValue>> {
        let mut hm = HashMap::<String, String>::new();
        let env_key_re = Regex::new(r"^XVC_?(.+)")?;
        for (k, v) in std::env::vars() {
            if let Some(cap) = env_key_re.captures(&k) {
                hm.insert(cap[1].to_owned(), v);
            }
        }

        // Try to parse the values:
        // bool -> i64 -> f64 -> String

        let hm_val = hm
            .into_iter()
            .map(|(k, v)| (k, Self::parse_to_value(v)))
            .collect();

        Ok(hm_val)
    }

    /// Parses a string to most specific type that can represent it.
    ///
    /// The parsing order is
    ///
    /// bool -> int -> float -> string.
    ///
    /// If it's not parsed as bool, int is tried, then float.
    /// If none of these work, return it as String.
    /// This is used in [self.env_map] to get TOML values from environment variables.
    /// Other documents in TOML form are using native TOML parsing.
    fn parse_to_value(v: String) -> TomlValue {
        if let Ok(b) = v.parse::<bool>() {
            TomlValue::Boolean(b)
        } else if let Ok(i) = v.parse::<i64>() {
            TomlValue::Integer(i)
        } else if let Ok(f) = v.parse::<f64>() {
            TomlValue::Float(f)
        } else {
            TomlValue::String(v)
        }
    }

    /// Parses a vector of strings, and returns a `Vec<(key, value)>`.
    fn parse_key_value_vector(vector: Vec<String>) -> Vec<(String, TomlValue)> {
        vector
            .into_iter()
            .map(|str| {
                let elements: Vec<&str> = str.split('=').collect();
                let key = elements[0].trim().to_owned();
                let value = Self::parse_to_value(elements[1].trim().to_owned());
                (key, value)
            })
            .collect()
    }

    /// Loads all config files
    /// Overrides all options with the given key=value style options in the
    /// command line
    pub fn previous_new(p: XvcLoadParams) -> Result<XvcConfig> {
        watch!(p);
        let mut config = XvcConfig::new(&p);
        watch!(config);

        config.current_dir = XvcConfigOption {
            option: p.current_dir,
            source: XvcConfigOptionSource::Runtime,
        };

        let mut update = |source, file: std::result::Result<&Path, &Error>| match file {
            Ok(config_file) => match config.update_from_file(config_file, source) {
                Ok(new_config) => config = new_config,
                Err(err) => {
                    err.debug();
                }
            },
            Err(err) => {
                debug!("{}", err);
            }
        };

        if p.include_system_config {
            let f = Self::system_config_file();
            update(XvcConfigOptionSource::System, f.as_deref());
        }

        if p.include_user_config {
            update(
                XvcConfigOptionSource::Global,
                Self::user_config_file().as_deref(),
            );
        }

        if let Some(project_config_path) = p.project_config_path {
            update(XvcConfigOptionSource::Project, Ok(&project_config_path));
        }

        if let Some(local_config_path) = p.local_config_path {
            update(XvcConfigOptionSource::Local, Ok(&local_config_path));
        }

        if p.include_environment_config {
            let env_config = Self::env_map().unwrap();
            match config.update_from_hash_map(env_config, XvcConfigOptionSource::Environment) {
                Ok(conf) => config = conf,
                Err(err) => {
                    err.debug();
                }
            }
        }

        if let Some(cli_config) = p.command_line_config {
            let map: HashMap<String, TomlValue> = Self::parse_key_value_vector(cli_config)
                .into_iter()
                .collect();
            match config.update_from_hash_map(map, XvcConfigOptionSource::CommandLine) {
                Ok(conf) => {
                    config = conf;
                }
                Err(err) => {
                    err.debug();
                }
            }
        }

        watch!(&config);

        Ok(config)
    }

    pub fn load_command_line_config(cli_config: Option<Vec<String>>) {

        
    }

    /// Where do we run the command?
    ///
    /// This can be modified by options in the command line, so it's not always equal to [std::env::current_dir()]
    pub fn current_dir(&self) -> Result<&AbsolutePath> {
        let pb = &self.current_dir.option;
        Ok(pb)
    }

    /// Globally Unique Identified for the Xvc Repository / Project
    ///
    /// It's stored in `core.guid` option.
    /// It's created in [`XvcRoot::init`] and shouldn't be tampered with.
    /// Storage commands use this to create different paths for different Xvc projects.
    pub fn guid(&self) -> Option<String> {
        match self.get_str("core.guid") {
            Ok(opt) => Some(opt.option),
            Err(err) => {
                err.warn();
                None
            }
        }
    }

    /// The current verbosity level.
    /// Set with `core.verbosity` option.
    pub fn verbosity(&self) -> XvcVerbosity {
        let verbosity_str = self.get_str("core.verbosity");
        let verbosity_str = match verbosity_str {
            Ok(opt) => opt.option,
            Err(err) => {
                err.warn();
                "1".to_owned()
            }
        };

        match XvcVerbosity::from_str(&verbosity_str) {
            Ok(v) => v,
            Err(source) => {
                Error::StrumError { source }.warn();
                XvcVerbosity::Default
            }
        }
    }

    /// Returns a struct (`T`) value by using its `FromStr` implementation.
    /// It parses the string to get the value.
    pub fn get_val<T>(&self, key: &str) -> Result<T>
    where
        T: FromStr,
    {
        let str_val = self.get_str(key)?;
        let val: T = T::from_str(&str_val.option).map_err(|_| Error::EnumTypeConversionError {
            cause_key: key.to_owned(),
        })?;
        Ok(val)
    }
}

/// Trait to update CLI options with defaults from configuration.
///
/// When a CLI struct like [xvc_pipeline::PipelineCLI] implements this trait, it reads the configuration and updates values not set in the command line accordingly.
pub trait UpdateFromXvcConfig {
    /// Update the implementing struct from the configuration.
    /// Reading the relevant keys and values of the config is in implementor's responsibility.
    ///
    /// This is used to abstract away CLI structs and crate options.
    fn update_from_conf(self, conf: &XvcConfig) -> Result<Box<Self>>;
}

/// A struct implementing this trait can instantiate itself from XvcConfig.
///
/// When an option should be parsed and converted to a struct, it implements this trait.
/// The functions are basically identical, and uses [XvcConfig::get_val] to instantiate.
/// It's used to bind a configuration key (str) "group.key" with a struct.
///
/// See [conf] macro below for a shortcut.
pub trait FromConfigKey<T: FromStr> {
    /// Create a value of type `T` from configuration.
    /// Supposed to panic! if there is no key, or the value cannot be parsed.
    fn from_conf(conf: &XvcConfig) -> T;

    /// Try to create a type `T` from the configuration.
    /// Returns error if there is no key, or the value cannot be parsed.
    fn try_from_conf(conf: &XvcConfig) -> Result<T>;
}

/// Binds a type with a configuration key.
///
/// When you declare `conf!("group.subgroup.key", MyType)`, this macro writes the code necessary to create `MyType` from the configuration.
#[macro_export]
macro_rules! conf {
    ($type: ty, $key: literal) => {
        impl FromConfigKey<$type> for $type {
            fn from_conf(conf: &$crate::XvcConfig) -> $type {
                conf.get_val::<$type>($key).unwrap()
            }

            fn try_from_conf(conf: &$crate::XvcConfig) -> $crate::error::Result<$type> {
                conf.get_val::<$type>($key)
            }
        }
    };
}

/// Convert a TomlValue which can be a [TomlValue::Table] or any other simple type to a hash map with keys in the hierarchical form.
///
/// A `key` in TOML table `[group]` will have `group.key` in the returned hash map.
/// The groups can be arbitrarily deep.
pub fn toml_value_to_hashmap(key: String, value: TomlValue) -> HashMap<String, TomlValue> {
    let mut key_value_stack = Vec::<(String, TomlValue)>::new();
    let mut key_value_map = HashMap::<String, TomlValue>::new();
    key_value_stack.push((key, value));
    while let Some((key, value)) = key_value_stack.pop() {
        match value {
            TomlValue::Table(t) => {
                for (subkey, subvalue) in t {
                    if key.is_empty() {
                        key_value_stack.push((subkey, subvalue));
                    } else {
                        key_value_stack.push((format!("{}.{}", key, subkey), subvalue));
                    }
                }
            }
            _ => {
                key_value_map.insert(key, value);
            }
        }
    }
    key_value_map
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::error::Result;
    use log::LevelFilter;
    use toml::Value as TomlValue;
    use xvc_logging::setup_logging;

    pub fn test_logging(level: LevelFilter) {
        setup_logging(Some(level), Some(LevelFilter::Trace));
    }

    #[test]
    fn test_toml_value_to_hashmap() -> Result<()> {
        test_logging(LevelFilter::Trace);
        let str_value = "foo = 'bar'".parse::<TomlValue>()?;
        let str_hm = toml_value_to_hashmap("".to_owned(), str_value);

        assert!(str_hm["foo"] == TomlValue::String("bar".to_string()));

        let table_value = r#"[core]
        foo = "bar"
        val = 100
        "#
        .parse::<TomlValue>()?;

        let table_hm = toml_value_to_hashmap("".to_owned(), table_value);
        assert!(table_hm.get("core.foo") == Some(&TomlValue::String("bar".to_string())));
        Ok(())
    }
}
