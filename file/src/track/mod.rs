//! Home of the `xvc file track` command and related functionality.
//!
//! - [`cmd_track`] is the entry point for the `xvc file track` command.
//! - [`TrackCLI`] is the command line interface
//! - [`update_file_gitignores`] and [`update_dir_gitignores`] are functions to
//!   update `.gitignore` files with the tracked paths.
//! - [`carry_in`] is a specialized carry in function for `xvc file track`.

use clap_complete::ArgValueCompleter;
use derive_more::From;
use xvc_core::util::completer::strum_variants_completer;

use std::collections::HashSet;

use xvc_core::util::git::build_gitignore;
use xvc_core::{FromConfigKey, XvcConfigResult};
use xvc_core::{UpdateFromXvcConfig, XvcConfig};

use xvc_core::XvcOutputSender;
use xvc_core::{
    ContentDigest, HashAlgorithm, TextOrBinary, XvcCachePath, XvcFileType, XvcMetadata, XvcRoot,
};

use crate::carry_in::carry_in;
use crate::common::compare::{
    diff_content_digest, diff_recheck_method, diff_text_or_binary, diff_xvc_path_metadata,
};
use crate::common::gitignore::{update_dir_gitignores, update_file_gitignores};
use crate::common::{targets_from_disk, update_store_records, FileTextOrBinary};
use crate::error::Result;

use clap::Parser;
use std::path::PathBuf;

use xvc_core::RecheckMethod;
use xvc_core::XvcPath;
use xvc_core::{HStore, XvcEntity};

/// Add files for tracking with Xvc
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, From, Parser)]
#[command(rename_all = "kebab-case")]
pub struct TrackCLI {
    /// How to track the file contents in cache: One of copy, symlink, hardlink, reflink.
    ///
    /// Note: Reflink uses copy if the underlying file system doesn't support it.
    #[arg(long, alias = "as", add = ArgValueCompleter::new(strum_variants_completer::<RecheckMethod>))]
    recheck_method: Option<RecheckMethod>,

    /// Do not copy/link added files to the file cache
    #[arg(long)]
    no_commit: bool,
    /// Calculate digests as text or binary file without checking contents, or by automatically. (Default:
    /// auto)
    #[arg(long, add = ArgValueCompleter::new(strum_variants_completer::<TextOrBinary>))]
    text_or_binary: Option<FileTextOrBinary>,

    /// Include git tracked files as well. (Default: false)
    ///
    /// Xvc doesn't track files that are already tracked by git by default.
    /// You can set files.track.include-git to true in the configuration file to
    /// change this behavior.
    #[arg(long)]
    include_git_files: bool,

    /// Add targets even if they are already tracked
    #[arg(long)]
    force: bool,

    /// Don't use parallelism
    #[arg(long)]
    no_parallel: bool,

    /// Files/directories to track
    #[arg(value_hint=clap::ValueHint::AnyPath)]
    targets: Option<Vec<String>>,
}

impl UpdateFromXvcConfig for TrackCLI {
    /// Updates `xvc file` configuration from the configuration files.
    /// Command line options take precedence over other sources.
    /// If options are not given, they are supplied from [XvcConfig]
    fn update_from_conf(self, conf: &XvcConfig) -> XvcConfigResult<Box<Self>> {
        let recheck_method = self
            .recheck_method
            .unwrap_or_else(|| RecheckMethod::from_conf(conf));
        let no_commit = self.no_commit || conf.get_bool("file.track.no_commit")?.option;
        let force = self.force || conf.get_bool("file.track.force")?.option;
        let no_parallel = self.no_parallel || conf.get_bool("file.track.no_parallel")?.option;
        let text_or_binary = self.text_or_binary.as_ref().map_or_else(
            || Some(FileTextOrBinary::from_conf(conf)),
            |v| Some(v.to_owned()),
        );
        let include_git_files =
            self.include_git_files || conf.get_bool("file.track.include_git_files")?.option;

        Ok(Box::new(Self {
            targets: self.targets.clone(),
            recheck_method: Some(recheck_method),
            no_commit,
            force,
            no_parallel,
            text_or_binary,
            include_git_files,
        }))
    }
}

