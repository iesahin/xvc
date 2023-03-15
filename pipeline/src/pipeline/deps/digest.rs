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
    attribute_digest, AttributeDigest, CollectionDigest, ContentDigest, HashAlgorithm, XvcDigest,
    XvcMetadataDigest, XvcPath, XvcRoot,
};

use super::{XvcParamFormat, XvcParamPair};

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
) -> Result<CollectionDigest> {
    let paths = filter_paths_by_directory(params.pmm, directory);
    CollectionDigest::new(paths.into_iter().map(|(p, _)| p), *params.algorithm)
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
fn glob_collection_digest(params: &DependencyDigestParams, glob: &str) -> Result<CollectionDigest> {
    let paths = glob_paths(params.xvc_root, params.pmm, params.pipeline_rundir, glob)?;
    CollectionDigest::new(paths.into_iter().map(|(p, _)| p), *params.algorithm)
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ParamsDigest(XvcDigest);
attribute_digest!(ParamsDigest, "params-digest");
impl Diffable<ParamsDigest> for ParamsDigest {}

impl ParamsDigest {
    /// Generates a digest from a parameter found in JSON, TOML, YAML files.
    pub fn new(
        path: &Path,
        param_format: &XvcParamFormat,
        key: &str,
        algorithm: HashAlgorithm,
    ) -> Result<ParamsDigest> {
        let param_pair = XvcParamPair::new_with_format(&path, param_format, key)?;
        let digest = XvcDigest::from_content(&format!("{}", param_pair.value), algorithm);
        Ok(Self(digest))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RegexSubsetDigest(XvcDigest);
attribute_digest!(RegexSubsetDigest, "regex-subset-digest");
impl Diffable<RegexSubsetDigest> for RegexSubsetDigest {}

impl RegexSubsetDigest {
    fn new(path: &Path, regex: &str, algorithm: HashAlgorithm) -> Result<RegexSubsetDigest> {
        let re = compiled_regex(regex.into())?;
        let content = fs::read_to_string(path)?;
        let all_capture = re
            .find_iter(&content)
            .fold("".to_string(), |p, c| format!("{}{}", p, c.as_str()));
        Ok(Self(XvcDigest::from_content(&all_capture, algorithm)))
    }
}

/// Generates a digest from concatenation of all matches of `regex` in `xvc_path`

/// Generates a digest from specified lines of a text file.
/// It doesn't read the whole file to the memory, so should be safe to use for very large files
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LinesSubsetDigest(XvcDigest);
attribute_digest!(LinesSubsetDigest, "lines-subset-digest");
impl Diffable<LinesSubsetDigest> for LinesSubsetDigest {}

impl LinesSubsetDigest {
    pub fn new(path: &Path, begin: usize, end: usize, algorithm: HashAlgorithm) -> Result<Self> {
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

        Ok(Self(XvcDigest::from_content(&content, algorithm)))
    }
}
