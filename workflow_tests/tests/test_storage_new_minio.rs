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

fn write_s3cmd_config(access_key: &str, secret_key: &str) -> Result<String> {
    let config_file_name = env::temp_dir().join(format!(
        "{}.cfg",
        common::random_dir_name("minio-config", None)
    ));
    let config = format!(
        r#"[default]
access_key = {access_key}
access_token =
add_encoding_exts =
add_headers =
bucket_location = us-east-1
ca_certs_file =
cache_file =
check_ssl_certificate = True
check_ssl_hostname = True
cloudfront_host = cloudfront.amazonaws.com
connection_max_age = 5
connection_pooling = True
content_disposition =
content_type =
default_mime_type = binary/octet-stream
delay_updates = False
delete_after = False
delete_after_fetch = False
delete_removed = False
dry_run = False
enable_multipart = True
encoding = UTF-8
encrypt = False
expiry_date =
expiry_days =
expiry_prefix =
follow_symlinks = False
force = False
get_continue = False
gpg_command = None
gpg_decrypt = %(gpg_command)s -d --verbose --no-use-agent --batch --yes --passphrase-fd %(passphrase_fd)s -o %(output_file)s %(input_file)s
gpg_encrypt = %(gpg_command)s -c --verbose --no-use-agent --batch --yes --passphrase-fd %(passphrase_fd)s -o %(output_file)s %(input_file)s
gpg_passphrase =
guess_mime_type = True
host_base = one.emresult.com:9000
host_bucket = one.emresult.com:9000
human_readable_sizes = False
invalidate_default_index_on_cf = False
invalidate_default_index_root_on_cf = True
invalidate_on_cf = False
kms_key =
limit = -1
limitrate = 0
list_allow_unordered = False
list_md5 = False
log_target_prefix =
long_listing = False
max_delete = -1
mime_type =
multipart_chunk_size_mb = 15
multipart_copy_chunk_size_mb = 1024
multipart_max_chunks = 10000
preserve_attrs = True
progress_meter = True
proxy_host =
proxy_port = 0
public_url_use_https = False
put_continue = False
recursive = False
recv_chunk = 65536
reduced_redundancy = False
requester_pays = False
restore_days = 1
restore_priority = Standard
secret_key = {secret_key}
send_chunk = 65536
server_side_encryption = False
signature_v2 = False
signurl_use_https = False
simpledb_host = sdb.amazonaws.com
skip_existing = False
socket_timeout = 300
ssl_client_cert_file =
ssl_client_key_file =
stats = False
stop_on_error = False
storage_class =
throttle_max = 100
upload_id =
urlencoding_mode = normal
use_http_expect = False
use_https = False
use_mime_magic = True
verbosity = WARNING
website_endpoint = http://%(bucket)s.s3-website-%(location)s.amazonaws.com/
website_error =
website_index = index.html
"#
    );

    fs::write(&config_file_name, config)?;

    Ok(config_file_name.to_string_lossy().to_string())
}

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
    let storage_prefix = common::random_dir_name("xvc", None);
    let region = "us-east-1";
    let local_test_dir = env::temp_dir().join(common::random_dir_name("xvc-storage-copy", None));
    let access_key = env::var("MINIO_ACCESS_KEY_ID")?;
    let secret_key = env::var("MINIO_SECRET_ACCESS_KEY")?;

    let config_file_name = write_s3cmd_config(&access_key, &secret_key)?;
    watch!(config_file_name);

    let s3cmd = |cmd: &str, append: &str| -> String {
        let sh_cmd = format!("s3cmd --config {config_file_name} {cmd} {append}");
        sh(sh_cmd)
    };

    let x = |cmd: &[&str]| -> Result<String> {
        common::run_xvc(Some(&xvc_root), cmd, XvcVerbosity::Warn)
    };

    let create_bucket_res = s3cmd(&format!("mb {bucket_name}"), "");

    watch!(create_bucket_res);

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

    let mc_bucket_list = s3cmd(&format!("ls xvc"), &format!("| rg {bucket_name}"));
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
    let push_result = x(&["file", "send", "--to", "minio-storage", the_file])?;
    watch!(push_result);

    let file_list = s3cmd(
        &format!("ls -r s3://one/{storage_prefix}"),
        &format!("| rg 0.bin"),
    );
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

    let fetch_result = x(&["file", "bring", "--no-checkout", "--from", "minio-storage"])?;

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

    assert!(n_storage_files_after == n_local_files_after_pull);
    assert!(PathBuf::from(the_file).exists());

    // Set remote specific passwords and remove general ones
    env::set_var("XVC_STORAGE_ACCESS_KEY_ID_minio-storage", access_key);
    env::set_var("XVC_STORAGE_SECRET_KEY_minio-storage", secret_key);

    env::remove_var("MINIO_ACCESS_KEY_ID");
    env::remove_var("MINIO_SECRET_ACCESS_KEY");

    let pull_result_2 = x(&["file", "bring", "--from", "minio-storage"])?;
    watch!(pull_result_2);

    Ok(())
}
