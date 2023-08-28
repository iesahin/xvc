//! Xvc operations on files
//!
//! Most of these commands require an Xvc repository [XvcRoot] to be present.
//!
//! Modules correspond to subcommands, and are documented separately.
//!
#![warn(missing_docs)]
#![forbid(unsafe_code)]
pub mod common;
pub use common::compare;

pub mod bring;
pub mod carry_in;
pub mod copy;
pub mod error;
pub mod hash;
pub mod list;
pub mod mv;
pub mod recheck;
pub mod remove;
pub mod send;
pub mod track;
pub mod untrack;

use crate::error::{Error, Result};
use carry_in::CarryInCLI;
use clap::Subcommand;
use copy::CopyCLI;
use crossbeam::thread;
use crossbeam_channel::bounded;

use list::ListCLI;
use log::{debug, error, info, warn, LevelFilter};
use mv::MoveCLI;
use remove::RemoveCLI;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use untrack::UntrackCLI;
use xvc_config::XvcConfigInitParams;
use xvc_config::XvcVerbosity;
use xvc_core::default_project_config;
use xvc_core::types::xvcroot::load_xvc_root;
use xvc_core::XvcRoot;
use xvc_core::CHANNEL_BOUND;
use xvc_logging::{setup_logging, watch};
use xvc_logging::{XvcOutputLine, XvcOutputSender};
use xvc_walker::AbsolutePath;

use bring::BringCLI;
use hash::HashCLI;
use recheck::RecheckCLI;
use send::SendCLI;
use track::TrackCLI;

use clap::Parser;

/// xvc file subcommands
#[derive(Debug, Clone, Subcommand)]
#[command(author, version)]
pub enum XvcFileSubCommand {
    /// Add file and directories to Xvc
    Track(TrackCLI),
    /// Get digest hash of files with the supported algorithms
    Hash(HashCLI),
    /// Get files from cache by copy or *link
    #[command(alias = "checkout")]
    Recheck(RecheckCLI),
    /// Carry (commit) changed files to cache
    #[command(alias = "commit")]
    CarryIn(CarryInCLI),
    /// Copy from source to another location in the workspace
    Copy(CopyCLI),
    /// Move files to another location in the workspace
    Move(MoveCLI),
    /// List tracked and untracked elements in the workspace
    List(ListCLI),
    /// Send (push, upload) files to external storages
    Send(SendCLI),
    /// Bring (download, pull, fetch) files from external storages
    Bring(BringCLI),
    /// Remove files from Xvc and possibly storages
    Remove(RemoveCLI),
    /// Untrack (delete) files from Xvc and possibly storages
    Untrack(UntrackCLI),
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

/// Entry point for the `xvc file` command.
///
/// It runs the subcommand specified in the command line arguments.
pub fn run(
    output_snd: &XvcOutputSender,
    xvc_root: Option<&XvcRoot>,
    opts: XvcFileCLI,
) -> Result<()> {
    watch!(opts);
    match opts.subcommand {
        XvcFileSubCommand::Track(opts) => track::cmd_track(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
        XvcFileSubCommand::Hash(opts) => hash::cmd_hash(output_snd, xvc_root, opts),
        XvcFileSubCommand::CarryIn(opts) => carry_in::cmd_carry_in(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
        XvcFileSubCommand::Recheck(opts) => recheck::cmd_recheck(
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
        XvcFileSubCommand::Copy(opts) => copy::cmd_copy(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
        XvcFileSubCommand::Move(opts) => mv::cmd_move(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
        XvcFileSubCommand::Untrack(opts) => untrack::cmd_untrack(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
        XvcFileSubCommand::Remove(opts) => remove::cmd_remove(
            output_snd,
            xvc_root.ok_or(Error::RequiresXvcRepository)?,
            opts,
        ),
    }
}

/// Dispatch function for the `xvc-file` binary.
///
/// This works almost identically with the [xvc::dispatch] function.
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

    let xvc_root = match load_xvc_root(Path::new(&cli_opts.workdir), xvc_config_params) {
        Ok(r) => Some(r),
        Err(e) => {
            e.info();
            None
        }
    };

    thread::scope(move |s| {
        let (output_snd, output_rec) = bounded::<Option<XvcOutputLine>>(CHANNEL_BOUND);
        s.spawn(move |_| {
            let mut output = io::stdout();
            while let Ok(Some(output_line)) = output_rec.recv() {
                match output_line {
                    XvcOutputLine::Output(m) => writeln!(output, "{}", m).unwrap(),
                    XvcOutputLine::Info(m) => info!("[INFO] {}", m),
                    XvcOutputLine::Warn(m) => warn!("[WARN] {}", m),
                    XvcOutputLine::Error(m) => error!("[ERROR] {}", m),
                    XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m),
                    XvcOutputLine::Debug(m) => debug!("[DEBUG] {}", m),
                    XvcOutputLine::Tick(_) => {}
                }
            }
        });

        s.spawn(move |_| run(&output_snd, xvc_root.as_ref(), cli_opts).map_err(|e| e.error()));
    })
    .map_err(|e| error!("{:?}", e))
    .expect("Crossbeam scope error");

    Ok(())
}

/// This is run during `xvc init` for `xvc file` related initialization.
///
/// It's a NOOP currently.
pub fn init(_xvc_root: &XvcRoot) -> Result<()> {
    Ok(())
}

/// Crossbeam channel capacity for channels in this crate
pub const CHANNEL_CAPACITY: usize = 100000;
