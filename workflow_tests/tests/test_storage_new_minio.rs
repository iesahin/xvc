mod common;
use std::{env, fs, path::PathBuf};

use log::LevelFilter;

use common::run_in_temp_xvc_dir;
use rand;
use subprocess::Exec;
use xvc::{error::Result, watch};
use xvc_config::XvcVerbosity;
use xvc_core::XvcRoot;
use xvc_storage::storage::XVC_STORAGE_GUID_FILENAME;
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
#[cfg_attr(not(feature = "test-minio"), ignore)]
fn test_storage_new_minio() -> Result<()> {
    common::test_logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let endpoint = "http://emresult.com:9000";
    let bucket_name = "one";
    let remote_prefix = common::random_dir_name("xvc", None);
    let region = "us-east-1";
    let local_test_dir = env::temp_dir().join(common::random_dir_name("xvc-storage-copy", None));
    let local_test_dir_str = local_test_dir.to_string_lossy().to_string();
    let access_key = env::var("MINIO_ACCESS_KEY_ID")?;
    let secret_key = env::var("MINIO_SECRET_ACCESS_KEY")?;

    let mc_alias = "one";

    let x = |cmd: &[&str]| {
        let mut c = vec!["xvc"];
        c.extend(cmd);
        watch!(cmd);
        xvc::test_dispatch(Some(&xvc_root), c, XvcVerbosity::Warn)
    };

    let mc_create_alias = sh(format!(
        "mc alias set xvc {endpoint} {access_key} {secret_key}"
    ));
    watch!(mc_create_alias);
    let mc_create_bucket = sh(format!("mc mb xvc/{bucket_name}"));
    watch!(mc_create_bucket);

    // Set the password in the environment
    env::set_var("XVC_STORAGE_ACCESS_KEY_ID", access_key);
    env::set_var("XVC_STORAGE_SECRET_KEY", secret_key);

    let out = x(&[
        "storage",
        "new",
        "minio",
        "--name",
        "minio-storage",
        "--endpoint",
        endpoint,
        "--bucket-name",
        bucket_name,
        "--remote-prefix",
        &remote_prefix,
        "--region",
        region,
    ])?;

    watch!(out);

    let mc_bucket_list = sh(format!("mc ls xvc | rg {bucket_name}"));
    watch!(mc_bucket_list);
    assert!(mc_bucket_list.len() > 0);

    let the_file = "file-0000.bin";

    let file_track_result = x(&["file", "track", the_file])?;
    watch!(file_track_result);

    let n_storage_files_before = jwalk::WalkDir::new(&local_test_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();
    let push_result = x(&["file", "push", "--to", "minio-storage", the_file])?;
    watch!(push_result);

    let file_list = sh(format!("mc ls -r xvc/one/{remote_prefix} | rg 0.bin"));
    watch!(file_list);

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
    fs::remove_dir_all(&cache_dir)?;

    let fetch_result = x(&["file", "fetch", "--from", "minio-storage"])?;

    watch!(fetch_result);

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
    fs::remove_dir_all(&cache_dir)?;
    fs::remove_file(the_file)?;

    let pull_result = x(&["file", "pull", "--from", "minio-storage"])?;
    watch!(pull_result);

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

    Ok(())
}
