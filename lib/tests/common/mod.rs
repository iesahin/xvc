#![allow(dead_code)]

use anyhow::anyhow;
use assert_cmd::Command;

use std::path::PathBuf;
use std::{env, path::Path};
use subprocess::{CaptureData, Exec};
use xvc::init::InitCLI;
use xvc_core::XvcVerbosity;

use xvc_core::XvcRoot;
use xvc_core::output;

use xvc::error::{Error, Result};

#[allow(unused_imports)]
pub use xvc_test_helper::{
    create_directory_tree, generate_random_file, generate_random_text_file, random_dir_name,
    random_temp_dir, run_in_temp_dir, run_in_temp_git_dir, test_logging,
};

const EXAMPLE_PROJECT_NAME: &str = "example-xvc";

pub fn run_xvc(cwd: Option<&Path>, args: &[&str], verbosity: XvcVerbosity) -> Result<String> {
    match Command::cargo_bin("xvc") {
        Ok(mut cmd) => {
            let verbosity_opt = match verbosity {
                XvcVerbosity::Quiet => vec!["--quiet"],
                XvcVerbosity::Default => vec![],
                XvcVerbosity::Warn => vec!["-v"],
                XvcVerbosity::Info => vec!["-vv"],
                XvcVerbosity::Debug => vec!["-vvv"],
                XvcVerbosity::Trace => vec!["--debug", "-vvvvv"],
            };

            let prepared = match cwd {
                Some(cwd) => cmd.args(verbosity_opt).args(args).current_dir(cwd),
                None => cmd.args(verbosity_opt).args(args),
            };

            let output = prepared.output()?;

            output!("{:?}", &output);
            output!("{:?}", &output.status);
            assert!(output.status.success(), "Command failed: {:?}", prepared);

            let output_str = String::from_utf8_lossy(&output.stdout).replace("\\\n", "\n");
            let debug_output_str = format!(
                "Command: {prepared:#?}\nStdout: {}\nStderr: {}",
                output_str,
                String::from_utf8_lossy(&output.stderr).replace("\\\n", "\n"),
            );
            println!("{}", debug_output_str);
            Ok(output_str)
        }
        Err(e) => {
            println!("{}", e);
            Err(e.into())
        }
    }
}

pub fn example_project_url() -> Result<String> {
    Ok("https://e1.xvc.dev/example-xvc.tgz".to_string())
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
        .arg(&curl_output_filename)
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
        .arg(env::temp_dir().to_string_lossy().as_ref())
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
    run_in_example_project()?;
    if with_git {
        Command::new("git").arg("init").output()?;
        let xvc_root = xvc::init::run(
            None,
            InitCLI {
                path: None,
                no_git: false,
                force: false,
            },
        )?;
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
        Ok(xvc_root)
    }
}

/// Create a temporary Xvc directory that's also Git repository
pub fn run_in_temp_xvc_dir() -> Result<XvcRoot> {
    run_in_temp_git_dir();
    let xvc_root = xvc::init::run(
        None,
        InitCLI {
            path: None,
            no_git: false,
            force: false,
        },
    )?;
    Ok(xvc_root)
}

pub fn sh(cmd: &str) -> Result<CaptureData> {
    Exec::shell(cmd)
        .capture()
        .map_err(|e| anyhow!("{}", e).into())
}

pub fn clean_up_path_buf(path_buf: PathBuf) -> Result<()> {
    sh(format!("chmod -R 777 {}", path_buf.to_string_lossy()).as_str())?;
    sh(format!("rm -rf {}", path_buf.to_string_lossy()).as_str())?;
    Ok(())
}

pub fn clean_up(xvc_root: &XvcRoot) -> Result<()> {
    sh(format!(
        "chmod -R 777 {}",
        xvc_root.absolute_path().to_string_lossy()
    )
    .as_str())?;
    sh(format!("rm -rf {}", xvc_root.absolute_path().to_string_lossy()).as_str())?;
    Ok(())
}
