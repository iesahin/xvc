use crossbeam_channel::Sender;
use petgraph::{dot::Dot, graph::NodeIndex, graphmap::DiGraphMap, Graph};
use xvc_core::{all_paths_and_metadata, XvcPath, XvcRoot};
use xvc_ecs::{HStore, XvcEntity, XvcStore};
use xvc_logging::XvcOutputLine;

use std::{fs::File, io::Write};

use crate::error::Result;
use std::path::PathBuf;

use strum_macros::{Display, EnumString, IntoStaticStr};

use crate::{
    pipeline::{add_explicit_dependencies, add_implicit_dependencies, XvcStepInvalidate},
    XvcDependency, XvcOutput, XvcPipeline, XvcPipelineRunDir, XvcStep, XvcStepCommand,
};
use xvc_config::FromConfigKey;

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

pub fn dot_from_graph(graph: Graph<&str, &str>) -> Result<String> {
    Ok(Dot::new(&graph).to_string())
}

pub fn cmd_dag(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    name: Option<String>,
    file: Option<PathBuf>,
    format: Option<XvcPipelineDagFormat>,
) -> Result<()> {
    let conf = xvc_root.config();
    let format = match format {
        Some(f) => f,
        None => XvcPipelineDagFormat::default(),
    };
    let pipeline_name = name.unwrap_or_else(|| XvcPipeline::from_conf(conf).name);

    let (pipeline_e, _) = XvcPipeline::from_name(xvc_root, &pipeline_name)?;

    let pipeline_steps = xvc_root
        .load_r1nstore::<XvcPipeline, XvcStep>()?
        .children_of(&pipeline_e)?;

    let consider_changed = xvc_root.load_store::<XvcStepInvalidate>()?;
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

    add_explicit_dependencies(&pipeline_steps, &all_deps, &mut dependency_graph)?;
    add_implicit_dependencies(
        xvc_root,
        &pmm,
        &pipeline_rundir,
        &all_deps,
        &all_outs,
        &pipeline_steps,
        &mut dependency_graph,
    )?;

    let step_desc = |e: &XvcEntity| {
        let step = &pipeline_steps[e];
        let changes = consider_changed[e];
        let command = &step_commands[e];
        format!("step: {} ({}, {})", step.name, changes, command)
    };

    let dep_desc = |dep: &XvcDependency| match dep {
        XvcDependency::Step { name } => {
            let (step_e, _) = XvcStep::from_name(xvc_root, &pipeline_e, name)
                .expect("Cannot find step in pipeline");
            step_desc(&step_e)
        }
        XvcDependency::Pipeline { name } => format!("pipeline: {}", name),
        XvcDependency::File { path } => format!("file: {}", path),
        XvcDependency::Directory { path } => format!("dir: {}", path),
        XvcDependency::Import { url, path } => format!("import: {} as {}", url, path),
        XvcDependency::Lines { path, begin, end } => {
            format!("lines: {}::{}-{}", path, begin, end)
        }
        XvcDependency::Regex { path, regex } => format!("regex: {}:/{}", path, regex),
        XvcDependency::Param { path, key, .. } => format!("param: {}::{}", path, key),
        XvcDependency::Glob { glob } => format!("glob: {}", glob),
        XvcDependency::Url { url } => format!("url: {}", url),
    };

    let mut output_graph = Graph::<&str, &str>::with_capacity(
        dependency_graph.node_count() + dependency_graph.edge_count(),
        dependency_graph.edge_count() * dependency_graph.node_count(),
    );

    let step_descs: HStore<String> = dependency_graph
        .nodes()
        .map(|n| (n, step_desc(&n)))
        .collect();
    let mut dep_descs = HStore::<String>::new();
    for n in dependency_graph.nodes() {
        for (_, e_to, dep) in dependency_graph.edges(n) {
            let dep = dep_desc(dep);
            dep_descs.insert(e_to, dep);
        }
    }

    let mut output_nodes = HStore::<NodeIndex>::new();

    for n in dependency_graph.nodes() {
        let desc = &step_descs[&n];
        let step_node = output_graph.add_node(desc);
        output_nodes.map.insert(n, step_node);
    }
    for n in dependency_graph.nodes() {
        let step_node = output_nodes[&n];
        for (_, e_to, dep) in dependency_graph.edges(n) {
            let desc = &dep_descs[&e_to];
            if matches!(dep, XvcDependency::Step { .. }) {
                let other_step = output_nodes[&e_to];
                output_graph.add_edge(step_node, other_step, "");
            } else {
                let dep_node = output_graph.add_node(desc);
                output_graph.add_edge(step_node, dep_node, "");
            }
        }
    }

    let out_string = match format {
        XvcPipelineDagFormat::Dot => dot_from_graph(output_graph)?,
        XvcPipelineDagFormat::Mermaid => todo!(),
    };

    match file {
        None => Ok(output_snd.send(format!("{}", out_string).into()).unwrap()),
        Some(file) => {
            let mut f = File::create(file)?;
            Ok(writeln!(f, "{}", out_string)?)
        }
    }
}
