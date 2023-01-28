mod common;

use std::{fs, path::PathBuf};

use crate::common::run_in_temp_xvc_dir;
use jwalk;
use log::LevelFilter;
use xvc::error::Result;
use xvc_config::XvcVerbosity;
use xvc_core::XvcRoot;
use xvc_test_helper::{create_directory_tree, generate_filled_file};
use xvc_tests::watch;

fn create_directory_hierarchy() -> Result<XvcRoot> {
    let temp_dir: XvcRoot = run_in_temp_xvc_dir()?;
    // for checking the content hash

    let child_path = temp_dir.join(&PathBuf::from("file-0000.bin"));
    generate_filled_file(&child_path, 10000, 100);
    assert!(child_path.exists());
    create_directory_tree(&temp_dir, 10, 10, Some(23))?;
    let level_1 = &temp_dir.join(&PathBuf::from("dir-0001"));
    create_directory_tree(&level_1, 10, 10, Some(23))?;

    Ok(temp_dir)
}

#[test]
fn test_file_recheck_serial() -> Result<()> {
    xvc_test_helper::test_logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let x = |cmd: &[&str]| common::run_xvc(Some(&xvc_root), cmd, XvcVerbosity::Trace);

    let file_to_add = "file-0000.bin";
    let path_to_add = PathBuf::from(file_to_add);
    watch!(x(&["file", "track", file_to_add])?);
    assert!(path_to_add.exists());
    fs::remove_file(file_to_add)?;
    assert!(!path_to_add.exists());
    watch!(x(&["file", "recheck", "--no-parallel", file_to_add])?);

    assert!(PathBuf::from(file_to_add).exists());

    x(&[
        "file",
        "recheck",
        "--no-parallel",
        "--force",
        "--as",
        "symlink",
        file_to_add,
    ])?;

    assert!(PathBuf::from(file_to_add).is_symlink());

    x(&[
        "file",
        "recheck",
        "--no-parallel",
        "--as",
        "hardlink",
        file_to_add,
    ])?;

    assert!(PathBuf::from(file_to_add).is_file());

    let dir_to_add = "dir-0001/";
    x(&["file", "track", "--no-parallel", dir_to_add])?;

    let n_files_before = jwalk::WalkDir::new(dir_to_add).into_iter().count();

    fs::remove_dir_all(dir_to_add)?;

    x(&["file", "recheck", "--no-parallel", dir_to_add])?;

    assert!(PathBuf::from(dir_to_add).exists());

    let n_files_after = jwalk::WalkDir::new(dir_to_add).into_iter().count();

    assert!(n_files_after == n_files_before);

    // xvc file recheck without targets checks out all

    fs::remove_file(file_to_add)?;
    x(&["file", "recheck", "--no-parallel"])?;
    assert!(PathBuf::from(file_to_add).exists());

    // xvc file recheck accepts globs as targets
    fs::remove_file(file_to_add)?;
    x(&["file", "recheck", "--no-parallel", "f*"])?;
    assert!(PathBuf::from(file_to_add).exists());

    Ok(())
}
