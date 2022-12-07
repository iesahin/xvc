mod common;
use common::*;
use xvc_config::XvcVerbosity;

use xvc::error::Result;

#[test]
fn test_aliases() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let out = run_xvc(None, &["aliases"], XvcVerbosity::Trace)?;
    assert!(out.len() > 0);
    Ok(())
}
