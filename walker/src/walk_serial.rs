//! Serial directory walker without parallelization
//! See [`walk_parallel`] for parallel version.
use std::{ffi::OsString, fs, path::Path, sync::{Arc, Mutex}};

use xvc_logging::{debug, warn, watch, XvcOutputSender};

use crate::{content_to_patterns, directory_list, pattern::MatchResult, update_ignore_rules, IgnoreRules, PathMetadata, Result, WalkOptions};

/// Walk `dir` with `walk_options`, with the given _initial_ `ignore_rules`.
/// Note that ignore rules are expanded with the rules given in the `ignore_filename` in
/// `walk_options`.
/// The result is added to given `res_paths` to reduce the number of memory inits for vec.
///
/// It collects all [`PathMetadata`] of the child paths.
/// Filters paths with the rules found in child directories and the given `ignore_rules`.
pub fn walk_serial(
    output_snd: &XvcOutputSender,
    global_ignore_rules: &str,
    dir: &Path,
    walk_options: &WalkOptions,
) -> Result<(Vec<PathMetadata>, IgnoreRules)> {
    let ignore_rules = IgnoreRules::from_global_patterns(dir,  walk_options.ignore_filename.as_deref(), global_ignore_rules);
    let ignore_rules = Arc::new(Mutex::new(ignore_rules));
    let dir_stack = crossbeam::queue::SegQueue::new();
    let res_paths = Arc::new(Mutex::new(Vec::<PathMetadata>::new()));

    dir_stack.push(dir.to_path_buf());

    let get_child_paths = |dir: &Path| -> Result<Vec<PathMetadata>> {
    Ok(directory_list(dir)?
        .into_iter()
        .filter_map(|pm_res| match pm_res {
            Ok(pm) => Some(pm),
            Err(e) => {
                warn!(output_snd, "{}", e);
                None
            }
        })
        .collect())
    };

    while let Some(dir) = dir_stack.pop() {
        watch!(dir);
        update_ignore_rules(&dir, &ignore_rules.lock().unwrap())?;
        let mut res_paths = res_paths.lock()?; 
        get_child_paths(&dir)?.drain(..).filter_map(|p| 
            match ignore_rules.lock().unwrap().check(p.path.as_ref()) {
                MatchResult::NoMatch | MatchResult::Whitelist => Some(p),
                MatchResult::Ignore => {
                    debug!(output_snd, "Ignored: {:?}", p.path);
                    None
                }
            }).for_each(|p| res_paths.push(p));
    }

    let res_paths: Vec<PathMetadata> = res_paths.lock()?.clone();
    let ignore_rules = ignore_rules.lock()?.clone();

    Ok((res_paths, ignore_rules))
}




