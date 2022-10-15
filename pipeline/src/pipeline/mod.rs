pub mod api;
pub mod command;
pub mod deps;
pub mod outs;
pub mod schema;
pub mod step;

use self::command::XvcStepCommand;
use self::deps::XvcDependency;
use self::outs::XvcOutput;
use self::step::XvcStep;

use crate::deps::compare::{compare_deps, DependencyComparisonParams, XvcDependencyChange};
use crate::deps::{dependencies_to_path, dependency_paths};
use crate::error::{Error, Result};
use crate::{XvcPipeline, XvcPipelineRunDir};

use crossbeam_channel::{Receiver, Sender};
use xvc_walker::notify::{make_watcher, PathEvent};

use log::{info, warn};
use petgraph::algo::toposort;
use petgraph::data::Build;
use petgraph::dot::Dot;
use petgraph::prelude::DiGraphMap;

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use strum_macros::{Display, EnumString};
use xvc_config::FromConfigKey;
use xvc_core::{
    all_paths_and_metadata, CollectionDigest, ContentDigest, HashAlgorithm, MetadataDigest,
    TextOrBinary, XvcFileType, XvcMetadata, XvcPath, XvcPathMetadataMap, XvcRoot,
};

use xvc_ecs::{persist, HStore, R1NStore, XvcEntity, XvcStore};
use xvc_logging::watch;

use sp::ExitStatus;
use subprocess as sp;

/// The option whether to consider a step changed
#[derive(
    Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, EnumString, Display, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum XvcStepInvalidate {
    /// Change when dependencies change or outputs missing
    ByDependencies,
    /// Always consider changed
    Always,
    /// Never consider changed,
    Never,
}

persist!(XvcStepInvalidate, "xvc-step-invalidate");

impl Default for XvcStepInvalidate {
    fn default() -> Self {
        XvcStepInvalidate::ByDependencies
    }
}

/// Adds dependencies to `graph` in the form of `XvcDependency::Step`. These are called explicit
/// dependencies, as steps are defined explicitly to be depending to each other.
pub fn add_explicit_dependencies(
    pipeline_steps: &XvcStore<XvcStep>,
    all_deps: &R1NStore<XvcStep, XvcDependency>,
    graph: &mut DiGraphMap<XvcEntity, XvcDependency>,
) -> Result<()> {
    for (from_step_e, from_step) in pipeline_steps.iter() {
        let deps = all_deps.children_of(from_step_e)?;
        for (_to_step_e, to_step) in deps.iter() {
            if let XvcDependency::Step { name } = to_step {
                let candidate_step = XvcStep {
                    name: name.to_string(),
                };
                match pipeline_steps.entity_by_value(&candidate_step) {
                    // We don't check from_step == to_step equality here as we'll check cycles later
                    Some(entity) => {
                        graph.update_edge(*from_step_e, entity, to_step.clone());
                        info!(
                            "Found explicit dependency: {:?} -> {:?}",
                            from_step, to_step
                        );
                    }
                    None => {
                        return Err(Error::StepNotFoundInPipeline {
                            step: name.to_string(),
                        });
                    }
                }
            }
        }
    }
    Ok(())
}

