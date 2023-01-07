//! Main CLI interface for XVC
use std::ffi::OsString;
use std::path::PathBuf;
use std::str::FromStr;

use crate::init;
use clap::Parser;
use crossbeam::thread;
use crossbeam_channel::bounded;
use log::{debug, error, info, warn, LevelFilter};
use std::io;
use subprocess::Exec;
use which;
use xvc_logging::XvcOutputLine;

use std::path::Path;
use xvc_config::{XvcConfigInitParams, XvcVerbosity};
use xvc_core::aliases;
use xvc_core::check_ignore;
use xvc_core::default_project_config;
use xvc_core::root;
use xvc_core::XvcRoot;
use xvc_core::CHANNEL_BOUND;
use xvc_file as file;
use xvc_logging::setup_logging;
use xvc_logging::watch;
use xvc_pipeline as pipeline;
use xvc_storage as storage;
use xvc_walker::AbsolutePath;

use crate::cli;
use crate::error::{Error, Result};

/// Xvc CLI to manage data and ML pipelines
#[derive(Debug, Parser)]
#[command(rename_all = "kebab-case", author, version)]
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

    /// Collects cli config with -c options along with direct options (like verbosity) to provide
    /// to XvcConfig constructor.
    pub fn consolidate_config_options(&self) -> Vec<String> {
        let mut output = self.config.clone().unwrap_or_default();
        output.push(format!(
            "core.verbosity = {}",
            XvcVerbosity::from(self.verbosity).to_string()
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

    let xvc_root_opt = match XvcRoot::new(Path::new(&cli_opts.workdir), xvc_config_params) {
        Ok(r) => Some(r),
        Err(e) => {
            e.debug();
            None
        }
    };

    if let Some(ref xvc_root) = xvc_root_opt {
        if let Some(from_ref) = cli_opts.from_ref {
            git_checkout_ref(xvc_root, from_ref)?;
        }
    }

    thread::scope(move |s| {
        let (output_snd, output_rec) = bounded::<XvcOutputLine>(CHANNEL_BOUND);

        s.spawn(move |_| -> Result<()> {
            match cli_opts.command {
                XvcSubCommand::Init(opts) => {
                    drop(output_snd);
                    let use_git = !opts.no_git;
                    let xvc_root = init::run(xvc_root_opt.as_ref(), opts)?;
                    if use_git {
                        handle_git_automation(
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
                    Ok(pipeline::run(
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
                    handle_git_automation(
                        xvc_root,
                        cli_opts.to_branch.as_deref(),
                        &cli_opts.command_string,
                    )?;
                }

                None => {
                    warn!("Xvc is outside of a project, no need to handle Git operations.");
                }
            }
            Ok(())
        });

        s.spawn(move |_| {
            while let Ok(output_line) = output_rec.recv() {
                // output_str.push_str(&output_line);
                match term_log_level {
                    LevelFilter::Off => match output_line {
                        XvcOutputLine::Output(_) => {}
                        XvcOutputLine::Info(_) => {}
                        XvcOutputLine::Warn(_) => {}
                        XvcOutputLine::Error(_) => {}
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m.to_string()),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Error => match output_line {
                        XvcOutputLine::Output(m) => println!("{m}"),
                        XvcOutputLine::Info(_) => {}
                        XvcOutputLine::Warn(_) => {}
                        XvcOutputLine::Error(m) => println!("[ERROR] {}", m.to_string()),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m.to_string()),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Warn => match output_line {
                        XvcOutputLine::Output(m) => println!("{m}"),
                        XvcOutputLine::Warn(m) => println!("[WARN] {}", m.to_string()),
                        XvcOutputLine::Error(m) => println!("[ERROR] {}", m.to_string()),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m.to_string()),
                        XvcOutputLine::Info(_) => {}
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Info => match output_line {
                        XvcOutputLine::Output(m) => println!("{m}"),
                        XvcOutputLine::Info(m) => println!("[INFO] {}", m.to_string()),
                        XvcOutputLine::Warn(m) => println!("[WARN] {}", m.to_string()),
                        XvcOutputLine::Error(m) => println!("[ERROR] {}", m.to_string()),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m.to_string()),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(_) => {}
                    },
                    LevelFilter::Debug => match output_line {
                        XvcOutputLine::Output(m) => println!("{m}"),
                        XvcOutputLine::Info(m) => println!("[INFO] {}", m.to_string()),
                        XvcOutputLine::Warn(m) => println!("[WARN] {}", m.to_string()),
                        XvcOutputLine::Error(m) => println!("[ERROR] {}", m.to_string()),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m.to_string()),
                        XvcOutputLine::Tick(_) => todo!(),
                        XvcOutputLine::Debug(m) => println!("[DEBUG] {}", m),
                    },
                    LevelFilter::Trace => match output_line {
                        XvcOutputLine::Output(m) => println!("{m}"),
                        XvcOutputLine::Info(m) => println!("[INFO] {}", m.to_string()),
                        XvcOutputLine::Warn(m) => println!("[WARN] {}", m.to_string()),
                        XvcOutputLine::Error(m) => println!("[ERROR] {}", m.to_string()),
                        XvcOutputLine::Debug(m) => println!("[DEBUG] {}", m),
                        XvcOutputLine::Panic(m) => panic!("[PANIC] {}", m.to_string()),
                        XvcOutputLine::Tick(_) => todo!(),
                    },
                }
            }
        })
        .join()
        .unwrap();
    })
    .unwrap();

    Ok(())
}

fn get_absolute_git_command(git_command: &str) -> Result<String> {
    let git_cmd_path = PathBuf::from(git_command);
    let git_cmd = if git_cmd_path.is_absolute() {
        git_command.to_string()
    } else {
        let cmd_path = which::which(git_command)?;
        cmd_path.to_string_lossy().to_string()
    };
    Ok(git_cmd)
}

fn exec_git(git_command: &str, xvc_directory: &str, args_str_vec: &[&str]) -> Result<String> {
    let mut args = vec!["-C", xvc_directory];
    args.extend(args_str_vec);
    let args: Vec<OsString> = args
        .iter()
        .map(|s| OsString::from_str(s).unwrap())
        .collect();
    watch!(args);
    let proc_res = Exec::cmd(git_command).args(&args).capture()?;

    match proc_res.exit_status {
        subprocess::ExitStatus::Exited(0) => Ok(proc_res.stdout_str()),
        subprocess::ExitStatus::Exited(_) => Err(Error::GitProcessError {
            stdout: proc_res.stdout_str(),
            stderr: proc_res.stderr_str(),
        }),
        subprocess::ExitStatus::Signaled(_)
        | subprocess::ExitStatus::Other(_)
        | subprocess::ExitStatus::Undetermined => Err(Error::GitProcessError {
            stdout: proc_res.stdout_str(),
            stderr: proc_res.stderr_str(),
        }),
    }
}

fn stash_user_staged_files(git_command: &str, xvc_directory: &str) -> Result<String> {
    // Do we have user staged files?
    let git_diff_staged_out = exec_git(
        git_command,
        xvc_directory,
        &["diff", "--name-only", "--cached"],
    )?;

    watch!(git_diff_staged_out);

    // If so stash them
    if git_diff_staged_out.trim().len() > 0 {
        debug!("Stashing user staged files: {git_diff_staged_out}");
        let stash_out = exec_git(git_command, xvc_directory, &["stash", "push", "--staged"])?;
        debug!("Stashed user staged files: {stash_out}");
    }

    Ok(git_diff_staged_out)
}

fn unstash_user_staged_files(git_command: &str, xvc_directory: &str) -> Result<()> {
    let res_git_stash_pop = exec_git(git_command, xvc_directory, &["stash", "pop", "--index"])?;
    debug!("Unstashed user staged files: {res_git_stash_pop}");
    Ok(())
}

fn git_checkout_ref(xvc_root: &XvcRoot, from_ref: String) -> Result<()> {
    let xvc_directory = xvc_root.as_path().to_str().unwrap();
    let git_command_option = xvc_root.config().get_str("git.command")?.option;
    let git_command = get_absolute_git_command(&git_command_option)?;

    let git_diff_staged_out = stash_user_staged_files(&git_command, xvc_directory)?;
    exec_git(&git_command, xvc_directory, &["checkout", &from_ref])?;

    if git_diff_staged_out.trim().len() > 0 {
        debug!("Unstashing user staged files: {git_diff_staged_out}");
        unstash_user_staged_files(&git_command, xvc_directory)?;
    }
    Ok(())
}

/// This receives `xvc_root` ownership because as a final operation, it must drop the root to
/// record the last entity counter before commit.
fn handle_git_automation(xvc_root: XvcRoot, to_branch: Option<&str>, xvc_cmd: &str) -> Result<()> {
    let xvc_root_dir = xvc_root.as_path().to_path_buf();
    let xvc_root_str = xvc_root_dir.to_str().unwrap();
    let use_git = xvc_root.config().get_bool("git.use_git")?.option;
    let auto_commit = xvc_root.config().get_bool("git.auto_commit")?.option;
    let auto_stage = xvc_root.config().get_bool("git.auto_stage")?.option;
    let git_command_str = xvc_root.config().get_str("git.command")?.option;
    let git_command = get_absolute_git_command(&git_command_str)?;
    let xvc_dir = xvc_root.xvc_dir().clone();
    let xvc_dir_str = xvc_dir.to_str().unwrap();

    // we drop here to record the final state
    drop(xvc_root);

    if use_git {
        if auto_commit {
            git_auto_commit(&git_command, xvc_root_str, xvc_dir_str, xvc_cmd, to_branch)?;
        } else if auto_stage {
            git_auto_stage(&git_command, xvc_root_str, xvc_dir_str)?;
        }
    }

    Ok(())
}

fn git_auto_commit(
    git_command: &str,
    xvc_root_str: &str,
    xvc_dir_str: &str,
    xvc_cmd: &str,
    to_branch: Option<&str>,
) -> Result<()> {
    debug!("Using Git: {git_command}");

    let git_diff_staged_out = stash_user_staged_files(&git_command, xvc_root_str)?;

    if let Some(branch) = to_branch {
        debug!("Checking out branch {branch}");
        exec_git(&git_command, xvc_root_str, &["checkout", "-b", branch])?;
    }

    // Add and commit `.xvc`
    let res_git_add = exec_git(
        &git_command,
        xvc_root_str,
        &["add", &xvc_dir_str, "*.gitignore", "*.xvcignore"],
    )?;
    debug!("Adding .xvc/ to git: {res_git_add}");
    let res_git_commit = exec_git(
        &git_command,
        xvc_root_str,
        &[
            "commit",
            "-m",
            &format!("Xvc auto-commit after '{xvc_cmd}'"),
        ],
    )?;
    debug!("Committing .xvc/ to git: {res_git_commit}");

    // Pop the stash if there were files we stashed

    if git_diff_staged_out.trim().len() > 0 {
        debug!("Unstashing user staged files: {git_diff_staged_out}");
        unstash_user_staged_files(&git_command, xvc_root_str)?;
    }
    Ok(())
}

fn git_auto_stage(git_command: &str, xvc_root_str: &str, xvc_dir_str: &str) -> Result<()> {
    let res_git_add = exec_git(
        &git_command,
        xvc_root_str,
        &["add", &xvc_dir_str, "*.gitignore", "*.xvcignore"],
    )?;
    debug!("Staging .xvc/ to git: {res_git_add}");
    Ok(())
}
