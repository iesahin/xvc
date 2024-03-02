use std::fs;
use std::str::FromStr;

use futures::StreamExt;
use regex::Regex;
use s3::creds::Credentials;
use s3::Bucket;
use s3::Region;
use tokio::io::AsyncWriteExt;
use xvc_core::XvcCachePath;
use xvc_logging::error;
use xvc_logging::info;
use xvc_logging::watch;
use xvc_logging::XvcOutputSender;

use crate::storage::XVC_STORAGE_GUID_FILENAME;
use crate::Error;
use crate::Result;
use crate::XvcStorageGuid;
use crate::XvcStorageOperations;

use super::XvcStorageDeleteEvent;
use super::XvcStorageInitEvent;
use super::XvcStorageListEvent;
use super::XvcStoragePath;
use super::XvcStorageReceiveEvent;
use super::XvcStorageSendEvent;
use super::XvcStorageTempDir;

pub trait XvcS3StorageOperations {
    fn remote_prefix(&self) -> &str;
    fn guid(&self) -> &XvcStorageGuid;
    fn get_bucket(&self) -> Result<Bucket>;
    fn credentials(&self) -> Result<Credentials>;
    fn bucket_name(&self) -> &str;
    fn build_remote_path(&self, cache_path: &XvcCachePath) -> XvcStoragePath;
    fn region(&self) -> &str;

    async fn a_init(&self, output_snd: &XvcOutputSender) -> Result<(XvcStorageInitEvent, &Self)> {
        let bucket = self.get_bucket()?;
        let guid = self.guid().clone();
        let guid_str = self.guid().to_string();
        let guid_bytes = guid_str.as_bytes();

        watch!(bucket);
        watch!(guid);
        watch!(guid_str);

        let res_response = bucket
            .put_object(
                format!("{}/{}", self.remote_prefix(), XVC_STORAGE_GUID_FILENAME),
                guid_bytes,
            )
            .await;

        match res_response {
            Ok(_) => Ok((XvcStorageInitEvent { guid }, self)),
            Err(err) => {
                error!(output_snd, "{}", err);
                Err(Error::S3Error { source: err })
            }
        }
    }

    async fn a_list(
        &self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
    ) -> Result<XvcStorageListEvent> {
        let credentials = self.credentials()?;
        let region = Region::from_str(&self.region()).unwrap_or("us-east-1".parse().unwrap());
        let bucket = Bucket::new(&self.bucket_name(), region, credentials)?;
        let xvc_guid = xvc_root.config().guid().unwrap();
        let prefix = self.remote_prefix().clone();

        let res_list = bucket
            .list(
                format!("{}/{}", self.remote_prefix(), xvc_guid),
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
            watch!(cache_path);
            let remote_path = self.build_remote_path(cache_path);
            let abs_cache_path = cache_path.to_absolute_path(xvc_root);
            watch!(abs_cache_path);

            let mut path = tokio::fs::File::open(&abs_cache_path).await?;
            watch!(path);

            let res_response = bucket
                .put_object_stream(&mut path, remote_path.as_str())
                .await;

            match res_response {
                Ok(_) => {
                    info!(output_snd, "{} -> {}", abs_cache_path, remote_path.as_str());
                    copied_paths.push(remote_path);
                    watch!(copied_paths.len());
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
            watch!(cache_path);
            let remote_path = self.build_remote_path(cache_path);
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
                guid: self.guid().clone(),
                paths: copied_paths,
            },
        ))
    }

    async fn a_delete(
        &self,
        output: &XvcOutputSender,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent> {
        let mut deleted_paths = Vec::<XvcStoragePath>::new();

        let bucket = self.get_bucket()?;

        for cache_path in paths {
            watch!(cache_path);
            let remote_path = self.build_remote_path(cache_path);
            bucket.delete_object(remote_path.as_str()).await?;
            info!(output, "[DELETE] {}", remote_path.as_str());
            deleted_paths.push(remote_path);
        }

        Ok(XvcStorageDeleteEvent {
            guid: self.guid().clone(),
            paths: deleted_paths,
        })
    }
}

impl<T: XvcS3StorageOperations> XvcStorageOperations for T {
    fn init(
        self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
    ) -> Result<(XvcStorageInitEvent, Self)>
    where
        Self: Sized {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        watch!(rt);
        rt.block_on(self.a_init(output, xvc_root))
    }

    fn list(&self, output: &XvcOutputSender, xvc_root: &xvc_core::XvcRoot) -> Result<XvcStorageListEvent> \{
        todo!()
    \}

    fn send(
        &self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<XvcStorageSendEvent> \{
        todo!()
    \}

    fn receive(
        &self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> \{
        todo!()
    \}

    fn delete(
        &self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent> \{
        todo!()
    \}

    fn share(
        &self,
        output: &XvcOutputSender,
        xvc_root: &xvc_core::XvcRoot,
        path: &XvcCachePath,
        period: std::time::Duration,
    ) -> Result<super::XvcStorageExpiringShareEvent> \{
        todo!()
    \}
}
