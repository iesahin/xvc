mod common;

use crate::common::{run_in_temp_dir, test_logging};
use std::fs;
use xvc_core::AbsolutePath;
use xvc_core::XvcConfigResult as Result;
use xvc_core::{XvcConfig, XvcConfigOptionSource, XvcLoadParams};

#[test]
fn test_find_value_source() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let temp_dir = run_in_temp_dir();
    let current_dir = AbsolutePath::from(temp_dir.clone());

    let project_config_path = temp_dir.join("xvc.toml");
    fs::write(&project_config_path, "[git]\nuse_git = false")?;

    // This seems to leak from other tests
    std::env::remove_var("XVC_GIT.USE_GIT");
    std::env::set_var("XVC_FILE.RECHECK.METHOD", "reflink");

    let params = XvcLoadParams::new(current_dir.clone(), Some(current_dir))
        .include_system_config(false)
        .include_user_config(false)
        .project_config_path(Some(AbsolutePath::from(project_config_path)))
        .command_line_config(Some(vec!["core.verbosity=trace".to_string()]));

    let config_loader = XvcConfig::new_v2(&params)?;

    assert_eq!(
        config_loader.find_value_source("core.verbosity"),
        Some(XvcConfigOptionSource::CommandLine)
    );

    assert_eq!(
        config_loader.find_value_source("file.recheck.method"),
        Some(XvcConfigOptionSource::Environment)
    );

    assert_eq!(
        config_loader.find_value_source("git.use_git"),
        Some(XvcConfigOptionSource::Project)
    );

    assert_eq!(
        config_loader.find_value_source("file.track.force"),
        Some(XvcConfigOptionSource::Default)
    );

    assert_eq!(config_loader.find_value_source("invalid.key"), None);

    std::env::remove_var("XVC_FILE.RECHECK.METHOD");
    fs::remove_dir_all(temp_dir)?;
    Ok(())
}
