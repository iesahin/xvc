mod common;
use common::*;

use std::fs;
use std::path::PathBuf;

use xvc::error::Result;
use xvc_core::XvcRoot;
use xvc_core::XvcVerbosity;

const PEOPLE: &str = "Jack,20\nMary,30\nJill,25\nTom,40\n";
const PEOPLE_CHANGED: &str = "Jack,20\nJune,35\nJill,25\nTom,40\n";

/// Steps append a line to their own log file on each invocation, so the
/// number of lines in a log shows how many times the step actually ran.
fn log_lines(xvc_root: &XvcRoot, name: &str) -> usize {
    let path = xvc_root.join(&PathBuf::from(name));
    if !path.exists() {
        return 0;
    }
    fs::read_to_string(path)
        .map(|s| s.lines().count())
        .unwrap_or(0)
}

#[test]
fn test_pipeline_run_deps() -> Result<()> {
    let xvc_root = run_in_temp_xvc_dir()?;
    let p = |s: &str| xvc_root.join(&PathBuf::from(s));
    fs::write(p("people.csv"), PEOPLE)?;
    fs::write(p("generic-input.txt"), "v1\n")?;
    let x = |cmd: &[&str]| -> Result<String> {
        let mut c = vec!["pipeline"];
        c.extend(cmd);
        run_xvc(Some(&xvc_root), &c, XvcVerbosity::Default)
    };

    let new_step = |name: &str, log: &str| -> Vec<String> {
        vec![
            "step".into(),
            "new".into(),
            "--step-name".into(),
            name.into(),
            "--command".into(),
            format!("echo ran >> {log}"),
        ]
    };
    let steps = [
        ("step_generic", "generic.log"),
        ("step_lines", "lines.log"),
        ("step_line_items", "line_items.log"),
        ("step_regex", "regex.log"),
        ("step_regex_items", "regex_items.log"),
    ];
    for (name, log) in &steps {
        let args = new_step(name, log);
        x(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>())?;
    }

    x(&[
        "step",
        "dependency",
        "-s",
        "step_generic",
        "--generic",
        "cat generic-input.txt",
    ])?;
    x(&[
        "step",
        "dependency",
        "-s",
        "step_lines",
        "--lines",
        "people.csv::1-4",
    ])?;
    x(&[
        "step",
        "dependency",
        "-s",
        "step_line_items",
        "--line-items",
        "people.csv::1-4",
    ])?;
    x(&[
        "step",
        "dependency",
        "-s",
        "step_regex",
        "--regex",
        "people.csv:/^J",
    ])?;
    x(&[
        "step",
        "dependency",
        "-s",
        "step_regex_items",
        "--regex-items",
        "people.csv:/^J",
    ])?;

    // First run: every step runs once
    x(&["run"])?;
    for (_, log) in &steps {
        assert_eq!(log_lines(&xvc_root, log), 1, "{log} after first run");
    }

    // Second run without changes: every step is skipped
    x(&["run"])?;
    for (_, log) in &steps {
        assert_eq!(log_lines(&xvc_root, log), 1, "{log} after unchanged run");
    }

    // Change the dependencies: line 2 changes (within lines 1-4 and matching ^J)
    // and the generic command output changes
    fs::write(p("people.csv"), PEOPLE_CHANGED)?;
    fs::write(p("generic-input.txt"), "v2\n")?;
    x(&["run"])?;
    for (_, log) in &steps {
        assert_eq!(log_lines(&xvc_root, log), 2, "{log} after changed run");
    }

    clean_up(&xvc_root)
}
