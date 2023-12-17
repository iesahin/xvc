//! A parameter dependency is a key-value pair that is extracted from a parameter in YAML,
//! TOML or JSON file.
use crate::error::{Error, Result};
use crate::XvcDependency;
use serde_json::value::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::ffi::OsString;
use std::{ffi::OsStr, fmt::Display, fs, path::Path};
use toml::Value as TomlValue;
use xvc_core::types::diff::Diffable;
use xvc_core::{Diff, XvcMetadata, XvcPath, XvcPathMetadataProvider, XvcRoot};
use xvc_ecs::persist;
use xvc_logging::watch;

use log::{error, warn};
use serde::{Deserialize, Serialize};

/// Invalidates when key in params file in path changes.
#[derive(Debug, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParamDep {
    /// Format of the params file.
    /// This is inferred from extension if not given.
    pub format: XvcParamFormat,
    /// Path of the file in the workspace
    pub path: XvcPath,
    /// Key like `mydict.mykey` to access the value
    pub key: String,
    /// The value of the key
    pub value: Option<XvcParamValue>,
    /// The metadata of the parameter file to detect if it has changed
    pub xvc_metadata: Option<XvcMetadata>,
}

persist!(ParamDep, "param-dependency");

impl From<ParamDep> for XvcDependency {
    fn from(val: ParamDep) -> Self {
        XvcDependency::Param(val)
    }
}

impl ParamDep {
    /// Creates a new ParamDep with the given path and key. If the format is None, it's inferred
    /// from the path.
    pub fn new(path: &XvcPath, format: Option<XvcParamFormat>, key: String) -> Result<Self> {
        Ok(Self {
            format: format.unwrap_or_else(|| XvcParamFormat::from_xvc_path(path)),
            path: path.clone(),
            key,
            value: None,
            xvc_metadata: None,
        })
    }

    /// Update metada from the [XvcPathMetadataProvider]
    pub fn update_metadata(self, pmp: &XvcPathMetadataProvider) -> Result<Self> {
        let xvc_metadata = pmp.get(&self.path);
        Ok(Self {
            xvc_metadata,
            ..self
        })
    }

    /// Update value by reading the file
    pub fn update_value(self, xvc_root: &XvcRoot) -> Result<Self> {
        let path = self.path.to_absolute_path(xvc_root);
        let value = Some(XvcParamValue::new_with_format(
            &path,
            &self.format,
            &self.key,
        )?);
        Ok(Self { value, ..self })
    }
}

impl Diffable for ParamDep {
    type Item = Self;

    /// ⚠️ Call actual.update_metadata before calling this function ⚠️
    fn diff_superficial(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        assert!(record.path == actual.path);
        watch!(record);
        watch!(actual);
        match (record.xvc_metadata, actual.xvc_metadata) {
            (Some(record_md), Some(actual_md)) => {
                if record_md == actual_md {
                    Diff::Identical
                } else {
                    Diff::Different {
                        record: record.clone(),
                        actual: actual.clone(),
                    }
                }
            }
            (None, Some(_)) => Diff::RecordMissing {
                actual: actual.clone(),
            },
            (Some(_), None) => Diff::ActualMissing {
                record: record.clone(),
            },
            (None, None) => unreachable!("One of the metadata should always be present"),
        }
    }

    /// ⚠️ Call actual.update_metadata and actual.update_value before calling this function ⚠️
    fn diff_thorough(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        assert!(record.path == actual.path);
        watch!(record);
        watch!(actual);
        match Self::diff_superficial(record, actual) {
            Diff::Identical => Diff::Identical,
            Diff::Different { .. } => {
                if record.value == actual.value {
                    Diff::Identical
                } else {
                    Diff::Different {
                        record: record.clone(),
                        actual: actual.clone(),
                    }
                }
            }
            Diff::RecordMissing { .. } => Diff::RecordMissing {
                actual: actual.clone(),
            },
            Diff::ActualMissing { .. } => Diff::ActualMissing {
                record: record.clone(),
            },
            Diff::Skipped => Diff::Skipped,
        }
    }
}

/// Parsable formats of a parameter file
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, PartialEq, Serialize, Deserialize)]
pub enum XvcParamFormat {
    /// The default value if we cannot infer the format somehow
    Unknown,
    /// Yaml files are parsed with [serde_yaml]
    YAML,
    /// Json files are parsed with [serde_json]
    JSON,
    /// Toml files are parsed with [toml]
    TOML,
}

impl XvcParamFormat {
    fn from_extension(ext: &OsStr) -> Self {
        match ext.to_str().unwrap_or("") {
            "json" | "JSON" => Self::JSON,
            "yaml" | "yml" => Self::YAML,
            "toml" | "tom" | "tml" => Self::TOML,
            _ => {
                warn!("[W0000] Unknown parameter file extension: {:?}", ext);
                Self::Unknown
            }
        }
    }

    /// Infer the (hyper)parameter file format from the file path, by checking
    /// its extension.
    pub fn from_path(path: &Path) -> Self {
        match path.extension() {
            None => {
                error!("[E0000] Params file has no extension: {:?}", path);
                Self::Unknown
            }
            Some(ext) => Self::from_extension(ext),
        }
    }

    /// Infer the (hyper)parameter file format from the xvc_path's extension
    pub fn from_xvc_path(xvc_path: &XvcPath) -> Self {
        let extension: OsString = xvc_path
            .extension()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| "".to_owned())
            .into();
        Self::from_extension(&extension)
    }
}

/// The value of a parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum XvcParamValue {
    /// Value of a key in JSON file
    Json(JsonValue),
    /// Value of a key in YAML file
    Yaml(YamlValue),
    /// Value of a key in TOML file
    Toml(TomlValue),
}

