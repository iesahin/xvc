//! Google Cloud Storage remote
use std::env;

use s3::creds::Credentials;
use s3::{Bucket, Region};
use serde::{Deserialize, Serialize};
use xvc_core::R1NStore;
use xvc_core::{info, watch, XvcOutputSender};

use anyhow::anyhow;

use crate::{Result, XvcStorage, XvcStorageEvent};
use crate::{XvcStorageGuid, XvcStorageOperations};

use super::async_common::XvcS3StorageOperations;

/// Configure a new Google Cloud Storage remote.
///
/// `bucket_name`, `region` and `storage_prefix` sets a URL for the storage
/// location.
///
/// This creates a [XvcGcsStorage], calls its
/// [init][XvcGcsStorage::init] function to create/update guid, and
/// saves [XvcStorageInitEvent] and [XvcStorage] in ECS.
pub fn cmd_new_gcs(
    _input: std::io::StdinLock,
    output_snd: &XvcOutputSender,
    xvc_root: &xvc_core::XvcRoot,
    name: String,
    bucket_name: String,
    region: String,
    storage_prefix: String,
) -> Result<()> {
    let mut storage = XvcGcsStorage {
        guid: XvcStorageGuid::new(),
        name,
        region,
        bucket_name,
        storage_prefix,
    };

    info!(output_snd, "New Storage: {:#?}", storage);

    let init_event = storage.init(output_snd, xvc_root)?;
    watch!(init_event);

    xvc_root.with_r1nstore_mut(|store: &mut R1NStore<XvcStorage, XvcStorageEvent>| {
        let store_e = xvc_root.new_entity();
        let event_e = xvc_root.new_entity();
        store.insert(
            store_e,
            XvcStorage::Gcs(storage.clone()),
            event_e,
            XvcStorageEvent::Init(init_event.clone()),
        );
        Ok(())
    })?;

    Ok(())
}

/// A Google Cloud Storage remote.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcGcsStorage {
    /// The unique identifier of the storage.
    pub guid: XvcStorageGuid,
    /// The name of the storage.
    pub name: String,
    /// The region of the storage.
    pub region: String,
    /// The bucket name of the storage.
    pub bucket_name: String,
    /// The path prefix on the storage.
    pub storage_prefix: String,
}

impl XvcS3StorageOperations for XvcGcsStorage {
    fn storage_prefix(&self) -> String {
        self.storage_prefix.clone()
    }

    fn guid(&self) -> &XvcStorageGuid {
        &self.guid
    }
    fn get_bucket(&self) -> Result<Box<Bucket>> {
        let credentials = self.credentials()?;
        let region = Region::Custom {
            region: self.region.to_owned(),
            endpoint: "https://storage.googleapis.com".to_owned(),
        };
        let mut bucket = Bucket::new(&self.bucket_name, region, credentials)?;
        bucket.set_listobjects_v1();
        Ok(bucket)
    }

    fn bucket_name(&self) -> String {
        self.bucket_name.clone()
    }

    fn region(&self) -> String {
        self.region.clone()
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
        let type_access_key_var = "GCS_ACCESS_KEY_ID";
        let type_secret_key_var = "GCS_SECRET_ACCESS_KEY";
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
}
