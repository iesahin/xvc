//! Decide whether a digest is calculated as text or binary file.
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, VariantNames};

use xvc_ecs::persist;

/// Represents whether a file is a text file or not
///
/// Each file can be configured separately.
///
/// Configuration for each file is saved in `text-or-binary` BStore.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    EnumString,
    Hash,
    Display,
    Copy,
    VariantNames,
)]
#[strum(serialize_all = "lowercase")]
pub enum TextOrBinary {
    /// Check whether first 8000 bytes contains 0s in a file.
    /// If found, it's considered binary.
    /// Otherwise it's text.
    /// This is the technique used by Git as well.
    Auto,
    /// Consider a file as text.
    /// Digest calculation is done after removing `\r` characters.
    Text,
    /// Consider a file as binary.
    /// Nothing is changed before digest calculation.
    Binary,
}

impl Default for TextOrBinary {
    fn default() -> Self {
        Self::Auto
    }
}

persist!(TextOrBinary, "text-or-binary");
