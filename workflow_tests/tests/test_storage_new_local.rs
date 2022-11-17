mod common;
use std::{fs, path::PathBuf};

use log::LevelFilter;

use common::run_in_temp_xvc_dir;
use xvc::{error::Result, watch};
use xvc_config::XvcVerbosity;
use xvc_core::XvcRoot;
use xvc_storage::storage::XVC_STORAGE_GUID_FILENAME;
use xvc_test_helper::{create_directory_tree, generate_filled_file};

fn create_directory_hierarchy() -> Result<XvcRoot> {
    let temp_dir: XvcRoot = run_in_temp_xvc_dir()?;
    watch!(temp_dir);
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

    let x = |cmd: &[&str]| {
        let mut c = vec!["xvc"];
        c.extend(cmd);
        watch!(cmd);
        xvc::test_dispatch(Some(&xvc_root), c, XvcVerbosity::Warn)
    };

    let out = x(&[
        "storage",
        "new",
        "local",
        "--name",
        "local-storage",
        "--path",
        &storage_dir.to_string_lossy().to_string(),
    ])?;

    assert!(storage_dir.join(XVC_STORAGE_GUID_FILENAME).exists());
    watch!(out);

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

    let push_result = x(&["file", "push", "--to", "local-storage", the_file])?;
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

    let fetch_result = x(&["file", "fetch", "--from", "local-storage"])?;

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

    let pull_result = x(&["file", "pull", "--from", "local-storage"])?;
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

    // When we reinit with the same storage path, it shouldn't update the GUID.
    // See https://github.com/iesahin/xvc/issues/123
    let current_guid = fs::read_to_string(&storage_dir.join(XVC_STORAGE_GUID_FILENAME))?;
    watch!(current_guid);
    // We'll use a separate process to run the following tests.
    // Entity counter cannot be loaded to the same process twice.
    let another_xvc_root = assert_fs::TempDir::new()?;
    watch!(another_xvc_root);
    let mut xvc = assert_cmd::cmd::Command::cargo_bin("xvc")?;
    watch!(xvc);
    xvc.current_dir(&another_xvc_root);
    xvc.arg("init").assert();
    xvc.args(&[
        "storage",
        "new",
        "local",
        "--name",
        "local-storage",
        "--path",
        &storage_dir.to_string_lossy().to_string(),
    ])
    .assert();
    let reread_guid = fs::read_to_string(&storage_dir.join(XVC_STORAGE_GUID_FILENAME))?;

    assert!(
        current_guid == reread_guid,
        "Guid Mismatch after reinit: {current_guid} - {reread_guid}"
    );

    Ok(())
}
