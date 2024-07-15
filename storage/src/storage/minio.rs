//! Minio remote storage implementation.
use std::env;

use anyhow::anyhow;

use s3::creds::Credentials;
use s3::{Bucket, Region};
use serde::{Deserialize, Serialize};
use xvc_core::{XvcCachePath, XvcRoot};
use xvc_ecs::R1NStore;
use xvc_logging::{watch, XvcOutputSender};

use crate::{Result, XvcStorage, XvcStorageEvent};
use crate::{XvcStorageGuid, XvcStorageOperations};

use super::async_common::XvcS3StorageOperations;
use super::XvcStoragePath;

/// Configure a new Minio remote storage.
///
/// `endpoint`, `bucket_name`, `region` and `storage_prefix` sets a URL for the
/// storage location.
///
/// This creates a [XvcMinioStorage], calls its
/// [init][XvcMinioStorage::init] function to create/update guid, and
/// saves [XvcStorageInitEvent] and [XvcStorage] in ECS.
/// TODO: Reduce the number of parameters of this function.
#[allow(clippy::too_many_arguments)]
pub fn cmd_new_minio(
    _input: std::io::StdinLock,
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    name: String,
    endpoint: String,
    bucket_name: String,
    region: String,
    storage_prefix: String,
) -> Result<()> {
    let mut storage = XvcMinioStorage {
        guid: XvcStorageGuid::new(),
        name,
        region,
        bucket_name,
        storage_prefix,
        endpoint,
    };
    watch!(storage);

    let init_event = storage.init(output_snd, xvc_root)?;
    watch!(init_event);

    xvc_root.with_r1nstore_mut(|store: &mut R1NStore<XvcStorage, XvcStorageEvent>| {
        let store_e = xvc_root.new_entity();
        let event_e = xvc_root.new_entity();
        store.insert(
            store_e,
            XvcStorage::Minio(storage.clone()),
            event_e,
            XvcStorageEvent::Init(init_event.clone()),
        );
        Ok(())
    })?;

    Ok(())
}

/// A Minio storage is a remote storage that is compatible with the S3 protocol.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcMinioStorage {
    /// GUID of the storage
    pub guid: XvcStorageGuid,
    /// Name of the storage
    pub name: String,
    /// Region of the storage
    pub region: String,
    /// Bucket name of the storage
    pub bucket_name: String,
    /// Prefix of the storage within the bucket_name
    pub storage_prefix: String,
    /// Full endpoint of the storage
    pub endpoint: String,
}

impl XvcMinioStorage {
    fn storage_specific_credentials(&self) -> Result<Credentials> {
        Credentials::new(
            Some(&env::var(format!(
                "XVC_STORAGE_ACCESS_KEY_ID_{}",
                self.name
            ))?),
            Some(&env::var(format!("XVC_STORAGE_SECRET_KEY_{}", self.name))?),
            None,
            None,
            None,
        )
        .map_err(|e| e.into())
    }

    fn storage_type_credentials(&self) -> Result<Credentials> {
        Credentials::new(
            Some(&env::var("MINIO_ACCESS_KEY_ID").unwrap()),
            Some(&env::var("MINIO_SECRET_ACCESS_KEY").unwrap()),
            None,
            None,
            None,
        )
        .map_err(|e| e.into())
    }
}

impl XvcS3StorageOperations for XvcMinioStorage {
    fn storage_prefix(&self) -> String {
        self.storage_prefix.clone()
    }

    fn guid(&self) -> &XvcStorageGuid {
        &self.guid
    }

    fn get_bucket(&self) -> Result<Bucket> {
        // We'll just put guid file to endpoint/bucket/prefix/XVC_GUID_FILENAME
        let credentials = self.credentials()?;
        let region = Region::Custom {
            region: self.region.clone(),
            endpoint: self.endpoint.clone(),
        };
        let bucket = Bucket::new(&self.bucket_name, region, credentials)?;
        Ok(bucket.with_path_style())
    }

    fn credentials(&self) -> Result<Credentials> {
        match self.storage_specific_credentials() {
            Ok(c) => Ok(c),
            Err(e1) => match self.storage_type_credentials() {
                Ok(c) => Ok(c),
                Err(e2) => Err(anyhow!(
                    "None of the required environment variables found for credentials: {}\n{}\n",
                    e1,
                    e2
                )),
            },
        }
        .map_err(|e| e.into())
    }

    fn bucket_name(&self) -> String {
        self.bucket_name.clone()
    }

    fn build_storage_path(&self, cache_path: &XvcCachePath) -> XvcStoragePath {
        XvcStoragePath::from(format!(
            "{}/{}/{}",
            self.storage_prefix,
            self.guid(),
            cache_path
        ))
    }

    fn region(&self) -> String {
        self.region.clone()
    }
}
