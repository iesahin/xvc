//! Rsync remote storage implementation.
use std::{env, fs};

use regex::Regex;
use serde::{Deserialize, Serialize};
use subprocess::{CaptureData, Exec};
use xvc_core::{XvcCachePath, XvcRoot};
use xvc_ecs::R1NStore;
use xvc_logging::{error, info, uwr, warn, watch, XvcOutputSender};
use xvc_walker::AbsolutePath;

use crate::{Error, Result, XvcStorage, XvcStorageEvent, XvcStorageGuid, XvcStorageOperations};

use super::{
    XvcStorageDeleteEvent, XvcStorageInitEvent, XvcStorageListEvent, XvcStoragePath,
    XvcStorageReceiveEvent, XvcStorageSendEvent, XvcStorageTempDir, XVC_STORAGE_GUID_FILENAME,
};

/// Entry point for `xvc storage new rsync` command.
/// Creates a new Rsync storage with the given options.
/// It creates a new [`XvcRsyncStorage`], uploads the `.xvc-guid` file and
/// records the storage.
///
/// If the connection options are not valid, [XvcRsyncStorage::init] will fail,
/// and this function will return an error before recording the storage.
pub fn cmd_new_rsync(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    name: String,
    host: String,
    port: Option<usize>,
    user: Option<String>,
    storage_dir: String,
) -> Result<()> {
    let storage = XvcRsyncStorage {
        guid: XvcStorageGuid::new(),
        name,
        host,
        port,
        user,
        storage_dir,
    };

    watch!(storage);

    let (init_event, storage) = storage.init(output_snd, xvc_root)?;

    xvc_root.with_r1nstore_mut(|store: &mut R1NStore<XvcStorage, XvcStorageEvent>| {
        let store_e = xvc_root.new_entity();
        let event_e = xvc_root.new_entity();
        store.insert(
            store_e,
            XvcStorage::Rsync(storage.clone()),
            event_e,
            XvcStorageEvent::Init(init_event.clone()),
        );
        Ok(())
    })?;

    Ok(())
}

/// Specifies an Rsync remote
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcRsyncStorage {
    /// The GUID of the storage
    pub guid: XvcStorageGuid,
    /// The name of the storage
    pub name: String,
    /// The host name of the storage without any protocol prefix
    pub host: String,
    /// The port of the storage
    pub port: Option<usize>,
    /// The user to connect to the storage
    pub user: Option<String>,
    /// The storage directory on the remote
    pub storage_dir: String,
}

impl XvcRsyncStorage {
    fn ssh_url(&self) -> String {
        match (self.port, &self.user) {
            (None, None) => self.host.to_string(),
            (Some(port), None) => format!("{}:{port}", self.host),
            (None, Some(user)) => format!("{user}@{}", self.host),
            (Some(port), Some(user)) => format!("{user}@{}:{port}", self.host),
        }
    }

