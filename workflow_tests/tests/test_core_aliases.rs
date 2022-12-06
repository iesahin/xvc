mod common;
use common::*;
use xvc_config::XvcVerbosity;

use xvc::test_dispatch;

use xvc::error::Result;

#[test]
fn test_aliases() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let out = test_dispatch(None, vec!["aliases"], XvcVerbosity::Trace)?;

    assert!(out.len() > 0);

    Ok(())
}