/// Adds implicit dependencies between files
/// If `step-A` outputs `file-X`  and `step-B` depends on `file-X`, `step-B` is considered as
/// depending to `step-A`.
pub fn add_implicit_dependencies(
    xvc_root: &XvcRoot,
    pmm: &XvcPathMetadataMap,
    pipeline_rundir: &XvcPath,
    all_deps: &R1NStore<XvcStep, XvcDependency>,
    all_outs: &R1NStore<XvcStep, XvcOutput>,
    pipeline_steps: &XvcStore<XvcStep>,
    graph: &mut DiGraphMap<XvcEntity, XvcDependency>,
) -> Result<()> {
    for (to_step_e, to_step) in pipeline_steps.iter() {
        let to_outs = all_outs.children_of(to_step_e)?;
        let to_paths: HStore<XvcPath> = to_outs
            .iter()
            .map(|(o_e, o)| (*o_e, XvcPath::from(o)))
            .collect();
        for (_to_path_e, to_path) in to_paths.iter() {
            for (dep_e, dep) in
                dependencies_to_path(xvc_root, pmm, pipeline_rundir, &all_deps.children, to_path)
                    .iter()
            {
                let (from_step_e, from_step) = all_deps.parent_of(dep_e)?;
                if pipeline_steps.contains_key(from_step_e) {
                    // We allow parallel edges, as there may be more than one implicit dependency
                    // between steps.
                    graph.add_edge((*from_step_e).clone().into(), *to_step_e, dep.clone());
                    info!(
                        "Found implicit dependency: {:?} -> {:?} (via {:?})",
                        from_step, to_step, to_path
                    )
                } else {
                    // We just warn here, as there may be paths that are both in the pipeline
                    // and outside of it, and we just met the one outside of the pipeline.
                    //
                    // Note that we don't require `all_outs` and `all_deps` limited to the
                    // current pipeline.
                    warn!(
                        "{:?}",
                        Error::StepNotFoundInPipeline {
                            step: from_step.name.clone()
                        }
                    );
                }
            }
        }
    }
    Ok(())
}

type DependencyGraph = DiGraphMap<XvcEntity, XvcDependency>;

use step::*;

/// These run conditions may be exposed to the user. We use this struct here for clarity and
/// granularity. The current steps have 3 conditions: Never, Always and Calculated (Default).
/// This struct shows what we mean by these.
#[derive(Clone, Debug, Copy)]
struct RunConditions {
    never: bool,
    wait_running_dep_steps: bool,
    ignore_broken_dep_steps: bool,
    ignore_missing_dependencies: bool,
    ignore_timestamp_comparison: bool,
    ignore_content_digest_comparison: bool,
    run_when_outputs_missing: bool,
}
#[derive(Debug, Clone)]
struct StateParams<'a> {
    step_e: &'a XvcEntity,
    step_command: &'a XvcStepCommand,
    dependency_graph: &'a DependencyGraph,
    current_states: &'a HStore<XvcStepState>,
    step_timeout: &'a Duration,
    run_conditions: &'a RunConditions,
    running_process_pool_size: usize,
    all_outputs: &'a R1NStore<XvcStep, XvcOutput>,
    all_dependencies: &'a R1NStore<XvcStep, XvcDependency>,
    xvc_root: &'a XvcRoot,
    pipeline_rundir: &'a XvcPath,
    pmm: &'a XvcPathMetadataMap,
    terminate_timeout_processes: bool,
    log_channel_size: usize,
}

/// Used for encapsulating a process and its outputs. This is used to associate steps with running
/// processes.
#[derive(Debug)]
struct CommandProcess {
    process: Rc<RefCell<sp::Popen>>,
    birth: Instant,
    stdout_sender: RefCell<Sender<String>>,
    stderr_sender: RefCell<Sender<String>>,
    stdout_receiver: RefCell<Receiver<String>>,
    stderr_receiver: RefCell<Receiver<String>>,
}
/// # XVC Pipeline Dependency Graph Rules
///
/// The dependency graph shows which steps of the pipeline depends on other
/// steps. The dependency steps are set to run before the dependent steps.
///
/// There are two ways to configure dependencies: Explicit and implicit. The
/// first way is using:
///
/// ```bash
/// xvc pipeline dependency --step-name <dependent> --step <dependency>
/// ```
///
/// This adds an explicit rule to the graph to run `<dependency>` step
/// before `<dependent>`
///
/// The other way of adding dependencies is by setting an output of a
/// step as the dependency of another step. This is similar to makefiles.
///
/// ```bash
/// xvc pipeline output --step-name <dependency> --file mymodel.h5
/// xvc pipeline dependency --step-name <dependent> --file mymodel.h5
/// ```
///
/// All dependency types that accepts paths as dependencies invoke these
/// implicit rules.
///
/// ```bash
/// xvc pipeline output --step-name training --file 'models/mymodel.h5'
/// xvc pipeline dependency --step-name evaluation --glob 'models/*.h5'
/// ```
///
/// creates a dependency between `training` and `evaluation` steps.

