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
/// `bucket_name`, `region` and `remote_prefix` sets a URL for the storage
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
    remote_prefix: String,
) -> Result<()> {
    let mut remote = XvcGcsStorage {
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
            XvcStorage::Gcs(remote.clone()),
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
    /// The unique identifier of the remote.
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

impl XvcGcsStorage {
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
    fn remote_prefix(&self) -> String {
        self.remote_prefix.clone()
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
}
// async fn a_init(
//     self,
//     output_snd: &XvcOutputSender,
//     _xvc_root: &xvc_core::XvcRoot,
// ) -> Result<(XvcStorageInitEvent, Self)> {
//     let bucket = self.get_bucket()?;
//     let guid = self.guid.clone();
//     let guid_str = self.guid.to_string();
//     let guid_bytes = guid_str.as_bytes();
//
//     watch!(bucket);
//     watch!(guid);
//     watch!(guid_str);
//
//     let res_response = bucket
//         .put_object(
//             format!("{}/{}", self.remote_prefix, XVC_STORAGE_GUID_FILENAME),
//             guid_bytes,
//         )
//         .await;
//
//     match res_response {
//         Ok(_) => Ok((XvcStorageInitEvent { guid }, self)),
//         Err(err) => {
//             error!(output_snd, "Error: {}", err);
//             Err(Error::S3Error { source: err })
//         }
//     }
// }
//
// async fn a_list(
//     &self,
//     output_snd: &XvcOutputSender,
//     xvc_root: &xvc_core::XvcRoot,
// ) -> Result<XvcStorageListEvent> {
//     let credentials = self.credentials()?;
//     let region = Region::from_str(&self.region).unwrap_or("us-east-1".parse().unwrap());
//     let bucket = Bucket::new(&self.bucket_name, region, credentials)?;
//     let _guid = self.guid.clone();
//     let guid_str = self.guid.to_string();
//     let _guid_bytes = guid_str.as_bytes();
//     let xvc_guid = xvc_root.config().guid().unwrap();
//     let prefix = self.remote_prefix.clone();
//
//     let res_list = bucket
//         .list(
//             format!("{}/{}", self.remote_prefix, xvc_guid),
//             Some("/".to_string()),
//         )
//         .await;
//
//     match res_list {
//         Ok(list_all) => {
//             // select only the matching elements
//             let re = Regex::new(&format!(
//                 "{prefix}/{xvc_guid}/{cp}/{d3}/{d3}/{d58}/0\\..*$",
//                 cp = r#"[a-zA-Z][0-9]"#,
//                 d3 = r#"[0-9A-Fa-f]{3}"#,
//                 d58 = r#"[0-9A-Fa-f]{58}"#
//             ))
//             .unwrap();
//
//             let paths = list_all
//                 .iter()
//                 .filter_map(|e| {
//                     if re.is_match(e.name.as_ref()) {
//                         Some(XvcStoragePath::from_str(&e.name).unwrap())
//                     } else {
//                         None
//                     }
//                 })
//                 .collect();
//
//             Ok(XvcStorageListEvent {
//                 guid: self.guid.clone(),
//                 paths,
//             })
//         }
//
//         Err(err) => {
//             error!(output_snd, "Error: {}", err);
//             Err(Error::S3Error { source: err })
//         }
//     }
// }
//
// fn build_remote_path(&self, repo_guid: &str, cache_path: &XvcCachePath) -> XvcStoragePath {
//     XvcStoragePath::from(format!(
//         "{}/{}/{}",
//         self.remote_prefix, repo_guid, cache_path
//     ))
// }
//
// async fn a_send(
//     &self,
//     output: &XvcOutputSender,
//     xvc_root: &xvc_core::XvcRoot,
//     paths: &[xvc_core::XvcCachePath],
//     _force: bool,
// ) -> crate::Result<super::XvcStorageSendEvent> {
//     let repo_guid = xvc_root
//         .config()
//         .guid()
//         .ok_or_else(|| crate::Error::NoRepositoryGuidFound)?;
//     let mut copied_paths = Vec::<XvcStoragePath>::new();
//
//     let bucket = self.get_bucket()?;
//
//     for cache_path in paths {
//         watch!(cache_path);
//         let remote_path = self.build_remote_path(&repo_guid, cache_path);
//         let abs_cache_path = cache_path.to_absolute_path(xvc_root);
//         watch!(abs_cache_path);
//
//         let mut path = tokio::fs::File::open(&abs_cache_path).await?;
//         watch!(path);
//
//         let res_response = bucket
//             .put_object_stream(&mut path, remote_path.as_str())
//             .await;
//
//         match res_response {
//             Ok(_) => {
//                 info!(output, "{} -> {}", abs_cache_path, remote_path.as_str());
//                 copied_paths.push(remote_path);
//                 watch!(copied_paths.len());
//             }
//             Err(err) => {
//                 error!(output, "Error: {}", err);
//             }
//         }
//     }
//
//     Ok(XvcStorageSendEvent {
//         guid: self.guid.clone(),
//         paths: copied_paths,
//     })
// }
//
// async fn a_receive(
//     &self,
//     output: &XvcOutputSender,
//     xvc_root: &xvc_core::XvcRoot,
//     paths: &[xvc_core::XvcCachePath],
//     _force: bool,
// ) -> Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> {
//     let repo_guid = xvc_root
//         .config()
//         .guid()
//         .ok_or_else(|| crate::Error::NoRepositoryGuidFound)?;
//     let mut copied_paths = Vec::<XvcStoragePath>::new();
//
//     let bucket = self.get_bucket()?;
//     let temp_dir = XvcStorageTempDir::new()?;
//
//     for cache_path in paths {
//         watch!(cache_path);
//         let remote_path = self.build_remote_path(&repo_guid, cache_path);
//         let abs_cache_dir = temp_dir.temp_cache_dir(cache_path)?;
//         fs::create_dir_all(&abs_cache_dir)?;
//         let abs_cache_path = temp_dir.temp_cache_path(cache_path)?;
//         watch!(abs_cache_path);
//
//         let response_data_stream = bucket.get_object_stream(remote_path.as_str()).await;
//
//         match response_data_stream {
//             Ok(mut response) => {
//                 info!(output, "{} -> {}", remote_path.as_str(), abs_cache_path);
//                 let mut async_cache_path = tokio::fs::File::create(&abs_cache_path).await?;
//                 while let Some(chunk) = response.bytes().next().await {
//                     async_cache_path.write_all(&chunk).await?;
//                 }
//                 copied_paths.push(remote_path);
//                 watch!(copied_paths.len());
//             }
//             Err(err) => {
//                 error!(output, "{}", err);
//             }
//         }
//     }
//
//     Ok((
//         temp_dir,
//         XvcStorageReceiveEvent {
//             guid: self.guid.clone(),
//             paths: copied_paths,
//         },
//     ))
// }
//
// async fn a_delete(
//     &self,
//     output: &XvcOutputSender,
//     xvc_root: &xvc_core::XvcRoot,
//     paths: &[XvcCachePath],
// ) -> Result<XvcStorageDeleteEvent> {
//     let repo_guid = xvc_root
//         .config()
//         .guid()
//         .ok_or_else(|| crate::Error::NoRepositoryGuidFound)?;
//     let mut deleted_paths = Vec::<XvcStoragePath>::new();
//
//     let bucket = self.get_bucket()?;
//
//     for cache_path in paths {
//         watch!(cache_path);
//         let remote_path = self.build_remote_path(&repo_guid, cache_path);
//         bucket.delete_object(remote_path.as_str()).await?;
//         info!(output, "[DELETE] {}", remote_path.as_str());
//         deleted_paths.push(remote_path);
//     }
//
//     Ok(XvcStorageDeleteEvent {
//         guid: self.guid.clone(),
//         paths: deleted_paths,
//     })
// }
// }
//
// impl XvcStorageOperations for XvcGcsStorage {
//     fn init(
//         self,
//         output: &XvcOutputSender,
//         xvc_root: &xvc_core::XvcRoot,
//     ) -> Result<(XvcStorageInitEvent, Self)> {
//         let rt = tokio::runtime::Builder::new_multi_thread()
//             .enable_all()
//             .build()?;
//         watch!(rt);
//         rt.block_on(self.a_init(output, xvc_root))
//     }
//
//     /// List the bucket contents that start with `self.remote_prefix`
//     fn list(
//         &self,
//         output: &XvcOutputSender,
//         xvc_root: &xvc_core::XvcRoot,
//     ) -> crate::Result<super::XvcStorageListEvent> {
//         let rt = tokio::runtime::Builder::new_multi_thread()
//             .enable_all()
//             .build()
//             .unwrap();
//         rt.block_on(self.a_list(output, xvc_root))
//     }
//
//     fn send(
//         &self,
//         output: &XvcOutputSender,
//         xvc_root: &xvc_core::XvcRoot,
//         paths: &[xvc_core::XvcCachePath],
//         force: bool,
//     ) -> crate::Result<super::XvcStorageSendEvent> {
//         let rt = tokio::runtime::Builder::new_multi_thread()
//             .enable_all()
//             .build()
//             .unwrap();
//         rt.block_on(self.a_send(output, xvc_root, paths, force))
//     }
//
//     fn receive(
//         &self,
//         output: &XvcOutputSender,
//         xvc_root: &xvc_core::XvcRoot,
//         paths: &[xvc_core::XvcCachePath],
//         force: bool,
//     ) -> crate::Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> {
//         let rt = tokio::runtime::Builder::new_multi_thread()
//             .enable_all()
//             .build()
//             .unwrap();
//         rt.block_on(self.a_receive(output, xvc_root, paths, force))
//     }
//
//     fn delete(
//         &self,
//         output: &XvcOutputSender,
//         xvc_root: &xvc_core::XvcRoot,
//         paths: &[xvc_core::XvcCachePath],
//     ) -> crate::Result<super::XvcStorageDeleteEvent> {
//         let rt = tokio::runtime::Builder::new_multi_thread()
//             .enable_all()
//             .build()
//             .unwrap();
//         rt.block_on(self.a_delete(output, xvc_root, paths))
//     }
// }
