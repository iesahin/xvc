mod common;
use common::*;

use std::thread::sleep;
use std::time::Duration;
use std::{fs, path::PathBuf};

use crate::common::run_in_temp_xvc_dir;
use jwalk;
use regex::Regex;
use xvc::error::{Error, Result};
use xvc::watch;
use xvc_config::XvcVerbosity;
use xvc_core::XvcRoot;
use xvc_test_helper::{create_directory_tree, generate_filled_file};

fn create_directory_hierarchy() -> Result<XvcRoot> {
    let temp_dir: XvcRoot = run_in_temp_xvc_dir()?;
    // for checking the content hash
    generate_filled_file(&temp_dir.join(&PathBuf::from("file-0000.bin")), 10000, 100);
    create_directory_tree(&temp_dir, 10, 10, Some(23))?;
    // root/dir1 may have another tree
    let level_1 = &temp_dir.join(&PathBuf::from("dir-0001"));
    create_directory_tree(&level_1, 10, 10, Some(23))?;

    Ok(temp_dir)
}
#[test]
fn test_file_track_parallel() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["file"];
        c.extend(cmd);
        common::run_xvc(Some(&xvc_root), &c, XvcVerbosity::Trace)
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

    let images_cache_path =
        ".xvc/b3/a57/262/2134fcb28679d2de66d225cc2a41c2594baa909781c0726eb7702baeb1/0.bin";
    let file_0 = "file-0000.bin";
    let track_file_0 = x(&["track", file_0])?;
    watch!(track_file_0);
    let cache_path = xvc_root.absolute_path().join(images_cache_path);
    watch!(cache_path);

    assert!(cache_path.exists());

    let gitignore_file_1 = PathBuf::from(".gitignore");

    assert!(
        gitignore_file_1.exists(),
        "Cannot find {:?}",
        gitignore_file_1
    );

    let gitignore_1 = fs::read_to_string(&gitignore_file_1)?;
    watch!(gitignore_1);

    assert!(
        gitignore_1.lines().filter(|l| l.ends_with(file_0)).count() == 1,
        "{}",
        gitignore_1
    );
    sleep(Duration::from_secs(1));
    let dir_to_add = "dir-0002/";

    let n_files_before = jwalk::WalkDir::new(".xvc/b3")
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();

    let track_dir_to_add = x(&["-vvvv", "track", dir_to_add])?;
    watch!(track_dir_to_add);

    let n_files_after = jwalk::WalkDir::new(".xvc/b3")
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();

    assert!(
        n_files_after - n_files_before == 10,
        "n_files_before: {n_files_before}\nn_files_after: {n_files_after}"
    );
    let gitignore_file_2 = PathBuf::from(&dir_to_add)
        .parent()
        .ok_or(Error::PathHasNoParent {
            path: dir_to_add.into(),
        })?
        .join(".gitignore");
    let gitignore_2 = fs::read_to_string(gitignore_file_2)?;
    assert!(
        gitignore_2
            .lines()
            .filter(|l| l.ends_with("dir-0002/"))
            .count()
            == 1,
        "{}\ncount: {}",
        gitignore_2,
        gitignore_2
            .lines()
            .filter(|l| l.ends_with("dir-0002/"))
            .count(),
    );

    let n_files_before = jwalk::WalkDir::new(".xvc/b3").into_iter().count();
    // re add should change n_files
    let second_add = x(&["track", file_0])?;
    watch!(second_add);

    let n_files_after = jwalk::WalkDir::new(".xvc/b3").into_iter().count();
    assert!(
        n_files_after == n_files_before,
        "n_files_after: {n_files_after}"
    );

    let gitignore_2_after = fs::read_to_string(&gitignore_file_1)?;

    assert!(
        gitignore_2_after == gitignore_2,
        "{} != {}",
        gitignore_2,
        gitignore_2_after
    );

    fs::remove_file(file_0)?;

    let list_after_delete = x(&["list", "--recursive"])?;

    let data_line = line_captures(&list_after_delete, file_0);

    assert!(data_line[0].len() > 0, "{}", data_line[0]);

    clean_up(&xvc_root)
}
