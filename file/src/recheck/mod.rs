//! Data structures and functions for `xvc file recheck`.
//!
//! - [RecheckCLI] describes the command line options.
//! - [cmd_recheck] is the entry point for the command line.
use std::fs;

use crate::{
    common::{
        cache_path,
        compare::{
            find_file_changes_parallel, find_file_changes_serial, DeltaField, FileDelta,
            FileDeltaStore, PathComparisonParams,
        },
        recheck_from_cache,
    },
    track::DataTextOrBinary,
    Result,
};
use clap::Parser;
use crossbeam_channel::Sender;
use log::error;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use xvc_config::{FromConfigKey, UpdateFromXvcConfig, XvcConfig};
use xvc_core::{
    CacheType, TextOrBinary, XvcFileType, XvcMetadata, XvcPath, XvcPathMetadataMap, XvcRoot,
};
use xvc_ecs::XvcEntity;
use xvc_logging::{watch, XvcOutputLine};
use xvc_walker::Glob;

/// Check out file from cache by a copy or link
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case", author, version)]
pub struct RecheckCLI {
    /// How to track the file contents in cache: One of copy, symlink, hardlink, reflink.
    ///
    /// Note: Reflink uses copy if the underlying file system doesn't support it.
    #[arg(long, alias = "as")]
    pub cache_type: Option<CacheType>,

    /// Don't use parallelism
    #[arg(long)]
    pub no_parallel: bool,

    /// Force even if target exists
    #[arg(long)]
    pub force: bool,

    /// Recheck files as text, binary (Default: auto)
    ///
    /// Text files may go OS specific line ending replacements.
    #[arg(long)]
    pub text_or_binary: Option<DataTextOrBinary>,
    /// Files/directories to recheck
    #[arg()]
    pub targets: Vec<String>,
}

impl UpdateFromXvcConfig for RecheckCLI {
    fn update_from_conf(self, conf: &XvcConfig) -> xvc_config::error::Result<Box<Self>> {
        let cache_type = self
            .cache_type
            .unwrap_or_else(|| CacheType::from_conf(conf));
        let no_parallel = self.no_parallel || conf.get_bool("file.add.no_parallel")?.option;

        let text_or_binary = self.text_or_binary.as_ref().map_or_else(
            || Some(DataTextOrBinary::from_conf(conf)),
            |v| Some(v.to_owned()),
        );
        let force = self.force;

        Ok(Box::new(Self {
            targets: self.targets,
            cache_type: Some(cache_type),
            force,
            no_parallel,
            text_or_binary,
        }))
    }
}

const PARALLEL_THRESHOLD: usize = 47;

pub fn cmd_recheck(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    cli_opts: RecheckCLI,
) -> Result<()> {
    let conf = xvc_root.config();
    let opts = cli_opts.update_from_conf(conf)?;
    let current_dir = conf.current_dir()?;

    let xvc_current_dir = XvcPath::new(xvc_root, current_dir, current_dir)?;
    watch!(xvc_current_dir);

    let pcp = PathComparisonParams::init(xvc_root)?;
    let path_store = pcp.xvc_path_store.clone();

    watch!(opts.targets);

    let target_store = if opts.targets.is_empty() {
        path_store
    } else {
        let mut globsetb = xvc_walker::GlobSetBuilder::new();
        for t in opts.targets.clone() {
            let pat = format!(
                "{xvc_current_dir}{}",
                if t.ends_with('/') {
                    &t[..(t.len() - 1)]
                } else {
                    &t
                }
            );

            match Glob::new(&pat) {
                Ok(g) => {
                    globsetb.add(g);
                }
                Err(e) => {
                    output_snd
                        .send(XvcOutputLine::Warn(format!("Error in glob: {} {}", t, e)))
                        .unwrap();
                }
            }
        }
        let globset = globsetb.build().map_err(|e| xvc_walker::Error::from(e))?;

        path_store.filter(|_, p| globset.is_match(p.to_string()))
    };

    let no_parallel = opts.no_parallel || target_store.len() < PARALLEL_THRESHOLD;

    let cache_type = opts.cache_type.unwrap_or_else(|| CacheType::default());
    let text_or_binary = opts
        .text_or_binary
        .unwrap_or_else(|| DataTextOrBinary::from(TextOrBinary::Auto));

    let mut file_targets = XvcPathMetadataMap::new();
    watch!(pcp.xvc_metadata_store);
    let xvc_targets: XvcPathMetadataMap = target_store
        .iter()
        .map(|(xe, xp)| {
            watch!(xp);
            watch!(xe);
            (
                xp.clone(),
                pcp.xvc_metadata_store
                    .get(xe)
                    .cloned()
                    .unwrap_or_else(|| XvcMetadata {
                        file_type: XvcFileType::RecordOnly,
                        size: None,
                        modified: None,
                    }),
            )
        })
        .collect();

    watch!(xvc_targets.len());

    for (xvc_path, x_md) in xvc_targets.iter() {
        watch!(xvc_path);
        watch!(x_md);
        if x_md.file_type == XvcFileType::Directory {
            for (child_e, child_path) in pcp.xvc_path_store.iter() {
                watch!(child_path);
                if child_path.starts_with(xvc_path) {
                    let child_md = pcp
                        .xvc_metadata_store
                        .get(child_e)
                        .cloned()
                        .unwrap_or_default();
                    if child_md.file_type == XvcFileType::File {
                        watch!(child_path);
                        let actual_md = XvcMetadata::from(
                            child_path.to_absolute_path(xvc_root).symlink_metadata(),
                        );
                        file_targets.insert(child_path.clone(), actual_md);
                    }
                }
            }
        } else if x_md.file_type == XvcFileType::File {
            let actual_md =
                XvcMetadata::from(xvc_path.to_absolute_path(xvc_root).symlink_metadata());
            file_targets.insert(xvc_path.clone(), actual_md);
        }
    }

    watch!(opts.targets);
    watch!(xvc_targets);
    watch!(file_targets);

    if no_parallel {
        let file_delta_store =
            find_file_changes_serial(xvc_root, &pcp, &cache_type, &text_or_binary, &file_targets)?;

        watch!(file_delta_store);

        recheck_serial(
            output_snd,
            xvc_root,
            cache_type,
            opts.force,
            &pcp,
            &file_delta_store,
        )
    } else {
        let file_delta_store = find_file_changes_parallel(
            xvc_root,
            &pcp,
            &cache_type,
            &text_or_binary,
            &file_targets,
        )?;
        watch!(file_delta_store);
        recheck_parallel(
            output_snd,
            xvc_root,
            cache_type,
            opts.force,
            &pcp,
            &file_delta_store,
        )
    }
}

