use petgraph::algo::toposort;

use petgraph::graphmap::DiGraphMap;
use tabbycat::attributes::{shape, Shape, label, Color, color};
use tabbycat::{GraphBuilder, StmtList, Identity, Edge, AttrList};
use xvc_core::{all_paths_and_metadata, XvcPath, XvcRoot};
use xvc_ecs::{HStore, XvcEntity, XvcStore, R1NStore};
use xvc_logging::{output, watch, XvcOutputSender};

use std::collections::HashMap;
use std::{fs::File, io::Write};

use crate::error::{Error, Result};

use crate::pipeline::deps::step::StepDep;
use std::path::PathBuf;

use strum_macros::{Display, EnumString, IntoStaticStr};

use crate::{
    pipeline::{add_explicit_dependencies, add_implicit_dependencies, XvcStepInvalidate},
    XvcDependency, XvcOutput, XvcPipeline, XvcPipelineRunDir, XvcStep, XvcStepCommand,
};

#[derive(Debug, Clone, Eq, PartialEq, EnumString, Display, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum XvcPipelineDagFormat {
    Dot,
    Mermaid,
}

impl Default for XvcPipelineDagFormat {
    fn default() -> XvcPipelineDagFormat {
        XvcPipelineDagFormat::Dot
    }
}

fn step_desc(
    step_commands: &XvcStore<XvcStepCommand>,
    invalidations: &XvcStore<XvcStepInvalidate>,
    pipeline_steps: &HStore<XvcStep>,
    start_e: XvcEntity,
    end_e: XvcEntity,
    step_e: XvcEntity,
) -> String {
    let step = pipeline_steps.get(&step_e).cloned().unwrap();

    // Start step runs always
    watch!(step_e, start_e, end_e);
    let changes = if step_e == start_e {
        XvcStepInvalidate::Always
    } else if step_e == end_e {
        XvcStepInvalidate::Never
    } else {
        invalidations.get(&step_e).copied().unwrap_or_default()
    };

    // Start step has no command
    let command = if step_e == start_e {
        XvcStepCommand {
            command: "".to_string(),
        }
    } else if step_e == end_e {
        XvcStepCommand {
            command: "".to_string(),
        }
    } else {
        step_commands.get(&step_e).cloned().unwrap()
    };

    format!("step: {} ({}, {})", step.name, changes, command)
}

fn dep_desc(
    pipeline_steps: &HStore<XvcStep>,
    step_descs: &HStore<String>,
    dep: &XvcDependency,
) -> String {
    match dep {
        XvcDependency::Step(step_dep) => {
            let step_e = pipeline_steps
                .entity_by_value(&XvcStep {
                    name: step_dep.name.clone(),
                })
                .expect(&format!("Cannot find step {} in pipeline.", step_dep.name));
            step_descs.get(&step_e).unwrap().clone()
        }
        XvcDependency::Generic(generic_dep) => {
            format!("generic: {}", generic_dep.generic_command)
        }
        XvcDependency::File(dep) => format!("file: {}", dep.path),
        XvcDependency::LineItems(dep) => {
            format!("lines: {}::{}-{}", dep.path, dep.begin, dep.end)
        }
        XvcDependency::Lines(dep) => {
            format!("lines (d): {}::{}-{}", dep.path, dep.begin, dep.end)
        }
        XvcDependency::RegexItems(dep) => format!("regex: {}:/{}", dep.path, dep.regex),
        XvcDependency::Regex(dep) => format!("regex (d): {}:/{}", dep.path, dep.regex),
        XvcDependency::Param(dep) => format!("param: {}::{}", dep.path, dep.key),
        XvcDependency::GlobItems(dep) => format!("glob: {}", dep.glob),
        XvcDependency::Glob(dep) => format!("glob (d): {}", dep.glob),
        XvcDependency::UrlDigest(dep) => format!("url: {}", dep.url),
    }
}

