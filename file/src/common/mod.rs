pub mod compare;

use std::fs::{self, Permissions};
use std::{
    fs::Metadata,
    path::{Path, PathBuf},
};

use crate::error::{Error, Result};
use crate::track::DataTextOrBinary;
use crossbeam_channel::{Receiver, Sender};
use xvc_core::types::xvcpath::XvcCachePath;
use xvc_core::util::file::make_symlink;
use xvc_core::{util::file::is_text_file, HashAlgorithm, XvcDigest};
use xvc_core::{
    CacheType, ContentDigest, TextOrBinary, XvcFileType, XvcPath, XvcPathMetadataMap, XvcRoot,
};
use xvc_logging::{error, info, warn, watch};

use xvc_ecs::XvcEntity;
use xvc_logging::XvcOutputLine;
use xvc_walker::{check_ignore, AbsolutePath, IgnoreRules, MatchResult, PathMetadata};

#[derive(Debug, Clone)]
pub struct PathMatch {
    xvc_path: Option<XvcPath>,
    actual_path: Option<PathMetadata>,
    xvc_entity: Option<XvcEntity>,
    actual_digest: Option<XvcDigest>,
}

pub fn calc_digest(
    path: &Path,
    algorithm: &HashAlgorithm,
    text_or_binary: TextOrBinary,
) -> Result<XvcDigest> {
    match text_or_binary {
        TextOrBinary::Auto => {
            let is_text_f = is_text_file(path).unwrap_or_else(|e| {
                e.warn();
                false
            });

            if is_text_f {
                Ok(XvcDigest::from_text_file(path, algorithm)?)
            } else {
                Ok(XvcDigest::from_binary_file(path, algorithm)?)
            }
        }
        TextOrBinary::Text => Ok(XvcDigest::from_text_file(path, algorithm)?),
        TextOrBinary::Binary => Ok(XvcDigest::from_binary_file(path, algorithm)?),
    }
}

pub fn pipe_path_digest(
    receiver: Receiver<(PathBuf, Metadata)>,
    sender: Sender<(PathBuf, XvcDigest)>,
    algorithm: &HashAlgorithm,
    text_or_binary: TextOrBinary,
) -> Result<()> {
    while let Ok((p, _)) = receiver.try_recv() {
        let digest = calc_digest(&p, algorithm, text_or_binary);
        match digest {
            Ok(digest) => {
                let _ = sender.send((p, digest));
            }
            Err(err) => {
                log::warn!("{:?}", err);
            }
        }
    }
    Ok(())
}

pub fn pathbuf_to_xvc_target(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    xvc_ignore: &IgnoreRules,
    current_dir: &AbsolutePath,
    targets: &Vec<PathBuf>,
) -> Vec<XvcPath> {
    targets
        .into_iter()
        .filter_map(|t| {
            watch!(t);
            watch!(t.is_file());
            watch!(t.is_dir());
            watch!(t.metadata());
            if t.is_file() || t.is_dir() {
                Some(t)
            } else {
                warn!(
                    output_snd,
                    "Unsupported Target Type: {}",
                    t.to_string_lossy()
                );
                None
            }
        })
        .filter(|t| {
            let ignore_result = check_ignore(&xvc_ignore, t);

            match ignore_result {
                MatchResult::Ignore => {
                    warn!(output_snd, "Ignored: {}", t.to_string_lossy());
                    false
                }
                MatchResult::Whitelist => {
                    info!(output_snd, "Whitelisted: {}", t.to_string_lossy());
                    true
                }
                MatchResult::NoMatch => true,
            }
        })
        .map(|t| XvcPath::new(xvc_root, current_dir, &t))
        .filter_map(|res_xp| match res_xp {
            Ok(xp) => Some(xp),
            Err(e) => {
                error!("{}", e);
                None
            }
        })
        .collect()
}

pub fn split_file_directory_targets(
    output_snd: Sender<XvcOutputLine>,
    xpmm: &XvcPathMetadataMap,
    xvc_targets: &[XvcPath],
) -> (XvcPathMetadataMap, XvcPathMetadataMap) {
    let mut dir_targets = XvcPathMetadataMap::new();
    let mut file_targets = XvcPathMetadataMap::new();

    for xvc_target in xvc_targets {
        if let Some(xmd) = xpmm.get(&xvc_target) {
            match xmd.file_type {
                XvcFileType::RecordOnly => {
                    error!(output_snd, "Target not found: {}", xvc_target);
                }
                XvcFileType::File => {
                    file_targets.insert(xvc_target.clone(), xmd.clone());
                }
                XvcFileType::Directory => {
                    dir_targets.insert(xvc_target.clone(), xmd.clone());
                }
                XvcFileType::Symlink => {
                    error!(output_snd, "Symlinks are not supported: {}", xvc_target)
                }
                XvcFileType::Hardlink => {
                    error!(output_snd, "Hardlinks are not supported: {xvc_target}");
                }
                XvcFileType::Reflink => {
                    error!(output_snd, "Reflinks are not supported: {xvc_target}")
                }
            }
        } else {
            warn!(output_snd, "Ignored or not found: {xvc_target}");
        }
    }

    (file_targets, dir_targets)
}

