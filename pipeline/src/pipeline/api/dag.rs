use petgraph::algo::toposort;
use petgraph::visit::{IntoEdgeReferences, IntoEdgesDirected, IntoNodeReferences};
use petgraph::{dot::Dot, graph::NodeIndex, graphmap::DiGraphMap, Graph};
use xvc_core::{all_paths_and_metadata, XvcPath, XvcRoot};
use xvc_ecs::{HStore, XvcEntity, XvcStore};
use xvc_logging::{output, watch, XvcOutputSender};

use std::{fs::File, io::Write};

use crate::error::{Error, Result};
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
        XvcDependency::Step { name } => {
            let step_e = pipeline_steps
                .entity_by_value(&XvcStep { name: name.clone() })
                .expect(&format!("Cannot find step {} in pipeline.", name));
            step_descs.get(&step_e).unwrap().clone()
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
    }
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

    // Create start and end nodes
    let start_step = XvcStep {
        name: "START".to_string(),
    };
    let end_step = XvcStep {
        name: "END".to_string(),
    };
    let start_e = (0, 0).into();
    let end_e = (u64::MAX, 0).into();

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
                name: end_step.clone().name,
            },
        );
    }

    // We add these steps to the pipeline steps temporarily to make the graph
    // We add these after creating [dependency graph] to avoid cycles.
    pipeline_steps.insert(start_e, start_step.clone());
    pipeline_steps.insert(end_e, end_step.clone());

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

    let step_descs: HStore<String> = pipeline_steps
        .iter()
        .map(|(e, _)| {
            (
                *e,
                step_desc(
                    &step_commands,
                    &invalidations,
                    &pipeline_steps,
                    start_e,
                    end_e,
                    *e,
                ),
            )
        })
        .collect();

    let out_string = match format {
        XvcPipelineDagFormat::Dot => {
            make_dot_graph(&pipeline_steps, &dependency_graph, &step_descs)?
        }
        XvcPipelineDagFormat::Mermaid => make_mermaid_graph(
            &pipeline_steps,
            &pipeline_name,
            &dependency_graph,
            &step_descs,
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
    dependency_graph: &DiGraphMap<XvcEntity, XvcDependency>,
    step_descs: &HStore<String>,
) -> Result<String> {
    let mut dep_descs = HStore::<String>::new();
    dependency_graph.nodes().for_each(|e_from| {
        dependency_graph.edges(e_from).for_each(|(_, e_to, dep)| {
            let dep = dep_desc(pipeline_steps, step_descs, dep);
            dep_descs.insert(e_to, dep);
        })
    });

    let mut output_nodes = HStore::<NodeIndex>::new();

    let mut output_graph = Graph::<&str, &str>::with_capacity(
        dependency_graph.node_count() + dependency_graph.edge_count(),
        dependency_graph.edge_count() * dependency_graph.node_count(),
    );

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
    dot_from_graph(output_graph)
}

/// Create a mermaid diagram from the given Graph.
/// Graph nodes are step descriptions, edges are dependencies.
fn make_mermaid_graph(
    pipeline_steps: &HStore<XvcStep>,
    pipeline_name: &str,
    dependency_graph: &DiGraphMap<XvcEntity, XvcDependency>,
    step_descs: &HStore<String>,
) -> Result<String> {
    let sorted_graph = match toposort(&dependency_graph, None) {
        Ok(vec) => vec,
        Err(c) => {
            let step_node = c.node_id();
            let step = pipeline_steps[&step_node].clone();
            return Err(Error::PipelineStepsContainCycle {
                pipeline: pipeline_name.to_string(),
                step: step.name,
            });
        }
    };
    let mut out_string = String::new();
    out_string.push_str("flowchart TD\n");
    let sanitize_node = |s: &str| {
        let node_name = s
            .replace(" ", "_")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "_");
        if node_name != s {
            format!("{node_name}[{s}]")
        } else {
            node_name
        }
    };

    sorted_graph.into_iter().for_each(|e_from| {
        let from_desc = &pipeline_steps[&e_from].name;
        let from_node_name = sanitize_node(from_desc);
        dependency_graph.edges(e_from).for_each(|(_, e_to, dep)| {
            let to_desc = &pipeline_steps[&e_to].name;
            let to_node_name = sanitize_node(to_desc);
            let node_desc = dep_desc(pipeline_steps, step_descs, dep);
            out_string.push_str(&format!(
                "\t{} --> {}[\"{}\"]\n",
                from_node_name, to_node_name, node_desc
            ));
        });
    });

    Ok(out_string)
}
