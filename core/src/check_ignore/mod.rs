//! xvc check ignore CLI handling module
use crate::error::Result;
use crate::util::xvcignore::COMMON_IGNORE_PATTERNS;
use crate::{XvcPath, XvcRoot, XVCIGNORE_FILENAME};
use clap::Parser;

use log::trace;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use xvc_logging::{output, watch, XvcOutputSender};
use xvc_walker::{build_ignore_patterns, IgnoreRules, MatchResult, WalkOptions};

// DIFFERENCES from DVC
// merged --all and --details, they are the same now

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command()]
/// Check whether xvcignore files ignore a path in an XVC repository
pub struct CheckIgnoreCLI {
    #[arg(
        long,
        default_value = XVCIGNORE_FILENAME,
    )]
    /// Filename that contains ignore rules
    ///
    /// This can be set to .gitignore to test whether Git and Xvc work the same way.
    ignore_filename: String,

    #[arg()]
    /// Targets to check.
    /// If no targets are provided, they are read from stdin.
    targets: Vec<String>,
}

/// # `xvc check_ignore`
///
/// Check whether paths are ignored by Xvc or not.
///
/// ## CLI usage
///
/// ## API Usage
///
/// ### Arguments
///
/// ## Errors
pub fn cmd_check_ignore<R: BufRead>(
    input: R,
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    opts: CheckIgnoreCLI,
) -> Result<()> {
    let conf = xvc_root.config();

    let current_dir = conf.current_dir()?;
    let walk_options = WalkOptions {
        ignore_filename: Some(opts.ignore_filename.clone()),
        include_dirs: true,
    };

    let ignore_rules = build_ignore_patterns(
        COMMON_IGNORE_PATTERNS,
        xvc_root,
        &walk_options.ignore_filename.unwrap_or_default(),
    )?;

    watch!(ignore_rules);
    watch!(opts.targets);

    if !opts.targets.is_empty() {
        let xvc_paths = opts
            .targets
            .iter()
            .map(|p| XvcPath::new(xvc_root, current_dir, &PathBuf::from(p)))
            .collect::<Result<Vec<XvcPath>>>()?;
        watch!(xvc_paths);
        check_ignore_paths(xvc_root, &ignore_rules, &xvc_paths)
    } else {
        check_ignore_stdin(input, output_snd, xvc_root, &ignore_rules)
    }
}

fn check_ignore_stdin<R: BufRead>(
    input: R,
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    ignore_rules: &IgnoreRules,
) -> Result<()> {
    let conf = xvc_root.config();
    let current_dir = conf.current_dir()?;
    let mut buffer = String::new();
    let lines_iter = input.lines();
    lines_iter
        .map_while(|line| {
            if let Ok(line) = line {
                XvcPath::new(xvc_root, current_dir, &PathBuf::from(line)).ok()
            } else {
                None
            }
        })
        .for_each(|xvc_path| {
            let absolute_path = xvc_path.to_absolute_path(xvc_root);
            let res = check_ignore_line(ignore_rules, &absolute_path);
            if !res.trim().is_empty() {
                output!(output_snd, "{}", res);
            }
            buffer.clear();
        });
    Ok(())
}

fn check_ignore_paths(
    xvc_root: &XvcRoot,
    ignore_rules: &IgnoreRules,
    xvc_paths: &[XvcPath],
) -> Result<()> {
    for path in xvc_paths {
        let absolute_path = path.to_absolute_path(xvc_root);
        let output = check_ignore_line(ignore_rules, &absolute_path);
        trace!("output: {}", output);
        println!("{}", output)
    }
    Ok(())
}

/// Check whether the records match to the full_path. It reports the details if
/// set true. Non_matching inverts the reporting.

fn check_ignore_line(ignore_rules: &IgnoreRules, absolute_path: &Path) -> String {
    match ignore_rules.check(absolute_path) {
        MatchResult::NoMatch => {
            format!("[NO MATCH] {}", absolute_path.to_string_lossy())
        }
        MatchResult::Ignore => {
            format!("[IGNORE] {}", absolute_path.to_string_lossy())
        }

        MatchResult::Whitelist => {
            format!("[WHITELIST] {}", absolute_path.to_string_lossy())
        }
    }
}
