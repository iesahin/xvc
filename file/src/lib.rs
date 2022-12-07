#![warn(missing_docs)]
#![forbid(unsafe_code)]
mod common;

pub mod bring;
pub mod error;
pub mod hash;
pub mod list;
pub mod recheck;
pub mod send;
pub mod track;

use crate::error::{Error, Result};
use crossbeam::thread;
use crossbeam_channel::bounded;
use crossbeam_channel::Sender;
use list::ListCLI;
use log::info;
use log::warn;
use log::{error, LevelFilter};
use recheck::RecheckCLI;
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

use bring::BringCLI;
use hash::HashCLI;
use send::SendCLI;
use track::TrackCLI;

use clap::Parser;

/// xvc file subcommands
#[derive(Debug, Clone, Parser)]
#[command(rename_all = "kebab-case")]
pub enum XvcFileSubCommand {
    /// Add file and directories to Xvc
    Track(TrackCLI),
    /// Get digest hash of files with the supported algorithms
    Hash(HashCLI),
    /// Get files from cache by copy or *link
    #[command(alias = "checkout")]
    Recheck(RecheckCLI),
    /// List tracked and untracked elements in the workspace
    List(ListCLI),
    /// Send (push, upload) files to external storages
    Send(SendCLI),
    /// Bring (download, pull, fetch) files from external storages
    Bring(BringCLI),
}

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
#[derive(Debug, Clone, Parser)]
pub struct XvcFileCLI {
    /// Verbosity level. Use multiple times to increase command output detail.
    #[arg(
        long = "verbose",
        short,
        action = clap::ArgAction::Count
    )]
    pub verbosity: u8,

    /// Don't show any messages.
    #[arg(long, help = "Suppress error messages")]
    pub quiet: bool,

    /// Set the working directory to run the command as if it's in that directory.
    #[arg(short = 'C', default_value = ".")]
    pub workdir: String,

    /// Configuration options set from the command line in the form section.key=value
    #[arg(long, short = 'c')]
    pub config: Option<Vec<String>>,

    /// Ignore system config file
    #[arg(long)]
    pub no_system_config: bool,

    /// Ignore user config file
    #[arg(long)]
    pub no_user_config: bool,

    /// Ignore project config (.xvc/config)
    #[arg(long)]
    pub no_project_config: bool,

    /// Ignore local config (.xvc/config.local)
    #[arg(long)]
    pub no_local_config: bool,

    /// Ignore configuration options from the environment
    #[arg(long)]
    pub no_env_config: bool,

    /// Subcommand for xvc file
    #[command(subcommand)]
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
        XvcFileSubCommand::Recheck(opts) => recheck::cmd_checkout(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
        XvcFileSubCommand::List(opts) => list::cmd_list(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
        XvcFileSubCommand::Send(opts) => send::cmd_send(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
        XvcFileSubCommand::Bring(opts) => bring::cmd_bring(
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
