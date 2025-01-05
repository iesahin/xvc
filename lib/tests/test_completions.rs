mod common;
use common::*;
use xvc_config::XvcVerbosity;

use xvc::error::Result;

#[test]
fn test_completions() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let out = run_xvc(None, &["completions"], XvcVerbosity::Default)?;
    assert!(!out.is_empty());

    let shells = ["bash", "elvish", "fish", "powershell", "zsh"];

    for s in shells {
        let out = run_xvc(None, &["completions", s], XvcVerbosity::Default)?;
        assert!(!out.is_empty());
    }

    Ok(())
}
