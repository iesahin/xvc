//! Main CLI interface for Xvc
use std::env::ArgsOs;

use std::ffi::OsString;
use std::path::PathBuf;
use std::str::FromStr;

use crate::comp;
use crate::init;
use crate::XvcRootOpt;

use xvc_core::git_checkout_ref;
use xvc_core::handle_git_automation;

use clap::Parser;

use clap_complete::engine::ArgValueCompleter;

use crossbeam::thread;
use crossbeam_channel::bounded;
use log::LevelFilter;
use std::io;
use xvc_core::types::xvcroot::find_root;
use xvc_core::types::xvcroot::load_xvc_root;
use xvc_core::util::completer::git_branch_completer;
use xvc_core::util::completer::git_reference_completer;
use xvc_core::XvcOutputSender;
use xvc_core::{debug, error, uwr, XvcOutputLine};

use xvc_core::{XvcConfigParams, XvcVerbosity};
use xvc_core::check_ignore;
pub use xvc_core::initial_project_config;
use xvc_core::root;
use xvc_core::CHANNEL_BOUND;
use xvc_file as file;
use xvc_core::setup_logging;
use xvc_pipeline as pipeline;
use xvc_storage as storage;
use xvc_core::AbsolutePath;

use crate::cli;
use crate::error::{Error, Result};

use git_version::git_version;
const GIT_VERSION: &str = git_version!(
    args = ["--always", "--dirty=modified", "--tags"],
    cargo_prefix = "",
    fallback = "unknown"
);

/// Xvc CLI to manage data and ML pipelines
#[derive(Debug, Parser, Clone)]
#[command(
    rename_all = "kebab-case",
    author,
    version = GIT_VERSION
)]
pub struct XvcCLI {
    /// Output verbosity. Use multiple times to increase the output detail.
    #[arg(long = "verbose", short, action = clap::ArgAction::Count)]
    pub verbosity: u8,

    /// Suppress all output.
    #[arg(long)]
    pub quiet: bool,

    /// Turn on all logging to $TMPDIR/xvc.log
    #[arg(long)]
    pub debug: bool,

    /// Set working directory for the command.
    /// It doesn't create a new shell, or change the directory.
    #[arg(short = 'C', default_value = ".")]
    pub workdir: PathBuf,

    /// Configuration options set from the command line in the form section.key=value
    /// You can use multiple times.
    #[arg(long, short = 'c')]
    config: Option<Vec<String>>,

    /// Ignore system configuration file.
    #[arg(long)]
    pub no_system_config: bool,

    /// Ignore user configuration file.
    #[arg(long)]
    pub no_user_config: bool,

    /// Ignore project configuration file (.xvc/config)
    #[arg(long)]
    pub no_project_config: bool,

    /// Ignore local (gitignored) configuration file (.xvc/config.local)
    #[arg(long)]
    pub no_local_config: bool,

    /// Ignore configuration options obtained from environment variables.
    #[arg(long)]
    pub no_env_config: bool,

    /// Don't run automated Git operations for this command.
    /// If you want to run git commands yourself all the time, you can set `git.auto_commit` and
    /// `git.auto_stage` options in the configuration to False.
    #[arg(long)]
    pub skip_git: bool,

    /// Checkout the given Git reference (branch, tag, commit etc.) before performing the Xvc
    /// operation.
    /// This runs `git checkout <given-value>` before running the command.
    #[arg(
        long,
        conflicts_with("skip_git"),
        add = ArgValueCompleter::new(git_reference_completer))]
    pub from_ref: Option<String>,

    /// If given, create (or checkout) the given branch before committing results of the operation.
    /// This runs `git checkout --branch <given-value>` before committing the changes.
    #[arg(long, conflicts_with("skip_git"), add=ArgValueCompleter::new(git_branch_completer))]
    pub to_branch: Option<String>,

    /// The subcommand to run
    #[command(subcommand)]
    pub command: XvcSubCommand,

    /// The calling command for logging and documentation purposes
    #[arg(skip)]
    pub command_string: String,
}

impl XvcCLI {
    /// Parse the given elements with [clap::Parser::parse_from] and merge them to set
    /// [XvcCLI::command_string].
    pub fn from_str_slice(args: &[&str]) -> Result<Self> {
        let command_string = args.join(" ");
        let parsed = Self::parse_from(args);
        Ok(Self {
            command_string,
            ..parsed
        })
    }

