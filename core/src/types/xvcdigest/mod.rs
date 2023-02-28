//! Xvc digest calculations
pub mod collection_digest;
pub mod content_digest;
pub mod stdout_digest;
pub mod url_get_digest;
pub mod url_head_digest;

use crate::util::file::is_text_file;
use crate::{types::hashalgorithm::HashAlgorithm, XvcPathMetadataMap};
use crate::{TextOrBinary, XvcMetadata};
use reqwest::Url;
use std::collections::{BTreeMap, HashMap};
use std::time::SystemTime;
use std::{fmt::Display, fs, path::Path};
use xvc_ecs::persist;

use crate::error::Result;
use blake2::{Blake2s, Digest};
use relative_path::RelativePathBuf;
use reqwest::blocking::Client as HttpClient;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sha3::Sha3_256;

/// The digest length in bytes that is supposed to stay constant for a long time
pub const DIGEST_LENGTH: usize = 32;

/// All content digests in Xvc are 32 bytes.
pub type Digest32 = [u8; DIGEST_LENGTH];
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// Bundles the digest and the algorithm that was used to calculate it.
pub struct XvcDigest {
    /// The [crate::HashAlgorithm] used to calculate the digest
    pub algorithm: HashAlgorithm,
    /// The digest obtained from the content.
    pub digest: Digest32,
}

persist!(XvcDigest, "xvc-digest");

impl XvcDigest {
    /// The directory prefix derived from the algorithm
    pub fn directory_prefix(&self) -> String {
        format!("{}", self.algorithm)
    }

    /// Digest as slice of bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.digest
    }

    /// Converts 32-bytes digest to a hexadecimal string
    pub fn hex_str(&self) -> String {
        hex::encode(self.digest)
    }

    /// Returns the path that this content refers to in cache
    pub fn cache_dir(&self) -> RelativePathBuf {
        let dir_prefix = self.directory_prefix();
        let mut rel_path = RelativePathBuf::new();
        rel_path.push(dir_prefix);
        let hex_str = self.hex_str();
        // We're using the first and second 3 characters as directory levels. Hence the first  and
        // second level contains at most 4096 directories. Up to 16 million can be addressed
        // without filling up a directory linearly. We can increase the number of levels if file
        // system seems to degrade in performance for millions / billions of files.
        let (first, rest) = hex_str.split_at(3);
        let (second, last) = rest.split_at(3);
        rel_path.push(first);
        rel_path.push(second);
        rel_path.push(last);
        rel_path
    }

    /// Returns the content hash of the file in `path` calculated by `algorithm`
    pub fn from_binary_file(path: &Path, algorithm: HashAlgorithm) -> Result<Self> {
        let content = fs::read(path)?;
        Ok(Self::from_bytes(&content, algorithm))
    }

    /// Returns the content hash of the text file in `path` calculated by `algorithm` The
    /// difference between `from_binary_file` function is that this function removes `CR` (13, 0x0d) and `LF` (13, 0x0d) from
    /// the content before applying the hashing to keep the calculated value consistent across OSes
    pub fn from_text_file(path: &Path, algorithm: HashAlgorithm) -> Result<Self> {
        let mut content = fs::read(path)?;
        // Delete CR and LF from the content
        content.retain(|c| !(*c == 0x0D || *c == 0x0A));
        Ok(Self::from_bytes(&content, algorithm))
    }

    /// Returns the digest of the `content` calculated by `algorithm`
    pub fn from_content(content: &str, algorithm: HashAlgorithm) -> Self {
        Self::from_bytes(content.as_bytes(), algorithm)
    }

    /// Returns the digest for `bytes` with the `algorithm`.
    pub fn from_bytes(bytes: &[u8], algorithm: HashAlgorithm) -> Self {
        let digest: Digest32 = match algorithm {
            HashAlgorithm::Blake3 => Self::blake3_digest(bytes),
            HashAlgorithm::Blake2s => Self::blake2s_digest(bytes),
            HashAlgorithm::SHA2_256 => Self::sha2_256_digest(bytes),
            HashAlgorithm::SHA3_256 => Self::sha3_256_digest(bytes),
            HashAlgorithm::AsIs => {
                let mut bytes_copy: Digest32 = [0; 32];
                bytes_copy.copy_from_slice(&bytes[..32]);
                bytes_copy
            }
        };

        Self { algorithm, digest }
    }

    fn blake2s_digest(bytes: &[u8]) -> Digest32 {
        Blake2s::digest(bytes).into()
    }

    fn blake3_digest(bytes: &[u8]) -> Digest32 {
        blake3::hash(bytes).into()
    }

    fn sha3_256_digest(bytes: &[u8]) -> Digest32 {
        Sha3_256::digest(bytes).into()
    }

    fn sha2_256_digest(bytes: &[u8]) -> Digest32 {
        Sha256::digest(bytes).into()
    }
}

impl Display for XvcDigest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hex_str())
    }
}

pub trait AttributeDigest {
    fn attribute(_: Self) -> String;
    fn digest(&self) -> XvcDigest;
}

/// An entity can contain more than one digest, e.g., a file can have a digest from its metadata and a digest from its content.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]

pub struct XvcDigests(pub BTreeMap<String, XvcDigest>);
persist!(XvcDigests, "xvc-digests");

impl From<Box<dyn AttributeDigest>> for XvcDigests {
    fn from(digest: Box<dyn AttributeDigest>) -> Self {
        let mut map = BTreeMap::new();
        map.insert(digest.attribute(), digest.digest());
        Self(map)
    }
}

impl XvcDigests {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, attribute_digest: impl AttributeDigest) {
        self.0
            .insert(attribute_digest.attribute(), attribute_digest.digest());
    }

    pub fn get(&self, attribute: &str) -> Option<&XvcDigest> {
        self.0.get(attribute)
    }

    pub fn has_digest_kind(&self, attribute: &str) -> bool {
        self.0.contains_key(attribute)
    }

    pub fn remove(&mut self, attribute: String) -> Option<XvcDigest> {
        self.0.remove(&attribute)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &XvcDigest)> {
        self.0.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &XvcDigest> {
        self.0.values()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
