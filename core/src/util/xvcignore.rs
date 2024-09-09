//! Walkers with Xvc-specific ignore rules
use crate::types::{xvcpath::XvcPath, xvcroot::XvcRoot};

use crate::{XvcMetadata, XvcPathMetadataMap, CHANNEL_BOUND, XVCIGNORE_FILENAME};

use crate::error::{Error, Result};
use crossbeam_channel::{bounded, Sender};

use std::sync::{Arc, RwLock};
use std::thread;
use xvc_logging::{warn, watch, XvcOutputSender};
use xvc_walker::{self, IgnoreRules, PathMetadata, WalkOptions};
use xvc_walker::{Result as XvcWalkerResult, SharedIgnoreRules};

/// We ignore `.git` directories even we are not using `.git`
pub const COMMON_IGNORE_PATTERNS: &str = ".xvc\n.git\n";

/// Get all files and the generated `.xvcignore` rules in the repository.
/// It's usually not required to run the traversal serially, use `walk_parallel` instead.
///
/// This function defines initial ignore rules (filename: `.xvcignore`).
/// Then runs [xvc_walker::walk_serial].
/// It converts the `walk_serial` result from [xvc_walker::PathMetadataMap] to [XvcPathMetadataMap].
///
/// ## Arguments
///
/// - `xvc_root`: The root structure for Xvc
/// - `include_dirs`: Whether to include directories themselves.
///   If `false`, only the actual files in the repository are listed.
///
/// ## Returns
///
/// - `XvcPathMetadataMap`: A hash map of files. Keys are [XvcPath], values are their
///   [XvcMetadata].
/// - `IgnoreRules`: The rules that were produced while reading the directories.
///   This is returned here to prevent a second traversal for ignores.
pub fn walk_serial(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    include_dirs: bool,
) -> Result<(XvcPathMetadataMap, IgnoreRules)> {
    let walk_options = WalkOptions {
        ignore_filename: Some(XVCIGNORE_FILENAME.to_owned()),
        include_dirs,
    };
    let (res_paths, ignore_rules) = xvc_walker::walk_serial::walk_serial(
        output_snd,
        COMMON_IGNORE_PATTERNS,
        xvc_root,
        &walk_options,
    )?;
    let pmp: XvcPathMetadataMap = res_paths
        .iter()
        .filter_map(|pm| {
            let md = XvcMetadata::from(&pm.metadata);
            //
            let rxp = XvcPath::new(xvc_root, xvc_root, &pm.path);
            //
            match rxp {
                Ok(xvc_path) => Some((xvc_path, md)),
                Err(e) => {
                    warn!(output_snd, "{:?}", e);
                    None
                }
            }
        })
        .collect();
    Ok((pmp, ignore_rules))
}

/// Get all files and the generated `.xvcignore` rules in the repository.
///
/// This function defines initial ignore rules (filename: `.xvcignore`).
/// It creates channels to communicate with `xvc_walker::walk_parallel`.
/// Then runs [xvc_walker::walk_parallel].
/// It converts the channel results from [xvc_walker::PathMetadataMap] to [XvcPathMetadataMap].
/// It merges IgnoreRules obtained from different directories into one.
///
/// ## Arguments
///
/// - `xvc_root`: The root structure for Xvc
/// - `include_dirs`: Whether to include directories themselves.
///    If `false`, only the actual files in the repository are listed.
///
/// ## Returns
///
/// - `XvcPathMetadataMap`: A hash map of files. Keys are [XvcPath], values are their
///   [XvcMetadata].
/// - `IgnoreRules`: The rules that were produced while reading the directories.
///   This is returned here to prevent a second traversal for ignores.
pub fn walk_parallel(
    xvc_root: &XvcRoot,
    global_ignore_rules: &str,
    include_dirs: bool,
) -> Result<(XvcPathMetadataMap, SharedIgnoreRules)> {
    let (sender, receiver) = bounded::<(XvcPath, XvcMetadata)>(CHANNEL_BOUND);
    watch!(sender);
    let ignore_rules = Arc::new(RwLock::new(IgnoreRules::from_global_patterns(
        xvc_root,
        Some(XVCIGNORE_FILENAME),
        global_ignore_rules,
    )));

    watch!(ignore_rules);

    walk_channel(xvc_root, ignore_rules.clone(), include_dirs, sender)?;

    let pmm = thread::spawn(move || {
        let mut pmm = XvcPathMetadataMap::new();
        for (path, md) in receiver.iter() {
            watch!(path);
            pmm.insert(path, md);
        }
        pmm
    })
    .join()
    .map_err(Error::from)?;

    Ok((pmm, ignore_rules))
}

/// Sends paths under `xvc_root`, ignoring `initial_patterns` and loading patterns from
/// `ignore_filenames` in all child directories.
///
/// This function creates initial ignore structures and runs `xvc_walker::walk_parallel`.
/// It harvests the channel in another thread to convert the results from [PathMetadata] to
/// `(XvcPath, XvcMetadata)`
///
/// TODO: This function should employ [XvcPathMetadata] struct instead of tuple.
///
/// # Arguments
///  - `xvc_root`: The repository root
///  - `initial_patterns`: A set of patterns arranged similar to an `.xvcignore` (`.gitignore`) content.
///  - `ignore_filename`: The name of the ignore files to be loaded for ignore rules.
///    (ex: `.xvcignore`, `.ignore`, or `.gitignore`)
///  - `include_dirs`: Whether to send directory records themselves.
///     If `false`, only the files in directories are sent.
///  - `xpm_upstream`: The channel this function sends the paths and metadata.
///  - `ignore_upstream`: The channel this function sends found ignore rules.
///    These ignore rules are only built from the directories they are found.
///    All these rules should be merged by the receiver using [merge_ignores].
///
///  Note that `xpm_upstream` and `ignore_upstream` may return in different frequencies.
///  Not all directories have ignore files.
///  The reason ignore rules are sent via another channel is not to block the traversal while
///  building new ignore rules.
///  Semantically it doesn't change anything, but most of the ignore rules returned from the
///  channel will have overlapping rules.
///  These overlapping rules can be merged with [merge_ignores].
pub fn walk_channel(
    xvc_root: &XvcRoot,
    ignore_rules: SharedIgnoreRules,
    include_dirs: bool,
    xpm_upstream: Sender<(XvcPath, XvcMetadata)>,
) -> Result<()> {
    let walk_options = WalkOptions {
        ignore_filename: ignore_rules.read()?.ignore_filename.clone(),
        include_dirs,
    };
    let (path_sender, path_receiver) = bounded::<XvcWalkerResult<PathMetadata>>(CHANNEL_BOUND);

    xvc_walker::walk_parallel::walk_parallel(ignore_rules, xvc_root, walk_options, path_sender)?;

    crossbeam::scope(|s| {
        s.spawn(|_| {
            for result in path_receiver {
                match result {
                    Ok(pm) => {
                        let md: XvcMetadata = XvcMetadata::from(pm.metadata);
                        // watch!(&md);
                        let rxp = XvcPath::new(xvc_root, xvc_root.absolute_path(), &pm.path);
                        // watch!(&rxp);
                        match rxp {
                            Ok(xvc_path) => match xpm_upstream.send((xvc_path, md)) {
                                Ok(_) => {}
                                Err(err) => {
                                    warn!("{:?}", err);
                                }
                            },
                            Err(e) => {
                                e.warn();
                            }
                        }
                    }
                    Err(e) => {
                        e.warn();
                    }
                }
            }
        });
    })
    .map_err(Error::from)?;
    Ok(())
}
