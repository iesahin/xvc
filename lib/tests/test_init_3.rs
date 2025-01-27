mod common;

use std::fs;

use xvc::init::InitCLI;

use xvc::error::Result;

use common::*;

// This tests the preexisting .gitignore rules bug
// https://github.com/iesahin/xvc/issues/119

#[test]
fn test_init_with_preexisting_git() -> Result<()> {
    let the_dir = common::run_in_temp_git_dir();
    let previously_ignored = "dir-0001";
    fs::write(
        the_dir.join(".gitignore"),
        format!("{previously_ignored}\n"),
    )?;
    let xvc_root = xvc::init::run(
        None,
        InitCLI {
            path: None,
            no_git: false,
            force: false,
        },
    )?;

    let git_status = sh(&format!(
        "git -C {} status -s | grep {previously_ignored}",
        the_dir.to_string_lossy()
    ))?
    .stdout_str();

    assert!(git_status.trim().is_empty(), "{}", git_status);

    clean_up(&xvc_root)
}
