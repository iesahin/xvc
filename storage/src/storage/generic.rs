use std::{collections::HashMap, env, fs, path::Path};

use anyhow::anyhow;
use crossbeam_channel::Sender;
use regex::Regex;
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use subprocess::Exec;
use xvc_core::{XvcCachePath, XvcRoot};
use xvc_ecs::R1NStore;
use xvc_logging::{watch, XvcOutputLine};

use crate::{Error, Result, XvcStorage, XvcStorageEvent, XvcStorageGuid, XvcStorageOperations};

use super::{
    XvcStorageDeleteEvent, XvcStorageInitEvent, XvcStorageListEvent, XvcStoragePath,
    XvcStorageReceiveEvent, XvcStorageSendEvent, XVC_STORAGE_GUID_FILENAME,
};

pub fn cmd_storage_new_generic(
    input: std::io::StdinLock,
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    name: String,
    url: Option<String>,
    storage_dir: Option<String>,
    max_processes: usize,
    init_command: String,
    list_command: String,
    download_command: String,
    upload_command: String,
    delete_command: String,
) -> Result<()> {
    let storage = XvcGenericStorage {
        guid: XvcStorageGuid::new(),
        name,
        url,
        storage_dir,
        init_command,
        list_command,
        upload_command,
        download_command,
        delete_command,
        max_processes,
    };

    watch!(storage);

    let (init_event, storage) = storage.init(output_snd.clone(), xvc_root)?;

    xvc_root.with_r1nstore_mut(|store: &mut R1NStore<XvcStorage, XvcStorageEvent>| {
        let store_e = xvc_root.new_entity();
        let event_e = xvc_root.new_entity();
        store.insert(
            store_e,
            XvcStorage::Generic(storage.clone()),
            event_e,
            XvcStorageEvent::Init(init_event.clone()),
        );
        Ok(())
    })?;

    Ok(())
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcGenericStorage {
    pub guid: XvcStorageGuid,
    pub name: String,
    pub url: Option<String>,
    pub storage_dir: Option<String>,
    pub init_command: String,
    pub list_command: String,
    pub upload_command: String,
    pub download_command: String,
    pub delete_command: String,
    pub max_processes: usize,
}

impl XvcGenericStorage {
    /// Replace keys with values in `template` using `hash_map`
    fn replace_map_elements(template: &str, hash_map: &HashMap<&str, String>) -> String {
        let mut out = template.to_string();
        for (pat, val) in hash_map {
            out = out.replace(pat, val);
        }
        out
    }
    /// returns a hash map that contains keys and values of address elements in commands
    /// - `{URL}` : The content of `--url` option. (default "")
    /// - `{STORAGE_DIR}` Content of `--storage-dir`  option. (default "")
    fn address_map(&self) -> HashMap<&str, String> {
        let hm = HashMap::from([
            ("{URL}", self.url.clone().unwrap_or_else(|| "".to_string())),
            (
                "{STORAGE_DIR}",
                self.storage_dir.clone().unwrap_or_else(|| "".to_string()),
            ),
        ]);
        hm
    }

    /// returns a map that contains keys and values for path elements in commands
    /// - `{XVC_GUID}`: The repository GUID used in storage paths.
    /// - `{RELATIVE_CACHE_PATH}` The portion of the cache path after `.xvc/`.
    /// - `{ABSOLUTE_CACHE_PATH}` The absolute local path for the cache element
    /// - `{RELATIVE_CACHE_DIR}` The portion of directory that contains the file after `.xvc/`
    /// - `{ABSOLUTE_CACHE_DIR}` The portion of the local directory that contains the file after `.xvc`
    /// - `{FULL_STORAGE_PATH}`: Concatenation of `{URL}{STORAGE_DIR}{XVC_GUID}/{RELATIVE_CACHE_PATH}`
    /// - `{FULL_STORAGE_DIR}`: Concatenation of `{URL}{STORAGE_DIR}{XVC_GUID}/{RELATIVE_CACHE_DIR}`
    fn path_map(&self, xvc_root: &XvcRoot, cache_path: &XvcCachePath) -> HashMap<&str, String> {
        let xvc_guid = xvc_root.config().guid().unwrap();
        let relative_cache_path = cache_path.to_string();
        let relative_cache_dir = cache_path
            .as_ref()
            .parent()
            .unwrap_or_else(|| RelativePath::new(""))
            .to_string();
        let absolute_cache_path = cache_path
            .to_absolute_path(xvc_root)
            .to_string_lossy()
            .to_string();
        let absolute_cache_dir = cache_path
            .to_absolute_path(xvc_root)
            .parent()
            .unwrap_or(Path::new(""))
            .to_string_lossy()
            .to_string();
        let url = self.url.clone().unwrap_or_else(|| "".to_string());
        let storage_dir = self.storage_dir.clone().unwrap_or_else(|| "".to_string());

        let full_storage_path = format!("{url}{storage_dir}{xvc_guid}/{relative_cache_path}");
        let full_storage_dir = format!("{url}{storage_dir}{xvc_guid}/{relative_cache_dir}");

        let hm = HashMap::from([
            ("{XVC_GUID}", xvc_guid),
            ("{RELATIVE_CACHE_PATH}", relative_cache_path),
            ("{ABSOLUTE_CACHE_PATH}", absolute_cache_path),
            ("{RELATIVE_CACHE_DIR}", relative_cache_dir),
            ("{ABSOLUTE_CACHE_DIR}", absolute_cache_dir),
            ("{FULL_STORAGE_PATH}", full_storage_path),
            ("{FULL_STORAGE_DIR}", full_storage_dir),
        ]);

        hm
    }

    fn run_for_paths(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
        prepared_cmd: &str,
        paths: &[XvcCachePath],
    ) -> Vec<XvcStoragePath> {
        let mut storage_paths = Vec::<XvcStoragePath>::with_capacity(paths.len());
        // TODO: Create a thread/process pool here
        // TODO: Refactor to use XvcStoragePath and XvcCachePath in replacements
        paths.iter().for_each(|cache_path| {
            let pm = self.path_map(xvc_root, cache_path);
            watch!(pm);
            let cmd = Self::replace_map_elements(prepared_cmd, &pm);
            watch!(cmd);
            let cmd_output = Exec::shell(cmd).capture();
            match cmd_output {
                Ok(cmd_output) => {
                    let stdout_str = cmd_output.stdout_str();
                    let stderr_str = cmd_output.stderr_str();
                    watch!(stdout_str);
                    watch!(stderr_str);

                    if cmd_output.success() {
                        output.send(XvcOutputLine::Info(stdout_str)).unwrap();
                        output.send(XvcOutputLine::Warn(stderr_str)).unwrap();
                        let storage_path = XvcStoragePath::new(xvc_root, cache_path);
                        storage_paths.push(storage_path);
                    } else {
                        output.send(XvcOutputLine::Error(stderr_str)).unwrap();
                        output.send(XvcOutputLine::Warn(stdout_str)).unwrap();
                    }
                }

                Err(err) => {
                    output.send(XvcOutputLine::Error(err.to_string())).unwrap();
                }
            }
        });

        storage_paths
    }
}

impl XvcStorageOperations for XvcGenericStorage {
    /// Run self.init_command
    ///
    /// The command should have {LOCAL_GUID_FILE_PATH}  and {STORAGE_GUID_FILE_PATH} fields to
    /// upload the guid file.
    fn init(
        self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
    ) -> Result<(super::XvcStorageInitEvent, Self)> {
        let mut address_map = self.address_map();
        watch!(address_map);
        let local_guid_path = env::temp_dir().join(self.guid.to_string());
        watch!(local_guid_path);

        fs::write(&local_guid_path, format!("{}", self.guid))?;

        address_map.insert(
            "{LOCAL_GUID_FILE_PATH}",
            local_guid_path.clone().to_string_lossy().to_string(),
        );

        let storage_guid_file_path = format!(
            "{}{}",
            address_map["{STORAGE_DIR}"], XVC_STORAGE_GUID_FILENAME
        );

        address_map.insert("{STORAGE_GUID_FILE_PATH}", storage_guid_file_path);

        let prepared_init_cmd = Self::replace_map_elements(&self.init_command, &address_map);
        watch!(prepared_init_cmd);
        let init_output = Exec::shell(prepared_init_cmd.clone())
            .capture()?
            .stdout_str();

        watch!(init_output);

        output.send(XvcOutputLine::Info(format!(
            "Run init command:\n{}\n{}\n",
            prepared_init_cmd, init_output,
        )))?;

        fs::remove_file(&local_guid_path)?;

        Ok((
            XvcStorageInitEvent {
                guid: self.guid.clone(),
            },
            self,
        ))
    }

    /// ??????  The output of the command should list all files.
    ///
    /// This command filters all relevant directories with the following conditions using a
    /// template.
    ///
    /// {XVC_GUID}/[a-zA-Z][0-9]/[0-9A-Fa-f]{3}/[0-9A-Fa-f]{3}/[0-9A-Fa-f]{58}/0
    ///
    fn list(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
    ) -> Result<XvcStorageListEvent> {
        let address_map = self.address_map();
        let prepared_cmd = Self::replace_map_elements(&self.list_command, &address_map);
        let cmd_output = Exec::shell(prepared_cmd).capture()?.stdout_str();
        let xvc_guid = xvc_root.config().guid().unwrap();
        let re = Regex::new(&format!(
            "{xvc_guid}/{cp}/{d3}/{d3}/{d58}/0\\..*$",
            cp = r#"[a-zA-Z][0-9]"#,
            d3 = r#"[0-9A-Fa-f]{3}"#,
            d58 = r#"[0-9A-Fa-f]{58}"#
        ))
        .unwrap();

        let paths = cmd_output
            .lines()
            .filter_map(|l| if re.is_match(l) { Some(l) } else { None })
            .map(|l| XvcStoragePath::from(String::from(l)))
            .collect();

        Ok(XvcStorageListEvent {
            guid: self.guid.clone(),
            paths,
        })
    }

    fn send(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        _force: bool,
    ) -> Result<XvcStorageSendEvent> {
        let address_map = self.address_map();
        watch!(address_map);
        let prepared_cmd = Self::replace_map_elements(&self.upload_command, &address_map);
        watch!(prepared_cmd);
        let storage_paths = self.run_for_paths(output, xvc_root, &prepared_cmd, paths);
        watch!(storage_paths);

        Ok(XvcStorageSendEvent {
            guid: self.guid.clone(),
            paths: storage_paths,
        })
    }

    fn receive(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<XvcStorageReceiveEvent> {
        let address_map = self.address_map();
        watch!(address_map);
        let prepared_cmd = Self::replace_map_elements(&self.download_command, &address_map);
        watch!(prepared_cmd);
        let storage_paths = self.run_for_paths(output, xvc_root, &prepared_cmd, paths);
        watch!(storage_paths);

        Ok(XvcStorageReceiveEvent {
            guid: self.guid.clone(),
            paths: storage_paths,
        })
    }

    fn delete(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
    ) -> Result<XvcStorageDeleteEvent> {
        let address_map = self.address_map();
        let prepared_cmd = Self::replace_map_elements(&self.delete_command, &address_map);
        let storage_paths = self.run_for_paths(output, xvc_root, &prepared_cmd, paths);

        Ok(XvcStorageDeleteEvent {
            guid: self.guid.clone(),
            paths: storage_paths,
        })
    }
}
