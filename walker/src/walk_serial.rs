//! Serial directory walker without parallelization
//! See [`walk_parallel`] for parallel version.
use std::path::{Path, PathBuf};

use xvc_logging::{debug, error, warn, XvcOutputSender};

use crate::{
    build_ignore_patterns, directory_list, pattern::MatchResult, update_ignore_rules, IgnoreRules,
    PathMetadata, Result, WalkOptions,
};

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
    let ignore_rules = IgnoreRules::from_global_patterns(
        dir,
        walk_options.ignore_filename.as_deref(),
        global_ignore_rules,
    );

    let mut dir_stack = Vec::new();

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

    let mut res_paths = Vec::new();
    while let Some(dir) = dir_stack.pop() {
        // TODO: Keep ignore rules in a single file in the root. Most of the time, we don't need to
        // read the ignore rules in all over the repository.
        update_ignore_rules(&dir, &ignore_rules)?;

        res_paths.extend(get_child_paths(&dir)?.drain(..).filter_map(|p| {
            let ignore_result = ignore_rules.check(p.path.as_ref());
            match ignore_result {
                MatchResult::NoMatch | MatchResult::Whitelist => {
                    if p.metadata.is_dir() {
                        dir_stack.push(p.path.clone());
                    }
                    Some(p)
                }
                MatchResult::Ignore => {
                    debug!(output_snd, "Ignored: {:?}", p.path);
                    None
                }
            }
        }));
    }

    Ok((res_paths, ignore_rules))
}

/// Return path metadata only for the listed files to avoid recursing directories
pub fn path_metadata_map_from_file_targets(
    output_snd: &XvcOutputSender,
    global_ignore_rules: &str,
    ignore_root: &Path,
    targets: Vec<String>,
    walk_options: &WalkOptions,
) -> Result<(Vec<PathMetadata>, IgnoreRules)> {
    let ignore_filename = walk_options.ignore_filename.as_deref();
    let ignore_rules = ignore_filename
        .map(|ignore_filename| {
            build_ignore_patterns(global_ignore_rules, ignore_root, ignore_filename)
        })
        .unwrap_or_else(|| {
            Ok(IgnoreRules::from_global_patterns(
                ignore_root,
                None,
                global_ignore_rules,
            ))
        })?;

    let mut res_paths = Vec::new();

    res_paths.extend(targets.into_iter().filter_map(|target| {
        let path: PathBuf = target.into();
        let ignore_result = ignore_rules.check(&path);
        match ignore_result {
            MatchResult::NoMatch | MatchResult::Whitelist => {
                let md_res = path.metadata();
                match md_res {
                    Ok(metadata) => Some(PathMetadata { path, metadata }),
                    Err(e) => {
                        error!(output_snd, "{}", e);
                        None
                    }
                }
            }

            MatchResult::Ignore => {
                warn!(output_snd, "Ignored: {:?}", path);
                None
            }
        }
    }));

    Ok((res_paths, ignore_rules))
}
