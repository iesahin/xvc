mod common;

use xvc::core::{XVCIGNORE_FILENAME, XVC_DIR};
use xvc::init::InitCLI;

use xvc::error::Result;

#[test]
fn test_init_with_git() -> Result<()> {
    let _the_dir = common::run_in_temp_git_dir();
    let xvc_root = xvc::init::run(
        None,
        InitCLI {
            path: None,
            no_git: false,
            force: false,
        },
    )?;

    // TODO: Add assertions about stores
    assert!(xvc_root.absolute_path().join(XVC_DIR).exists());
    assert!(xvc_root.absolute_path().join(XVCIGNORE_FILENAME).exists());
    assert!(xvc_root.absolute_path().join(".gitignore").exists());

    Ok(())
}
