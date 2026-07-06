mod common;
use common::*;

use assert_cmd::cargo::cargo_bin_cmd;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use xvc::error::Result;
use xvc_core::XvcVerbosity;

fn append(path: &Path, line: &str) -> Result<()> {
    let mut f = OpenOptions::new().append(true).open(path)?;
    writeln!(f, "{line}")?;
    Ok(())
}

#[test]
fn test_check_ignore() -> Result<()> {
    let xvc_root = run_in_temp_xvc_dir()?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["check-ignore"];
        c.extend(cmd);
        run_xvc(Some(&xvc_root), &c, XvcVerbosity::Default)
    };

    let xvcignore = xvc_root.join(&PathBuf::from(".xvcignore"));
    append(&xvcignore, "my-dir/my-file")?;

    // Paths given on the CLI are checked against .xvcignore rules
    let out = x(&["my-dir/my-file", "another-dir/another-file"])?;
    let ignore_line = out
        .lines()
        .find(|l| l.contains("my-dir/my-file"))
        .expect(&out);
    assert!(ignore_line.contains("[IGNORE]"), "{out}");
    let no_match_line = out
        .lines()
        .find(|l| l.contains("another-dir/another-file"))
        .expect(&out);
    assert!(no_match_line.contains("[NO MATCH]"), "{out}");

    // Whitelist patterns are reported separately
    append(&xvcignore, "!another-dir/*")?;
    let out = x(&["my-dir/my-file", "another-dir/another-file"])?;
    let whitelist_line = out
        .lines()
        .find(|l| l.contains("another-dir/another-file"))
        .expect(&out);
    assert!(whitelist_line.contains("[WHITELIST]"), "{out}");

    // Without CLI targets, paths are read from stdin
    let output = cargo_bin_cmd!("xvc")
        .args(["check-ignore"])
        .current_dir(xvc_root.absolute_path())
        .write_stdin("my-dir/my-file\n")
        .output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("[IGNORE]") && stdout.contains("my-dir/my-file"),
        "stdout: {stdout}"
    );

    // Rules can be read from another ignore file, e.g. .gitignore
    let gitignore = xvc_root.join(&PathBuf::from(".gitignore"));
    append(&gitignore, "ignored-by-git.txt")?;
    let out = x(&[
        "--ignore-filename",
        ".gitignore",
        "ignored-by-git.txt",
        "not-ignored.txt",
    ])?;
    let git_ignored_line = out
        .lines()
        .find(|l| l.contains("ignored-by-git.txt"))
        .expect(&out);
    assert!(git_ignored_line.contains("[IGNORE]"), "{out}");
    let not_ignored_line = out
        .lines()
        .find(|l| l.contains("not-ignored.txt"))
        .expect(&out);
    assert!(not_ignored_line.contains("[NO MATCH]"), "{out}");

    clean_up(&xvc_root)
}
