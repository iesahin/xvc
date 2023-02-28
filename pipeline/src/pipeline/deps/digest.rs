use log::warn;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead};

use url::Url;

use crate::error::{Error, Result};

use xvc_core::types::xvcdigest::collection_digest;
use xvc_core::util::file::{
    compiled_regex, filter_paths_by_directory, glob_paths, XvcPathMetadataMap,
};
use xvc_core::{
    CollectionDigest, ContentDigest, HashAlgorithm, MetadataDigest, XvcDigest, XvcPath, XvcRoot,
};

use super::{XvcDependency, XvcParamFormat, XvcParamPair};

pub struct DependencyDigestParams<'a> {
    pub xvc_root: &'a XvcRoot,
    pub algorithm: &'a HashAlgorithm,
    pub pipeline_rundir: &'a XvcPath,
    pub pmm: &'a XvcPathMetadataMap,
}

/// Returns the collection digest associated with the dependency.
/// A collection digest is a list defined by the dependency. Currently only [XvcDependency::Glob]
/// and [XvcDependency::Directory] dependencies has collection digests that show file lists
/// defined by them.
#[allow(dead_code)]
pub fn dependency_collection_digest(
    params: &DependencyDigestParams,
    dependency: &XvcDependency,
) -> Result<CollectionDigest> {
    let digest = match dependency {
        XvcDependency::Pipeline { .. } => CollectionDigest(None),
        XvcDependency::Step { .. } => CollectionDigest(None),
        XvcDependency::File { .. } => CollectionDigest(None),
        XvcDependency::Glob { glob } => glob_collection_digest(params, glob)?,

        XvcDependency::Directory { path } => directory_collection_digest(params, path)?,
        XvcDependency::Url { .. } => CollectionDigest(None),
        XvcDependency::Import { .. } => CollectionDigest(None),
        XvcDependency::Param { .. } => CollectionDigest(None),
        XvcDependency::Regex { .. } => CollectionDigest(None),
        XvcDependency::Lines { .. } => CollectionDigest(None),
    };
    Ok(digest)
}

/// Returns the filesystem / URL metadata digest associated with the dependency to see if it has changed. We use this as a shortcut to decide whether to calculate content digests.
#[allow(dead_code)]
pub fn dependency_metadata_digest(
    params: &DependencyDigestParams,
    dependency: &XvcDependency,
) -> Result<MetadataDigest> {
    let digest = match dependency {
        XvcDependency::Pipeline { .. } => MetadataDigest(None),
        XvcDependency::Step { .. } => MetadataDigest(None),
        XvcDependency::File { path } => xvc_path_metadata_digest(params, path)?,
        XvcDependency::Glob { glob } => glob_metadata_digest(params, glob)?,
        XvcDependency::Directory { path } => directory_metadata_digest(params, path)?,
        XvcDependency::Url { url } => url_metadata_digest(params, url)?,
        XvcDependency::Import { url, .. } => url_metadata_digest(params, url)?,
        XvcDependency::Param { path, .. } => xvc_path_metadata_digest(params, path)?,
        XvcDependency::Regex { path, .. } => xvc_path_metadata_digest(params, path)?,
        XvcDependency::Lines { path, .. } => xvc_path_metadata_digest(params, path)?,
    };
    Ok(digest)
}

/// Calculate a content digest from a dependency
/// This uses various different functions for each type of dependency
pub fn dependency_content_digest(
    params: &DependencyDigestParams,
    dependency: &XvcDependency,
) -> Result<ContentDigest> {
    let digest = match dependency {
        XvcDependency::Pipeline { .. } => ContentDigest(None),
        XvcDependency::Step { .. } => ContentDigest(None),
        XvcDependency::File { path } => xvc_path_content_digest(params, path)?,
        XvcDependency::Glob { glob } => glob_content_digest(params, glob)?,
        XvcDependency::Directory { path } => directory_content_digest(params, path)?,
        XvcDependency::Url { url } => url_content_digest(params, url)?,
        XvcDependency::Import { url, path: _ } => url_content_digest(params, url)?,
        XvcDependency::Param { format, path, key } => {
            params_content_digest(params, path, format, key)?
        }
        XvcDependency::Regex { path, regex } => regex_content_digest(params, path, regex)?,
        XvcDependency::Lines { path, begin, end } => {
            lines_content_digest(params, path, *begin, *end)?
        }
    };
    Ok(digest)
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
) -> Result<MetadataDigest> {
    let paths = filter_paths_by_directory(params.pmm, directory);
    actual_paths_metadata_digest(params, &paths)
}

