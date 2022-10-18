use std::{
    fs::{self, create_dir_all},
    path::PathBuf,
};

use crossbeam_channel::Sender;
use serde::{Deserialize, Serialize};
use xvc_core::XvcRoot;
use xvc_ecs::R1NStore;
use xvc_logging::{watch, XvcOutputLine};

use super::{
    XvcCachePath, XvcStorageDeleteEvent, XvcStorageGuid, XvcStorageInitEvent, XvcStorageListEvent,
    XvcStorageOperations, XvcStoragePath, XvcStorageReceiveEvent, XvcStorageSendEvent,
    XVC_STORAGE_GUID_FILENAME,
};
use crate::{Result, XvcStorage, XvcStorageEvent};

pub fn cmd_storage_new_local(
    input: std::io::StdinLock,
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    path: PathBuf,
    name: String,
) -> Result<()> {
    let remote = XvcLocalStorage {
        guid: XvcStorageGuid::new(),
        name,
        path,
    };
    let init_event = remote.init(output_snd, xvc_root)?;

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

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcLocalStorage {
    pub guid: XvcStorageGuid,
    pub name: String,
    pub path: PathBuf,
}

impl XvcStorageOperations for XvcLocalStorage {
    fn init(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
    ) -> Result<XvcStorageInitEvent> {
        // Check if there is no directory as `self.path`
        if self.path.exists() {
            return Err(anyhow::anyhow!("Remote should point to a blank directory").into());
        } else {
            create_dir_all(&self.path)?;
        }

        let guid_filename = self.path.join(XVC_STORAGE_GUID_FILENAME);
        fs::write(guid_filename, format!("{}", self.guid))?;
        output.send(XvcOutputLine::Info(format!(
            "Created local remote directory {} with guid: {}",
            self.path.to_string_lossy(),
            self.guid,
        )));

        Ok(XvcStorageInitEvent {
            guid: self.guid.clone(),
        })
    }

    fn list(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
    ) -> Result<XvcStorageListEvent> {
        todo!()
    }

    fn send(
        &self,
        output: Sender<XvcOutputLine>,
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
            watch!(cache_path);
            let remote_path = XvcStoragePath::from(format!("{}/{}", repo_guid, cache_path));
            watch!(remote_path);
            let abs_remote_path = remote_path.as_ref().to_logical_path(&self.path);
            watch!(abs_remote_path);
            if force && abs_remote_path.exists() {
                fs::remove_file(abs_remote_path.clone())?;
            }
            let abs_cache_path = cache_path.to_absolute_path(xvc_root);
            watch!(abs_cache_path);
            let abs_remote_dir = abs_remote_path.parent().unwrap();
            fs::create_dir_all(&abs_remote_dir)?;
            fs::copy(&abs_cache_path, &abs_remote_path)?;
            copied_paths.push(remote_path);
            watch!(copied_paths.len());
            output
                .send(XvcOutputLine::Info(format!(
                    "{} -> {}",
                    abs_cache_path,
                    abs_remote_path.to_string_lossy()
                )))
                .unwrap();
        }

        Ok(XvcStorageSendEvent {
            guid: self.guid.clone(),
            paths: copied_paths,
        })
    }

    fn receive(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<XvcStorageReceiveEvent> {
        let repo_guid = xvc_root
            .config()
            .guid()
            .ok_or_else(|| crate::Error::NoRepositoryGuidFound)?;
        let mut copied_paths = Vec::<XvcStoragePath>::new();

        for cache_path in paths {
            watch!(cache_path);
            let remote_path = XvcStoragePath::from(format!("{}/{}", repo_guid, cache_path));
            watch!(remote_path);
            let abs_remote_path = remote_path.as_ref().to_logical_path(&self.path);
            watch!(abs_remote_path);
            let abs_cache_path = cache_path.to_absolute_path(xvc_root);
            watch!(abs_cache_path);
            if force && abs_remote_path.exists() {
                fs::remove_file(abs_remote_path.clone())?;
            }
            let abs_cache_dir = abs_cache_path.parent().unwrap();
            fs::create_dir_all(&abs_cache_dir)?;
            fs::copy(&abs_remote_path, &abs_cache_path)?;
            copied_paths.push(remote_path);
            watch!(copied_paths.len());
            output
                .send(XvcOutputLine::Info(format!(
                    "{} -> {}",
                    abs_remote_path.to_string_lossy(),
                    abs_cache_path,
                )))
                .unwrap();
        }

        Ok(XvcStorageReceiveEvent {
            guid: self.guid.clone(),
            paths: copied_paths,
        })
    }

    fn delete(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent> {
        todo!()
    }
}
