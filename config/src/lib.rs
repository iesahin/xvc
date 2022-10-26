#![warn(missing_docs)]
#![forbid(unsafe_code)]
pub mod error;
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
use xvc_walker::AbsolutePath;

use strum_macros::{Display as EnumDisplay, EnumString, IntoStaticStr};

use crate::error::{Error, Result};
use toml::Value as TomlValue;
// pub const LOCAL_CONFIG_FILENAME: MetadataFileLocation =
//     MetadataFileLocation::DotXvc("config.local");
// pub const PROJECT_CONFIG_FILENAME: MetadataFileLocation = MetadataFileLocation::DotXvc("config");

lazy_static! {
    pub static ref SYSTEM_CONFIG_DIRS: Option<ProjectDirs> =
        ProjectDirs::from("com", "emresult", "xvc");
    pub static ref USER_CONFIG_DIRS: Option<BaseDirs> = BaseDirs::new();
    pub static ref USER_DIRS: Option<UserDirs> = UserDirs::new();
}

#[derive(Debug, Copy, Clone, EnumString, EnumDisplay, IntoStaticStr, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum XvcConfigOptionSource {
    Default,
    System,
    Global,
    Project,
    Local,
    CommandLine,
    Environment,
    Runtime,
}

#[derive(Debug, Copy, Clone)]
pub struct XvcConfigOption<T> {
    pub source: XvcConfigOptionSource,
    pub option: T,
}

#[derive(Debug, Copy, Clone, EnumString, EnumDisplay, IntoStaticStr)]
pub enum XvcVerbosity {
    #[strum(serialize = "quiet", serialize = "0")]
    Quiet,
    #[strum(serialize = "default", serialize = "1")]
    Default,
    #[strum(serialize = "warn", serialize = "2")]
    Warn,
    #[strum(serialize = "info", serialize = "3")]
    Info,
    #[strum(serialize = "debug", serialize = "4")]
    Debug,
    #[strum(serialize = "trace", serialize = "5")]
    Trace,
}

#[derive(Debug, Clone)]
pub struct XvcConfigValue {
    pub source: XvcConfigOptionSource,
    pub value: TomlValue,
}

impl XvcConfigValue {
    pub fn new(source: XvcConfigOptionSource, value: TomlValue) -> Self {
        Self { source, value }
    }
}

#[derive(Debug, Clone)]
pub struct XvcConfigMap {
    pub source: XvcConfigOptionSource,
    pub map: HashMap<String, TomlValue>,
}

#[derive(Debug, Clone)]
pub struct XvcConfigInitParams {
    pub default_configuration: String,
    pub current_dir: AbsolutePath,
    pub include_system_config: bool,
    pub include_user_config: bool,
    pub project_config_path: Option<PathBuf>,
    pub local_config_path: Option<PathBuf>,
    pub include_environment_config: bool,
    pub command_line_config: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct XvcConfig {
    /// Current directory. It can be set with xvc -C option
    pub current_dir: XvcConfigOption<AbsolutePath>,
    // /// The root if the command is happen to be run in XVC directory
    // pub xvc_root: XvcConfigOption<Option<XvcRoot>>,
    /// Configuration values for each level
    pub config_maps: Vec<XvcConfigMap>,
    /// The current configuration map, updated cascadingly
    pub the_config: HashMap<String, XvcConfigValue>,
    /// The init params used to create this config
    pub init_params: XvcConfigInitParams,
}

impl fmt::Display for XvcConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        writeln!(f, "\nCurrent Configuration")?;
        writeln!(
            f,
            "current_dir: {:?} ({:?})",
            self.current_dir.option, self.current_dir.source
        )?;
        for (k, v) in &self.the_config {
            writeln!(f, "{}: {} ({})", k, v.value, v.source)?;
        }
        writeln!(f)
    }
}

