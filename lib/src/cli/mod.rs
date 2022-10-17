//! Main CLI interface for XVC
use std::path::PathBuf;

use crate::init;
use clap::Parser;
use crossbeam::thread;
use crossbeam_channel::bounded;
use log::info;
use log::LevelFilter;
use std::io;
use xvc_logging::XvcOutputLine;

use std::path::Path;
use xvc_config::{XvcConfigInitParams, XvcVerbosity};
use xvc_core::check_ignore;
use xvc_core::default_project_config;
use xvc_core::root;
use xvc_core::XvcRoot;
use xvc_core::CHANNEL_BOUND;
use xvc_file as file;
use xvc_logging::setup_logging;
use xvc_logging::watch;
use xvc_pipeline as pipeline;
use xvc_remote as remote;
use xvc_walker::AbsolutePath;

use crate::cli;
use crate::error::{Error, Result};

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
/// Xvc Main Command-line Interface
pub struct XvcCLI {
    #[clap(long = "verbose", short, parse(from_occurrences))]
    /// Output verbosity. Use multiple times to increase emitted logs.
    /// TODO: Setting this option here turns off progress bars.
    pub verbosity: u8,

    #[clap(long)]
    /// Suppress all output.
    pub quiet: bool,

    #[clap(short = 'C', default_value = ".")]
    /// Set working directory for the command.
    /// It doesn't create a new shell, or change the directory.
    pub workdir: PathBuf,

    #[clap(long, short = 'c')]
    /// Configuration options set from the command line in the form section.key=value
    /// You can use multiple times.
    pub config: Option<Vec<String>>,

    #[clap(long)]
    /// Ignore system configuration file.
    pub no_system_config: bool,

    #[clap(long)]
    /// Ignore user configuration file.
    pub no_user_config: bool,

    #[clap(long)]
    /// Ignore project configuration file (.xvc/config)
    pub no_project_config: bool,

    #[clap(long)]
    /// Ignore local (gitignored) configuration file (.xvc/config.local)
    pub no_local_config: bool,

    #[clap(long)]
    /// Ignore configuration options gathered from environment variables
    pub no_env_config: bool,

    #[clap(subcommand)]
    /// The subcommand to run
    pub command: XvcSubCommand,
}

/// Xvc subcommands
#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
pub enum XvcSubCommand {
    /// File and directory management commands
    File(xvc_file::XvcFileCLI),
    /// Find the root directory of a project
    Root(xvc_core::root::RootCLI),
    /// Check whether files are ignored with `.xvcignore`
    CheckIgnore(xvc_core::check_ignore::CheckIgnoreCLI),
    /// Initialize an Xvc project
    Init(crate::init::InitCLI),
    /// Pipeline management commands
    Pipeline(xvc_pipeline::PipelineCLI),
    /// Remote (cloud) management commands
    Remote(xvc_remote::RemoteCLI),
}

/// Runs the supplied xvc command.
pub fn run(args: &[&str]) -> Result<()> {
    let cli_options = cli::XvcCLI::parse_from(args);
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

    setup_logging(Some(term_log_level), None);

    let xvc_config_params = XvcConfigInitParams {
        current_dir: AbsolutePath::from(&cli_opts.workdir),
        include_system_config: !cli_opts.no_system_config,
        include_user_config: !cli_opts.no_user_config,
        project_config_path: None,
        local_config_path: None,
        include_environment_config: !cli_opts.no_env_config,
        command_line_config: cli_opts.config.clone(),
        default_configuration: default_project_config(true),
    };

    let xvc_root_opt = match XvcRoot::new(Path::new(&cli_opts.workdir), xvc_config_params) {
        Ok(r) => Some(r),
        Err(e) => {
            e.info();
            None
        }
    };

    thread::scope(move |s| {
        let (output_snd, output_rec) = bounded::<XvcOutputLine>(CHANNEL_BOUND);

        s.spawn(move |_| -> Result<()> {
            match cli_opts.command {
                XvcSubCommand::Init(opts) => {
                    drop(output_snd);
                    init::run(xvc_root_opt.as_ref(), opts)?;
                    Result::Ok(())
                }

                // following commands can only be run inside a repository
                XvcSubCommand::Root(opts) => Ok(root::run(
                    output_snd,
                    xvc_root_opt
                        .as_ref()
                        .ok_or_else(|| Error::RequiresXvcRepository)?,
                    opts,
                )?),

                XvcSubCommand::File(opts) => {
                    Ok(file::run(output_snd, xvc_root_opt.as_ref(), opts)?)
                }

                XvcSubCommand::Pipeline(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();
                    Ok(pipeline::run(
                        input,
                        output_snd,
                        xvc_root_opt.as_ref().ok_or(Error::RequiresXvcRepository)?,
                        opts,
                    )?)
                }

                XvcSubCommand::CheckIgnore(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();

                    Ok(check_ignore::cmd_check_ignore(
                        input,
                        output_snd,
                        xvc_root_opt.as_ref().ok_or(Error::RequiresXvcRepository)?,
                        opts,
                    )?)
                }
                XvcSubCommand::Remote(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();
                    Ok(remote::cmd_storage(
                        input,
                        output_snd,
                        xvc_root_opt.as_ref().ok_or(Error::RequiresXvcRepository)?,
                        opts,
                    )?)
                }
            }?;

            Ok(())
        });

        s.spawn(move |_| {
            while let Ok(output_line) = output_rec.recv() {
                // output_str.push_str(&output_line);
                match output_line {
                    XvcOutputLine::Output(m) => println!("{m}"),
                    XvcOutputLine::Info(m) => println!("[INFO] {m}"),
                    XvcOutputLine::Warn(m) => println!("[WARN] {m}"),
                    XvcOutputLine::Error(m) => println!("[ERROR] {m}"),
                    XvcOutputLine::Panic(m) => panic!("[ERROR] {m}"),
                    XvcOutputLine::Tick(t) => todo!(),
                }
            }
        })
        .join()
        .unwrap();
    })
    .unwrap();

    Ok(())
}

