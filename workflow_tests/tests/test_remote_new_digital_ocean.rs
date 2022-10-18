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
#[cfg_attr(not(feature = "test-digital-ocean"), ignore)]
fn test_storage_new_digital_ocean() -> Result<()> {
    common::test_logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let bucket_name = "xvc";
    let storage_prefix = common::random_dir_name("xvc-storage", None);

    let access_key = env::var("DIGITAL_OCEAN_ACCESS_KEY_ID")?;
    let secret_key = env::var("DIGITAL_OCEAN_SECRET_ACCESS_KEY")?;
    let region = "fra1";

    let s3cmd = |cmd: &str, append: &str| -> String {
        let acc = access_key.clone();
        let sec = secret_key.clone();
        let sh_cmd = format!("s3cmd {cmd} --host={region}.digitaloceanspaces.com --access_key={acc} --secret_key={sec} {append}");
        sh(sh_cmd)
    };

    let x = |cmd: &[&str]| {
        let mut c = vec!["xvc"];
        c.extend(cmd);
        watch!(cmd);
        xvc::test_dispatch(Some(&xvc_root), c, XvcVerbosity::Warn)
    };

    // Set the password in the environment
    env::set_var("XVC_STORAGE_ACCESS_KEY_ID", access_key.clone());
    env::set_var("XVC_STORAGE_SECRET_KEY", secret_key.clone());

    let out = x(&[
        "storage",
        "new",
        "digital-ocean",
        "--name",
        "do-storage",
        "--bucket-name",
        bucket_name,
        "--storage-prefix",
        &storage_prefix,
        "--region",
        &region,
    ])?;

    watch!(out);
    let s3_bucket_list = s3cmd(
        &format!("ls --recursive 's3://{bucket_name}/'"),
        &format!("| rg {storage_prefix} | rg {XVC_STORAGE_GUID_FILENAME}"),
    );
    watch!(s3_bucket_list);
    assert!(s3_bucket_list.len() > 0);

    let the_file = "file-0000.bin";

    let file_track_result = x(&["file", "track", the_file])?;
    watch!(file_track_result);

    let cache_dir = xvc_root.xvc_dir().join("b3");

    let file_list_before = s3cmd(
        &format!("ls --recursive s3://{bucket_name}"),
        &format!("| rg {storage_prefix} | rg 0.bin"),
    );
    watch!(file_list_before);
    let n_storage_files_before = file_list_before.lines().count();
    let push_result = x(&["file", "push", "--to", "do-storage", the_file])?;
    watch!(push_result);

    let file_list_after = s3cmd(
        &format!("ls --recursive s3://{bucket_name}"),
        &format!("| rg {storage_prefix} | rg 0.bin"),
    );
    watch!(file_list_after);

    // The file should be in:
    // - storage_dir/REPO_ID/b3/ABCD...123/0.bin

    let n_storage_files_after = file_list_after.lines().count();

    assert!(
        n_storage_files_before + 1 == n_storage_files_after,
        "{} - {}",
        n_storage_files_before,
        n_storage_files_after
    );

    // remove all cache
    fs::remove_dir_all(&cache_dir)?;

    let fetch_result = x(&["file", "fetch", "--from", "do-storage"])?;

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

    let pull_result = x(&["file", "pull", "--from", "do-storage"])?;
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
