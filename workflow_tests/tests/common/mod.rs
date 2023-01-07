#![allow(dead_code)]

use anyhow::anyhow;
use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;
use std::{env, path::Path};
use subprocess::{CaptureData, Exec};
use xvc::init::InitCLI;
use xvc_config::XvcVerbosity;

use xvc_core::XvcRoot;
use xvc_logging::watch;

use xvc::error::{Error, Result};

pub use xvc_test_helper::{
    create_directory_tree, generate_random_file, generate_random_text_file, random_dir_name,
    random_temp_dir, run_in_temp_dir, run_in_temp_git_dir, test_logging,
};

const EXAMPLE_PROJECT_NAME: &str = "example-xvc";

pub fn run_xvc(cwd: Option<&Path>, args: &[&str], verbosity: XvcVerbosity) -> Result<String> {
    let mut cmd = Command::cargo_bin("xvc").unwrap();

    let verbosity_opt = match verbosity {
        XvcVerbosity::Quiet => ["--quiet"],
        XvcVerbosity::Default => [""],
        XvcVerbosity::Warn => ["-vv"],
        XvcVerbosity::Info => ["-vvv"],
        XvcVerbosity::Debug => ["-vvvv"],
        XvcVerbosity::Trace => ["-vvvvv"],
    };

    // watch!(cmd);
    // watch!(verbosity_opt);
    // watch!(args);

    let prepared = match cwd {
        Some(cwd) => cmd
            .arg("--debug")
            .args(verbosity_opt)
            .args(args)
            .current_dir(cwd),
        None => cmd.arg("--debug").args(verbosity_opt).args(args),
    };

    watch!(prepared);
    let output = prepared.output()?;

    watch!(output);

    assert!(output.status.success(), "Command failed: {:?}", prepared);

    let output_str = format!(
        "Command: {prepared:#?}\nStdout: {}\nStderr: {}",
        String::from_utf8(output.stdout)?.replace("\\\n", "\n"),
        String::from_utf8(output.stderr)?.replace("\\\n", "\n"),
    );
    Ok(output_str)
}

pub fn example_project_url() -> Result<String> {
    Ok(format!("http://one.emresult.com/~iex/{EXAMPLE_PROJECT_NAME}.tgz").to_string())
}

pub fn example_project_template_path() -> Result<PathBuf> {
    Ok(env::temp_dir().join(EXAMPLE_PROJECT_NAME))
}

pub fn download_example_project() -> Result<()> {
    let curl_output_filename = format!(
        "{}.tgz",
        env::temp_dir().join(EXAMPLE_PROJECT_NAME).to_string_lossy()
    );

    let curl_output = Command::new("curl")
        .arg(example_project_url()?)
        .arg("--output")
        .arg(curl_output_filename.to_string())
        .output()?;

    if !curl_output.status.success() {
        return Err(Error::ProcessError {
            stdout: String::from_utf8_lossy(&curl_output.stdout).into(),
            stderr: String::from_utf8_lossy(&curl_output.stderr).into(),
        });
    }

    let tar_output = Command::new("tar")
        .arg("xzf")
        .arg(curl_output_filename)
        .arg("--directory")
        .arg(&env::temp_dir().to_string_lossy().to_string())
        .output()?;

    if !tar_output.status.success() {
        return Err(Error::ProcessError {
            stdout: String::from_utf8_lossy(&tar_output.stdout).into(),
            stderr: String::from_utf8_lossy(&tar_output.stderr).into(),
        });
    }

    Ok(())
}

pub fn run_in_example_project() -> Result<PathBuf> {
    let project_template_path = example_project_template_path()?;
    if !(project_template_path.exists()) {
        download_example_project()?;
    }
    let random_example_dir = random_temp_dir(None);
    let _rsync_res = Command::new("cp")
        .args([
            "-r",
            &project_template_path.to_string_lossy(),
            &random_example_dir.to_string_lossy(),
        ])
        .output()?;
    env::set_current_dir(&random_example_dir).expect("Cannot change directory");

    Ok(random_example_dir)
}

pub fn run_in_example_xvc(with_git: bool) -> Result<XvcRoot> {
    let example_project_dir = run_in_example_project()?;
    watch!(example_project_dir);
    if with_git {
        let output = Command::new("git").arg("init").output()?;
        watch!(output);
        let xvc_root = xvc::init::run(
            None,
            InitCLI {
                path: None,
                no_git: false,
                force: false,
            },
        )?;
        watch!(xvc_root);
        Ok(xvc_root)
    } else {
        let xvc_root = xvc::init::run(
            None,
            InitCLI {
                path: None,
                no_git: true,
                force: false,
            },
        )?;
        watch!(xvc_root);
        Ok(xvc_root)
    }
}

/// Create a temporary XVC directory that's also Git repository
pub fn run_in_temp_xvc_dir() -> Result<XvcRoot> {
    let the_dir = run_in_temp_git_dir();
    watch!(&the_dir);
    let xvc_root = xvc::init::run(
        None,
        InitCLI {
            path: None,
            no_git: false,
            force: false,
        },
    )?;
    // watch!(xvc_root);
    Ok(xvc_root)
}

pub fn sh(cmd: &str) -> Result<CaptureData> {
    Exec::shell(cmd)
        .capture()
        .map_err(|e| anyhow!("{}", e).into())
}

pub fn clean_up_path_buf(path_buf: PathBuf) -> Result<()> {
    fs::remove_dir_all(&path_buf).map_err(|e| e.into())
}

pub fn clean_up(xvc_root: &XvcRoot) -> Result<()> {
    fs::remove_dir_all(&xvc_root.absolute_path()).map_err(|e| e.into())
}