pub fn expand_directory_targets(
    output_snd: Sender<XvcOutputLine>,
    xpmm: &XvcPathMetadataMap,
    dir_targets: &XvcPathMetadataMap,
) -> (XvcPathMetadataMap, XvcPathMetadataMap) {
    let mut dir_targets = XvcPathMetadataMap::new();
    let mut file_targets = XvcPathMetadataMap::new();

    for (dir_target, dir_md) in &given_dir_targets {
        for (xvc_path, xvc_md) in &xpmm {
            if xvc_path.starts_with(&dir_target) && *xvc_path != *dir_target {
                match xvc_md.file_type {
                    XvcFileType::Directory => {
                        dir_targets.insert(xvc_path.clone(), xvc_md.clone());
                    }
                    XvcFileType::File => {
                        file_targets.insert(xvc_path.clone(), xvc_md.clone());
                    }
                    _ => {
                        error!(output_snd, "Unsupported Target: {xvc_path}");
                    }
                }
            }
        }
        dir_targets.insert(dir_target.clone(), dir_md.clone());
    }
    (dir_targets, file_targets)
}

pub fn expanded_xvc_dir_file_targets(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    targets: Vec<PathBuf>,
) -> (XvcPathMetadataMap, XvcPathMetadataMap) {
    let (xpmm, xvc_ignore) = all_paths_and_metadata(xvc_root);
    let xvc_targets =
        pathbuf_to_xvc_target(output_snd, xvc_root, &xvc_ignore, current_dir, &targets);

    let (mut file_targets, given_dir_targets) =
        split_file_directory_targets(output_snd, &xpmm, &xvc_targets);
    // Add all paths under directory targets
    let (mut dir_targets, implicit_file_targets) =
        expand_directory_targets(output_snd, &xpmm, &given_dir_targets);

    dir_targets.extend(given_dir_targets.iter());
    file_targets.extend(implicit_file_targets.iter());

    (dir_targets, file_targets)
}

pub const PARALLEL_THRESHOLD: usize = 47;

/// Use parallel processing if the number of targets is greater than the threshold
/// or directories are included in the targets.
pub fn decide_no_parallel(from_opts: bool, targets: &[PathBuf]) -> bool {
    from_opts || (targets.iter().all(|t| t.is_file()) && targets.len() < PARALLEL_THRESHOLD)
}

pub fn recheck_from_cache(
    xvc_root: &XvcRoot,
    xvc_path: &XvcPath,
    cache_path: &XvcCachePath,
    cache_type: CacheType,
) -> Result<()> {
    if let Some(parent) = xvc_path.parents().get(0) {
        let parent_dir = parent.to_absolute_path(xvc_root);
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }
    }
    let cache_path = cache_path.to_absolute_path(xvc_root);
    let path = xvc_path.to_absolute_path(xvc_root);
    watch!(path);
    watch!(cache_type);
    match cache_type {
        CacheType::Copy => {
            fs::copy(&cache_path, &path)?;
            let mut perm = path.metadata()?.permissions();
            perm.set_readonly(false);
            fs::set_permissions(&path, perm)?;
        }
        CacheType::Hardlink => {
            fs::hard_link(&cache_path, &path)?;
        }
        CacheType::Symlink => {
            make_symlink(&cache_path, &path)?;
        }
        CacheType::Reflink => {
            match reflink::reflink_or_copy(&cache_path, &path) {
                Ok(None) => (),
                Ok(Some(_)) => {
                    warn!("File system doesn't support reflink. Used copy.");
                    let mut perm = path.metadata()?.permissions();
                    perm.set_readonly(false);
                    fs::set_permissions(&path, perm)?;
                }
                Err(source) => {
                    Error::IoError { source }.error();
                }
            };
        }
    }
    Ok(())
}

pub fn cache_path(xvc_path: &XvcPath, content_digest: &ContentDigest) -> XvcCachePath {
    XvcCachePath::new(xvc_path, content_digest).unwrap()
}

