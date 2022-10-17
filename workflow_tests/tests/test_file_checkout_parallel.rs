mod common;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, path::PathBuf};

use crate::common::run_in_temp_xvc_dir;
use jwalk;
use xvc::error::{Error, Result};
use xvc::{test_dispatch, watch};
use xvc_config::XvcVerbosity;
use xvc_core::XvcRoot;
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
fn test_file_checkout_parallel() -> Result<()> {
    let xvc_root = create_directory_hierarchy()?;
    watch!(xvc_root);
    let x = |cmd: &[&str]| {
        let mut c = vec!["xvc", "file"];
        c.extend(cmd);
        watch!(c);
        test_dispatch(Some(&xvc_root), c, XvcVerbosity::Trace)
    };

    let file_to_add = "file-0000.bin";
    x(&["track", file_to_add])?;

    fs::remove_file(file_to_add)?;

    x(&["checkout", file_to_add])?;

    assert!(PathBuf::from(file_to_add).exists());

    x(&[
        "checkout",
        "--force",
        "--cache-type",
        "symlink",
        file_to_add,
    ])?;

    assert!(PathBuf::from(file_to_add).is_symlink());

    x(&["checkout", "--cache-type", "hardlink", file_to_add])?;

    // No --force, it shouldn't overwrite

    assert!(PathBuf::from(file_to_add).is_symlink());

    let dir_to_add = "dir-0001/";
    x(&["track", dir_to_add])?;

    let n_files_before = jwalk::WalkDir::new(dir_to_add).into_iter().count();

    fs::remove_dir_all(dir_to_add)?;

    x(&["checkout", dir_to_add])?;

    assert!(PathBuf::from(dir_to_add).exists());

    let n_files_after = jwalk::WalkDir::new(dir_to_add).into_iter().count();

    assert!(n_files_after == n_files_before);

    // xvc file checkout without targets checks out all

    fs::remove_file(file_to_add)?;
    x(&["checkout"])?;
    assert!(PathBuf::from(file_to_add).exists());
    // xvc file checkout accepts globs as targets
    fs::remove_file(file_to_add)?;
    x(&["checkout", "f*"])?;
    assert!(PathBuf::from(file_to_add).exists());

    Ok(())
}
