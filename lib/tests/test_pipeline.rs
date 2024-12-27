mod common;

use common::*;
use xvc::error::Result;
use xvc::watch;
use xvc_config::XvcVerbosity;

#[test]
fn test_pipeline() -> Result<()> {
    let xvc_root = run_in_example_xvc(true)?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["pipeline"];
        c.extend(cmd);
        common::run_xvc(Some(&xvc_root), &c, XvcVerbosity::Trace)
    };

    x(&["new", "--pipeline-name", "pipeline-1"])?;
    x(&[
        "new",
        "--pipeline-name",
        "pipeline-2",
        "--workdir",
        "pipeline-2",
    ])?;
    x(&[
        "update",
        "--pipeline-name",
        "pipeline-1",
        "--rename",
        "pipeline-old",
    ])?;
    let pipelines_1 = x(&["list"])?;
    assert!(pipelines_1.contains("pipeline-old"));
    x(&["delete", "--pipeline-name", "pipeline-old"])?;
    let pipelines_2 = x(&["list"])?;
    assert!(!pipelines_2.contains("pipeline-old"));

    let res = x(&[
        "-p",
        "pipeline-2",
        "step",
        "new",
        "--step-name",
        "step1",
        "--command",
        "touch abc.txt",
    ])?;

    x(&[
        "-p",
        "pipeline-2",
        "step",
        "new",
        "--step-name",
        "step2",
        "--command",
        "echo hi xvc",
        "--when",
        "always",
    ])?;
    x(&[
        "-p",
        "pipeline-2",
        "step",
        "dependency",
        "-s",
        "step1",
        "--step",
        "step2",
    ])?;

    x(&[
        "-p",
        "pipeline-2",
        "step",
        "dependency",
        "-s",
        "step1",
        "--file",
        "data/images.tar.gz",
    ])?;

    x(&[
        "-p",
        "pipeline-2",
        "step",
        "dependency",
        "-s",
        "step1",
        "--glob",
        "data/images/train/1/*",
    ])?;

    x(&[
        "-p",
        "pipeline-2",
        "step",
        "dependency",
        "-s",
        "step1",
        "--param",
        "params.yaml::model.conv_units",
    ])?;

    x(&[
        "-p",
        "pipeline-2",
        "step",
        "dependency",
        "-s",
        "step1",
        "--regex",
        "requirements.txt:/^tensorflow",
    ])?;

    x(&[
        "-p",
        "pipeline-2",
        "step",
        "dependency",
        "-s",
        "step1",
        "--lines",
        "requirements.txt::-100",
    ])?;

    clean_up(&xvc_root)
}
