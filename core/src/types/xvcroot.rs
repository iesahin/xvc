use log::info;
use log::trace;
use path_absolutize::Absolutize;
use std::fmt;
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use xvc_ecs::ecs::timestamp;
use xvc_ecs::{XvcEntity, XvcEntityGenerator};
use xvc_walker::AbsolutePath;

use xvc_config::{XvcConfig, XvcConfigInitParams};

use crate::error::{Error, Result};
use crate::GITIGNORE_INITIAL_CONTENT;
use crate::XVCIGNORE_FILENAME;
use crate::XVCIGNORE_INITIAL_CONTENT;
use crate::XVC_DIR;

/// Location of a path
#[derive(Debug)]
pub enum MetadataFileLocation {
    Root(String),
    DotXvc(String),
    Store(String),
    FileCache(String),
}

#[derive(Debug)]
pub struct XvcRoot {
    absolute_path: AbsolutePath,
    xvc_dir: PathBuf,
    store_dir: PathBuf,
    config: XvcConfig,
    local_config_path: PathBuf,
    project_config_path: PathBuf,
    entity_generator: XvcEntityGenerator,
    absolute_file_cache_path: AbsolutePath,
}

impl fmt::Display for XvcRoot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.absolute_path.to_string_lossy())
    }
}

impl Deref for XvcRoot {
    type Target = AbsolutePath;

    fn deref(&self) -> &Self::Target {
        &self.absolute_path
    }
}

impl XvcRoot {
    pub fn new(path: &Path, config_opts: XvcConfigInitParams) -> Result<XvcRoot> {
        match XvcRoot::find_root(path) {
            Ok(absolute_path) => {
                let xvc_dir = absolute_path.join(Self::XVC_DIR);
                let local_config_path = xvc_dir.join(Self::LOCAL_CONFIG_PATH);
                let project_config_path = xvc_dir.join(Self::PROJECT_CONFIG_PATH);
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
                    xvc_ecs::load_generator(&xvc_dir.join(Self::ENTITY_GENERATOR_PATH))?;
                let file_cache_dir = PathBuf::from(config.get_str("cache.path")?.option);
                let absolute_file_cache_path = if file_cache_dir.is_absolute() {
                    file_cache_dir.into()
                } else {
                    xvc_dir
                        .join(file_cache_dir)
                        .absolutize()?
                        .to_path_buf()
                        .into()
                };

                let store_dir = xvc_dir.join(Self::STORE_PATH);
                let xvc_root = XvcRoot {
                    xvc_dir,
                    store_dir,
                    local_config_path,
                    project_config_path,
                    absolute_path,
                    config,
                    entity_generator,
                    absolute_file_cache_path,
                };
                Ok(xvc_root)
            }
            Err(e) => Err(e),
        }
    }

    /// Creates a new .xvc dir in `path` and initializes a directory.
    /// *Warning:* This should only be used in `xvc init`, not in other commands.
    pub fn init(path: &Path, config_opts: XvcConfigInitParams) -> Result<XvcRoot> {
        match XvcRoot::find_root(path) {
            Ok(abs_path) => Err(Error::CannotNestXvcRepositories {
                path: abs_path.to_path_buf(),
            }),
            Err(e) => {
                if matches!(e, Error::CannotFindXvcRoot { .. }) {
                    let path = PathBuf::from(path).canonicalize()?;
                    let xvc_dir = path.join(Self::XVC_DIR);
                    fs::create_dir(&xvc_dir)?;
                    let initial_config = config_opts.default_configuration.clone();
                    let project_config_path = xvc_dir.join(Self::PROJECT_CONFIG_PATH);
                    fs::write(&project_config_path, initial_config)?;
                    let local_config_path = xvc_dir.join(Self::LOCAL_CONFIG_PATH);
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
                    let entity_generator_dir = &xvc_dir.join(Self::ENTITY_GENERATOR_PATH);
                    fs::create_dir_all(entity_generator_dir)?;
                    let entity_generator_path = entity_generator_dir.join(timestamp());
                    fs::write(&entity_generator_path, "1")?;
                    let store_dir = xvc_dir.join(Self::STORE_PATH);
                    fs::create_dir(&store_dir)?;
                    // TODO: Add crate specific initializations

                    let xvcignore_path = path.join(XVCIGNORE_FILENAME);
                    fs::write(xvcignore_path, XVCIGNORE_INITIAL_CONTENT)?;

                    let use_git = config.get_bool("core.use_git")?.option;
                    if use_git {
                        let gitignore_path = path.join(&PathBuf::from(".gitignore"));
                        fs::write(gitignore_path, GITIGNORE_INITIAL_CONTENT)?;
                    }
                    XvcRoot::new(&path, project_config_opts)
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn join(&self, path: &Path) -> PathBuf {
        self.absolute_path.join(path)
    }

    pub fn entity_generator(&self) -> &XvcEntityGenerator {
        &self.entity_generator
    }

    pub fn new_entity(&self) -> XvcEntity {
        self.entity_generator.next_element()
    }

    pub fn absolute_path(&self) -> &AbsolutePath {
        &self.absolute_path
    }

    pub fn xvc_dir(&self) -> &PathBuf {
        &self.xvc_dir
    }
    pub fn store_dir(&self) -> &PathBuf {
        &self.store_dir
    }
    pub fn config(&self) -> &XvcConfig {
        &self.config
    }
    pub fn local_config_path(&self) -> &PathBuf {
        &self.local_config_path
    }
    pub fn project_config_path(&self) -> &PathBuf {
        &self.project_config_path
    }

    fn entity_generator_path(&self) -> PathBuf {
        self.xvc_dir().join(Self::ENTITY_GENERATOR_PATH)
    }

    pub fn get_metafile_path(&self, mf: &MetadataFileLocation) -> PathBuf {
        match mf {
            MetadataFileLocation::Store(s) => self.store_dir.join(s),
            MetadataFileLocation::Root(s) => self.absolute_path.join(s),
            MetadataFileLocation::DotXvc(s) => self.xvc_dir.join(s),
            MetadataFileLocation::FileCache(s) => self.absolute_file_cache_path.join(s),
        }
    }

    const XVC_DIR: &'static str = ".xvc";
    const LOCAL_CONFIG_PATH: &'static str = "config.local.toml";
    const PROJECT_CONFIG_PATH: &'static str = "config.toml";
    const STORE_PATH: &'static str = "store";
    const ENTITY_GENERATOR_PATH: &'static str = "ec";

    fn find_root(path: &Path) -> Result<AbsolutePath> {
        trace!("{:?}", path);
        let mut pb = PathBuf::from(path)
            .canonicalize()
            .expect("Cannot canonicalize the path. Possible symlink loop.");
        loop {
            if pb.join(XVC_DIR).is_dir() {
                info!("XVC DIR: {:?}", pb);
                return Ok(pb.into());
            } else if pb.parent() == None {
                return Err(Error::CannotFindXvcRoot { path: path.into() });
            } else {
                pb.pop();
            }
        }
    }
}

impl Drop for XvcRoot {
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