/// Used to run commands when an already xvc_root is present and loaded.
/// This is meant to be used in tests.
///
/// This one is similar to [dispatch] above.
/// Differences:
/// - It uses a possibly loaded [XvcRoot].
///   This prevents reinit errors for those functions run [std::sync::Once].
/// - verbosity can be set directly.
/// - The output is sent as `String` instead of writing to stdout.
pub fn test_dispatch(
    xvc_root_opt: Option<&XvcRoot>,
    cli_opts: cli::XvcCLI,
    verbosity: XvcVerbosity,
) -> Result<String> {
    let term_log_level = match verbosity {
        XvcVerbosity::Quiet => LevelFilter::Off,
        XvcVerbosity::Default => LevelFilter::Error,
        XvcVerbosity::Warn => LevelFilter::Warn,
        XvcVerbosity::Info => LevelFilter::Info,
        XvcVerbosity::Debug => LevelFilter::Debug,
        XvcVerbosity::Trace => LevelFilter::Trace,
    };

    watch!(term_log_level);

    setup_logging(Some(term_log_level), Some(LevelFilter::Trace));

    let xvc_config_params = XvcConfigInitParams {
        current_dir: AbsolutePath::from(&cli_opts.workdir),
        include_system_config: !cli_opts.no_system_config,
        include_user_config: !cli_opts.no_user_config,
        project_config_path: None,
        local_config_path: None,
        include_environment_config: !cli_opts.no_env_config,
        command_line_config: cli_opts.config.clone(),
        default_configuration: default_project_config(true),
    };

    watch!(xvc_config_params.current_dir);

    let xvc_root_res = Box::new(XvcRoot::new(
        Path::new(&cli_opts.workdir),
        xvc_config_params,
    ));
    let xvc_root_opt_res = if xvc_root_opt.is_none() {
        match xvc_root_res.as_ref() {
            Ok(r) => Some(r),
            Err(e) => {
                info!("{:?}", e);
                None
            }
        }
    } else {
        xvc_root_opt
    };

    let output_str = thread::scope(move |s| {
        let (output_snd, output_rec) = bounded::<XvcOutputLine>(CHANNEL_BOUND);

        s.spawn(move |_| -> Result<()> {
            match cli_opts.command {
                XvcSubCommand::Init(opts) => {
                    drop(output_snd);
                    init::run(xvc_root_opt_res, opts)?;
                    Result::Ok(())
                }

                // following commands can only be run inside a repository
                XvcSubCommand::Root(opts) => Ok(root::run(
                    output_snd,
                    xvc_root_opt
                        .as_ref()
                        .ok_or_else(|| Error::RequiresXvcRepository)?,
                    opts,
                )?),

                XvcSubCommand::File(opts) => Ok(file::run(output_snd, xvc_root_opt, opts)?),

                XvcSubCommand::Pipeline(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();
                    Ok(pipeline::run(
                        input,
                        output_snd,
                        xvc_root_opt.as_ref().ok_or(Error::RequiresXvcRepository)?,
                        opts,
                    )?)
                }

                XvcSubCommand::CheckIgnore(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();

                    Ok(check_ignore::cmd_check_ignore(
                        input,
                        output_snd,
                        xvc_root_opt.as_ref().ok_or(Error::RequiresXvcRepository)?,
                        opts,
                    )?)
                }
                XvcSubCommand::Remote(opts) => {
                    let stdin = io::stdin();
                    let input = stdin.lock();

                    Ok(remote::cmd_storage(
                        input,
                        output_snd,
                        xvc_root_opt.as_ref().ok_or(Error::RequiresXvcRepository)?,
                        opts,
                    )?)
                }
            }?;

            Ok(())
        });

        watch!("Spawned match");

        let output_res = s
            .spawn(move |_| {
                let mut output_str = String::new();
                watch!(output_str);
                while let Ok(output_line) = output_rec.recv() {
                    match output_line {
                        // TODO: We should handle ticks and other stuff here
                        XvcOutputLine::Output(m) => output_str.push_str(&m),
                        XvcOutputLine::Info(m) => output_str.push_str(&format!("[INFO] {m}")),
                        XvcOutputLine::Warn(m) => output_str.push_str(&format!("[WARN] {m}")),
                        XvcOutputLine::Error(m) => output_str.push_str(&format!("[ERROR] {m}")),
                        XvcOutputLine::Panic(m) => {
                            output_str.push_str(&format!("[PANIC] {m}"));
                            break;
                        }
                        XvcOutputLine::Tick(_) => {}
                    }
                    output_str.push('\n');
                    watch!(output_str);
                }
                watch!("Exit output loop");
                output_str
            })
            .join();

        match output_res {
            Ok(s) => {
                watch!("Output returned");
                Ok(s)
            }
            Err(err) => {
                log::error!("Error in output handler");
                Err(Error::OutputError)
            }
        }
    })
    .unwrap();

    output_str
}
