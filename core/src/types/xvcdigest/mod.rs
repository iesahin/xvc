//! Xvc digest calculations
pub mod content_digest;
pub mod path_collection_digest;
pub mod stdout_digest;
pub mod url_get_digest;
pub mod xvc_metadata_digest;

use crate::types::hashalgorithm::HashAlgorithm;

use std::collections::BTreeMap;

use std::{fmt::Display, fs, path::Path};
use xvc_ecs::{persist, Storable};

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

/// A digest that's used as an attribute of an entity e.g., [ContentDigest], [CollectionDigest],
/// [XvcMetadataDigest]
///
/// We can have multiple digests for an entity, e.g., a file can have a digest from its metadata
/// and a digest from its content. This trait is used to assign names to digest as attributes
pub trait AttributeDigest: Storable + From<XvcDigest> + Into<XvcDigest> + AsRef<XvcDigest> {
    /// The name of this digest, e.g., "content-digest"
    fn attribute() -> String {
        <Self as Storable>::type_description()
    }

    /// The digest value of this digest
    fn digest(&self) -> XvcDigest;
}

#[macro_export]
/// Specifies an attribute digest
///
/// ## Example
/// ```rust,ignore
/// attribute_digest!(MyDigestType, "my-digest-type");
/// ```
///
/// makes `MyType` to implement [xvc_ecs::Storable], From<XvcDigest> and Into<XvcDigest> traits.
/// It also implements [xvc_core::AttributeDigest] trait.
/// This trait has a `attribute` function that returns the specified string.
/// This string is then used in [XvcDigests] as a key to store the digest.
macro_rules! attribute_digest {
    ( $t:ty, $desc:literal ) => {
        impl ::xvc_ecs::Storable for $t {
            fn type_description() -> String {
                $desc.to_string()
            }
        }

        impl From<$crate::XvcDigest> for $t {
            fn from(digest: $crate::XvcDigest) -> Self {
                Self(digest)
            }
        }

        impl From<$t> for $crate::XvcDigest {
            fn from(digest: $t) -> Self {
                digest.digest()
            }
        }

        impl AsRef<$crate::XvcDigest> for $t {
            fn as_ref(&self) -> &$crate::XvcDigest {
                &self.0
            }
        }

        impl $crate::AttributeDigest for $t {
            fn digest(&self) -> XvcDigest {
                self.0
            }
        }
    };
}

/// An entity can contain more than one digest, e.g., a file can have a digest from its metadata and a digest from its content.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]

pub struct XvcDigests(pub BTreeMap<String, XvcDigest>);
persist!(XvcDigests, "xvc-digests");

impl<T> From<T> for XvcDigests
where
    T: AttributeDigest,
{
    fn from(digest: T) -> Self {
        let mut map = BTreeMap::new();
        map.insert(<T as AttributeDigest>::attribute(), digest.digest());
        Self(map)
    }
}

impl Default for XvcDigests {
    fn default() -> Self {
        Self::new()
    }
}

impl XvcDigests {
    /// Returns a new empty [XvcDigests]
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    /// Returns all available attributes/keys for an entity
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    /// Number of digests in [XvcDigests]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if there are no digests in [XvcDigests]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Inserts a digest into [XvcDigests]
    pub fn insert<T: AttributeDigest>(&mut self, attribute_digest: T) {
        self.0.insert(T::attribute(), attribute_digest.digest());
    }

    /// Returns the digest for the specified attribute
    pub fn get<T: AttributeDigest>(&self) -> Option<T> {
        let attribute = T::attribute();
        self.0.get(&attribute).cloned().map(|d| d.into())
    }

    /// Returns true if there is a digest for the specified attribute
    pub fn has_attribute<T: AttributeDigest>(&self) -> bool {
        self.0.contains_key(&T::attribute())
    }

    /// Removes the digest for the specified attribute
    pub fn remove<T: AttributeDigest>(&mut self) -> Option<T> {
        let attribute = T::attribute();
        self.0.remove(&attribute).map(|d| d.into())
    }

    /// Inserts a digest into [XvcDigests] with the a string key
    pub fn insert_with_arbitrary_attribute(&mut self, attribute: String, digest: XvcDigest) {
        self.0.insert(attribute, digest);
    }

    /// Merges with another [XvcDigests]
    pub fn merge_with(&mut self, other: &Self) {
        self.0.extend(other.0.clone());
    }

    /// Returns a new [XvcDigests] created from the specified attribute digest as only digest
    pub fn from_attribute_digest<T: AttributeDigest>(attribute_digest: T) -> Self {
        let mut s = Self::new();
        s.insert(attribute_digest);
        s
    }
}

impl AsRef<BTreeMap<String, XvcDigest>> for XvcDigests {
    fn as_ref(&self) -> &BTreeMap<String, XvcDigest> {
        &self.0
    }
}