pub fn the_grand_pipeline_loop(xvc_root: &XvcRoot, pipeline_name: String) -> Result<()> {
    let conf = xvc_root.config();
    let (pipeline_e, _) = XvcPipeline::from_name(xvc_root, &pipeline_name)?;

    let pipeline_steps = xvc_root
        .load_r1nstore::<XvcPipeline, XvcStep>()?
        .children_of(&pipeline_e)?;

    let consider_changed = xvc_root.load_store::<XvcStepInvalidate>()?;

    let all_deps = xvc_root.load_r1nstore::<XvcStep, XvcDependency>()?;
    let all_outs = xvc_root.load_r1nstore::<XvcStep, XvcOutput>()?;
    let (mut pmm, ignore_rules) = all_paths_and_metadata(xvc_root);
    let (_fs_watcher, fs_receiver) = make_watcher(ignore_rules)?;

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

    let debug_output = Dot::new(&dependency_graph);

    info!("Pipeline Graph:\n{}\n", debug_output);
    // Topological sort to get the cycles and run order
    let _sorted_steps = match toposort(&dependency_graph, None) {
        Ok(vec) => vec,
        Err(c) => {
            let step_node = c.node_id();
            let step = pipeline_steps[&step_node].clone();
            return Err(Error::PipelineStepsContainCycle {
                pipeline: pipeline_name,
                step: step.name,
            });
        }
    };

    // We are ready to run the pipeline state loop with the sorted steps

    let run_never = RunConditions {
        never: true,
        run_when_outputs_missing: false,
        ignore_missing_dependencies: false,
        wait_running_dep_steps: false,
        ignore_broken_dep_steps: false,
        ignore_timestamp_comparison: false,
        ignore_content_digest_comparison: false,
    };

    //  This is the DVC behavior. It doesn't run when _only_ dependency timestamp changed. For
    //  Makefile behavior `dependencies_new` can be set to `true`.
    let run_calculated = RunConditions {
        never: false,
        wait_running_dep_steps: true,
        ignore_broken_dep_steps: false,
        run_when_outputs_missing: true,
        ignore_missing_dependencies: false,
        ignore_timestamp_comparison: false,
        ignore_content_digest_comparison: false,
    };

    let run_always = RunConditions {
        never: false,
        run_when_outputs_missing: true,
        ignore_missing_dependencies: true,
        wait_running_dep_steps: true,
        ignore_broken_dep_steps: true,
        ignore_timestamp_comparison: true,
        ignore_content_digest_comparison: true,
    };

    let run_conditions: HStore<RunConditions> = pipeline_steps
        .iter()
        .map(|(step_e, _)| match consider_changed[step_e] {
            XvcStepInvalidate::ByDependencies => (*step_e, run_calculated),
            XvcStepInvalidate::Always => (*step_e, run_always),
            XvcStepInvalidate::Never => (*step_e, run_never),
        })
        .collect();

    let mut step_states: HStore<XvcStepState> = pipeline_steps
        .iter()
        .map(|(step_e, _)| (*step_e, step::XvcStepState::begin()))
        .collect();

    let mut continue_running = true;
    // the following definitions should be moved to config
    // let break_for_nonzero_exit = true;
    let parallel_processes = 1;
    let sleep_duration = 10;
    let log_channel_size = 1000;
    let default_step_timeout: u64 = 10000;
    let step_timeouts: HStore<Duration> = pipeline_steps
        .keys()
        .map(|step_e| (*step_e, Duration::from_secs(default_step_timeout)))
        .collect();

    let mut process_pool = HStore::<CommandProcess>::with_capacity(pipeline_len);

    let step_commands = xvc_root.load_store::<XvcStepCommand>()?;

    let stored_dependency_content_digests =
        xvc_root.load_r11store::<XvcDependency, ContentDigest>()?;
    let stored_dependency_metadata_digests =
        xvc_root.load_r11store::<XvcDependency, MetadataDigest>()?;
    let stored_dependency_collection_digests =
        xvc_root.load_r11store::<XvcDependency, CollectionDigest>()?;
    let stored_dependency_paths = xvc_root.load_r1nstore::<XvcDependency, XvcPath>()?;
    let stored_path_metadata = xvc_root.load_r11store::<XvcPath, XvcMetadata>()?;
    let stored_path_content_digest = xvc_root.load_r11store::<XvcPath, ContentDigest>()?;
    let stored_path_metadata_digest = xvc_root.load_r11store::<XvcPath, MetadataDigest>()?;
    let stored_path_collection_digest = xvc_root.load_r11store::<XvcPath, CollectionDigest>()?;
    let text_files = xvc_root.load_r11store::<XvcPath, TextOrBinary>()?;
    let algorithm = HashAlgorithm::from_conf(conf);

    while continue_running {
        let mut next_states = step_states.clone();
        let mut dependency_changes = HStore::<XvcDependencyChange>::new();

        for (step_e, step_s) in step_states.iter() {
            let params = StateParams {
                step_e,
                dependency_graph: &dependency_graph,
                current_states: &step_states,
                step_command: &step_commands[step_e],
                running_process_pool_size: parallel_processes,
                run_conditions: &run_conditions[step_e],
                all_outputs: &all_outs,
                all_dependencies: &all_deps,
                xvc_root,
                pmm: &pmm,
                terminate_timeout_processes: true,
                log_channel_size,
                step_timeout: &step_timeouts[step_e],
                pipeline_rundir: &pipeline_rundir,
            };
            let params_debug = params.clone();
            let r_next_state = match step_s {
                XvcStepState::Begin(s) => s_begin(s, params),
                XvcStepState::NoNeedToRun(s) => s_no_need_to_run(s, params),
                XvcStepState::WaitingDependencySteps(s) => s_waiting_dependency_steps(s, params),
                XvcStepState::CheckingMissingDependencies(s) => {
                    s_checking_missing_dependencies(s, params)
                }
                XvcStepState::Broken(s) => s_broken(s, params),
                XvcStepState::CheckingMissingOutputs(s) => s_checking_missing_outputs(s, params),
                XvcStepState::CheckingTimestamps(s) => s_checking_timestamps(s, params),
                XvcStepState::WaitingToRun(s) => s_waiting_to_run(s, params, &mut process_pool),
                XvcStepState::CheckingDependencyContentDigest(s) => {
                    let dep_comparison_params = DependencyComparisonParams {
                        xvc_root,
                        pipeline_rundir: &pipeline_rundir,
                        pmm: &pmm,
                        algorithm: &algorithm,
                        all_dependencies: &all_deps.children,
                        stored_dependency_paths: &stored_dependency_paths,
                        stored_path_metadata: &stored_path_metadata,
                        stored_dependency_collection_digest: &stored_dependency_collection_digests,
                        stored_dependency_content_digest: &stored_dependency_content_digests,
                        stored_dependency_metadata_digest: &stored_dependency_metadata_digests,
                        stored_path_collection_digest: &stored_path_collection_digest,
                        stored_path_content_digest: &stored_path_content_digest,
                        stored_path_metadata_digest: &stored_path_metadata_digest,
                        text_files: &text_files,
                    };
                    s_checking_dependency_content_digest(
                        s,
                        params,
                        &dep_comparison_params,
                        &mut dependency_changes,
                    )
                }
                XvcStepState::Done(s) => s_done(s, params),
                XvcStepState::Running(s) => s_running(s, params, &mut process_pool),
            };

            watch!(r_next_state);

            match r_next_state {
                Ok(state) => {
                    next_states.map.insert(*step_e, state.clone());
                }
                Err(e) => {
                    warn!("{}", e);
                    watch!(params_debug.step_command);
                    watch!(step_s);
                    continue_running = false;
                    break;
                }
            }
        }

        for (_, cp) in process_pool.iter() {
            let stdout = cp.stdout_receiver.borrow();
            let stderr = cp.stderr_receiver.borrow();
            stdout.try_iter().for_each(|m| warn!("[OUT] {}", m));
            stderr.try_iter().for_each(|m| warn!("[ERR] {}", m));
        }

        // update pmp with fs events
        //
        while let Ok(fs_event) = fs_receiver.try_recv() {
            watch!(fs_event);
            match fs_event {
                PathEvent::Create { path, metadata } => {
                    let xvc_path = XvcPath::new(xvc_root, xvc_root, &path)?;
                    let xvc_md = XvcMetadata::from(metadata);
                    pmm.insert(xvc_path, xvc_md);
                }
                PathEvent::Update { path, metadata } => {
                    let xvc_path = XvcPath::new(xvc_root, xvc_root, &path)?;
                    let xvc_md = XvcMetadata::from(metadata);
                    pmm.insert(xvc_path, xvc_md);
                }
                PathEvent::Delete { path } => {
                    let xvc_path = XvcPath::new(xvc_root, xvc_root, &path)?;
                    let xvc_md = XvcMetadata {
                        file_type: XvcFileType::RecordOnly,
                        size: None,
                        modified: None,
                    };
                    pmm.insert(xvc_path, xvc_md);
                }
            }
        }

        thread::sleep(Duration::from_millis(sleep_duration));

        step_states = next_states;

        // if any of the states are Broken, we leave the loop
        let broken_steps = step_states.iter().filter_map(|(step_e, step_s)| {
            if matches!(step_s, XvcStepState::Broken(_)) {
                Some(step_e)
            } else {
                None
            }
        });

        for step_e in broken_steps {
            warn!(
                "Broken Step: {:?} ({}) with Command {:?}",
                pipeline_steps[step_e], step_e, step_commands[step_e]
            );
            continue_running = false;
        }

        // if all of the steps are done, we can end
        if step_states
            .iter()
            .all(|(_, step_s)| matches!(step_s, XvcStepState::Done(_)))
        {
            continue_running = false;
        }
    }

    // TODO: Check if there are running processes in the process pool and terminate them (per option)
    Ok(())
}

