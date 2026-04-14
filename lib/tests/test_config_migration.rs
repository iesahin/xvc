mod common;
use common::*;
use std::fs;
use xvc::error::Result;
use xvc_core::XvcVerbosity;
use xvc_core::configuration::{OptionalCoreConfig, OptionalGitConfig, XvcOptionalConfiguration};

#[test]
fn test_config_migration_core_guid() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let xvc_root = run_in_temp_xvc_dir()?;
    let xvc_dir = xvc_root.xvc_dir();
    let config_path = xvc_dir.join("config.toml");
    let guid_path = xvc_dir.join("guid");
    let gitignore_path = xvc_root.absolute_path().join(".gitignore");

    // 1. Get original GUID
    let original_guid = fs::read_to_string(&guid_path)?;

    // 2. Remove .xvc/guid
    fs::remove_file(&guid_path)?;
    assert!(!guid_path.exists());

    // 3. Add core.guid to config.toml
    let config = XvcOptionalConfiguration {
        core: Some(OptionalCoreConfig {
            verbosity: Some("warn".to_string()),
            guid: Some(original_guid.trim().to_string()),
            ..Default::default()
        }),
        git: Some(OptionalGitConfig {
            use_git: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    };

    let new_config_content = toml::to_string(&config).unwrap();
    fs::write(&config_path, new_config_content)?;

    // 4. Modify .gitignore to remove the lines we want to test restoration of
    let gitignore_content = fs::read_to_string(&gitignore_path)?;
    let new_gitignore_content = gitignore_content
        .lines()
        .filter(|l| !l.contains("!.xvc/guid") && !l.contains("!.xvc/pipelines/"))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&gitignore_path, new_gitignore_content)?;

    // 5. Run an Xvc command to trigger migration
    run_xvc(Some(&xvc_root), &["root"], XvcVerbosity::Trace)?;

    // 6. Assertions

    // .xvc/guid should exist and contain the original guid
    assert!(guid_path.exists(), "guid file should exist after migration");
    let migrated_guid = fs::read_to_string(&guid_path)?;
    assert_eq!(migrated_guid.trim(), original_guid.trim(), "Guid content mismatch");

    // config.toml should NOT contain guid = ...
    let migrated_config = fs::read_to_string(&config_path)?;
    assert!(!migrated_config.contains("guid ="), "config.toml should not contain guid key");
    
    // .gitignore should contain the restored lines
    let migrated_gitignore = fs::read_to_string(&gitignore_path)?;
    assert!(migrated_gitignore.contains("!.xvc/guid"), ".gitignore should contain !.xvc/guid");
    assert!(migrated_gitignore.contains("!.xvc/pipelines/"), ".gitignore should contain !.xvc/pipelines/");

    clean_up(&xvc_root)
}
