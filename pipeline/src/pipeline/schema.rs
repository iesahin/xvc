use std::{ffi::OsStr, path::Path};

use crate::error::{Error, Result};
use crate::pipeline::deps::XvcDependency;
use crate::pipeline::XvcOutput;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, IntoStaticStr};
use xvc_core::XvcPath;

use super::XvcStepInvalidate;

#[derive(Debug, Clone, Eq, PartialEq, EnumString, Display, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum XvcSchemaSerializationFormat {
    Json,
    Yaml,
    // Turned off because of UnsupportedType error
    // TOML,
}

impl XvcSchemaSerializationFormat {
    fn from_extension(ext: &OsStr) -> Result<Self> {
        match ext.to_str().unwrap_or("") {
            "json" | "JSON" => Ok(Self::Json),
            "yaml" | "yml" => Ok(Self::Yaml),
            // "toml" | "tom" | "tml" => Ok(Self::TOML),
            _ => Err(Error::CannotInferFormatFromExtension {
                extension: ext.into(),
            }),
        }
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        Self::from_extension(path.extension().unwrap_or_else(|| OsStr::new("")))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct XvcPipelineSchema {
    pub version: i32,
    pub name: String,
    pub workdir: XvcPath,
    pub steps: Vec<XvcStepSchema>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct XvcStepSchema {
    pub name: String,
    pub command: String,
    pub invalidate: XvcStepInvalidate,
    pub dependencies: Vec<XvcDependency>,
    pub outputs: Vec<XvcOutput>,
}
