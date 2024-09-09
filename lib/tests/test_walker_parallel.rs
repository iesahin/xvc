use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use log::LevelFilter;
use xvc_walker::*;

use test_case::test_case;

use xvc::error::Result;
use xvc_logging::watch;
use xvc_test_helper::*;
use xvc_walker::AbsolutePath;

// TODO: Patterns shouldn't have / prefix, but an appropriate PathKind
#[test_case(true => matches Ok(_); "this is to refresh the dir for each test run")]
// This builds a directory hierarchy to run the tests
fn create_directory_hierarchy(force: bool) -> Result<AbsolutePath> {
    let temp_dir: PathBuf = seeded_temp_dir("xvc-walker", None);

    if force && temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }

    if !temp_dir.exists() {
        // in parallel tests, sometimes this fail
        fs::create_dir(&temp_dir)?;
        create_directory_tree(&temp_dir, 10, 10, 1000, Some(47))?;
        // root/dir1 may have another tree
        let level_1 = &temp_dir.join("dir-0001");
        create_directory_tree(level_1, 10, 10, 1000, Some(47))?;
        // and another level
        let level_2 = &level_1.join("dir-0001");
        create_directory_tree(level_2, 10, 10, 1000, Some(47))?;
    }

    Ok(AbsolutePath::from(temp_dir))
}

fn new_dir_with_ignores(
    root: &str,
    dir: Option<&str>,
    initial_patterns: &str,
) -> Result<IgnoreRules> {
    let patterns = create_patterns(root, dir, initial_patterns);
    let initialized = IgnoreRules::empty(&PathBuf::from(root), Some(".gitignore"));
    watch!(patterns);
    initialized.add_patterns(patterns).unwrap();
    Ok(initialized)
}

fn create_patterns(root: &str, dir: Option<&str>, patterns: &str) -> Vec<Pattern> {
    xvc_walker::content_to_patterns(Path::new(root), dir.map(Path::new), patterns)
}

#[test_case("", "" => it contains "dir-0002/file-0001.bin" ; "t3733909666")]
#[test_case("", "file-0001.bin" => it not contains "dir-0002/file-0001.bin" ; "t2733909666")]
#[test_case("", "dir-0002/" => it not contains "dir-0002/file-0001.bin" ; "t3433909666")]
#[test_case("", "dir-0002/*" => it  not contains "dir-0002/file-0001.bin" ; "t4733909666")]
#[test_case("dir-0002/", "file-0001.bin" => it not contains "dir-0002/file-0001.bin" ; "t2312253429" )]
#[test_case("dir-0002/", "*" => it not contains "dir-0002/file-0001.bin" ; "t1653614181" )]
#[test_case("dir-0002/", "**" => it not contains "dir-0002/file-0001.bin" ; "t987815317" )]
#[test_case("dir-0002/", "*.bin" => it not contains "dir-0002/file-0001.bin" ; "t2105105898" )]
#[test_case("", "*.bin\n!file-0001.*" => it contains "dir-0002/file-0001.bin" ; "t3772817176" )]
#[test_case("", "!*.bin\nfile-0001.*" => it contains "dir-0002/file-0001.bin" ; "t3272817176" )]
#[test_case("", "*.bin\n!dir-0002/*" => it contains "dir-0002/file-0001.bin" ; "t3572817176" )]
#[test_case("", "*.bin\n!dir-0002/**" => it contains "dir-0002/file-0001.bin" ; "t7772817176" )]
fn test_walk_parallel(ignore_src: &str, ignore_content: &str) -> Vec<String> {
    watch!(ignore_src);
    watch!(ignore_content);
    test_logging(LevelFilter::Trace);
    let root = create_directory_hierarchy(true).unwrap();
    let (path_sender, path_receiver) = crossbeam_channel::unbounded();
    // We assume ignore_src is among the directories created
    fs::write(
        format!("{}/{ignore_src}.gitignore", root.to_string_lossy()),
        ignore_content,
    )
    .unwrap();
    let initial_rules = Arc::new(RwLock::new(
        new_dir_with_ignores(root.to_string_lossy().as_ref(), None, "").unwrap(),
    ));
    let walk_options = WalkOptions {
        ignore_filename: Some(".gitignore".to_owned()),
        include_dirs: true,
    };
    walk_parallel(initial_rules, &root, walk_options, path_sender).unwrap();
    let paths = path_receiver
        .iter()
        .filter_map(|e| match e {
            Ok(p) => Some(
                p.path
                    .strip_prefix(&root)
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
            ),
            Err(_) => None,
        })
        .collect();
    watch!(paths);
    paths
}
