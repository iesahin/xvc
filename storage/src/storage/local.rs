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
use crate::{Result, XvcStorage, XvcStorageEvent};

/// Entry point for `xvc storage new local` command
pub fn cmd_storage_new_local(
    _input: std::io::StdinLock,
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    path: PathBuf,
    name: String,
) -> Result<()> {
    let remote = XvcLocalStorage {
        guid: XvcStorageGuid::new(),
        name,
        path,
    };
    let (init_event, remote) = remote.init(output_snd, xvc_root)?;

    xvc_root.with_r1nstore_mut(|store: &mut R1NStore<XvcStorage, XvcStorageEvent>| {
        let store_e = xvc_root.new_entity();
        let event_e = xvc_root.new_entity();
        store.insert(
            store_e,
            XvcStorage::Local(remote.clone()),
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
    fn remote_path(&self, repo_guid: &str, cache_path: &XvcCachePath) -> XvcStoragePath {
        XvcStoragePath::from(format!("{}/{}", repo_guid, cache_path))
    }
}

impl XvcStorageOperations for XvcLocalStorage {
    fn init(
        self,
        output: &XvcOutputSender,
        _xvc_root: &XvcRoot,
    ) -> Result<(XvcStorageInitEvent, Self)> {
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

            let new_self = XvcLocalStorage {
                guid: already_available_guid.clone(),
                ..self
            };
            return Ok((
                XvcStorageInitEvent {
                    guid: already_available_guid,
                },
                new_self,
            ));
        }

        if !self.path.exists() {
            create_dir_all(&self.path)?;
        }

        fs::write(guid_filename, format!("{}", self.guid))?;
        info!(
            output,
            "Created local remote directory {} with guid: {}",
            self.path.to_string_lossy(),
            self.guid
        );

        Ok((
            XvcStorageInitEvent {
                guid: self.guid.clone(),
            },
            self,
        ))
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
            let remote_path = self.remote_path(&repo_guid, cache_path);
            let abs_remote_path = remote_path.as_ref().to_logical_path(&self.path);
            if force && abs_remote_path.exists() {
                fs::remove_file(abs_remote_path.clone())?;
            } else {
                info!(output, "[SKIPPED] {}", remote_path)
            }
            let abs_cache_path = cache_path.to_absolute_path(xvc_root);
            let abs_remote_dir = abs_remote_path.parent().unwrap();
            fs::create_dir_all(abs_remote_dir)?;
            fs::copy(&abs_cache_path, &abs_remote_path)?;
            copied_paths.push(remote_path);
            watch!(copied_paths.len());
            info!(
                output,
                "{} -> {}",
                abs_cache_path,
                abs_remote_path.to_string_lossy()
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
            let remote_path = self.remote_path(&repo_guid, cache_path);
            let abs_remote_path = remote_path.as_ref().to_logical_path(&self.path);
            let abs_cache_path = temp_dir.temp_cache_path(cache_path)?;
            let abs_cache_dir = temp_dir.temp_cache_dir(cache_path)?;
            fs::create_dir_all(&abs_cache_dir)?;
            fs::copy(&abs_remote_path, &abs_cache_path)?;
            watch!(abs_cache_path.exists());
            copied_paths.push(remote_path);
            watch!(copied_paths.len());
            info!(
                output,
                "{} -> {}",
                abs_remote_path.to_string_lossy(),
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
            let remote_path = self.remote_path(&repo_guid, cache_path);
            let abs_remote_path = remote_path.as_ref().to_logical_path(&self.path);
            fs::remove_file(&abs_remote_path)?;
            info!(output, "[DELETE] {}", abs_remote_path.to_string_lossy());
            deleted_paths.push(remote_path);
        }

        Ok(XvcStorageDeleteEvent {
            guid: self.guid.clone(),
            paths: deleted_paths,
        })
    }
}
