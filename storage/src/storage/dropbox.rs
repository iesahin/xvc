//! Dropbox remote storage implementation.
use std::env;
use std::fs;

use regex::Regex;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use xvc_core::R1NStore;
use xvc_core::XvcCachePath;
use xvc_core::XvcOutputSender;
use xvc_core::XvcRoot;
use xvc_core::{error, info, output};

use crate::{Error, Result, XvcStorage, XvcStorageEvent};
use crate::{XvcStorageGuid, XvcStorageOperations};

use super::{
    XVC_STORAGE_GUID_FILENAME, XvcStorageDeleteEvent, XvcStorageExpiringShareEvent,
    XvcStorageInitEvent, XvcStorageListEvent, XvcStoragePath, XvcStorageReceiveEvent,
    XvcStorageSendEvent, XvcStorageTempDir,
};

const DBX_RPC_URL: &str = "https://api.dropboxapi.com/2";
const DBX_CONTENT_URL: &str = "https://content.dropboxapi.com/2";

/// Dropbox temporary links (used for [XvcDropboxStorage::share]) are valid for four hours.
/// Dropbox doesn't allow the expiration duration to be configured.
const DBX_TEMPORARY_LINK_EXPIRATION_SECONDS: u32 = 4 * 60 * 60;

/// Entry point for `xvc storage new dropbox` command.
///
/// Creates a new [XvcDropboxStorage], calls its [init][XvcDropboxStorage::init] to write the
/// `.xvc-guid` file to the storage, and saves the storage record and init event in the ECS.
pub fn cmd_new_dropbox(
    _input: std::io::StdinLock,
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

    info!(output_snd, "Created Dropbox Storage: {:#?}", storage);

    Ok(())
}

/// A Dropbox storage.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcDropboxStorage {
    /// The GUID of the storage.
    pub guid: XvcStorageGuid,
    /// The name of the storage.
    pub name: String,
    /// The directory in Dropbox to store the files, without leading/trailing slashes.
    pub storage_prefix: String,
}

#[derive(Deserialize)]
struct DbxListFolderEntry {
    #[serde(rename = ".tag")]
    tag: String,
    path_lower: Option<String>,
}

#[derive(Deserialize)]
struct DbxListFolderResult {
    entries: Vec<DbxListFolderEntry>,
    cursor: String,
    has_more: bool,
}

#[derive(Deserialize)]
struct DbxTemporaryLinkResult {
    link: String,
}

impl XvcDropboxStorage {
    /// Storage prefix without leading/trailing slashes.
    fn storage_prefix_trimmed(&self) -> String {
        self.storage_prefix.trim_matches('/').to_string()
    }

    /// The relative storage path (no leading slash) of a cache path in the storage:
    /// `{storage_prefix}/{repo_guid}/{cache_path}`. This is used both as the actual Dropbox
    /// object key and as the [XvcStoragePath] value recorded in storage events, so it must use
    /// the same repository GUID that [XvcDropboxStorage::list] filters on.
    fn build_storage_path(&self, xvc_root: &XvcRoot, cache_path: &XvcCachePath) -> XvcStoragePath {
        let prefix = self.storage_prefix_trimmed();
        let xvc_guid = xvc_root.guid();
        let relative = if prefix.is_empty() {
            format!("{xvc_guid}/{cache_path}")
        } else {
            format!("{prefix}/{xvc_guid}/{cache_path}")
        };
        XvcStoragePath::from(relative)
    }

    /// Dropbox requires absolute paths, prefixed with `/`.
    fn to_dropbox_path(relative: &str) -> String {
        format!("/{relative}")
    }

    /// The Dropbox path of the `storage_prefix` directory. Root is represented as `""` in the
    /// Dropbox API, not `/`.
    fn root_dropbox_path(&self) -> String {
        let prefix = self.storage_prefix_trimmed();
        if prefix.is_empty() {
            String::new()
        } else {
            format!("/{prefix}")
        }
    }

    /// The Dropbox path of the `.xvc-guid` file.
    fn guid_dropbox_path(&self) -> String {
        let prefix = self.storage_prefix_trimmed();
        if prefix.is_empty() {
            Self::to_dropbox_path(XVC_STORAGE_GUID_FILENAME)
        } else {
            Self::to_dropbox_path(&format!("{prefix}/{XVC_STORAGE_GUID_FILENAME}"))
        }
    }

