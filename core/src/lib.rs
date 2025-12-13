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
        CacheConfig, CheckIgnoreConfig, CoreConfig, FileCarryInConfig, FileConfig,
        FileListConfig, FileRecheckConfig, FileTrackConfig, GitConfig, PipelineConfig,
    },
    FromConfigKey, UpdateFromXvcConfig, XvcConfig, XvcConfigOptionSource, XvcConfigParams,
    XvcVerbosity,
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

/// The initial content for `.xvc/.gitignore` to hide files in .xvc/
///
/// We ignore all, and just track the store, entity counter and the configuration
pub const GITIGNORE_INITIAL_CONTENT: &str = "
## Following are required for Xvc to function correctly.
.xvc/*
!.xvc/store/
!.xvc/ec/
!.xvc/config.toml
";

/// Creates a new project configuration by writing all default values.
/// This is used when initializing a new project.
/// The repository GUID is created here.
///
/// # Arguments
///
/// - `use_git`: sets `core.use_git` option.
pub fn default_project_config(use_git: bool) -> String {
    let uuid = uuid::Uuid::new_v4();
    let guid = hex::encode(seahash::hash(uuid.as_bytes()).to_le_bytes());

    let config = ProjectConfig {
        core: CoreConfig {
            guid,
            verbosity: "error".to_string(),
        },
        git: GitConfig {
            use_git,
            command: "git".to_string(),
            auto_commit: true,
            auto_stage: false,
        },
        cache: CacheConfig {
            algorithm: "blake3".to_string(),
        },
        file: FileConfig {
            track: FileTrackConfig {
                no_commit: false,
                force: false,
                text_or_binary: "auto".to_string(),
                no_parallel: false,
                include_git_files: !use_git,
            },
            list: FileListConfig {
                format: "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}".to_string(),
                sort: "name-desc".to_string(),
                show_dot_files: false,
                no_summary: false,
                recursive: false,
                include_git_files: !use_git,
            },
            carry_in: FileCarryInConfig {
                force: false,
                no_parallel: false,
            },
            recheck: FileRecheckConfig {
                method: "copy".to_string(),
            },
        },
        pipeline: PipelineConfig {
            current_pipeline: "default".to_string(),
            default: "default".to_string(),
            default_params_file: "params.yaml".to_string(),
            process_pool_size: 4,
        },
        check_ignore: CheckIgnoreConfig { details: false },
    };

    format!(
        r##"
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = "{guid}"
# Default verbosity level.
# One of "error", "warn", "info"
verbosity = "{verbosity}"

[git]
# Automate git operations.
# Turning this off leads Xvc to behave as if it's not in a Git repository.
# Not recommended unless you're really not using Git
use_git = {use_git}
# Command to run Git process.
# You can set this to an absolute path to specify an executable
# If set to a non-absolute path, the executable will be searched in $PATH.
command = "{git_command}"

# Commit changes in .xvc/ directory after commands.
# You can set this to false if you want to commit manually.
auto_commit = {auto_commit}

# Stage changes in .xvc/ directory without committing.
# auto_commit implies auto_stage.
# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
auto_stage = {auto_stage}

[cache]
# The hash algorithm used for the cache.
# It may take blake3, blake2, sha2 or sha3 as values.
# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# The cache path is produced by prepending algorithm name to the cache.
# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
algorithm = "{cache_algorithm}"

[file]

[file.track]

# Don't move file content to cache after xvc file track
no_commit = {file_track_no_commit}
# Force to track files even if they are already tracked.
force = {file_track_force}

# Xvc calculates file content digest differently for text and binary files.
# This option controls whether to treat files as text or binary.
# It may take auto, text or binary as values.
# Auto check each file individually and treat it as text if it's text.
text_or_binary = "{file_track_text_or_binary}"

# Don't use parallelism in track operations.
# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
no_parallel = {file_track_no_parallel}

# Track files that are tracked by Git. 
include_git_files = {file_track_include_git_files}

[file.list]

# Format for `xvc file list` rows. You can reorder or remove columns.
# The following are the keys for each row:
# - {{acd64}}:  actual content digest. All 64 digits from the workspace file's content.
# - {{acd8}}:  actual content digest. First 8 digits the file content digest.
# - {{aft}}:  actual file type. Whether the entry is a file (F), directory (D),
#   symlink (S), hardlink (H) or reflink (R).
# - {{asz}}:  actual size. The size of the workspace file in bytes. It uses MB,
#   GB and TB to represent sizes larger than 1MB.
# - {{ats}}:  actual timestamp. The timestamp of the workspace file.
# - {{cst}}:  cache status. One of "=", ">", "<", "X", or "?" to show
#   whether the file timestamp is the same as the cached timestamp, newer,
#   older, not cached or not tracked.
# - {{name}}: The name of the file or directory.
# - {{rcd64}}:  recorded content digest. All 64 digits.
# - {{rcd8}}:  recorded content digest. First 8 digits.
# - {{rrm}}:  recorded recheck method. Whether the entry is linked to the workspace
#   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# - {{rsz}}:  recorded size. The size of the cached content in bytes. It uses
#   MB, GB and TB to represent sizes larged than 1MB.
# - {{rts}}:  recorded timestamp. The timestamp of the cached content.
#
# There are no escape sequences in the format string.
# If you want to add a tab, type it to the string.
# If you want to add a literal double curly brace, open an issue.
format = "{file_list_format}"

# Default sort order for `xvc file list`.
# Valid values are
# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
sort = "{file_list_sort}"

# Show dot files like .gitignore
show_dot_files = {file_list_show_dot_files}

# Do not show a summary for as the final row for `xvc file list`.
no_summary = {file_list_no_summary}

# List files recursively always.
recursive = {file_list_recursive}

# List files tracked by Git. 
include_git_files = {file_list_include_git_files}

[file.carry-in]
# Carry-in the files to cache always, even if they are already present.
force = {file_carry_in_force}

# Don't use parallel move/copy in carry-in
no_parallel = {file_carry_in_no_parallel}

[file.recheck]
# The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# The default is copy to make sure the options is portable.
# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# Note that hardlink and symlink are read-only as they link the files in cache.
method = "{file_recheck_method}"

[pipeline]
# Name of the current pipeline to run
current_pipeline = "{pipeline_current_pipeline}"
# Name of the default pipeline
default = "{pipeline_default}"
# Name of the default params file name
default_params_file = "{pipeline_default_params_file}"
# Number of command processes to run concurrently
process_pool_size = {pipeline_process_pool_size}
 
[check-ignore]
# Show details by default
details = {check_ignore_details}

"##,
        guid = config.core.guid,
        verbosity = config.core.verbosity,
        use_git = config.git.use_git,
        git_command = config.git.command,
        auto_commit = config.git.auto_commit,
        auto_stage = config.git.auto_stage,
        cache_algorithm = config.cache.algorithm,
        file_track_no_commit = config.file.track.no_commit,
        file_track_force = config.file.track.force,
        file_track_text_or_binary = config.file.track.text_or_binary,
        file_track_no_parallel = config.file.track.no_parallel,
        file_track_include_git_files = config.file.track.include_git_files,
        file_list_format = config.file.list.format,
        file_list_sort = config.file.list.sort,
        file_list_show_dot_files = config.file.list.show_dot_files,
        file_list_no_summary = config.file.list.no_summary,
        file_list_recursive = config.file.list.recursive,
        file_list_include_git_files = config.file.list.include_git_files,
        file_carry_in_force = config.file.carry_in.force,
        file_carry_in_no_parallel = config.file.carry_in.no_parallel,
        file_recheck_method = config.file.recheck.method,
        pipeline_current_pipeline = config.pipeline.current_pipeline,
        pipeline_default = config.pipeline.default,
        pipeline_default_params_file = config.pipeline.default_params_file,
        pipeline_process_pool_size = config.pipeline.process_pool_size,
        check_ignore_details = config.check_ignore.details,
    )
}

