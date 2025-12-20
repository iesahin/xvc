//! Common trait for all storage operations
//! See also async_common.rs for async operations with network storages

use std::time::Duration;

use crate::storage::{
    XvcStorageDeleteEvent, XvcStorageExpiringShareEvent, XvcStorageInitEvent, XvcStorageListEvent,
    XvcStorageReceiveEvent, XvcStorageSendEvent,
};

use xvc_core::XvcOutputSender;

use crate::Result;

use xvc_core::{XvcCachePath, XvcRoot};

use crate::XvcStorage;

use super::XvcStorageTempDir;

/// All storages implement this trait. xvc storage new   and xvc file send / bring / remove
/// commands use this trait to communicate with the storages.
pub trait XvcStorageOperations {
    /// The init operation is creates a directory with the "short guid" of the Xvc repository and
    /// adds a .xvc-guid file with the guid of the storage.
    fn init(&mut self, output: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<XvcStorageInitEvent>;

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
        self.as_dyn_mut().init(output, xvc_root)
    }

    fn list(&self, output: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<XvcStorageListEvent> {
        self.as_dyn().list(output, xvc_root)
    }

    fn send(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<XvcStorageSendEvent> {
        self.as_dyn().send(output, xvc_root, paths, force)
    }

    fn receive(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> {
        self.as_dyn().receive(output, xvc_root, paths, force)
    }

    fn delete(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent> {
        self.as_dyn().delete(output, xvc_root, paths)
    }

    fn share(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        path: &XvcCachePath,
        period: Duration,
    ) -> Result<XvcStorageExpiringShareEvent> {
        self.as_dyn().share(output, xvc_root, path, period)
    }
}
