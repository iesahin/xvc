//! Local storage implementation
use std::{
    fs::{self, create_dir_all},
    path::PathBuf,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use xvc_core::XvcRoot;
use xvc_ecs::R1NStore;
use xvc_logging::{info, watch, XvcOutputSender};

use super::{
    XvcCachePath, XvcStorageDeleteEvent, XvcStorageGuid, XvcStorageInitEvent, XvcStorageListEvent,
    XvcStorageOperations, XvcStoragePath, XvcStorageReceiveEvent, XvcStorageSendEvent,
    XvcStorageTempDir, XVC_STORAGE_GUID_FILENAME,
};
use crate::{Error, Result, XvcStorage, XvcStorageEvent};

/// Entry point for `xvc storage new local` command
pub fn cmd_storage_new_local(
    _input: std::io::StdinLock,
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    path: PathBuf,
    name: String,
) -> Result<()> {
    let mut storage = XvcLocalStorage {
        guid: XvcStorageGuid::new(),
        name,
        path,
    };
    let init_event = storage.init(output_snd, xvc_root)?;

    xvc_root.with_r1nstore_mut(|store: &mut R1NStore<XvcStorage, XvcStorageEvent>| {
        let store_e = xvc_root.new_entity();
        let event_e = xvc_root.new_entity();
        store.insert(
            store_e,
            XvcStorage::Local(storage.clone()),
            event_e,
            XvcStorageEvent::Init(init_event.clone()),
        );
        Ok(())
    })?;

    Ok(())
}

/// A local storage is a directory that is on the same machine as the repository.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcLocalStorage {
    /// GUID of the storage
    pub guid: XvcStorageGuid,
    /// Name of the storage
    pub name: String,
    /// Path to the storage
    pub path: PathBuf,
}

impl XvcLocalStorage {
    fn storage_path(&self, repo_guid: &str, cache_path: &XvcCachePath) -> XvcStoragePath {
        XvcStoragePath::from(format!("{}/{}", repo_guid, cache_path))
    }
}

impl XvcStorageOperations for XvcLocalStorage {
    fn init(
        &mut self,
        output: &XvcOutputSender,
        _xvc_root: &XvcRoot,
    ) -> Result<XvcStorageInitEvent> {
        let guid_filename = self.path.join(XVC_STORAGE_GUID_FILENAME);
        // If guid filename exists, we can report a reinit and exit.
        watch!(guid_filename);

        if guid_filename.exists() {
            let already_available_guid =
                XvcStorageGuid::from_str(&fs::read_to_string(guid_filename)?)?;
            info!(
                output,
                "Found previous storage {} with GUID: {}",
                self.path.to_string_lossy(),
                already_available_guid
            );

            self.guid = already_available_guid.clone();

            return Ok(XvcStorageInitEvent {
                guid: already_available_guid,
            });
        }

        if !self.path.exists() {
            create_dir_all(&self.path)?;
        }

        fs::write(guid_filename, format!("{}", self.guid))?;
        info!(
            output,
            "Created local storage directory {} with guid: {}",
            self.path.to_string_lossy(),
            self.guid
        );

        Ok(XvcStorageInitEvent {
            guid: self.guid.clone(),
        })
    }

    fn list(&self, _output: &XvcOutputSender, _xvc_root: &XvcRoot) -> Result<XvcStorageListEvent> {
        todo!()
    }

    fn send(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<XvcStorageSendEvent> {
        let repo_guid = xvc_root
            .config()
            .guid()
            .ok_or_else(|| crate::Error::NoRepositoryGuidFound)?;
        let mut copied_paths = Vec::<XvcStoragePath>::new();

        for cache_path in paths {
            let storage_path = self.storage_path(&repo_guid, cache_path);
            let abs_storage_path = storage_path.as_ref().to_logical_path(&self.path);
            if force && abs_storage_path.exists() {
                fs::remove_file(abs_storage_path.clone())?;
            } else {
                info!(output, "[SKIPPED] {}", storage_path)
            }
            let abs_cache_path = cache_path.to_absolute_path(xvc_root);
            let abs_storage_dir = abs_storage_path.parent().unwrap();
            fs::create_dir_all(abs_storage_dir)?;
            fs::copy(&abs_cache_path, &abs_storage_path)?;
            copied_paths.push(storage_path);
            watch!(copied_paths.len());
            info!(
                output,
                "{} -> {}",
                abs_cache_path,
                abs_storage_path.to_string_lossy()
            );
        }

        Ok(XvcStorageSendEvent {
            guid: self.guid.clone(),
            paths: copied_paths,
        })
    }

    fn receive(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        _force: bool,
    ) -> Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> {
        let repo_guid = xvc_root
            .config()
            .guid()
            .ok_or_else(|| crate::Error::NoRepositoryGuidFound)?;
        let mut copied_paths = Vec::<XvcStoragePath>::new();
        let temp_dir = XvcStorageTempDir::new()?;

        for cache_path in paths {
            let storage_path = self.storage_path(&repo_guid, cache_path);
            let abs_storage_path = storage_path.as_ref().to_logical_path(&self.path);
            let abs_cache_path = temp_dir.temp_cache_path(cache_path)?;
            let abs_cache_dir = temp_dir.temp_cache_dir(cache_path)?;
            fs::create_dir_all(&abs_cache_dir)?;
            fs::copy(&abs_storage_path, &abs_cache_path)?;
            watch!(abs_cache_path.exists());
            copied_paths.push(storage_path);
            watch!(copied_paths.len());
            info!(
                output,
                "{} -> {}",
                abs_storage_path.to_string_lossy(),
                abs_cache_path
            );
        }

        Ok((
            temp_dir,
            XvcStorageReceiveEvent {
                guid: self.guid.clone(),
                paths: copied_paths,
            },
        ))
    }

    fn delete(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent> {
        let repo_guid = xvc_root
            .config()
            .guid()
            .ok_or_else(|| crate::Error::NoRepositoryGuidFound)?;
        let mut deleted_paths = Vec::<XvcStoragePath>::new();

        for cache_path in paths {
            let storage_path = self.storage_path(&repo_guid, cache_path);
            let abs_storage_path = storage_path.as_ref().to_logical_path(&self.path);
            fs::remove_file(&abs_storage_path)?;
            info!(output, "[DELETE] {}", abs_storage_path.to_string_lossy());
            deleted_paths.push(storage_path);
        }

        Ok(XvcStorageDeleteEvent {
            guid: self.guid.clone(),
            paths: deleted_paths,
        })
    }

    fn share(
        &self,
        _output: &XvcOutputSender,
        _xvc_root: &XvcRoot,
        _path: &XvcCachePath,
        _period: std::time::Duration,
    ) -> Result<super::XvcStorageExpiringShareEvent> {
        Err(Error::StorageDoesNotSupportSignedUrls)
    }
}