    /// Parse the given elements with [clap::Parser::parse_from] and merge them to set
    /// [XvcCLI::command_string].
    pub fn from_string_slice(args: &[String]) -> Result<Self> {
        let command_string = args.join(" ");
        let parsed = Self::parse_from(args);
        Ok(Self {
            command_string,
            ..parsed
        })
    }

    /// Parse the command line from the result of [`std::env::args_os`].
    /// This updates [XvcCLI::command_string] with the command line.
    pub fn from_args_os(args_os: ArgsOs) -> Result<Self> {
        let args: Vec<OsString> = args_os.collect();
        let args: Vec<String> = args
            .iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect();
        let command_string = args.join(" ");
        let parsed = Self::parse_from(args);
        Ok(Self {
            command_string,
            ..parsed
        })
    }

    /// Collects cli config with -c options along with direct options (like verbosity) to provide
    /// to XvcConfig constructor.
    pub fn consolidate_config_options(&self) -> Vec<String> {
        let mut output = self.config.clone().unwrap_or_default();
        output.push(format!(
            "core.verbosity = {}",
            XvcVerbosity::from(self.verbosity)
        ));
        output.push(format!("core.quiet = {}", self.quiet));

        output
    }
}

// Implement FromStr for XvcCLI

impl FromStr for XvcCLI {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let command_string = s.to_owned();
        let args: Vec<String> = s.split(' ').map(|a| a.trim().to_owned()).collect();
        let parsed = Self::parse_from(args);
        Ok(Self {
            command_string,
            ..parsed
        })
    }
}

/// Xvc subcommands
#[derive(Debug, Parser, Clone)]
#[command(rename_all = "kebab-case")]
pub enum XvcSubCommand {
    /// File and directory management commands
    #[command(visible_aliases=&["f"])]
    File(xvc_file::XvcFileCLI),

    /// Pipeline management commands
    #[command(visible_aliases=&["p"])]
    Pipeline(xvc_pipeline::PipelineCLI),

    /// Storage (cloud) management commands
    #[command(visible_aliases=&["s"])]
    Storage(xvc_storage::StorageCLI),

    /// Find the root directory of a project
    #[command(visible_aliases=&["r"])]
    Root(xvc_core::root::RootCLI),

    /// Initialize an Xvc project
    #[command()]
    Init(crate::init::InitCLI),

    /// Check whether files are ignored with `.xvcignore`
    #[command()]
    CheckIgnore(xvc_core::check_ignore::CheckIgnoreCLI),

    /// Completion Helpers
    #[command(name = "_comp")]
    _Comp(crate::comp::CompCLI),
}

/// Runs the supplied xvc command.
pub fn run(args: &[&str]) -> Result<XvcRootOpt> {
    let cli_options = cli::XvcCLI::from_str_slice(args)?;
    dispatch(cli_options)
}

