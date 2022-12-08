pub mod compare;

use std::fs::{self, Permissions};
use std::{
    fs::Metadata,
    path::{Path, PathBuf},
};

use crate::error::{Error, Result};
use crate::track::DataTextOrBinary;
use crossbeam_channel::{Receiver, Sender};
use log::warn;
use xvc_core::types::xvcpath::XvcCachePath;
use xvc_core::util::file::make_symlink;
use xvc_core::{util::file::is_text_file, HashAlgorithm, XvcDigest};
use xvc_core::{CacheType, ContentDigest, TextOrBinary, XvcPath, XvcRoot};

use xvc_ecs::XvcEntity;
use xvc_logging::watch;
use xvc_walker::PathMetadata;

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
        }
        CacheType::Hardlink => {
            fs::hard_link(&cache_path, &path)?;
            let mut perm = path.metadata()?.permissions();
            perm.set_readonly(true);
            fs::set_permissions(&path, perm)?;
        }
        CacheType::Symlink => {
            make_symlink(&cache_path, &path)?;
            let mut perm = path.metadata()?.permissions();
            perm.set_readonly(true);
            fs::set_permissions(&path, perm)?;
        }
        CacheType::Reflink => {
            match reflink::reflink_or_copy(&cache_path, &path) {
                Ok(None) => (),
                Ok(Some(_)) => warn!("File system doesn't support reflink. Used copy."),
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
    fs::create_dir_all(cache_dir)?;
    fs::rename(&path, &cache_path)
        .map_err(|source| Error::IoError { source })
        .unwrap_or_else(|e| {
            e.error();
        });
    Ok(())
}
