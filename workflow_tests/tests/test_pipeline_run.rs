mod common;

use std::{fs, path::Path};

use common::*;
use xvc_config::XvcVerbosity;

use xvc::error::Result;

use subprocess::Exec;
use xvc_tests::watch;

#[test]
fn test_pipeline_run() -> Result<()> {
    test_logging(log::LevelFilter::Warn);
    let xvc_root = run_in_example_xvc(true)?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["pipeline"];
        c.extend(cmd);
        run_xvc(Some(&xvc_root), &c, XvcVerbosity::Debug)
    };

    let create_pipeline = || -> Result<()> {
        x(&[
            "step",
            "new",
            "--step-name",
            "hello",
            "--command",
            "echo 'hello xvc!'",
            "--when",
            "always",
        ])?;

        x(&[
            "step",
            "new",
            "--step-name",
            "step1",
            "--command",
            "touch abc.txt",
        ])?;

        x(&[
            "step",
            "output",
            "--step-name",
            "step1",
            "--output-file",
            "abc.txt",
        ])?;

        x(&[
            "step",
            "new",
            "--step-name",
            "step_dep",
            "--command",
            "touch step_dep.txt",
        ])?;

        x(&[
            "step",
            "dependency",
            "--step-name",
            "step_dep",
            "--step",
            "step1",
        ])?;
        x(&[
            "step",
            "output",
            "--step-name",
            "step_dep",
            "--output-file",
            "step_dep.txt",
        ])?;

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

        x(&[
            "step",
            "new",
            "--step-name",
            "training_files",
            "--command",
            "find data/images/train -name '*.png' > training-files.txt",
        ])?;

        x(&[
            "step",
            "dependency",
            "--step-name",
            "training_files",
            "--directory",
            "data/images/train",
        ])?;

        x(&[
            "step",
            "output",
            "--step-name",
            "training_files",
            "--output-file",
            "training-files.txt",
        ])?;

        x(&[
            "step",
            "new",
            "--step-name",
            "glob_dep",
            "--command",
            "touch glob_dep.json",
        ])?;

        x(&[
            "step",
            "output",
            "--step-name",
            "glob_dep",
            "--output-metric",
            "glob_dep.json",
        ])?;

        x(&[
            "step",
            "new",
            "--step-name",
            "count_training_files",
            "--command",
            "wc -l training-files.txt > num-training-files.txt",
        ])?;
        x(&[
            "step",
            "dependency",
            "--step-name",
            "count_training_files",
            "--lines",
            "training-files.txt::-1000000",
        ])?;
        x(&[
            "step",
            "output",
            "--step-name",
            "count_training_files",
            "--output-file",
            "num-training-files.txt",
        ])?;

        Ok(())
    };

    create_pipeline()?;
    watch!("Before first");
    let _run_res = x(&["run"])?;
    println!("run_res: {}", _run_res);
    assert!(Path::new("abc.txt").exists());
    assert!(Path::new("src-files.txt").exists());
    assert!(Path::new("training-files.txt").exists());
    assert!(Path::new("num-training-files.txt").exists());

    Exec::shell("rm -f training-files.txt").join()?;
    watch!("Before second");
    x(&["run"])?;
    assert!(Path::new("training-files.txt").exists());

    // remove a file from training files and run again
    let file_to_remove = "data/images/train/0/59988.png";
    let training_files_before = fs::read_to_string("training-files.txt")?;
    assert!(training_files_before.contains(file_to_remove));
    Exec::shell(format!("rm -f {file_to_remove}")).join()?;
    watch!("Before third");
    x(&["run"])?;
    let training_files_after = fs::read_to_string("training-files.txt")?;
    assert!(!training_files_after.contains(file_to_remove));

    // Could we move this to ref as trycmd?

    clean_up(&xvc_root)
}