fn s_checking_dependency_content_digest(
    s: &CheckingDependencyContentDigestState,
    params: StateParams,
    dependency_comparison_params: &DependencyComparisonParams,
    dependency_changes: &mut HStore<XvcDependencyChange>,
) -> Result<XvcStepState> {
    watch!(s);
    if params.run_conditions.ignore_content_digest_comparison {
        return Ok(s.content_digest_ignored());
    }
    let step_e = params.step_e;
    // PANIC: If RStore.left doesn't have `step_e` as key.
    let deps = params.all_dependencies.children_of(step_e).unwrap();

    let mut comparison_results = HStore::<XvcDependencyChange>::with_capacity(deps.len());

    // We update the comparison parameters as we iterate through the dependencies
    let cmp_params = dependency_comparison_params.clone();

    for (dep_e, _) in deps.iter() {
        // We wait step and pipeline dependencies in an earlier state
        let comparison_result = compare_deps(cmp_params.clone(), dep_e)?;
        comparison_results.map.insert(*dep_e, comparison_result);
    }
    if comparison_results.iter().all(|(_, dc)| {
        dc.updated_collection_digest.is_none()
            && dc.updated_metadata.is_none()
            && dc.updated_content_digests.is_none()
    }) {
        Ok(s.content_digest_not_changed())
    } else {
        // We'll update all elements
        dependency_changes
            .map
            .extend(comparison_results.map.drain());
        Ok(s.content_digest_changed())
    }
}

