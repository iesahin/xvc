mod common;
use common::*;
use xvc_config::XvcVerbosity;
use xvc_walker::AbsolutePath;

use xvc_logging::watch;

use xvc::error::Result;

#[test]
fn test_root() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let xvc_root = run_in_temp_xvc_dir()?;

    watch!(xvc_root);

    let rel = run_xvc(Some(&xvc_root), &["root"], XvcVerbosity::Trace)?;
    watch!(rel);
    assert!(rel.trim().to_string() == ".".to_string(), "{}", rel.trim());
    let abs = run_xvc(
        Some(&xvc_root),
        &["root", "--absolute"],
        XvcVerbosity::Trace,
    )?;

    watch!(abs);
    assert!(AbsolutePath::from(abs.trim().to_string()) == xvc_root.absolute_path().clone());
    clean_up(&xvc_root)
}