impl XvcConfig {
    fn default_conf(p: &XvcConfigInitParams) -> Self {
        let default_conf = p
            .default_configuration
            .parse::<TomlValue>()
            .expect("Error in default configuration!");
        let hm = toml_value_to_hashmap("".into(), default_conf);
        let hm_for_list = hm.clone();
        let the_config: HashMap<String, XvcConfigValue> = hm
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    XvcConfigValue {
                        source: XvcConfigOptionSource::Default,
                        value: v,
                    },
                )
            })
            .collect();

        XvcConfig {
            current_dir: XvcConfigOption {
                option: std::env::current_dir()
                    .expect("Cannot determine current directory")
                    .into(),
                source: XvcConfigOptionSource::Default,
            },
            the_config,
            config_maps: vec![XvcConfigMap {
                map: hm_for_list,
                source: XvcConfigOptionSource::Default,
            }],
            init_params: p.clone(),
        }
    }

    pub fn get_str(&self, key: &str) -> Result<XvcConfigOption<String>> {
        let opt = self.get_toml_value(key)?;
        if let TomlValue::String(val) = opt.option {
            Ok(XvcConfigOption::<String> {
                option: val,
                source: opt.source,
            })
        } else {
            Err(Error::MismatchedValueType { key: key.into() })
        }
    }

    pub fn get_bool(&self, key: &str) -> Result<XvcConfigOption<bool>> {
        let opt = self.get_toml_value(key)?;
        if let TomlValue::Boolean(val) = opt.option {
            Ok(XvcConfigOption::<bool> {
                option: val,
                source: opt.source,
            })
        } else {
            Err(Error::MismatchedValueType { key: key.into() })
        }
    }

    pub fn get_int(&self, key: &str) -> Result<XvcConfigOption<i64>> {
        let opt = self.get_toml_value(key)?;
        if let TomlValue::Integer(val) = opt.option {
            Ok(XvcConfigOption::<i64> {
                option: val,
                source: opt.source,
            })
        } else {
            Err(Error::MismatchedValueType { key: key.into() })
        }
    }

    pub fn get_float(&self, key: &str) -> Result<XvcConfigOption<f64>> {
        let opt = self.get_toml_value(key)?;
        if let TomlValue::Float(val) = opt.option {
            Ok(XvcConfigOption::<f64> {
                option: val,
                source: opt.source,
            })
        } else {
            Err(Error::MismatchedValueType { key: key.into() })
        }
    }

    pub fn get_toml_value(&self, key: &str) -> Result<XvcConfigOption<TomlValue>> {
        let value = self
            .the_config
            .get(key)
            .ok_or(Error::ConfigKeyNotFound { key: key.into() })?
            .to_owned();

        Ok(XvcConfigOption::<TomlValue> {
            option: value.value,
            source: value.source,
        })
    }

    fn update_from_hash_map(
        &self,
        new_map: HashMap<String, TomlValue>,
        new_source: XvcConfigOptionSource,
    ) -> Result<Self> {
        let mut current_map = self.the_config.clone();
        new_map.iter().for_each(|(k, v)| {
            current_map.insert(
                k.clone(),
                XvcConfigValue {
                    source: new_source,
                    value: v.clone(),
                },
            );
        });

        let mut new_config_maps = self.config_maps.clone();
        new_config_maps.push(XvcConfigMap {
            source: new_source,
            map: new_map,
        });

        Ok(Self {
            current_dir: self.current_dir.clone(),
            init_params: self.init_params.clone(),
            the_config: current_map,
            config_maps: new_config_maps,
        })
    }

    fn update_from_toml(
        &self,
        configuration: String,
        new_source: XvcConfigOptionSource,
    ) -> Result<Self> {
        let new_val = configuration.parse::<TomlValue>()?;
        let key = "".to_string();
        let new_map = toml_value_to_hashmap(key, new_val);
        self.update_from_hash_map(new_map, new_source)
    }

    fn update_from_file(
        &self,
        file_name: &Path,
        source: XvcConfigOptionSource,
    ) -> Result<XvcConfig> {
        if file_name.is_file() {
            let config_string =
                fs::read_to_string(file_name).map_err(|source| Error::IoError { source })?;
            self.update_from_toml(config_string, source)
        } else {
            Err(Error::ConfigurationForSourceNotFound {
                config_source: source.to_string(),
                path: file_name.as_os_str().into(),
            })
        }
    }

    fn system_config_file() -> Result<PathBuf> {
        Ok(SYSTEM_CONFIG_DIRS
            .to_owned()
            .ok_or(Error::CannotDetermineSystemConfigurationPath)?
            .config_dir()
            .to_path_buf())
    }

    fn user_config_file() -> Result<PathBuf> {
        Ok(USER_CONFIG_DIRS
            .to_owned()
            .ok_or(Error::CannotDetermineUserConfigurationPath)?
            .config_dir()
            .join("xvc"))
    }

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

    fn parse_key_value_vector(vector: Vec<String>) -> Vec<(String, TomlValue)> {
        vector
            .into_iter()
            .map(|str| {
                let elements: Vec<&str> = str.split('=').collect();
                let key = elements[0].to_owned();
                let value = Self::parse_to_value(elements[1].to_owned());
                (key, value)
            })
            .collect()
    }

    /// Loads all config files
    /// Overrides all options with the given key=value style options in the
    /// command line
    pub fn new(p: XvcConfigInitParams) -> Result<XvcConfig> {
        let mut config = XvcConfig::default_conf(&p);

        config.current_dir = XvcConfigOption {
            option: p.current_dir,
            source: XvcConfigOptionSource::Runtime,
        };

        let mut update = |source, file: std::result::Result<PathBuf, Error>| match file {
            Ok(config_file) => match config.update_from_file(&config_file, source) {
                Ok(new_config) => config = new_config,
                Err(err) => {
                    err.info();
                }
            },
            Err(err) => {
                err.info();
            }
        };

        if p.include_system_config {
            let f = Self::system_config_file();
            update(XvcConfigOptionSource::System, f);
        }

        if p.include_user_config {
            update(XvcConfigOptionSource::Global, Self::user_config_file());
        }

        if let Some(project_config_path) = p.project_config_path {
            update(XvcConfigOptionSource::Project, Ok(project_config_path));
        }

        if let Some(local_config_path) = p.local_config_path {
            update(XvcConfigOptionSource::Local, Ok(local_config_path));
        }

        if p.include_environment_config {
            let env_config = Self::env_map().unwrap();
            match config.update_from_hash_map(env_config, XvcConfigOptionSource::Environment) {
                Ok(conf) => config = conf,
                Err(err) => {
                    err.info();
                }
            }
        }

        if let Some(cli_config) = p.command_line_config {
            let map: HashMap<String, TomlValue> = Self::parse_key_value_vector(cli_config)
                .into_iter()
                .collect();
            match config.update_from_hash_map(map, XvcConfigOptionSource::CommandLine) {
                Ok(conf) => config = conf,
                Err(err) => {
                    err.info();
                }
            }
        }

        Ok(config)
    }

    pub fn current_dir(&self) -> Result<&AbsolutePath> {
        let pb = &self.current_dir.option;
        Ok(pb)
    }

    pub fn guid(&self) -> Option<String> {
        match self.get_str("core.guid") {
            Ok(opt) => Some(opt.option),
            Err(err) => {
                err.warn();
                None
            }
        }
    }

    pub fn verbosity(&self) -> XvcVerbosity {
        let verbosity_str = match self.get_str("core.verbosity") {
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

pub trait UpdateFromXvcConfig {
    fn update_from_conf(self, conf: &XvcConfig) -> Result<Box<Self>>;
}

pub trait FromConfigKey<T: FromStr> {
    fn from_conf(conf: &XvcConfig) -> T;
    fn try_from_conf(conf: &XvcConfig) -> Result<T>;
}

#[macro_export]
macro_rules! conf {
    ($type: ty, $key: literal) => {
        impl FromConfigKey<$type> for $type {
            fn from_conf(conf: &XvcConfig) -> $type {
                conf.get_val::<$type>($key).unwrap()
            }

            fn try_from_conf(conf: &XvcConfig) -> xvc_config::error::Result<$type> {
                conf.get_val::<$type>($key)
            }
        }
    };
}

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
    use xvc_logging::{setup_logging, watch};

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
        watch!(table_hm);
        assert!(table_hm.get("core.foo") == Some(&TomlValue::String("bar".to_string())));
        Ok(())
    }
}
