//! Generate a KDL document from an [`XvcPipelineSchema`].
use kdl::{KdlDocument, KdlEntry, KdlNode};
use xvc_core::XvcPath;

use crate::error::Result;
use crate::pipeline::XvcStepInvalidate;
use crate::pipeline::schema::{XvcPipelineSchema, XvcStepSchema};
use crate::{XvcDependency, XvcMetricsFormat, XvcOutput};
use std::path::Path;

/// The KDL properties defining a dependency and the default node id derived
/// from its spec text. Step dependencies have no node form (they become
/// `after`), so they return None.
fn dep_properties(dep: &XvcDependency) -> Option<(String, Vec<(&'static str, String)>)> {
    let (id, props) = match dep {
        XvcDependency::Step(_) => return None,
        XvcDependency::File(d) => (d.path.to_string(), vec![("file", d.path.to_string())]),
        XvcDependency::Glob(d) => (d.glob.clone(), vec![("glob", d.glob.clone())]),
        XvcDependency::GlobItems(d) => (d.glob.clone(), vec![("glob-items", d.glob.clone())]),
        XvcDependency::Generic(d) => (
            d.generic_command.clone(),
            vec![("generic", d.generic_command.clone())],
        ),
        XvcDependency::UrlDigest(d) => (d.url.to_string(), vec![("url", d.url.to_string())]),
        XvcDependency::Param(d) => {
            let spec = format!("{}::{}", d.path, d.key);
            (spec.clone(), vec![("param", spec)])
        }
        XvcDependency::Regex(d) => {
            let spec = format!("{}:/{}", d.path, d.regex);
            (spec.clone(), vec![("regex", spec)])
        }
        XvcDependency::RegexItems(d) => {
            let spec = format!("{}:/{}", d.path, d.regex);
            (spec.clone(), vec![("regex-items", spec)])
        }
        XvcDependency::Lines(d) => {
            let spec = lines_spec(&d.path, d.begin, d.end);
            (spec.clone(), vec![("lines", spec)])
        }
        XvcDependency::LineItems(d) => {
            let spec = lines_spec(&d.path, d.begin, d.end);
            (spec.clone(), vec![("line-items", spec)])
        }
        XvcDependency::SqliteQueryDigest(d) => (
            d.path.to_string(),
            vec![
                ("sqlite-file", d.path.to_string()),
                ("sqlite-query", d.query.clone()),
            ],
        ),
    };
    Some((id, props))
}

fn lines_spec(path: &XvcPath, begin: usize, end: usize) -> String {
    if end == usize::MAX {
        format!("{path}::{begin}-")
    } else {
        format!("{path}::{begin}-{end}")
    }
}

/// A dependency node collected for the top of the document. Distinct
/// dependencies sharing a default id (e.g. a `glob` and a `glob-items` with
/// the same pattern) get the type appended to disambiguate.
struct DepNode {
    id: String,
    props: Vec<(&'static str, String)>,
}

/// Render the pipeline schema as a KDL document, the inverse of
/// [`super::pipeline_schema_from_kdl`]. Runtime bookkeeping (digests,
/// metadata) is not represented in KDL and is dropped.
pub fn pipeline_schema_to_kdl(schema: &XvcPipelineSchema) -> Result<String> {
    let mut nodes = Vec::<DepNode>::new();
    // node ids per (step index, dependency index), None for step deps
    let mut dep_ids = Vec::<Vec<Option<String>>>::new();

    for step in &schema.steps {
        let mut step_dep_ids = Vec::new();
        for dep in &step.dependencies {
            step_dep_ids.push(dep_properties(dep).map(|(id, props)| {
                match nodes.iter().find(|n| n.props == props) {
                    Some(existing) => existing.id.clone(),
                    None => {
                        let id = if nodes.iter().any(|n| n.id == id) {
                            format!("{} ({})", id, props[0].0)
                        } else {
                            id
                        };
                        nodes.push(DepNode {
                            id: id.clone(),
                            props,
                        });
                        id
                    }
                }
            }));
        }
        dep_ids.push(step_dep_ids);
    }

    let mut pipeline = KdlNode::new("pipeline");
    pipeline
        .entries_mut()
        .push(KdlEntry::new(schema.name.clone()));
    let children = pipeline.ensure_children();

    if schema.workdir != XvcPath::root_path()? {
        let mut workdir = KdlNode::new("workdir");
        workdir
            .entries_mut()
            .push(KdlEntry::new(schema.workdir.to_string()));
        children.nodes_mut().push(workdir);
    }

    for dep_node in &nodes {
        let mut node = KdlNode::new("node");
        node.entries_mut().push(KdlEntry::new(dep_node.id.clone()));
        for (name, value) in &dep_node.props {
            node.entries_mut()
                .push(KdlEntry::new_prop(*name, value.clone()));
        }
        children.nodes_mut().push(node);
    }

    for (step, step_dep_ids) in schema.steps.iter().zip(&dep_ids) {
        children.nodes_mut().push(step_node(step, step_dep_ids));
    }

    let mut doc = KdlDocument::new();
    doc.nodes_mut().push(pipeline);
    doc.autoformat();
    Ok(doc.to_string())
}

fn step_node(step: &XvcStepSchema, dep_ids: &[Option<String>]) -> KdlNode {
    let mut node = KdlNode::new("step");
    node.entries_mut().push(KdlEntry::new(step.name.clone()));
    node.entries_mut()
        .push(KdlEntry::new_prop("command", step.command.clone()));
    if step.invalidate != XvcStepInvalidate::default() {
        node.entries_mut()
            .push(KdlEntry::new_prop("when", step.invalidate.to_string()));
    }

    let children = node.ensure_children();

    let node_refs: Vec<&String> = dep_ids.iter().flatten().collect();
    if !node_refs.is_empty() {
        let mut deps = KdlNode::new("deps");
        for id in node_refs {
            deps.entries_mut().push(KdlEntry::new(id.clone()));
        }
        children.nodes_mut().push(deps);
    }

    let step_deps: Vec<&XvcDependency> = step
        .dependencies
        .iter()
        .filter(|d| matches!(d, XvcDependency::Step(_)))
        .collect();
    if !step_deps.is_empty() {
        let mut after = KdlNode::new("after");
        for dep in step_deps {
            if let XvcDependency::Step(step_dep) = dep {
                after
                    .entries_mut()
                    .push(KdlEntry::new(step_dep.name.clone()));
            }
        }
        children.nodes_mut().push(after);
    }

    if !step.outputs.is_empty() {
        let mut outs = KdlNode::new("outs");
        let outs_children = outs.ensure_children();
        for output in &step.outputs {
            outs_children.nodes_mut().push(output_node(output));
        }
        children.nodes_mut().push(outs);
    }

    node
}

fn output_node(output: &XvcOutput) -> KdlNode {
    match output {
        XvcOutput::File { path } => {
            let mut node = KdlNode::new("file");
            node.entries_mut().push(KdlEntry::new(path.to_string()));
            node
        }
        XvcOutput::Image { path } => {
            let mut node = KdlNode::new("image");
            node.entries_mut().push(KdlEntry::new(path.to_string()));
            node
        }
        XvcOutput::Metric { path, format } => {
            let mut node = KdlNode::new("metric");
            node.entries_mut().push(KdlEntry::new(path.to_string()));
            // The format is emitted only when the path alone doesn't imply it.
            if *format != XvcMetricsFormat::from_path(Path::new(&path.to_string())) {
                let format_str = match format {
                    XvcMetricsFormat::CSV => "csv",
                    XvcMetricsFormat::JSON => "json",
                    XvcMetricsFormat::TSV => "tsv",
                    XvcMetricsFormat::Unknown => "unknown",
                };
                node.entries_mut()
                    .push(KdlEntry::new_prop("format", format_str));
            }
            node
        }
    }
}
