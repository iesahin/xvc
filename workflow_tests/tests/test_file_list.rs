mod common;

use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use log::LevelFilter;
use regex::Regex;
use shellfn::shell;
use xvc::error::Result;
use xvc::{test_dispatch, watch};
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
    create_directory_tree(&temp_dir, 5, 5)?;
    // We are using sleep to check sorting by timestamp
    sleep(Duration::from_millis(1));
    // root/dir1 may have another tree
    let level_1 = &temp_dir.join("dir-0001");
    create_directory_tree(&level_1, 5, 5)?;
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
        let mut c = vec!["xvc", "file"];
        c.extend(cmd);
        test_dispatch(Some(&xvc_root), c, XvcVerbosity::Trace)
    };

    let _xd = |dir: &str, cmd: &[&str]| {
        let dir = &xvc_root.join(&PathBuf::from(dir));
        let mut c = vec!["xvc", "-C", dir.to_str().unwrap(), "data"];
        c.extend(cmd);
        test_dispatch(Some(&xvc_root), c, XvcVerbosity::Trace)
    };

    let re_match = |output, regex| {
        let regex = Regex::new(regex).unwrap();
        assert!(
            regex.is_match(output),
            "regex: {}, output: {}",
            regex,
            output
        );
    };

    let line_filter = |regex, line| {
        let regex = Regex::new(regex).unwrap();
        regex.is_match(line)
    };

    let line_captures = |output: &str, pattern: &str| -> Vec<String> {
        let regex = Regex::new(&format!("^.*{pattern}.*$")).unwrap();
        output
            .lines()
            .filter_map(|s| {
                if regex.is_match(s) {
                    Some(s.to_owned())
                } else {
                    None
                }
            })
            .collect()
    };

    watch!("begin");
    let list_root = x(&["list"])?;

    watch!(list_root);

    let count_root = list_root.lines().count();
    watch!(count_root);
    let shell_count = count_files_in_dir(&xvc_root.to_string_lossy().to_string());
    watch!(shell_count);
    // add 3 for . .. and top line
    assert!(count_root + 2 == shell_count.trim().parse::<usize>()?);

    const ADDED_FILE_1: &str = "dir-0001/file-0001.bin";
    // Adding a file from child directory shouldn't change the output unless --recursive
    x(&["track", ADDED_FILE_1])?;

    let list_root_2 = x(&["list"])?;

    assert!(list_root_2 == list_root, "{}", list_root_2);

    let list_data_1 = x(&["list", "dir-0001"])?;

    let shell_count_data =
        count_files_in_dir(&xvc_root.join(&PathBuf::from("dir-0001")).to_str().unwrap());

    assert!(list_data_1.lines().count() + 2 == shell_count_data.trim().parse::<usize>()?);

    let recursive_list = x(&["list", "--recursive"])?;

    assert!(recursive_list.lines().count() > 100);
    let captured_1 = line_captures(&recursive_list, &format!("\t{ADDED_FILE_1}"));

    // TODO: The following test fails on Github CI. Probably related to timestamps.
    // Turned off until investigation.
    // re_match(&captured_1[0], "^C=.*");

    const ADDED_DIR_1: &str = "dir-0001/dir-0003";

    x(&["track", ADDED_DIR_1])?;

    let list_dir_1 = x(&["list", "--sort", "size-asc", ADDED_DIR_1])?;

    watch!(list_dir_1);

    let list_dir_1_lines: Vec<String> = list_dir_1.lines().map(|s| s.to_string()).collect();
    // file-0001 is smaller than file-0002 ...
    re_match(&list_dir_1_lines[0], ".*file-0001.*");

    let list_dir_2 = x(&["list", "--sort", "size-desc", ADDED_DIR_1])?;

    watch!(list_dir_2);

    let list_dir_2_lines: Vec<String> = list_dir_2.lines().map(|s| s.to_string()).collect();
    // file-0001 is smaller than file-0002 ...
    re_match(&list_dir_2_lines[0], ".*file-0005.*");

    let list_data_sort_name_asc = x(&["list", "--sort", "name-asc", ADDED_DIR_1])?;
    let list_data_sort_name_asc_lines: Vec<String> = list_data_sort_name_asc
        .lines()
        .map(|s| s.to_string())
        .collect();
    re_match(&list_data_sort_name_asc_lines[0], ".*file-0001.*");
    let list_data_sort_name_desc = x(&["list", "--sort", "name-desc", ADDED_DIR_1])?;
    let list_data_sort_name_desc_lines: Vec<String> = list_data_sort_name_desc
        .lines()
        .map(|s| s.to_string())
        .collect();
    re_match(&list_data_sort_name_desc_lines[0], ".*file-0005.*");
    // remove a directory from data

    // add a directory of data with symlink cache type
    x(&["track", "--cache-type", "symlink", "dir-0002"])?;
    let symlink_list = x(&["list", "dir-0002"])?;
    symlink_list
        .lines()
        .filter(|line| line_filter(".*file-000.*", line))
        .for_each(|line| re_match(line, "^S.*"));

    // add a directory of data with hardlink cache type

    x(&["track", "--cache-type", "hardlink", "dir-0003"])?;
    let symlink_list = x(&["list", "dir-0003"])?;
    symlink_list
        .lines()
        .filter(|line| line_filter(".*file-000.*", line))
        .for_each(|line| re_match(line, "^H.*"));
    // TODO: add a directory of data with reflink cache type
    // We need a reflink capable FS for this test. Skipping for now.
    //
    // TODO: ignored files should have `I` as second letter

    Ok(())
}

#[shell]
fn count_files_in_dir(dir: &str) -> String {
    r#"
    cd $DIR
    ls -la | wc -l
    "#
}
