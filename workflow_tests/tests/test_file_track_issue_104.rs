mod common;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, path::PathBuf};

use crate::common::{run_in_example_xvc, run_in_temp_xvc_dir, run_xvc};
use assert_cmd::Command;
use jwalk;
use regex::Regex;
use subprocess::Exec;
use xvc::error::{Error, Result};
use xvc::watch;
use xvc_config::XvcVerbosity;
use xvc_core::XvcRoot;
use xvc_test_helper::create_directory_tree;

fn create_directory_hierarchy() -> Result<XvcRoot> {
    let temp_dir: XvcRoot = run_in_temp_xvc_dir()?;
    // for checking the content hash
    create_directory_tree(&temp_dir, 2, 10)?;
    Ok(temp_dir)
}

fn sh(cmd: String) -> String {
    watch!(cmd);
    Exec::shell(cmd).capture().unwrap().stdout_str()
}

/// When a directory is added to projects, its child files are also ignored.
///
#[test]
fn test_file_track_issue_104() -> Result<()> {
    // setup::logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;

    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["file"];
        c.extend(cmd);
        run_xvc(Some(&xvc_root), &c, XvcVerbosity::Trace)
    };

    let dir_1 = "dir-0001";
    let track_dir_1 = x(&["track", dir_1, "--no-parallel"])?;
    watch!(track_dir_1);
    // Create dir-0001 and dir-0002 with files file-0001..0010.bin inside them.

    let root_gitignore = fs::read_to_string(xvc_root.join(Path::new(".gitignore")))?;

    assert!(!xvc_root.join(Path::new("dir-0001/.gitignore")).exists());

    assert!(
        root_gitignore
            .lines()
            .filter(|l| l.to_string() == "/dir-0001/".to_string())
            .count()
            == 1,
        "{}",
        root_gitignore
    );

    Ok(())
}