fn s_checking_timestamps(s: &CheckingTimestampsState, params: StateParams) -> Result<XvcStepState> {
    watch!(s);
    if params.run_conditions.ignore_timestamp_comparison {
        return Ok(s.timestamps_ignored());
    }
    let xvc_root = params.xvc_root;
    let step_e = params.step_e;
    let pipeline_rundir = params.pipeline_rundir;
    let deps = params.all_dependencies.children_of(step_e)?;
    let outs = params.all_outputs.children_of(step_e)?;
    let pmm = params.pmm;

    let dep_paths = deps
        .iter()
        .fold(XvcPathMetadataMap::new(), |mut collected, (_, dep)| {
            collected.extend(dependency_paths(xvc_root, pmm, pipeline_rundir, dep));
            collected
        });

    // no dependency paths means no newer dependency paths
    if dep_paths.is_empty() {
        return Ok(s.has_no_newer_dependencies());
    }

    let dep_modified = dep_paths.iter().map(|(path, md)| (path, md.modified));

    let max_dep_ts =
        dep_modified.fold(
            Some(SystemTime::UNIX_EPOCH),
            |opt_st, (path, modified)| match modified {
                None => {
                    Error::PathNotFoundInPathMetadataMap {
                        path: path.to_absolute_path(xvc_root).as_os_str().to_owned(),
                    }
                    .error();
                    None
                }
                Some(modified) => match opt_st {
                    None => None,
                    Some(max) => {
                        if modified > max {
                            Some(modified)
                        } else {
                            Some(max)
                        }
                    }
                },
            },
        );

    if let Some(max_dep_ts) = max_dep_ts {
        let out_paths = outs.iter().map(|(_, out)| {
            let path = XvcPath::from(out);
            let md = pmm.get(&path);
            (path, md)
        });

        let min_out_ts = out_paths.fold(
            Some(SystemTime::from(chrono::MAX_DATETIME)),
            |opt_st, (path, md)| match md {
                None => {
                    Error::PathNotFoundInPathMetadataMap {
                        path: path.to_absolute_path(xvc_root).as_os_str().to_owned(),
                    }
                    .error();
                    None
                }
                Some(md) => match opt_st {
                    None => None,
                    Some(min) => match md.modified {
                        None => {
                            Error::PathHasNoModificationTime {
                                path: path.to_absolute_path(xvc_root).as_os_str().to_owned(),
                            }
                            .error();
                            None
                        }
                        Some(modified) => {
                            if modified < min {
                                Some(modified)
                            } else {
                                Some(min)
                            }
                        }
                    },
                },
            },
        );
        if let Some(min_out_ts) = min_out_ts {
            if max_dep_ts >= min_out_ts {
                Ok(s.has_newer_dependencies())
            } else {
                Ok(s.has_no_newer_dependencies())
            }
        } else {
            Ok(s.has_newer_dependencies())
        }
    } else {
        // We can return an error in this case but this shouldn't happen anyway
        Ok(s.has_newer_dependencies())
    }
}

