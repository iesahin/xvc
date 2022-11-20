mod common;
use std::env;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, path::PathBuf};

use crate::common::{run_in_example_xvc, run_in_temp_xvc_dir};
use assert_cmd::Command;
use jwalk;
use regex::Regex;
use subprocess::Exec;
use xvc::error::{Error, Result};
use xvc::{test_dispatch, watch};
use xvc_config::XvcVerbosity;
use xvc_core::XvcRoot;
use xvc_test_helper::create_directory_tree;

fn sh(cmd: String) -> String {
    watch!(cmd);
    Exec::shell(cmd).capture().unwrap().stdout_str()
}

/// When a directory is added to projects, its child files are also ignored.
///
#[test]
fn test_file_track_issue_104() -> Result<()> {
    // setup::logging(LevelFilter::Trace);
    let xvc_dir = assert_fs::TempDir::new()?;
    let git_exec = which::which("git")?;
    let mut git = assert_cmd::cmd::Command::from_std(std::process::Command::new(git_exec));
    git.current_dir(&xvc_dir);
    git.arg("init").assert();
    let mut xvc = assert_cmd::cmd::Command::cargo_bin("xvc")?;
    xvc.current_dir(&xvc_dir);
    xvc.arg("init").assert();

    // Create dir-0001 and dir-0002 with files file-0001..0010.bin inside them.
    create_directory_tree(&xvc_dir, 2, 10)?;
    assert!(xvc_dir.join("dir-0001").exists());

    xvc.arg("-vvvv")
        .arg("file")
        .arg("track")
        .arg("dir-0001/")
        .assert();

    let root_gitignore = fs::read_to_string(xvc_dir.join(".gitignore"))?;

    assert!(!xvc_dir.join("dir-0001").join(".gitignore").exists());

    assert!(
        root_gitignore
            .lines()
            .filter(|l| l.to_string() == "dir-0001/".to_string())
            .count()
            == 1,
        "{}",
        root_gitignore
    );

    Ok(())
}
