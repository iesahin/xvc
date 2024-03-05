//! Cloudflare R2 remote storage implementation.
use std::str::FromStr;
use std::{env, fs};

use futures::StreamExt;
use regex::Regex;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use xvc_core::{XvcCachePath, XvcRoot};
use xvc_ecs::R1NStore;
use xvc_logging::{error, info, watch, XvcOutputSender};

use crate::storage::XVC_STORAGE_GUID_FILENAME;
use crate::{Error, Result, XvcStorage, XvcStorageEvent};
use crate::{XvcStorageGuid, XvcStorageOperations};
use anyhow::anyhow;

use super::async_common::XvcS3StorageOperations;
use super::minio::XvcMinioStorage;
use super::{
    XvcStorageDeleteEvent, XvcStorageInitEvent, XvcStorageListEvent, XvcStoragePath,
    XvcStorageReceiveEvent, XvcStorageSendEvent, XvcStorageTempDir,
};

/// Configure a new Cloudflare R2 remote storage.
///
/// `account_id`, `bucket_name`, and `remote_prefix` sets a URL for the
/// storage location.
///
/// This creates a [XvcR2Storage], calls its
/// [init][XvcR2Storage::init] function to create/update guid, and
/// saves [XvcStorageInitEvent] and [XvcStorage] in ECS.
pub fn cmd_new_r2(
    _input: std::io::StdinLock,
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    name: String,
    account_id: String,
    bucket_name: String,
    remote_prefix: String,
) -> Result<()> {
    let remote = XvcR2Storage {
        guid: XvcStorageGuid::new(),
        name,
        account_id,
        bucket_name,
        remote_prefix,
    };

    watch!(remote);

    let (init_event, remote) = remote.init(output_snd, xvc_root)?;
    watch!(init_event);

    xvc_root.with_r1nstore_mut(|store: &mut R1NStore<XvcStorage, XvcStorageEvent>| {
        let store_e = xvc_root.new_entity();
        let event_e = xvc_root.new_entity();
        store.insert(
            store_e,
            XvcStorage::R2(remote.clone()),
            event_e,
            XvcStorageEvent::Init(init_event.clone()),
        );
        Ok(())
    })?;

    Ok(())
}

/// A Cloudflare R2 remote storage
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcR2Storage {
    /// GUID of the storage
    pub guid: XvcStorageGuid,
    /// Name of the storage
    pub name: String,
    /// Account ID of the R2 storage
    pub account_id: String,
    /// Bucket name of the R2 storage
    pub bucket_name: String,
    /// Remote prefix in the bucket
    pub remote_prefix: String,
}

impl XvcR2Storage {
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
            Some(&env::var("R2_ACCESS_KEY_ID").unwrap()),
            Some(&env::var("R2_SECRET_ACCESS_KEY").unwrap()),
            None,
            None,
            None,
        )
        .map_err(|e| e.into())
    }
}

impl XvcS3StorageOperations for XvcR2Storage {
    fn remote_prefix(&self) -> &str {
        &self.remote_prefix
    }

    fn guid(&self) -> &XvcStorageGuid {
        &self.guid
    }
    fn bucket_name(&self) -> &str {
        self.bucket_name.as_str()
    }
    fn build_remote_path(&self, cache_path: &XvcCachePath) -> XvcStoragePath {
        XvcStoragePath::from(format!(
            "{}/{}/{}",
            self.remote_prefix, self.repo_guid, cache_path
        ))
    }

    /// This doesn't apply to R2
    fn region(&self) -> &str {
        self.account_id.as_str()
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
        let region = Region::R2 {
            account_id: self.account_id.clone(),
        };
        let bucket = Bucket::new(&self.bucket_name, region, credentials)?;
        Ok(bucket)
    }

    async fn a_list(
        &self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
    ) -> Result<XvcStorageListEvent> {
        let bucket = self.get_bucket()?;
        let xvc_guid = xvc_root.config().guid().unwrap();
        let prefix = self.remote_prefix.clone();

        let res_list = bucket
            .list(
                format!("{}/{}", self.remote_prefix, xvc_guid),
                Some("/".to_string()),
            )
            .await;

        match res_list {
            Ok(list_all) => {
                // select only the matching elements
                let re = Regex::new(&format!(
                    "{prefix}/{xvc_guid}/{cp}/{d3}/{d3}/{d58}/0\\..*$",
                    cp = r#"[a-zA-Z][0-9]"#,
                    d3 = r#"[0-9A-Fa-f]{3}"#,
                    d58 = r#"[0-9A-Fa-f]{58}"#
                ))
                .unwrap();

                let paths = list_all
                    .iter()
                    .filter_map(|e| {
                        if re.is_match(e.name.as_ref()) {
                            Some(XvcStoragePath::from_str(&e.name).unwrap())
                        } else {
                            None
                        }
                    })
                    .collect();

                Ok(XvcStorageListEvent {
                    guid: self.guid.clone(),
                    paths,
                })
            }

            Err(err) => {
                error!(output, "{}", err);
                Err(Error::S3Error { source: err })
            }
        }
    }
}
