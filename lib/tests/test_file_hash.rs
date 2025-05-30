mod common;
use common::*;
use std::path::PathBuf;

use regex::Regex;
use xvc_core::XvcVerbosity;

use xvc::error::Result;
use xvc_core::XvcRoot;
use xvc_test_helper::{create_directory_tree, generate_filled_file};

fn create_directory_hierarchy() -> Result<XvcRoot> {
    let temp_dir: XvcRoot = run_in_temp_xvc_dir()?;
    // for checking the content hash
    generate_filled_file(&temp_dir.join(&PathBuf::from("file-0000.bin")), 10000, 100);
    create_directory_tree(&temp_dir, 1, 100, 1000, None)?;
    // root/dir1 may have another tree

    Ok(temp_dir)
}

#[test]
fn test_file_hash() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let x = |cmd: &[&str]| common::run_xvc(Some(&xvc_root), cmd, XvcVerbosity::Trace);

    let re_match = |output, regex| {
        let regex = Regex::new(regex).unwrap();
        regex.is_match(output)
    };

    let dir_hash = x(&["file", "hash", "dir-0001/"])?;
    assert!({ dir_hash.lines().count() == 100 });
    let images_hash = x(&["file", "hash", "file-0000.bin"])?;
    assert!(
        re_match(
            &images_hash,
            "^a572622134fcb28679d2de66d225cc2a41c2594baa909781c0726eb7702baeb1\tfile-0000.bin.*"
        ),
        "images_hash = {}",
        &images_hash
    );

    let text_hash = x(&["file", "hash", "--text-or-binary", "text", "file-0000.bin"])?;
    assert!(re_match(
        &text_hash,
        "^a572622134fcb28679d2de66d225cc2a41c2594baa909781c0726eb7702baeb1\tfile-0000.bin.*"
    ));

    let binary_hash = x(&[
        "file",
        "hash",
        "--text-or-binary",
        "binary",
        "dir-0001/file-0010.bin",
    ])?;
    let text_hash = x(&[
        "file",
        "hash",
        "--text-or-binary",
        "text",
        "dir-0001/file-0010.bin",
    ])?;

    assert!(
        binary_hash != text_hash,
        "Hashes {} and {} should not be equal",
        binary_hash,
        text_hash
    );

    clean_up(&xvc_root)
}
