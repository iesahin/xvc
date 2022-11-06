//! Xvc Core Library for common operations
#![warn(missing_docs)]
#![forbid(unsafe_code)]
pub mod aliases;
pub mod check_ignore;
pub mod error;
pub mod root;
pub mod types;
pub mod util;

pub use types::cachetype::CacheType;
pub use types::hashalgorithm::HashAlgorithm;
pub use types::xvcdigest::CollectionDigest;
pub use types::xvcdigest::ContentDigest;
pub use types::xvcdigest::MetadataDigest;
pub use types::xvcdigest::XvcDigest;

pub use types::xvcfiletype::XvcFileType;
pub use types::xvcmetadata::XvcMetadata;
pub use types::xvcpath::TextOrBinary;
pub use types::xvcpath::XvcCachePath;
pub use types::xvcpath::XvcPath;
pub use types::xvcroot::XvcRoot;

pub use error::Error;
pub use error::Result;

pub use util::file::{
    all_paths_and_metadata, dir_includes, directory_paths, glob_includes, glob_paths,
    XvcPathMetadataMap,
};

pub use types::xvcroot::MetadataFileLocation;

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
";

/// The Git directory for a project.
///
/// This is not expected to change for some time.
pub const GIT_DIR: &str = ".git";

/// The initial content for `.xvc/.gitignore` to hide Xvc fs structures
///
/// Note that most elements are not hidden.
pub const GITIGNORE_INITIAL_CONTENT: &str = "
/config.local
/tmp
/cache
";

/// Creates a new project configuration by writing all default values.
///
/// # Arguments
///
/// - `use_git`: sets `core.use_git` option.
pub fn default_project_config(use_git: bool) -> String {
    let uuid = uuid::Uuid::new_v4();
    let guid = hex::encode(seahash::hash(uuid.as_bytes()).to_le_bytes());
    format!(
        r##"
[core]
# The repository id. Please do not delete or change it. (XVC)
guid = "{guid}"
# default verbosity level
verbosity = "error"

[git]
# whether to integrate with Git
# turning this off causes all git operations, including adding paths added to Xvc to be added to `.gitignore` or staging committing to Git after Xvc operations to be turned off.
# not recommended unless you're really not using Git
use_git = {use_git}
# git command to use when running git.
# set this to an absolute path to specify an executable
# if set to a non-absolute path, it will be searched in $PATH and run.
command = "git"

# commit any changes in .xvc/ directory after the commands
# you can handle git manually
auto_commit = true

# stage any changes in .xvc/ directory without committing
# if you want to commit after multiple Xvc commands, but don't want to stage after each operation you can turn auto-commit off and turn auto-stage on.
auto_stage = false

[cache]
# The cache type for XVC. It may take copy, hardlink, softlink, reflink as values
type = "copy"
location = "{guid}"
algorithm = "blake3"
path = "cache"

[file]

[file.add]
# true => don't store the file content in cache when added
no_commit = false
# true => don't cache the file hash results for quick retrieval
no_hash_cache = false
# true => force even if the files are already added
force = false
# whether add considers files always "text" or "binary" when moving to the cache, or decides like Git, ("auto")
text_or_binary = "auto"
# whether to add files/directories serial only
no_parallel = false

[file.list]
# columns for xvc file list command. you can reorder or remove columns
columns = "cache-type,cache-status,timestamp,size,name,content-hash"
# order for xvc file list. one of name, size, timestamp with asc or desc.
sort = "name-desc"

[pipeline]
# name of the current pipeline to run
current_pipeline = "default"
# name of the default pipeline
default = "default"
# name of the default params file name
default_params_file = "params.yaml"

"##,
        guid = guid,
        use_git = use_git,
    )
}
