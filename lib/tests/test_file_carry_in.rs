mod common;
use common::*;

use std::fs;
use std::path::PathBuf;

use xvc::error::Result;
use xvc_core::XvcRoot;
use xvc_core::XvcVerbosity;

const CONTENT: &str = "Oh, data, my, data\n";

fn setup() -> Result<XvcRoot> {
    let xvc_root = run_in_temp_xvc_dir()?;
    fs::write(xvc_root.join(&PathBuf::from("data.txt")), CONTENT)?;
    Ok(xvc_root)
}

#[test]
fn test_file_carry_in() -> Result<()> {
    let xvc_root = setup()?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["file"];
        c.extend(cmd);
        run_xvc(Some(&xvc_root), &c, XvcVerbosity::Default)
    };
    let p = |s: &str| xvc_root.join(&PathBuf::from(s));

    x(&["track", "data.txt"])?;
    let initial_cache = cache_paths(&xvc_root);
    assert_eq!(initial_cache.len(), 1);
    assert_eq!(fs::read_to_string(&initial_cache[0])?, CONTENT);

    // Carrying in an unchanged file is a no-op
    x(&["carry-in", "data.txt"])?;
    assert_eq!(cache_paths(&xvc_root), initial_cache);

    // Carrying in a changed file adds a new cache version and keeps the old one
    let new_content = "carried in version\n";
    fs::write(p("data.txt"), new_content)?;
    x(&["carry-in", "data.txt"])?;
    let cache_after_change = cache_paths(&xvc_root);
    assert_eq!(cache_after_change.len(), 2);
    let new_cache_file = cache_after_change
        .iter()
        .find(|f| !initial_cache.contains(f))
        .expect("a new cache file should be created");
    assert_eq!(fs::read_to_string(new_cache_file)?, new_content);

    // --force re-adds the file even if the content digest is unchanged
    x(&["carry-in", "--force", "--no-parallel", "data.txt"])?;
    let cache_after_force = cache_paths(&xvc_root);
    assert_eq!(cache_after_force.len(), 2);
    assert_eq!(fs::read_to_string(new_cache_file)?, new_content);

    // Carry-in with explicit text/binary digest mode
    fs::write(p("data.txt"), "binary mode version\n")?;
    x(&["carry-in", "--text-or-binary", "binary", "data.txt"])?;
    assert_eq!(cache_paths(&xvc_root).len(), 3);

    clean_up(&xvc_root)
}
