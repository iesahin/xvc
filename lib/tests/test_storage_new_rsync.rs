mod common;
use std::{env, fs, path::PathBuf};

use log::LevelFilter;

use common::*;
use subprocess::Exec;
use xvc::error::Result;
use xvc_core::XvcVerbosity;
use xvc_core::XvcRoot;
use xvc_storage::storage::XVC_STORAGE_GUID_FILENAME;
use xvc_test_helper::{create_directory_tree, generate_filled_file};

fn create_directory_hierarchy() -> Result<XvcRoot> {
    let temp_dir: XvcRoot = run_in_temp_xvc_dir()?;
    // for checking the content hash
    generate_filled_file(&temp_dir.join(&PathBuf::from("file-0000.bin")), 10000, 100);
    create_directory_tree(&temp_dir, 10, 10, 1000, Some(47))?;
    // root/dir1 may have another tree
    let level_1 = &temp_dir.join(&PathBuf::from("dir-0001"));
    create_directory_tree(level_1, 10, 10, 1000, Some(47))?;

    Ok(temp_dir)
}

fn sh(cmd: String) -> String {
    Exec::shell(cmd).capture().unwrap().stdout_str()
}

#[test]
#[cfg_attr(not(feature = "test-rsync"), ignore)]
fn test_storage_new_rsync() -> Result<()> {
    common::test_logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let storage_dir_name = format!(
        "/tmp/{}/",
        common::random_dir_name("xvc-storage", Some(111))
    );
    let test_user = "iex";
    let test_host = "e1.xvc.dev";
    let url = format!("{test_user}@{test_host}");
    let local_test_dir = env::temp_dir().join(common::random_dir_name("xvc-storage-copy", None));

    let x = |cmd: &[&str]| -> Result<String> {
        common::run_xvc(Some(&xvc_root), cmd, XvcVerbosity::Warn)
    };

    sh(format!(
        "ssh {url} 'test -e {storage_dir_name} && rm -rf {storage_dir_name}'"
    ));

    x(&[
        "storage",
        "new",
        "rsync",
        "--name",
        "rsync-storage",
        "--host",
        test_host,
        "--user",
        test_user,
        "--storage-dir",
        &storage_dir_name,
    ])?;

    let storage_list = sh(format!(
        "ssh {url} 'ls -l {storage_dir_name}{XVC_STORAGE_GUID_FILENAME}'"
    ));

    assert!(!storage_list.is_empty());

    let the_file = "file-0000.bin";

    x(&["file", "track", the_file])?;

    let n_storage_files_before = jwalk::WalkDir::new(local_test_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();
    x(&["file", "send", "--to", "rsync-storage", the_file])?;

    let file_list = sh(format!("ssh {url} 'ls -1R {storage_dir_name} | grep bin'"));

    // The file should be in:
    // - storage_dir/REPO_ID/b3/ABCD...123/0.bin

    let n_storage_files_after = file_list.lines().count();

    assert!(
        n_storage_files_before + 1 == n_storage_files_after,
        "{} - {}",
        n_storage_files_before,
        n_storage_files_after
    );

    // remove all cache
    //
    let cache_dir = xvc_root.xvc_dir().join("b3");
    sh(format!("rm -rf {}", cache_dir.to_string_lossy()));

    x(&["file", "bring", "--no-recheck", "--from", "rsync-storage"])?;

    let n_local_files_after_fetch = jwalk::WalkDir::new(&cache_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();

    assert!(n_storage_files_after == n_local_files_after_fetch);

    let cache_dir = xvc_root.xvc_dir().join("b3");
    sh(format!("rm -rf {}", cache_dir.to_string_lossy()));
    fs::remove_file(the_file)?;

    x(&["file", "bring", "--from", "rsync-storage"])?;

    let n_local_files_after_pull = jwalk::WalkDir::new(&cache_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();

    assert!(n_storage_files_after == n_local_files_after_pull);
    assert!(PathBuf::from(the_file).exists());

    clean_up(&xvc_root)
}
