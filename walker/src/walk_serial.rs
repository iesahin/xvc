use std::{ffi::OsString, fs, path::Path, sync::{Arc, Mutex}};

use xvc_logging::{debug, warn, watch, XvcOutputSender};

use crate::{content_to_patterns, directory_list, pattern::MatchResult, update_ignore_rules, IgnoreRules, PathMetadata, Pattern, Result, WalkOptions};

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
    let ignore_filename = walk_options.ignore_filename.clone().map(OsString::from);
    let ignore_rules = IgnoreRules::from_global_patterns(dir,  global_ignore_rules);
    let ignore_rules = Arc::new(Mutex::new(ignore_rules));
    let dir_stack = crossbeam::queue::SegQueue::new();
    let res_paths = Arc::new(Mutex::new(Vec::<PathMetadata>::new()));
    let ignore_root = dir.to_path_buf();

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

    let filter_child_paths = |child_paths: &Vec<PathMetadata>| -> Result<()> {
        for child_path in child_paths {
            watch!(child_path.path);
            let ignore_res = ignore_rules.lock()?.check(child_path.path.as_ref());
            match ignore_res {
                MatchResult::NoMatch | MatchResult::Whitelist => {
                    if child_path.metadata.is_dir() {
                        if walk_options.include_dirs {
                            res_paths.lock()?.push(child_path.clone());
                        }
                        dir_stack.push(child_path.path.clone());
                    } else {
                        res_paths.lock()?.push(child_path.clone());
                    }
                }
                // We can return anyhow! error here to notice the user that the path is ignored
                MatchResult::Ignore => {
                    debug!(output_snd, "Ignored: {:?}", child_path.path);
                }
            }
            watch!(child_path);
        }
        Ok(())
    };

    while let Some(dir) = dir_stack.pop() {
        watch!(dir);
        update_ignore_rules(&ignore_filename, &dir, &ignore_rules.lock().unwrap())?;
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




