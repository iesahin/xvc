mod common;
use common::*;
use xvc_config::XvcVerbosity;
use xvc_walker::AbsolutePath;

use xvc::test_dispatch;
use xvc_logging::watch;

use xvc::error::Result;

#[test]
fn test_alias() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let xvc_root = run_in_temp_xvc_dir()?;

    let rel = test_dispatch(Some(&xvc_root), vec!["xvc", "alias", "--bash"], XvcVerbosity::Trace)?;

    assert!(rel.trim().to_string() == ".".to_string());

    let abs = test_dispatch(
        Some(&xvc_root),
        vec!["xvc", "alias", "--zsh"],
        XvcVerbosity::Trace,
    )?;
    watch!(abs);
    assert!(AbsolutePath::from(abs.trim().to_string()) == xvc_root.absolute_path().clone());

    Ok(())
}
