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
fn test_file_remove() -> Result<()> {
    let xvc_root = setup()?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["file"];
        c.extend(cmd);
        run_xvc(Some(&xvc_root), &c, XvcVerbosity::Default)
    };
    let p = |s: &str| xvc_root.join(&PathBuf::from(s));

    x(&["track", "data.txt"])?;
    assert_eq!(cache_paths(&xvc_root).len(), 1);

    // Either --from-cache or --from-storage is required
    let (success, _stdout, stderr) =
        run_xvc_unchecked(Some(&xvc_root), &["file", "remove", "data.txt"])?;
    assert!(
        !success,
        "remove without --from-cache/--from-storage should fail"
    );
    assert!(
        stderr.contains("required arguments"),
        "stderr should be a clap error: {stderr}"
    );

    // --from-cache deletes the cached copy; the file stays tracked and in the workspace
    let remove_out = x(&["remove", "--from-cache", "data.txt"])?;
    assert!(remove_out.contains("[DELETE]"), "{remove_out}");
    assert_eq!(cache_paths(&xvc_root).len(), 0);
    assert!(p("data.txt").exists());

    // carry-in --force puts it back to the cache
    x(&["carry-in", "--force", "data.txt"])?;
    assert_eq!(cache_paths(&xvc_root).len(), 1);

    // Create a second version, then delete only the old one by its hash prefix
    fs::write(p("data.txt"), "second version\n")?;
    x(&["carry-in", "data.txt"])?;
    let all_versions = cache_paths(&xvc_root);
    assert_eq!(all_versions.len(), 2);
    let old_cache_file = all_versions
        .iter()
        .find(|f| fs::read_to_string(f).map(|c| c == CONTENT).unwrap_or(false))
        .expect("the first version should be in the cache");
    // Cache paths are .xvc/b3/<3-char>/<3-char>/<rest>/0.ext; versions are
    // specified with dash-separated hash prefixes like c85-f3e
    let components: Vec<String> = old_cache_file
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();
    let b3_pos = components
        .iter()
        .position(|c| c == "b3")
        .expect("cache path should contain b3");
    let version_prefix = format!("{}-{}", components[b3_pos + 1], components[b3_pos + 2]);
    x(&[
        "remove",
        "--from-cache",
        "--only-version",
        &version_prefix,
        "data.txt",
    ])?;
    let remaining = cache_paths(&xvc_root);
    assert_eq!(remaining.len(), 1);
    assert_eq!(fs::read_to_string(&remaining[0])?, "second version\n");

    // --all-versions empties the cache for the target
    x(&["remove", "--from-cache", "--all-versions", "data.txt"])?;
    assert_eq!(cache_paths(&xvc_root).len(), 0);

    // Deduplicated cache files are kept unless --force is given
    x(&["carry-in", "--force", "data.txt"])?;
    x(&["copy", "data.txt", "data2.txt", "--as", "symlink"])?;
    assert_eq!(cache_paths(&xvc_root).len(), 1);
    let remove_out = x(&["remove", "--from-cache", "data.txt"])?;
    assert!(
        remove_out.contains("also used by"),
        "should report deduplicated use: {remove_out}"
    );
    assert_eq!(cache_paths(&xvc_root).len(), 1);
    // The dedup warning is emitted at warn level, so run with -v and check both streams
    let (success, stdout, stderr) = run_xvc_unchecked(
        Some(&xvc_root),
        &[
            "-v",
            "file",
            "remove",
            "--from-cache",
            "--force",
            "data.txt",
        ],
    )?;
    assert!(success, "forced removal should succeed: {stderr}");
    assert!(
        format!("{stdout}{stderr}").contains("even though"),
        "forced removal should warn about deduplicated use: {stdout}{stderr}"
    );
    assert_eq!(cache_paths(&xvc_root).len(), 0);

    clean_up(&xvc_root)
}
