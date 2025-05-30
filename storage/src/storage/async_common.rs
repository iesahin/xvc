//! Home for async operations for S3 compatible storage services.
use std::fs;
use std::str::FromStr;

use futures::StreamExt;
use regex::Regex;
use s3::creds::Credentials;
use s3::Bucket;
use s3::Region;
use tokio::io::AsyncWriteExt;
use xvc_core::XvcCachePath;
use xvc_core::XvcRoot;
use xvc_core::error;
use xvc_core::info;
use xvc_core::output;
use xvc_core::XvcOutputSender;

use crate::Error;
use crate::Result;
use crate::XvcStorageGuid;
use crate::XvcStorageOperations;

use super::XvcStorageDeleteEvent;
use super::XvcStorageExpiringShareEvent;
use super::XvcStorageInitEvent;
use super::XvcStorageListEvent;
use super::XvcStoragePath;
use super::XvcStorageReceiveEvent;
use super::XvcStorageSendEvent;
use super::XvcStorageTempDir;
use super::XVC_STORAGE_GUID_FILENAME;

/// Operations for S3 compatible storage services. Each service implements functions in this trait
/// for xvc file send and xvc file bring commands to work with the common functions.
pub(crate) trait XvcS3StorageOperations {
    /// Prefix within the storage bucket if you want to separate Xvc files from the rest of the
    /// bucket.
    fn storage_prefix(&self) -> String;
    /// GUID for the storage. This is generated when the storage is first initialized.
    fn guid(&self) -> &XvcStorageGuid;
    /// Get the bucket for the storage
    fn get_bucket(&self) -> Result<Box<Bucket>>;
    /// Get the credentials for the
    fn credentials(&self) -> Result<Credentials>;
    /// Name of the bucket
    fn bucket_name(&self) -> String;
    /// Build the storage path for the S3 compatible storage
    fn build_storage_path(&self, cache_path: &XvcCachePath) -> XvcStoragePath {
        XvcStoragePath::from(format!(
            "{}/{}/{}",
            self.storage_prefix(),
            self.guid(),
            cache_path
        ))
    }

    /// Region of the bucket
    fn region(&self) -> String;

