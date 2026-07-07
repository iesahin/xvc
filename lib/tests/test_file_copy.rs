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
fn test_file_copy() -> Result<()> {
    let xvc_root = setup()?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["file"];
        c.extend(cmd);
        run_xvc(Some(&xvc_root), &c, XvcVerbosity::Default)
    };

    x(&["track", "data.txt"])?;
    assert_eq!(cache_paths(&xvc_root).len(), 1);

    // Copying a tracked file rechecks the destination without duplicating the cache
    x(&["copy", "data.txt", "data2.txt"])?;
    let data2 = xvc_root.join(&PathBuf::from("data2.txt"));
    assert!(data2.exists());
    assert_eq!(fs::read_to_string(&data2)?, CONTENT);
    assert_eq!(cache_paths(&xvc_root).len(), 1);

    // The recheck method of the destination can differ from the source
    x(&["copy", "data.txt", "data3.txt", "--as", "symlink"])?;
    let data3 = xvc_root.join(&PathBuf::from("data3.txt"));
    assert!(is_symlink(&data3), "data3.txt should be a symlink");
    let link_target = fs::read_link(&data3)?;
    assert!(
        link_target.to_string_lossy().contains(".xvc/b3"),
        "symlink should point into the cache: {link_target:?}"
    );

    // Glob source with a directory destination
    x(&["copy", "data*", "another-set/", "--as", "hardlink"])?;
    for f in ["data.txt", "data2.txt", "data3.txt"] {
        let copied = xvc_root.join(&PathBuf::from("another-set")).join(f);
        assert!(copied.exists(), "another-set/{f} should exist");
        assert_eq!(fs::read_to_string(&copied)?, CONTENT);
    }
    // Hardlinked copies don't add new cache files
    assert_eq!(cache_paths(&xvc_root).len(), 1);

    // Copying from a changed source is refused
    fs::write(
        xvc_root.join(&PathBuf::from("data.txt")),
        "changed content\n",
    )?;
    let (success, stdout, stderr) =
        run_xvc_unchecked(Some(&xvc_root), &["file", "copy", "data.txt", "data5.txt"])?;
    assert!(!success, "copy from a changed source should fail");
    assert!(
        format!("{stdout}{stderr}").contains("Sources have changed"),
        "stdout: {stdout}\nstderr: {stderr}"
    );
    assert!(!xvc_root.join(&PathBuf::from("data5.txt")).exists());

    // A missing workspace file can still be copied from the cache
    fs::remove_file(xvc_root.join(&PathBuf::from("data.txt")))?;
    x(&["copy", "data.txt", "data6.txt"])?;
    let data6 = xvc_root.join(&PathBuf::from("data6.txt"));
    assert!(data6.exists());
    assert_eq!(fs::read_to_string(&data6)?, CONTENT);

    // --no-recheck only records the copy without materializing it
    x(&["copy", "data.txt", "data7.txt", "--no-recheck"])?;
    let data7 = xvc_root.join(&PathBuf::from("data7.txt"));
    assert!(!data7.exists(), "data7.txt should not be rechecked yet");
    let list_out = x(&["list", "data7.txt"])?;
    assert!(list_out.contains("data7.txt"), "{list_out}");

    // ... until it's explicitly rechecked
    x(&["recheck", "data7.txt"])?;
    assert!(data7.exists());
    assert_eq!(fs::read_to_string(&data7)?, CONTENT);

    clean_up(&xvc_root)
}
