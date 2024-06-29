//! Google Cloud Storage remote
use std::env;

use s3::creds::Credentials;
use s3::{Bucket, Region};
use serde::{Deserialize, Serialize};
use xvc_ecs::R1NStore;
use xvc_logging::{watch, XvcOutputSender};

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

    watch!(storage);

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

impl XvcGcsStorage {
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
            Some(&env::var("GCS_ACCESS_KEY_ID").unwrap()),
            Some(&env::var("GCS_SECRET_ACCESS_KEY").unwrap()),
            None,
            None,
            None,
        )
        .map_err(|e| e.into())
    }
}

impl XvcS3StorageOperations for XvcGcsStorage {
    fn storage_prefix(&self) -> String {
        self.storage_prefix.clone()
    }

    fn guid(&self) -> &XvcStorageGuid {
        &self.guid
    }
    fn get_bucket(&self) -> Result<Bucket> {
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
}
