use xvc_walker::AbsolutePath;

/// How should we initialize the configuration?
///
/// It's possible to ignore certain sources by supplying `None` to their values here.
#[derive(Debug, Clone)]
pub struct XvcConfigParams {
    /// The default configuration for the project.
    /// It should contain all default values as a TOML document.
    /// Xvc produces this in [xvc_core::default_configuration].
    pub default_configuration: String,
    /// The directory where the application runs.
    /// This can be set by various Options.
    /// It affects how paths are handled in general.
    pub current_dir: AbsolutePath,
    /// Should we include system configuration?
    /// If `true`, it's read from [SYSTEM_CONFIG_DIRS].
    pub include_system_config: bool,
    /// Should the user's (home) config be included.
    /// If `true`, it's read from [USER_CONFIG_DIRS].
    pub include_user_config: bool,
    /// Where should we load the project's (public) configuration?
    /// It's loaded in [XvcRootInner::new]
    /// TODO: Add a option to ignore this
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

impl XvcConfigParams {
    /// Create a new blank config params
    pub fn new(default_configuration: String, current_dir: AbsolutePath) -> Self {
        Self {
            default_configuration,
            current_dir,
            include_system_config: true,
            include_user_config: true,
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
