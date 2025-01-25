//! Cloud storage implementations for xvc.
pub mod common_ops;

#[cfg(feature = "async")]
pub mod async_common;

#[cfg(feature = "digital-ocean")]
pub mod digital_ocean;
pub mod event;
#[cfg(feature = "gcs")]
pub mod gcs;
pub mod generic;
pub mod local;
#[cfg(feature = "minio")]
pub mod minio;
#[cfg(feature = "r2")]
pub mod r2;
pub mod rsync;
#[cfg(feature = "s3")]
pub mod s3;
#[cfg(feature = "wasabi")]
pub mod wasabi;

use std::{env, ffi::OsStr, str::FromStr};

use derive_more::Display;
pub use event::{
    XvcStorageDeleteEvent, XvcStorageEvent, XvcStorageExpiringShareEvent, XvcStorageInitEvent,
    XvcStorageListEvent, XvcStorageReceiveEvent, XvcStorageSendEvent,
};

pub use local::XvcLocalStorage;

use serde::{Deserialize, Serialize};
use tempfile::TempDir;
use uuid::Uuid;
use xvc_logging::{error, XvcOutputSender};
use xvc_walker::AbsolutePath;

use clap_complete::CompletionCandidate;

use crate::{Error, Result, StorageIdentifier};

use relative_path::{RelativePath, RelativePathBuf};

use xvc_core::{util::completer::load_store_for_completion, XvcCachePath, XvcRoot};
use xvc_ecs::{persist, XvcStore};

use self::generic::XvcGenericStorage;

/// A storage that can be used to send and receive files with several different backends
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum XvcStorage {
    /// A local storage is a directory that is on the same machine as the repository.
    Local(XvcLocalStorage),
    /// A generic storage creates the storage and sends, receives, deletes files with shell
    /// commands
    Generic(XvcGenericStorage),
    /// A rsync storage is a directory in an Rsync/SSH host
    Rsync(rsync::XvcRsyncStorage),
    /// An S3 storage is a bucket in AWS S3
    #[cfg(feature = "s3")]
    S3(s3::XvcS3Storage),
    /// An R2 storage is a bucket in R2
    #[cfg(feature = "r2")]
    R2(r2::XvcR2Storage),
    /// A GCS storage is a bucket in Google Cloud Storage
    #[cfg(feature = "gcs")]
    Gcs(gcs::XvcGcsStorage),
    /// A Minio storage is a bucket in Minio
    #[cfg(feature = "minio")]
    Minio(minio::XvcMinioStorage),
    /// A Wasabi storage is a bucket in Wasabi
    #[cfg(feature = "wasabi")]
    Wasabi(wasabi::XvcWasabiStorage),
    /// A DigitalOcean storage is a bucket in DigitalOcean
    #[cfg(feature = "digital-ocean")]
    DigitalOcean(digital_ocean::XvcDigitalOceanStorage),
}
persist!(XvcStorage, "storage");

impl XvcStorage {
    pub fn name(&self) -> String {
        match &self {
            XvcStorage::Local(s) => s.name,
            XvcStorage::Generic(s) => s.name,
            XvcStorage::Rsync(s) => s.name,
            #[cfg(feature = "s3")]
            XvcStorage::S3(s) => s.name,
            #[cfg(feature = "minio")]
            XvcStorage::Minio(s) => s.name,
            #[cfg(feature = "r2")]
            XvcStorage::R2(s) => s.name,
            #[cfg(feature = "gcs")]
            XvcStorage::Gcs(s) => s.name,
            #[cfg(feature = "wasabi")]
            XvcStorage::Wasabi(s) => s.name,
            #[cfg(feature = "digital-ocean")]
            XvcStorage::DigitalOcean(s) => s.name,
        }
    }

    pub fn guid(&self) -> String {
        match &self {
            XvcStorage::Local(s) => s.guid,
            XvcStorage::Generic(s) => s.guid,
            XvcStorage::Rsync(s) => s.guid,
            #[cfg(feature = "s3")]
            XvcStorage::S3(s) => s.guid,
            #[cfg(feature = "minio")]
            XvcStorage::Minio(s) => s.guid,
            #[cfg(feature = "r2")]
            XvcStorage::R2(s) => s.guid,
            #[cfg(feature = "gcs")]
            XvcStorage::Gcs(s) => s.guid,
            #[cfg(feature = "wasabi")]
            XvcStorage::Wasabi(s) => s.guid,
            #[cfg(feature = "digital-ocean")]
            XvcStorage::DigitalOcean(s) => s.guid,
        }
    }
}

