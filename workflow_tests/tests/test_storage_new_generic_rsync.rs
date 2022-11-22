mod common;
use std::{env, fs, path::PathBuf};

use log::LevelFilter;

use common::run_in_temp_xvc_dir;
use rand;
use subprocess::Exec;
use xvc::{error::Result, watch};
use xvc_config::XvcVerbosity;
use xvc_core::XvcRoot;
use xvc_storage::storage::XVC_STORAGE_GUID_FILENAME;
use xvc_test_helper::{create_directory_tree, generate_filled_file};

fn create_directory_hierarchy() -> Result<XvcRoot> {
    let temp_dir: XvcRoot = run_in_temp_xvc_dir()?;
    // for checking the content hash
    generate_filled_file(&temp_dir.join(&PathBuf::from("file-0000.bin")), 10000, 100);
    create_directory_tree(&temp_dir, 10, 10)?;
    // root/dir1 may have another tree
    let level_1 = &temp_dir.join(&PathBuf::from("dir-0001"));
    create_directory_tree(&level_1, 10, 10)?;

    Ok(temp_dir)
}

fn sh(cmd: String) -> String {
    watch!(cmd);
    Exec::shell(cmd).capture().unwrap().stdout_str()
}

#[test]
#[cfg_attr(not(feature = "test-generic-rsync"), ignore)]
fn test_storage_new_generic_rsync() -> Result<()> {
    common::test_logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let storage_dir_name = format!(
        "/tmp/{}/",
        common::random_dir_name("xvc-storage", Some(111))
    );
    let test_host = "xvc-test@one.emresult.com";
    let url = format!("{test_host}");
    let local_test_dir = env::temp_dir().join(common::random_dir_name("xvc-storage-copy", None));

    let x = |cmd: &[&str]| {
        let mut c = vec!["xvc"];
        c.extend(cmd);
        watch!(cmd);
        xvc::test_dispatch(Some(&xvc_root), c, XvcVerbosity::Warn)
    };

    let delete_dir = sh(format!(
        "ssh {url} 'test -e {storage_dir_name} && rm -rf {storage_dir_name}'"
    ));

    let out = x(&[
        "storage",
        "new",
        "generic",
        "--name",
        "generic-storage",
        "--url",
        &url,
        "--storage-dir",
        &storage_dir_name,
        "--init",
        "ssh {URL} 'mkdir -p {STORAGE_DIR}' ; rsync -av {LOCAL_GUID_FILE_PATH} {URL}:{STORAGE_GUID_FILE_PATH}",
        "--list",
        "ssh {URL} 'ls -1 {STORAGE_DIR}'",
        "--upload",
        "ssh {URL} 'mkdir -p {STORAGE_DIR}{XVC_GUID}/{RELATIVE_CACHE_DIR}' ; rsync -av {ABSOLUTE_CACHE_PATH} {URL}:{STORAGE_DIR}{XVC_GUID}/{RELATIVE_CACHE_PATH}",
        "--download",
        "mkdir -p {ABSOLUTE_CACHE_DIR} ; rsync -av {URL}:{STORAGE_DIR}{XVC_GUID}/{RELATIVE_CACHE_PATH} {ABSOLUTE_CACHE_PATH}",
        "--delete",
        "ssh {URL} 'rm {STORAGE_DIR}{RELATIVE_CACHE_PATH}'",
        "--processes",
        "4",
    ])?;

    watch!(out);

    let storage_list = sh(format!(
        "ssh {url} 'ls -l {storage_dir_name}{XVC_STORAGE_GUID_FILENAME}'"
    ));

    watch!(storage_list);
    assert!(storage_list.len() > 0);

    let the_file = "file-0000.bin";

    let file_track_result = x(&["file", "track", the_file])?;
    watch!(file_track_result);

    let n_storage_files_before = jwalk::WalkDir::new(&local_test_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();
    let push_result = x(&["file", "send", "--to", "generic-storage", the_file])?;
    watch!(push_result);

    let file_list = sh(format!("ssh {url} 'ls -1R {storage_dir_name} | grep bin'"));
    watch!(file_list);

    // The file should be in:
    // - storage_dir/REPO_ID/b3/ABCD...123/0.bin

    let n_storage_files_after = file_list.lines().count();

    assert!(
        n_storage_files_before + 1 == n_storage_files_after,
        "{} - {}",
        n_storage_files_before,
        n_storage_files_after
    );

    // remove all cache
    //
    let cache_dir = xvc_root.xvc_dir().join("b3");
    fs::remove_dir_all(&cache_dir)?;

    let fetch_result = x(&["file", "fetch", "--from", "generic-storage"])?;

    watch!(fetch_result);

    let n_local_files_after_fetch = jwalk::WalkDir::new(&cache_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();

    assert!(n_storage_files_after == n_local_files_after_fetch);

    let cache_dir = xvc_root.xvc_dir().join("b3");
    fs::remove_dir_all(&cache_dir)?;
    fs::remove_file(the_file)?;

    let pull_result = x(&["file", "pull", "--from", "generic-storage"])?;
    watch!(pull_result);

    let n_local_files_after_pull = jwalk::WalkDir::new(&cache_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();

    assert!(n_storage_files_after == n_local_files_after_pull);
    assert!(PathBuf::from(the_file).exists());

    Ok(())
}
