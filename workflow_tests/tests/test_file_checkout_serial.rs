mod common;

use std::{fs, path::PathBuf};

use crate::common::run_in_temp_xvc_dir;
use jwalk;
use xvc::error::Result;
use xvc::test_dispatch;
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
fn test_file_checkout_serial() -> Result<()> {
    // setup::logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let x = |cmd: &[&str], assert_fn| {
        common::assert_xvc(Some(&xvc_root), cmd, XvcVerbosity::Trace, assert_fn)
    };

    let file_to_add = "file-0000.bin";
    x(&["file", "track", file_to_add], |_| true)?;

    fs::remove_file(file_to_add)?;

    x(&["file", "checkout", "--no-parallel", file_to_add], |_| {
        true
    })?;

    assert!(PathBuf::from(file_to_add).exists());

    x(
        &[
            "file",
            "checkout",
            "--no-parallel",
            "--force",
            "--cache-type",
            "symlink",
            file_to_add,
        ],
        |_| true,
    )?;

    assert!(PathBuf::from(file_to_add).is_symlink());

    x(
        &[
            "file",
            "checkout",
            "--no-parallel",
            "--cache-type",
            "hardlink",
            file_to_add,
        ],
        |_| true,
    )?;

    // No --force, it shouldn't overwrite

    assert!(PathBuf::from(file_to_add).is_symlink());

    let dir_to_add = "dir-0001/";
    x(&["file", "track", dir_to_add], |_| true)?;

    let n_files_before = jwalk::WalkDir::new(dir_to_add).into_iter().count();

    fs::remove_dir_all(dir_to_add)?;

    x(&["file", "checkout", "--no-parallel", dir_to_add], |_| true)?;

    assert!(PathBuf::from(dir_to_add).exists());

    let n_files_after = jwalk::WalkDir::new(dir_to_add).into_iter().count();

    assert!(n_files_after == n_files_before);

    // xvc file checkout without targets checks out all

    fs::remove_file(file_to_add)?;
    x(&["file", "checkout", "--no-parallel"], |_| true)?;
    assert!(PathBuf::from(file_to_add).exists());

    // xvc file checkout accepts globs as targets
    fs::remove_file(file_to_add)?;
    x(&["file", "checkout", "--no-parallel", "f*"], |_| true)?;
    assert!(PathBuf::from(file_to_add).exists());

    Ok(())
}