/// Run the supplied command within the optional [XvcRoot]. If xvc_root is None, it will be tried
/// to be loaded from `cli_opts.workdir`.
pub fn dispatch_with_root(cli_opts: cli::XvcCLI, xvc_root_opt: XvcRootOpt) -> Result<XvcRootOpt> {
    // XvcRoot should be kept per repository and shouldn't change directory across runs
    assert!(
        xvc_root_opt.as_ref().is_none()
            || xvc_root_opt
                .as_ref()
                .map(|xvc_root| find_root(&cli_opts.workdir).unwrap() == *xvc_root.absolute_path())
                .unwrap()
    );

    let term_log_level = get_term_log_level(get_verbosity(&cli_opts));

    let xvc_root_opt = thread::scope(move |s| {
        let (output_snd, output_rec) = bounded::<Option<XvcOutputLine>>(CHANNEL_BOUND);

        let output_snd_clone = output_snd.clone();

        let output_thread = s.spawn(move |_| {
            while let Ok(Some(output_line)) = output_rec.recv() {
                // output_str.push_str(&output_line);
                match term_log_level {
                    LevelFilter::Off => match output_line {
                        XvcOutputLine::Output(_) => {}
                        XvcOutputLine::Info(_) => {}
                        XvcOutputLine::Warn(_) => {}
                        XvcOutputLine::Error(_) => {}
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Error => match output_line {
                        XvcOutputLine::Output(m) => println!("{m}"),
                        XvcOutputLine::Info(_) => {}
                        XvcOutputLine::Warn(_) => {}
                        XvcOutputLine::Error(m) => eprintln!("[ERROR] {}", m),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Warn => match output_line {
                        XvcOutputLine::Output(m) => println!("{m}"),
                        XvcOutputLine::Warn(m) => eprintln!("[WARN] {}", m),
                        XvcOutputLine::Error(m) => eprintln!("[ERROR] {}", m),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m),
                        XvcOutputLine::Info(_) => {}
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Info => match output_line {
                        XvcOutputLine::Output(m) => println!("{m}"),
                        XvcOutputLine::Info(m) => eprintln!("[INFO] {}", m),
                        XvcOutputLine::Warn(m) => eprintln!("[WARN] {}", m),
                        XvcOutputLine::Error(m) => eprintln!("[ERROR] {}", m),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Debug => match output_line {
                        XvcOutputLine::Output(m) => println!("{m}"),
                        XvcOutputLine::Info(m) => eprintln!("[INFO] {}", m),
                        XvcOutputLine::Warn(m) => eprintln!("[WARN] {}", m),
                        XvcOutputLine::Error(m) => eprintln!("[ERROR] {}", m),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(m) => eprintln!("[DEBUG] {}", m),
                    },
                    LevelFilter::Trace => match output_line {
                        XvcOutputLine::Output(m) => println!("{m}"),
                        XvcOutputLine::Info(m) => eprintln!("[INFO] {}", m),
                        XvcOutputLine::Warn(m) => eprintln!("[WARN] {}", m),
                        XvcOutputLine::Error(m) => eprintln!("[ERROR] {}", m),
                        XvcOutputLine::Debug(m) => eprintln!("[DEBUG] {}", m),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m),
                        XvcOutputLine::Tick(_) => todo!(),
                    },
                }
            }
        });

        if let Some(ref xvc_root) = xvc_root_opt {
            if let Some(ref from_ref) = cli_opts.from_ref {
                uwr!(
                    git_checkout_ref(&output_snd, xvc_root, from_ref),
                    output_snd
                );
            }
        }
        let xvc_root_opt_res = s.spawn(move |_| -> Result<XvcRootOpt> {
            let xvc_root_opt = command_matcher(cli_opts.clone(), xvc_root_opt, &output_snd)?;

            match xvc_root_opt {
                Some(ref xvc_root) => {
                    xvc_root.record();
                    if cli_opts.skip_git {
                        debug!(output_snd, "Skipping Git operations");
                    } else {
                        handle_git_automation(
                            &output_snd,
                            xvc_root,
                            cli_opts.to_branch.as_deref(),
                            &cli_opts.command_string,
                        )?;
                    }
                }
                None => {
                    debug!(
                        output_snd,
                        "Xvc is outside of a project, no need to handle Git operations."
                    );
                }
            }
            Ok(xvc_root_opt)
        });

        let xvc_root_opt = xvc_root_opt_res.join().unwrap();
        match &xvc_root_opt {
            Ok(_) => debug!(output_snd_clone, "Command completed successfully."),
            Err(e) => error!(output_snd_clone, "{}", e),
        }
        output_snd_clone.send(None).unwrap();
        output_thread.join().unwrap();

        xvc_root_opt
    })
    .unwrap();

    xvc_root_opt
}

/// Dispatch commands to respective functions in the API
///
/// It sets output verbosity with [XvcCLI::verbosity].
/// Determines configuration sources by filling [XvcConfigInitParams].
/// Tries to create an XvcRoot to determine whether we're inside one.
/// It calls [dispatch_with_root] with an optional root.
///
/// A corresponding function to reuse the same [XvcRoot] object is [test_dispatch].
/// It doesn't recreate the whole configuration and this prevents errors regarding multiple
/// initializations.
pub fn dispatch(cli_opts: cli::XvcCLI) -> Result<XvcRootOpt> {
    let verbosity = get_verbosity(&cli_opts);

    let term_log_level = get_term_log_level(verbosity);

    setup_logging(
        Some(term_log_level),
        if cli_opts.debug {
            Some(LevelFilter::Trace)
        } else {
            None
        },
    );

    let xvc_config_params = get_xvc_config_params(&cli_opts);

    let xvc_root_opt = match load_xvc_root(xvc_config_params) {
        Ok(r) => Some(r),
        Err(e) => {
            e.debug();
            None
        }
    };

    dispatch_with_root(cli_opts, xvc_root_opt)
}

/// Decide configuration sources  from CLI options
pub fn get_xvc_config_params(cli_opts: &XvcCLI) -> XvcConfigParams {
    XvcConfigParams {
        current_dir: AbsolutePath::from(&cli_opts.workdir),
        include_system_config: !cli_opts.no_system_config,
        include_user_config: !cli_opts.no_user_config,
        project_config_path: None,
        local_config_path: None,
        include_environment_config: !cli_opts.no_env_config,
        command_line_config: Some(cli_opts.consolidate_config_options()),
        default_configuration: initial_project_config(true),
    }
}

/// Convert verbosity to log level
pub fn get_term_log_level(verbosity: XvcVerbosity) -> LevelFilter {
    match verbosity {
        XvcVerbosity::Quiet => LevelFilter::Off,
        XvcVerbosity::Default => LevelFilter::Error,
        XvcVerbosity::Warn => LevelFilter::Warn,
        XvcVerbosity::Info => LevelFilter::Info,
        XvcVerbosity::Debug => LevelFilter::Debug,
        XvcVerbosity::Trace => LevelFilter::Trace,
    }
}

/// Convert verbosity value to XvcVerbosity
pub fn get_verbosity(cli_opts: &XvcCLI) -> XvcVerbosity {
    if cli_opts.quiet {
        XvcVerbosity::Quiet
    } else {
        match cli_opts.verbosity {
            0 => XvcVerbosity::Default,
            1 => XvcVerbosity::Warn,
            2 => XvcVerbosity::Info,
            3 => XvcVerbosity::Debug,
            _ => XvcVerbosity::Trace,
        }
    }
}

/// Collect all output from the channel in a string and return
// FIXME: Maybe move to xvc-logging
pub fn collect_output(
    output_rcv: &crossbeam_channel::Receiver<Option<XvcOutputLine>>,
    term_log_level: LevelFilter,
) -> String {
    let mut output_str = String::new();
    while let Ok(Some(output_line)) = output_rcv.recv() {
        // output_str.push_str(&output_line);
        match term_log_level {
            LevelFilter::Off => match output_line {
                XvcOutputLine::Output(_) => {}
                XvcOutputLine::Info(_) => {}
                XvcOutputLine::Warn(_) => {}
                XvcOutputLine::Error(_) => {}
                XvcOutputLine::Panic(m) => output_str.push_str(&format!("[PANIC] {}", m)),
                XvcOutputLine::Tick(_) => todo!(),
                XvcOutputLine::Debug(_) => {}
            },
            LevelFilter::Error => match output_line {
                XvcOutputLine::Output(m) => output_str.push_str(&m),
                XvcOutputLine::Info(_) => {}
                XvcOutputLine::Warn(_) => {}
                XvcOutputLine::Error(m) => output_str.push_str(&format!("[ERROR] {}", m)),
                XvcOutputLine::Panic(m) => output_str.push_str(&format!("[PANIC] {}", m)),
                XvcOutputLine::Tick(_) => todo!(),
                XvcOutputLine::Debug(_) => {}
            },
            LevelFilter::Warn => match output_line {
                XvcOutputLine::Output(m) => output_str.push_str(&m),
                XvcOutputLine::Warn(m) => output_str.push_str(&format!("[WARN] {}", m)),
                XvcOutputLine::Error(m) => output_str.push_str(&format!("[ERROR] {}", m)),
                XvcOutputLine::Panic(m) => output_str.push_str(&format!("[PANIC] {}", m)),
                XvcOutputLine::Info(_) => {}
                XvcOutputLine::Tick(_) => todo!(),
                XvcOutputLine::Debug(_) => {}
            },
            LevelFilter::Info => match output_line {
                XvcOutputLine::Output(m) => output_str.push_str(&m),
                XvcOutputLine::Info(m) => output_str.push_str(&format!("[INFO] {}", m)),
                XvcOutputLine::Warn(m) => output_str.push_str(&format!("[WARN] {}", m)),
                XvcOutputLine::Error(m) => output_str.push_str(&format!("[ERROR] {}", m)),
                XvcOutputLine::Panic(m) => output_str.push_str(&format!("[PANIC] {}", m)),
                XvcOutputLine::Tick(_) => todo!(),
                XvcOutputLine::Debug(_) => {}
            },
            LevelFilter::Debug => match output_line {
                XvcOutputLine::Output(m) => output_str.push_str(&m),
                XvcOutputLine::Info(m) => output_str.push_str(&format!("[INFO] {}", m)),
                XvcOutputLine::Warn(m) => output_str.push_str(&format!("[WARN] {}", m)),
                XvcOutputLine::Error(m) => output_str.push_str(&format!("[ERROR] {}", m)),
                XvcOutputLine::Panic(m) => output_str.push_str(&format!("[PANIC] {}", m)),
                XvcOutputLine::Debug(m) => output_str.push_str(&format!("[DEBUG] {}", m)),
                XvcOutputLine::Tick(_) => todo!(),
            },
            LevelFilter::Trace => match output_line {
                XvcOutputLine::Output(m) => output_str.push_str(&m),
                XvcOutputLine::Info(m) => output_str.push_str(&format!("[INFO] {}", m)),
                XvcOutputLine::Warn(m) => output_str.push_str(&format!("[WARN] {}", m)),
                XvcOutputLine::Error(m) => output_str.push_str(&format!("[ERROR] {}", m)),
                XvcOutputLine::Debug(m) => output_str.push_str(&format!("[DEBUG] {}", m)),
                XvcOutputLine::Panic(m) => output_str.push_str(&format!("[PANIC] {}", m)),
                XvcOutputLine::Tick(_) => todo!(),
            },
        }
    }
    output_str
}

/// Run the given command and return the modified [XvcRoot]
pub fn command_matcher(
    cli_opts: XvcCLI,
    xvc_root_opt: XvcRootOpt,
    output_snd: &XvcOutputSender,
) -> Result<XvcRootOpt> {
    {
        let res_xvc_root_opt: Result<XvcRootOpt> = match cli_opts.command {
            XvcSubCommand::Init(opts) => {
                let use_git = !opts.no_git;
                let xvc_root = init::run(xvc_root_opt.as_ref(), opts)?;

                if use_git {
                    handle_git_automation(
                        output_snd,
                        &xvc_root,
                        cli_opts.to_branch.as_deref(),
                        &cli_opts.command_string,
                    )?;
                }
                Ok(Some(xvc_root))
            }

            // following commands can only be run inside a repository
            XvcSubCommand::Root(opts) => {
                root::run(
                    output_snd,
                    xvc_root_opt
                        .as_ref()
                        .ok_or_else(|| Error::RequiresXvcRepository)?,
                    opts,
                )?;

                Ok(xvc_root_opt)
            }

            XvcSubCommand::File(opts) => {
                file::run(output_snd, xvc_root_opt.as_ref(), opts)?;
                Ok(xvc_root_opt)
            }

            XvcSubCommand::Pipeline(opts) => {
                // FIXME: We can replace this stdin with another channel
                let stdin = io::stdin();
                let input = stdin.lock();
                pipeline::cmd_pipeline(
                    input,
                    output_snd,
                    xvc_root_opt.as_ref().ok_or(Error::RequiresXvcRepository)?,
                    opts,
                )?;
                Ok(xvc_root_opt)
            }

            XvcSubCommand::CheckIgnore(opts) => {
                // FIXME: We can replace this stdin with another channel
                let stdin = io::stdin();
                let input = stdin.lock();

                check_ignore::cmd_check_ignore(
                    input,
                    output_snd,
                    xvc_root_opt.as_ref().ok_or(Error::RequiresXvcRepository)?,
                    opts,
                )?;

                Ok(xvc_root_opt)
            }

            XvcSubCommand::Storage(opts) => {
                let stdin = io::stdin();
                let input = stdin.lock();
                storage::cmd_storage(
                    input,
                    output_snd,
                    xvc_root_opt.as_ref().ok_or(Error::RequiresXvcRepository)?,
                    opts,
                )?;

                Ok(xvc_root_opt)
            }

            XvcSubCommand::_Comp(comp_cli) => {
                comp::run(comp_cli)?;
                Ok(xvc_root_opt)
            }
        };

        let xvc_root_opt = match res_xvc_root_opt {
            Ok(xvc_root_opt) => xvc_root_opt,
            Err(e) => {
                error!(&output_snd, "{}", e);
                None
            }
        };

        if let Some(ref xvc_root) = xvc_root_opt {
            if !cli_opts.skip_git {
                xvc_root.record();
                handle_git_automation(
                    output_snd,
                    xvc_root,
                    cli_opts.to_branch.as_deref(),
                    &cli_opts.command_string,
                    // FIXME: Handle this error more gracefully
                )
                .unwrap();
            }
        }

        Ok(xvc_root_opt)
    }
}
