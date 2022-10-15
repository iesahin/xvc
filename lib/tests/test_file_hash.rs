mod common;
use std::path::PathBuf;

use crate::common::run_in_temp_xvc_dir;
use regex::Regex;
use xvc_config::XvcVerbosity;

use xvc::{error::Result, watch};
use xvc_core::XvcRoot;
use xvc_test_helper::{create_directory_tree, generate_filled_file};

fn create_directory_hierarchy() -> Result<XvcRoot> {
    let temp_dir: XvcRoot = run_in_temp_xvc_dir()?;
    // for checking the content hash
    generate_filled_file(&temp_dir.join(&PathBuf::from("file-0000.bin")), 10000, 100);
    create_directory_tree(&temp_dir, 1, 100)?;
    // root/dir1 may have another tree

    Ok(temp_dir)
}

#[test]
fn test_file_hash() -> Result<()> {
    // setup::logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let x = |cmd: &[&str]| {
        let mut c = vec!["xvc", "file"];
        c.extend(cmd);
        xvc::test_dispatch(Some(&xvc_root), c, XvcVerbosity::Warn)
    };

    let re_match = |output, regex| {
        let regex = Regex::new(regex).unwrap();
        assert!(regex.is_match(output), "output: {}", output);
    };

    let dir_hash = x(&["hash", "dir-0001/"])?;
    re_match(&dir_hash, ".*");
    let line_count = dir_hash.lines().count();
    watch!(line_count);
    assert!(line_count == 100, "{}", line_count);
    let images_hash = x(&["hash", "file-0000.bin"])?;
    re_match(
        &images_hash,
        "^a572622134fcb28679d2de66d225cc2a41c2594baa909781c0726eb7702baeb1\tfile-0000.bin.*",
    );
    let images_as_text_hash = x(&["hash", "--text-file", "file-0000.bin"])?;
    assert!(images_hash == images_as_text_hash);

    let binary_hash = x(&["hash", "dir-0001/file-0010.bin"])?;
    let text_hash = x(&["hash", "--text-file", "dir-0001/file-0010.bin"])?;

    assert!(binary_hash != text_hash);

    Ok(())
}
