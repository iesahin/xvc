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
fn test_remote_new_generic_rsync() -> Result<()> {
    common::test_logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let remote_dir_name = format!("/tmp/{}/", common::random_dir_name("xvc-remote", Some(111)));
    let test_host = "iex@one.emresult.com";
    let url = format!("{test_host}");
    let local_test_dir = env::temp_dir().join(common::random_dir_name("xvc-remote-copy", None));
    let local_test_dir_str = local_test_dir.to_string_lossy().to_string();

    let x = |cmd: &[&str]| {
        let mut c = vec!["xvc"];
        c.extend(cmd);
        watch!(cmd);
        xvc::test_dispatch(Some(&xvc_root), c, XvcVerbosity::Warn)
    };

    watch!(url);
    watch!(remote_dir_name);
    let delete_dir = sh(format!(
        "ssh {url} 'test -e {remote_dir_name} && rm -rf {remote_dir_name}'"
    ));
    watch!(delete_dir);

    let out = x(&[
        "remote",
        "new",
        "generic",
        "--name",
        "generic-remote",
        "--url",
        &url,
        "--remote-dir",
        &remote_dir_name,
        "--init",
        "ssh {URL} 'mkdir -p {REMOTE_DIR}' ; rsync -av {LOCAL_GUID_FILE_PATH} {URL}:{REMOTE_GUID_FILE_PATH}",
        "--list",
        "ssh {URL} 'ls -1 {REMOTE_DIR}'",
        "--upload",
        "ssh {URL} 'mkdir -p {REMOTE_DIR}{XVC_GUID}/{RELATIVE_CACHE_DIR}' ; rsync -av {ABSOLUTE_CACHE_PATH} {URL}:{REMOTE_DIR}{XVC_GUID}/{RELATIVE_CACHE_PATH}",
        "--download",
        "mkdir -p {ABSOLUTE_CACHE_DIR} ; rsync -av {URL}:{REMOTE_DIR}{XVC_GUID}/{RELATIVE_CACHE_PATH} {ABSOLUTE_CACHE_PATH}",
        "--delete",
        "ssh {URL} 'rm {REMOTE_DIR}{RELATIVE_CACHE_PATH}'",
        "--processes",
        "4",
    ])?;

    watch!(out);

    let remote_list = sh(format!(
        "ssh {url} 'ls -l {remote_dir_name}{XVC_REMOTE_GUID_FILENAME}'"
    ));

    watch!(remote_list);
    assert!(remote_list.len() > 0);

    let the_file = "file-0000.bin";

    let file_track_result = x(&["file", "track", the_file])?;
    watch!(file_track_result);

    let n_remote_files_before = jwalk::WalkDir::new(&local_test_dir)
        .into_iter()
        .filter(|f| {
            f.as_ref()
                .map(|f| f.file_type().is_file())
                .unwrap_or_else(|_| false)
        })
        .count();
    let push_result = x(&["file", "push", "--to", "generic-remote", the_file])?;
    watch!(push_result);

    let file_list = sh(format!("ssh {url} 'ls -1R {remote_dir_name} | grep bin'"));
    watch!(file_list);

    // The file should be in:
    // - remote_dir/REPO_ID/b3/ABCD...123/0.bin

    let n_remote_files_after = file_list.lines().count();

    assert!(
        n_remote_files_before + 1 == n_remote_files_after,
        "{} - {}",
        n_remote_files_before,
        n_remote_files_after
    );

    // remove all cache
    //
    let cache_dir = xvc_root.xvc_dir().join("b3");
    fs::remove_dir_all(&cache_dir)?;

    let fetch_result = x(&["file", "fetch", "--from", "generic-remote"])?;

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

    let pull_result = x(&["file", "pull", "--from", "generic-remote"])?;
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
