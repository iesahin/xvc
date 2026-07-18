//! Parse a KDL document into an [`XvcPipelineSchema`].
use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;

use kdl::{KdlDocument, KdlNode, KdlValue};
use regex::Regex;
use relative_path::RelativePathBuf;
use url::Url;
use xvc_core::XvcPath;

use crate::error::{Error, Result};
use crate::pipeline::deps::{
    FileDep, GenericDep, GlobDep, GlobItemsDep, LineItemsDep, LinesDep, ParamDep, RegexDep,
    RegexItemsDep, SqliteQueryDep, StepDep, UrlDigestDep,
};
use crate::pipeline::schema::{XvcPipelineSchema, XvcStepSchema};
use crate::pipeline::{XvcOutput, XvcStepInvalidate};
use crate::{XvcDependency, XvcMetricsFormat};

/// Dependency node type properties, shared by `node` declarations and inline
/// `deps` properties. `sqlite-file`/`sqlite-query` must appear together.
const NODE_TYPES: &[&str] = &[
    "file",
    "glob",
    "glob-items",
    "param",
    "regex",
    "regex-items",
    "lines",
    "line-items",
    "url",
    "generic",
    "sqlite-file",
    "sqlite-query",
];

fn err<T>(message: impl Into<String>) -> Result<T> {
    Err(Error::InvalidKdlPipeline {
        message: message.into(),
    })
}

fn xvc_path(spec: &str) -> XvcPath {
    XvcPath::from(RelativePathBuf::from(spec))
}

fn string_value(value: &KdlValue, what: &str) -> Result<String> {
    match value.as_string() {
        Some(s) => Ok(s.to_string()),
        None => err(format!("{what} must be a string, found {value}")),
    }
}

/// Parse the KDL text of a pipeline definition into the schema used by
/// `xvc pipeline import`.
pub fn pipeline_schema_from_kdl(content: &str) -> Result<XvcPipelineSchema> {
    let doc = KdlDocument::parse(content).map_err(|e| Error::InvalidKdlPipeline {
        message: e
            .diagnostics
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join("\n"),
    })?;

    let mut pipeline_node = None;
    for node in doc.nodes() {
        match node.name().value() {
            "version" => match node.get(0).and_then(KdlValue::as_integer) {
                Some(1) => {}
                _ => return err("only 'version 1' is supported"),
            },
            "pipeline" => {
                if pipeline_node.replace(node).is_some() {
                    return err("a KDL pipeline file must contain a single 'pipeline' node");
                }
            }
            other => return err(format!("unexpected top level node '{other}'")),
        }
    }

    match pipeline_node {
        Some(node) => parse_pipeline(node),
        None => err("no 'pipeline' node found"),
    }
}

fn parse_pipeline(pipeline: &KdlNode) -> Result<XvcPipelineSchema> {
    let name = match pipeline.get(0) {
        Some(v) => string_value(v, "pipeline name")?,
        None => return err("'pipeline' requires a name argument"),
    };

    let mut workdir = XvcPath::root_path()?;
    // Node declarations by id; BTreeMap is enough as steps refer by name.
    let mut nodes = BTreeMap::<String, XvcDependency>::new();
    let mut steps = Vec::<XvcStepSchema>::new();

    let children = pipeline
        .children()
        .map(KdlDocument::nodes)
        .unwrap_or_default();

    // Two passes so that steps can reference nodes declared after them.
    for child in children {
        match child.name().value() {
            "workdir" => {
                let dir = match child.get(0) {
                    Some(v) => string_value(v, "workdir")?,
                    None => return err("'workdir' requires a directory argument"),
                };
                workdir = xvc_path(&dir);
            }
            "node" => {
                let (id, dep) = parse_node(child)?;
                if nodes.insert(id.clone(), dep).is_some() {
                    return err(format!("duplicate node id '{id}'"));
                }
            }
            "step" => {}
            other => {
                return err(format!(
                    "unexpected node '{other}' in pipeline; expected 'workdir', 'node' or 'step'"
                ));
            }
        }
    }

    for child in children {
        if child.name().value() == "step" {
            steps.push(parse_step(child, &nodes)?);
        }
    }

    if let Some(step) = steps
        .iter()
        .find(|s| steps.iter().filter(|o| o.name == s.name).count() > 1)
    {
        return err(format!("duplicate step name '{}'", step.name));
    }

    Ok(XvcPipelineSchema {
        version: 1,
        name,
        workdir,
        steps,
    })
}

