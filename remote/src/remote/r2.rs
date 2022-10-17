use std::str::FromStr;
use std::{env, fs};

use crossbeam_channel::Sender;
use regex::Regex;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use serde::{Deserialize, Serialize};
use xvc_core::{XvcCachePath, XvcRoot};
use xvc_ecs::R1NStore;
use xvc_logging::{watch, XvcOutputLine};

use crate::remote::XVC_REMOTE_GUID_FILENAME;
use crate::{Error, Result, XvcStorage, XvcStorageEvent};
use crate::{XvcStorageGuid, XvcStorageOperations};

use super::{
    XvcRemotePath, XvcStorageDeleteEvent, XvcStorageInitEvent, XvcStorageListEvent,
    XvcStorageReceiveEvent, XvcStorageSendEvent,
};

pub fn cmd_new_r2(
    input: std::io::StdinLock,
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    name: String,
    account_id: String,
    bucket_name: String,
    remote_prefix: String,
) -> Result<()> {
    let remote = XvcR2Remote {
        guid: XvcStorageGuid::new(),
        name,
        account_id,
        bucket_name,
        remote_prefix,
    };

    watch!(remote);

    let init_event = remote.init(output_snd.clone(), xvc_root)?;
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

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcR2Remote {
    pub guid: XvcStorageGuid,
    pub name: String,
    pub account_id: String,
    pub bucket_name: String,
    pub remote_prefix: String,
}

impl XvcR2Remote {
    fn credentials(&self) -> Result<Credentials> {
        Credentials::new(
            Some(&env::var("XVC_REMOTE_ACCESS_KEY_ID").unwrap()),
            Some(&env::var("XVC_REMOTE_SECRET_KEY").unwrap()),
            env::var("XVC_REMOTE_SECURITY_TOKEN").as_deref().ok(),
            env::var("XVC_REMOTE_SESSION_TOKEN").as_deref().ok(),
            env::var("XVC_REMOTE_PROFILE").as_deref().ok(),
        )
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

    async fn a_init(
        &self,
        output: crossbeam_channel::Sender<xvc_logging::XvcOutputLine>,
        xvc_root: &xvc_core::XvcRoot,
    ) -> Result<XvcStorageInitEvent> {
        let bucket = self.get_bucket()?;
        let guid = self.guid.clone();
        let guid_str = self.guid.to_string();
        let guid_bytes = guid_str.as_bytes();

        watch!(bucket);
        watch!(guid);
        watch!(guid_str);

        let res_response = bucket
            .put_object(
                format!("{}/{}", self.remote_prefix, XVC_REMOTE_GUID_FILENAME),
                guid_bytes,
            )
            .await;

        match res_response {
            Ok(_) => Ok(XvcStorageInitEvent { guid }),
            Err(err) => {
                output.send(xvc_logging::XvcOutputLine::Error(err.to_string()))?;
                Err(Error::S3Error { source: err })
            }
        }
    }

    async fn a_list(
        &self,
        output: crossbeam_channel::Sender<xvc_logging::XvcOutputLine>,
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
                            Some(XvcRemotePath::from_str(&e.name).unwrap())
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
                output.send(xvc_logging::XvcOutputLine::Error(err.to_string()))?;
                Err(Error::S3Error { source: err })
            }
        }
    }

    fn build_remote_path(&self, repo_guid: &str, cache_path: &XvcCachePath) -> XvcRemotePath {
        let remote_path = XvcRemotePath::from(format!(
            "{}/{}/{}",
            self.remote_prefix, repo_guid, cache_path
        ));

        remote_path
    }

    async fn a_send(
        &self,
        output: crossbeam_channel::Sender<xvc_logging::XvcOutputLine>,
        xvc_root: &xvc_core::XvcRoot,
        paths: &[xvc_core::XvcCachePath],
        force: bool,
    ) -> crate::Result<super::XvcStorageSendEvent> {
        let repo_guid = xvc_root
            .config()
            .guid()
            .ok_or_else(|| crate::Error::NoRepositoryGuidFound)?;
        let mut copied_paths = Vec::<XvcRemotePath>::new();

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
                    output
                        .send(XvcOutputLine::Info(format!(
                            "{} -> {}",
                            abs_cache_path,
                            remote_path.as_str()
                        )))
                        .unwrap();
                    copied_paths.push(remote_path);
                    watch!(copied_paths.len());
                }
                Err(err) => {
                    output.send(xvc_logging::XvcOutputLine::Error(err.to_string()))?;
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
        output: crossbeam_channel::Sender<XvcOutputLine>,
        xvc_root: &xvc_core::XvcRoot,
        paths: &[xvc_core::XvcCachePath],
        force: bool,
    ) -> Result<XvcStorageReceiveEvent> {
        let repo_guid = xvc_root
            .config()
            .guid()
            .ok_or_else(|| crate::Error::NoRepositoryGuidFound)?;
        let mut copied_paths = Vec::<XvcRemotePath>::new();

        let bucket = self.get_bucket()?;

        for cache_path in paths {
            watch!(cache_path);
            let remote_path = self.build_remote_path(&repo_guid, cache_path);
            let abs_cache_path = cache_path.to_absolute_path(xvc_root);
            watch!(abs_cache_path);
            let abs_cache_dir = abs_cache_path.parent().unwrap();
            fs::create_dir_all(&abs_cache_dir)?;
            let mut async_cache_path = tokio::fs::File::create(&abs_cache_path).await?;

            let response = bucket
                .get_object_stream(remote_path.as_str(), &mut async_cache_path)
                .await;

            match response {
                Ok(_) => {
                    output
                        .send(XvcOutputLine::Info(format!(
                            "{} -> {}",
                            remote_path.as_str(),
                            abs_cache_path,
                        )))
                        .unwrap();
                    copied_paths.push(remote_path);
                    watch!(copied_paths.len());
                }
                Err(err) => {
                    output.send(XvcOutputLine::Error(err.to_string())).unwrap();
                }
            }
        }

        Ok(XvcStorageReceiveEvent {
            guid: self.guid.clone(),
            paths: copied_paths,
        })
    }

    async fn a_delete(
        &self,
        output: crossbeam_channel::Sender<XvcOutputLine>,
        xvc_root: &xvc_core::XvcRoot,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent> {
        todo!();
    }
}

impl XvcStorageOperations for XvcR2Remote {
    fn init(
        &self,
        output: crossbeam_channel::Sender<xvc_logging::XvcOutputLine>,
        xvc_root: &xvc_core::XvcRoot,
    ) -> Result<XvcStorageInitEvent> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        watch!(rt);
        rt.block_on(self.a_init(output, xvc_root))
    }

    /// List the bucket contents that start with `self.remote_prefix`
    fn list(
        &self,
        output: crossbeam_channel::Sender<xvc_logging::XvcOutputLine>,
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
        output: crossbeam_channel::Sender<xvc_logging::XvcOutputLine>,
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
        output: crossbeam_channel::Sender<xvc_logging::XvcOutputLine>,
        xvc_root: &xvc_core::XvcRoot,
        paths: &[xvc_core::XvcCachePath],
        force: bool,
    ) -> crate::Result<super::XvcStorageReceiveEvent> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(self.a_receive(output, xvc_root, paths, force))
    }

    fn delete(
        &self,
        output: crossbeam_channel::Sender<xvc_logging::XvcOutputLine>,
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
