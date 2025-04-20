//! Rclone storage implementation
//!
//! This module implements the XvcStorage trait for Rclone storage. It uses the rclone command line
//! for the time being. In the future, it can use the rclone library or RPC interface.
//!
//! TODO: Use the rclone library or RPC interface instead of the command line.
use std::env;
use std::fs;

use regex::Regex;
use serde::{Deserialize, Serialize};
use subprocess::{CaptureData, Exec};
use xvc_core::AbsolutePath;
use xvc_core::XvcCachePath;
use xvc_core::XvcRoot;
use xvc_core::R1NStore;
use xvc_core::{error, info, trace, uwr, warn, XvcOutputSender};

use crate::{Error, Result, XvcStorage, XvcStorageEvent, XvcStorageGuid, XvcStorageOperations};

use super::{
    XvcStorageDeleteEvent, XvcStorageInitEvent, XvcStorageListEvent, XvcStoragePath,
    XvcStorageReceiveEvent, XvcStorageSendEvent, XvcStorageTempDir, XVC_STORAGE_GUID_FILENAME,
};

/// Add a new Rclone storage to the repository
pub fn cmd_new_rclone(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    name: String,
    remote: String,
    storage_prefix: String,
) -> Result<()> {
    let mut storage = XvcRcloneStorage {
        guid: XvcStorageGuid::new(),
        name,
        remote,
        storage_prefix,
    };

    let init_event = storage.init(output_snd, xvc_root)?;

    xvc_root.with_r1nstore_mut(|store: &mut R1NStore<XvcStorage, XvcStorageEvent>| {
        let store_e = xvc_root.new_entity();
        let event_e = xvc_root.new_entity();
        store.insert(
            store_e,
            XvcStorage::Rclone(storage.clone()),
            event_e,
            XvcStorageEvent::Init(init_event.clone()),
        );
        Ok(())
    })?;

    info!(output_snd, "Added Rclone Storage: {:#?}", storage);

    Ok(())
}

/// Specifies an Rsync remote storage
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcRcloneStorage {
    /// The GUID of the storage
    pub guid: XvcStorageGuid,
    /// The name of the storage
    pub name: String,
    /// Rclone identifier for the remote
    /// e.g. remote (without :) in remote://dir/
    pub remote: String,
    /// The storage directory on the remote
    /// e.g. dir/ in remote://dir/
    pub storage_prefix: String,
}

#[cfg(unix)]
fn rclone_executable() -> Result<AbsolutePath> {
    // TODO: Make rclone executable configurable
    let rclone_executable = which::which("rclone")?;
    Ok(AbsolutePath::from(rclone_executable))
}

#[cfg(windows)]
fn rclone_executable() -> Result<AbsolutePath> {
    let rclone_executable = which::which("rclone.exe")?;
    Ok(AbsolutePath::from(rclone_executable))
}

/// Run rclone with the given executable, options, command and urls.
///
/// Get `rclone_executable` from the [rclone_executable] function by supplying a command.
/// Note that the second URL is optional for commands like `rclone ls myremote:dir/`
pub fn rclone_cmd(
    rclone_executable: &AbsolutePath,
    options: &str,
    command: &str,
    first_url: &str,
    second_url: Option<&str>,
) -> Result<CaptureData> {
    let second_url = second_url.unwrap_or("");
    trace!(second_url);

    let mut cmd = Exec::cmd(rclone_executable.as_path());
    if !options.is_empty() {
        cmd = cmd.arg(options);
    }

    cmd = cmd.arg(command).arg(first_url);

    if !second_url.is_empty() {
        cmd = cmd.arg(second_url);
    }

    let cmd_res = cmd.capture();

    match cmd_res {
        Ok(res) => match res.exit_status {
            subprocess::ExitStatus::Exited(0) => Ok(res),
            subprocess::ExitStatus::Exited(_)
            | subprocess::ExitStatus::Signaled(_)
            | subprocess::ExitStatus::Other(_)
            | subprocess::ExitStatus::Undetermined => Err(Error::ProcessError {
                stdout: res.stdout_str(),
                stderr: res.stderr_str(),
            }),
        },
        Err(e) => Err(e.into()),
    }
}

impl XvcRcloneStorage {
    fn rclone_path_url(&self, path: &str) -> String {
        let storage_dir = self
            .storage_prefix
            .trim_start_matches('/')
            .trim_end_matches('/');

        let remote_name = self.remote.trim_end_matches(':');
        if storage_dir.is_empty() {
            format!("{remote_name}:/{path}")
        } else {
            format!("{remote_name}:/{storage_dir}/{path}")
        }
    }

