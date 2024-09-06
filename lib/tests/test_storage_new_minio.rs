mod common;
use common::*;
use std::{env, fs, path::PathBuf};

use log::LevelFilter;

use subprocess::Exec;
use xvc::{error::Result, watch};
use xvc_config::XvcVerbosity;
use xvc_core::XvcRoot;
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

fn mc_config(alias: &str, endpoint: &str, access_key: &str, secret_key: &str) -> String {
    format!(
        r##"
{{
	"version": "10",
	"aliases": {{
		"{alias}": {{
			"url": "{endpoint}",
			"accessKey": "{access_key}",
			"secretKey": "{secret_key}",
			"api": "s3v4",
			"path": "auto"
    }}
	}}
}}
"##
    )
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
    let endpoint = "http://e1.xvc.dev:9000";
    let bucket_name = "xvctests";
    let storage_prefix = common::random_dir_name("xvc", None);
    let region = "us-east-1";
    let local_test_dir = env::temp_dir().join(common::random_dir_name("xvc-storage-copy", None));
    let access_key = env::var("MINIO_ACCESS_KEY_ID")?;
    let secret_key = env::var("MINIO_SECRET_ACCESS_KEY")?;
    let alias_name = "xvc";
    let mc_config = mc_config(
        alias_name,
        endpoint,
        access_key.as_str(),
        secret_key.as_str(),
    );
    let mc_config_dir = xvc_root.xvc_dir().join(".mc");
    fs::create_dir_all(&mc_config_dir)?;
    let mc_config_file = mc_config_dir.join("config.json");
    fs::write(mc_config_file, mc_config)?;

    let mc = |cmd: &str, append: &str| -> String {
        let sh_cmd = format!("mc --config-dir {mc_config_dir} {cmd} {alias_name} {append}");
        sh(sh_cmd)
    };

    let x = |cmd: &[&str]| -> Result<String> {
        common::run_xvc(Some(&xvc_root), cmd, XvcVerbosity::Warn)
    };

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
        "--storage-prefix",
        &storage_prefix,
        "--region",
        region,
    ])?;

    watch!(out);

    let mc_bucket_list = mc("ls", &format!("| rg {bucket_name}"));
    watch!(mc_bucket_list);
    assert!(!mc_bucket_list.is_empty());

    let the_file = "file-0000.bin";

    let file_track_result = x(&["file", "track", the_file])?;
    watch!(file_track_result);

    let n_storage_files_before = jwalk::WalkDir::new(local_test_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();
    let push_result = x(&["file", "send", "--to", "minio-storage", the_file])?;
    watch!(push_result);

    let file_list = mc("ls -r ", &format!("| rg {bucket_name}/{storage_prefix}"));
    watch!(file_list);

    // The file should be in:
    // - storage_dir/REPO_ID/b3/ABCD...123/0.bin

    // Remove guid file from the count
    let n_storage_files_after = file_list.lines().count() - 1;

    assert!(
        n_storage_files_before + 1 == n_storage_files_after,
        "{} - {}",
        n_storage_files_before,
        n_storage_files_after
    );

    // remove all cache
    //
    let cache_dir = xvc_root.xvc_dir().join("b3");
    sh(format!("chmod -R +w {}", cache_dir.to_string_lossy()));
    sh(format!("rm -rf {}", cache_dir.to_string_lossy()));

    let fetch_result = x(&["file", "bring", "--no-recheck", "--from", "minio-storage"])?;

    watch!(fetch_result);

    let n_local_files_after_fetch = jwalk::WalkDir::new(&cache_dir)
        .into_iter()
        .filter(|f| {
            watch!(f);
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();

    assert!(
        n_storage_files_after == n_local_files_after_fetch,
        "{} - {}",
        n_storage_files_after,
        n_local_files_after_fetch
    );

    let cache_dir = xvc_root.xvc_dir().join("b3");
    sh(format!("chmod -R +w {}", cache_dir.to_string_lossy()));
    sh(format!("rm -rf {}", cache_dir.to_string_lossy()));
    fs::remove_file(the_file)?;

    let pull_result = x(&["file", "bring", "--from", "minio-storage"])?;
    watch!(pull_result);

    let n_local_files_after_pull = jwalk::WalkDir::new(&cache_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();

    assert!(
        n_storage_files_after == n_local_files_after_pull,
        "{} - {}",
        n_storage_files_after,
        n_local_files_after_pull
    );
    assert!(PathBuf::from(the_file).exists());

    // Set remote specific passwords and remove general ones
    env::set_var("XVC_STORAGE_ACCESS_KEY_ID_minio-storage", access_key);
    env::set_var("XVC_STORAGE_SECRET_KEY_minio-storage", secret_key);

    env::remove_var("MINIO_ACCESS_KEY_ID");
    env::remove_var("MINIO_SECRET_ACCESS_KEY");

    let pull_result_2 = x(&["file", "bring", "--from", "minio-storage"])?;
    watch!(pull_result_2);

    clean_up(&xvc_root)
}
