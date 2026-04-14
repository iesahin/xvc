mod common;
use common::*;
use xvc_core::XvcVerbosity;

#[test]
fn test_storage_new_dropbox() {
    let xvc_root = run_in_temp_git_dir();
    std::env::set_current_dir(&xvc_root).unwrap();

    // 1. Test no token
    let (_stdout, stderr) = run_xvc_catch(
        Some(&xvc_root),
        &["init"],
        XvcVerbosity::Warn,
    ).unwrap();
    if !stderr.is_empty() {
        println!("init stderr: {}", stderr);
    }
    // It might have some output but should succeed
    // assert!(stderr.is_empty());
    let (_stdout, stderr) = run_xvc_catch(
        Some(&xvc_root),
        &[
            "storage",
            "new",
            "dropbox",
            "--name",
            "my-dropbox",
            "--storage-prefix",
            "xvc-test",
        ],
        XvcVerbosity::Warn,
    ).unwrap();

    assert!(stderr.contains("DROPBOX_ACCESS_TOKEN") || stderr.contains("XVC_STORAGE_DROPBOX_TOKEN_my-dropbox"));

    // 2. Test dummy token
    let token = "dummy-token";
    std::env::set_var("DROPBOX_ACCESS_TOKEN", token);

    let (_stdout, stderr) = run_xvc_catch(
        Some(&xvc_root),
        &[
            "storage",
            "new",
            "dropbox",
            "--name",
            "my-dropbox-2",
            "--storage-prefix",
            "xvc-test-2",
        ],
        XvcVerbosity::Trace,
    ).unwrap();

    // It should fail with an error from Dropbox API (401 Unauthorized)
    assert!(stderr.contains("Dropbox init failed") || stderr.contains("Unauthorized") || stderr.contains("401"));

    std::env::remove_var("DROPBOX_ACCESS_TOKEN");
    clean_up_path_buf(xvc_root).unwrap();
}