/// Entry point for `xvc pipeline dag` command.
/// Create a graph of the pipeline and output it in the specified format.
pub fn cmd_dag(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    pipeline_name: String,
    file: Option<PathBuf>,
    format: XvcPipelineDagFormat,
) -> Result<()> {
    let _conf = xvc_root.config();

    let (pipeline_e, _) = XvcPipeline::from_name(xvc_root, &pipeline_name)?;

    // This is mutable to allow adding start and end nodes
    let mut pipeline_steps = xvc_root
        .load_r1nstore::<XvcPipeline, XvcStep>()?
        .children_of(&pipeline_e)?;

    let invalidations = xvc_root.load_store::<XvcStepInvalidate>()?;
    let step_commands = xvc_root.load_store::<XvcStepCommand>()?;

    let all_deps = xvc_root.load_r1nstore::<XvcStep, XvcDependency>()?;
    let all_outs = xvc_root.load_r1nstore::<XvcStep, XvcOutput>()?;
    let (pmm, _ignore_rules) = all_paths_and_metadata(xvc_root);

    let pipeline_len = pipeline_steps.len();


    let mut dependency_graph = DiGraphMap::<XvcEntity, XvcDependency>::with_capacity(
        pipeline_len,
        pipeline_len * pipeline_len,
    );

    let bs_pipeline_rundir = xvc_root.load_store::<XvcPipelineRunDir>()?;
    let pipeline_rundir = if bs_pipeline_rundir.contains_key(&pipeline_e) {
        let rd: XvcPipelineRunDir = bs_pipeline_rundir[&pipeline_e].clone();
        rd.run_dir
    } else {
        XvcPath::root_path()?
    };

    watch!(pipeline_steps);
    add_explicit_dependencies(
        output_snd,
        &pipeline_steps,
        &all_deps,
        &mut dependency_graph,
    )?;
    add_implicit_dependencies(
        output_snd,
        xvc_root,
        &pmm,
        &pipeline_rundir,
        &all_deps,
        &all_outs,
        &pipeline_steps,
        &mut dependency_graph,
    )?;

    watch!(dependency_graph);

    let out_string = match format {
        XvcPipelineDagFormat::Dot => {
            make_dot_graph(&pipeline_steps, &pipeline_name, &all_deps, &all_outs)?
        }
        XvcPipelineDagFormat::Mermaid => make_mermaid_graph(
            &pipeline_steps,
            &pipeline_name,
            &all_deps,
            &all_outs
        )?,
    };

    match file {
        None => Ok(output!(output_snd, "{}", out_string)),
        Some(file) => {
            let mut f = File::create(file)?;
            Ok(writeln!(f, "{}", out_string)?)
        }
    }
}

fn make_dot_graph(
    pipeline_steps: &HStore<XvcStep>,
    pipeline_name: &str,
    all_deps: &R1NStore<XvcStep, XvcDependency>,
    all_outs: &R1NStore<XvcStep, XvcOutput>,
) -> Result<String> {
    let graph = GraphBuilder::default()
        .graph_type(tabbycat::GraphType::DiGraph)
        .strict(false)
        .id(Identity::id("pipeline".to_string())?)
        .stmts(dependency_graph_stmts(pipeline_steps, all_deps, all_outs)?)
        .build().map_err(|e| anyhow::anyhow!("Failed to build graph. {e}"))?;

    Ok(format!("{}", graph))
}

fn id_from_string(s: &str) -> Result<Identity> {
    let hash_val = format!("i{}", seahash::hash(s.as_bytes()));
    Identity::id(hash_val).map_err(|e| e.into())
}