fn directory_collection_digest(
    params: &DependencyDigestParams,
    directory: &XvcPath,
) -> Result<CollectionDigest> {
    let paths = filter_paths_by_directory(params.pmm, directory);
    paths_collection_digest(params, &paths).into()
}

fn glob_content_digest(params: &DependencyDigestParams, glob: &str) -> Result<ContentDigest> {
    let paths = glob_paths(params.xvc_root, params.pmm, params.pipeline_rundir, glob)?;
    paths_content_digest(params, &paths)
}

fn glob_metadata_digest(params: &DependencyDigestParams, glob: &str) -> Result<MetadataDigest> {
    let paths = glob_paths(params.xvc_root, params.pmm, params.pipeline_rundir, glob)?;
    actual_paths_metadata_digest(params, &paths)
}

/// Calculates the digest from a list of files defined by a glob
fn glob_collection_digest(params: &DependencyDigestParams, glob: &str) -> Result<CollectionDigest> {
    let paths = glob_paths(params.xvc_root, params.pmm, params.pipeline_rundir, glob)?;
    paths_collection_digest(params, &paths)
}

fn url_content_digest(_params: &DependencyDigestParams, _url: &Url) -> Result<ContentDigest> {
    todo!("Not Implemented")
}

fn url_metadata_digest(_params: &DependencyDigestParams, _url: &Url) -> Result<MetadataDigest> {
    todo!("Not Implemented")
}

/// Compare digest from actual `path` metadata with its stored version
fn xvc_path_metadata_digest(
    params: &DependencyDigestParams,
    path: &XvcPath,
) -> Result<MetadataDigest> {
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
        whole_content[i * 32..(i + 1) * 32].copy_from_slice(digest.digest().digest);
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

/// Generates a digest from a parameter found in JSON, TOML, YAML files.
fn params_content_digest(
    digest_params: &DependencyDigestParams,
    param_file: &XvcPath,
    param_format: &XvcParamFormat,
    key: &str,
) -> Result<ContentDigest> {
    let path = param_file.to_absolute_path(digest_params.xvc_root);
    let param_pair = XvcParamPair::new_with_format(&path, param_format, key)?;
    let digest =
        XvcDigest::from_content(&format!("{}", param_pair.value), *digest_params.algorithm);
    Ok(digest.into())
}

/// Generates a digest from concatenation of all matches of `regex` in `xvc_path`
fn regex_content_digest(
    params: &DependencyDigestParams,
    xvc_path: &XvcPath,
    regex: &str,
) -> Result<ContentDigest> {
    let xvc_root = params.xvc_root;
    let algorithm = params.algorithm;
    let path = xvc_path.to_absolute_path(xvc_root);
    let re = compiled_regex(regex.into())?;
    let content = fs::read_to_string(path)?;
    let all_capture = re
        .find_iter(&content)
        .fold("".to_string(), |p, c| format!("{}{}", p, c.as_str()));
    Ok(XvcDigest::from_content(&all_capture, *algorithm).into())
}

/// Generates a digest from specified lines of a text file.
/// It doesn't read the whole file to the memory, so should be safe to use for very large files
fn lines_content_digest(
    params: &DependencyDigestParams,
    xvc_path: &XvcPath,
    begin: usize,
    end: usize,
) -> Result<ContentDigest> {
    let xvc_root = params.xvc_root;
    let algorithm = params.algorithm;
    let path = xvc_path.to_absolute_path(xvc_root);
    let f = File::open(path)?;
    let reader = io::BufReader::new(f).lines();
    // assuming each line is ~1K in length
    let mut content = String::with_capacity((end - begin) * 1000);
    for (i, line_res) in reader.enumerate() {
        if i >= begin {
            if let Ok(line) = line_res {
                content.push_str(&line);
                content.push('\n');
            }
        }
        if i > end {
            break;
        }
    }

    Ok(XvcDigest::from_content(&content, *algorithm).into())
}
