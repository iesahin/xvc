mod common;
use common::*;
use std::{fs, path::PathBuf};

use jwalk;
use xvc::error::Result;
use xvc::watch;
use xvc_config::XvcVerbosity;
use xvc_core::XvcRoot;
use xvc_test_helper::{create_directory_tree, generate_filled_file};

fn create_directory_hierarchy() -> Result<XvcRoot> {
    let temp_dir: XvcRoot = run_in_temp_xvc_dir()?;
    // for checking the content hash
    generate_filled_file(&temp_dir.join(&PathBuf::from("file-0000.bin")), 10000, 100);
    create_directory_tree(&temp_dir, 10, 10, Some(23))?;
    // root/dir1 may have another tree
    let level_1 = &temp_dir.join(&PathBuf::from("dir-0001"));
    create_directory_tree(&level_1, 10, 10, Some(23))?;

    Ok(temp_dir)
}

#[test]
fn test_file_recheck_parallel() -> Result<()> {
    let xvc_root = create_directory_hierarchy()?;
    watch!(xvc_root);
    let x = |cmd: &[&str]| common::run_xvc(Some(&xvc_root), cmd, XvcVerbosity::Trace);

    let file_to_add = "file-0000.bin";
    x(&["file", "track", file_to_add])?;

    fs::remove_file(file_to_add)?;

    x(&["file", "recheck", file_to_add])?;

    assert!(PathBuf::from(file_to_add).exists());

    x(&[
        "file",
        "recheck",
        "--force",
        "--cache-type",
        "symlink",
        file_to_add,
    ])?;

    assert!(PathBuf::from(file_to_add).is_symlink());

    x(&["file", "recheck", "--cache-type", "hardlink", file_to_add])?;

    // No --force, it shouldn't overwrite

    assert!(PathBuf::from(file_to_add).is_symlink());

    let dir_to_add = "dir-0001/";
    x(&["file", "track", dir_to_add])?;

    let n_files_before = jwalk::WalkDir::new(dir_to_add).into_iter().count();

    fs::remove_dir_all(dir_to_add)?;

    x(&["file", "recheck", dir_to_add])?;

    assert!(PathBuf::from(dir_to_add).exists());

    let n_files_after = jwalk::WalkDir::new(dir_to_add).into_iter().count();

    assert!(n_files_after == n_files_before);

    // xvc file recheck without targets checks out all

    fs::remove_file(file_to_add)?;
    x(&["file", "recheck"])?;
    assert!(PathBuf::from(file_to_add).exists());
    // xvc file recheck accepts globs as targets
    fs::remove_file(file_to_add)?;
    x(&["file", "recheck", "f*"])?;
    assert!(PathBuf::from(file_to_add).exists());

    clean_up(&xvc_root)
}
