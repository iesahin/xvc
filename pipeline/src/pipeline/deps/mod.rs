//! Step dependencies implementation
pub mod compare;
pub mod file;
pub mod generic;
pub mod glob;
pub mod glob_items;
pub mod line_items;
pub mod lines;
pub mod param;
pub mod regex;
pub mod regex_items;
pub mod step;
pub mod url;

use itertools::Itertools;
pub use param::*;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use xvc_config::XvcConfig;
use xvc_core::{glob_includes, glob_paths, XvcPath, XvcPathMetadataMap, XvcRoot};
use xvc_ecs::{persist, HStore, XvcStore};

pub use self::file::FileDep;
pub use self::generic::GenericDep;
pub use self::glob::GlobDep;
pub use self::glob_items::GlobItemsDep;
pub use self::line_items::LineItemsDep;
pub use self::lines::LinesDep;
pub use self::regex::RegexDep;
pub use self::regex_items::RegexItemsDep;
pub use self::step::StepDep;
pub use self::url::UrlDigestDep;

/// Return default name for the params file from the config
pub fn conf_params_file(conf: &XvcConfig) -> Result<String> {
    Ok(conf.get_str("pipeline.default_params_file")?.option)
}

/// Represents variety of dependencies Xvc supports.
/// This is to unify all dependencies without dynamic dispatch and having
/// compile time errors when we miss something about dependencies.
#[derive(
    Debug, strum_macros::Display, PartialOrd, Ord, Clone, Eq, PartialEq, Serialize, Deserialize,
)]
pub enum XvcDependency {
    /// Explicitly defined step depenedencies
    Step(StepDep),
    /// Dependencies which checks the change of output of a shell command
    Generic(GenericDep),
    /// Invalidates when the file content changes.
    File(FileDep),
    /// Invalidates when contents in any of the files this glob describes. Keeps track of
    /// individual files.
    GlobItems(GlobItemsDep),
    /// Invalidates when contents in any of the files this glob describes. Doesn't keep track of
    /// individual files.
    Glob(GlobDep),
    /// A dependency to a set of lines defined by a regex. Keeps track of individual lines.
    RegexItems(RegexItemsDep),
    /// A dependency to a set of lines defined by a regex. Doesn't keep track of individual lines.
    Regex(RegexDep),
    /// A dependency to a parameter in JSON, YAML or TOML file.
    Param(ParamDep),
    /// A dependenci to a set of lines defined by a range. Keeps track of individual lines.
    LineItems(LineItemsDep),
    /// A dependenci to a set of lines defined by a range. Doesn't keep track of individual lines.
    Lines(LinesDep),
    /// A dependency to a URL's content
    UrlDigest(UrlDigestDep),
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
            XvcDependency::File(file_dep) => Some(file_dep.path.clone()),
            XvcDependency::RegexItems(dep) => Some(dep.path.clone()),
            XvcDependency::Regex(dep) => Some(dep.path.clone()),
            XvcDependency::Param(dep) => Some(dep.path.clone()),
            XvcDependency::LineItems(dep) => Some(dep.path.clone()),
            XvcDependency::Lines(dep) => Some(dep.path.clone()),
            XvcDependency::Step(_) => None,
            XvcDependency::Generic(_) => None,
            XvcDependency::GlobItems(_) => None,
            XvcDependency::Glob(_) => None,
            XvcDependency::UrlDigest(_) => None,
        }
    }

    /// Send a list of items if the dependency has a list of items. Otherwise returns None.
    pub fn items(&self) -> Option<Vec<String>> {
        match self {
            XvcDependency::GlobItems(dep) => Some(
                dep.xvc_path_metadata_map
                    .keys()
                    .map(|xp| xp.to_string())
                    .sorted()
                    .collect::<Vec<String>>(),
            ),
            XvcDependency::RegexItems(dep) => {
                Some(dep.lines.clone().into_iter().sorted().collect())
            }
            XvcDependency::LineItems(dep) => Some(dep.lines.clone().into_iter().sorted().collect()),

            XvcDependency::Step(_)
            | XvcDependency::Generic(_)
            | XvcDependency::File(_)
            | XvcDependency::Glob(_)
            | XvcDependency::Regex(_)
            | XvcDependency::Param(_)
            | XvcDependency::Lines(_)
            | XvcDependency::UrlDigest(_) => None,
        }
    }
}

/// Returns steps that depend to `to_path`
/// For dependencies with a single file `path`, these makes equality checks.
/// For `XvcDependency::Glob ( glob )`, it checks whether `to_path` is in the paths of the dep.
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
            XvcDependency::Glob(dep) => {
                glob_includes(xvc_root, pmm, pipeline_rundir, dep.glob.as_str(), to_path)
                    .unwrap_or_else(|e| {
                        e.warn();
                        false
                    })
            }
            XvcDependency::File(dep) => dep.path == *to_path,
            XvcDependency::GlobItems(dep) => dep.xvc_path_metadata_map.keys().contains(to_path),
            XvcDependency::RegexItems(dep) => dep.path == *to_path,
            XvcDependency::Regex(dep) => dep.path == *to_path,
            XvcDependency::Param(dep) => dep.path == *to_path,
            XvcDependency::LineItems(dep) => dep.path == *to_path,
            XvcDependency::Lines(dep) => dep.path == *to_path,
            XvcDependency::Generic(_) | XvcDependency::Step(_) | XvcDependency::UrlDigest(_) => {
                false
            }
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
                Error::PathNotFound {
                    path: xp.to_absolute_path(xvc_root).as_os_str().to_owned(),
                }
                .warn();
            }
        }
        result_map
    };

    let empty = XvcPathMetadataMap::with_capacity(0);
    match dep {
        XvcDependency::Generic(_) => empty,
        XvcDependency::Step(_) => empty,
        XvcDependency::File(dep) => make_map(&dep.path),
        XvcDependency::GlobItems(dep) => dep
            .xvc_path_metadata_map
            .iter()
            .map(|(xp, xmd)| (xp.clone(), *xmd))
            .collect(),
        XvcDependency::Glob(dep) => glob_paths(xvc_root, pmm, pipeline_rundir, &dep.glob).unwrap(),
        XvcDependency::UrlDigest(_) => empty,
        XvcDependency::Param(dep) => make_map(&dep.path),
        XvcDependency::RegexItems(dep) => make_map(&dep.path),
        XvcDependency::LineItems(dep) => make_map(&dep.path),
        XvcDependency::Regex(dep) => make_map(&dep.path),
        XvcDependency::Lines(dep) => make_map(&dep.path),
    }
}