    fn ssh_cmd(&self, ssh_executable: &AbsolutePath, cmd: &str) -> Result<CaptureData> {
        let ssh_url = self.ssh_url();
        watch!(ssh_url);
        let cmd_res = Exec::cmd(ssh_executable.as_path())
            .arg(ssh_url)
            .arg(cmd)
            .capture();

        watch!(cmd_res);

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

    fn ssh_cache_path(&self, xvc_guid: &str, cache_path: &XvcCachePath) -> String {
        let storage_dir = self.storage_dir.trim_end_matches('/');
        format!("{storage_dir}/{xvc_guid}/{cache_path}")
    }

    fn ssh_cache_dir(&self, xvc_guid: &str, cache_path: &XvcCachePath) -> String {
        let storage_dir = self.storage_dir.trim_end_matches('/');
        let remote_dir = cache_path.directory();
        format!("{storage_dir}/{xvc_guid}/{remote_dir}")
    }

    fn rsync_path_url(&self, path: &str) -> String {
        let storage_dir = self.storage_dir.trim_end_matches('/');
        match (self.port, &self.user) {
            (None, None) => format!("{}:{storage_dir}/{path}", self.host),
            (Some(port), None) => format!("{}:{port}:{storage_dir}/{path}", self.host),
            (None, Some(user)) => format!("{user}@{}:{storage_dir}/{path}", self.host),
            (Some(port), Some(user)) => {
                format!("{user}@{}:{port}:{storage_dir}/{path}", self.host,)
            }
        }
    }

    fn rsync_cache_url(&self, xvc_guid: &str, path: &XvcCachePath) -> String {
        let storage_dir = self.storage_dir.trim_end_matches('/');
        match (self.port, &self.user) {
            (None, None) => format!("{}:{storage_dir}/{xvc_guid}/{path}", self.host),
            (Some(port), None) => format!("{}:{port}:{storage_dir}/{xvc_guid}/{path}", self.host),
            (None, Some(user)) => format!("{user}@{}:{storage_dir}/{xvc_guid}/{path}", self.host),
            (Some(port), Some(user)) => format!(
                "{user}@{}:{port}:{storage_dir}/{xvc_guid}/{path}",
                self.host,
            ),
        }
    }

    fn rsync_copy_to_storage(
        &self,
        rsync_executable: &AbsolutePath,
        local_path: &AbsolutePath,
        remote_url: &str,
    ) -> Result<CaptureData> {
        let rsync_opts = "-av";

        let cmd_res = Exec::cmd(rsync_executable.as_path())
            .arg(rsync_opts)
            .arg(local_path.to_string())
            .arg(remote_url)
            .capture();

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

    fn rsync_copy_from_storage(
        &self,
        rsync_executable: &AbsolutePath,
        remote_url: &str,
        local_path: &AbsolutePath,
    ) -> Result<CaptureData> {
        watch!(remote_url);
        watch!(local_path);

        let rsync_opts = "-av";

        let cmd_res = Exec::cmd(rsync_executable.as_path())
            .arg(rsync_opts)
            .arg(remote_url)
            .arg(local_path.to_string())
            .capture();

        watch!(cmd_res);

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

    #[cfg(unix)]
    fn ssh_executable() -> Result<AbsolutePath> {
        Ok(AbsolutePath::from(which::which("ssh")?))
    }

    #[cfg(windows)]
    fn ssh_executable() -> Result<AbsolutePath> {
        Ok(AbsolutePath::from(which::which("ssh.exe")?))
    }

    #[cfg(unix)]
    fn rsync_executable() -> Result<AbsolutePath> {
        Ok(AbsolutePath::from(which::which("rsync")?))
    }

    #[cfg(windows)]
    fn rsync_executable() -> Result<AbsolutePath> {
        Ok(AbsolutePath::from(which::which("rsync.exe")?))
    }

    fn create_storage_dir(
        &self,
        ssh_executable: &AbsolutePath,
        xvc_guid: &str,
        cache_path: &XvcCachePath,
    ) -> Result<CaptureData> {
        let remote_dir = self.ssh_cache_dir(xvc_guid, cache_path);
        self.ssh_cmd(ssh_executable, &format!("mkdir -p '{}'", remote_dir))
    }
}

impl XvcStorageOperations for XvcRsyncStorage {
    /// Initialize the repository by copying guid file to remote storage directory.
    fn init(
        self,
        output: &XvcOutputSender,
        _xvc_root: &XvcRoot,
    ) -> Result<(super::XvcStorageInitEvent, Self)> {
        // "--init",
        // "ssh {URL} 'mkdir -p {STORAGE_DIR}' ; rsync -av {LOCAL_GUID_FILE_PATH} {URL}:{STORAGE_GUID_FILE_PATH}",
        //

        let ssh_executable = Self::ssh_executable()?;
        let rsync_executable = Self::rsync_executable()?;

        watch!(ssh_executable);
        watch!(rsync_executable);

        self.ssh_cmd(&ssh_executable, &format!("mkdir -p '{}'", self.storage_dir))?;

        let local_guid_path = AbsolutePath::from(env::temp_dir().join(self.guid.to_string()));
        fs::write(&local_guid_path, format!("{}", self.guid))?;
        watch!(local_guid_path);

        let remote_guid_path = self.rsync_path_url(XVC_STORAGE_GUID_FILENAME);

        watch!(remote_guid_path);

        let rsync_result =
            self.rsync_copy_to_storage(&rsync_executable, &local_guid_path, &remote_guid_path)?;
        watch!(rsync_result);

        info!(
            output,
            "Initialized:\n{}\n{}\n",
            remote_guid_path,
            rsync_result.stdout_str()
        );

        fs::remove_file(&local_guid_path)?;

        Ok((
            XvcStorageInitEvent {
                guid: self.guid.clone(),
            },
            self,
        ))
    }

    /// Lists all files in the remote directory that match to regex:
    ///
    /// {XVC_GUID}/[a-zA-Z][0-9]/[0-9A-Fa-f]{3}/[0-9A-Fa-f]{3}/[0-9A-Fa-f]{58}/0
    fn list(&self, _output: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<XvcStorageListEvent> {
        // "--list",
        // "ssh {URL} 'ls -1R {STORAGE_DIR}'",
        //
        let ssh_executable = Self::ssh_executable()?;

        let cmd_output = self.ssh_cmd(&ssh_executable, &format!("ls -1R {}", self.storage_dir))?;
        let xvc_guid = xvc_root.config().guid().unwrap();
        let re = Regex::new(&format!(
            "{xvc_guid}/{cp}/{d3}/{d3}/{d58}/0\\..*$",
            cp = r#"[a-zA-Z][0-9]"#,
            d3 = r#"[0-9A-Fa-f]{3}"#,
            d58 = r#"[0-9A-Fa-f]{58}"#
        ))
        .unwrap();

        let paths = cmd_output
            .stdout_str()
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
        // "--upload",
        // "ssh {URL} 'mkdir -p {STORAGE_DIR}{XVC_GUID}/{RELATIVE_CACHE_DIR}' ; rsync -av {ABSOLUTE_CACHE_PATH} {URL}:{STORAGE_DIR}{XVC_GUID}/{RELATIVE_CACHE_PATH}",
        //
        // TODO: We can make this parallel

        let rsync_executable = Self::rsync_executable()?;
        let ssh_executable = Self::ssh_executable()?;

        let xvc_guid = xvc_root.config().guid().expect("Repo GUID");
        let mut storage_paths = Vec::<XvcStoragePath>::with_capacity(paths.len());
        paths.iter().for_each(|cache_path| {
            let local_path = cache_path.to_absolute_path(xvc_root);
            let remote_url = self.rsync_cache_url(&xvc_guid, cache_path);
            let _remote_dir_res = self.create_storage_dir(&ssh_executable, &xvc_guid, cache_path);
            // TODO: Handle possible error.
            let cmd_output =
                self.rsync_copy_to_storage(&rsync_executable, &local_path, &remote_url);

            match cmd_output {
                Ok(cmd_output) => {
                    let stdout_str = cmd_output.stdout_str();
                    let stderr_str = cmd_output.stderr_str();
                    watch!(stdout_str);
                    watch!(stderr_str);
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
        // "--download",
        // "mkdir -p {ABSOLUTE_CACHE_DIR} ; rsync -av {URL}:{STORAGE_DIR}{XVC_GUID}/{RELATIVE_CACHE_PATH} {ABSOLUTE_CACHE_PATH}",
        //
        let rsync_executable = Self::rsync_executable()?;
        let temp_dir = XvcStorageTempDir::new()?;

        let xvc_guid = xvc_root.config().guid().expect("Repo GUID");
        let mut storage_paths = Vec::<XvcStoragePath>::with_capacity(paths.len());
        paths.iter().for_each(|cache_path| {
            let local_path = temp_dir.temp_cache_path(cache_path).unwrap();
            let remote_url = self.rsync_cache_url(&xvc_guid, cache_path);
            let cache_dir = temp_dir.temp_cache_dir(cache_path).unwrap();
            watch!(cache_dir);
            if !cache_dir.exists() {
                watch!(cache_dir);
                uwr!(fs::create_dir_all(&cache_dir), output);
            }

            watch!(remote_url);

            let cmd_output =
                self.rsync_copy_from_storage(&rsync_executable, &remote_url, &local_path);

            match cmd_output {
                Ok(cmd_output) => {
                    let stdout_str = cmd_output.stdout_str();
                    let stderr_str = cmd_output.stderr_str();
                    watch!(stdout_str);
                    watch!(stderr_str);
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
        let ssh_executable = Self::ssh_executable()?;

        let xvc_guid = xvc_root.config().guid().expect("Repo GUID");
        let mut storage_paths = Vec::<XvcStoragePath>::with_capacity(paths.len());
        paths.iter().for_each(|cache_path| {
            let remote_path = self.ssh_cache_path(xvc_guid.as_str(), cache_path);
            let delete_cmd = format!("rm -f '{}'", remote_path);
            let cmd_output = self.ssh_cmd(&ssh_executable, &delete_cmd);

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
}