fn recheck_inner(
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    cache_type: CacheType,
    force: bool,
    path_comparison_params: &PathComparisonParams,
    path_delta_store: &FileDeltaStore,
    xvc_entity: &XvcEntity,
    path_delta: &FileDelta,
) -> Result<()> {
    watch!(xvc_entity);
    let xvc_path = &path_comparison_params.xvc_path_store[xvc_entity];
    watch!(xvc_path);

    let content_digest = path_comparison_params
        .content_digest_store
        .get(&xvc_entity)
        .ok_or_else(|| xvc_ecs::Error::CannotFindKeyInStore {
            key: usize::from(*xvc_entity),
        })?;

    watch!(content_digest);

    let checkout = || -> Result<()> {
        let cache_path = cache_path(&xvc_path, &content_digest);
        watch!(cache_path);
        if cache_path.to_absolute_path(xvc_root).exists() {
            let target_path = xvc_path.to_absolute_path(xvc_root);
            watch!(target_path);
            if target_path.exists() {
                fs::remove_file(target_path)?;
            }
            recheck_from_cache(xvc_root, &xvc_path, &cache_path, cache_type)
        } else {
            error!("{} cannot found in cache", xvc_path);
            Ok(())
        }
    };

    watch!(path_delta.delta_content_digest);

    match path_delta.delta_content_digest {
        DeltaField::Identical | DeltaField::Skipped => {
            if force {
                output_snd.send(XvcOutputLine::Info(format!(
                    "{xvc_path} already exists. Overwriting."
                )))?;
                checkout()?;
            } else {
                output_snd.send(XvcOutputLine::Warn(format!(
                    "{xvc_path} already exists. Use --force to overwrite"
                )))?;
            }
        }
        DeltaField::RecordMissing { .. } => {
            output_snd.send(XvcOutputLine::Error(format!("No record for {xvc_path}")))?;
        }
        DeltaField::ActualMissing { .. } => {
            checkout()?;
        }
        DeltaField::Different { .. } => {
            if force {
                checkout()?;
            } else {
                output_snd.send(XvcOutputLine::Warn(format!(
                    "{xvc_path} has changed, use --force to overwrite"
                )))?;
            }
        }
    }

    Ok(())
}

fn recheck_serial(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    cache_type: CacheType,
    force: bool,
    path_comparison_params: &PathComparisonParams,
    file_delta_store: &FileDeltaStore,
) -> Result<()> {
    watch!(file_delta_store.len());
    for (xvc_entity, path_delta) in file_delta_store.iter() {
        recheck_inner(
            &output_snd,
            xvc_root,
            cache_type,
            force,
            path_comparison_params,
            file_delta_store,
            xvc_entity,
            path_delta,
        )?;
    }
    Ok(())
}

fn recheck_parallel(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    cache_type: CacheType,
    force: bool,
    path_comparison_params: &PathComparisonParams,
    path_delta_store: &FileDeltaStore,
) -> Result<()> {
    watch!(path_delta_store);
    path_delta_store
        .par_iter()
        .for_each(|(xvc_entity, path_delta)| {
            let _ = recheck_inner(
                &output_snd,
                xvc_root,
                cache_type,
                force,
                path_comparison_params,
                path_delta_store,
                xvc_entity,
                path_delta,
            )
            .map_err(|e| {
                e.warn();
            });
        });

    Ok(())
}