impl std::fmt::Display for XvcStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XvcStorage::Local(lr) => {
                write!(
                    f,
                    "Local:   {}\t{}\t{}",
                    lr.name,
                    lr.guid,
                    lr.path.to_string_lossy()
                )
            }
            XvcStorage::Generic(gr) => write!(
                f,
                "Generic: {}\t{}\t{}{}",
                gr.name,
                gr.guid,
                gr.url.as_ref().unwrap_or(&String::new()),
                gr.storage_dir.as_ref().unwrap_or(&String::new())
            ),
            XvcStorage::Rsync(r) => write!(
                f,
                "Rsync:   {}\t{}\t{}:{}{}",
                r.name,
                r.guid,
                r.host,
                r.port
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "".to_string()),
                r.storage_dir
            ),

            #[cfg(feature = "s3")]
            XvcStorage::S3(r) => write!(
                f,
                "S3:      {}\t{}\t{}.{}/{}",
                r.name, r.guid, r.region, r.bucket_name, r.storage_prefix
            ),
            #[cfg(feature = "minio")]
            XvcStorage::Minio(mr) => write!(
                f,
                "Minio:   {}\t{}\t{}.{}/{}",
                mr.name, mr.guid, mr.endpoint, mr.bucket_name, mr.storage_prefix
            ),
            #[cfg(feature = "r2")]
            XvcStorage::R2(r2r) => write!(
                f,
                "R2:      {}\t{}\t{} {}/{}",
                r2r.name, r2r.guid, r2r.account_id, r2r.bucket_name, r2r.storage_prefix
            ),
            #[cfg(feature = "gcs")]
            XvcStorage::Gcs(gcsr) => write!(
                f,
                "GCS:     {}\t{}\t{}.{}/{}",
                gcsr.name, gcsr.guid, gcsr.region, gcsr.bucket_name, gcsr.storage_prefix
            ),
            #[cfg(feature = "wasabi")]
            XvcStorage::Wasabi(wr) => write!(
                f,
                "Wasabi:  {}\t{}\t{}.{}/{}",
                wr.name, wr.guid, wr.endpoint, wr.bucket_name, wr.storage_prefix
            ),
            #[cfg(feature = "digital-ocean")]
            XvcStorage::DigitalOcean(dor) => write!(
                f,
                "DO:      {}\t{}\t{}.{}/{}",
                dor.name, dor.guid, dor.region, dor.bucket_name, dor.storage_prefix
            ),
        }
    }
}

/// The temporary directory used to store files that are sent or received from a remote storage.
/// Xvc downloads to a temporary directory and then copies the files to the cache directory.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct XvcStorageTempDir(AbsolutePath);
impl XvcStorageTempDir {
    /// Create a new temporary directory
    pub fn new() -> Result<Self> {
        let temp_dir = AbsolutePath::from(TempDir::new()?.into_path());
        Ok(Self(temp_dir))
    }

    /// The path of the temporary directory
    pub fn path(&self) -> &AbsolutePath {
        &self.0
    }

    /// A temporary directory for the cache directory
    pub fn temp_cache_dir(&self, cache_path: &XvcCachePath) -> Result<AbsolutePath> {
        let temp_cache_dir = self.0.join(cache_path.directory().as_str());
        Ok(temp_cache_dir)
    }

    /// A temporary path for a cache file
    pub fn temp_cache_path(&self, cache_path: &XvcCachePath) -> Result<AbsolutePath> {
        let temp_cache_path = self.0.join(cache_path.inner().as_str());
        Ok(temp_cache_path)
    }
}

/// The storage path relative to `$STORAGE_ROOT/$REPO_GUID/`.
/// It uses [RelativePathBuf] internally.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize, Display)]
pub struct XvcStoragePath(RelativePathBuf);
persist!(XvcStoragePath, "storage-path");

impl From<String> for XvcStoragePath {
    fn from(p: String) -> Self {
        Self(RelativePathBuf::from(p))
    }
}
impl FromStr for XvcStoragePath {
    fn from_str(p: &str) -> Result<Self> {
        Ok(Self(RelativePathBuf::from(p.to_string())))
    }

    type Err = crate::Error;
}

impl AsRef<RelativePath> for XvcStoragePath {
    fn as_ref(&self) -> &RelativePath {
        self.0.as_ref()
    }
}

impl XvcStoragePath {
    /// The storage path of a cache path is like {guid}/{cache-path}
    /// ⚠️  The separator between {guid} and {cache-path} is always /
    pub fn new(xvc_root: &XvcRoot, local: &XvcCachePath) -> Self {
        let guid = xvc_root.config().guid().unwrap();
        let guid_path = RelativePathBuf::from(guid);
        let rel_path = guid_path.join(local);
        Self(rel_path)
    }

