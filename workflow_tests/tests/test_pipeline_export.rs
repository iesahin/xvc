mod common;

use std::path::Path;

use common::*;
use serde_json;
use xvc_config::XvcVerbosity;
use xvc_core::XvcPath;
use xvc_pipeline::{XvcDependency, XvcMetricsFormat, XvcOutput, XvcParamFormat, XvcPipelineSchema};

use xvc::error::Result;
use xvc_tests::watch;

#[test]
fn test_pipeline_export() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let xvc_root = run_in_example_xvc(true)?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["pipeline"];
        c.extend(cmd);
        common::run_xvc(Some(&xvc_root), &c, XvcVerbosity::Trace)
    };

    x(&[
        "step",
        "new",
        "--step-name",
        "step1",
        "--command",
        "touch abc.txt",
    ])?;

    let export_res1 = x(&["export"])?;

    let ps1: XvcPipelineSchema = serde_json::from_str(&export_res1)?;

    assert!(ps1.name == "default");
    assert!(ps1.version == 1);
    assert!(ps1.steps.len() == 1);
    assert!(ps1.steps[0].command == "touch abc.txt");
    assert!(ps1.steps[0].name == "step1");

    x(&[
        "step",
        "new",
        "--step-name",
        "step2",
        "--command",
        "touch def.txt",
    ])?;
    x(&["step", "dependency", "-s", "step2", "--step", "step1"])?;
    x(&["step", "dependency", "-s", "step2", "--glob", "*.txt"])?;
    x(&["step", "dependency", "-s", "step2", "--directory", "data/"])?;
    x(&[
        "step",
        "dependency",
        "-s",
        "step2",
        "--param",
        "model.conv_units",
    ])?;

    x(&[
        "step",
        "dependency",
        "-s",
        "step2",
        "--regex",
        "requirements.txt:/^tensorflow",
    ])?;

    x(&[
        "step",
        "dependency",
        "-s",
        "step2",
        "--lines",
        "params.yaml::1-20",
    ])?;

    x(&[
        "step",
        "output",
        "-s",
        "step2",
        "--output-metric",
        "metrics.json",
    ])?;
    x(&["step", "output", "-s", "step2", "--output-file", "def.txt"])?;
    x(&[
        "step",
        "output",
        "-s",
        "step2",
        "--output-image",
        "plots/confusion.png",
    ])?;

    let json_export = x(&["export", "--format", "json"])?;
    let ps_json: XvcPipelineSchema = serde_json::from_str(&json_export)?;

    watch!(ps_json);

    assert!(ps_json.name == "default");
    assert!(ps_json.version == 1);
    assert!(ps_json.steps.len() == 2);
    assert!(ps_json.steps[0].name == "step1");
    assert!(ps_json.steps[0].command == "touch abc.txt");
    assert!(ps_json.steps[1].name == "step2");
    assert!(ps_json.steps[1].command == "touch def.txt");
    let deps_json = &ps_json.steps[1].dependencies;
    assert!(deps_json.len() == 6);
    assert!(
        deps_json[0]
            == XvcDependency::Step {
                name: "step1".to_string()
            }
    );

    assert!(
        deps_json[1]
            == XvcDependency::Glob {
                glob: "*.txt".to_string()
            }
    );

    assert!(
        deps_json[2]
            == XvcDependency::Directory {
                path: XvcPath::new(&xvc_root, &xvc_root.absolute_path(), Path::new("data/"))?
            }
    );

    assert!(
        deps_json[3]
            == XvcDependency::Param {
                format: XvcParamFormat::YAML,
                path: XvcPath::new(
                    &xvc_root,
                    &xvc_root.absolute_path(),
                    Path::new("params.yaml")
                )?,
                key: "model.conv_units".to_string(),
            }
    );

    assert!(
        deps_json[4]
            == XvcDependency::Regex {
                path: XvcPath::new(
                    &xvc_root,
                    &xvc_root.absolute_path(),
                    Path::new("requirements.txt"),
                )?,
                regex: "^tensorflow".to_string(),
            }
    );

    assert!(
        deps_json[5]
            == XvcDependency::Lines {
                path: XvcPath::new(
                    &xvc_root,
                    &xvc_root.absolute_path(),
                    Path::new("params.yaml")
                )?,
                begin: 1,
                end: 20
            }
    );

    let outs_json = &ps_json.steps[1].outputs;

    assert!(outs_json.len() == 3);
    assert!(
        outs_json[0]
            == XvcOutput::Metric {
                path: XvcPath::new(
                    &xvc_root,
                    &xvc_root.absolute_path(),
                    Path::new("metrics.json")
                )?,
                format: XvcMetricsFormat::JSON,
            }
    );

    assert!(
        outs_json[1]
            == XvcOutput::File {
                path: XvcPath::new(&xvc_root, &xvc_root.absolute_path(), Path::new("def.txt"))?,
            }
    );

    assert!(
        outs_json[2]
            == XvcOutput::Image {
                path: XvcPath::new(
                    &xvc_root,
                    &xvc_root.absolute_path(),
                    Path::new("plots/confusion.png")
                )?,
            }
    );

    let yaml_export = x(&["export", "--format", "yaml"])?;

    let ps_yaml: XvcPipelineSchema = serde_yaml::from_str(&yaml_export)?;

    assert_eq!(ps_json, ps_yaml);

    clean_up(&xvc_root)
}
