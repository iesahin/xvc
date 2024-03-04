//! Digital Ocean Spaces remote storage implementation.
use anyhow::anyhow;
use std::env;

use s3::creds::Credentials;
use s3::{Bucket, Region};
use serde::{Deserialize, Serialize};
use xvc_core::XvcRoot;
use xvc_ecs::R1NStore;
use xvc_logging::{watch, XvcOutputSender};

use crate::{Result, XvcStorage, XvcStorageEvent};
use crate::{XvcStorageGuid, XvcStorageOperations};

use super::async_common::XvcS3StorageOperations;

/// Configure a new Digital Ocean Spaces remote.
///
/// `bucket_name`, `region` and `remote_prefix` sets a URL for the storage
/// location.
///
/// This creates a [XvcDigitalOceanStorage], calls its
/// [init][XvcDigitalOceanStorage::init] function to create/update guid, and
/// saves [XvcStorageInitEvent] and [XvcStorage] in ECS.
pub fn cmd_new_digital_ocean(
    _input: std::io::StdinLock,
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    name: String,
    bucket_name: String,
    region: String,
    remote_prefix: String,
) -> Result<()> {
    let mut remote = XvcDigitalOceanStorage {
        guid: XvcStorageGuid::new(),
        name,
        region,
        bucket_name,
        remote_prefix,
    };
    watch!(remote);

    let init_event = remote.init(output_snd, xvc_root)?;
    watch!(init_event);

    xvc_root.with_r1nstore_mut(|store: &mut R1NStore<XvcStorage, XvcStorageEvent>| {
        let store_e = xvc_root.new_entity();
        let event_e = xvc_root.new_entity();
        store.insert(
            store_e,
            XvcStorage::DigitalOcean(remote.clone()),
            event_e,
            XvcStorageEvent::Init(init_event.clone()),
        );
        Ok(())
    })?;

    Ok(())
}

/// A Digital Ocean Spaces remote.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcDigitalOceanStorage {
    /// The GUID of the remote.
    pub guid: XvcStorageGuid,
    /// The name of the remote.
    pub name: String,
    /// The region of the remote.
    pub region: String,
    /// The bucket name of the remote.
    pub bucket_name: String,
    /// The remote prefix of the remote.
    pub remote_prefix: String,
}

impl XvcDigitalOceanStorage {
    fn remote_specific_credentials(&self) -> Result<Credentials> {
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
            Some(&env::var("DIGITAL_OCEAN_ACCESS_KEY_ID").unwrap()),
            Some(&env::var("DIGITAL_OCEAN_SECRET_ACCESS_KEY").unwrap()),
            None,
            None,
            None,
        )
        .map_err(|e| e.into())
    }
}

impl XvcS3StorageOperations for XvcDigitalOceanStorage {
    fn remote_prefix(&self) -> &str {
        self.remote_prefix.as_str()
    }

    fn guid(&self) -> &XvcStorageGuid {
        &self.guid
    }

    fn credentials(&self) -> Result<Credentials> {
        match self.remote_specific_credentials() {
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

    fn get_bucket(&self) -> Result<Bucket> {
        // We'll just put guid file to endpoint/bucket/prefix/XVC_GUID_FILENAME
        let credentials = self.credentials()?;
        let region: Region = self.region.parse().expect("Cannot parse region name");
        let bucket = Bucket::new(&self.bucket_name, region, credentials)?;
        Ok(bucket)
    }

    fn bucket_name(&self) -> &str {
        &self.bucket_name
    }

    fn region(&self) -> &str {
        self.region.as_str()
    }
}
