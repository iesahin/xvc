mod common;

use crate::common::test_logging;
use xvc_core::configuration::XvcOptionalConfiguration;
use xvc_core::XvcConfigResult as Result;

#[test]
fn test_from_env() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    std::env::set_var("XVC_CORE.VERBOSITY", "warn");
    std::env::set_var("XVC_GIT.AUTO_COMMIT", "false");

    let opt_config = XvcOptionalConfiguration::from_env();

    assert_eq!(
        opt_config.core.unwrap().verbosity.unwrap(),
        "warn".to_string()
    );
    assert_eq!(opt_config.git.unwrap().auto_commit.unwrap(), false);

    std::env::remove_var("XVC_CORE.VERBOSITY");
    std::env::remove_var("XVC_GIT.AUTO_COMMIT");
    Ok(())
}
