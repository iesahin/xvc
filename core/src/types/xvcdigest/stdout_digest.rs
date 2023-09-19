//! Digest of a command output.
use crate::types::hashalgorithm::HashAlgorithm;
use crate::{attribute_digest, XvcDigest};

use serde::{Deserialize, Serialize};

use super::AttributeDigest;

/// Digest of a command output.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StdoutDigest(XvcDigest);
attribute_digest!(StdoutDigest, "stdout-digest");

impl StdoutDigest {
    /// Returns a stable digest of the stdout of a command.
    pub fn new(stdout: &str, algorithm: HashAlgorithm) -> Self {
        Self(XvcDigest::from_content(stdout, algorithm))
    }
}
