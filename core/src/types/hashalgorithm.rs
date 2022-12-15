//! Available hash algorithms in Xvc
use serde::{Deserialize, Serialize};
use strum_macros::{Display as EnumDisplay, EnumString, EnumVariantNames};
use xvc_config::{conf, FromConfigKey};

/// The available content hash algorithms to get content addresses.
/// Note that, the selection is based on the digest size being 32 bytes.
/// We already represent this as 64 digit hex in the file hierarchies.
/// Using a longer digests than 32 bytes will probably cause file length problems, especially in Windows.
///
/// The default algorithm is set by `cache.algorithm` config key.
///
/// See [crate::XvcDigest] for details of digest calculation.

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    EnumString,
    EnumDisplay,
    EnumVariantNames,
)]
pub enum HashAlgorithm {
    /// Do nothing.
    /// When the data to be hashed is already less than 32 bytes, it's used as is.
    /// Please do not use this in your functions.
    /// This is currently only used to prevent [crate::XvcMetadata] to be hashed in
    /// [xvc_ecs::HStore]
    #[strum(to_string = "a0", serialize = "asis")]
    AsIs,
    /// Use BLAKE3 to calculate digests: https://github.com/BLAKE3-team/BLAKE3
    /// This is the default
    #[strum(to_string = "b3", serialize = "blake3")]
    Blake3,
    /// Use BLAKE2S to calculate digests: https://www.blake2.net
    #[strum(to_string = "b2", serialize = "blake2")]
    Blake2s,
    /// Use SHA2-256 to calculate digests. https://en.wikipedia.org/wiki/SHA-2
    /// This may be used when FIPS/NIST compatibility is required.
    #[strum(to_string = "s2", serialize = "sha2")]
    SHA2_256,
    /// Use SHA2-256 to calculate digests: https://en.wikipedia.org/wiki/SHA-3
    /// This may be used when NIST compatibility is required.
    #[strum(to_string = "s3", serialize = "sha3")]
    SHA3_256,
}

conf!(HashAlgorithm, "cache.algorithm");
