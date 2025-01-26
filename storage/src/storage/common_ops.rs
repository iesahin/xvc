//! Common trait for all storage operations
//! See also async_common.rs for async operations with network storages

use std::time::Duration;

use crate::storage::{
    XvcStorageDeleteEvent, XvcStorageExpiringShareEvent, XvcStorageInitEvent, XvcStorageListEvent,
    XvcStorageReceiveEvent, XvcStorageSendEvent,
};

use xvc_logging::XvcOutputSender;

use crate::{Error, Result};

use xvc_core::{XvcCachePath, XvcRoot};

use crate::XvcStorage;

use super::XvcStorageTempDir;

/// All storages implement this trait. xvc storage new   and xvc file send / bring / remove
/// commands use this trait to communicate with the storages.
pub trait XvcStorageOperations {
    /// The init operation is creates a directory with the "short guid" of the Xvc repository and
    /// adds a .xvc-guid file with the guid of the storage.
    fn init(&mut self, output: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<XvcStorageInitEvent>
    where
        Self: Sized;

    /// Used by xvc file list command to list the contents of a directory in the storage.
    fn list(&self, output: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<XvcStorageListEvent>;
    /// Used by xvc file send command to send files to the storage.
    fn send(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<XvcStorageSendEvent>;
    /// Used by xvc file bring command to bring files from the storage.
    fn receive(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<(XvcStorageTempDir, XvcStorageReceiveEvent)>;
    /// Used by xvc file remove command to remove files from the storage.
    fn delete(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent>;

    /// Used to share files from S3 compatible storages with a signed URL.
    fn share(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        path: &XvcCachePath,
        period: Duration,
    ) -> Result<XvcStorageExpiringShareEvent>;
}

impl XvcStorageOperations for XvcStorage {
    fn init(
        &mut self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
    ) -> Result<XvcStorageInitEvent> {
        match self {
            XvcStorage::Local(ref mut r) => r.init(output, xvc_root),
            XvcStorage::Generic(ref mut r) => r.init(output, xvc_root),
            XvcStorage::Rsync(ref mut r) => r.init(output, xvc_root),
            #[cfg(feature = "s3")]
            XvcStorage::S3(ref mut r) => r.init(output, xvc_root),
            #[cfg(feature = "minio")]
            XvcStorage::Minio(ref mut r) => r.init(output, xvc_root),
            #[cfg(feature = "r2")]
            XvcStorage::R2(ref mut r) => r.init(output, xvc_root),
            #[cfg(feature = "gcs")]
            XvcStorage::Gcs(ref mut r) => r.init(output, xvc_root),
            #[cfg(feature = "wasabi")]
            XvcStorage::Wasabi(ref mut r) => r.init(output, xvc_root),
            #[cfg(feature = "digital-ocean")]
            XvcStorage::DigitalOcean(ref mut r) => r.init(output, xvc_root),
        }
    }

    fn list(&self, output: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<XvcStorageListEvent> {
        match self {
            XvcStorage::Local(lr) => lr.list(output, xvc_root),
            XvcStorage::Generic(gr) => gr.list(output, xvc_root),
            XvcStorage::Rsync(r) => r.list(output, xvc_root),
            #[cfg(feature = "s3")]
            XvcStorage::S3(s3r) => s3r.list(output, xvc_root),
            #[cfg(feature = "minio")]
            XvcStorage::Minio(mr) => mr.list(output, xvc_root),
            #[cfg(feature = "r2")]
            XvcStorage::R2(r) => r.list(output, xvc_root),
            #[cfg(feature = "gcs")]
            XvcStorage::Gcs(r) => r.list(output, xvc_root),
            #[cfg(feature = "wasabi")]
            XvcStorage::Wasabi(r) => r.list(output, xvc_root),
            #[cfg(feature = "digital-ocean")]
            XvcStorage::DigitalOcean(r) => r.list(output, xvc_root),
        }
    }

    fn send(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<XvcStorageSendEvent> {
        match self {
            XvcStorage::Local(lr) => lr.send(output, xvc_root, paths, force),
            XvcStorage::Generic(gr) => gr.send(output, xvc_root, paths, force),
            XvcStorage::Rsync(r) => r.send(output, xvc_root, paths, force),
            #[cfg(feature = "s3")]
            XvcStorage::S3(s3r) => s3r.send(output, xvc_root, paths, force),
            #[cfg(feature = "minio")]
            XvcStorage::Minio(mr) => mr.send(output, xvc_root, paths, force),
            #[cfg(feature = "r2")]
            XvcStorage::R2(r) => r.send(output, xvc_root, paths, force),
            #[cfg(feature = "gcs")]
            XvcStorage::Gcs(r) => r.send(output, xvc_root, paths, force),
            #[cfg(feature = "wasabi")]
            XvcStorage::Wasabi(r) => r.send(output, xvc_root, paths, force),
            #[cfg(feature = "digital-ocean")]
            XvcStorage::DigitalOcean(r) => r.send(output, xvc_root, paths, force),
        }
    }

    fn receive(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> {
        match self {
            XvcStorage::Local(lr) => lr.receive(output, xvc_root, paths, force),
            XvcStorage::Generic(gr) => gr.receive(output, xvc_root, paths, force),
            XvcStorage::Rsync(r) => r.receive(output, xvc_root, paths, force),
            #[cfg(feature = "s3")]
            XvcStorage::S3(s3r) => s3r.receive(output, xvc_root, paths, force),
            #[cfg(feature = "minio")]
            XvcStorage::Minio(mr) => mr.receive(output, xvc_root, paths, force),
            #[cfg(feature = "r2")]
            XvcStorage::R2(r) => r.receive(output, xvc_root, paths, force),
            #[cfg(feature = "gcs")]
            XvcStorage::Gcs(r) => r.receive(output, xvc_root, paths, force),
            #[cfg(feature = "wasabi")]
            XvcStorage::Wasabi(r) => r.receive(output, xvc_root, paths, force),
            #[cfg(feature = "digital-ocean")]
            XvcStorage::DigitalOcean(r) => r.receive(output, xvc_root, paths, force),
        }
    }

    fn delete(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent> {
        match self {
            XvcStorage::Local(lr) => lr.delete(output, xvc_root, paths),
            XvcStorage::Generic(gr) => gr.delete(output, xvc_root, paths),
            XvcStorage::Rsync(r) => r.delete(output, xvc_root, paths),
            #[cfg(feature = "s3")]
            XvcStorage::S3(s3r) => s3r.delete(output, xvc_root, paths),
            #[cfg(feature = "minio")]
            XvcStorage::Minio(mr) => mr.delete(output, xvc_root, paths),
            #[cfg(feature = "r2")]
            XvcStorage::R2(r) => r.delete(output, xvc_root, paths),
            #[cfg(feature = "gcs")]
            XvcStorage::Gcs(r) => r.delete(output, xvc_root, paths),
            #[cfg(feature = "wasabi")]
            XvcStorage::Wasabi(r) => r.delete(output, xvc_root, paths),
            #[cfg(feature = "digital-ocean")]
            XvcStorage::DigitalOcean(r) => r.delete(output, xvc_root, paths),
        }
    }

    fn share(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        path: &XvcCachePath,
        period: Duration,
    ) -> Result<XvcStorageExpiringShareEvent> {
        match self {
            XvcStorage::Local(_) | XvcStorage::Generic(_) | XvcStorage::Rsync(_) => {
                Err(Error::StorageDoesNotSupportSignedUrls)
            }
            #[cfg(feature = "s3")]
            XvcStorage::S3(r) => r.share(output, xvc_root, path, period),
            #[cfg(feature = "minio")]
            XvcStorage::Minio(r) => r.share(output, xvc_root, path, period),
            #[cfg(feature = "r2")]
            XvcStorage::R2(r) => r.share(output, xvc_root, path, period),
            #[cfg(feature = "gcs")]
            XvcStorage::Gcs(r) => r.share(output, xvc_root, path, period),
            #[cfg(feature = "wasabi")]
            XvcStorage::Wasabi(r) => r.share(output, xvc_root, path, period),
            #[cfg(feature = "digital-ocean")]
            XvcStorage::DigitalOcean(r) => r.share(output, xvc_root, path, period),
        }
    }
}
