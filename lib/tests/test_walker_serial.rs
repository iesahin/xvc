use std::{fs, path::PathBuf};

use test_case::test_case;

use xvc::error::Result;
use xvc_core::{walk_serial, AbsolutePath, WalkOptions};
use xvc_test_helper::*;

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

#[test_case("", "" => it contains "/dir-0002/file-0001.bin" ; "t3733909666")]
#[test_case("", "file-0001.bin" => it not contains "/dir-0002/file-0001.bin" ; "t2733909666")]
#[test_case("", "dir-0002/" => it not contains "/dir-0002/file-0001.bin" ; "t3433909666")]
#[test_case("", "dir-0002/*" => it  not contains "/dir-0002/file-0001.bin" ; "t4733909666")]
#[test_case("dir-0002/", "file-0001.bin" => it not contains "/dir-0002/file-0001.bin" ; "t2312253429" )]
#[test_case("dir-0002/", "*" => it not contains "/dir-0002/file-0001.bin" ; "t1653614181" )]
#[test_case("dir-0002/", "**" => it not contains "/dir-0002/file-0001.bin" ; "t987815317" )]
#[test_case("dir-0002/", "*.bin" => it not contains "/dir-0002/file-0001.bin" ; "t2105105898" )]
#[test_case("", "*.bin\n!file-0001.*" => it contains "/dir-0002/file-0001.bin" ; "t3772817176" )]
#[test_case("", "!*.bin\nfile-0001.*" => it contains "/dir-0002/file-0001.bin" ; "t3272817176" )]
#[test_case("", "*.bin\n!dir-0002/*" => it contains "/dir-0002/file-0001.bin" ; "t3572817176" )]
#[test_case("", "*.bin\n!dir-0002/**" => it contains "/dir-0002/file-0001.bin" ; "t7772817176" )]
#[test_case("dir-0001/", "/dir-0001/" => it not contains "/dir-0001/dir-0001/" ; "t9772817176" )]
#[test_case("dir-0001/", "/dir-0001/" => it not contains "/dir-0001/dir-0001/file-0001.bin" ; "t9972817176" )]
fn test_walk_serial(ignore_src: &str, ignore_content: &str) -> Vec<String> {
    test_logging(log::LevelFilter::Trace);
    let root = create_directory_hierarchy(true).unwrap();
    // We assume ignore_src is among the directories created
    fs::write(
        format!("{}/{ignore_src}.gitignore", root.to_string_lossy()),
        ignore_content,
    )
    .unwrap();
    let walk_options = WalkOptions {
        ignore_filename: Some(".gitignore".to_owned()),
        include_dirs: true,
    };
    let (output_sender, _output_receiver) = crossbeam_channel::unbounded();
    let (res_paths, _ignore_rules) = walk_serial(&output_sender, "", &root, &walk_options).unwrap();
    fs::remove_dir_all(&root).unwrap();
    let out_paths = res_paths
        .iter()
        .map(|pm| {
            let p = pm.path.to_string_lossy().to_string();

            p.strip_prefix(&root.to_string()).unwrap_or(&p).to_owned()
        })
        .collect();
    out_paths
}