    /// String representation of the internal [RelativePathBuf]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// A unique identifier for (remote) storages.
/// These can be used in commands to identify the storages.
/// Uses [Uuid] internally.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize, Display)]
pub struct XvcStorageGuid(Uuid);

impl XvcStorageGuid {
    /// Create a new storage guid to be stored in [XvcStore<XvcStorage>] and storage's [XVC_STORAGE_GUID_FILENAME]
    /// Used to identify a storage uniquely
    pub fn new() -> Self {
        let guid = uuid::Uuid::new_v4();
        Self(guid)
    }
}

impl Default for XvcStorageGuid {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for XvcStorageGuid {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for XvcStorageGuid {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

/// Contains the filename that the storage GUID is store in storage root
pub const XVC_STORAGE_GUID_FILENAME: &str = ".xvc-guid";

/// Retrieve [XvcStorage] from [StorageIdentifier] by trying to match the name first and if not found, guid.
pub fn get_storage_record(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    identifier: &StorageIdentifier,
) -> Result<XvcStorage> {
    let store: XvcStore<XvcStorage> = xvc_root.load_store()?;
    let storage_store = store.filter(|_, r| match identifier {
        StorageIdentifier::Name(ref n) => match r {
            XvcStorage::Local(r) => r.name == *n,
            XvcStorage::Generic(r) => r.name == *n,
            XvcStorage::Rsync(r) => r.name == *n,
            #[cfg(feature = "s3")]
            XvcStorage::S3(r) => r.name == *n,
            #[cfg(feature = "minio")]
            XvcStorage::Minio(r) => r.name == *n,
            #[cfg(feature = "r2")]
            XvcStorage::R2(r) => r.name == *n,
            #[cfg(feature = "gcs")]
            XvcStorage::Gcs(r) => r.name == *n,
            #[cfg(feature = "wasabi")]
            XvcStorage::Wasabi(r) => r.name == *n,
            #[cfg(feature = "digital-ocean")]
            XvcStorage::DigitalOcean(r) => r.name == *n,
        },
        StorageIdentifier::Uuid(ref id) => match r {
            XvcStorage::Local(lr) => lr.guid == (*id).into(),
            XvcStorage::Generic(gr) => gr.guid == (*id).into(),
            XvcStorage::Rsync(r) => r.guid == (*id).into(),
            #[cfg(feature = "s3")]
            XvcStorage::S3(s3r) => s3r.guid == (*id).into(),
            #[cfg(feature = "minio")]
            XvcStorage::Minio(mr) => mr.guid == (*id).into(),
            #[cfg(feature = "r2")]
            XvcStorage::R2(r) => r.guid == (*id).into(),
            #[cfg(feature = "gcs")]
            XvcStorage::Gcs(r) => r.guid == (*id).into(),
            #[cfg(feature = "wasabi")]
            XvcStorage::Wasabi(r) => r.guid == (*id).into(),
            #[cfg(feature = "digital-ocean")]
            XvcStorage::DigitalOcean(r) => r.guid == (*id).into(),
        },
    });

    if storage_store.is_empty() {
        error!(output_snd, "Cannot find remote {}", identifier);
    }
    if storage_store.len() > 1 {
        error!(
            output_snd,
            "Ambiguous remote identifier: {} Please use Storage GUID.", identifier
        );
    }

    let (_, storage) =
        storage_store
            .first()
            .ok_or_else(|| Error::CannotFindStorageWithIdentifier {
                identifier: identifier.clone(),
            })?;
    Ok(storage.clone())
}

pub fn storage_identifier_completer(prefix: &OsStr) -> Vec<CompletionCandidate> {
    // This must be safe as we don't allow Non-UTF-8 strings for storage identifiers
    let prefix = prefix.to_str().unwrap_or("");
    env::current_dir()
        .map_err(Error::from)
        .and_then(|current_dir| {
            load_store_for_completion::<XvcStorage>(&current_dir)
                .map(|xvc_storage_store| {
                    let filtered_by_name: Vec<String> = xvc_storage_store
                        .filter(|_, xs| xs.name().starts_with(prefix))
                        .iter()
                        .map(|_, xs| xs.name())
                        .collect();
                    let filtered_by_guid: Vec<String> = xvc_storage_store
                        .filter(|_, xs| xs.guid().starts_with(prefix))
                        .iter()
                        .map(|_, xs| xs.guid())
                        .collect();

                    filtered_by_name
                        .iter()
                        .chain(filtered_by_guid.iter())
                        .map(|xs| xs.into())
                        .collect()
                })
                .unwrap_or_default()
        })
}
