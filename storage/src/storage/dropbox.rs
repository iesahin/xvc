//! Dropbox remote storage implementation
use std::env;
use std::fs;
use std::io::Read;

use serde::{Deserialize, Serialize};
use xvc_core::R1NStore;
use xvc_core::{error, info, watch, XvcOutputSender};
use xvc_core::{XvcCachePath, XvcRoot};

use crate::storage::XVC_STORAGE_GUID_FILENAME;
use crate::{Error, Result, XvcStorage, XvcStorageEvent};
use crate::{XvcStorageGuid, XvcStorageOperations};

use super::XvcStorageDeleteEvent;
use super::XvcStorageInitEvent;
use super::XvcStorageListEvent;
use super::XvcStoragePath;
use super::XvcStorageReceiveEvent;
use super::XvcStorageSendEvent;
use super::XvcStorageTempDir;

/// Configure a new Dropbox remote storage.
///
/// `storage_prefix` sets the directory in Dropbox for the storage location.
///
/// This creates a [XvcDropboxStorage], calls its
/// [init][XvcDropboxStorage::init] function to create/update guid, and
/// saves [XvcStorageInitEvent] and [XvcStorage] in ECS.
pub fn cmd_new_dropbox(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    name: String,
    storage_prefix: String,
) -> Result<()> {
    let mut storage = XvcDropboxStorage {
        guid: XvcStorageGuid::new(),
        name,
        storage_prefix,
    };

    let init_event = storage.init(output_snd, xvc_root)?;
    watch!(init_event);

    xvc_root.with_r1nstore_mut(|store: &mut R1NStore<XvcStorage, XvcStorageEvent>| {
        let store_e = xvc_root.new_entity();
        let event_e = xvc_root.new_entity();
        store.insert(
            store_e,
            XvcStorage::Dropbox(storage.clone()),
            event_e,
            XvcStorageEvent::Init(init_event.clone()),
        );
        Ok(())
    })?;

    info!(output_snd, "Dropbox Storage Created: {:#?}", storage);

    Ok(())
}

/// A Dropbox configuration as a remote storage location
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcDropboxStorage {
    /// Specifies the storage uniquely.
    ///
    /// This is also stored in
    /// `storage_prefix/.xvc-guid` to identify the
    /// remote location.
    pub guid: XvcStorageGuid,
    /// Name of the remote storage to be used in commands.
    ///
    /// It doesn't have to be unique, though in practice setting unique names is
    /// preferred.
    pub name: String,
    /// The "directory" in Dropbox that Xvc will use.
    pub storage_prefix: String,
}

impl XvcDropboxStorage {
    fn credentials(&self) -> Result<String> {
        let specific_token_var = format!("XVC_STORAGE_DROPBOX_TOKEN_{}", self.name);
        if let Ok(token) = env::var(&specific_token_var) {
            return Ok(token);
        }

        let generic_token_var = "DROPBOX_ACCESS_TOKEN";
        if let Ok(token) = env::var(generic_token_var) {
            return Ok(token);
        }

        Err(Error::CloudCredentialsNotFound {
            storage_name: self.name.clone(),
            var_pairs: vec![(specific_token_var, generic_token_var.to_string())],
        })
    }

    fn dropbox_path(&self, path: &str) -> String {
        let prefix = self.storage_prefix.trim_start_matches('/').trim_end_matches('/');
        if prefix.is_empty() {
            format!("/{}", path.trim_start_matches('/'))
        } else {
            format!("/{}", format!("{}/{}", prefix, path.trim_start_matches('/')).trim_start_matches('/'))
        }
    }

    fn build_storage_path(&self, xvc_root: &XvcRoot, cache_path: &XvcCachePath) -> XvcStoragePath {
        XvcStoragePath::from(format!(
            "{}/{}/{}",
            self.storage_prefix.trim_start_matches('/').trim_end_matches('/'),
            xvc_root.guid(),
            cache_path
        ))
    }
}

#[derive(Serialize)]
struct ListFolderArg {
    path: String,
    recursive: bool,
}

#[derive(Deserialize)]
struct ListFolderResult {
    entries: Vec<DropboxEntry>,
    has_more: bool,
    cursor: String,
}

