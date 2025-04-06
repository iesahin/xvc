mod common;
use std::{env, fs, path::PathBuf};

use log::LevelFilter;

use common::*;
use subprocess::Exec;
use xvc::{error::Result, watch};
use xvc_config::XvcVerbosity;
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

fn sh(cmd: String) -> String {
    Exec::shell(cmd).capture().unwrap().stdout_str()
}

#[test]
fn test_storage_new_rclone() -> Result<()> {
    common::test_logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let remote_name = "xvc-generic-rclone-test";
    let storage_dir_name = common::random_dir_name("xvc-storage", None);
    let storage_dir = env::temp_dir().join(&storage_dir_name);

    let x = |cmd: &[&str]| -> Result<String> {
        common::run_xvc(Some(&xvc_root), cmd, XvcVerbosity::Trace)
    };

    x(&[
        "storage",
        "new",
        "rclone",
        "--name",
        "rclone-storage",
        "--remote-name",
        remote_name,
    ])?;

    // Drop rclone remote if it exists
    sh(format!("rclone config delete {}", remote_name));

    // Create rclone remote
    sh(format!(
        "rclone config create {} alias remote={}",
        remote_name,
        storage_dir.to_string_lossy()
    ));

    // Create storage directory
    sh(format!("mkdir -p {}", storage_dir.to_string_lossy()));

    let storage_name = "generic-rclone-storage";

    let storage_list = sh(format!(
        "rclone ls {remote_name}:{storage_dir_name}/{XVC_STORAGE_GUID_FILENAME}",
    ));

    assert!(!storage_list.is_empty());

    let the_file = "file-0000.bin";

    x(&["file", "track", the_file])?;

    let n_storage_files_before = jwalk::WalkDir::new(storage_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();

    let push_result = x(&["file", "send", "--to", storage_name, the_file])?;

    watch!(push_result);

    let file_list = sh(format!("rclone ls {remote_name}: | grep bin"));

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
    sh(format!("rm -rf {}", cache_dir.to_string_lossy()));

    x(&["file", "bring", "--no-recheck", "--from", storage_name])?;

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
    sh(format!("rm -rf {}", cache_dir.to_string_lossy()));
    fs::remove_file(the_file)?;

    x(&["file", "bring", "--from", storage_name])?;

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

    clean_up(&xvc_root)
}