/// Parse a `node "id" <type>="<spec>"` declaration. The id defaults to the
/// spec text itself.
fn parse_node(node: &KdlNode) -> Result<(String, XvcDependency)> {
    let mut id = None;
    for entry in node.entries() {
        if entry.name().is_none()
            && id
                .replace(string_value(entry.value(), "node id")?)
                .is_some()
        {
            return err("'node' takes a single id argument");
        }
    }
    let (default_id, dep) = parse_dep_properties(node)?;
    Ok((id.unwrap_or(default_id), dep))
}

/// Parse the dependency type properties of a `node` or an inline `deps` entry.
/// Returns the default node id (the spec text) and the dependency.
fn parse_dep_properties(node: &KdlNode) -> Result<(String, XvcDependency)> {
    let mut typed = Vec::<(String, String)>::new();
    for entry in node.entries() {
        if let Some(name) = entry.name() {
            let name = name.value();
            if !NODE_TYPES.contains(&name) {
                return err(format!(
                    "unknown dependency type '{name}'; expected one of {}",
                    NODE_TYPES.join(", ")
                ));
            }
            typed.push((
                name.to_string(),
                string_value(entry.value(), &format!("'{name}' value"))?,
            ));
        }
    }

    match typed.as_slice() {
        [] => err("missing dependency type property"),
        [(ty, spec)] => Ok((spec.clone(), dep_from_spec(ty, spec)?)),
        [(ty1, v1), (ty2, v2)] => {
            // Only the sqlite pair uses two properties.
            let (file, query) = match (ty1.as_str(), ty2.as_str()) {
                ("sqlite-file", "sqlite-query") => (v1, v2),
                ("sqlite-query", "sqlite-file") => (v2, v1),
                _ => {
                    return err(format!(
                        "a dependency takes a single type property, found '{ty1}' and '{ty2}'"
                    ));
                }
            };
            Ok((
                file.clone(),
                XvcDependency::SqliteQueryDigest(SqliteQueryDep::new(
                    xvc_path(file),
                    query.clone(),
                )),
            ))
        }
        _ => err("a dependency takes a single type property"),
    }
}

/// Build a dependency from a type name and a spec string. Spec syntaxes are
/// identical to the corresponding `xvc pipeline step dependency` flags.
fn dep_from_spec(ty: &str, spec: &str) -> Result<XvcDependency> {
    let dep = match ty {
        "sqlite-file" | "sqlite-query" => {
            return err("sqlite dependencies need both sqlite-file and sqlite-query properties");
        }
        "file" => XvcDependency::File(FileDep::new(xvc_path(spec))),
        "glob" => XvcDependency::Glob(GlobDep::new(spec.to_string())),
        "glob-items" => XvcDependency::GlobItems(GlobItemsDep::new(spec.to_string())),
        "generic" => XvcDependency::Generic(GenericDep::new(spec.to_string())),
        "url" => XvcDependency::UrlDigest(UrlDigestDep::new(Url::parse(spec)?)),
        "param" => match spec.split_once("::") {
            Some((file, key)) if !file.is_empty() && !key.is_empty() => {
                XvcDependency::Param(ParamDep::new(&xvc_path(file), None, key.to_string())?)
            }
            _ => return err(format!("param '{spec}' must be in the form 'file::key'")),
        },
        "regex" | "regex-items" => {
            let (file, re) = match spec.split_once(":/") {
                Some((file, re)) if !file.is_empty() && !re.is_empty() => (file, re),
                _ => {
                    return err(format!("{ty} '{spec}' must be in the form 'file:/regex'"));
                }
            };
            if Regex::new(re).is_err() {
                return Err(Error::InvalidRegexFormat {
                    regex: re.to_string(),
                });
            }
            if ty == "regex" {
                XvcDependency::Regex(RegexDep::new(xvc_path(file), re.to_string()))
            } else {
                XvcDependency::RegexItems(RegexItemsDep::new(xvc_path(file), re.to_string()))
            }
        }
        "lines" | "line-items" => {
            let parsed = spec.split_once("::").and_then(|(file, range)| {
                let (begin, end) = range.split_once('-')?;
                let begin = if begin.is_empty() {
                    0
                } else {
                    begin.parse().ok()?
                };
                let end = if end.is_empty() {
                    usize::MAX
                } else {
                    end.parse().ok()?
                };
                Some((file, begin, end))
            });
            let (file, begin, end) = match parsed {
                Some(p) if !p.0.is_empty() => p,
                _ => {
                    return err(format!(
                        "{ty} '{spec}' must be in the form 'file::begin-end'"
                    ));
                }
            };
            if ty == "lines" {
                XvcDependency::Lines(LinesDep::new(xvc_path(file), begin, end))
            } else {
                XvcDependency::LineItems(LineItemsDep::new(xvc_path(file), begin, end))
            }
        }
        _ => unreachable!("checked against NODE_TYPES"),
    };
    Ok(dep)
}