#[derive(Deserialize)]
struct DropboxEntry {
    #[serde(rename = ".tag")]
    tag: String,
    path_display: String,
}

#[derive(Serialize)]
struct CommitInfo {
    path: String,
    mode: String,
    autorename: bool,
    mute: bool,
    strict_conflict: bool,
}

#[derive(Serialize)]
struct UploadSessionStartArg {
    close: bool,
}

#[derive(Deserialize)]
struct UploadSessionStartResult {
    session_id: String,
}

#[derive(Serialize)]
struct UploadSessionCursor {
    session_id: String,
    offset: u64,
}

#[derive(Serialize)]
struct UploadSessionAppendArg {
    cursor: UploadSessionCursor,
    close: bool,
}

#[derive(Serialize)]
struct UploadSessionFinishArg {
    cursor: UploadSessionCursor,
    commit: CommitInfo,
}

#[derive(Serialize)]
struct DeleteArg {
    path: String,
}

const DROPBOX_MAX_UPLOAD_SIZE: u64 = 150 * 1024 * 1024;
const DROPBOX_CHUNK_SIZE: u64 = 8 * 1024 * 1024;

impl XvcStorageOperations for XvcDropboxStorage {
    fn init(&mut self, _output: &XvcOutputSender, _xvc_root: &XvcRoot) -> Result<XvcStorageInitEvent> {
        let token = self.credentials()?;
        let client = reqwest::blocking::Client::new();
        let guid_str = self.guid.to_string();
        let path = self.dropbox_path(XVC_STORAGE_GUID_FILENAME);

        let commit_info = CommitInfo {
            path: path.clone(),
            mode: "overwrite".to_string(),
            autorename: false,
            mute: true,
            strict_conflict: false,
        };

        let response = client
            .post("https://content.dropboxapi.com/2/files/upload")
            .header("Authorization", format!("Bearer {}", token))
            .header("Dropbox-API-Arg", serde_json::to_string(&commit_info).unwrap())
            .header("Content-Type", "application/octet-stream")
            .body(guid_str)
            .send()
            .map_err(|e| Error::AnyhowError { source: anyhow::anyhow!("Dropbox upload error: {}", e) })?;

        if !response.status().is_success() {
            let err_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::AnyhowError { source: anyhow::anyhow!("Dropbox init failed: {}", err_text) });
        }

