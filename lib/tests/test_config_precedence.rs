mod common;

use crate::common::{run_in_temp_dir, test_logging};
use std::fs;
use xvc_core::AbsolutePath;
use xvc_core::XvcConfigResult as Result;
use xvc_core::{XvcConfig, XvcLoadParams};

#[test]
fn test_xvc_config_new_v2_precedence() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let temp_dir = run_in_temp_dir();
    let current_dir = AbsolutePath::from(temp_dir.clone());

    // 1. Project config file
    let project_config_path = temp_dir.join("xvc.toml");
    let project_toml = r#"
[core]
verbosity = "info" # Project
[git]
use_git = false # Project
"#;
    fs::write(&project_config_path, project_toml)?;

    // 2. Local config file
    let local_config_path = temp_dir.join("xvc.local.toml");
    let local_toml = r#"#;

[core]
verbosity = "debug" # Local overrides project
#
"#;
    fs::write(&local_config_path, local_toml)?;

    // 3. Environment variables
    std::env::set_var("XVC_GIT.USE_GIT", "true"); // Env overrides project
    std::env::set_var("XVC_CACHE.ALGORITHM", "sha256"); // Env provides new value

    // 4. Command line
    let command_line_config = Some(vec!["core.verbosity=trace".to_string()]); // CLI overrides all

    let params = XvcLoadParams::new(current_dir.clone(), Some(current_dir))
        .include_system_config(false)
        .include_user_config(false)
        .project_config_path(Some(AbsolutePath::from(project_config_path)))
        .local_config_path(Some(AbsolutePath::from(local_config_path)))
        .command_line_config(command_line_config);

    let config_loader = XvcConfig::new_v2(&params)?;
    let config = config_loader.config();

    // Check precedence
    assert_eq!(config.core.verbosity, "trace"); // 1. CLI
    assert_eq!(config.git.use_git, true); // 2. Env
    assert_eq!(config.cache.algorithm, "sha256"); // 3. Env (new value)
    assert_eq!(config.file.track.force, false); // 4. Default (untouched)

    // Cleanup
    std::env::remove_var("XVC_GIT.USE_GIT");
    std::env::remove_var("XVC_CACHE.ALGORITHM");
    fs::remove_dir_all(temp_dir)?;
    Ok(())
}
