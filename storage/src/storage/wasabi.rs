//! Wasabi storage implementation.
use std::str::FromStr;
use std::{env, fs};

use futures::StreamExt;
use regex::Regex;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use xvc_core::XvcCachePath;
use xvc_ecs::R1NStore;
use xvc_logging::{error, info, watch, XvcOutputSender};

use crate::storage::XVC_STORAGE_GUID_FILENAME;
use crate::{Error, Result, XvcStorage, XvcStorageEvent};
use crate::{XvcStorageGuid, XvcStorageOperations};

use anyhow::anyhow;

use super::{
    XvcStorageDeleteEvent, XvcStorageInitEvent, XvcStorageListEvent, XvcStoragePath,
    XvcStorageReceiveEvent, XvcStorageSendEvent, XvcStorageTempDir,
};

/// Configure a new Wasabi storage remote.
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
    let remote = XvcWasabiStorage {
        guid: XvcStorageGuid::new(),
        name,
        endpoint,
        bucket_name,
        storage_prefix,
    };

    watch!(remote);

    let (init_event, remote) = remote.init(output_snd, xvc_root)?;
    watch!(init_event);

    xvc_root.with_r1nstore_mut(|store: &mut R1NStore<XvcStorage, XvcStorageEvent>| {
        let store_e = xvc_root.new_entity();
        let event_e = xvc_root.new_entity();
        store.insert(
            store_e,
            XvcStorage::Wasabi(remote.clone()),
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
            Some(&env::var("WASABI_ACCESS_KEY_ID").unwrap()),
            Some(&env::var("WASABI_SECRET_ACCESS_KEY").unwrap()),
            None,
            None,
            None,
        )
        .map_err(|e| e.into())
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
        let region: Region = Region::Custom {
            region: "".to_string(),
            endpoint: self.endpoint.clone(),
        };

        let bucket = Bucket::new(&self.bucket_name, region, credentials)?;
        watch!(bucket);
        Ok(bucket)
    }

    async fn a_init(
        self,
        output: &XvcOutputSender,
        _xvc_root: &xvc_core::XvcRoot,
    ) -> Result<(XvcStorageInitEvent, Self)> {
        let bucket = self.get_bucket()?;
        let guid = self.guid.clone();
        let guid_str = self.guid.to_string();
        let guid_bytes = guid_str.as_bytes();

        watch!(bucket);
        watch!(guid);
        watch!(guid_str);

        let res_response = bucket
            .put_object(
                format!("{}/{}", self.storage_prefix, XVC_STORAGE_GUID_FILENAME),
                guid_bytes,
            )
            .await;

        match res_response {
            Ok(_) => Ok((XvcStorageInitEvent { guid }, self)),
            Err(err) => {
                error!(output, "Error while initializing Wasabi storage: {}", err);
                Err(Error::S3Error { source: err })
            }
        }
    }

    async fn a_list(
        &self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
    ) -> Result<XvcStorageListEvent> {
        let bucket = self.get_bucket()?;
        let xvc_guid = xvc_root.config().guid().unwrap();
        let prefix = self.storage_prefix.clone();

        let res_list = bucket
            .list(
                format!("{}/{}", self.storage_prefix, xvc_guid),
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

    fn build_remote_path(&self, repo_guid: &str, cache_path: &XvcCachePath) -> XvcStoragePath {
        XvcStoragePath::from(format!(
            "{}/{}/{}",
            self.storage_prefix, repo_guid, cache_path
        ))
    }

    async fn a_send(
        &self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
        paths: &[xvc_core::XvcCachePath],
        _force: bool,
    ) -> crate::Result<super::XvcStorageSendEvent> {
        let repo_guid = xvc_root
            .config()
            .guid()
            .ok_or_else(|| crate::Error::NoRepositoryGuidFound)?;
        let mut copied_paths = Vec::<XvcStoragePath>::new();

        let bucket = self.get_bucket()?;

        for cache_path in paths {
            watch!(cache_path);
            let remote_path = self.build_remote_path(&repo_guid, cache_path);
            let abs_cache_path = cache_path.to_absolute_path(xvc_root);
            watch!(abs_cache_path);

            let mut path = tokio::fs::File::open(&abs_cache_path).await?;
            watch!(path);

            let res_response = bucket
                .put_object_stream(&mut path, remote_path.as_str())
                .await;

            match res_response {
                Ok(_) => {
                    info!(output, "{} -> {}", abs_cache_path, remote_path.as_str());
                    copied_paths.push(remote_path);
                    watch!(copied_paths.len());
                }
                Err(err) => {
                    error!(output, "{}", err);
                }
            }
        }

        Ok(XvcStorageSendEvent {
            guid: self.guid.clone(),
            paths: copied_paths,
        })
    }

    async fn a_receive(
        &self,
        output_snd: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
        paths: &[xvc_core::XvcCachePath],
        _force: bool,
    ) -> Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> {
        let repo_guid = xvc_root
            .config()
            .guid()
            .ok_or_else(|| crate::Error::NoRepositoryGuidFound)?;
        let mut copied_paths = Vec::<XvcStoragePath>::new();

        let bucket = self.get_bucket()?;
        let temp_dir = XvcStorageTempDir::new()?;

        for cache_path in paths {
            watch!(cache_path);
            let remote_path = self.build_remote_path(&repo_guid, cache_path);
            let abs_cache_dir = temp_dir.temp_cache_dir(cache_path)?;
            fs::create_dir_all(&abs_cache_dir)?;
            let abs_cache_path = temp_dir.temp_cache_path(cache_path)?;
            watch!(abs_cache_path);

            let response_data_stream = bucket.get_object_stream(remote_path.as_str()).await;

            match response_data_stream {
                Ok(mut response) => {
                    info!(output_snd, "{} -> {}", remote_path.as_str(), abs_cache_path);
                    let mut async_cache_path = tokio::fs::File::create(&abs_cache_path).await?;
                    while let Some(chunk) = response.bytes().next().await {
                        async_cache_path.write_all(&chunk).await?;
                    }
                    copied_paths.push(remote_path);
                    watch!(copied_paths.len());
                }
                Err(err) => {
                    error!(output_snd, "{}", err);
                }
            }
        }

        Ok((
            temp_dir,
            XvcStorageReceiveEvent {
                guid: self.guid.clone(),
                paths: copied_paths,
            },
        ))
    }

    async fn a_delete(
        &self,
        output_snd: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent> {
        let repo_guid = xvc_root
            .config()
            .guid()
            .ok_or_else(|| crate::Error::NoRepositoryGuidFound)?;
        let mut deleted_paths = Vec::<XvcStoragePath>::new();

        let bucket = self.get_bucket()?;

        for cache_path in paths {
            watch!(cache_path);
            let remote_path = self.build_remote_path(&repo_guid, cache_path);
            bucket.delete_object(remote_path.as_str()).await?;
            info!(output_snd, "[DELETE] {}", remote_path.as_str());
            deleted_paths.push(remote_path);
        }

        Ok(XvcStorageDeleteEvent {
            guid: self.guid.clone(),
            paths: deleted_paths,
        })
    }
}

impl XvcStorageOperations for XvcWasabiStorage {
    fn init(
        self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
    ) -> Result<(XvcStorageInitEvent, Self)> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        watch!(rt);
        rt.block_on(self.a_init(output, xvc_root))
    }

    /// List the bucket contents that start with `self.storage_prefix`
    fn list(
        &self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
    ) -> crate::Result<super::XvcStorageListEvent> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(self.a_list(output, xvc_root))
    }

    fn send(
        &self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
        paths: &[xvc_core::XvcCachePath],
        force: bool,
    ) -> crate::Result<super::XvcStorageSendEvent> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(self.a_send(output, xvc_root, paths, force))
    }

    fn receive(
        &self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
        paths: &[xvc_core::XvcCachePath],
        force: bool,
    ) -> crate::Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(self.a_receive(output, xvc_root, paths, force))
    }

    fn delete(
        &self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
        paths: &[xvc_core::XvcCachePath],
    ) -> crate::Result<super::XvcStorageDeleteEvent> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(self.a_delete(output, xvc_root, paths))
    }
}
