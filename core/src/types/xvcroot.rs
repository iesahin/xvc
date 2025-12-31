//! The home of [XvcRoot], the primary data structure for Xvc repository.
//!
//! It's used to pass around the repository information and configuration.
use std::fmt;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use xvc_config::configuration::merge_configs;
use xvc_config::configuration::XvcConfiguration;
use xvc_config::configuration::XvcOptionalConfiguration;
use xvc_config::default_config;
use xvc_config::initial_xvc_configuration_file;
use xvc_ecs::ecs::timestamp;
use xvc_ecs::{XvcEntity, XvcEntityGenerator};
use xvc_logging::watch;
use xvc_walker::AbsolutePath;

use xvc_config::{XvcConfig, XvcLoadParams};

use crate::error::{Error, Result};
use crate::GITIGNORE_INITIAL_CONTENT;
use crate::GUID_FILENAME;
use crate::XVCIGNORE_FILENAME;
use crate::XVCIGNORE_INITIAL_CONTENT;
use crate::XVC_DIR;

/// The primary data structure for Xvc repository.
///
/// It's created from `.xvc` directory and the config. It contains all the
/// information about the repository.
///
/// It loads [entity generator][XvcEntityGenerator] from `.xvc/ec/` files. This
/// is the place it's initialized and there can only be a single instance of it.
///
/// It contains the [configuration][XvcConfig] loaded from `.xvc/config.toml`
/// and other sources.
///
/// It contains the [store][XvcStore] which is the main data structure for Xvc.
/// [Storable] structs are used in these directories.
///
/// Almost all operations receive a reference to this structure.
#[derive(Debug)]
pub struct XvcRootInner {
    // Moved here from config in repository version 2
    guid: String,
    absolute_path: AbsolutePath,
    xvc_dir: AbsolutePath,
    store_dir: AbsolutePath,
    config: XvcConfig,
    local_config_path: AbsolutePath,
    project_config_path: AbsolutePath,
    entity_generator: XvcEntityGenerator,
}

/// We wrap the XvcRootInner in an Arc to make it easier to share between threads.
pub type XvcRoot = Arc<XvcRootInner>;

impl fmt::Display for XvcRootInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.absolute_path.to_string_lossy())
    }
}

impl Deref for XvcRootInner {
    type Target = AbsolutePath;

    fn deref(&self) -> &Self::Target {
        &self.absolute_path
    }
}

/// Create a new XvcRoot object from the `path`.
/// Configuration is loaded according to [`config_opts`][XvcConfigInitParams].
pub fn load_xvc_root(config_opts: XvcLoadParams) -> Result<XvcRoot> {
    match &config_opts.xvc_root_dir {
        Some(absolute_path) => Ok(Arc::new(XvcRootInner::new(
            absolute_path.clone(),
            config_opts,
        )?)),

        None => Err(Error::CannotFindXvcRoot {
            path: config_opts.current_dir.to_path_buf(),
        }),
    }
}

/// Creates a new .xvc dir in `path` and initializes a directory.
/// *Warning:* This should only be used in `xvc init`, not in other commands.
pub fn init_xvc_root(
    path: &Path,
    config_opts: XvcLoadParams,
    initial_user_config: &XvcOptionalConfiguration,
) -> Result<XvcRoot> {
    let default_config = default_config();
    let initial_config = merge_configs(&default_config, initial_user_config);

    match find_root(path) {
        Ok(abs_path) => Err(Error::CannotNestXvcRepositories {
            path: abs_path.to_path_buf(),
        }),
        Err(e) => {
            if matches!(e, Error::CannotFindXvcRoot { .. }) {
                let abs_path = AbsolutePath::from(path);
                watch!(abs_path);
                let xvc_dir = abs_path.join(XvcRootInner::XVC_DIR);
                watch!(xvc_dir);
                fs::create_dir(&xvc_dir)?;
                // Create GUID
                let uuid = uuid::Uuid::new_v4();
                let guid = hex::encode(seahash::hash(uuid.as_bytes()).to_le_bytes());
                let guid_path = xvc_dir.join(GUID_FILENAME);
                fs::write(&guid_path, guid)?;

                let project_config_path = xvc_dir.join(XvcRootInner::PROJECT_CONFIG_PATH);
                let initial_configuration_file_content =
                    initial_xvc_configuration_file(&initial_config)?;
                fs::write(&project_config_path, &initial_configuration_file_content)?;
                watch!(&project_config_path);

                let local_config_path = xvc_dir.join(XvcRootInner::LOCAL_CONFIG_PATH);
                fs::write(
                    &local_config_path,
                    "# Please add your local config here. This file is .gitignored",
                )?;
                watch!(&local_config_path);

                let project_config_opts = XvcLoadParams {
                    xvc_root_dir: Some(abs_path.clone()),
                    project_config_path: Some(project_config_path),
                    local_config_path: Some(local_config_path),
                    ..config_opts
                };

                let config = XvcConfig::new_v2(&project_config_opts)?;
                watch!(&config);
                // We write the initial entity value directly, without init_entity_generator,
                // because we can't initialize the generator more than once, and we'll read
                // from this value below
                let entity_generator_dir = &xvc_dir.join(XvcRootInner::ENTITY_GENERATOR_PATH);
                watch!(entity_generator_dir);
                fs::create_dir_all(entity_generator_dir)?;
                let entity_generator_path = entity_generator_dir.join(timestamp());
                watch!(entity_generator_path);
                fs::write(entity_generator_path, "1")?;
                let store_dir = xvc_dir.join(XvcRootInner::STORE_DIR);

                watch!(&store_dir);
                fs::create_dir(store_dir)?;
                // TODO: Add crate specific initializations

                let xvcignore_path = abs_path.join(XVCIGNORE_FILENAME);
                fs::write(xvcignore_path, XVCIGNORE_INITIAL_CONTENT)?;

                let use_git = initial_config.git.use_git;
                dbg!(use_git);

                if use_git {
                    let gitignore_path = abs_path.join(PathBuf::from(".gitignore"));
                    let mut out = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(gitignore_path)?;
                    writeln!(out, "{}", GITIGNORE_INITIAL_CONTENT)?;
                }
                load_xvc_root(project_config_opts)
            } else {
                Err(e)
            }
        }
    }
}

