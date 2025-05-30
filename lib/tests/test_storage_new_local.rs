mod common;
use std::{fs, path::PathBuf};

use log::LevelFilter;

use common::*;
use xvc::error::Result;
use xvc_core::XvcVerbosity;
use xvc_core::XvcRoot;
use xvc_storage::storage::XVC_STORAGE_GUID_FILENAME;
use xvc_test_helper::generate_filled_file;

fn create_directory_hierarchy() -> Result<XvcRoot> {
    let temp_dir: XvcRoot = run_in_temp_xvc_dir()?;
    // for checking the content hash
    generate_filled_file(&temp_dir.join(&PathBuf::from("file-0000.bin")), 10000, 100);
    // create_directory_tree(&temp_dir, 10, 10)?;
    // // root/dir1 may have another tree
    // let level_1 = &temp_dir.join(&PathBuf::from("dir-0001"));
    // create_directory_tree(&level_1, 10, 10)?;

    Ok(temp_dir)
}

#[test]
fn test_storage_new_local() -> Result<()> {
    common::test_logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let storage_dir = common::random_temp_dir(Some("xvc-storage"));

    let x = |cmd: &[&str]| -> Result<String> {
        common::run_xvc(Some(&xvc_root), cmd, XvcVerbosity::Trace)
    };

    x(&[
        "storage",
        "new",
        "local",
        "--name",
        "local-storage",
        "--path",
        storage_dir.to_string_lossy().as_ref(),
    ])?;

    assert!(storage_dir.join(XVC_STORAGE_GUID_FILENAME).exists());

    let the_file = "file-0000.bin";

    x(&["file", "track", the_file])?;

    let n_storage_files_before = jwalk::WalkDir::new(&storage_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();

    x(&["file", "send", "--to", "local-storage", the_file])?;

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
    sh(&format!(
        "rm -rf '{}'",
        &cache_dir.to_string_lossy().to_string()
    ))?;

    x(&["file", "bring", "--no-recheck", "--from", "local-storage"])?;

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
    sh(&format!(
        "rm -rf '{}'",
        &cache_dir.to_string_lossy().to_string()
    ))?;
    fs::remove_file(the_file)?;

    x(&["file", "bring", "--from", "local-storage"])?;

    let n_local_files_after_pull = jwalk::WalkDir::new(&cache_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();
    assert!(n_storage_files_after == n_local_files_after_pull);
    sh("tree")?;
    assert!(PathBuf::from(the_file).exists());

    // When we reinit with the same storage path, it shouldn't update the GUID.
    // See https://github.com/iesahin/xvc/issues/123
    let current_guid = fs::read_to_string(storage_dir.join(XVC_STORAGE_GUID_FILENAME))?;
    // We'll use a separate process to run the following tests.
    // Entity counter cannot be loaded to the same process twice.
    let another_xvc_root = assert_fs::TempDir::new()?;
    let mut xvc = assert_cmd::cmd::Command::cargo_bin("xvc")?;
    xvc.current_dir(&another_xvc_root);
    xvc.arg("init").assert();
    xvc.args([
        "storage",
        "new",
        "local",
        "--name",
        "local-storage",
        "--path",
        storage_dir.to_string_lossy().as_ref(),
    ])
    .assert();
    let reread_guid = fs::read_to_string(storage_dir.join(XVC_STORAGE_GUID_FILENAME))?;

    assert!(
        current_guid == reread_guid,
        "Guid Mismatch after reinit: {current_guid} - {reread_guid}"
    );

    clean_up(&xvc_root)
}
