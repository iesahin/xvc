#![warn(missing_docs)]
#![forbid(unsafe_code)]
mod common;

pub mod checkout;
pub mod error;
pub mod fetch;
pub mod hash;
pub mod list;
pub mod pull;
pub mod push;
pub mod track;

use crate::error::{Error, Result};
use checkout::CheckoutCLI;
use crossbeam::thread;
use crossbeam_channel::bounded;
use crossbeam_channel::Sender;
use fetch::FetchCLI;
use list::ListCLI;
use log::info;
use log::warn;
use log::{error, LevelFilter};
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use xvc_config::XvcConfigInitParams;
use xvc_config::XvcVerbosity;
use xvc_core::default_project_config;
use xvc_core::XvcRoot;
use xvc_core::CHANNEL_BOUND;
use xvc_logging::setup_logging;
use xvc_logging::XvcOutputLine;
use xvc_walker::AbsolutePath;

use hash::HashCLI;
use pull::PullCLI;
use push::PushCLI;
use track::TrackCLI;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(rename_all = "kebab-case")]
pub enum XvcFileSubCommand {
    Track(TrackCLI),
    Hash(HashCLI),
    Checkout(CheckoutCLI),
    List(ListCLI),
    Push(PushCLI),
    Fetch(FetchCLI),
    Pull(PullCLI),
}

#[derive(Debug, Clone, Parser)]
/// Operations on data files
///
/// This command can be used to operate on files, like
///
/// - adding files to xvc cache and link by various methods
///
/// - calculating hash of files (even outside of xvc repo)
///
/// - listing files in repo (even if they are deleted from workspace)
///
/// - moving files to other locations
///
/// - deleting files and all their associated cache content
pub struct XvcFileCLI {
    #[clap(
        long = "verbose",
        short,
        help = "Verbosity level, use multiple times to increase",
        parse(from_occurrences)
    )]
    pub verbosity: u8,

    #[clap(long, help = "Suppress error messages")]
    pub quiet: bool,

    #[clap(short = 'C', default_value = ".")]
    pub workdir: String,

    #[clap(
        long,
        short = 'c',
        help = "Configuration options set from the command line in the form section.key=value"
    )]
    pub config: Option<Vec<String>>,

    #[clap(long, help = "Ignore system config")]
    pub no_system_config: bool,

    #[clap(long, help = "Ignore user config")]
    pub no_user_config: bool,

    #[clap(long, help = "Ignore project config (.xvc/config)")]
    pub no_project_config: bool,

    #[clap(long, help = "Ignore local config (.xvc/config.local)")]
    pub no_local_config: bool,

    #[clap(long, help = "Ignore configuration options from the environment")]
    pub no_env_config: bool,

    #[clap(subcommand)]
    subcommand: XvcFileSubCommand,
}

pub fn run(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: Option<&XvcRoot>,
    opts: XvcFileCLI,
) -> Result<()> {
    match opts.subcommand {
        XvcFileSubCommand::Track(opts) => track::cmd_track(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
        XvcFileSubCommand::Hash(opts) => hash::cmd_hash(output_snd, xvc_root, opts),
        XvcFileSubCommand::Checkout(opts) => checkout::cmd_checkout(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
        XvcFileSubCommand::List(opts) => list::cmd_list(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
        XvcFileSubCommand::Push(opts) => push::cmd_push(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
        XvcFileSubCommand::Fetch(opts) => fetch::cmd_fetch(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
        XvcFileSubCommand::Pull(opts) => pull::cmd_pull(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
    }
}

pub fn dispatch(cli_opts: XvcFileCLI) -> Result<()> {
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
    let dir = PathBuf::from(cli_opts.workdir.clone());
    let current_dir = if dir.is_absolute() {
        AbsolutePath::from(dir)
    } else {
        AbsolutePath::from(std::env::current_dir()?.join(dir).canonicalize()?)
    };
    // try to create root
    let xvc_config_params = XvcConfigInitParams {
        current_dir,
        include_system_config: !cli_opts.no_system_config,
        include_user_config: !cli_opts.no_user_config,
        project_config_path: None,
        local_config_path: None,
        include_environment_config: !cli_opts.no_env_config,
        command_line_config: cli_opts.config.clone(),
        default_configuration: default_project_config(true),
    };

    let xvc_root = match XvcRoot::new(Path::new(&cli_opts.workdir), xvc_config_params) {
        Ok(r) => Some(r),
        Err(e) => {
            e.info();
            None
        }
    };

    thread::scope(move |s| {
        let (output_snd, output_rec) = bounded::<XvcOutputLine>(CHANNEL_BOUND);
        // let (input_snd, input_rec) = bounded(CHANNEL_BOUND);
        s.spawn(move |_| {
            let mut output = io::stdout();
            while let Ok(output_line) = output_rec.recv() {
                match output_line {
                    XvcOutputLine::Output(m) => writeln!(output, "{}", m).unwrap(),
                    XvcOutputLine::Info(m) => info!("[INFO] {}", m),
                    XvcOutputLine::Warn(m) => warn!("[WARN] {}", m),
                    XvcOutputLine::Error(m) => error!("[ERROR] {}", m),
                    XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m),
                    XvcOutputLine::Tick(_) => {}
                }
            }
        });

        s.spawn(move |_| run(output_snd, xvc_root.as_ref(), cli_opts).map_err(|e| e.error()));
    })
    .map_err(|e| error!("{:?}", e))
    .expect("Crossbeam scope error");

    Ok(())
}

// this is run during repository initialization
pub fn init(_xvc_root: &XvcRoot) -> Result<()> {
    Ok(())
}