/// Checks whether a dependency is missing.
/// Note that this doesn't check URL dependencies as of now. We should add it though.
fn s_checking_missing_dependencies(
    s: &CheckingMissingDependenciesState,
    params: StateParams,
) -> Result<XvcStepState> {
    watch!(s);
    if params.run_conditions.ignore_missing_dependencies {
        return Ok(s.missing_dependencies_ignored());
    }

    let step_e = params.step_e;
    let pmm = params.pmm;
    let deps = params.all_dependencies.children_of(step_e)?;
    for (_, dep) in deps.iter() {
        if let Some(xvc_path) = dep.xvc_path() {
            watch!(xvc_path);
            match pmm.get(&xvc_path) {
                None => return Ok(s.has_missing_dependencies()),
                Some(xvc_md) => {
                    if xvc_md.file_type == XvcFileType::RecordOnly {
                        return Ok(s.has_missing_dependencies());
                    }
                }
            }
        }
    }
    Ok(s.no_missing_dependencies())
}

fn s_waiting_dependency_steps(
    s: &WaitingDependencyStepsState,
    params: StateParams,
) -> Result<XvcStepState> {
    watch!(s);
    if !params.run_conditions.wait_running_dep_steps {
        return Ok(s.dependency_steps_running_ignored());
    }
    let dependency_graph = params.dependency_graph;
    let dep_neighbors = dependency_graph.neighbors(*params.step_e);
    let dep_states = params.current_states.subset(dep_neighbors)?;
    // if there are no dependencies, we can claim successfully finished
    watch!(dep_states);
    if dep_states.len() == 0 {
        return Ok(s.dependency_steps_finished_successfully());
    }

    // if all dependencies are completed somehow (Done or Broken) move to checking run conditions
    if dep_states
        .iter()
        .all(|(_, dep_state)| matches!(dep_state, &XvcStepState::Done(_)))
    {
        Ok(s.dependency_steps_finished_successfully())
    } else if dep_states.iter().all(|(_, dep_state)| {
        matches!(dep_state, &XvcStepState::Done(_)) || matches!(dep_state, &XvcStepState::Broken(_))
    }) {
        if params.run_conditions.ignore_broken_dep_steps {
            Ok(s.dependency_steps_finished_broken_ignored())
        } else {
            Ok(s.dependency_steps_finished_broken())
        }
    } else {
        Ok(s.dependency_steps_running())
    }
}

