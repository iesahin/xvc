mod common;
use common::*;
use xvc_config::XvcVerbosity;

use xvc::error::Result;

#[test]
fn test_aliases() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    assert_xvc(&["aliases"], None, XvcVerbosity::Trace, |out| out.len() > 0)
}
