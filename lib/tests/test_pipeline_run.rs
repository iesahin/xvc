mod common;

use std::path::Path;

use common::*;
use xvc_core::XvcVerbosity;

use xvc::error::Result;

#[test]
fn test_pipeline_run() -> Result<()> {
    test_logging(log::LevelFilter::Warn);
    let xvc_root = run_in_example_xvc(true)?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["pipeline"];
        c.extend(cmd);
        run_xvc(Some(&xvc_root), &c, XvcVerbosity::Trace)
    };

    let create_pipeline = || -> Result<()> {
        x(&[
            "step",
            "new",
            "--step-name",
            "txt_files",
            "--command",
            "find . -name '*.py' > src-files.txt",
        ])?;

        x(&[
            "step",
            "dependency",
            "--step-name",
            "txt_files",
            "--glob",
            "*/*.py",
        ])?;

        x(&[
            "step",
            "output",
            "--step-name",
            "txt_files",
            "--output-file",
            "src-files.txt",
        ])?;
        Ok(())
    };

    create_pipeline()?;
    let _run_res = x(&["run"])?;
    println!("run_res: {}", _run_res);
    assert!(Path::new("src-files.txt").exists());
    clean_up(&xvc_root)
}