    /// Reads the access token from `XVC_STORAGE_ACCESS_TOKEN_<name>` or `DROPBOX_ACCESS_TOKEN`
    /// environment variables, in that order.
    fn access_token(&self) -> Result<String> {
        let specific_var = format!("XVC_STORAGE_ACCESS_TOKEN_{}", self.name);
        if let Ok(token) = env::var(&specific_var) {
            return Ok(token);
        }

        let generic_var = "DROPBOX_ACCESS_TOKEN";
        if let Ok(token) = env::var(generic_var) {
            return Ok(token);
        }

        Err(Error::CloudTokenNotFound {
            storage_name: self.name.clone(),
            vars: vec![specific_var, generic_var.to_string()],
        })
    }

    /// Calls a Dropbox RPC endpoint (`api.dropboxapi.com`) with a JSON body and returns the
    /// parsed JSON response.
    fn rpc_call(&self, endpoint: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let token = self.access_token()?;
        let client = Client::new();
        let response = client
            .post(format!("{DBX_RPC_URL}/{endpoint}"))
            .bearer_auth(token)
            .json(&body)
            .send()?;

        let status = response.status();
        let text = response.text()?;

        if !status.is_success() {
            return Err(Error::DropboxApiError(format!(
                "{endpoint}: {status}: {text}"
            )));
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// Uploads `content` to `dropbox_path`, overwriting any existing file.
    fn upload(&self, dropbox_path: &str, content: Vec<u8>) -> Result<()> {
        let token = self.access_token()?;
        let client = Client::new();
        let arg = json!({
            "path": dropbox_path,
            "mode": "overwrite",
            "autorename": false,
            "mute": true,
        });

        let response = client
            .post(format!("{DBX_CONTENT_URL}/files/upload"))
            .bearer_auth(token)
            .header("Dropbox-API-Arg", arg.to_string())
            .header("Content-Type", "application/octet-stream")
            .body(content)
            .send()?;

        let status = response.status();
        if !status.is_success() {
            let text = response.text()?;
            return Err(Error::DropboxApiError(format!(
                "files/upload: {status}: {text}"
            )));
        }

        Ok(())
    }

    /// Downloads and returns the content of `dropbox_path`.
    fn download(&self, dropbox_path: &str) -> Result<Vec<u8>> {
        let token = self.access_token()?;
        let client = Client::new();
        let arg = json!({ "path": dropbox_path });

        let response = client
            .post(format!("{DBX_CONTENT_URL}/files/download"))
            .bearer_auth(token)
            .header("Dropbox-API-Arg", arg.to_string())
            .send()?;

        let status = response.status();
        if !status.is_success() {
            let text = response.text()?;
            return Err(Error::DropboxApiError(format!(
                "files/download: {status}: {text}"
            )));
        }

        Ok(response.bytes()?.to_vec())
    }

    /// Deletes `dropbox_path` from the storage.
    fn delete_path(&self, dropbox_path: &str) -> Result<()> {
        self.rpc_call("files/delete_v2", json!({ "path": dropbox_path }))?;
        Ok(())
    }

    /// Recursively lists all file paths (relative, no leading slash) under `storage_prefix`.
    fn list_all_files(&self) -> Result<Vec<String>> {
        let mut paths = Vec::new();

        let mut result: DbxListFolderResult = serde_json::from_value(self.rpc_call(
            "files/list_folder",
            json!({ "path": self.root_dropbox_path(), "recursive": true }),
        )?)?;

        loop {
            for entry in result.entries {
                if entry.tag == "file"
                    && let Some(path_lower) = entry.path_lower
                {
                    paths.push(path_lower.trim_start_matches('/').to_string());
                }
            }

            if !result.has_more {
                break;
            }

            result = serde_json::from_value(self.rpc_call(
                "files/list_folder/continue",
                json!({ "cursor": result.cursor }),
            )?)?;
        }

        Ok(paths)
    }
}

impl XvcStorageOperations for XvcDropboxStorage {
    /// Writes the `.xvc-guid` file to `storage_prefix` in Dropbox.
    fn init(
        &mut self,
        output: &XvcOutputSender,
        _xvc_root: &XvcRoot,
    ) -> Result<XvcStorageInitEvent> {
        let guid_dropbox_path = self.guid_dropbox_path();
        self.upload(&guid_dropbox_path, self.guid.to_string().into_bytes())?;

        info!(output, "Initialized Dropbox storage at {guid_dropbox_path}");

        Ok(XvcStorageInitEvent {
            guid: self.guid.clone(),
        })
    }

    /// Lists all files in the storage that match the Xvc cache path pattern:
    ///
    /// {storage_prefix}/{XVC_GUID}/[a-zA-Z][0-9]/[0-9A-Fa-f]{3}/[0-9A-Fa-f]{3}/[0-9A-Fa-f]{58}/0
    fn list(&self, _output: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<XvcStorageListEvent> {
        let xvc_guid = xvc_root.guid();
        let prefix = self.storage_prefix_trimmed();
        let prefix_pattern = if prefix.is_empty() {
            String::new()
        } else {
            format!("{prefix}/")
        };

        let re = Regex::new(&format!(
            "^{prefix_pattern}{xvc_guid}/{cp}/{d3}/{d3}/{d58}/0\\..*$",
            cp = r#"[a-zA-Z][0-9]"#,
            d3 = r#"[0-9A-Fa-f]{3}"#,
            d58 = r#"[0-9A-Fa-f]{58}"#
        ))
        .unwrap();

        let paths = self
            .list_all_files()?
            .into_iter()
            .filter(|p| re.is_match(p))
            .map(XvcStoragePath::from)
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
        let mut sent_paths = Vec::<XvcStoragePath>::with_capacity(paths.len());

        for cache_path in paths {
            let storage_path = self.build_storage_path(xvc_root, cache_path);
            let dropbox_path = Self::to_dropbox_path(storage_path.as_str());
            let abs_cache_path = cache_path.to_absolute_path(xvc_root);

            match fs::read(&abs_cache_path) {
                Ok(content) => match self.upload(&dropbox_path, content) {
                    Ok(_) => {
                        info!(output, "{} -> {}", abs_cache_path, dropbox_path);
                        sent_paths.push(storage_path);
                    }
                    Err(err) => error!(output, "{}", err),
                },
                Err(err) => error!(output, "{}", err),
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
        let temp_dir = XvcStorageTempDir::new()?;
        let mut received_paths = Vec::<XvcStoragePath>::with_capacity(paths.len());

        for cache_path in paths {
            let storage_path = self.build_storage_path(xvc_root, cache_path);
            let dropbox_path = Self::to_dropbox_path(storage_path.as_str());

            match self.download(&dropbox_path) {
                Ok(content) => {
                    let cache_dir = temp_dir.temp_cache_dir(cache_path)?;
                    fs::create_dir_all(&cache_dir)?;
                    let local_path = temp_dir.temp_cache_path(cache_path)?;
                    fs::write(&local_path, content)?;
                    info!(output, "{} -> {}", dropbox_path, local_path);
                    received_paths.push(storage_path);
                }
                Err(err) => error!(output, "{}", err),
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
        let mut deleted_paths = Vec::<XvcStoragePath>::with_capacity(paths.len());

        for cache_path in paths {
            let storage_path = self.build_storage_path(xvc_root, cache_path);
            let dropbox_path = Self::to_dropbox_path(storage_path.as_str());

            match self.delete_path(&dropbox_path) {
                Ok(_) => {
                    info!(output, "[DELETE] {}", dropbox_path);
                    deleted_paths.push(storage_path);
                }
                Err(err) => error!(output, "{}", err),
            }
        }

        Ok(XvcStorageDeleteEvent {
            guid: self.guid.clone(),
            paths: deleted_paths,
        })
    }

    /// Shares a file with a Dropbox temporary link, valid for four hours.
    ///
    /// Dropbox doesn't support arbitrary expiration durations for temporary links, so `period`
    /// is ignored.
    fn share(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        path: &XvcCachePath,
        _period: std::time::Duration,
    ) -> Result<XvcStorageExpiringShareEvent> {
        let storage_path = self.build_storage_path(xvc_root, path);
        let dropbox_path = Self::to_dropbox_path(storage_path.as_str());

        let result: DbxTemporaryLinkResult = serde_json::from_value(
            self.rpc_call("files/get_temporary_link", json!({ "path": dropbox_path }))?,
        )?;

        info!(output, "[SHARED] {}", dropbox_path);
        output!(output, "{}", result.link);

        Ok(XvcStorageExpiringShareEvent {
            guid: self.guid.clone(),
            path: storage_path,
            signed_url: result.link,
            expiration_seconds: DBX_TEMPORARY_LINK_EXPIRATION_SECONDS,
        })
    }
}
