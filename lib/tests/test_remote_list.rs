mod common;
use std::{fs, path::PathBuf};

use log::LevelFilter;

use common::run_in_temp_xvc_dir;
use regex::Regex;
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

#[test]
fn test_remote_list() -> Result<()> {
    common::test_logging(LevelFilter::Trace);
    let xvc_root = create_directory_hierarchy()?;
    let remote_dir = common::random_temp_dir(Some("xvc-remote"));

    let x = |cmd: &[&str]| {
        let mut c = vec!["xvc"];
        c.extend(cmd);
        watch!(cmd);
        xvc::test_dispatch(Some(&xvc_root), c, XvcVerbosity::Warn)
    };

    let out = x(&[
        "remote",
        "new",
        "local",
        "--name",
        "local-remote",
        "--path",
        &remote_dir.to_string_lossy().to_string(),
    ])?;

    watch!(out);

    let list_out = x(&["remote", "list"])?;

    watch!(list_out);

    // The output table should contain
    // - name
    // - guid
    // - path (or URL)
    // These can be put to Display implementation of remote
    let line_regex = Regex::new(&format!(
        "^Local:[ ]*local-remote.*{}\n",
        remote_dir.to_string_lossy()
    ))
    .expect("Regex error");
    assert!(line_regex.is_match(&list_out));

    Ok(())
}
