mod common;
use common::*;
use std::fs;
use std::path::Path;

use xvc::error::Result;
use xvc::watch;
use xvc_config::XvcVerbosity;
use xvc_core::XvcRoot;
use xvc_test_helper::create_directory_tree;

fn create_directory_hierarchy() -> Result<XvcRoot> {
    let temp_dir: XvcRoot = run_in_temp_xvc_dir()?;
    // for checking the content hash
    create_directory_tree(&temp_dir, 2, 10, Some(23))?;
    Ok(temp_dir)
}

/// When a directory is added to projects, its child files are also ignored.
///
#[test]
fn test_file_track_issue_104() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;

    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["file"];
        c.extend(cmd);
        run_xvc(Some(&xvc_root), &c, XvcVerbosity::Trace)
    };

    let dir_1 = "dir-0001/";
    let track_dir_1 = x(&["track", dir_1, "--no-parallel"])?;
    watch!(track_dir_1);
    // Create dir-0001 and dir-0002 with files file-0001..0010.bin inside them.

    let root_gitignore = fs::read_to_string(xvc_root.join(Path::new(".gitignore")))?;
    watch!(root_gitignore);
    let dir_ignore = xvc_root.join(Path::new("dir-0001/.gitignore"));
    assert!(!dir_ignore.exists());

    assert!(
        root_gitignore
            .lines()
            .filter(|l| l.to_string() == "/dir-0001/".to_string())
            .count()
            == 1,
        "{}",
        root_gitignore
    );

    clean_up(&xvc_root)
}
