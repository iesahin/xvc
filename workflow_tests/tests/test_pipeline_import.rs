mod common;

use std::{iter::zip, path::Path};

use common::*;
use xvc::error::Result;
use xvc_config::XvcVerbosity;
use xvc_core::XvcPath;
use xvc_ecs::{R11Store, R1NStore, XvcEntity, XvcStore};
use xvc_logging::watch;
use xvc_pipeline::{
    XvcDependency, XvcMetricsFormat, XvcOutput, XvcParamFormat, XvcPipeline, XvcStep,
    XvcStepCommand,
};

#[test]
fn test_pipeline_import() -> Result<()> {
    let xvc_root = run_in_example_xvc(true)?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["xvc", "pipeline"];
        c.extend(cmd);
        xvc::test_dispatch(Some(&xvc_root), c, XvcVerbosity::Warn)
    };

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

    let json_filename = "pipeline.json";
    x(&["export", "--file", json_filename])?;
    x(&["import", "--name", "json_pipeline", "--file", json_filename])?;

    let yaml_filename = "pipeline.yaml";
    x(&["export", "--file", yaml_filename])?;
    x(&["import", "--name", "yaml_pipeline", "--file", yaml_filename])?;

    // All 3 pipelines should be identical now

    let pipelines: XvcStore<XvcPipeline> = xvc_root.load_store()?;

    assert!(pipelines.len() == 3);

    for (pipeline, pipeline_name) in zip(
        pipelines.values(),
        ["default", "json_pipeline", "yaml_pipeline"],
    ) {
        assert_eq!(
            *pipeline,
            XvcPipeline {
                name: pipeline_name.to_string()
            }
        )
    }

    let all_steps: R1NStore<XvcPipeline, XvcStep> = xvc_root.load_r1nstore()?;
    let all_commands: R11Store<XvcStep, XvcStepCommand> = xvc_root.load_r11store()?;
    let all_deps: R1NStore<XvcStep, XvcDependency> = xvc_root.load_r1nstore()?;
    let all_outs: R1NStore<XvcStep, XvcOutput> = xvc_root.load_r1nstore()?;

    for (pipeline_e, _) in pipelines.iter() {
        let steps = all_steps.children_of(pipeline_e)?;
        watch!(steps);
        assert_eq!(steps.len(), 2);
        for (step, step_name) in zip(steps.values(), ["step1", "step2"]) {
            assert_eq!(
                *step,
                XvcStep {
                    name: step_name.to_string()
                }
            );
        }

        for (step_e, step_command) in zip(steps.keys(), ["touch abc.txt", "touch def.txt"]) {
            watch!(step_e);
            let command = all_commands
                .left_to_right(step_e)
                .ok_or(xvc_ecs::error::Error::CannotFindKeyInStore {
                    key: (*step_e).into(),
                })?
                .1;
            watch!(command);
            assert_eq!(
                *command,
                XvcStepCommand {
                    command: step_command.to_string()
                }
            );
        }
        let step_v: Vec<(&XvcEntity, &XvcStep)> = steps.iter().collect();

        assert_eq!(
            *step_v[0].1,
            XvcStep {
                name: "step1".to_string()
            }
        );

        assert_eq!(
            *step_v[1].1,
            XvcStep {
                name: "step2".to_string()
            }
        );

        watch!(step_v[0]);
        assert_eq!(all_deps.children_of(&step_v[0].0)?.len(), 0);
        watch!(step_v[1]);
        assert_eq!(all_deps.children_of(&step_v[1].0)?.len(), 6);
        let deps2_s = all_deps.children_of(&step_v[1].0)?;
        watch!(deps2_s.len());
        let deps2: Vec<&XvcDependency> = deps2_s.values().collect();
        watch!(deps2);
        watch!(deps2[0]);
        assert!(
            *deps2[0]
                == XvcDependency::Step {
                    name: "step1".to_string()
                }
        );
        watch!(deps2[1]);
        assert!(
            *deps2[1]
                == XvcDependency::Glob {
                    glob: "*.txt".to_string()
                }
        );

        assert!(
            *deps2[2]
                == XvcDependency::Directory {
                    path: XvcPath::new(&xvc_root, xvc_root.absolute_path(), Path::new("data/"))?
                }
        );

        assert!(
            *deps2[3]
                == XvcDependency::Param {
                    format: XvcParamFormat::YAML,
                    path: XvcPath::new(
                        &xvc_root,
                        xvc_root.absolute_path(),
                        Path::new("params.yaml")
                    )?,
                    key: "model.conv_units".to_string(),
                }
        );

        assert!(
            *deps2[4]
                == XvcDependency::Regex {
                    path: XvcPath::new(
                        &xvc_root,
                        xvc_root.absolute_path(),
                        Path::new("requirements.txt"),
                    )?,
                    regex: "^tensorflow".to_string(),
                }
        );

        assert!(
            *deps2[5]
                == XvcDependency::Lines {
                    path: XvcPath::new(
                        &xvc_root,
                        xvc_root.absolute_path(),
                        Path::new("params.yaml")
                    )?,
                    begin: 1,
                    end: 20
                }
        );
        let outs2_v = all_outs.children_of(&step_v[1].0)?;
        watch!(outs2_v);
        let outs2: Vec<&XvcOutput> = outs2_v.values().collect();
        watch!(outs2);
        assert!(outs2.len() == 3);
        watch!(outs2[0]);
        assert!(
            *outs2[0]
                == XvcOutput::Metric {
                    path: XvcPath::new(
                        &xvc_root,
                        xvc_root.absolute_path(),
                        Path::new("metrics.json")
                    )?,
                    format: XvcMetricsFormat::JSON,
                }
        );
        watch!(outs2[1]);
        assert!(
            *outs2[1]
                == XvcOutput::File {
                    path: XvcPath::new(&xvc_root, xvc_root.absolute_path(), Path::new("def.txt"))?,
                }
        );
        watch!(outs2[2]);
        assert!(
            *outs2[2]
                == XvcOutput::Image {
                    path: XvcPath::new(
                        &xvc_root,
                        xvc_root.absolute_path(),
                        Path::new("plots/confusion.png")
                    )?,
                }
        );
    }

    Ok(())
}
