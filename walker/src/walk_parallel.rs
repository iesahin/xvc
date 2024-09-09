use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use crossbeam::queue::SegQueue;
use crossbeam_channel::Sender;
use xvc_logging::{uwr, watch};

use crate::{
    directory_list, update_ignore_rules, IgnoreRules, MatchResult, PathMetadata, Result,
    SharedIgnoreRules, WalkOptions, MAX_THREADS_PARALLEL_WALK,
};

fn walk_parallel_inner(
    ignore_rules: SharedIgnoreRules,
    dir: &Path,
    walk_options: WalkOptions,
    path_sender: Sender<Result<PathMetadata>>,
) -> Result<Vec<PathMetadata>> {
    update_ignore_rules(dir, &ignore_rules.write().unwrap())?;

    Ok(directory_list(dir)?
        .drain(..)
        .filter_map(|pm_res| match pm_res {
            Ok(pm) => {
                watch!(pm);
                Some(pm)
            }
            Err(e) => {
                path_sender
                    .send(Err(e))
                    .expect("Channel error in walk_parallel");
                None
            }
        })
        .filter_map(|pm| {
            let ignore_res = ignore_rules.read().unwrap().check(pm.path.as_ref());
            watch!(ignore_res);
            match ignore_res {
                MatchResult::NoMatch | MatchResult::Whitelist => {
                    // If the path is a file, don't send it to caller, just send it to the channel.
                    // If the path is a directory, send it to the channel if `include_dirs` is true.
                    // The caller expects a list of directories to recurse into.

                    if pm.metadata.is_file() || pm.metadata.is_symlink() {
                        path_sender
                            .send(Ok(pm.clone()))
                            .expect("Channel error in walk_parallel");
                        None
                    } else if pm.metadata.is_dir() {
                        path_sender
                            .send(Ok(pm.clone()))
                            .expect("Channel error in walk_parallel");

                        if walk_options.include_dirs {
                            Some(pm)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }

                MatchResult::Ignore => {
                    watch!(pm.path);
                    None
                }
            }
        })
        .collect::<Vec<PathMetadata>>())
}

/// Walk all child paths under `dir` and send non-ignored paths to `path_sender`.
/// Newly found ignore rules are sent through `ignore_sender`.
/// The ignore file name (`.xvcignore`, `.gitignore`, `.ignore`, ...) is set by `walk_options`.
///
/// It lists elements of a directory, then creates a new crossbeam scope for each child directory and
/// calls itself recursively. It may not be feasible for small directories to create threads.
pub fn walk_parallel(
    ignore_rules: SharedIgnoreRules,
    dir: &Path,
    walk_options: WalkOptions,
    path_sender: Sender<Result<PathMetadata>>,
) -> Result<()> {
    let dir_queue = Arc::new(SegQueue::<PathMetadata>::new());

    let child_dirs = walk_parallel_inner(
        ignore_rules.clone(),
        dir,
        walk_options.clone(),
        path_sender.clone(),
    )?;
    watch!(child_dirs);

    child_dirs.into_iter().for_each(|pm| {
        dir_queue.push(pm);
    });

    watch!(dir_queue);

    if dir_queue.is_empty() {
        return Ok(());
    }

    crossbeam::scope(|s| {
        for thread_i in 0..MAX_THREADS_PARALLEL_WALK {
            let path_sender = path_sender.clone();
            let walk_options = walk_options.clone();
            let ignore_rules = ignore_rules.clone();
            let dir_queue = dir_queue.clone();

            s.spawn(move |_| {
                watch!(path_sender);
                while let Some(pm) = dir_queue.pop() {
                    let child_dirs = walk_parallel_inner(
                        ignore_rules.clone(),
                        &pm.path,
                        walk_options.clone(),
                        path_sender.clone(),
                    )
                    .unwrap();

                    for child_dir in child_dirs {
                        dir_queue.push(child_dir);
                    }
                }
                watch!("End of thread {}", thread_i);
            });
        }
    })
    .expect("Error in crossbeam scope in walk_parallel");

    watch!("End of walk_parallel");

    Ok(())
}
