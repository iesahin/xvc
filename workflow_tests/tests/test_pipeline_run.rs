mod common;

use std::{fs, path::Path};

use common::*;
use xvc_config::XvcVerbosity;

use xvc::error::Result;

use subprocess::Exec;

const PIPELINE_YAML: &str = r#"
---
version: 1
name: default
workdir: ""
steps:
  - name: hello
    command: echo "hello xvc!"
    invalidate: Always
    dependencies: []
    outputs: []
  - name: step1
    command: touch abc.txt
    invalidate: ByDependencies
    dependencies: []
    outputs:
      - File:
          path: abc.txt
  - name: step_dep
    command: touch step_dep.txt
    invalidate: ByDependencies
    dependencies:
      - Step:
          name: step1
    outputs:
      - File:
          path: step_dep.txt
  - name: txt_files
    command: find . -name '*.py' > src-files.txt
    invalidate: ByDependencies
    dependencies:
      - Glob:
          glob: "*/*.py"
    outputs:
      - File:
          path: src-files.txt
  - name: training-files
    command: find data/images/train -name '*.png' > training-files.txt
    invalidate: ByDependencies
    dependencies:
      - Directory:
          path: data/images/train
    outputs:
      - File:
          path: training-files.txt
  - name: glob_dep
    command: touch glob_dep.json
    invalidate: ByDependencies
    dependencies:
      - Glob:
          glob: "*.txt"
#      - Directory:
#         path: data
#     - Param:
#         format: YAML
#         path: params.yaml
#         key: model.conv_units
#     - Regex:
#         path: requirements.txt
#         regex: ^tensorflow
    outputs:
      - Metric:
          path: glob_dep.json
          format: JSON
#     - File:
#         path: def.txt
#     - Image:
#         path: plots/confusion.png
  - name: count_training_files
    command: wc -l training-files.txt > num-training-files.txt
    invalidate: ByDependencies
    dependencies:
      - Lines:
          path: training-files.txt
          begin: 1
          end: 1000000
    outputs:
      - File:
          path: num-training-files.txt
"#;

#[test]
fn test_pipeline_run() -> Result<()> {
    let xvc_root = run_in_example_xvc(true)?;
    let x = |cmd: &[&str]| {
        let mut c = vec!["xvc", "pipeline"];
        c.extend(cmd);
        xvc::test_dispatch(Some(&xvc_root), c, XvcVerbosity::Warn)
    };

    let yaml_filename = "pipeline.yaml";
    fs::write(Path::new(yaml_filename), PIPELINE_YAML)?;
    x(&["import", "--overwrite", "--file", yaml_filename])?;
    x(&["run"])?;

    assert!(Path::new("abc.txt").exists());
    assert!(Path::new("src-files.txt").exists());
    assert!(Path::new("training-files.txt").exists());
    assert!(Path::new("num-training-files.txt").exists());

    Exec::shell("rm -f training-files.txt").join()?;
    x(&["run"])?;
    assert!(Path::new("training-files.txt").exists());

    // remove a file from training files and run again
    let file_to_remove = "data/images/train/0/59988.png";
    let training_files_before = fs::read_to_string("training-files.txt")?;
    assert!(training_files_before.contains(file_to_remove));
    Exec::shell(format!("rm -f {file_to_remove}")).join()?;
    x(&["run"])?;
    let training_files_after = fs::read_to_string("training-files.txt")?;
    assert!(!training_files_after.contains(file_to_remove));

    Ok(())
}
