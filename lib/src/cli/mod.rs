//! Main CLI interface for XVC
use std::env::ArgsOs;

use std::ffi::OsString;
use std::path::PathBuf;

use crate::git_checkout_ref;
use crate::handle_git_automation;
use crate::init;

use clap::Parser;
use crossbeam::thread;
use crossbeam_channel::bounded;
use log::LevelFilter;
use std::io;
use xvc_core::types::xvcroot::load_xvc_root;
use xvc_logging::{debug, error, uwr, XvcOutputLine};

use std::path::Path;
use xvc_config::{XvcConfigInitParams, XvcVerbosity};
use xvc_core::aliases;
use xvc_core::check_ignore;
use xvc_core::default_project_config;
use xvc_core::root;
use xvc_core::CHANNEL_BOUND;
use xvc_file as file;
use xvc_logging::setup_logging;
use xvc_logging::watch;
use xvc_pipeline as pipeline;
use xvc_storage as storage;
use xvc_walker::AbsolutePath;

use crate::cli;
use crate::error::{Error, Result};

use git_version::git_version;
const GIT_VERSION: &str = git_version!(prefix = "git:", cargo_prefix = "");

/// Xvc CLI to manage data and ML pipelines
#[derive(Debug, Parser)]
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
    #[arg(long, conflicts_with("skip_git"))]
    pub from_ref: Option<String>,

    /// If given, create (or checkout) the given branch before committing results of the operation.
    /// This runs `git checkout --branch <given-value>` before committing the changes.
    #[arg(long, conflicts_with("skip_git"))]
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
    pub fn from_str_slice(args: &[&str]) -> Result<XvcCLI> {
        let command_string = args.join(" ");
        let parsed = Self::parse_from(args);
        Ok(Self {
            command_string,
            ..parsed
        })
    }

    /// Parse the command line from the result of [`std::env::args_os`].
    /// This updates [XvcCLI::command_string] with the command line.
    pub fn from_args_os(args_os: ArgsOs) -> Result<XvcCLI> {
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

/// Xvc subcommands
#[derive(Debug, Parser)]
#[command(rename_all = "kebab-case")]
pub enum XvcSubCommand {
    /// File and directory management commands
    File(xvc_file::XvcFileCLI),
    /// Initialize an Xvc project
    Init(crate::init::InitCLI),
    /// Pipeline management commands
    Pipeline(xvc_pipeline::PipelineCLI),
    /// Storage (cloud) management commands
    Storage(xvc_storage::StorageCLI),
    /// Find the root directory of a project
    Root(xvc_core::root::RootCLI),
    /// Check whether files are ignored with `.xvcignore`
    CheckIgnore(xvc_core::check_ignore::CheckIgnoreCLI),
    /// Print command aliases to be sourced in shell files
    Aliases(xvc_core::aliases::AliasesCLI),
}

/// Runs the supplied xvc command.
pub fn run(args: &[&str]) -> Result<()> {
    let cli_options = cli::XvcCLI::from_str_slice(args)?;
    dispatch(cli_options)
}

/// Dispatch commands to respective functions in the API
///
/// It sets output verbosity with [XvcCLI::verbosity].
/// Determines configuration sources by filling [XvcConfigInitParams].
/// Tries to create an XvcRoot to determine whether we're inside one.
/// Creates two threads: One for running the API function, one for getting strings from output
/// channel.
///
/// A corresponding function to reuse the same [XvcRoot] object is [test_dispatch].
/// It doesn't recreate the whole configuration and this prevents errors regarding multiple
/// initializations.
pub fn dispatch(cli_opts: cli::XvcCLI) -> Result<()> {
    let verbosity = if cli_opts.quiet {
        XvcVerbosity::Quiet
    } else {
        match cli_opts.verbosity {
            0 => XvcVerbosity::Default,
            1 => XvcVerbosity::Warn,
            2 => XvcVerbosity::Info,
            3 => XvcVerbosity::Debug,
            _ => XvcVerbosity::Trace,
        }
    };

    let term_log_level = match verbosity {
        XvcVerbosity::Quiet => LevelFilter::Off,
        XvcVerbosity::Default => LevelFilter::Error,
        XvcVerbosity::Warn => LevelFilter::Warn,
        XvcVerbosity::Info => LevelFilter::Info,
        XvcVerbosity::Debug => LevelFilter::Debug,
        XvcVerbosity::Trace => LevelFilter::Trace,
    };

    setup_logging(
        Some(term_log_level),
        if cli_opts.debug {
            Some(LevelFilter::Trace)
        } else {
            None
        },
    );

    let xvc_config_params = XvcConfigInitParams {
        current_dir: AbsolutePath::from(&cli_opts.workdir),
        include_system_config: !cli_opts.no_system_config,
        include_user_config: !cli_opts.no_user_config,
        project_config_path: None,
        local_config_path: None,
        include_environment_config: !cli_opts.no_env_config,
        command_line_config: Some(cli_opts.consolidate_config_options()),
        default_configuration: default_project_config(true),
    };

    let xvc_root_opt = match load_xvc_root(Path::new(&cli_opts.workdir), xvc_config_params) {
        Ok(r) => Some(r),
        Err(e) => {
            e.debug();
            None
        }
    };

    thread::scope(move |s| {
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
            if let Some(from_ref) = cli_opts.from_ref {
                uwr!(
                    git_checkout_ref(&output_snd, xvc_root, from_ref),
                    output_snd
                );
            }
        }
        let command_thread = s.spawn(move |_| -> Result<()> {
            match cli_opts.command {
                XvcSubCommand::Init(opts) => {
                    let use_git = !opts.no_git;
                    let xvc_root = init::run(xvc_root_opt.as_ref(), opts)?;
                    if use_git {
                        handle_git_automation(
                            &output_snd,
                            xvc_root,
                            cli_opts.to_branch.as_deref(),
                            &cli_opts.command_string,
                        )?;
                    }
                    Result::Ok(())
                }

                XvcSubCommand::Aliases(opts) => Ok(aliases::run(&output_snd, opts)?),

                // following commands can only be run inside a repository
                XvcSubCommand::Root(opts) => Ok(root::run(
                    &output_snd,
                    xvc_root_opt
                        .as_ref()
                        .ok_or_else(|| Error::RequiresXvcRepository)?,
                    opts,
                )?),

                XvcSubCommand::File(opts) => {
                    Ok(file::run(&output_snd, xvc_root_opt.as_ref(), opts)?)
                }

                XvcSubCommand::Pipeline(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();
                    Ok(pipeline::cmd_pipeline(
                        input,
                        &output_snd,
                        xvc_root_opt.as_ref().ok_or(Error::RequiresXvcRepository)?,
                        opts,
                    )?)
                }

                XvcSubCommand::CheckIgnore(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();

                    Ok(check_ignore::cmd_check_ignore(
                        input,
                        &output_snd,
                        xvc_root_opt.as_ref().ok_or(Error::RequiresXvcRepository)?,
                        opts,
                    )?)
                }

                XvcSubCommand::Storage(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();
                    Ok(storage::cmd_storage(
                        input,
                        &output_snd,
                        xvc_root_opt.as_ref().ok_or(Error::RequiresXvcRepository)?,
                        opts,
                    )?)
                }
            }?;

            watch!("Before handle_git_automation");
            match xvc_root_opt {
                Some(xvc_root) => {
                    watch!(&cli_opts.command_string);
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
            Ok(())
        });

        match command_thread.join().unwrap() {
            Ok(_) => debug!(output_snd_clone, "Command completed successfully."),
            Err(e) => error!(output_snd_clone, "{}", e),
        }
        output_snd_clone.send(None).unwrap();
        output_thread.join().unwrap();
    })
    .unwrap();

    Ok(())
}
