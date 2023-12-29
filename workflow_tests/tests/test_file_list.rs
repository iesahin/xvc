mod common;
use common::*;

use std::path::Path;

use log::LevelFilter;

use xvc::error::Result;
use xvc::watch;
use xvc_config::XvcVerbosity;
use xvc_test_helper::{create_directory_tree, test_logging};
use xvc_walker::AbsolutePath;

// This builds a directory hierarchy to run the tests
//
// ```txt
// temp_dir
// |
// +- dir-0001
// |  + dir-0001
// |  + dir-0002
// |  + dir-0003
// |  + ...
// + dir-0002
// + ...
// + file-0001.bin
// + ...
// + file-0005.bin
// ```
//
// Each directory contains 10 files. So there are 10 * 3 * 10 = 300 files.
fn create_directory_hierarchy(temp_dir: &Path) -> Result<AbsolutePath> {
    create_directory_tree(temp_dir, 5, 5, 1000, Some(23))?;
    Ok(AbsolutePath::from(temp_dir))
}

#[test]
#[cfg(unix)]
fn test_file_list() -> Result<()> {
    use std::path::PathBuf;

    test_logging(LevelFilter::Trace);
    let xvc_root = common::run_in_temp_xvc_dir()?;
    create_directory_hierarchy(&xvc_root)?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["file"];
        c.extend(cmd);
        common::run_xvc(Some(&xvc_root), &c, XvcVerbosity::Trace)
    };

    let _xd = |dir: &str, cmd: &[&str]| {
        let dir = &xvc_root.join(&PathBuf::from(dir));
        let mut c = vec!["-C", dir.to_str().unwrap(), "data"];
        c.extend(cmd);
        common::run_xvc(Some(&xvc_root), &c, XvcVerbosity::Trace)
    };

    watch!("begin");
    let list_all = x(&["list", "--format", "{{name}}", "--show-dot-files"])?;

    watch!(list_all);

    let count_all = list_all.trim().lines().count();
    watch!(count_all);
    // There must be 33 elements in total. 6 x 5: directories, 1 for .gitignore,
    // 1 for .xvcignore, another line for the summary.
    assert!(count_all == 33);

    let list_no_dots = x(&["list", "--format", "{{name}}"])?;
    let count_no_dots = list_no_dots.trim().lines().count();
    // There must be 31 elements in total. 6 x 5: directories another line for the summary.
    assert!(count_no_dots == 31);

    let list_no_dots_no_summary = x(&["list", "--format", "{{name}}", "--no-summary"])?;
    let count_no_dots_no_summary = list_no_dots_no_summary.trim().lines().count();
    // There must be 31 elements in total. 6 x 5: directories another line for the summary.
    assert!(count_no_dots_no_summary == 30);

    // test all sort options

    for (sort_option, top_element) in &[
        ("name-asc", "file-0001.bin"),
        ("name-desc", "file-0005.bin"),
        ("size-asc", "file-0001.bin"),
        ("size-desc", "file-0005.bin"),
        // The following two fail on GH CI
        // ("ts-asc", "file-0001.bin"),
        // ("ts-desc", "file-0005.bin"),
    ] {
        let cmd_res = x(&[
            "list",
            "--format",
            "{{name}}",
            "--sort",
            sort_option,
            "dir-0001",
        ])?;
        let top_line = cmd_res.lines().next().unwrap();
        assert!(
            top_line.ends_with(top_element),
            "sort: {sort_option}\ncmd_res: {}",
            cmd_res
        );
    }

    // TODO: Test for other formatting options, recheck methods, cache status.
    // Some of these tests are done in `xvc-file-list.md` file in the reference.

    clean_up(&xvc_root)
}