        Ok(XvcStorageInitEvent { guid: self.guid.clone() })
    }

    fn list(&self, _output: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<XvcStorageListEvent> {
        let token = self.credentials()?;
        let client = reqwest::blocking::Client::new();
        let xvc_guid = xvc_root.guid();
        let path = self.dropbox_path(&xvc_guid);

        let arg = ListFolderArg {
            path: path.clone(),
            recursive: true,
        };

        let response = client
            .post("https://api.dropboxapi.com/2/files/list_folder")
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&arg)
            .send()
            .map_err(|e| Error::AnyhowError { source: anyhow::anyhow!("Dropbox list error: {}", e) })?;

        if !response.status().is_success() {
            // If folder doesn't exist, Dropbox returns 409 Path not found.
            // We should return an empty list in that case instead of error.
            if response.status().as_u16() == 409 {
                return Ok(XvcStorageListEvent {
                    guid: self.guid.clone(),
                    paths: vec![],
                });
            }
            let err_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::AnyhowError { source: anyhow::anyhow!("Dropbox list failed: {}", err_text) });
        }

        let mut result: ListFolderResult = response.json().map_err(|e| Error::AnyhowError { source: anyhow::anyhow!("Dropbox json error: {}", e) })?;
        let mut entries = result.entries;

        while result.has_more {
            let arg = serde_json::json!({ "cursor": result.cursor });
            let response = client
                .post("https://api.dropboxapi.com/2/files/list_folder/continue")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .json(&arg)
                .send()
                .map_err(|e| Error::AnyhowError { source: anyhow::anyhow!("Dropbox list continue error: {}", e) })?;
            
            result = response.json().map_err(|e| Error::AnyhowError { source: anyhow::anyhow!("Dropbox json error: {}", e) })?;
            entries.extend(result.entries);
        }

        let paths = entries
            .into_iter()
            .filter(|e| e.tag == "file")
            .map(|e| XvcStoragePath::from(e.path_display.trim_start_matches('/').to_string()))
            .collect();

        Ok(XvcStorageListEvent {
            guid: self.guid.clone(),
            paths,
        })
    }

    fn send(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        _force: bool,
    ) -> Result<XvcStorageSendEvent> {
        let token = self.credentials()?;
        let client = reqwest::blocking::Client::new();
        let mut sent_paths = Vec::new();

        for cache_path in paths {
            let abs_path = cache_path.to_absolute_path(xvc_root);
            let storage_path = self.build_storage_path(xvc_root, cache_path);
            let dropbox_path = format!("/{}", storage_path.as_str());

            let metadata = fs::metadata(&abs_path)?;
            let file_size = metadata.len();

            let res = if file_size <= DROPBOX_MAX_UPLOAD_SIZE {
                let file = fs::File::open(&abs_path)?;
                let commit_info = CommitInfo {
                    path: dropbox_path.clone(),
                    mode: "overwrite".to_string(),
                    autorename: false,
                    mute: true,
                    strict_conflict: false,
                };

                client
                    .post("https://content.dropboxapi.com/2/files/upload")
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Dropbox-API-Arg", serde_json::to_string(&commit_info).unwrap())
                    .header("Content-Type", "application/octet-stream")
                    .body(file)
                    .send()
                    .map_err(|e| Error::AnyhowError { source: anyhow::anyhow!("Dropbox upload error: {}", e) })?
            } else {
                // Chunked upload
                let mut file = fs::File::open(&abs_path)?;
                
                // Start session
                let arg = UploadSessionStartArg { close: false };
                let response = client
                    .post("https://content.dropboxapi.com/2/files/upload_session/start")
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Dropbox-API-Arg", serde_json::to_string(&arg).unwrap())
                    .header("Content-Type", "application/octet-stream")
                    .send()
                    .map_err(|e| Error::AnyhowError { source: anyhow::anyhow!("Dropbox session start error: {}", e) })?;

                if !response.status().is_success() {
                    let err_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
                    return Err(Error::AnyhowError { source: anyhow::anyhow!("Dropbox session start failed: {}", err_text) });
                }

                let start_result: UploadSessionStartResult = response.json().map_err(|e| Error::AnyhowError { source: anyhow::anyhow!("Dropbox json error: {}", e) })?;
                let session_id = start_result.session_id;

                let mut offset = 0;
                let mut buffer = vec![0u8; DROPBOX_CHUNK_SIZE as usize];
                let mut last_response = None;
                loop {
                    let bytes_read = file.read(&mut buffer)?;
                    if bytes_read == 0 {
                        break;
                    }

                    offset += bytes_read as u64;
                    let is_last_chunk = offset == file_size;

                    if !is_last_chunk {
                        let arg = UploadSessionAppendArg {
                            cursor: UploadSessionCursor {
                                session_id: session_id.clone(),
                                offset: offset - bytes_read as u64,
                            },
                            close: false,
                        };

                        let response = client
                            .post("https://content.dropboxapi.com/2/files/upload_session/append_v2")
                            .header("Authorization", format!("Bearer {}", token))
                            .header("Dropbox-API-Arg", serde_json::to_string(&arg).unwrap())
                            .header("Content-Type", "application/octet-stream")
                            .body(buffer[..bytes_read].to_vec())
                            .send()
                            .map_err(|e| Error::AnyhowError { source: anyhow::anyhow!("Dropbox session append error: {}", e) })?;

                        if !response.status().is_success() {
                            let err_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
                            return Err(Error::AnyhowError { source: anyhow::anyhow!("Dropbox session append failed: {}", err_text) });
                        }
                    } else {
                        // Finish session
                        let commit_info = CommitInfo {
                            path: dropbox_path.clone(),
                            mode: "overwrite".to_string(),
                            autorename: false,
                            mute: true,
                            strict_conflict: false,
                        };

                        let arg = UploadSessionFinishArg {
                            cursor: UploadSessionCursor {
                                session_id: session_id.clone(),
                                offset: offset - bytes_read as u64,
                            },
                            commit: commit_info,
                        };

                        let response = client
                            .post("https://content.dropboxapi.com/2/files/upload_session/finish")
                            .header("Authorization", format!("Bearer {}", token))
                            .header("Dropbox-API-Arg", serde_json::to_string(&arg).unwrap())
                            .header("Content-Type", "application/octet-stream")
                            .body(buffer[..bytes_read].to_vec())
                            .send()
                            .map_err(|e| Error::AnyhowError { source: anyhow::anyhow!("Dropbox session finish error: {}", e) })?;

                        if !response.status().is_success() {
                            let err_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
                            return Err(Error::AnyhowError { source: anyhow::anyhow!("Dropbox session finish failed: {}", err_text) });
                        }
                        last_response = Some(response);
                    }
                }
                last_response.unwrap() 
            };

            if res.status().is_success() {
                info!(output, "{} -> {}", abs_path, dropbox_path);
                sent_paths.push(storage_path);
            } else {
                let err_text = res.text().unwrap_or_else(|_| "Unknown error".to_string());
                error!(output, "Failed to upload {}: {}", abs_path, err_text);
            }
        }

        Ok(XvcStorageSendEvent {
            guid: self.guid.clone(),
            paths: sent_paths,
        })
    }

    fn receive(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        _force: bool,
    ) -> Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> {
        let token = self.credentials()?;
        let client = reqwest::blocking::Client::new();
        let temp_dir = XvcStorageTempDir::new()?;
        let mut received_paths = Vec::new();

        for cache_path in paths {
            let storage_path = self.build_storage_path(xvc_root, cache_path);
            let dropbox_path = format!("/{}", storage_path.as_str());
            let abs_cache_dir = temp_dir.temp_cache_dir(cache_path)?;
            fs::create_dir_all(&abs_cache_dir)?;
            let abs_cache_path = temp_dir.temp_cache_path(cache_path)?;

            let mut response = client
                .post("https://content.dropboxapi.com/2/files/download")
                .header("Authorization", format!("Bearer {}", token))
                .header("Dropbox-API-Arg", serde_json::to_string(&serde_json::json!({ "path": dropbox_path })).unwrap())
                .send()
                .map_err(|e| Error::AnyhowError { source: anyhow::anyhow!("Dropbox download error: {}", e) })?;

            if response.status().is_success() {
                let mut file = fs::File::create(&abs_cache_path)?;
                response.copy_to(&mut file).map_err(|e| Error::AnyhowError { source: anyhow::anyhow!("Dropbox download copy error: {}", e) })?;
                info!(output, "{} -> {}", dropbox_path, abs_cache_path);
                received_paths.push(storage_path);
            } else {
                let err_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
                error!(output, "Failed to download {}: {}", dropbox_path, err_text);
            }
        }

        Ok((
            temp_dir,
            XvcStorageReceiveEvent {
                guid: self.guid.clone(),
                paths: received_paths,
            },
        ))
    }

    fn delete(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent> {
        let token = self.credentials()?;
        let client = reqwest::blocking::Client::new();
        let mut deleted_paths = Vec::new();

        for cache_path in paths {
            let storage_path = self.build_storage_path(xvc_root, cache_path);
            let dropbox_path = format!("/{}", storage_path.as_str());

            let arg = DeleteArg { path: dropbox_path.clone() };

            let response = client
                .post("https://api.dropboxapi.com/2/files/delete_v2")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .json(&arg)
                .send()
                .map_err(|e| Error::AnyhowError { source: anyhow::anyhow!("Dropbox delete error: {}", e) })?;

            if response.status().is_success() {
                info!(output, "[REMOTE DELETE] {}", dropbox_path);
                deleted_paths.push(storage_path);
            } else {
                let err_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
                error!(output, "Failed to delete {}: {}", dropbox_path, err_text);
            }
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
    ) -> Result<crate::storage::XvcStorageExpiringShareEvent> {
        Err(Error::StorageDoesNotSupportSignedUrls)
    }
}
