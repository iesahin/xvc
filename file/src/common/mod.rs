pub mod compare;

use std::fs::{self, Permissions};
use std::{
    fs::Metadata,
    path::{Path, PathBuf},
};

use crate::error::{Error, Result};
use crate::track::DataTextOrBinary;
use crossbeam_channel::{Receiver, Sender};
use log::{error, info, warn};
use xvc_core::types::xvcpath::XvcCachePath;
use xvc_core::util::file::make_symlink;
use xvc_core::{util::file::is_text_file, HashAlgorithm, XvcDigest};
use xvc_core::{CacheType, ContentDigest, TextOrBinary, XvcPath, XvcRoot};

use xvc_ecs::XvcEntity;
use xvc_logging::{watch, XvcOutputLine};
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
                output_snd
                    .send(format!("Unsupported Target Type: {}", t.to_string_lossy()).into())
                    .unwrap();
                None
            }
        })
        .filter(|t| {
            let ignore_result = check_ignore(&xvc_ignore, t);

            match ignore_result {
                MatchResult::Ignore => {
                    warn!("Ignored: {}", t.to_string_lossy());
                    false
                }
                MatchResult::Whitelist => {
                    info!("Whitelisted: {}", t.to_string_lossy());
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
