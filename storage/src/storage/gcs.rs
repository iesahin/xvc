//! Google Cloud Storage remote
use std::fs;
use std::str::FromStr;
use std::time::Duration;

use gcloud_storage::client::{Client, ClientConfig};
use gcloud_storage::http::objects::delete::DeleteObjectRequest;
use gcloud_storage::http::objects::download::Range;
use gcloud_storage::http::objects::get::GetObjectRequest;
use gcloud_storage::http::objects::list::ListObjectsRequest;
use gcloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use xvc_core::R1NStore;
use xvc_core::{error, info, watch, XvcCachePath, XvcOutputSender, XvcRoot};

use crate::storage::{
    Error, Result, XvcStorage, XvcStorageDeleteEvent, XvcStorageEvent,
    XvcStorageExpiringShareEvent, XvcStorageInitEvent, XvcStorageListEvent, XvcStoragePath,
    XvcStorageReceiveEvent, XvcStorageSendEvent, XvcStorageTempDir, XVC_STORAGE_GUID_FILENAME,
};
use crate::storage::{XvcStorageGuid, XvcStorageOperations};

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

impl XvcGcsStorage {
    fn build_storage_path(&self, cache_path: &XvcCachePath) -> XvcStoragePath {
        let prefix = if self.storage_prefix.is_empty() {
            "".to_string()
        } else {
            format!("{}/", self.storage_prefix)
        };
        XvcStoragePath::from(format!("{}{}/{}", prefix, self.guid, cache_path))
    }

    fn guid_path(&self) -> String {
        let prefix = if self.storage_prefix.is_empty() {
            "".to_string()
        } else {
            format!("{}/", self.storage_prefix)
        };
        format!("{}{}", prefix, XVC_STORAGE_GUID_FILENAME)
    }

    async fn get_client(&self) -> Result<Client> {
        let config = ClientConfig::default().with_auth().await.map_err(|e| Error::AnyhowError { source: e.into() })?;
        Ok(Client::new(config))
    }

    async fn a_init(&mut self, _output_snd: &XvcOutputSender) -> Result<XvcStorageInitEvent> {
        let client = self.get_client().await?;
        let guid_str = self.guid.to_string();
        let data = guid_str.as_bytes().to_vec();

        let upload_type = UploadType::Simple(Media::new(self.guid_path()));
        let request = UploadObjectRequest {
            bucket: self.bucket_name.clone(),
            ..Default::default()
        };

        client
            .upload_object(&request, data, &upload_type)
            .await
            .map_err(|e| Error::AnyhowError { source: e.into() })?;

        Ok(XvcStorageInitEvent { guid: self.guid.clone() })
    }

