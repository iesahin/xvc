//! Xvc digest calculations
use crate::{types::hashalgorithm::HashAlgorithm, XvcPathMetadataMap};
use std::{fmt::Display, fs, path::Path};
use xvc_ecs::persist;

use crate::error::Result;
use blake2::{Blake2s, Digest};
use relative_path::RelativePathBuf;
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
    pub fn from_binary_file(path: &Path, algorithm: &HashAlgorithm) -> Result<Self> {
        let content = fs::read(path)?;
        Ok(Self::from_bytes(&content, algorithm))
    }

    /// Returns the content hash of the text file in `path` calculated by `algorithm` The
    /// difference between `from_binary_file` function is that this function removes `CR` (13, 0x0d) and `LF` (13, 0x0d) from
    /// the content before applying the hashing to keep the calculated value consistent across OSes
    pub fn from_text_file(path: &Path, algorithm: &HashAlgorithm) -> Result<Self> {
        let mut content = fs::read(path)?;
        // Delete CR and LF from the content
        content.retain(|c| !(*c == 0x0D || *c == 0x0A));
        Ok(Self::from_bytes(&content, algorithm))
    }

    /// Returns the digest of the `content` calculated by `algorithm`
    pub fn from_content(content: &str, algorithm: &HashAlgorithm) -> Self {
        Self::from_bytes(content.as_bytes(), algorithm)
    }

    /// Returns the digest for `bytes` with the `algorithm`.
    pub fn from_bytes(bytes: &[u8], algorithm: &HashAlgorithm) -> Self {
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

        Self {
            algorithm: *algorithm,
            digest,
        }
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

/// Defines digests we can calculate for various kinds of dependencies, outputs, etc. These
/// are used to skip heavier digest calculations by first checking lighter versions, e.g., if the
/// files' metadata for a glob has not changed, there is no need to check which file(s) content has
/// changed.

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// The digest generated from, e.g.,
/// - Metadata of file
/// - Header of HTTP request
pub struct MetadataDigest(pub Option<XvcDigest>);
persist!(MetadataDigest, "metadata-digest");

impl From<XvcDigest> for MetadataDigest {
    fn from(xd: XvcDigest) -> Self {
        Self(Some(xd))
    }
}

impl Display for MetadataDigest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => write!(f, ""),
            Some(d) => write!(f, "{}", d.hex_str()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// The digest generated from, e.g.,
/// - The content of a file in path
/// - The downloaded HTTP file
/// - The contents of all files in a directory
pub struct ContentDigest(pub Option<XvcDigest>);
persist!(ContentDigest, "content-digest");

impl From<XvcDigest> for ContentDigest {
    fn from(xd: XvcDigest) -> Self {
        Self(Some(xd))
    }
}

impl Display for ContentDigest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => write!(f, ""),
            Some(d) => write!(f, "{}", d.hex_str()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// The digest generated from, e.g.,
/// - List of files from a glob
/// - List of files in a directory
pub struct CollectionDigest(pub Option<XvcDigest>);
persist!(CollectionDigest, "collection-digest");

impl From<XvcDigest> for CollectionDigest {
    fn from(xd: XvcDigest) -> Self {
        Self(Some(xd))
    }
}

impl Display for CollectionDigest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => write!(f, ""),
            Some(d) => write!(f, "{}", d.hex_str()),
        }
    }
}

/// Returns a stable digest of the list of paths.
pub fn collection_digest(
    paths: &XvcPathMetadataMap,
    algorithm: &HashAlgorithm,
) -> Result<CollectionDigest> {
    let paths_str = paths.keys().fold("".to_string(), |mut s, xp| {
        s.push_str(xp.as_ref());
        s
    });

    Ok(CollectionDigest(Some(XvcDigest::from_content(
        &paths_str, algorithm,
    ))))
}
