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
use xvc_ecs::ecs::timestamp;
use xvc_ecs::{XvcEntity, XvcEntityGenerator};
use xvc_logging::{debug, trace};
use xvc_walker::AbsolutePath;

use xvc_config::{XvcConfig, XvcConfigInitParams};

use crate::error::{Error, Result};
use crate::GITIGNORE_INITIAL_CONTENT;
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
/// The path is not required to be the root of the repository.
/// This function searches for the root of the repository using
/// [XvcRoot::find_root] and uses it as the root.
pub fn load_xvc_root(path: &Path, config_opts: XvcConfigInitParams) -> Result<XvcRoot> {
    match XvcRootInner::find_root(path) {
        Ok(absolute_path) => {
            let xvc_dir = absolute_path.join(XvcRootInner::XVC_DIR);
            let local_config_path = xvc_dir.join(XvcRootInner::LOCAL_CONFIG_PATH);
            let project_config_path = xvc_dir.join(XvcRootInner::PROJECT_CONFIG_PATH);
            let config_opts = XvcConfigInitParams {
                project_config_path: Some(project_config_path.clone()),
                local_config_path: Some(local_config_path.clone()),
                default_configuration: config_opts.default_configuration,
                current_dir: config_opts.current_dir,
                include_system_config: config_opts.include_system_config,
                include_user_config: config_opts.include_user_config,
                include_environment_config: config_opts.include_environment_config,
                command_line_config: config_opts.command_line_config,
            };
            let config = XvcConfig::new(config_opts)?;
            let entity_generator =
                xvc_ecs::load_generator(&xvc_dir.join(XvcRootInner::ENTITY_GENERATOR_PATH))?;

            let store_dir = xvc_dir.join(XvcRootInner::STORE_DIR);
            let xvc_root = Arc::new(XvcRootInner {
                xvc_dir,
                store_dir,
                local_config_path,
                project_config_path,
                absolute_path,
                config,
                entity_generator,
            });
            Ok(xvc_root)
        }
        Err(e) => Err(e),
    }
}

/// Creates a new .xvc dir in `path` and initializes a directory.
/// *Warning:* This should only be used in `xvc init`, not in other commands.
pub fn init_xvc_root(path: &Path, config_opts: XvcConfigInitParams) -> Result<XvcRoot> {
    match XvcRootInner::find_root(path) {
        Ok(abs_path) => Err(Error::CannotNestXvcRepositories {
            path: abs_path.to_path_buf(),
        }),
        Err(e) => {
            if matches!(e, Error::CannotFindXvcRoot { .. }) {
                let abs_path = AbsolutePath::from(path);
                let xvc_dir = abs_path.join(XvcRootInner::XVC_DIR);
                fs::create_dir(&xvc_dir)?;
                let initial_config = config_opts.default_configuration.clone();
                let project_config_path = xvc_dir.join(XvcRootInner::PROJECT_CONFIG_PATH);
                fs::write(&project_config_path, initial_config)?;
                let local_config_path = xvc_dir.join(XvcRootInner::LOCAL_CONFIG_PATH);
                fs::write(
                    &local_config_path,
                    "# Please add your local config here. This file is .gitignored",
                )?;

                let project_config_opts = XvcConfigInitParams {
                    default_configuration: config_opts.default_configuration,
                    current_dir: config_opts.current_dir,
                    include_system_config: config_opts.include_system_config,
                    include_user_config: config_opts.include_user_config,
                    project_config_path: Some(project_config_path),
                    local_config_path: Some(local_config_path),
                    include_environment_config: config_opts.include_environment_config,
                    command_line_config: config_opts.command_line_config,
                };

                let config = XvcConfig::new(project_config_opts.clone())?;
                // We write the initial entity value directly, without init_entity_generator,
                // because we can't initialize the generator more than once, and we'll read
                // from this value below
                let entity_generator_dir = &xvc_dir.join(XvcRootInner::ENTITY_GENERATOR_PATH);
                fs::create_dir_all(entity_generator_dir)?;
                let entity_generator_path = entity_generator_dir.join(timestamp());
                fs::write(entity_generator_path, "1")?;
                let store_dir = xvc_dir.join(XvcRootInner::STORE_DIR);
                fs::create_dir(store_dir)?;
                // TODO: Add crate specific initializations

                let xvcignore_path = abs_path.join(XVCIGNORE_FILENAME);
                fs::write(xvcignore_path, XVCIGNORE_INITIAL_CONTENT)?;

                let use_git = config.get_bool("git.use_git")?.option;
                if use_git {
                    let gitignore_path = abs_path.join(PathBuf::from(".gitignore"));
                    let mut out = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(gitignore_path)?;
                    writeln!(out, "{}", GITIGNORE_INITIAL_CONTENT)?;
                }
                load_xvc_root(&abs_path, project_config_opts)
            } else {
                Err(e)
            }
        }
    }
}

impl XvcRootInner {
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

    /// Get the absolute path to the .xvc directory.
    pub fn xvc_dir(&self) -> &AbsolutePath {
        &self.xvc_dir
    }

    /// Get the configuration for this repository.
    /// The configuration is initialized using [XvcConfigInitParams] in [Self::new].
    pub fn config(&self) -> &XvcConfig {
        &self.config
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
    const XVC_DIR: &'static str = ".xvc";
    /// The file name for the git-ignored configuration.
    const LOCAL_CONFIG_PATH: &'static str = "config.local.toml";
    /// The file name for the git-tracked configuration.
    const PROJECT_CONFIG_PATH: &'static str = "config.toml";
    /// The directory name for the entity generator.
    const ENTITY_GENERATOR_PATH: &'static str = "ec";

    /// The directory name for the store.
    const STORE_DIR: &'static str = "store";

    /// Finds the root of the xvc repository by looking for the .xvc directory
    /// in parents of a given path.
    pub fn find_root(path: &Path) -> Result<AbsolutePath> {
        trace!("{:?}", path);
        let mut pb = PathBuf::from(path)
            .canonicalize()
            .expect("Cannot canonicalize the path. Possible symlink loop.");
        loop {
            if pb.join(XVC_DIR).is_dir() {
                debug!("XVC DIR: {:?}", pb);
                return Ok(pb.into());
            } else if pb.parent().is_none() {
                return Err(Error::CannotFindXvcRoot { path: path.into() });
            } else {
                pb.pop();
            }
        }
    }
}

impl Drop for XvcRootInner {
    /// Saves the entity_generator before dropping
    fn drop(&mut self) {
        match self.entity_generator.save(&self.entity_generator_path()) {
            Ok(_) => (),
            Err(e) => {
                e.warn();
            }
        }
    }
}