fn dep_identity(dep: &XvcDependency) -> Result<Identity> {
    match dep {
        XvcDependency::Step(dep) => id_from_string(&dep.name),
        XvcDependency::Generic(dep) => id_from_string(&dep.generic_command),
        XvcDependency::File(dep) => id_from_string(&dep.path.to_string()),
        XvcDependency::GlobItems(dep) => id_from_string(&dep.glob.to_string()),
        XvcDependency::Glob(dep) => id_from_string(&dep.glob.to_string()),
        XvcDependency::RegexItems(dep) => id_from_string(&format!("{}:/{}", dep.path.to_string(), dep.regex.to_string())),
        XvcDependency::Regex(dep) => id_from_string(&format!("{}:/{}", dep.path.to_string(), dep.regex.to_string())),
        XvcDependency::Param(dep) => id_from_string(&format!("{}::{}", dep.path.to_string(), dep.key.to_string())),
        XvcDependency::LineItems(dep) => id_from_string(&format!("{}::{}-{}", dep.path.to_string(),dep.begin.to_string(), dep.end.to_string())),
        XvcDependency::Lines(dep) => id_from_string(&format!("{}::{}-{}", dep.path.to_string(),dep.begin.to_string(), dep.end.to_string())),
        XvcDependency::UrlDigest(dep) => id_from_string(&dep.url.to_string()),
    }
}

fn out_identity(out: &XvcOutput) -> Result<Identity> {
    match out {
        XvcOutput::File { path } => id_from_string(&path.to_string()),
        XvcOutput::Metric { path,  .. } => id_from_string(&path.to_string()),
        XvcOutput::Image { path } => id_from_string(&path.to_string()),
    }
}

fn step_node_attributes(step: &XvcStep) -> AttrList {
    AttrList::new().add_pair(shape(Shape::Box)).add_pair(label(step.name.clone()))
}

fn dep_node_attributes(dep: &XvcDependency) -> AttrList {

    let dep_shape = match dep {
        XvcDependency::Step(_) => Shape::Box,
        XvcDependency::Generic(_) => Shape::Trapezium,
        XvcDependency::File(_) => Shape::Note,
        XvcDependency::GlobItems(_) => Shape::Folder,
        XvcDependency::Glob(_) => Shape::Folder,
        XvcDependency::RegexItems(_) => Shape::Signature,
        XvcDependency::Regex(_) => Shape::Signature,
        XvcDependency::Param(_) => Shape::Msquare,
        XvcDependency::LineItems(_) => Shape::Component,
        XvcDependency::Lines(_) => Shape::Component,
        XvcDependency::UrlDigest(_) => Shape::Invtrapezium,
    };


    AttrList::new().add_pair(shape(dep_shape)).add_pair(label(dep_label(dep)))

}

fn dep_label(dep: &XvcDependency) -> String {
        match dep {
        XvcDependency::Step(dep) => dep.name.clone(),
        XvcDependency::Generic(dep) => dep.generic_command.clone(),
        XvcDependency::File(dep) => dep.path.to_string(),
        XvcDependency::GlobItems(dep) => dep.glob.to_string(),
        XvcDependency::Glob(dep) => dep.glob.to_string(),
        XvcDependency::RegexItems(dep) => format!("{}:/{}", dep.path.to_string(), dep.regex.to_string()),
        XvcDependency::Regex(dep) => format!("{}:/{}", dep.path.to_string(), dep.regex.to_string()),
        XvcDependency::Param(dep) => format!("{}::{}", dep.path.to_string(), dep.key.to_string()),
        XvcDependency::LineItems(dep) => format!("{}::{}-{}", dep.path.to_string(),dep.begin.to_string(), dep.end.to_string()),
        XvcDependency::Lines(dep) => format!("{}::{}-{}", dep.path.to_string(),dep.begin.to_string(), dep.end.to_string()),
        XvcDependency::UrlDigest(dep) => dep.url.to_string(),
    }
}

fn out_label(out: &XvcOutput) -> String {
        match out {
        XvcOutput::File { path } => path.to_string(),
        XvcOutput::Metric { path, ..} => path.to_string(),
        XvcOutput::Image { path } => path.to_string(),
    }
}
fn out_node_attributes(out: &XvcOutput) -> AttrList {

    let out_shape = Shape::Note;


    let out_color = match out {
        XvcOutput::File { path } => Color::Black,
        XvcOutput::Metric { path, format } => Color::Blue,
        XvcOutput::Image { path } => Color::Green,
    };

    AttrList::new().add_pair(shape(out_shape)).add_pair(color(out_color)).add_pair(label(out_label(out)))
}

