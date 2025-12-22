//! Xvc Core Library for common operations
#![warn(missing_docs)]
#![forbid(unsafe_code)]
pub mod check_ignore;
pub mod error;
pub mod root;
pub mod types;
pub mod util;

pub use types::hashalgorithm::HashAlgorithm;
pub use types::recheckmethod::RecheckMethod;

pub use types::xvcdigest::content_digest::ContentDigest;
pub use types::xvcdigest::path_collection_digest::PathCollectionDigest;
pub use types::xvcdigest::stdout_digest::StdoutDigest;
pub use types::xvcdigest::url_get_digest::UrlContentDigest;
pub use types::xvcdigest::xvc_metadata_digest::XvcMetadataDigest;
pub use types::xvcdigest::AttributeDigest;
pub use types::xvcdigest::XvcDigest;
pub use types::xvcdigest::XvcDigests;

pub use types::diff::Diff;
pub use types::diff::DiffStore;
pub use types::diff::DiffStore2;
pub use types::diff::DiffStore3;
pub use types::diff::DiffStore4;

pub use types::diff::apply_diff;
pub use types::diff::diff_store;
pub use types::diff::update_with_actual;

pub use types::textorbinary::TextOrBinary;
pub use types::xvcfiletype::XvcFileType;
pub use types::xvcmetadata::XvcMetadata;
pub use types::xvcpath::XvcCachePath;
pub use types::xvcpath::XvcPath;
pub use types::xvcroot::XvcRoot;

pub use error::Error;
pub use error::Result;

// Reexported types
pub use xvc_ecs::error::Error as XvcEcsError;
pub use xvc_ecs::error::Result as XvcEcsResult;
pub use xvc_ecs::{
    persist, Event, EventLog, HStore, R11Store, R1NStore, RMNStore, SharedHStore, SharedXStore,
    Storable, VStore, XvcEntity, XvcStore,
};

pub use xvc_logging::{
    debug, error, info, output, panic, setup_logging, trace, uwo, uwr, warn, watch, XvcOutputLine,
    XvcOutputSender,
};

pub use xvc_walker as walker;
pub use xvc_walker::Error as XvcWalkerError;
pub use xvc_walker::Result as XvcWalkerResult;

pub use xvc_walker::{
    content_to_patterns, make_polling_watcher, path_metadata_map_from_file_targets, walk_parallel,
    walk_serial, AbsolutePath, Glob, IgnoreRules, MatchResult, PathEvent, PathSync, WalkOptions,
};

pub use xvc_config::error::Error as XvcConfigError;
pub use xvc_config::error::Result as XvcConfigResult;
pub use xvc_config::{
    conf,
    config_params::{
        CacheConfig, CheckIgnoreConfig, CoreConfig, FileCarryInConfig, FileConfig, FileListConfig,
        FileRecheckConfig, FileTrackConfig, GitConfig, PipelineConfig, XvcConfig,
    },
    FromConfigKey, UpdateFromXvcConfig, XvcConfig as XvcConfigLoader, XvcConfigOptionSource,
    XvcLoadParams, XvcVerbosity,
};

pub use util::git;

pub use util::file::{all_paths_and_metadata, dir_includes, glob_includes, glob_paths};
pub use util::git::{
    build_gitignore, exec_git, get_absolute_git_command, get_git_tracked_files, git_auto_commit,
    git_auto_stage, git_checkout_ref, git_ignored, handle_git_automation, inside_git,
    stash_user_staged_files, unstash_user_staged_files,
};

pub use util::pmp::XvcPathMetadataProvider;
pub use util::XvcPathMetadataMap;

/// Channel size for [crossbeam_channel::bounded] used across the library.
/// TODO: This can be configurable for smaller/larger RAM sizes.
pub const CHANNEL_BOUND: usize = 1000000;

/// The standard metadata directory
///
/// For an Xvc project under `dir`, all relevant metadata is kept under `dir/.xvc`
pub const XVC_DIR: &str = ".xvc";

/// The standard ignore filename
///
/// Xvc considers patterns in this filename to be ignored.
/// Patterns are identical in structure to Gitignore
pub const XVCIGNORE_FILENAME: &str = ".xvcignore";

/// Initial .xvgignore content when a project is initialized
///
/// This is written to [XVCIGNORE_FILENAME] in the root of repository once it's initialized.
pub const XVCIGNORE_INITIAL_CONTENT: &str = "
# Add patterns of files xvc should ignore, which could improve
# the performance.
# It's in the same format as .gitignore files.

.DS_Store
";

/// The Git directory for a project.
///
/// This is not expected to change for some time.
pub const GIT_DIR: &str = ".git";

/// Globally Unique Identified for the Xvc Repository / Project
///
/// It's stored in `.xvc/guid` file.
/// Storage commands use this to create different paths for different Xvc projects.

pub const GUID_FILENAME: &str = "guid"

/// The initial content for `.xvc/.gitignore` to hide files in .xvc/
///
/// We ignore all, and just track the store, entity counter and the configuration
pub const GITIGNORE_INITIAL_CONTENT: &str = "
## Following are required for Xvc to function correctly.
.xvc/*
!.xvc/guid
!.xvc/store/
!.xvc/ec/
!.xvc/config.toml
!.xvc/pipelines/
";

/// Deserializes the default project configuration string into a [ProjectConfig] struct.
///
/// # Arguments
///
/// - `use_git`: sets `core.use_git` option.
pub fn get_default_project_config_struct(use_git: bool) -> Result<XvcConfig> {
    let config_str = default_project_config(use_git);
    let config: XvcConfig =
        toml::from_str(&config_str).map_err(|e| Error::TomlDeserializationError { source: e })?;
    Ok(config)
}
