mod common;
use common::*;

use std::fs;
use std::path::{Path, PathBuf};

use xvc::error::Result;
use xvc_core::XvcRoot;
use xvc_core::XvcVerbosity;

const CONTENT: &str = "Oh, data, my, data\n";

fn setup() -> Result<XvcRoot> {
    let xvc_root = run_in_temp_xvc_dir()?;
    fs::write(xvc_root.join(&PathBuf::from("data.txt")), CONTENT)?;
    Ok(xvc_root)
}

fn is_symlink(path: &Path) -> bool {
    path.symlink_metadata()
        .map(|md| md.file_type().is_symlink())
        .unwrap_or(false)
}

#[test]
fn test_file_untrack() -> Result<()> {
    let xvc_root = setup()?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["file"];
        c.extend(cmd);
        run_xvc(Some(&xvc_root), &c, XvcVerbosity::Default)
    };
    let p = |s: &str| xvc_root.join(&PathBuf::from(s));

    // Untracking deletes the cached copy but leaves the workspace file
    x(&["track", "data.txt"])?;
    assert_eq!(cache_paths(&xvc_root).len(), 1);
    let untrack_out = x(&["untrack", "data.txt"])?;
    assert!(untrack_out.contains("[DELETE]"), "{untrack_out}");
    assert_eq!(cache_paths(&xvc_root).len(), 0);
    assert!(p("data.txt").exists());
    assert_eq!(fs::read_to_string(p("data.txt"))?, CONTENT);

    // Untracking a symlink-rechecked file restores it as a regular file
    x(&["track", "data.txt", "--as", "symlink"])?;
    assert!(is_symlink(&p("data.txt")));
    x(&["untrack", "data.txt"])?;
    assert!(!is_symlink(&p("data.txt")), "data.txt should be a copy now");
    assert_eq!(fs::read_to_string(p("data.txt"))?, CONTENT);
    assert_eq!(cache_paths(&xvc_root).len(), 0);

    // --restore-versions copies all cached versions before deleting them
    x(&["track", "data.txt"])?;
    fs::write(p("data.txt"), "second version\n")?;
    x(&["carry-in", "data.txt"])?;
    assert_eq!(cache_paths(&xvc_root).len(), 2);
    x(&[
        "untrack",
        "data.txt",
        "--restore-versions",
        "data-versions/",
    ])?;
    assert_eq!(cache_paths(&xvc_root).len(), 0);
    let restored: Vec<_> = fs::read_dir(p("data-versions"))?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .collect();
    assert_eq!(
        restored.len(),
        2,
        "both versions should be restored: {restored:?}"
    );

    // Deduplicated cache files are only deleted when the last user is untracked
    fs::write(p("data.txt"), CONTENT)?;
    x(&["track", "data.txt"])?;
    x(&["copy", "data.txt", "data2.txt", "--as", "symlink"])?;
    assert_eq!(cache_paths(&xvc_root).len(), 1);
    let untrack_out = x(&["untrack", "data.txt"])?;
    assert!(
        untrack_out.contains("also used by"),
        "should report the other path using the cache file: {untrack_out}"
    );
    assert_eq!(cache_paths(&xvc_root).len(), 1);
    x(&["untrack", "data2.txt"])?;
    assert_eq!(cache_paths(&xvc_root).len(), 0);

    clean_up(&xvc_root)
}
