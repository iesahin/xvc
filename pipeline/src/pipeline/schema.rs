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

/// Defines the user editable pipeline schema used in `xvc pipeline export` and
/// `xvc pipeline import` commands.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct XvcPipelineSchema {
    /// Version of the schema, currently 1.
    pub version: i32,
    /// Name of the pipeline.
    /// Note that this can also be specified in CLI with `--name` flag and it
    /// supersedes this value.
    pub name: String,
    /// Path to the pipeline root directory.
    pub workdir: XvcPath,
    /// List of steps in the pipeline.
    pub steps: Vec<XvcStepSchema>,
}

/// User editable pipeline step schema used in `xvc pipeline export` and `xvc
/// pipeline import` commands.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct XvcStepSchema {
    /// Name of the step.
    pub name: String,
    /// Command to run in the step.
    pub command: String,
    /// When we consider the step as changed?
    pub invalidate: XvcStepInvalidate,
    /// List of dependencies of the step.
    /// These do not require a separate schema.
    pub dependencies: Vec<XvcDependency>,
    /// List of outputs of the step.
    /// These do not require a separate schema.
    pub outputs: Vec<XvcOutput>,
}
