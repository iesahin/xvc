mod common;
use std::{env, fs, path::PathBuf};

use log::LevelFilter;

use common::run_in_temp_xvc_dir;
use rand;
use subprocess::Exec;
use xvc::{error::Result, watch};
use xvc_config::XvcVerbosity;
use xvc_core::XvcRoot;
use xvc_remote::remote::XVC_REMOTE_GUID_FILENAME;
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
#[cfg_attr(not(feature = "test-s3"), ignore)]
fn test_remote_new_s3() -> Result<()> {
    common::test_logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let bucket_name = "xvc-test";
    let remote_prefix = common::random_dir_name("xvc-remote", None);

    let access_key = env::var("AWS_ACCESS_KEY_ID")?;
    let secret_key = env::var("AWS_SECRET_ACCESS_KEY")?;
    let region = env::var("AWS_DEFAULT_REGION").unwrap_or("us-east-1".to_string());

    let x = |cmd: &[&str]| {
        let mut c = vec!["xvc"];
        c.extend(cmd);
        watch!(cmd);
        xvc::test_dispatch(Some(&xvc_root), c, XvcVerbosity::Warn)
    };

    let aws_create_bucket = sh(format!("aws s3 mb s3://{bucket_name}"));
    watch!(aws_create_bucket);
    //
    // Set the password in the environment
    env::set_var("XVC_REMOTE_ACCESS_KEY_ID", access_key);
    env::set_var("XVC_REMOTE_SECRET_KEY", secret_key);

    let out = x(&[
        "remote",
        "new-s3",
        "--name",
        "s3-remote",
        "--bucket-name",
        bucket_name,
        "--remote-prefix",
        &remote_prefix,
        "--region",
        &region,
    ])?;

    watch!(out);

    let s3_bucket_list = sh(format!("aws s3 ls --recursive 's3://{bucket_name}/' | rg {remote_prefix} | rg {XVC_REMOTE_GUID_FILENAME}"));
    watch!(s3_bucket_list);
    assert!(s3_bucket_list.len() > 0);

    let the_file = "file-0000.bin";

    let file_track_result = x(&["file", "track", the_file])?;
    watch!(file_track_result);

    let cache_dir = xvc_root.xvc_dir().join("b3");

    let file_list_before = sh(format!(
        "aws s3 ls --recursive {bucket_name} | rg {remote_prefix} | rg 0.bin"
    ));
    watch!(file_list_before);
    let n_remote_files_before = file_list_before.lines().count();
    let push_result = x(&["file", "push", "--to", "s3-remote", the_file])?;
    watch!(push_result);

    let file_list_after = sh(format!(
        "aws s3 ls --recursive {bucket_name} | rg {remote_prefix} | rg 0.bin"
    ));
    watch!(file_list_after);

    // The file should be in:
    // - remote_dir/REPO_ID/b3/ABCD...123/0.bin

    let n_remote_files_after = file_list_after.lines().count();

    assert!(
        n_remote_files_before + 1 == n_remote_files_after,
        "{} - {}",
        n_remote_files_before,
        n_remote_files_after
    );

    // remove all cache
    fs::remove_dir_all(&cache_dir)?;

    let fetch_result = x(&["file", "fetch", "--from", "s3-remote"])?;

    watch!(fetch_result);

    let n_local_files_after_fetch = jwalk::WalkDir::new(&cache_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();

    assert!(n_remote_files_after == n_local_files_after_fetch);

    let cache_dir = xvc_root.xvc_dir().join("b3");
    fs::remove_dir_all(&cache_dir)?;
    fs::remove_file(the_file)?;

    let pull_result = x(&["file", "pull", "--from", "s3-remote"])?;
    watch!(pull_result);

    let n_local_files_after_pull = jwalk::WalkDir::new(&cache_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();

    assert!(n_remote_files_after == n_local_files_after_pull);
    assert!(PathBuf::from(the_file).exists());

    Ok(())
}
