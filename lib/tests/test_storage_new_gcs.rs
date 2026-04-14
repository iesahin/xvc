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
#[cfg_attr(not(feature = "test-gcs"), ignore)]
fn test_storage_new_gcs() -> Result<()> {
    common::test_logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let bucket_name = "xvc-test";
    let storage_prefix = common::random_dir_name("xvc-storage", None);

    let region = "europe-west3";

    let gsutil = |cmd: &str, append: &str| -> String {
        let sh_cmd = format!("gsutil {cmd} {append}");
        sh(sh_cmd)
    };

    let x = |cmd: &[&str]| -> Result<String> {
        common::run_xvc(Some(&xvc_root), cmd, XvcVerbosity::Warn)
    };

    x(&[
        "storage",
        "new",
        "gcs",
        "--name",
        "gcs-storage",
        "--bucket-name",
        bucket_name,
        "--storage-prefix",
        &storage_prefix,
        "--region",
        region,
    ])?;

    let s3_bucket_list = gsutil(
        &format!("ls gs://{bucket_name}/{storage_prefix}/**"),
        &format!("| rg {XVC_STORAGE_GUID_FILENAME}"),
    );
    assert!(!s3_bucket_list.is_empty());

    let the_file = "file-0000.bin";

    x(&["file", "track", the_file])?;

    let cache_dir = xvc_root.xvc_dir().join("b3");

    let file_list_before = gsutil(
        &format!("ls gs://{bucket_name}/{storage_prefix}/**"),
        &format!("| rg 0.bin || true"),
    );
    let n_storage_files_before = if file_list_before.trim().is_empty() { 0 } else { file_list_before.lines().count() };
    x(&["file", "send", "--to", "gcs-storage", the_file])?;

    let file_list_after = gsutil(
        &format!("ls gs://{bucket_name}/{storage_prefix}/**"),
        &format!("| rg 0.bin"),
    );

    let n_storage_files_after = file_list_after.lines().count();

    assert!(
        n_storage_files_before + 1 == n_storage_files_after,
        "{} - {}",
        n_storage_files_before,
        n_storage_files_after
    );

    // remove all cache
    sh(format!("rm -rf {}", cache_dir.to_string_lossy()));

    x(&["file", "bring", "--no-recheck", "--from", "gcs-storage"])?;

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

    x(&["file", "bring", "--from", "gcs-storage"])?;

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