fn s_no_need_to_run(s: &NoNeedToRunState, _params: StateParams) -> Result<XvcStepState> {
    watch!(s);
    Ok(s.completed_without_running_step())
}

/// Broken stays always Broken
fn s_broken(s: &BrokenState, _params: StateParams) -> Result<XvcStepState> {
    watch!(s);
    Ok(s.has_broken())
}

fn s_running(
    s: &RunningState,
    params: StateParams,
    process_pool: &mut HStore<CommandProcess>,
) -> Result<XvcStepState> {
    // Check whether the process is still running
    watch!(s);
    let step_e = params.step_e;
    let command_process = &mut process_pool.get(step_e).unwrap();
    let process_rc = command_process.process.clone();
    let mut process = process_rc.borrow_mut();
    let stdout_sender = command_process.stdout_sender.borrow_mut();
    let stderr_sender = command_process.stderr_sender.borrow_mut();
    let timeout = params.step_timeout;
    let birth = command_process.birth;
    // unload the process outputs
    // we can move these to the main loop
    let (process_stdout, process_stderr) = match process.communicate(None) {
        Ok((opt_out, opt_err)) => (
            opt_out.unwrap_or_else(|| "".into()),
            opt_err.unwrap_or_else(|| "".into()),
        ),
        // TODO: convert to Error
        Err(err) => (
            "".to_string(),
            format!("Process communication error: {:?}", err),
        ),
    };

    watch!(&process_stdout);
    watch!(&process_stderr);

    if !process_stdout.is_empty() {
        stdout_sender.send(process_stdout)?;
    }

    if !process_stderr.is_empty() {
        stderr_sender.send(process_stderr)?;
    }

    match process.poll() {
        // Still running:
        None => {
            if birth.elapsed() < *timeout {
                Ok(s.wait_process())
            } else {
                if params.terminate_timeout_processes {
                    process.terminate()?;
                }
                // drop senders
                drop(stdout_sender);
                drop(stderr_sender);
                Ok(s.process_timeout())
            }
        }

        Some(exit_code) => match exit_code {
            ExitStatus::Exited(0) => {
                stdout_sender.send("[EXIT] Successfully".to_string())?;
                drop(stdout_sender);
                drop(stderr_sender);
                Ok(s.process_completed_successfully())
            }
            ,
            // we don't handle other variants in the state machine. Either it exited
            // successfully or died for some reason.
            //
            _ => {
                stderr_sender.send("[EXIT] Non-Successfully".to_string())?;
                drop(stdout_sender);
                drop(stderr_sender);
                Ok(s.process_returned_non_zero())
            },
        },
    }
}

