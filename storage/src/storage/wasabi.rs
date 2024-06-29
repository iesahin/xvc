//! Wasabi storage implementation.
use std::env;

use s3::creds::Credentials;
use s3::{Bucket, Region};
use serde::{Deserialize, Serialize};
use xvc_core::XvcCachePath;
use xvc_ecs::R1NStore;
use xvc_logging::{watch, XvcOutputSender};

use crate::{Result, XvcStorage, XvcStorageEvent};
use crate::{XvcStorageGuid, XvcStorageOperations};

use anyhow::anyhow;

use super::async_common::XvcS3StorageOperations;
use super::XvcStoragePath;

/// Configure a new Wasabi remote storage.
///
/// `bucket_name`, `endpoint` and `storage_prefix` sets a URL for the storage
/// location.
///
/// This creates a [XvcWasabiStorage], calls its
/// [init][XvcWasabiStorage::init] function to create/update guid, and
/// saves [XvcStorageInitEvent] and [XvcStorage] in ECS.
pub(crate) fn cmd_new_wasabi(
    output_snd: &XvcOutputSender,
    xvc_root: &xvc_core::XvcRoot,
    name: String,
    bucket_name: String,
    endpoint: String,
    storage_prefix: String,
) -> Result<()> {
    let mut storage = XvcWasabiStorage {
        guid: XvcStorageGuid::new(),
        name,
        endpoint,
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
            XvcStorage::Wasabi(storage.clone()),
            event_e,
            XvcStorageEvent::Init(init_event.clone()),
        );
        Ok(())
    })?;

    Ok(())
}

/// A Wasabi storage configuration.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcWasabiStorage {
    /// Specifies the storage uniquely.
    ///
    /// This is also stored in
    /// `bucket_name.s3.wasabisys.com/storage_prefix/.xvc-guid` to identify the
    /// remote location.
    pub guid: XvcStorageGuid,

    /// Name of the remote to be used in commands.
    ///
    /// It doesn't have to be unique, though in practice setting unique names is
    /// preferred.
    pub name: String,

    /// The endpoint to communicate with the server.
    ///
    /// In most cases this is `s3.wasabisys.com`
    pub endpoint: String,

    /// The bucket name that you created before configuring this.
    pub bucket_name: String,

    /// The directory in the bucket that Xvc will use.
    ///
    /// Xvc checks the presence of Guid file before creating this folder.
    pub storage_prefix: String,
}

impl XvcWasabiStorage {
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
            Some(&env::var("WASABI_ACCESS_KEY_ID").unwrap()),
            Some(&env::var("WASABI_SECRET_ACCESS_KEY").unwrap()),
            None,
            None,
            None,
        )
        .map_err(|e| e.into())
    }
}

impl XvcS3StorageOperations for XvcWasabiStorage {
    fn storage_prefix(&self) -> String {
        self.storage_prefix.clone()
    }

    fn guid(&self) -> &XvcStorageGuid {
        &self.guid
    }
    fn bucket_name(&self) -> String {
        self.bucket_name.clone()
    }

    fn region(&self) -> String {
        self.endpoint.clone()
    }

    fn get_bucket(&self) -> Result<Bucket> {
        // We'll just put guid file to endpoint/bucket/prefix/XVC_GUID_FILENAME
        let credentials = self.credentials()?;
        let region: Region = Region::Custom {
            region: "".to_string(),
            endpoint: self.endpoint.clone(),
        };

        let bucket = Bucket::new(&self.bucket_name, region, credentials)?;
        watch!(bucket);
        Ok(bucket)
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

    fn build_storage_path(&self, cache_path: &XvcCachePath) -> XvcStoragePath {
        XvcStoragePath::from(format!(
            "{}/{}/{}",
            self.storage_prefix,
            self.guid(),
            cache_path
        ))
    }
}