    /// Write GUID to the storage when first initializing the storage
    async fn write_storage_guid(&self) -> Result<()> {
        let guid_str = self.guid().to_string();
        let guid_bytes = guid_str.as_bytes();
        let bucket = self.get_bucket()?;
        let response = bucket
            .put_object(
                format!("{}/{}", self.storage_prefix(), XVC_STORAGE_GUID_FILENAME),
                guid_bytes,
            )
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::S3Error { source: err }),
        }
    }

    /// Initialze the bucket as Xvc storage by adding a GUID
    async fn a_init(&mut self, output_snd: &XvcOutputSender) -> Result<XvcStorageInitEvent> {
        let res_response = self.write_storage_guid().await;

        let guid = self.guid().clone();

        match res_response {
            Ok(_) => Ok(XvcStorageInitEvent { guid }),
            Err(err) => {
                error!(output_snd, "{}", err);
                Err(err)
            }
        }
    }

    /// List files in the S3 compatible storage
    async fn a_list(
        &self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
    ) -> Result<XvcStorageListEvent> {
        let credentials = self.credentials()?;
        let region = Region::from_str(&self.region()).unwrap_or("us-east-1".parse().unwrap());
        let bucket = Bucket::new(&self.bucket_name(), region, credentials)?;
        let xvc_guid = xvc_root.config().guid().unwrap();
        let prefix = self.storage_prefix().clone();

        let res_list = bucket
            .list(
                format!("{}/{}", self.storage_prefix(), xvc_guid),
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
                    guid: self.guid().clone(),
                    paths,
                })
            }

            Err(err) => {
                error!(output, "{}", err);
                Err(Error::S3Error { source: err })
            }
        }
    }

    /// Send files to S3 compatible storage
    async fn a_send(
        &self,
        output_snd: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
        paths: &[xvc_core::XvcCachePath],
        _force: bool,
    ) -> crate::Result<super::XvcStorageSendEvent> {
        let mut copied_paths = Vec::<XvcStoragePath>::new();

        let bucket = self.get_bucket()?;

        for cache_path in paths {
            let storage_path = self.build_storage_path(cache_path);
            let abs_cache_path = cache_path.to_absolute_path(xvc_root);

            let mut path = tokio::fs::File::open(&abs_cache_path).await?;

            let res_response = bucket
                .put_object_stream(&mut path, storage_path.as_str())
                .await;

            match res_response {
                Ok(_) => {
                    info!(
                        output_snd,
                        "{} -> {}",
                        abs_cache_path,
                        storage_path.as_str()
                    );
                    copied_paths.push(storage_path);
                }
                Err(err) => {
                    error!(output_snd, "{}", err);
                }
            }
        }

        Ok(XvcStorageSendEvent {
            guid: self.guid().clone(),
            paths: copied_paths,
        })
    }

    /// Receive files from S3 compatible storage
    async fn a_receive(
        &self,
        output_snd: &XvcOutputSender,
        paths: &[xvc_core::XvcCachePath],
        _force: bool,
    ) -> Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> {
        let mut copied_paths = Vec::<XvcStoragePath>::new();

        let bucket = self.get_bucket()?;
        let temp_dir = XvcStorageTempDir::new()?;

        for cache_path in paths {
            let storage_path = self.build_storage_path(cache_path);
            let abs_cache_dir = temp_dir.temp_cache_dir(cache_path)?;
            fs::create_dir_all(&abs_cache_dir)?;
            let abs_cache_path = temp_dir.temp_cache_path(cache_path)?;
            let response_data_stream = bucket.get_object_stream(storage_path.as_str()).await;

            match response_data_stream {
                Ok(mut response) => {
                    info!(
                        output_snd,
                        "{} -> {}",
                        storage_path.as_str(),
                        abs_cache_path
                    );
                    let mut async_cache_path = tokio::fs::File::create(&abs_cache_path).await?;
                    while let Some(chunk) = response.bytes().next().await {
                        async_cache_path.write_all(&chunk?).await?;
                    }
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
                guid: self.guid().clone(),
                paths: copied_paths,
            },
        ))
    }

    /// Delete files from S3 compatible storage
    async fn a_delete(
        &self,
        output: &XvcOutputSender,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent> {
        let mut deleted_paths = Vec::<XvcStoragePath>::new();

        let bucket = self.get_bucket()?;

        for cache_path in paths {
            let storage_path = self.build_storage_path(cache_path);
            bucket.delete_object(storage_path.as_str()).await?;
            info!(output, "[DELETE] {}", storage_path.as_str());
            deleted_paths.push(storage_path);
        }

        Ok(XvcStorageDeleteEvent {
            guid: self.guid().clone(),
            paths: deleted_paths,
        })
    }

    /// Share files from S3 compatible storage for a duration with a signed url
    async fn a_share(
        &self,
        output: &XvcOutputSender,
        path: &XvcCachePath,
        duration: std::time::Duration,
    ) -> Result<XvcStorageExpiringShareEvent> {
        let bucket = self.get_bucket()?;
        // These are optional
        // let mut custom_queries = HashMap::new();
        // custom_queries.insert(
        //    "response-content-disposition".into(),
        //    "attachment; filename=\"test.png\"".into(),
        // );
        //

        let expiration_seconds = duration.as_secs() as u32;
        let path = self.build_storage_path(path);
        let signed_url = bucket
            .presign_get(path.as_str(), expiration_seconds, None)
            .await?;
        info!(output, "[SHARED] {}", path.as_str());
        output!(output, "{}", signed_url);
        Ok(super::XvcStorageExpiringShareEvent {
            guid: self.guid().clone(),
            signed_url,
            expiration_seconds,
            path,
        })
    }
}

impl<T: XvcS3StorageOperations> XvcStorageOperations for T {
    // FIXME: Do we need xvc_root here?
    fn init(&mut self, output: &XvcOutputSender, _xvc_root: &XvcRoot) -> Result<XvcStorageInitEvent>
    where
        Self: Sized,
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        rt.block_on(self.a_init(output))
    }

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
        // FIXME: Do we need xvc_root here?
        _xvc_root: &xvc_core::XvcRoot,
        paths: &[xvc_core::XvcCachePath],
        force: bool,
    ) -> crate::Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(self.a_receive(output, paths, force))
    }

    fn delete(
        &self,
        output: &XvcOutputSender,
        // FIXME: Do we need xvc_root?
        _xvc_root: &xvc_core::XvcRoot,
        paths: &[xvc_core::XvcCachePath],
    ) -> crate::Result<super::XvcStorageDeleteEvent> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(self.a_delete(output, paths))
    }

    fn share(
        &self,
        output: &XvcOutputSender,
        //  FIXME: Do we need xvc_root here?
        _xvc_root: &xvc_core::XvcRoot,
        path: &XvcCachePath,
        duration: std::time::Duration,
    ) -> Result<XvcStorageExpiringShareEvent> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(self.a_share(output, path, duration))
    }
}
