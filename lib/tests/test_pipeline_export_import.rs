mod common;
use common::*;

use std::fs;
use std::path::PathBuf;

use xvc::error::Result;
use xvc_core::XvcVerbosity;

#[test]
fn test_pipeline_export_import() -> Result<()> {
    let xvc_root = run_in_temp_xvc_dir()?;
    fs::write(xvc_root.join(&PathBuf::from("data.txt")), "some data\n")?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["pipeline"];
        c.extend(cmd);
        run_xvc(Some(&xvc_root), &c, XvcVerbosity::Default)
    };

    x(&[
        "step",
        "new",
        "--step-name",
        "step1",
        "--command",
        "echo hello",
    ])?;
    x(&[
        "step",
        "new",
        "--step-name",
        "step2",
        "--command",
        "echo world",
        "--when",
        "always",
    ])?;
    x(&["step", "dependency", "-s", "step2", "--step", "step1"])?;
    x(&["step", "dependency", "-s", "step1", "--file", "data.txt"])?;
    x(&["step", "dependency", "-s", "step1", "--glob", "data*"])?;

    // step list prints step names with their commands
    let step_list = x(&["step", "list"])?;
    assert!(step_list.contains("step1"), "{step_list}");
    assert!(step_list.contains("echo hello"), "{step_list}");
    assert!(step_list.contains("step2"), "{step_list}");

    // ... and only the names with --names-only
    let step_names = x(&["step", "list", "--names-only"])?;
    assert!(step_names.contains("step1"), "{step_names}");
    assert!(!step_names.contains("echo hello"), "{step_names}");

    // step show prints the step definition with its dependencies
    let step_show = x(&["step", "show", "--step-name", "step1"])?;
    assert!(step_show.contains("step1"), "{step_show}");
    assert!(step_show.contains("echo hello"), "{step_show}");
    assert!(step_show.contains("data.txt"), "{step_show}");

    // step update changes the command
    x(&[
        "step",
        "update",
        "--step-name",
        "step1",
        "--command",
        "echo updated",
        "--when",
        "by_dependencies",
    ])?;
    let step_show = x(&["step", "show", "--step-name", "step1"])?;
    assert!(step_show.contains("echo updated"), "{step_show}");
    assert!(!step_show.contains("echo hello"), "{step_show}");

    // export prints to stdout by default (JSON)
    let exported = x(&["export"])?;
    assert!(exported.contains("step1"), "{exported}");
    assert!(exported.contains("echo updated"), "{exported}");

    // export to files; format is inferred from the extension
    x(&["export", "--file", "pipeline.json"])?;
    let json_content = fs::read_to_string(xvc_root.join(&PathBuf::from("pipeline.json")))?;
    assert!(json_content.contains("step1"), "{json_content}");
    x(&["export", "--file", "pipeline.yaml"])?;
    let yaml_content = fs::read_to_string(xvc_root.join(&PathBuf::from("pipeline.yaml")))?;
    assert!(yaml_content.contains("step1"), "{yaml_content}");

    // importing over an existing pipeline requires --overwrite
    let (success, stdout, stderr) = run_xvc_unchecked(
        Some(&xvc_root),
        &["pipeline", "import", "--file", "pipeline.json"],
    )?;
    assert!(
        !success || format!("{stdout}{stderr}").contains("already"),
        "import without --overwrite should not overwrite: {stdout}{stderr}"
    );
    x(&["import", "--file", "pipeline.json", "--overwrite"])?;

    // import into a new pipeline name
    x(&[
        "--pipeline-name",
        "imported",
        "import",
        "--file",
        "pipeline.yaml",
    ])?;
    let pipelines = x(&["list"])?;
    assert!(pipelines.contains("imported"), "{pipelines}");
    let imported_steps = x(&["--pipeline-name", "imported", "step", "list"])?;
    assert!(imported_steps.contains("step1"), "{imported_steps}");
    assert!(imported_steps.contains("step2"), "{imported_steps}");

    // step remove deletes a step from the pipeline
    x(&["step", "remove", "--step-name", "step2"])?;
    let step_list = x(&["step", "list"])?;
    assert!(!step_list.contains("step2"), "{step_list}");
    assert!(step_list.contains("step1"), "{step_list}");

    clean_up(&xvc_root)
}