/// We don't pass params mutable, so
/// in order to run a process and add it to the pool, this one receives `process_pool` as a
/// mutable reference.
fn s_waiting_to_run(
    s: &WaitingToRunState,
    params: StateParams,
    process_pool: &mut HStore<CommandProcess>,
) -> Result<XvcStepState> {
    watch!(s);
    let all_states = params.current_states;
    let step_e = params.step_e;
    let n_running = all_states
        .iter()
        .filter(|(_, dep_state)| matches!(dep_state, XvcStepState::Running(_)))
        .count();
    if n_running <= params.running_process_pool_size {
        // Start process and add to pool
        // WARNING: We use `shell` instead of `cmd` here to run the command in default shell
        let process = sp::Exec::shell(&params.step_command.command)
            .stdout(sp::Redirection::Pipe)
            .stderr(sp::Redirection::Pipe)
            .stdin(sp::Redirection::None)
            .detached()
            .popen();
        // TODO: Add environment variable and stdin support
        match process {
            Ok(p) => {
                let (stdout_sender, stdout_receiver) =
                    crossbeam_channel::bounded(params.log_channel_size);
                let (stderr_sender, stderr_receiver) =
                    crossbeam_channel::bounded(params.log_channel_size);
                process_pool.map.insert(
                    *step_e,
                    CommandProcess {
                        process: Rc::new(RefCell::new(p)),
                        birth: Instant::now(),
                        stdout_sender: RefCell::new(stdout_sender),
                        stderr_sender: RefCell::new(stderr_sender),
                        stdout_receiver: RefCell::new(stdout_receiver),
                        stderr_receiver: RefCell::new(stderr_receiver),
                    },
                );
                Ok(s.start_process())
            }
            Err(e) => {
                Error::ProcessError {
                    stdout: format!("[ERR] Cannot Start Process: {:?}", e),
                    stderr: format!("[ERR] Cannot Start Process: {:?}", e),
                }
                .warn();
                Ok(s.cannot_start_process())
            }
        }
    } else {
        // We don't check timeout here, timeouts are checked in the running process
        Ok(s.process_pool_full())
    }
}

/// Terminal state: Waits till the end of times
fn s_done(s: &DoneState, _params: StateParams) -> Result<XvcStepState> {
    watch!(s);
    Ok(s.has_done())
}

fn s_checking_missing_outputs(
    s: &CheckingMissingOutputsState,
    params: StateParams,
) -> Result<XvcStepState> {
    watch!(s);
    let step_e = params.step_e;
    let run_conditions = params.run_conditions;
    let step_outs = params.all_outputs.children_of(step_e)?;
    let pmm = params.pmm;

    if run_conditions.run_when_outputs_missing {
        for out in step_outs.values() {
            let out_path = XvcPath::from(out);
            if !pmm.contains_key(&out_path) {
                return Ok(s.has_missing_outputs());
            }
        }
        // if we reach here, we don't have missing outputs
        Ok(s.has_no_missing_outputs())
    } else {
        Ok(s.missing_outputs_ignored())
    }
}

fn s_begin(s: &BeginState, params: StateParams) -> Result<XvcStepState> {
    // checking whether we run this step or not
    watch!(s);
    if params.run_conditions.never {
        Ok(s.run_never())
    } else {
        Ok(s.run_conditional())
    }
}
