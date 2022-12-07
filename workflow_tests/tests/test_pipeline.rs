mod common;

use common::*;
use xvc::error::Result;
use xvc_config::XvcVerbosity;

#[test]
fn test_pipeline() -> Result<()> {
    let xvc_root = run_in_example_xvc(true)?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["pipeline"];
        c.extend(cmd);
        common::run_xvc(Some(&xvc_root), &c, XvcVerbosity::Trace)
    };

    x(&["new", "--name", "pipeline-1"])?;
    x(&["new", "--name", "pipeline-2", "--workdir", "pipeline-2"])?;
    x(&["update", "--name", "pipeline-1", "--rename", "pipeline-old"])?;
    let pipelines_1 = x(&["list"])?;
    assert!(matches!(pipelines_1.find("pipeline-old"), Some(..)));
    x(&["delete", "--name", "pipeline-old"])?;
    let pipelines_2 = x(&["list"])?;
    assert!(matches!(pipelines_2.find("pipeline-old"), None));

    x(&[
        "-n",
        "pipeline-2",
        "step",
        "new",
        "--step-name",
        "step1",
        "--command",
        "touch abc.txt",
    ])?;

    x(&[
        "-n",
        "pipeline-2",
        "step",
        "new",
        "--step-name",
        "step2",
        "--command",
        "echo hi xvc",
        "--changed",
        "always",
    ])?;
    x(&[
        "-n",
        "pipeline-2",
        "step",
        "dependency",
        "-s",
        "step1",
        "--step",
        "step2",
    ])?;

    x(&[
        "-n",
        "pipeline-2",
        "step",
        "dependency",
        "-s",
        "step1",
        "--file",
        "data/images.tar.gz",
    ])?;

    x(&[
        "-n",
        "pipeline-2",
        "step",
        "dependency",
        "-s",
        "step1",
        "--glob",
        "data/images/train/1/*",
    ])?;

    x(&[
        "-n",
        "pipeline-2",
        "step",
        "dependency",
        "-s",
        "step1",
        "--directory",
        "data/images/test/",
    ])?;

    x(&[
        "-n",
        "pipeline-2",
        "step",
        "dependency",
        "-s",
        "step1",
        "--param",
        "params.yaml::model.conv_units",
    ])?;

    x(&[
        "-n",
        "pipeline-2",
        "step",
        "dependency",
        "-s",
        "step1",
        "--regex",
        "requirements.txt:/^tensorflow",
    ])?;

    x(&[
        "-n",
        "pipeline-2",
        "step",
        "dependency",
        "-s",
        "step1",
        "--lines",
        "requirements.txt::-100",
    ])?;

    Ok(())
}