/// Create tabbycat::StmtList for dependencies and outputs
fn dependency_graph_stmts(
    pipeline_steps: &HStore<XvcStep>,
    all_deps: &R1NStore<XvcStep, XvcDependency>,
    all_outs: &R1NStore<XvcStep, XvcOutput>,
) -> Result<StmtList> {
    let mut stmts = StmtList::new();

    let  mut id_map = HashMap::<String, Identity>::new();

    let mut short_id = |id: Identity| -> Result<Identity> {
        let str_key = id.to_string();
        if !id_map.contains_key(&str_key) {
            id_map.insert(str_key.clone(), Identity::id(format!("n{}", id_map.len()))?);
            }
        Ok(id_map[&str_key].clone())
    };

    for (xe, step) in pipeline_steps.iter() {
        let step_identity = short_id(id_from_string(&step.name)?)?;
        let step_deps = all_deps.children_of(&xe)?;
        let step_outs = all_outs.children_of(&xe)?;

        stmts = stmts.add_node(step_identity.clone(), None, Some(step_node_attributes(&step)));

        for (xe_dep, dep) in step_deps.iter() {
            let dep_identity = short_id(dep_identity(dep)?)?;
            stmts = stmts.add_node(dep_identity.clone(), None, Some(dep_node_attributes(&dep)));
            stmts = stmts.add_edge(Edge::head_node(step_identity.clone(), None).arrow_to_node(dep_identity, None));
        }

        for (xe_dep, out) in step_outs.iter() {
            let out_identity = short_id(out_identity(out)?)?;
            stmts = stmts.add_node(out_identity.clone(), None, Some(out_node_attributes(&out)));
            stmts = stmts.add_edge(Edge::head_node(step_identity.clone(), None).arrow_to_node(out_identity, None));
        }
    }

    Ok(stmts)
}

/// Create a mermaid diagram from the given Graph.
/// Graph nodes are step descriptions, edges are dependencies.
fn make_mermaid_graph(
    pipeline_steps: &HStore<XvcStep>,
    pipeline_name: &str,
    all_deps: &R1NStore<XvcStep, XvcDependency>,
    all_outs: &R1NStore<XvcStep, XvcOutput>
) -> Result<String> {

    let  mut id_map = HashMap::<String, String>::new();

    let mut short_id = |id: Identity| -> Result<String> {
        let str_key = id.to_string();
        if !id_map.contains_key(&str_key) {
            id_map.insert(str_key.clone(), format!("n{}", id_map.len()));
            }
        Ok(id_map[&str_key].clone())
    };

    let mut res_string = String::new();
    res_string.push_str("flowchart TD\n");

    for (xe, s) in pipeline_steps.iter() {
        let deps = all_deps.children_of(xe)?;
        let outs = all_outs.children_of(xe)?;

        let step_label = s.name.clone();
        let step_id = short_id(id_from_string(&step_label)?)?;
        res_string.push_str(&format!("    {}[\"{}\"]\n", step_id, step_label));

        for (_, dep) in deps.iter() {
            let dep_label = dep_label(dep);
            let dep_id = short_id(id_from_string(&dep_label)?)?;
            // TODO: Specialize the shape according to dependency type
            res_string.push_str(&format!("    {}[\"{}\"] --> {}\n", dep_id, dep_label, step_id));
        }

        for (_, out) in outs.iter() {
            let out_label = out_label(out);
            let out_id = short_id(id_from_string(&out_label)?)?;
            // TODO: Add commands that produce these outputs
            res_string.push_str(&format!("    {}[\"{}\"] --> {}\n", out_id, out_label, step_id));
        }
    }

    Ok(res_string)
}
