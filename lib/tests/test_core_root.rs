mod common;
use common::*;
use xvc_config::XvcVerbosity;
use xvc_test_helper::test_logging;
use xvc_walker::AbsolutePath;

use xvc::error::Result;

#[test]
fn test_root() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let xvc_root = run_in_temp_xvc_dir()?;

    let rel = run_xvc(Some(&xvc_root), &["root"], XvcVerbosity::Default)?;
    // assert!(
    //     rel.trim().to_string() == ".".to_string(),
    //     "Relative: {}",
    //     rel
    // );
    let abs = run_xvc(
        Some(&xvc_root),
        &["root", "--absolute"],
        XvcVerbosity::Default,
    )?;

    assert!(AbsolutePath::from(abs.trim().to_string()) == xvc_root.absolute_path().clone());
    clean_up(&xvc_root).map_err(|e| anyhow::anyhow!("Cleanup error: {e:?}"))?;
    Ok(())
}