impl PartialOrd for XvcParamValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for XvcParamValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_str = self.to_string();
        let other_str = other.to_string();
        self_str.cmp(&other_str)
    }
}

impl Eq for XvcParamValue {}

impl Display for XvcParamValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XvcParamValue::Json(json) => write!(f, "{}", json),
            XvcParamValue::Yaml(yaml) => {
                let s =
                    serde_yaml::to_string(yaml).unwrap_or_else(|_| "Error in YAML String".into());
                write!(f, "{}", s)
            }
            XvcParamValue::Toml(toml) => write!(f, "{}", toml),
        }
    }
}

impl XvcParamValue {
    /// Creates a new key with an empty value pointing to a file with an explicit [XvcParamFormat]
    pub fn new_with_format(path: &Path, format: &XvcParamFormat, key: &str) -> Result<Self> {
        let all_content = fs::read_to_string(path)?;

        let res = match format {
            XvcParamFormat::JSON => Self::parse_json(&all_content, key),
            XvcParamFormat::YAML => Self::parse_yaml(&all_content, key),
            XvcParamFormat::TOML => Self::parse_toml(&all_content, key),
            XvcParamFormat::Unknown => Err(Error::UnsupportedParamFileFormat {
                path: path.as_os_str().into(),
            }),
        };

        match res {
            // Adding the path here, normally there should be two different error messages
            Err(Error::KeyNotFoundInDocument { .. }) => Err(Error::KeyNotFoundInDocument {
                key: key.to_string(),
                path: path.to_path_buf(),
            }),
            Err(e) => Err(e),
            Ok(p) => Ok(p),
        }
    }

    fn parse_json(all_content: &str, key: &str) -> Result<Self> {
        let json_map: JsonValue = serde_json::from_str(all_content)?;
        let nested_keys: Vec<&str> = key.split('.').collect();
        let mut current_scope = json_map;
        for k in &nested_keys {
            if let Some(current_value) = current_scope.get(*k) {
                match current_value {
                    JsonValue::Object(_) => current_scope = current_value.clone(),
                    JsonValue::String(_)
                    | JsonValue::Number(_)
                    | JsonValue::Bool(_)
                    | JsonValue::Array(_) => return Ok(XvcParamValue::Json(current_value.clone())),
                    JsonValue::Null => {
                        return Err(Error::JsonNullValueForKey { key: key.into() });
                    }
                }
            } else {
                return Err(Error::KeyNotFound { key: key.into() });
            }
        }
        // If we consumed all key elements and come to here, we consider the current scope as value
        Ok(XvcParamValue::Json(current_scope))
    }

    /// Loads the key (in the form of a.b.c) from a YAML document
    fn parse_yaml(all_content: &str, key: &str) -> Result<XvcParamValue> {
        let yaml_map: YamlValue = serde_yaml::from_str(all_content)?;
        let nested_keys: Vec<&str> = key.split('.').collect();
        let mut current_scope: YamlValue = yaml_map;
        for k in &nested_keys {
            if let Some(current_value) = current_scope.get(*k) {
                match current_value {
                    YamlValue::Mapping(_) => {
                        current_scope = serde_yaml::from_value(current_value.clone())?;
                    }
                    YamlValue::Tagged(tv) => {
                        current_scope = serde_yaml::from_value(tv.value.clone())?
                    }
                    YamlValue::String(_)
                    | YamlValue::Number(_)
                    | YamlValue::Bool(_)
                    | YamlValue::Sequence(_) => {
                        return Ok(XvcParamValue::Yaml(current_value.clone()));
                    }
                    YamlValue::Null => {
                        return Err(Error::YamlNullValueForKey { key: key.into() });
                    }
                }
            } else {
                return Err(Error::KeyNotFound { key: key.into() });
            }
        }
        // If we consumed the key without errors, we consider the resulting scope as the value
        Ok(XvcParamValue::Yaml(current_scope))
    }

    /// Loads a TOML file and returns the `XvcParamPair::TOML(TomlValue)`
    /// associated with the key

    fn parse_toml(all_content: &str, key: &str) -> Result<Self> {
        let toml_map = all_content.parse::<TomlValue>()?;
        let nested_keys: Vec<&str> = key.split('.').collect();
        let mut current_scope: TomlValue = toml_map;
        for k in &nested_keys {
            if let Some(current_value) = current_scope.get(*k) {
                match current_value {
                    TomlValue::Table(_) => {
                        current_scope = current_value.clone();
                    }
                    TomlValue::String(_)
                    | TomlValue::Integer(_)
                    | TomlValue::Float(_)
                    | TomlValue::Boolean(_)
                    | TomlValue::Datetime(_)
                    | TomlValue::Array(_) => {
                        return Ok(XvcParamValue::Toml(current_value.clone()));
                    }
                }
            } else {
                return Err(Error::KeyNotFound { key: key.into() });
            }
        }
        // If we consumed the key without errors, we consider the resulting scope as the value
        Ok(XvcParamValue::Toml(current_scope))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const YAML_PARAMS: &str = r#"
train:
  epochs: 10
model:
  conv_units: 16
"#;

    #[test]
    fn test_yaml_params() -> Result<()> {
        let train_epochs = XvcParamValue::parse_yaml(YAML_PARAMS, "train.epochs")?;
        if let XvcParamValue::Yaml(YamlValue::Number(n)) = train_epochs {
            assert!(n.as_u64() == Some(10u64))
        } else {
            panic!("Mismatched Yaml Type: {}", train_epochs);
        }
        Ok(())
    }
}
