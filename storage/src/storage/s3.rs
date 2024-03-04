//! AWS S3 remote storage implementation
use std::env;

use anyhow::anyhow;

use s3::creds::Credentials;
use s3::{Bucket, Region};
use serde::{Deserialize, Serialize};
use xvc_core::{XvcCachePath, XvcRoot};
use xvc_ecs::R1NStore;
use xvc_logging::{watch, XvcOutputSender};

use crate::storage::XVC_STORAGE_GUID_FILENAME;
use crate::{Error, Result, XvcStorage, XvcStorageEvent};
use crate::{XvcStorageGuid, XvcStorageOperations};

use super::async_common::XvcS3StorageOperations;
use super::XvcStoragePath;

/// Configure a new Amazon Web Services S3 remote storage.
///
/// `bucket_name`, `region` and `remote_prefix` sets a URL for the storage
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
    let mut remote = XvcS3Storage {
        guid: XvcStorageGuid::new(),
        name,
        region,
        bucket_name,
        storage_prefix,
    };

    watch!(remote);

    let init_event = remote.init(output_snd, xvc_root)?;
    watch!(init_event);

    xvc_root.with_r1nstore_mut(|store: &mut R1NStore<XvcStorage, XvcStorageEvent>| {
        let store_e = xvc_root.new_entity();
        let event_e = xvc_root.new_entity();
        store.insert(
            store_e,
            XvcStorage::S3(remote.clone()),
            event_e,
            XvcStorageEvent::Init(init_event.clone()),
        );
        Ok(())
    })?;

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
    /// Name of the remote to be used in commands.
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

impl XvcS3Storage {
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
            Some(&env::var("AWS_ACCESS_KEY_ID").unwrap()),
            Some(&env::var("AWS_SECRET_ACCESS_KEY").unwrap()),
            None,
            None,
            None,
        )
        .map_err(|e| e.into())
    }
}

impl XvcS3StorageOperations for XvcS3Storage {
    fn remote_prefix(&self) -> &str {
        self.storage_prefix.as_str()
    }

    fn guid(&self) -> &XvcStorageGuid {
        &self.guid
    }

    fn get_bucket(&self) -> Result<Bucket> {
        // We'll just put guid file to endpoint/bucket/prefix/XVC_GUID_FILENAME
        let credentials = self.credentials()?;
        let region: Region = self.region.parse().expect("Cannot parse region name");
        let bucket = Bucket::new(&self.bucket_name, region, credentials)?;
        Ok(bucket)
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

    fn bucket_name(&self) -> &str {
        self.bucket_name.as_str()
    }

    async fn write_remote_guid(&self) -> Result<()> {
        let guid_str = self.guid().to_string();
        let guid_bytes = guid_str.as_bytes();
        let bucket = self.get_bucket()?;
        let response = bucket
            .put_object(
                format!("{}/{}", self.remote_prefix(), XVC_STORAGE_GUID_FILENAME),
                guid_bytes,
            )
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::S3Error { source: err }),
        }
    }

    fn build_remote_path(&self, cache_path: &XvcCachePath) -> XvcStoragePath {
        XvcStoragePath::from(format!(
            "{}/{}/{}/{}",
            self.bucket_name(),
            self.remote_prefix(),
            self.guid(),
            cache_path
        ))
    }

    fn region(&self) -> &str {
        self.region().to_string().as_str()
    }
}
