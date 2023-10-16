mod common;
use common::*;
use log::LevelFilter;

use xvc::core::{XVCIGNORE_FILENAME, XVC_DIR};
use xvc::init::InitCLI;

use xvc::error::Result;

#[test]
fn test_remote_add() -> Result<()> {
    common::test_logging(LevelFilter::Trace);
    let _the_dir = common::run_in_temp_dir();
    let xvc_root = xvc::init::run(
        None,
        InitCLI {
            path: None,
            no_git: true,
            force: false,
        },
    )?;
    assert!(xvc_root.absolute_path().join(XVC_DIR).exists());
    assert!(xvc_root.absolute_path().join(XVCIGNORE_FILENAME).exists());
    assert!(!xvc_root.absolute_path().join(".gitignore").exists());

    clean_up(&xvc_root)
}
