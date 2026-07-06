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
fn test_file_move() -> Result<()> {
    let xvc_root = setup()?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["file"];
        c.extend(cmd);
        run_xvc(Some(&xvc_root), &c, XvcVerbosity::Default)
    };
    let p = |s: &str| xvc_root.join(&PathBuf::from(s));

    x(&["track", "data.txt"])?;
    assert_eq!(cache_paths(&xvc_root).len(), 1);

    // Moving renames the workspace file and keeps the cache intact
    x(&["move", "data.txt", "data2.txt"])?;
    assert!(!p("data.txt").exists(), "data.txt should be moved away");
    assert!(p("data2.txt").exists());
    assert_eq!(fs::read_to_string(p("data2.txt"))?, CONTENT);
    assert_eq!(cache_paths(&xvc_root).len(), 1);

    // Move can change the recheck method
    x(&["move", "data2.txt", "data3.txt", "--as", "symlink"])?;
    assert!(!p("data2.txt").exists());
    assert!(is_symlink(&p("data3.txt")), "data3.txt should be a symlink");

    // A file missing from the workspace can still be moved; it's rechecked from the cache
    fs::remove_file(p("data3.txt"))?;
    x(&["move", "data3.txt", "data4.txt"])?;
    assert!(p("data4.txt").exists() || is_symlink(&p("data4.txt")));

    // Glob source with a directory destination
    x(&["copy", "data4.txt", "data5.txt"])?;
    x(&["move", "data*", "another-set/"])?;
    assert!(p("another-set/data4.txt").exists());
    assert!(p("another-set/data5.txt").exists());
    assert!(!p("data4.txt").exists());
    assert!(!p("data5.txt").exists());

    // --no-recheck records the move without materializing the destination
    x(&["move", "another-set/data5.txt", "data6.txt", "--no-recheck"])?;
    assert!(!p("another-set/data5.txt").exists());
    assert!(!p("data6.txt").exists());
    let list_out = x(&["list", "data6.txt"])?;
    assert!(list_out.contains("data6.txt"), "{list_out}");

    x(&["recheck", "data6.txt"])?;
    assert!(p("data6.txt").exists());
    assert_eq!(fs::read_to_string(p("data6.txt"))?, CONTENT);

    clean_up(&xvc_root)
}