fn parse_step(step: &KdlNode, nodes: &BTreeMap<String, XvcDependency>) -> Result<XvcStepSchema> {
    let name = match step.get(0) {
        Some(v) => string_value(v, "step name")?,
        None => return err("'step' requires a name argument"),
    };

    let command = match step.get("command") {
        Some(v) => string_value(v, "step command")?,
        None => return err(format!("step '{name}' requires a command property")),
    };

    let invalidate = match step.get("when") {
        Some(v) => {
            let when = string_value(v, "step when")?;
            XvcStepInvalidate::from_str(&when.replace('-', "_")).map_err(|_| {
                Error::InvalidKdlPipeline {
                    message: format!(
                        "step '{name}': when=\"{when}\" must be one of by_dependencies, always, never"
                    ),
                }
            })?
        }
        None => XvcStepInvalidate::default(),
    };

    for entry in step.entries() {
        match entry.name().map(|n| n.value()) {
            None | Some("command") | Some("when") => {}
            Some(other) => {
                return err(format!("step '{name}': unknown property '{other}'"));
            }
        }
    }

    let mut dependencies = Vec::<XvcDependency>::new();
    let mut outputs = Vec::<XvcOutput>::new();

    let children = step.children().map(KdlDocument::nodes).unwrap_or_default();
    for child in children {
        match child.name().value() {
            "deps" => {
                for entry in child.entries() {
                    if entry.name().is_none() {
                        // Argument: a reference to a declared node.
                        let id = string_value(entry.value(), "deps node reference")?;
                        match nodes.get(&id) {
                            Some(dep) => dependencies.push(dep.clone()),
                            None => {
                                return err(format!("step '{name}' references unknown node '{id}'"));
                            }
                        }
                    }
                }
                // Properties: anonymous inline nodes.
                if child.entries().iter().any(|e| e.name().is_some()) {
                    let (_, dep) = parse_dep_properties(child)?;
                    dependencies.push(dep);
                }
            }
            "after" => {
                for entry in child.entries() {
                    let step_name = string_value(entry.value(), "after step name")?;
                    dependencies.push(XvcDependency::Step(StepDep::new(step_name)));
                }
            }
            "outs" => {
                let outs_children = child.children().map(KdlDocument::nodes).unwrap_or_default();
                for out in outs_children {
                    outputs.push(parse_output(&name, out)?);
                }
            }
            other => {
                return err(format!(
                    "step '{name}': unexpected child '{other}'; expected 'deps', 'after' or 'outs'"
                ));
            }
        }
    }

    Ok(XvcStepSchema {
        name,
        command,
        invalidate,
        dependencies,
        outputs,
    })
}

fn parse_output(step_name: &str, out: &KdlNode) -> Result<XvcOutput> {
    let kind = out.name().value();
    let path_str = match out.get(0) {
        Some(v) => string_value(v, "output path")?,
        None => {
            return err(format!(
                "step '{step_name}': '{kind}' output requires a path"
            ));
        }
    };
    let path = xvc_path(&path_str);
    let output = match kind {
        "file" => XvcOutput::File { path },
        "image" => XvcOutput::Image { path },
        "metric" => {
            let format = match out.get("format") {
                Some(v) => match string_value(v, "metric format")?.to_lowercase().as_str() {
                    "csv" => XvcMetricsFormat::CSV,
                    "json" => XvcMetricsFormat::JSON,
                    "tsv" => XvcMetricsFormat::TSV,
                    other => {
                        return err(format!(
                            "step '{step_name}': metric format '{other}' must be csv, json or tsv"
                        ));
                    }
                },
                None => XvcMetricsFormat::from_path(Path::new(&path_str)),
            };
            XvcOutput::Metric { path, format }
        }
        other => {
            return err(format!(
                "step '{step_name}': unknown output type '{other}'; expected file, metric or image"
            ));
        }
    };
    Ok(output)
}