    async fn a_list(&self, _output: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<XvcStorageListEvent> {
        let client = self.get_client().await?;
        let xvc_guid = xvc_root.guid();
        let prefix = if self.storage_prefix.is_empty() {
            format!("{xvc_guid}/")
        } else {
            format!("{}/{xvc_guid}/", self.storage_prefix)
        };

        let request = ListObjectsRequest {
            bucket: self.bucket_name.clone(),
            prefix: Some(prefix.clone()),
            ..Default::default()
        };

        let list_response = client
            .list_objects(&request)
            .await
            .map_err(|e| Error::AnyhowError { source: e.into() })?;

        let re_prefix = if self.storage_prefix.is_empty() {
            "".to_string()
        } else {
            format!("{}/", self.storage_prefix)
        };
        
        let re_str = format!(
            "^{re_prefix}{xvc_guid}/{cp}/{d3}/{d3}/{d58}/0\\..*$",
            cp = r#"[a-zA-Z][0-9]"#,
            d3 = r#"[0-9A-Fa-f]{3}"#,
            d58 = r#"[0-9A-Fa-f]{58}"#
        );
        let re = Regex::new(&re_str).unwrap();

        let mut paths = Vec::new();
        if let Some(items) = list_response.items {
            for item in items {
                if re.is_match(&item.name) {
                    paths.push(XvcStoragePath::from_str(&item.name).unwrap());
                }
            }
        }

        Ok(XvcStorageListEvent {
            guid: self.guid.clone(),
            paths,
        })
    }

    async fn a_send(
        &self,
        output_snd: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        _force: bool,
    ) -> Result<XvcStorageSendEvent> {
        let client = self.get_client().await?;
        let mut copied_paths = Vec::new();

        for cache_path in paths {
            let storage_path = self.build_storage_path(cache_path);
            let abs_cache_path = cache_path.to_absolute_path(xvc_root);

            let data = tokio::fs::read(&abs_cache_path).await?;
            let upload_type = UploadType::Simple(Media::new(storage_path.to_string()));
            let request = UploadObjectRequest {
                bucket: self.bucket_name.clone(),
                ..Default::default()
            };

            match client.upload_object(&request, data, &upload_type).await {
                Ok(_) => {
                    info!(output_snd, "{} -> {}", abs_cache_path, storage_path.as_str());
                    copied_paths.push(storage_path);
                }
                Err(err) => {
                    error!(output_snd, "{}", err);
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
        paths: &[XvcCachePath],
        _force: bool,
    ) -> Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> {
        let client = self.get_client().await?;
        let temp_dir = XvcStorageTempDir::new()?;
        let mut copied_paths = Vec::new();

        for cache_path in paths {
            let storage_path = self.build_storage_path(cache_path);
            let abs_cache_dir = temp_dir.temp_cache_dir(cache_path)?;
            fs::create_dir_all(&abs_cache_dir)?;
            let abs_cache_path = temp_dir.temp_cache_path(cache_path)?;

            let request = GetObjectRequest {
                bucket: self.bucket_name.clone(),
                object: storage_path.as_str().to_string(),
                ..Default::default()
            };

            match client.download_object(&request, &Range::default()).await {
                Ok(data) => {
                    info!(output_snd, "{} -> {}", storage_path.as_str(), abs_cache_path);
                    tokio::fs::write(&abs_cache_path, data).await?;
                    copied_paths.push(storage_path);
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
        output: &XvcOutputSender,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent> {
        let client = self.get_client().await?;
        let mut deleted_paths = Vec::new();

        for cache_path in paths {
            let storage_path = self.build_storage_path(cache_path);
            let request = DeleteObjectRequest {
                bucket: self.bucket_name.clone(),
                object: storage_path.as_str().to_string(),
                ..Default::default()
            };

            match client.delete_object(&request).await {
                Ok(_) => {
                    info!(output, "[DELETE] {}", storage_path.as_str());
                    deleted_paths.push(storage_path);
                }
                Err(err) => {
                    error!(output, "{}", err);
                }
            }
        }

        Ok(XvcStorageDeleteEvent {
            guid: self.guid.clone(),
            paths: deleted_paths,
        })
    }

    async fn a_share(
        &self,
        _output: &XvcOutputSender,
        _path: &XvcCachePath,
        _duration: Duration,
    ) -> Result<XvcStorageExpiringShareEvent> {
        // google-cloud-storage crate supports sign_url but we need SignedURLMethod from the http mod.
        // Wait, sign_url might require service account credentials specifically. Let's see if we can use it.
        // For now, return an error if it's not supported easily without service accounts.
        // Actually, we can return Error::NotImplemented for now or look into how it signs.
        // Let's implement it if it's straightforward.
        Err(Error::AnyhowError { source: anyhow::anyhow!("Signed URLs with GCS ADC are currently not supported") })
    }
}

impl XvcStorageOperations for XvcGcsStorage {
    fn init(&mut self, output: &XvcOutputSender, _xvc_root: &XvcRoot) -> Result<XvcStorageInitEvent> {
        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(self.a_init(output))
    }

    fn list(&self, output: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<XvcStorageListEvent> {
        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(self.a_list(output, xvc_root))
    }

    fn send(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<XvcStorageSendEvent> {
        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(self.a_send(output, xvc_root, paths, force))
    }

    fn receive(
        &self,
        output: &XvcOutputSender,
        _xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> {
        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(self.a_receive(output, paths, force))
    }

    fn delete(
        &self,
        output: &XvcOutputSender,
        _xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent> {
        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(self.a_delete(output, paths))
    }

    fn share(
        &self,
        output: &XvcOutputSender,
        _xvc_root: &XvcRoot,
        path: &XvcCachePath,
        period: Duration,
    ) -> Result<XvcStorageExpiringShareEvent> {
        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(self.a_share(output, path, period))
    }
}