    fn rclone_cache_url(&self, xvc_guid: &str, path: &XvcCachePath) -> String {
        let storage_dir = self
            .storage_prefix
            .trim_start_matches('/')
            .trim_end_matches('/');
        let remote_name = self.remote.trim_end_matches(':');
        if storage_dir.is_empty() {
            format!("{remote_name}:/{xvc_guid}/{path}")
        } else {
            format!("{remote_name}:/{storage_dir}/{xvc_guid}/{path}")
        }
    }

    fn rclone_copy_to_storage(
        &self,
        rclone_executable: &AbsolutePath,
        local_path: &AbsolutePath,
        remote_url: &str,
    ) -> Result<CaptureData> {
        trace!(remote_url);
        trace!(local_path.to_string());
        trace!(remote_url);

        // With copyto we need to drop the filename in the remote path
        // e.g. remote://dir/file.txt -> remote://dir/
        //
        // This uses copy instead, to rename files

        rclone_cmd(
            rclone_executable,
            "",
            "copy",
            &local_path.to_string(),
            Some(remote_url),
        )
    }

    fn rclone_copy_from_storage(
        &self,
        rclone_executable: &AbsolutePath,
        remote_url: &str,
        local_path: &AbsolutePath,
    ) -> Result<CaptureData> {
        trace!(remote_url);

        rclone_cmd(
            rclone_executable,
            "",
            "copy",
            remote_url,
            Some(&local_path.to_string()),
        )
    }

    fn create_storage_dir(
        &self,
        rclone_executable: &AbsolutePath,
        xvc_guid: &str,
        cache_path: &XvcCachePath,
    ) -> Result<CaptureData> {
        let storage_dir = self.rclone_cache_url(xvc_guid, cache_path);
        rclone_cmd(rclone_executable, "", "mkdir", &storage_dir, None)
    }
}

impl XvcStorageOperations for XvcRcloneStorage {
    fn init(
        &mut self,
        output: &XvcOutputSender,
        _xvc_root: &XvcRoot,
    ) -> Result<super::XvcStorageInitEvent> {
        let rclone_executable = rclone_executable()?;

        trace!(rclone_executable);

        if !self.storage_prefix.is_empty() {
            rclone_cmd(
                &rclone_executable,
                "",
                "mkdir",
                &self.rclone_path_url(&self.storage_prefix),
                None,
            )?;
        }

        let local_guid_path = AbsolutePath::from(env::temp_dir().join(self.guid.to_string()));
        fs::write(&local_guid_path, format!("{}", self.guid))?;

        let storage_guid_path = self.rclone_path_url(XVC_STORAGE_GUID_FILENAME);

        let rclone_result =
            self.rclone_copy_to_storage(&rclone_executable, &local_guid_path, &storage_guid_path)?;

        info!(
            output,
            "Initialized:\n{}\n{}\n",
            storage_guid_path,
            rclone_result.stdout_str()
        );

        fs::remove_file(&local_guid_path)?;

        Ok(XvcStorageInitEvent {
            guid: self.guid.clone(),
        })
    }

