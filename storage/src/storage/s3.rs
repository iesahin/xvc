//! AWS S3 remote storage implementation
use std::env;

use anyhow::anyhow;

use s3::creds::Credentials;
use s3::{Bucket, Region};
use serde::{Deserialize, Serialize};
use xvc_core::{XvcCachePath, XvcRoot};
use xvc_core::R1NStore;
use xvc_core::{info, watch, XvcOutputSender};

use crate::storage::XVC_STORAGE_GUID_FILENAME;
use crate::{Error, Result, XvcStorage, XvcStorageEvent};
use crate::{XvcStorageGuid, XvcStorageOperations};

use super::async_common::XvcS3StorageOperations;
use super::XvcStoragePath;

/// Configure a new Amazon Web Services S3 remote storage.
///
/// `bucket_name`, `region` and `storage_prefix` sets a URL for the storage
/// location.
///
/// This creates a [XvcS3Storage], calls its
/// [init][XvcS3Storage::init] function to create/update guid, and
/// saves [XvcStorageInitEvent] and [XvcStorage] in ECS.
pub fn cmd_new_s3(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    name: String,
    region: String,
    bucket_name: String,
    storage_prefix: String,
) -> Result<()> {
    let mut storage = XvcS3Storage {
        guid: XvcStorageGuid::new(),
        name,
        region,
        bucket_name,
        storage_prefix,
    };

    let init_event = storage.init(output_snd, xvc_root)?;
    watch!(init_event);

    xvc_root.with_r1nstore_mut(|store: &mut R1NStore<XvcStorage, XvcStorageEvent>| {
        let store_e = xvc_root.new_entity();
        let event_e = xvc_root.new_entity();
        store.insert(
            store_e,
            XvcStorage::S3(storage.clone()),
            event_e,
            XvcStorageEvent::Init(init_event.clone()),
        );
        Ok(())
    })?;

    info!(output_snd, "S3 Storage Created: {:#?}", storage);

    Ok(())
}

/// An AWS S3 configuration as a remote storage location
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcS3Storage {
    /// Specifies the storage uniquely.
    ///
    /// This is also stored in
    /// `bucket_name.region.s3.amazonaws.com/storage_prefix/.xvc-guid` to identify the
    /// remote location.
    pub guid: XvcStorageGuid,
    /// Name of the remote storage to be used in commands.
    ///
    /// It doesn't have to be unique, though in practice setting unique names is
    /// preferred.
    pub name: String,

    /// The region that the bucket resides. (e.g. us-east-1)
    pub region: String,

    /// The bucket name that you created before configuring this.
    pub bucket_name: String,

    /// The "directory" in the bucket that Xvc will use.
    ///
    /// Xvc checks the presence of Guid file before creating this folder.
    pub storage_prefix: String,
}

impl XvcS3StorageOperations for XvcS3Storage {
    fn storage_prefix(&self) -> String {
        self.storage_prefix.clone()
    }

    fn guid(&self) -> &XvcStorageGuid {
        &self.guid
    }

    fn get_bucket(&self) -> Result<Box<Bucket>> {
        // We'll just put guid file to endpoint/bucket/prefix/XVC_GUID_FILENAME
        let credentials = self.credentials()?;
        let region: Region = self.region.parse().expect("Cannot parse region name");
        let bucket = Bucket::new(&self.bucket_name, region, credentials)?;
        Ok(bucket)
    }

    fn credentials(&self) -> Result<Credentials> {
        // Try storage-specific credentials
        let specific_access_key_var = format!("XVC_STORAGE_ACCESS_KEY_ID_{}", self.name);
        let specific_secret_key_var = format!("XVC_STORAGE_SECRET_ACCESS_KEY_{}", self.name);

        if let (Ok(access_key), Ok(secret_key)) =
            (env::var(&specific_access_key_var), env::var(&specific_secret_key_var))
        {
            return Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)
                .map_err(|e| e.into());
        }

        // Try storage-type credentials
        let type_access_key_var = "AWS_ACCESS_KEY_ID";
        let type_secret_key_var = "AWS_SECRET_ACCESS_KEY";
        if let (Ok(access_key), Ok(secret_key)) =
            (env::var(type_access_key_var), env::var(type_secret_key_var))
        {
            return Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)
                .map_err(|e| e.into());
        }

        // If both fail, return an error
        Err(Error::CloudCredentialsNotFound {
            storage_name: self.name.clone(),
            var_pairs: vec![
                (specific_access_key_var, specific_secret_key_var),
                (
                    type_access_key_var.to_string(),
                    type_secret_key_var.to_string(),
                ),
            ],
        })
    }

    fn bucket_name(&self) -> String {
        self.bucket_name.clone()
    }

    async fn write_storage_guid(&self) -> Result<()> {
        let guid_str = self.guid().to_string();
        let guid_bytes = guid_str.as_bytes();
        let bucket = self.get_bucket()?;
        let response = bucket
            .put_object(
                format!("{}/{}", self.storage_prefix(), XVC_STORAGE_GUID_FILENAME),
                guid_bytes,
            )
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::S3Error { source: err }),
        }
    }

    fn build_storage_path(&self, cache_path: &XvcCachePath) -> XvcStoragePath {
        XvcStoragePath::from(format!(
            "{}/{}/{}/{}",
            self.bucket_name(),
            self.storage_prefix(),
            self.guid(),
            cache_path
        ))
    }

    fn region(&self) -> String {
        self.region.clone()
    }
}
