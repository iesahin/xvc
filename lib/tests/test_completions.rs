mod common;
use common::*;
use xvc_config::XvcVerbosity;

use xvc::error::Result;

#[test]
fn test_completions() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    // let help_text = run_xvc(None, &[], XvcVerbosity::Default)?;
    // assert!(!help_text.is_empty());

    let shells = ["bash", "elvish", "fish", "powershell", "zsh"];

    for s in shells {
        std::env::set_var("COMPLETE", s);
        let complete_script = run_xvc(None, &[], XvcVerbosity::Default)?;
        // TODO: Test the contents of each of these scripts
        assert!(!complete_script.is_empty());
        // assert!(complete_script != help_text);
    }

    // Nushell only has static completions for the time.
    let nushell_completions = run_xvc(None, &["_comp", "generate-nushell"], XvcVerbosity::Default)?;
    assert!(!nushell_completions.is_empty());

    Ok(())
}
