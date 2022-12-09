use crate::error::{Error, Result};
use serde_json::value::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::{ffi::OsStr, fmt::Display, fs, path::Path};
use toml::Value as TomlValue;

use log::{error, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, PartialEq, Serialize, Deserialize)]
pub enum XvcParamFormat {
    Unknown,
    YAML,
    JSON,
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

    pub fn from_path(path: &Path) -> Self {
        match path.extension() {
            None => {
                error!("[E0000] Params file has no extension: {:?}", path);
                Self::Unknown
            }
            Some(ext) => Self::from_extension(ext),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum XvcParamValue {
    Json(JsonValue),
    Yaml(YamlValue),
    Toml(TomlValue),
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XvcParamPair {
    pub key: String,
    pub value: XvcParamValue,
}

impl XvcParamPair {
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

    fn parse_json(all_content: &str, key: &str) -> Result<XvcParamPair> {
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
                    | JsonValue::Array(_) => {
                        return Ok(XvcParamPair {
                            key: key.to_string(),
                            value: XvcParamValue::Json(current_value.clone()),
                        })
                    }
                    JsonValue::Null => {
                        return Err(Error::JsonNullValueForKey { key: key.into() });
                    }
                }
            } else {
                return Err(Error::KeyNotFound { key: key.into() });
            }
        }
        // If we consumed all key elements and come to here, we consider the current scope as value
        Ok(XvcParamPair {
            key: key.into(),
            value: XvcParamValue::Json(current_scope),
        })
    }

    /// Loads the key (in the form of a.b.c) from a YAML document
    fn parse_yaml(all_content: &str, key: &str) -> Result<XvcParamPair> {
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
                        return Ok(XvcParamPair {
                            key: key.into(),
                            value: XvcParamValue::Yaml(current_value.clone()),
                        })
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
        Ok(XvcParamPair {
            key: key.into(),
            value: XvcParamValue::Yaml(current_scope),
        })
    }

    /// Loads a TOML file and returns the `XvcParamPair::TOML(TomlValue)`
    /// associated with the key

    fn parse_toml(all_content: &str, key: &str) -> Result<XvcParamPair> {
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
                        return Ok(XvcParamPair {
                            key: key.into(),
                            value: XvcParamValue::Toml(current_value.clone()),
                        })
                    }
                }
            } else {
                return Err(Error::KeyNotFound { key: key.into() });
            }
        }
        // If we consumed the key without errors, we consider the resulting scope as the value
        Ok(XvcParamPair {
            key: key.into(),
            value: XvcParamValue::Toml(current_scope),
        })
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
        let train_epochs = XvcParamPair::parse_yaml(YAML_PARAMS, "train.epochs")?;
        assert!(train_epochs.key == "train.epochs");
        if let XvcParamValue::Yaml(YamlValue::Number(n)) = train_epochs.value {
            assert!(n.as_u64() == Some(10u64))
        } else {
            panic!("Mismatched Yaml Type: {}", train_epochs.value);
        }
        Ok(())
    }
}
