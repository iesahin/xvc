use log::warn;

use rayon::prelude::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
use xvc_core::types::diff::Diffable;

use crate::error::{Error, Result};

use xvc_core::util::file::{
    compiled_regex, filter_paths_by_directory, glob_paths, XvcPathMetadataMap,
};
use xvc_core::{
    attribute_digest, AttributeDigest, ContentDigest, HashAlgorithm, PathCollectionDigest,
    XvcDigest, XvcMetadataDigest, XvcPath, XvcRoot,
};

use super::XvcParamFormat;

pub struct DependencyDigestParams<'a> {
    pub xvc_root: &'a XvcRoot,
    pub algorithm: &'a HashAlgorithm,
    pub pipeline_rundir: &'a XvcPath,
    pub pmm: &'a XvcPathMetadataMap,
}

fn directory_content_digest(
    params: &DependencyDigestParams,
    directory: &XvcPath,
) -> Result<ContentDigest> {
    let paths = filter_paths_by_directory(params.pmm, directory);
    paths_content_digest(params, &paths)
}

fn directory_metadata_digest(
    params: &DependencyDigestParams,
    directory: &XvcPath,
) -> Result<XvcMetadataDigest> {
    let paths = filter_paths_by_directory(params.pmm, directory);
    actual_paths_metadata_digest(params, &paths)
}

fn directory_collection_digest(
    params: &DependencyDigestParams,
    directory: &XvcPath,
) -> Result<PathCollectionDigest> {
    let paths = filter_paths_by_directory(params.pmm, directory);
    PathCollectionDigest::new(paths.into_iter().map(|(p, _)| p), *params.algorithm)
        .map_err(|e| e.into())
}

fn glob_content_digest(params: &DependencyDigestParams, glob: &str) -> Result<ContentDigest> {
    let paths = glob_paths(params.xvc_root, params.pmm, params.pipeline_rundir, glob)?;
    paths_content_digest(params, &paths)
}

fn glob_metadata_digest(params: &DependencyDigestParams, glob: &str) -> Result<XvcMetadataDigest> {
    let paths = glob_paths(params.xvc_root, params.pmm, params.pipeline_rundir, glob)?;
    actual_paths_metadata_digest(params, &paths)
}

/// Calculates the digest from a list of files defined by a glob
fn glob_collection_digest(
    params: &DependencyDigestParams,
    glob: &str,
) -> Result<PathCollectionDigest> {
    let paths = glob_paths(params.xvc_root, params.pmm, params.pipeline_rundir, glob)?;
    PathCollectionDigest::new(paths.into_iter().map(|(p, _)| p), *params.algorithm)
        .map_err(|e| e.into())
}

/// Compare digest from actual `path` metadata with its stored version
fn xvc_path_metadata_digest(
    params: &DependencyDigestParams,
    path: &XvcPath,
) -> Result<XvcMetadataDigest> {
    match params.pmm.get(path) {
        None => Err(Error::PathNotFoundInPathMetadataMap {
            path: path.to_absolute_path(params.xvc_root).into_os_string(),
        }
        .warn()),
        Some(metadata) => Ok(metadata.digest()?.into()),
    }
}

/// Returns a stable digest of the list of paths.
pub fn actual_paths_metadata_digest(
    params: &DependencyDigestParams,
    paths: &XvcPathMetadataMap,
) -> Result<XvcMetadataDigest> {
    let algorithm = params.algorithm;
    // These use string representations because of possible endianness changes across systems. It might be optimized but I don't think it will matter much.
    let md_str = paths.values().fold("".to_string(), |mut s, md| {
        s.push_str(&format!("{:?}", md));
        s
    });

    Ok(XvcDigest::from_content(&md_str, *algorithm).into())
}

///
///
fn paths_content_digest(
    params: &DependencyDigestParams,
    paths: &XvcPathMetadataMap,
) -> Result<ContentDigest> {
    let digests: HashMap<XvcPath, ContentDigest> = paths
        .par_iter()
        .filter_map(|(p, _)| match xvc_path_content_digest(params, p) {
            Ok(digest) => Some((p.clone(), digest)),
            Err(e) => {
                warn!("{:?}", e);
                None
            }
        })
        .collect();

    let mut whole_content = Vec::<u8>::with_capacity(digests.len() * 32);
    for (i, digest) in digests.values().enumerate() {
        // TODO: What about a zero copy operation here?
        whole_content[i * 32..(i + 1) * 32].copy_from_slice(&digest.digest().digest);
    }

    Ok(XvcDigest::from_bytes(&whole_content, *params.algorithm).into())
}

/// Returns content digest for `xvc_path`
/// Assumes `xvc_path` points to a file, not directory
pub fn xvc_path_content_digest(
    params: &DependencyDigestParams,
    xvc_path: &XvcPath,
) -> Result<ContentDigest> {
    Ok(XvcDigest::from_binary_file(
        &xvc_path.to_absolute_path(params.xvc_root),
        *params.algorithm,
    )?
    .into())
}
