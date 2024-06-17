mod common;

use common::*;
use log::LevelFilter;
use xvc::error::Result;
use xvc_config::XvcVerbosity;

#[test]
fn test_pipeline_dag() -> Result<()> {
    test_logging(LevelFilter::Trace);
    let xvc_root = run_in_example_xvc(true)?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["pipeline"];
        c.extend(cmd);
        common::run_xvc(Some(&xvc_root), &c, XvcVerbosity::Warn)
    };
    let xsn = |name: &str, command: &str| -> Result<String> {
        x(&["step", "new", "--step-name", name, "--command", command])
    };
    let xsd = |name: &str, dep_type: &str, dep: &str| -> Result<String> {
        x(&["step", "dependency", "--step-name", name, dep_type, dep])
    };
    let xsof = |name: &str, file: &str| -> Result<String> {
        x(&["step", "output", "--step-name", name, "--output-file", file])
    };
    let xsoi = |name: &str, file: &str| -> Result<String> {
        x(&[
            "step",
            "output",
            "--step-name",
            name,
            "--output-image",
            file,
        ])
    };
    let xsom = |name: &str, file: &str| -> Result<String> {
        x(&[
            "step",
            "output",
            "--step-name",
            name,
            "--output-metric",
            file,
        ])
    };

    xsn("hello", "echo \"hello xvc!\"")?;

    xsn("step1", "touch abc.txt")?;
    xsof("step1", "abc.txt")?;

    xsn("step_dep", "touch step_dep.txt")?;
    xsd("step_dep", "--step", "step1")?;
    xsof("step_dep", "step_dep.txt")?;

    xsn("txt_files", "find . -name '*.py' > src-files.txt")?;
    xsd("txt_files", "--glob", "*/*.py")?;
    xsof("txt_files", "src-files.txt")?;

    xsn(
        "training-files",
        "find data/images/train -name '*.png' > training-files.txt",
    )?;
    xsd("training-files", "--glob", "data/images/train/*")?;
    xsof("training-files", "training-files.txt")?;

    xsn("glob_dep", "touch glob_dep.json")?;
    xsd("glob_dep", "--glob", "*.txt")?;
    xsd("glob_dep", "--glob-items", "data/*")?;
    xsd("glob_dep", "--param", "params.yaml:model.conv_units")?;
    // FIXME: There is an error here regarding regex format
    xsd("glob_dep", "--regex", "requirements.txt:^tensorflow")?;
    xsom("glob_dep", "glob_dep.json")?;
    xsof("glob_dep", "def.txt")?;
    xsoi("glob_dep", "plots/confusion.png")?;

    xsn(
        "count_training_files",
        "wc -l training-files.txt > num-training-files.txt",
    )?;
    xsd(
        "count_training_files",
        "--lines",
        "training-files.txt::1-1000000",
    )?;
    xsof("count_training_files", "num-training-files.txt")?;

    x(&["dag", "--file", "dag.out"])?;

    clean_up(&xvc_root)
}
