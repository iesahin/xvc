mod common;
use common::*;

use std::{env, fs, path::PathBuf};

use log::LevelFilter;

use subprocess::Exec;
use xvc::error::Result;
use xvc_core::XvcVerbosity;
use xvc_core::XvcRoot;
use xvc_storage::storage::XVC_STORAGE_GUID_FILENAME;
use xvc_test_helper::{create_directory_tree, generate_filled_file};

use crate::common::run_xvc;

fn write_s3cmd_config(region: &str, access_key: &str, secret_key: &str) -> Result<String> {
    let config_file_name = env::temp_dir().join(format!(
        "{}.cfg",
        common::random_dir_name("wasabi-config", None)
    ));
    let config = format!(
        r#"[default]
access_key = {access_key}
access_token =
add_encoding_exts =
add_headers =
bucket_location = eu-central-1
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
host_base = {region}.digitaloceanspaces.com
host_bucket = %(bucket)s.{region}.digitaloceanspaces.com
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
use_https = True
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
    create_directory_tree(&temp_dir, 10, 10, 1000, Some(23))?;
    // root/dir1 may have another tree
    let level_1 = &temp_dir.join(&PathBuf::from("dir-0001"));
    create_directory_tree(level_1, 10, 10, 1000, Some(23))?;

    Ok(temp_dir)
}

fn sh(cmd: String) -> String {
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

    let config_file_name = write_s3cmd_config(region, &access_key, &secret_key)?;

    let s3cmd = |cmd: &str, append: &str| -> String {
        let sh_cmd = format!("s3cmd --config {config_file_name} {cmd} {append}");
        sh(sh_cmd)
    };

    let x = |cmd: &[&str]| -> Result<String> { run_xvc(Some(&xvc_root), cmd, XvcVerbosity::Trace) };

    x(&[
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
        region,
    ])?;

    let s3_bucket_list = s3cmd(
        &format!("ls --recursive 's3://{bucket_name}/'"),
        &format!("| rg {storage_prefix} | rg {XVC_STORAGE_GUID_FILENAME}"),
    );
    assert!(!s3_bucket_list.is_empty());

    let the_file = "file-0000.bin";

    x(&["file", "track", the_file])?;

    let cache_dir = xvc_root.xvc_dir().join("b3");

    let file_list_before = s3cmd(
        &format!("ls --recursive s3://{bucket_name}"),
        &format!("| rg {storage_prefix} | rg 0.bin"),
    );
    let n_storage_files_before = file_list_before.lines().count();
    x(&["file", "send", "--to", "do-storage", the_file])?;

    let file_list_after = s3cmd(
        &format!("ls --recursive s3://{bucket_name}"),
        &format!("| rg {storage_prefix} | rg 0.bin"),
    );

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
    sh(format!("rm -rf {}", cache_dir.to_string_lossy()));

    x(&["file", "bring", "--no-recheck", "--from", "do-storage"])?;

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

    x(&["file", "bring", "--from", "do-storage"])?;

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

    // Set remote specific passwords and remove DO ones
    env::set_var("XVC_STORAGE_ACCESS_KEY_ID_do-storage", access_key);
    env::set_var("XVC_STORAGE_SECRET_KEY_do-storage", secret_key);

    env::remove_var("DIGITAL_OCEAN_ACCESS_KEY_ID");
    env::remove_var("DIGITAL_OCEAN_SECRET_ACCESS_KEY");

    x(&["file", "bring", "--from", "do-storage"])?;

    clean_up(&xvc_root)
}
