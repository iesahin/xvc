mod common;
use std::env;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, path::PathBuf};

use crate::common::{run_in_example_xvc, run_in_temp_xvc_dir};
use jwalk;
use regex::Regex;
use subprocess::Exec;
use xvc::error::{Error, Result};
use xvc::watch;
use xvc_config::XvcVerbosity;
use xvc_core::XvcRoot;
use xvc_test_helper::{create_directory_tree, generate_filled_file};

fn create_directory_hierarchy() -> Result<XvcRoot> {
    let temp_dir: XvcRoot = run_in_temp_xvc_dir()?;
    // for checking the content hash
    generate_filled_file(&temp_dir.join(&PathBuf::from("file-0000.bin")), 10000, 100);
    create_directory_tree(&temp_dir, 10, 10)?;
    // root/dir1 may have another tree
    let level_1 = &temp_dir.join(&PathBuf::from("dir-0001"));
    create_directory_tree(&level_1, 10, 10)?;

    Ok(temp_dir)
}

fn sh(cmd: String) -> String {
    watch!(cmd);
    Exec::shell(cmd).capture().unwrap().stdout_str()
}

#[test]
fn test_file_track_serial() -> Result<()> {
    // setup::logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["file"];
        c.extend(cmd);
        common::run_xvc(Some(&xvc_root), &c, XvcVerbosity::Trace)
    };

    let re_match = |output, regex| {
        let regex = Regex::new(regex).unwrap();
        assert!(regex.is_match(output), "output: {}", output);
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
    let track_file_0 = x(&["track", file_0, "--no-parallel"])?;
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

    // re add should change n_files
    let n_files_before = jwalk::WalkDir::new(".xvc/b3").into_iter().count();

    let second_add = x(&["track", file_0, "--no-parallel"])?;
    watch!(second_add);

    let n_files_after = jwalk::WalkDir::new(".xvc/b3").into_iter().count();
    assert!(
        n_files_after == n_files_before,
        "n_files_after: {n_files_after}"
    );

    let gitignore_1_after = fs::read_to_string(&gitignore_file_1)?;

    assert!(
        gitignore_1_after == gitignore_1,
        "{} != {}",
        gitignore_1,
        gitignore_1_after
    );

    // track dirs
    let dir_to_add = "dir-0002/";

    let n_files_before = jwalk::WalkDir::new(".xvc/b3")
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();
    let track_dir_to_add = x(&["track", dir_to_add, "--no-parallel"])?;
    watch!(track_dir_to_add);

    let n_files_after = jwalk::WalkDir::new(".xvc/b3")
        .into_iter()
        .filter(|f| {
            watch!(f);
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

    fs::remove_file(file_0)?;

    let list_after_delete = x(&["list", "--recursive"])?;

    let data_line = line_captures(&list_after_delete, file_0);

    assert!(data_line[0].len() > 0, "{}", data_line[0]);

    // todo!("Test different types of cache (symlink, hardlink, copy, reflink");

    //

    let git_status_res = sh("git status -s".to_string());
    let gs_xvc = line_captures(&git_status_res, r#".* \.xvc/.*"#);
    assert!(
        gs_xvc.is_empty(),
        "{gs_xvc:?} shouldn't contain any `.xvc/` lines."
    );

    let gs_gitignore = line_captures(&git_status_res, r#".* \.gitignore"#);
    assert!(
        gs_gitignore.is_empty(),
        "{gs_gitignore:?} shouldn't contain any `.gitignore` lines."
    );

    let gs_xvcignore = line_captures(&git_status_res, r#".* \.xvcignore"#);
    assert!(
        gs_xvcignore.is_empty(),
        "{gs_xvcignore:?} shouldn't contain any `.xvcignore` lines."
    );

    Ok(())
}
