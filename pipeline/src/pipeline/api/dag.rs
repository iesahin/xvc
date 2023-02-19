use petgraph::visit::{IntoEdgesDirected, IntoNodeReferences};
use petgraph::{dot::Dot, graph::NodeIndex, graphmap::DiGraphMap, Graph};
use xvc_core::{all_paths_and_metadata, XvcPath, XvcRoot};
use xvc_ecs::{HStore, XvcEntity};
use xvc_logging::{output, watch, XvcOutputSender};

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

/// Entry point for `xvc pipeline dag` command.
/// Create a graph of the pipeline and output it in the specified format.
pub fn cmd_dag(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    name: Option<String>,
    file: Option<PathBuf>,
    format: XvcPipelineDagFormat,
) -> Result<()> {
    let conf = xvc_root.config();
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

    // Create start and end nodes
    let start_step = XvcStep {
        name: "START".to_string(),
    };
    let end_step = XvcStep {
        name: "END".to_string(),
    };
    let start_e = xvc_root.new_entity();
    let end_e = xvc_root.new_entity();

    // All pipeline steps depend on start step

    let mut dependency_graph = DiGraphMap::<XvcEntity, XvcDependency>::with_capacity(
        pipeline_len,
        pipeline_len * pipeline_len,
    );

    for (step_e, step) in pipeline_steps.iter() {
        dependency_graph.add_edge(
            start_e,
            *step_e,
            XvcDependency::Step {
                name: step.name.clone(),
            },
        );

        dependency_graph.add_edge(
            *step_e,
            end_e,
            XvcDependency::Step {
                name: end_step.name.clone(),
            },
        );
    }

    let bs_pipeline_rundir = xvc_root.load_store::<XvcPipelineRunDir>()?;
    let pipeline_rundir = if bs_pipeline_rundir.contains_key(&pipeline_e) {
        let rd: XvcPipelineRunDir = bs_pipeline_rundir[&pipeline_e].clone();
        rd.run_dir
    } else {
        XvcPath::root_path()?
    };

    watch!(pipeline_steps);
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

    watch!(dependency_graph);

    let step_desc = |e: &XvcEntity| {
        let step = if *e == start_e {
            start_step.clone()
        } else if *e == end_e {
            end_step.clone()
        } else {
            pipeline_steps.get(e).cloned().unwrap()
        };

        // Start step runs always
        let changes = if *e == start_e {
            XvcStepInvalidate::Always
        } else if *e == end_e {
            XvcStepInvalidate::Never
        } else {
            consider_changed.get(e).copied().unwrap()
        };

        // Start step has no command
        let command = if *e == start_e {
            XvcStepCommand {
                command: "".to_string(),
            }
        } else if *e == end_e {
            XvcStepCommand {
                command: "".to_string(),
            }
        } else {
            step_commands.get(e).cloned().unwrap()
        };

        format!("step: {} ({}, {})", step.name, changes, command)
    };

    let mut output_graph = Graph::<&str, &str>::with_capacity(
        dependency_graph.node_count() + dependency_graph.edge_count(),
        dependency_graph.edge_count() * dependency_graph.node_count(),
    );

    let step_descs: HStore<String> = dependency_graph
        .nodes()
        .map(|n| (n, step_desc(&n)))
        .collect();

    let dep_desc = |dep: &XvcDependency| match dep {
        XvcDependency::Step { name } => {
            let (step_e, _) = XvcStep::from_name(xvc_root, &pipeline_e, name)
                .expect("Cannot find step in pipeline");
            step_descs[&step_e].clone()
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

    let mut dep_descs = HStore::<String>::new();
    for e_from in dependency_graph.nodes() {
        for (_, e_to, dep) in dependency_graph.edges(e_from) {
            if e_to == start_e {
                dep_descs.insert(e_to, step_descs[&e_to].clone());
            } else if e_to == end_e {
                dep_descs.insert(e_to, step_descs[&e_to].clone());
            } else {
                let dep = dep_desc(dep);
                dep_descs.insert(e_to, dep);
            }
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

    watch!(output_graph);
    let out_string = match format {
        XvcPipelineDagFormat::Dot => dot_from_graph(output_graph)?,
        XvcPipelineDagFormat::Mermaid => mermaid_from_graph(output_graph)?,
    };

    match file {
        None => Ok(output!(output_snd, "{}", out_string)),
        Some(file) => {
            let mut f = File::create(file)?;
            Ok(writeln!(f, "{}", out_string)?)
        }
    }
}

/// Create a mermaid diagram from the given Graph.
/// Graph nodes are step descriptions, edges are dependencies.
fn mermaid_from_graph(output_graph: Graph<&str, &str>) -> Result<String> {
    let mut out_string = String::new();
    out_string.push_str("graph TD\n");
    let sanitize_node = |s: &str| {
        let node_name = s.replace(" ", "_").replace("(", "").replace(")", "");
        if node_name != s {
            format!("{node_name}[{s}]")
        } else {
            node_name
        }
    };
    for e in output_graph.edge_indices() {
        let (from, to) = output_graph.edge_endpoints(e).unwrap();

        let from_label = sanitize_node(output_graph[from]);
        let to_label = sanitize_node(output_graph[to]);
        let edge_label = output_graph.edge_weight(e).unwrap_or(&"");
        out_string.push_str(&format!(
            "{} --> |{}|{}\n",
            from_label, to_label, edge_label
        ));
    }

    Ok(out_string)
}
