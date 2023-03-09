pub mod compare;
pub mod digest;
pub mod param;

pub use param::*;

use serde::{Deserialize, Serialize};

use strum_macros::Display;

use crate::error::{Error, Result};
use url::Url;
use xvc_config::XvcConfig;
use xvc_core::{
    dir_includes, filter_paths_by_directory, glob_includes, glob_paths, XvcPath,
    XvcPathMetadataMap, XvcRoot,
};
use xvc_ecs::{persist, HStore, XvcStore};

pub fn conf_params_file(conf: &XvcConfig) -> Result<String> {
    Ok(conf.get_str("pipeline.default_params_file")?.option)
}

/// Represents variety of dependencies Xvc supports.
/// This is to unify all dependencies without dynamic dispatch and having
/// compile time errors when we miss something about dependencies.
#[derive(Debug, Display, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum XvcDependency {
    /// A generic dependency that's invalidated when the given command's output has changed.
    Generic { generic_command: String },

    /// Invalidates when the dependency step is invalidated.
    Step {
        /// The name of the step
        name: String,
    },
    /// Invalidates when the file content changes.
    File {
        /// The path in the workspace
        path: XvcPath,
    },
    /// Invalidates when contents in any of the files this glob describes
    Glob {
        /// The glob pattern that will be converted to a [Glob]
        glob: String,
    },
    /// Invalidates when contents of any of the files in the directory changes.
    Directory {
        /// The path in the workspace
        path: XvcPath,
    },
    /// Invalidates when header of the URL get request changes.
    Url {
        /// URL like https://example.com/my-file.html
        url: Url,
    },
    /// Invalidates when key in params file in path changes.
    Param {
        /// Format of the params file.
        /// This is inferred from extension if not given.
        format: XvcParamFormat,
        /// Path of the file in the workspace
        path: XvcPath,
        /// Key like `mydict.mykey` to access the value
        key: String,
    },
    /// When a step depends to a regex searched in a text file
    Regex {
        /// Path of the file in the workspace
        path: XvcPath,
        /// The regex to search in the file
        // We use this because Regex is not Serializable
        regex: String,
    },
    /// When a step depends to a set of lines in a text file
    Lines {
        /// Path of the file in the workspace
        path: XvcPath,
        /// The beginning of range
        begin: usize,
        /// The end of range
        end: usize,
    },
    // TODO: Slice {path, begin, length} to specify portions of binary files
    // TODO: DatabaseTable { database, table } to specify particular tables from databases
    // TODO: DatabaseQuery { database, query } to specify the result of queries
    // TODO: GraphQL { url, query } to specify a graphql
    // TODO: S3 { url } to specify S3 buckets
    // TODO: REST { url } to make Rest queries
    // TODO: Bitcoin { wallet } to check Bitcoin wallets
    // TODO: JupyterNotebook { file, cell }
    // TODO: EnvironmentVariable { name }
    // TODO: PythonFunc {file, name}
    // TODO: PythonClass {file, name}
}

persist!(XvcDependency, "xvc-dependency");

impl XvcDependency {
    /// Returns the path of the dependency if it has a single path.
    pub fn xvc_path(&self) -> Option<XvcPath> {
        match self {
            XvcDependency::File { path } => Some(path.clone()),
            XvcDependency::Directory { path, .. } => Some(path.clone()),
            XvcDependency::Param { path, .. } => Some(path.clone()),
            XvcDependency::Regex { path, .. } => Some(path.clone()),
            XvcDependency::Lines { path, .. } => Some(path.clone()),
            XvcDependency::Generic { .. } => None,
            XvcDependency::Step { .. } => None,
            XvcDependency::Glob { .. } => None,
            XvcDependency::Url { .. } => None,
        }
    }
}

/// Returns steps that depend to `to_path`
/// For dependencies with a single file `path`, these makes equality checks.
/// For `XvcDependency::Glob { glob }`, it checks whether `to_path` is matched with `glob`
/// For `XvcDependency::Directory { dir }`, it checks whether `to_path` is under `dir`.
/// Note that for granular dependencies (`Param`, `Regex`, `Lines`), there may be required further
/// checks whether the step actually depends to `to_path`, but as we don't have outputs that are
/// described more granular than a file, it simply assumes if `step-A` writes to `file-A`, any
/// other step that depends on `file-A` is a dependency to `step-A`.

pub fn dependencies_to_path(
    xvc_root: &XvcRoot,
    pmm: &XvcPathMetadataMap,
    pipeline_rundir: &XvcPath,
    all_deps: &XvcStore<XvcDependency>,
    to_path: &XvcPath,
) -> HStore<XvcDependency> {
    let mut deps_to_path = HStore::<XvcDependency>::with_capacity(all_deps.len());
    for (dep_e, dep) in all_deps.iter() {
        let has_path = match dep {
            XvcDependency::File { path } => *path == *to_path,
            XvcDependency::Glob { glob } => {
                glob_includes(xvc_root, pmm, pipeline_rundir, glob.as_str(), to_path)
                    .unwrap_or_else(|e| {
                        e.warn();
                        false
                    })
            }
            XvcDependency::Directory { path, .. } => dir_includes(pmm, path, to_path)
                .unwrap_or_else(|e| {
                    e.warn();
                    false
                }),
            XvcDependency::Param { path, .. } => *path == *to_path,
            XvcDependency::Regex { path, .. } => *path == *to_path,
            XvcDependency::Lines { path, .. } => *path == *to_path,
            XvcDependency::Generic { .. }
            | XvcDependency::Step { .. }
            | XvcDependency::Url { .. } => false,
        };

        if has_path {
            deps_to_path.insert(*dep_e, dep.clone());
        }
    }
    deps_to_path
}

/// Returns the local paths associated with a dependency. Some dependency types (Pipeline, Step, URL) don't have local paths.
pub fn dependency_paths(
    xvc_root: &XvcRoot,
    pmm: &XvcPathMetadataMap,
    pipeline_rundir: &XvcPath,
    dep: &XvcDependency,
) -> XvcPathMetadataMap {
    let make_map = |xp: &XvcPath| {
        let mut result_map = XvcPathMetadataMap::with_capacity(1);
        match pmm.get(xp) {
            Some(md) => {
                result_map.insert(xp.clone(), *md);
            }
            None => {
                Error::PathNotFoundInPathMetadataMap {
                    path: xp.to_absolute_path(xvc_root).as_os_str().to_owned(),
                }
                .warn();
            }
        }
        result_map
    };

    let empty = XvcPathMetadataMap::with_capacity(0);
    match dep {
        XvcDependency::Generic { .. } => empty,
        XvcDependency::Step { .. } => empty,
        XvcDependency::File { path } => make_map(path),
        XvcDependency::Glob { glob } => {
            glob_paths(xvc_root, pmm, pipeline_rundir, glob).unwrap_or(empty)
        }
        XvcDependency::Directory { path } => filter_paths_by_directory(pmm, path),
        XvcDependency::Url { .. } => empty,
        XvcDependency::Param { path, .. } => make_map(path),
        XvcDependency::Regex { path, .. } => make_map(path),
        XvcDependency::Lines { path, .. } => make_map(path),
    }
}