/// ## The pipeline
///
/// ```mermaid
/// graph LR
///     Target --> |File| Path
///     Target -->|Directory| Dir
///     Dir --> |File| Path
///     Dir --> |Directory| Dir
///     Path --> Ignored{Is this ignored?}
///     Ignored --> |Yes| Ignore
///     Ignored --> |No| XvcPath
///     XvcPath --> |Force| XvcDigest
///     XvcPath --> Filter{Is this changed?}
///     Filter -->|Yes| XvcDigest
///     Filter -->|No| Ignore
///     XvcDigest --> CacheLocation
///     CacheLocation --> RecheckMethod
///     RecheckMethod --> |Copy| Copy
///     RecheckMethod --> |Symlink| Symlink
///     RecheckMethod --> |Hardlink| Hardlink
///     RecheckMethod --> |Reflink| Reflink
/// ```
pub fn cmd_track(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    cli_opts: TrackCLI,
) -> Result<()> {
    let conf = xvc_root.config();
    let opts = cli_opts.update_from_conf(conf)?;
    let current_dir = conf.current_dir()?;
    let filter_git_files = !opts.include_git_files;
    let targets = targets_from_disk(
        output_snd,
        xvc_root,
        current_dir,
        &opts.targets,
        filter_git_files,
    )?;
    let requested_recheck_method = opts.recheck_method;
    let text_or_binary = opts.text_or_binary.unwrap_or_default();
    let no_parallel = opts.no_parallel;

    let stored_xvc_path_store = xvc_root.load_store::<XvcPath>()?;
    let stored_xvc_metadata_store = xvc_root.load_store::<XvcMetadata>()?;

    let xvc_path_metadata_diff = diff_xvc_path_metadata(
        xvc_root,
        &stored_xvc_path_store,
        &stored_xvc_metadata_store,
        &targets,
    );

    let xvc_path_diff = xvc_path_metadata_diff.0;
    let xvc_metadata_diff = xvc_path_metadata_diff.1;

    let changed_entities: HashSet<XvcEntity> =
        xvc_path_diff
            .iter()
            .filter_map(|(xe, xpd)| if xpd.changed() { Some(*xe) } else { None })
            .chain(xvc_metadata_diff.iter().filter_map(|(xe, xpd)| {
                if xpd.changed() {
                    Some(*xe)
                } else {
                    None
                }
            }))
            .collect();

    let stored_recheck_method_store = xvc_root.load_store::<RecheckMethod>()?;
    let default_recheck_method = RecheckMethod::from_conf(conf);
    let recheck_method_diff = diff_recheck_method(
        default_recheck_method,
        &stored_recheck_method_store,
        requested_recheck_method,
        &changed_entities,
    );

    let stored_text_or_binary_store = xvc_root.load_store::<FileTextOrBinary>()?;
    let text_or_binary_diff = diff_text_or_binary(
        &stored_text_or_binary_store,
        text_or_binary,
        &changed_entities,
    );

    let hash_algorithm = HashAlgorithm::from_conf(conf);

    let stored_content_digest_store = xvc_root.load_store::<ContentDigest>()?;

    let content_digest_diff = diff_content_digest(
        output_snd,
        xvc_root,
        &stored_xvc_path_store,
        &stored_xvc_metadata_store,
        &stored_content_digest_store,
        &stored_text_or_binary_store,
        &xvc_path_diff,
        &xvc_metadata_diff,
        opts.text_or_binary,
        Some(hash_algorithm),
        !no_parallel,
    );

    update_store_records(xvc_root, &xvc_path_diff, true, false)?;
    update_store_records(xvc_root, &xvc_metadata_diff, true, false)?;
    update_store_records(xvc_root, &recheck_method_diff, true, false)?;
    update_store_records(xvc_root, &text_or_binary_diff, true, false)?;
    update_store_records(xvc_root, &content_digest_diff, true, false)?;

    let file_targets: Vec<XvcPath> = targets
        .iter()
        .filter_map(|(xp, xmd)| {
            if xmd.file_type == XvcFileType::File {
                Some(xp.clone())
            } else {
                None
            }
        })
        .collect();

    // Warning: This one uses `opts.targets` instead of `targets` because
    // `targets` has been filtered to only include files.
    let dir_targets: Vec<XvcPath> = opts
        .targets
        .unwrap_or_else(|| vec![current_dir.to_string()])
        .iter()
        .filter_map(|t| {
            let p = PathBuf::from(t);
            if p.is_dir() {
                XvcPath::new(xvc_root, current_dir, &p).ok()
            } else {
                None
            }
        })
        .collect();

    let current_gitignore = build_gitignore(xvc_root)?;

    update_dir_gitignores(xvc_root, &current_gitignore, &dir_targets)?;
    // We reload gitignores here to make sure we ignore the given dirs

    let current_gitignore = build_gitignore(xvc_root)?;

    update_file_gitignores(xvc_root, &current_gitignore, &file_targets)?;

    if !opts.no_commit {
        let current_xvc_path_store = xvc_root.load_store::<XvcPath>()?;

        let updated_content_digest_store: HStore<ContentDigest> = content_digest_diff
            .into_iter()
            .filter_map(|(xe, cdd)| match cdd {
                xvc_core::Diff::Identical => None,
                xvc_core::Diff::RecordMissing { actual } => Some((xe, actual)),
                xvc_core::Diff::ActualMissing { .. } => None,
                xvc_core::Diff::Different { actual, .. } => Some((xe, actual)),
                xvc_core::Diff::Skipped => None,
            })
            .collect();

        // Only the updated paths are carried in
        // TODO: Allow --force option to carry-in all paths
        let xvc_paths_to_carry =
            current_xvc_path_store.subset(updated_content_digest_store.keys().cloned())?;

        // Filter directories from carry_in / recheck
        let xvc_paths_to_carry: HStore<XvcPath> = xvc_paths_to_carry
            .into_iter()
            .filter_map(|(xe, xp)| {
                targets.get(&xp).and_then(|xmd| {
                    if xmd.file_type == XvcFileType::File {
                        Some((xe, xp))
                    } else {
                        None
                    }
                })
            })
            .collect();

        let cache_paths = updated_content_digest_store
            .iter()
            .filter_map(|(xe, cd)| {
                xvc_paths_to_carry
                    .get(xe)
                    .and_then(|xp| XvcCachePath::new(xp, cd).ok())
                    .map(|cp| (*xe, cp))
            })
            .collect();

        let recheck_method_store = xvc_root.load_store::<RecheckMethod>()?;

        carry_in(
            output_snd,
            xvc_root,
            &xvc_paths_to_carry,
            &cache_paths,
            &recheck_method_store,
            !no_parallel,
            opts.force,
        )?;
    }

    Ok(())
}
