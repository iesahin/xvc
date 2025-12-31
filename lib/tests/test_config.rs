mod common;

use crate::common::{run_in_temp_dir, test_logging};
use std::collections::HashMap;
use std::fs;
use xvc_core::configuration::{
    blank_optional_config, default_config, initial_xvc_config, merge_configs, OptionalCoreConfig,
    OptionalGitConfig, XvcConfiguration, XvcOptionalConfiguration,
};

use xvc_core::AbsolutePath;
use xvc_core::XvcConfigResult as Result;
use xvc_core::{XvcConfig, XvcConfigOptionSource, XvcLoadParams};

#[test]
fn test_default_config() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let config = default_config();
    assert_eq!(config.core.verbosity, "error");
    assert_eq!(config.git.use_git, true);
    assert_eq!(config.cache.algorithm, "blake3");
    assert_eq!(config.file.recheck.method, "copy");
    Ok(())
}

#[test]
fn test_blank_optional_config() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let opt_config = blank_optional_config();
    assert!(opt_config.core.is_none());
    assert!(opt_config.git.is_none());
    assert!(opt_config.cache.is_none());
    assert!(opt_config.file.is_none());
    assert!(opt_config.pipeline.is_none());
    assert!(opt_config.check_ignore.is_none());
    Ok(())
}

#[test]
fn test_merge_configs() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let base_config = default_config();
    let mut opt_config = blank_optional_config();
    opt_config.core = Some(OptionalCoreConfig {
        verbosity: Some("debug".to_string()),
        ..Default::default()
    });
    opt_config.git = Some(OptionalGitConfig {
        use_git: Some(false),
        ..Default::default()
    });

    let merged_config = merge_configs(&base_config, &opt_config);

    assert_eq!(merged_config.core.verbosity, "debug");
    assert_eq!(merged_config.git.use_git, false);
    // Unchanged value
    assert_eq!(merged_config.cache.algorithm, base_config.cache.algorithm);
    Ok(())
}

#[test]
fn test_from_hash_map() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let mut values = HashMap::new();
    values.insert("CORE.VERBOSITY".to_string(), "info".to_string());
    values.insert("GIT.USE_GIT".to_string(), "false".to_string());
    values.insert("CACHE.ALGORITHM".to_string(), "sha2".to_string());
    values.insert("UNKNOWN.KEY".to_string(), "some_value".to_string());

    let opt_config = XvcOptionalConfiguration::from_hash_map("", &values);

    assert_eq!(
        opt_config.core.unwrap().verbosity.unwrap(),
        "info".to_string()
    );
    assert_eq!(opt_config.git.unwrap().use_git.unwrap(), false);
    assert_eq!(
        opt_config.cache.unwrap().algorithm.unwrap(),
        "sha2".to_string()
    );

    Ok(())
}

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

#[test]
fn test_config_from_file() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let temp_dir = run_in_temp_dir();
    let config_path = temp_dir.join("test.toml");

    let toml_content = r#"#

[core]
verbosity = "debug"

[git]
use_git = false
#"#;

    fs::write(&config_path, toml_content)?;

    let opt_config = XvcOptionalConfiguration::from_file(&config_path)?;
    assert_eq!(
        opt_config.core.unwrap().verbosity.unwrap(),
        "debug".to_string()
    );
    assert_eq!(opt_config.git.unwrap().use_git.unwrap(), false);

    // Test full configuration loading
    let full_toml_content = toml::to_string(&default_config()).unwrap();
    let full_config_path = temp_dir.join("full.toml");
    fs::write(&full_config_path, full_toml_content)?;

    let full_config = XvcConfiguration::from_file(&full_config_path)?;
    assert_eq!(full_config.core.verbosity, "error");

    fs::remove_dir_all(temp_dir)?;
    Ok(())
}

#[test]
fn test_initial_xvc_config() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let base_config = default_config();
    let mut user_options = blank_optional_config();
    user_options.core = Some(OptionalCoreConfig {
        verbosity: Some("info".to_string()),
        ..Default::default()
    });

    let config_str = initial_xvc_config(base_config, user_options)?;

    assert!(config_str.contains("verbosity = \"info\""));
    assert!(config_str.contains("use_git = true")); // from default

    Ok(())
}

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

    dbg!(&config_loader.config().core.verbosity);

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
