mod common;

use std::{fs, path::Path};

use common::*;
use log::LevelFilter;
use xvc::error::Result;
use xvc_config::XvcVerbosity;
use xvc_logging::setup_logging;

const PIPELINE_YAML: &str = r#"
---
version: 1
name: default
workdir: ""
steps:
  - version: 1
    name: hello
    command: echo "hello xvc!"
    invalidate: Always
    dependencies: []
    outputs: []
  - version: 1
    name: step1
    command: touch abc.txt
    invalidate: ByDependencies
    dependencies: []
    outputs:
      - File:
          path: abc.txt
  - version: 1
    name: step_dep
    command: touch step_dep.txt
    invalidate: ByDependencies
    dependencies:
      - Step:
          name: step1
    outputs:
      - File:
          path: step_dep.txt
  - version: 1
    name: txt_files
    command: find . -name '*.py' > src-files.txt
    invalidate: ByDependencies
    dependencies:
      - Glob:
          glob: "*/*.py"
    outputs:
      - File:
          path: src-files.txt
  - version: 1
    name: training-files
    command: find data/images/train -name '*.png' > training-files.txt
    invalidate: ByDependencies
    dependencies:
      - Directory:
          path: data/images/train
    outputs:
      - File:
          path: training-files.txt
  - version: 1
    name: glob_dep
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
  - version: 1
    name: count_training_files
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
fn test_pipeline_dag() -> Result<()> {
    test_logging(LevelFilter::Trace);
    let xvc_root = run_in_example_xvc(true)?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["pipeline"];
        c.extend(cmd);
        common::run_xvc(Some(&xvc_root), &c, XvcVerbosity::Warn)
    };

    let yaml_filename = "pipeline.yaml";
    fs::write(Path::new(yaml_filename), PIPELINE_YAML)?;
    x(&["import", "--overwrite", "--file", yaml_filename])?;
    x(&["dag", "--file", "dag.out"])?;

    clean_up(&xvc_root)
}