impl XvcRootInner {
    /// Create a new XvcRootInner object by reading the configuration from `absolute_path/.xvc/` or
    /// other locations. `config_opts` can determine which configuration files to read
    pub fn new(absolute_path: AbsolutePath, config_opts: XvcLoadParams) -> Result<Self> {
        let xvc_dir = absolute_path.join(XvcRootInner::XVC_DIR);
        let local_config_path = xvc_dir.join(XvcRootInner::LOCAL_CONFIG_PATH);
        let project_config_path = xvc_dir.join(XvcRootInner::PROJECT_CONFIG_PATH);
        let config_opts = XvcLoadParams {
            xvc_root_dir: Some(absolute_path.clone()),
            project_config_path: Some(project_config_path.clone()),
            local_config_path: Some(local_config_path.clone()),
            ..config_opts
        };
        let config = XvcConfig::new_v2(&config_opts)?;
        // TODO: Update to new configurations from earliers here
        let guid = fs::read_to_string(&xvc_dir.join(GUID_FILENAME))?;
        let entity_generator =
            xvc_ecs::load_generator(&xvc_dir.join(XvcRootInner::ENTITY_GENERATOR_PATH))?;

        let store_dir = xvc_dir.join(XvcRootInner::STORE_DIR);
        Ok(Self {
            xvc_dir,
            guid,
            store_dir,
            local_config_path,
            project_config_path,
            absolute_path,
            config,
            entity_generator,
        })
    }

    /// Join `path` to the repository root and return the absolute path of the
    /// given path.
    ///
    /// Uses [AbsolutePath::join] internally.
    pub fn join(&self, path: &Path) -> AbsolutePath {
        self.absolute_path.join(path)
    }

    /// Get the [XvcEntityGenerator] for this repository.
    /// This is used to generate new entity IDs.
    /// There can only be one entity generator per process.
    pub fn entity_generator(&self) -> &XvcEntityGenerator {
        &self.entity_generator
    }

    /// Get the next entity ID from the entity generator.
    pub fn new_entity(&self) -> XvcEntity {
        self.entity_generator.next_element()
    }

    /// Get the absolute path of the repository root.
    pub fn absolute_path(&self) -> &AbsolutePath {
        &self.absolute_path
    }

    /// Guid for this repository
    pub fn guid(&self) -> &str {
        &self.guid
    }

    /// Get the absolute path to the .xvc directory.
    pub fn xvc_dir(&self) -> &AbsolutePath {
        &self.xvc_dir
    }

    /// Get the configuration for this repository.
    pub fn config(&self) -> &XvcConfiguration {
        &self.config.config()
    }

    /// The current directory we run the commands
    pub fn current_dir(&self) -> &AbsolutePath {
        &self.config.current_dir
    }

    /// Get the absolute path to the local config file.
    /// This is the file that is used to store (gitignored) project configuration.
    pub fn local_config_path(&self) -> &AbsolutePath {
        &self.local_config_path
    }

    /// Get the absolute path to the project config file.
    /// This is the file used to store (git tracked) project configuration.
    pub fn project_config_path(&self) -> &AbsolutePath {
        &self.project_config_path
    }

    /// Get the absolute path to the store directory.
    pub fn store_dir(&self) -> &AbsolutePath {
        &self.store_dir
    }

    /// The path of the entity generator directory.
    /// Normally it's in `.xvc/ec/` and can be configured using [Self::ENTITY_GENERATOR_PATH].
    fn entity_generator_path(&self) -> AbsolutePath {
        self.xvc_dir().join(Self::ENTITY_GENERATOR_PATH)
    }

    /// The name for the repository metadata directory.
    pub const XVC_DIR: &'static str = ".xvc";

    /// The file name for the git-ignored configuration.
    const LOCAL_CONFIG_PATH: &'static str = "config.local.toml";

    /// The file name for the git-tracked configuration.
    const PROJECT_CONFIG_PATH: &'static str = "config.toml";

    /// The directory name for the entity generator.
    const ENTITY_GENERATOR_PATH: &'static str = "ec";

    /// The directory name for the store.
    pub const STORE_DIR: &'static str = "store";

    /// Record the entity generator to the disk
    pub fn record(&self) {
        match self.entity_generator.save(&self.entity_generator_path()) {
            Ok(_) => (),
            Err(e) => {
                e.warn();
            }
        }
    }
}

/// Finds the root of the xvc repository by looking for the .xvc directory
/// in parents of a given path.
pub fn find_root(path: &Path) -> Result<AbsolutePath> {
    let abs_path = PathBuf::from(path)
        .canonicalize()
        .expect("Cannot canonicalize the path. Possible symlink loop.");

    watch!(abs_path);

    for parent in abs_path.ancestors() {
        if parent.join(XVC_DIR).is_dir() {
            watch!(parent);
            return Ok(parent.into());
        }
    }
    Err(Error::CannotFindXvcRoot { path: path.into() })
}

impl Drop for XvcRootInner {
    /// Saves the entity_generator before dropping
    fn drop(&mut self) {
        self.record()
    }
}