pub fn move_to_cache(
    xvc_root: &XvcRoot,
    xvc_path: &XvcPath,
    cache_path: &XvcCachePath,
) -> Result<()> {
    let path = xvc_path.to_absolute_path(xvc_root);
    let cache_path = cache_path.to_absolute_path(xvc_root);
    let cache_dir = cache_path.parent().ok_or(Error::InternalError {
        message: "Cache path has no parent.".to_string(),
    })?;
    watch!(cache_dir);
    fs::create_dir_all(cache_dir)?;
    watch!(path);
    watch!(cache_path);
    fs::rename(&path, &cache_path).map_err(|source| Error::IoError { source })?;
    let mut perm = cache_path.metadata()?.permissions();
    perm.set_readonly(true);
    fs::set_permissions(&cache_path, perm)?;
    Ok(())
}

/// Record store records checking their DeltaField status
pub fn update_store_records<T>(
    xvc_root: &XvcRoot,
    delta_store: HStore<&DeltaField<T>>,
) -> Result<()>
where
    T: Storable,
{
    xvc_root.with_store_mut(|store: &mut XvcStore<T>| {
        for (xe, dd) in delta_store.iter() {
            match dd {
                DeltaField::Identical | DeltaField::Skipped => {
                    info!("Not changed: {:?}", xe);
                }
                DeltaField::RecordMissing { actual } => {
                    store.insert(*xe, actual.clone());
                }
                DeltaField::ActualMissing { .. } => {
                    info!("Record not changed. {}", xe);
                }
                DeltaField::Different { actual, .. } => {
                    store.insert(*xe, actual.clone());
                }
            }
        }
        Ok(())
    })?;

    Ok(())
}

/// Record updated directory records to various stores
pub fn update_dir_records(
    xvc_root: &XvcRoot,
    dir_delta_store: &HStore<DirectoryDelta>,
) -> Result<()> {
    let collection_delta_store: HStore<&DeltaField<CollectionDigest>> = dir_delta_store
        .iter()
        .map(|(xe, dd)| (*xe, &dd.delta_collection_digest))
        .collect();
    update_store_records(xvc_root, collection_delta_store)?;

    let metadata_digest_delta_store: HStore<&DeltaField<MetadataDigest>> = dir_delta_store
        .iter()
        .map(|(xe, dd)| (*xe, &dd.delta_metadata_digest))
        .collect();
    update_store_records(xvc_root, metadata_digest_delta_store)?;

    let content_delta_store: HStore<&DeltaField<ContentDigest>> = dir_delta_store
        .iter()
        .map(|(xe, dd)| (*xe, &dd.delta_content_digest))
        .collect();
    update_store_records(xvc_root, content_delta_store)?;

    let metadata_delta_store: HStore<&DeltaField<XvcMetadata>> = dir_delta_store
        .iter()
        .map(|(xe, dd)| (*xe, &dd.delta_xvc_metadata))
        .collect();
    update_store_records(xvc_root, metadata_delta_store)?;

    Ok(())
}

/// Record changes in `path_delta_store` to various stores in `xvc_root`
pub fn update_file_records(xvc_root: &XvcRoot, path_delta_store: &FileDeltaStore) -> Result<()> {
    let xvc_metadata_delta_store: HStore<&DeltaField<XvcMetadata>> = path_delta_store
        .iter()
        .map(|(xe, pd)| (xe.xvc_path.clone(), &pd.delta_md))
        .collect();
    update_store_records(xvc_root, xvc_metadata_delta_store)?;

    let content_digest_delta_store: HStore<&DeltaField<ContentDigest>> = path_delta_store
        .iter()
        .map(|(xe, pd)| (xe.xvc_path.clone(), &pd.delta_content_digest))
        .collect();
    update_store_records(xvc_root, content_digest_delta_store)?;

    let metadata_digest_delta_store: HStore<&DeltaField<MetadataDigest>> = path_delta_store
        .iter()
        .map(|(xe, pd)| (xe.xvc_path.clone(), &pd.delta_metadata_digest))
        .collect();
    update_store_records(xvc_root, metadata_digest_delta_store)?;

    let cache_type_delta_store: HStore<&DeltaField<CacheType>> = path_delta_store
        .iter()
        .map(|(xe, pd)| (xe.xvc_path.clone(), &pd.delta_cache_type))
        .collect();
    update_store_records(xvc_root, cache_type_delta_store)?;

    let data_text_or_binary_delta_store: HStore<&DeltaField<DataTextOrBinary>> = path_delta_store
        .iter()
        .map(|(xe, pd)| (xe.xvc_path.clone(), &pd.delta_text_or_binary))
        .collect();

    update_store_records(xvc_root, data_text_or_binary_delta_store)?;

    Ok(())
}