    /// Lists all files in rclone remote directory that match to the regex:
    ///
    /// {XVC_GUID}/[a-zA-Z][0-9]/[0-9A-Fa-f]{3}/[0-9A-Fa-f]{3}/[0-9A-Fa-f]{58}/0
    fn list(&self, _output: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<XvcStorageListEvent> {
        let rclone_executable = rclone_executable()?;
        let storage_dir = self.storage_prefix.trim_end_matches('/');
        let storage_url = self.rclone_path_url(storage_dir);

        let cmd_output = rclone_cmd(&rclone_executable, "", "ls", &storage_url, None)?.stdout_str();
        let xvc_guid = xvc_root.config().guid().unwrap();
        // TODO: Move this regex to a central place
        let re = Regex::new(&format!(
            "{xvc_guid}/{cp}/{d3}/{d3}/{d58}/0\\..*$",
            cp = r#"[a-zA-Z][0-9]"#,
            d3 = r#"[0-9A-Fa-f]{3}"#,
            d58 = r#"[0-9A-Fa-f]{58}"#
        ))
        .unwrap();

        let paths = cmd_output
            .lines()
            .filter_map(|l| {
                if re.is_match(l) {
                    Some(XvcStoragePath::from(String::from(l)))
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

    fn send(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        _force: bool,
    ) -> Result<XvcStorageSendEvent> {
        // TODO: Parallelize this

        let rclone_executable = rclone_executable()?;

        let xvc_guid = xvc_root.config().guid().expect("Repo GUID");
        let mut storage_paths = Vec::<XvcStoragePath>::with_capacity(paths.len());
        paths.iter().for_each(|cache_path| {
            let local_path = cache_path.to_absolute_path(xvc_root);
            let storage_url = self.rclone_cache_url(&xvc_guid, cache_path);
            uwr!(
                self.create_storage_dir(&rclone_executable, &xvc_guid, cache_path),
                output
            );
            // TODO: Handle possible error.
            let cmd_output =
                self.rclone_copy_to_storage(&rclone_executable, &local_path, &storage_url);

            match cmd_output {
                Ok(cmd_output) => {
                    let stdout_str = cmd_output.stdout_str();
                    let stderr_str = cmd_output.stderr_str();
                    info!(output, "{}", stdout_str);
                    warn!(output, "{}", stderr_str);
                    let storage_path = XvcStoragePath::new(xvc_root, cache_path);
                    storage_paths.push(storage_path);
                }

                Err(err) => {
                    error!(output, "{}", err);
                }
            }
        });

        Ok(XvcStorageSendEvent {
            guid: self.guid.clone(),
            paths: storage_paths,
        })
    }

    fn receive(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        _force: bool,
    ) -> Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> {
        let rsync_executable = rclone_executable()?;
        let temp_dir = XvcStorageTempDir::new()?;

        let xvc_guid = xvc_root.config().guid().expect("Repo GUID");
        let mut storage_paths = Vec::<XvcStoragePath>::with_capacity(paths.len());
        paths.iter().for_each(|cache_path| {
            let local_path = temp_dir.temp_cache_path(cache_path).unwrap();
            let remote_url = self.rclone_cache_url(&xvc_guid, cache_path);
            let cache_dir = temp_dir.temp_cache_dir(cache_path).unwrap();
            trace!(cache_dir);
            if !cache_dir.exists() {
                uwr!(fs::create_dir_all(&cache_dir), output);
            }

            trace!(remote_url);

            let cmd_output =
                self.rclone_copy_from_storage(&rsync_executable, &remote_url, &local_path);

            match cmd_output {
                Ok(cmd_output) => {
                    let stdout_str = cmd_output.stdout_str();
                    let stderr_str = cmd_output.stderr_str();
                    info!(output, "{}", stdout_str);
                    warn!(output, "{}", stderr_str);
                    let storage_path = XvcStoragePath::new(xvc_root, cache_path);
                    storage_paths.push(storage_path);
                }

                Err(err) => {
                    error!(output, "{}", err);
                }
            }
        });

        Ok((
            temp_dir,
            XvcStorageReceiveEvent {
                guid: self.guid.clone(),
                paths: storage_paths,
            },
        ))
    }

    fn delete(
        &self,
        output: &XvcOutputSender,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent> {
        // "--delete",
        // "ssh {URL} 'rm {STORAGE_DIR}{RELATIVE_CACHE_PATH}'",
        //
        let rclone_executable = rclone_executable()?;

        let xvc_guid = xvc_root.config().guid().expect("Repo GUID");
        let mut storage_paths = Vec::<XvcStoragePath>::with_capacity(paths.len());
        paths.iter().for_each(|cache_path| {
            let remote_path = self.rclone_cache_url(xvc_guid.as_str(), cache_path);
            let cmd_output = rclone_cmd(
                &rclone_executable,
                "",
                "delete",
                &self.rclone_path_url(""),
                None,
            );

            match cmd_output {
                Ok(cmd_output) => {
                    let stdout_str = cmd_output.stdout_str();
                    let stderr_str = cmd_output.stderr_str();
                    info!(output, "[REMOTE DELETE] {}", remote_path);
                    info!(output, "{}", stdout_str);
                    warn!(output, "{}", stderr_str);
                    let storage_path = XvcStoragePath::new(xvc_root, cache_path);
                    storage_paths.push(storage_path);
                }

                Err(err) => {
                    error!(output, "{}", err);
                }
            }
        });
        Ok(XvcStorageDeleteEvent {
            guid: self.guid.clone(),
            paths: storage_paths,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rclone_drive_list() {
        let rclone_exec = rclone_executable().map_err(|e| format!("Failed to find rclone executable: {}", e))?;
        let drive_list = rclone_cmd(&rclone_exec, "", "ls", "drive://", None);

        match drive_list {
            Ok(cmd_output) => {
                let stdout_str = cmd_output.stdout_str();
                let lines = stdout_str.lines();
                assert!(lines.count() > 1000);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
