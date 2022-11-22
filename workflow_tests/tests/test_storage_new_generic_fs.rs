mod common;
use std::{env, fs, path::PathBuf};

use log::LevelFilter;

use common::run_in_temp_xvc_dir;
use rand;
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

#[test]
fn test_storage_new_generic_fs() -> Result<()> {
    common::test_logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let storage_dir_name = format!("{}/", common::random_dir_name("xvc-storage", None));
    let temp_directory = format!("{}/", env::temp_dir().to_string_lossy());

    let x = |cmd: &[&str]| {
        let mut c = vec!["xvc"];
        c.extend(cmd);
        watch!(cmd);
        xvc::test_dispatch(Some(&xvc_root), c, XvcVerbosity::Warn)
    };

    let out = x(&[
        "storage",
        "new",
        "generic",
        "--name",
        "generic-storage",
        "--url",
        &temp_directory,
        "--storage-dir",
        &storage_dir_name,
        "--init",
        "mkdir -p {URL}/{STORAGE_DIR} ; cp {LOCAL_GUID_FILE_PATH} {URL}/{STORAGE_GUID_FILE_PATH}",
        "--list",
        "ls -1 {URL}{STORAGE_DIR}",
        "--download",
        "mkdir -p {ABSOLUTE_CACHE_DIR} ; cp {FULL_STORAGE_PATH} {ABSOLUTE_CACHE_PATH}",
        "--upload",
        "mkdir -p {FULL_STORAGE_DIR} ; cp {ABSOLUTE_CACHE_PATH} {FULL_STORAGE_PATH}",
        "--delete",
        "rm {FULL_STORAGE_PATH} ; rmdir {FULL_STORAGE_DIR}",
        "--processes",
        "4",
    ])?;

    let storage_dir = PathBuf::from(temp_directory).join(storage_dir_name);

    watch!(storage_dir);
    watch!(out);

    assert!(storage_dir.exists());

    assert!(storage_dir.join(XVC_STORAGE_GUID_FILENAME).exists());

    let the_file = "file-0000.bin";

    let file_track_result = x(&["file", "track", the_file])?;
    watch!(file_track_result);

    let n_storage_files_before = jwalk::WalkDir::new(&storage_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();

    let push_result = x(&["file", "send", "--to", "generic-storage", the_file])?;
    watch!(push_result);

    // The file should be in:
    // - storage_dir/REPO_ID/b3/ABCD...123/0.bin

    let n_storage_files_after = jwalk::WalkDir::new(&storage_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();

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

    let fetch_result = x(&[
        "file",
        "bring",
        "--no-checkout",
        "--from",
        "generic-storage",
    ])?;

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
