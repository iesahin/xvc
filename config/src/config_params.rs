use xvc_walker::AbsolutePath;

/// How should we initialize the configuration?
///
/// It's possible to ignore certain sources by supplying `None` to their values here.
#[derive(Debug, Clone)]
pub struct XvcLoadParams {
    /// The directory where the application runs.
    /// This can be set by various Options.
    /// It affects how paths are handled in general.
    pub current_dir: AbsolutePath,

    /// This is Some(dir) if we run in an Xvc directory
    /// Guid will be read from here in version 2 onwards
    pub xvc_root_dir: Option<AbsolutePath>,

    /// Should we include system configuration?
    /// If `true`, it's read from [SYSTEM_CONFIG_DIRS].
    pub include_system_config: bool,
    /// Should the user's (home) config be included.
    /// If `true`, it's read from [USER_CONFIG_DIRS].
    pub include_user_config: bool,
    /// Should we include the project config at .xvc/config.toml?
    pub include_project_config: bool,

    /// Should we include the local config at .xvc/config-local.toml
    pub include_local_config: bool,

    /// Where should we load the project's (public) configuration?
    /// It's loaded in [XvcRootInner::new]
    pub project_config_path: Option<AbsolutePath>,
    /// Where should we load the project's (private) configuration?
    /// It's loaded in [XvcRootInner::new]
    /// TODO: Add a option to ignore this
    pub local_config_path: Option<AbsolutePath>,
    /// Should we include configuration from the environment.
    /// If `true`, look for all variables in the form
    ///
    /// `XVC_group.key=value`
    ///
    /// from the environment and put them into the configuration.
    pub include_environment_config: bool,
    /// Command line configuration
    pub command_line_config: Option<Vec<String>>,
}

impl XvcLoadParams {
    /// Create a new blank config params
    pub fn new(current_dir: AbsolutePath, xvc_root_dir: Option<AbsolutePath>) -> Self {
        Self {
            current_dir,
            xvc_root_dir,
            include_system_config: true,
            include_user_config: true,
            include_project_config: true,
            include_local_config: true,
            project_config_path: None,
            local_config_path: None,
            include_environment_config: true,
            command_line_config: None,
        }
    }

    /// Update include_system_config value
    pub fn include_system_config(mut self, include_system_config: bool) -> Self {
        self.include_system_config = include_system_config;
        self
    }

    /// Update include_user_config value
    pub fn include_user_config(mut self, include_user_config: bool) -> Self {
        self.include_user_config = include_user_config;
        self
    }

    /// Update project config path
    pub fn project_config_path(mut self, project_config_path: Option<AbsolutePath>) -> Self {
        self.project_config_path = project_config_path;
        self
    }

    /// Update local config path
    pub fn local_config_path(mut self, local_config_path: Option<AbsolutePath>) -> Self {
        self.local_config_path = local_config_path;
        self
    }

    /// Whether to include enviroment variables in the configuration
    pub fn include_environment_config(mut self, include_environment_config: bool) -> Self {
        self.include_environment_config = include_environment_config;
        self
    }

    /// Command line config from key=value definitions
    pub fn command_line_config(mut self, command_line_config: Option<Vec<String>>) -> Self {
        self.command_line_config = command_line_config;
        self
    }
}
