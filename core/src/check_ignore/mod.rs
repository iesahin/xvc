//! xvc check ignore CLI handling module
use crate::error::Result;
use crate::util::expand_globs_to_paths;
use crate::util::xvcignore::COMMON_IGNORE_PATTERNS;
use crate::{XvcPath, XvcRoot, XVCIGNORE_FILENAME};
use clap::Parser;

use log::trace;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use xvc_config::{UpdateFromXvcConfig, XvcConfig};
use xvc_logging::{output, XvcOutputSender};
use xvc_walker::{build_ignore_rules, check_ignore, IgnoreRules, MatchResult, WalkOptions};

// DIFFERENCES from DVC
// merged --all and --details, they are the same now

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command()]
/// Check whether xvcignore files ignore a path in an XVC repository
pub struct CheckIgnoreCLI {
    #[arg(short, long, alias = "all")]
    /// Show the exclude patterns along with each target path.
    /// A series of lines are printed in this format: <path/to/.xvcignore>:<line_num>:<pattern> <target_path>
    details: bool,
    #[arg(
        long,
        default_value = XVCIGNORE_FILENAME,
    )]
    /// Filename that contains ignore rules
    ///
    /// This can be set to .gitignore to test whether Git and Xvc work the same way.
    ignore_filename: String,
    #[arg(short, long)]
    /// Include the target paths which don’t match any pattern in the --details list.
    /// All fields in each line, except for <target_path>, will be empty. Has no effect without --details.
    non_matching: bool,
    #[arg()]
    /// Targets to check.
    /// If no targets are provided, they are read from stdin.
    targets: Vec<String>,
}

impl UpdateFromXvcConfig for CheckIgnoreCLI {
    fn update_from_conf(self, conf: &XvcConfig) -> xvc_config::error::Result<Box<Self>> {
        let details = self.details || conf.get_bool("check_ignore.details")?.option;
        let non_matching = self.non_matching || conf.get_bool("check_ignore.non_matching")?.option;
        let ignore_filename = self.ignore_filename.clone();
        Ok(Box::new(Self {
            details,
            non_matching,
            ignore_filename,
            targets: self.targets.clone(),
        }))
    }
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
    let opts = opts.update_from_conf(conf)?;

    let current_dir = conf.current_dir()?;
    let walk_options = WalkOptions {
        ignore_filename: Some(opts.ignore_filename.clone()),
        include_dirs: true,
    };
    let initial_rules = IgnoreRules::try_from_patterns(xvc_root, COMMON_IGNORE_PATTERNS)?;
    let ignore_rules = build_ignore_rules(
        initial_rules,
        current_dir,
        &walk_options.ignore_filename.unwrap_or_default(),
    )?;
    if !opts.targets.is_empty() {
        let path_bufs = expand_globs_to_paths(current_dir, &opts.targets)?;

        let mut xvc_paths = Vec::<XvcPath>::new();
        for p in path_bufs {
            xvc_paths.push(XvcPath::new(xvc_root, current_dir, &p)?);
        }
        check_ignore_paths(xvc_root, &opts, &ignore_rules, &xvc_paths)
    } else {
        check_ignore_stdin(input, output_snd, xvc_root, &opts, &ignore_rules)
    }
}

fn check_ignore_stdin<R: BufRead>(
    input: R,
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    opts: &CheckIgnoreCLI,
    ignore_rules: &IgnoreRules,
) -> Result<()> {
    let conf = xvc_root.config();
    let current_dir = conf.current_dir()?;
    let mut buffer = String::new();
    for line in input.lines().flatten() {
        let xvc_path = XvcPath::new(xvc_root, current_dir, &PathBuf::from(line))?;
        let absolute_path = xvc_path.to_absolute_path(xvc_root);
        let res = check_ignore_line(ignore_rules, &absolute_path, opts.non_matching);
        if !res.trim().is_empty() {
            output!(output_snd, "{}", res);
        }
        buffer.clear();
    }
    Ok(())
}

fn check_ignore_paths(
    xvc_root: &XvcRoot,
    opts: &CheckIgnoreCLI,
    ignore_rules: &IgnoreRules,
    xvc_paths: &[XvcPath],
) -> Result<()> {
    for path in xvc_paths {
        let absolute_path = path.to_absolute_path(xvc_root);
        let output = check_ignore_line(ignore_rules, &absolute_path, opts.non_matching);
        trace!("output: {}", output);
        println!("{}", output)
    }
    Ok(())
}

/// Check whether the records match to the full_path. It reports the details if
/// set true. Non_matching inverts the reporting.

fn check_ignore_line(
    ignore_rules: &IgnoreRules,
    absolute_path: &Path,
    show_no_match: bool,
) -> String {
    match check_ignore(ignore_rules, absolute_path) {
        MatchResult::NoMatch => {
            if show_no_match {
                format!("[NO MATCH] {}", absolute_path.to_string_lossy())
            } else {
                String::new()
            }
        }
        MatchResult::Ignore => {
            format!("[IGNORE] {}", absolute_path.to_string_lossy())
        }

        MatchResult::Whitelist => {
            format!("[WHITELIST] {}", absolute_path.to_string_lossy())
        }
    }
}
