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
use anyhow::anyhow;
use clap::Command;
use xvc_file::CHANNEL_CAPACITY;

use crate::deps::compare::{compare_dependency, DependencyComparisonParams};
use crate::deps::{dependencies_to_path, dependency_paths};
use crate::error::{Error, Result};
use crate::pipeline::command::CommandProcess;
use crate::{XvcPipeline, XvcPipelineRunDir};

use chrono::Utc;
use crossbeam_channel::{bounded, Receiver, Select, Sender};
use petgraph::Direction;
use xvc_logging::{debug, error, info, output, uwr, warn, watch, XvcOutputSender};
use xvc_walker::notify::{make_watcher, PathEvent};

use petgraph::algo::toposort;
use petgraph::data::Build;
use petgraph::dot::Dot;
use petgraph::prelude::DiGraphMap;

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle, ScopedJoinHandle};
use std::time::{Duration, Instant, SystemTime};
use strum_macros::{Display, EnumString};
use xvc_config::FromConfigKey;
use xvc_core::{
    all_paths_and_metadata, apply_diff, AttributeDigest, ContentDigest, Diff, HashAlgorithm,
    PathCollectionDigest, TextOrBinary, XvcDigests, XvcFileType, XvcMetadata, XvcPath,
    XvcPathMetadataMap, XvcRoot,
};

use xvc_ecs::{persist, HStore, R1NStore, XvcEntity, XvcStore};

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
/// All steps depend on the `start_step_entity` step that's run always. It's used to collect all independent (parallel)
/// steps into the graph.
pub fn add_explicit_dependencies(
    output_snd: &XvcOutputSender,
    pipeline_steps: &HStore<XvcStep>,
    all_deps: &R1NStore<XvcStep, XvcDependency>,
    graph: &mut DiGraphMap<XvcEntity, XvcDependency>,
) -> Result<()> {
    for (from_step_e, from_step) in pipeline_steps.iter() {
        let deps = all_deps.children_of(from_step_e)?;
        for (_to_step_e, to_step) in deps.iter() {
            if let XvcDependency::Step(step_dep) = to_step {
                let candidate_step = XvcStep {
                    name: step_dep.name.to_string(),
                };
                match pipeline_steps.entity_by_value(&candidate_step) {
                    // We don't check from_step == to_step equality here as we'll check cycles later
                    Some(entity) => {
                        graph.update_edge(*from_step_e, entity, to_step.clone());
                        info!(
                            output_snd,
                            "Found explicit dependency: {:?} -> {:?}", from_step, to_step
                        );
                    }
                    None => {
                        return Err(Error::StepNotFoundInPipeline {
                            step: step_dep.name.to_string(),
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
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    pmm: &XvcPathMetadataMap,
    pipeline_rundir: &XvcPath,
    all_deps: &R1NStore<XvcStep, XvcDependency>,
    all_outs: &R1NStore<XvcStep, XvcOutput>,
    pipeline_steps: &HStore<XvcStep>,
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
                        output_snd,
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
    ignore_diffs: bool,
    run_when_outputs_missing: bool,
}
#[derive(Debug, Clone)]
struct StepStateParams<'a> {
    xvc_root: &'a XvcRoot,
    output_snd: &'a XvcOutputSender,
    pmm: Arc<RwLock<XvcPathMetadataMap>>,
    run_conditions: &'a RunConditions,
    pipeline_rundir: &'a XvcPath,
    terminate_timeout_processes: bool,
    algorithm: HashAlgorithm,

    command_process: Arc<RwLock<CommandProcess>>,
    runnable_process_count: Arc<RwLock<usize>>,

    step_e: XvcEntity,
    step: &'a XvcStep,
    step_command: &'a XvcStepCommand,
    dependency_states: Arc<RwLock<HStore<XvcStepState>>>,
    step_timeout: &'a Duration,

    step_dependencies: &'a HStore<XvcDependency>,
    step_outputs: &'a HStore<XvcOutput>,
    step_xvc_digests: &'a HStore<XvcDigests>,
}

/// This structure is passed to step_threads as a parameter.
/// It contains all the information needed to run a step.
#[derive(Debug, Clone)]
struct StepThreadParams<'a> {
    xvc_root: &'a XvcRoot,
    output_snd: &'a XvcOutputSender,
    pipeline_rundir: &'a XvcPath,
    step_e: XvcEntity,
    state_channels: &'a HStore<(Sender<Option<XvcStepState>>, Receiver<Option<XvcStepState>>)>,
    step_commands: &'a XvcStore<XvcStepCommand>,
    steps: &'a HStore<XvcStep>,
    dependency_graph: &'a DependencyGraph,
    step_timeout: &'a Duration,
    run_conditions: &'a RunConditions,
    terminate_on_timeout: bool,
    algorithm: HashAlgorithm,
    current_pmm: Arc<RwLock<XvcPathMetadataMap>>,
    process_pool: Arc<RwLock<HStore<CommandProcess>>>,
    process_pool_size: usize,
    recorded_dependencies: &'a R1NStore<XvcStep, XvcDependency>,
    recorded_outputs: &'a R1NStore<XvcStep, XvcOutput>,
    recorded_xvc_digests: &'a R1NStore<XvcStep, XvcDigests>,
}

type XvcDependencyDiff = Diff<XvcDigests>;

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

type StateTransition<'a> = Result<(XvcStepState, StepStateParams<'a>)>;

pub fn the_grand_pipeline_loop(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    pipeline_name: String,
) -> Result<()> {
    let config = xvc_root.config();
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
    watch!(pipeline_len);

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

    // Add all steps as nodes to the pipeline
    for (step_e, _) in pipeline_steps.iter() {
        dependency_graph.add_node(*step_e);
    }

    add_explicit_dependencies(
        output_snd,
        &pipeline_steps,
        &all_deps,
        &mut dependency_graph,
    )?;
    watch!(&dependency_graph);
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

    let debug_output = Dot::new(&dependency_graph);

    info!("Pipeline Graph:\n{}\n", debug_output);
    // Topological sort to get the cycles and run order
    let sorted_steps = match toposort(&dependency_graph, None) {
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
        ignore_diffs: false,
    };

    //  This is the DVC behavior. It doesn't run when _only_ dependency timestamp changed. For
    //  Makefile behavior `dependencies_new` can be set to `true`.
    let run_calculated = RunConditions {
        never: false,
        wait_running_dep_steps: true,
        ignore_broken_dep_steps: false,
        run_when_outputs_missing: true,
        ignore_missing_dependencies: false,
        ignore_diffs: false,
    };

    let run_always = RunConditions {
        never: false,
        run_when_outputs_missing: true,
        ignore_missing_dependencies: true,
        wait_running_dep_steps: true,
        ignore_broken_dep_steps: true,
        ignore_diffs: true,
    };

    let run_conditions: HStore<RunConditions> = pipeline_steps
        .iter()
        .map(|(step_e, _)| {
            match consider_changed[step_e] {
                // If the step has no dependencies, we run it always
                XvcStepInvalidate::ByDependencies => {
                    let step_deps = uwr!(all_deps.children_of(step_e), output_snd);
                    if step_deps.is_empty() {
                        (*step_e, run_always)
                    } else {
                        (*step_e, run_calculated)
                    }
                }
                XvcStepInvalidate::Always => (*step_e, run_always),
                XvcStepInvalidate::Never => (*step_e, run_never),
            }
        })
        .collect();

    let mut step_states: HStore<XvcStepState> = pipeline_steps
        .iter()
        .map(|(step_e, _)| (*step_e, step::XvcStepState::begin()))
        .collect();

    watch!(step_states);

    let mut continue_running = true;
    // the following definitions should be moved to config
    // let break_for_nonzero_exit = true;
    let process_pool_size = 1;
    let sleep_duration = 10;
    let log_channel_size = 1000;
    let default_step_timeout: u64 = 10000;
    let terminate_on_timeout = true;
    let step_timeouts: HStore<Duration> = pipeline_steps
        .keys()
        .map(|step_e| (*step_e, Duration::from_secs(default_step_timeout)))
        .collect();

    let process_pool = Arc::new(RwLock::new(HStore::<CommandProcess>::with_capacity(
        pipeline_len,
    )));

    let current_pmm = Arc::new(RwLock::new(pmm.clone()));

    let step_commands = xvc_root.load_store::<XvcStepCommand>()?;

    let stored_dependency_paths = xvc_root.load_r1nstore::<XvcDependency, XvcPath>()?;
    let mut updated_dependencies = all_deps.children.clone();
    let xvc_path_store: XvcStore<XvcPath> = xvc_root.load_store()?;
    let mut updated_xvc_path_store = xvc_path_store.clone();
    let xvc_metadata_store: XvcStore<XvcMetadata> = xvc_root.load_store()?;
    let mut updated_xvc_metadata_store = xvc_metadata_store.clone();
    let xvc_digests_store: XvcStore<XvcDigests> = xvc_root.load_store()?;
    let mut updated_xvc_digests_store = xvc_digests_store.clone();
    let text_files = xvc_root.load_store::<TextOrBinary>()?;
    let algorithm = HashAlgorithm::from_conf(config);

    let state_channels = sorted_steps
        .iter()
        .map(|step_e| (*step_e, bounded(CHANNEL_CAPACITY)))
        .collect();
    let step_timeout = Duration::from_secs(default_step_timeout);

    let recorded_dependencies = xvc_root
        .load_r1nstore::<XvcStep, XvcDependency>()
        .expect("Cannot load store");
    let recorded_outputs = xvc_root
        .load_r1nstore::<XvcStep, XvcOutput>()
        .expect("Cannot load store");
    let recorded_xvc_digests = xvc_root
        .load_r1nstore::<XvcStep, XvcDigests>()
        .expect("Cannot load store");

    // Create a thread for each of the steps
    // We create these in topological order because the dependents need to subscribe to their dependencies' channels.
    let done_successfully: Result<bool> = thread::scope(|s| {
        let step_thread_store: HStore<ScopedJoinHandle<_>> = sorted_steps
            .iter()
            .map(|step_e| {
                (
                    *step_e,
                    s.spawn(|| {
                        let output_snd = output_snd.clone();
                        let step_thread_params = StepThreadParams {
                            xvc_root,
                            pipeline_rundir: &pipeline_rundir,
                            step_e: *step_e,
                            state_channels: &state_channels,
                            dependency_graph: &dependency_graph,
                            step_timeout: &step_timeout,
                            run_conditions: &run_conditions[step_e],
                            terminate_on_timeout,
                            current_pmm: current_pmm.clone(),
                            output_snd: &output_snd,
                            step_commands: &step_commands,
                            steps: &pipeline_steps,
                            process_pool: process_pool.clone(),
                            process_pool_size,
                            algorithm,
                            recorded_dependencies: &recorded_dependencies,
                            recorded_outputs: &recorded_outputs,
                            recorded_xvc_digests: &recorded_xvc_digests,
                        };
                        step_state_handler(*step_e, step_thread_params)
                    }),
                )
            })
            .collect();
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
        }

        // if all of the steps are done, we can end
        if step_states
            .iter()
            .all(|(_, step_s)| matches!(step_s, XvcStepState::Done(_)))
        {
            return Ok(true);
        } else {
            return Ok(false);
        }
    });
    // We only save the stores if the pipeline was run successfully
    if let Ok(true) = done_successfully {
        xvc_root.save_store(&updated_xvc_path_store)?;
        xvc_root.save_store(&updated_xvc_metadata_store)?;
        xvc_root.save_store(&updated_xvc_digests_store)?;
        xvc_root.save_store(&updated_dependencies)?;
    }
    Ok(())
}

fn dependencies(step_e: XvcEntity, dependency_graph: &DependencyGraph) -> Result<Vec<XvcEntity>> {
    let dep_neighbors = dependency_graph.neighbors(step_e);
    let mut dependencies = Vec::new();
    for dep_neighbor in dep_neighbors {
        dependencies.push(dep_neighbor);
    }
    Ok(dependencies)
}

type StateTransitionMap<'a> =
    HashMap<XvcStepState, dyn FnOnce(&XvcStepState, StepStateParams) -> StateTransition<'a>>;

fn step_state_handler(step_e: XvcEntity, params: StepThreadParams) -> Result<()> {
    let dependencies = dependencies(step_e, &params.dependency_graph)?;
    let dependency_channels = params.state_channels.subset(dependencies.iter().cloned())?;
    let dependency_receivers: Vec<(XvcEntity, &Receiver<Option<XvcStepState>>)> =
        dependency_channels
            .iter()
            .map(|(e, (s, r))| (*e, r))
            .collect();
    // We'll update these states internally and check for actions
    let mut dependency_states = Arc::new(RwLock::new(
        dependencies
            .iter()
            .map(|e| (*e, XvcStepState::begin()))
            .collect(),
    ));
    let mut sel = Select::new();
    dependency_receivers.iter().for_each(|(_, r)| {
        sel.recv(r);
    });

    let step_state_sender = params.state_channels.get(&step_e).unwrap().0.clone();
    let mut step_state = XvcStepState::begin();
    let step_dependencies = params.recorded_dependencies.children_of(&step_e)?;
    let step_outputs = params.recorded_outputs.children_of(&step_e)?;
    let step_xvc_digests = params.recorded_xvc_digests.children_of(&step_e)?;
    let step = &params.steps[&step_e];
    let step_command = &params.step_commands[&step_e];
    let command_process = Arc::new(RwLock::new(CommandProcess::new(step, step_command)));

    let mut params = StepStateParams {
        step_e,
        step,
        dependency_states: dependency_states.clone(),
        output_snd: &params.output_snd,
        algorithm: params.algorithm,
        step_command,
        command_process,
        runnable_process_count: Arc::new(RwLock::new(0)),
        terminate_timeout_processes: params.terminate_on_timeout,
        step_timeout: params.step_timeout,
        run_conditions: &params.run_conditions,
        xvc_root: params.xvc_root,
        pipeline_rundir: params.pipeline_rundir,
        pmm: params.current_pmm,
        step_dependencies: &step_dependencies,
        step_outputs: &step_outputs,
        step_xvc_digests: &step_xvc_digests,
    };

    loop {
        // If we don't have dependencies, we simply run the step
        if dependencies.len() > 0 {
            // Block until a receive operation becomes ready and try executing it.
            let index = sel.ready();
            let res = dependency_receivers[index].1.try_recv()?;
            if let Some(state) = res {
                dependency_states
                    .write()?
                    .insert(dependency_receivers[index].0, state);
            } else {
                // Dependency channels are closed due to an error. We're closing too.
                step_state_sender.send(None)?;
                return Err(anyhow!("Dependency channels closed due to an error.").into());
            }
        }

        let (r_next_state, next_params) = match step_state {
            XvcStepState::Begin(s) => match s {
                BeginState::FromInit => s_begin_f_init(&s, params)?,
            },

            XvcStepState::NoNeedToRun(s) => match s {
                NoNeedToRunState::FromRunNever => s_no_need_to_run_f_run_never(&s, params)?,
                NoNeedToRunState::FromHasNoNewerDependencies => {
                    s_no_need_to_run_f_has_no_newer_dependencies(&s, params)?
                }
                NoNeedToRunState::FromThoroughDiffsNotChanged => {
                    s_no_need_to_run_f_thorough_diffs_not_changed(&s, params)?
                }
            },
            XvcStepState::WaitingDependencySteps(s) => match s {
                WaitingDependencyStepsState::FromDependencyStepsRunning => {
                    s_waiting_dependency_steps_f_dependency_steps_running(&s, params)?
                }
                WaitingDependencyStepsState::FromRunConditional => {
                    s_waiting_dependency_steps_f_run_conditional(&s, params)?
                }
            },

            XvcStepState::CheckingMissingDependencies(s) => match s {
                CheckingMissingDependenciesState::FromDependencyStepsFinishedBrokenIgnored => {
                    s_checking_missing_dependencies_f_dependency_steps_finished_broken_ignored(
                        &s, params,
                    )?
                }
                CheckingMissingDependenciesState::FromDependencyStepsFinishedSuccessfully => {
                    s_checking_missing_dependencies_f_dependency_steps_finished_successfully(
                        &s, params,
                    )?
                }
            },
            XvcStepState::CheckingMissingOutputs(s) => match s {
                CheckingMissingOutputsState::FromMissingDependenciesIgnored => {
                    s_checking_missing_outputs_f_missing_dependencies_ignored(&s, params)?
                }
                CheckingMissingOutputsState::FromNoMissingDependencies => {
                    s_checking_missing_outputs_f_no_missing_dependencies(&s, params)?
                }
            },
            XvcStepState::CheckingShallowDiffs(s) => match s {
                CheckingShallowDiffsState::FromHasMissingOutputs => {
                    s_checking_shallow_diffs_f_has_missing_outputs(&s, params)?
                }
                CheckingShallowDiffsState::FromMissingOutputsIgnored => {
                    s_checking_shallow_diffs_f_missing_outputs_ignored(&s, params)?
                }
                CheckingShallowDiffsState::FromHasNoMissingOutputs => {
                    s_checking_shallow_diffs_f_no_missing_outputs(&s, params)?
                }
            },
            XvcStepState::WaitingToRun(s) => match s {
                WaitingToRunState::FromThoroughDiffsChanged => {
                    s_waiting_to_run_f_thorough_diffs_changed(&s, params)?
                }
                WaitingToRunState::FromDiffsIgnored => {
                    s_waiting_to_run_f_diffs_ignored(&s, params)?
                }
                WaitingToRunState::FromProcessPoolFull => {
                    s_waiting_to_run_f_process_pool_full(&s, params)?
                }
            },
            XvcStepState::CheckingThoroughDiffs(s) => match s {
                CheckingThoroughDiffsState::FromHasNewerDependencies => {
                    s_checking_thorough_diffs_f_has_newer_dependencies(&s, params)?
                }
            },
            XvcStepState::Running(s) => match s {
                RunningState::FromStartProcess => s_running_f_start_process(&s, params)?,
                RunningState::FromWaitProcess => s_running_f_wait_process(&s, params)?,
            },
            XvcStepState::Broken(s) => match s {
                BrokenState::FromCannotStartProcess => s_broken_f_cannot_start_process(&s, params)?,
                BrokenState::FromHasMissingDependencies => {
                    s_broken_f_has_missing_dependencies(&s, params)?
                }
                BrokenState::FromDependencyStepsFinishedBroken => {
                    s_broken_f_dependency_steps_finished_broken(&s, params)?
                }
                BrokenState::FromProcessTimeout => s_broken_f_process_timeout(&s, params)?,
                BrokenState::FromProcessReturnedNonZero => {
                    s_broken_f_process_returned_non_zero(&s, params)?
                }
                BrokenState::FromKeepBroken => (XvcStepState::Broken(s), params),
            },
            XvcStepState::Done(s) => match s {
                DoneState::FromCompletedWithoutRunningStep => {
                    s_done_f_completed_without_running_step(&s, params)?
                }
                DoneState::FromProcessCompletedSuccessfully => {
                    s_done_f_process_completed_successfully(&s, params)?
                }
                DoneState::FromKeepDone => (XvcStepState::Done(s), params),
            },
        };

        step_state_sender.send(Some(r_next_state.clone()))?;
        step_state = r_next_state;

        params = next_params;

        match &step_state {
            XvcStepState::Done(_) | XvcStepState::Broken(_) => {
                // We're done. No need to keep the step state channel open.
                step_state_sender.send(None)?;
                return Ok(());
            }
            _ => {
                // We're not done yet. Keep the step state channel open.
            }
        }
    }
}

fn update_pmp(
    xvc_root: &XvcRoot,
    fs_receiver: Receiver<Option<PathEvent>>,
    pmm: &mut XvcPathMetadataMap,
) -> Result<()> {
    while let Ok(Some(fs_event)) = fs_receiver.try_recv() {
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
                    file_type: XvcFileType::Missing,
                    size: None,
                    modified: None,
                };
                pmm.insert(xvc_path, xvc_md);
            }
        }
    }

    Ok(())
}

fn s_checking_thorough_diffs_f_has_newer_dependencies<'a>(
    s: &CheckingThoroughDiffsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    s_checking_thorough_diffs(s, params)
}

fn s_checking_shallow_diffs_f_no_missing_outputs<'a>(
    s: &CheckingShallowDiffsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    s_checking_shallow_diffs(s, params)
}

fn s_checking_shallow_diffs_f_missing_outputs_ignored<'a>(
    s: &CheckingShallowDiffsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    s_checking_shallow_diffs(s, params)
}

fn s_checking_shallow_diffs_f_has_missing_outputs<'a>(
    s: &CheckingShallowDiffsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    s_checking_shallow_diffs(s, params)
}

fn s_checking_missing_outputs_f_no_missing_dependencies<'a>(
    s: &CheckingMissingOutputsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    s_checking_missing_outputs(s, params)
}

fn s_checking_missing_outputs<'a>(
    s: &CheckingMissingOutputsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    let run_conditions = params.run_conditions;
    let step_outs = params.step_outputs;
    let pmm = params.pmm.clone();

    if run_conditions.run_when_outputs_missing {
        for out in step_outs.values() {
            let out_path = XvcPath::from(out);
            if !pmm.read()?.contains_key(&out_path) {
                return Ok((s.has_missing_outputs(), params));
            }
        }
        // if we reach here, we don't have missing outputs
        Ok((s.has_no_missing_outputs(), params))
    } else {
        Ok((s.missing_outputs_ignored(), params))
    }
}

fn s_checking_missing_outputs_f_missing_dependencies_ignored<'a>(
    s: &CheckingMissingOutputsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    s_checking_missing_outputs(s, params)
}

/// Checks whether a dependency is missing.
/// Note that this doesn't check URL dependencies as of now. We should add it though.
fn s_checking_missing_dependencies<'a>(
    s: &CheckingMissingDependenciesState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    if params.run_conditions.ignore_missing_dependencies {
        info!(
            params.output_snd,
            "Ignoring missing dependencies for step {}", params.step.name
        );
        return Ok((s.missing_dependencies_ignored(), params));
    }

    let pmm = params.pmm.clone();
    let deps = params.step_dependencies;
    for (_, dep) in deps.iter() {
        if let Some(xvc_path) = dep.xvc_path() {
            match pmm.read()?.get(&xvc_path) {
                None => return Ok((s.has_missing_dependencies(), params)),
                Some(xvc_md) => {
                    if xvc_md.file_type == XvcFileType::Missing {
                        return Ok((s.has_missing_dependencies(), params));
                    }
                }
            }
        }
    }
    Ok((s.no_missing_dependencies(), params))
}

fn s_checking_missing_dependencies_f_dependency_steps_finished_successfully<'a>(
    s: &CheckingMissingDependenciesState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    s_checking_missing_dependencies(s, params)
}

fn s_checking_missing_dependencies_f_dependency_steps_finished_broken_ignored<'a>(
    s: &CheckingMissingDependenciesState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    s_checking_missing_dependencies(s, params)
}

fn s_waiting_dependency_steps_f_dependency_steps_running<'a>(
    s: &WaitingDependencyStepsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    let dep_states = params.dependency_states.clone();

    // if all dependencies are completed somehow (Done or Broken) move to checking run conditions
    if dep_states
        .read()?
        .iter()
        .all(|(_, dep_state)| matches!(dep_state, &XvcStepState::Done(_)))
    {
        info!(
            params.output_snd,
            "Dependency steps completed successfully for step {}", params.step.name
        );
        Ok((s.dependency_steps_finished_successfully(), params))
    } else if dep_states.read()?.iter().all(|(_, dep_state)| {
        matches!(dep_state, &XvcStepState::Done(_)) || matches!(dep_state, &XvcStepState::Broken(_))
    }) {
        if params.run_conditions.ignore_broken_dep_steps {
            info!(
                params.output_snd,
                "Dependency steps completed for step {} (ignoring broken steps)", params.step.name
            );
            Ok((s.dependency_steps_finished_broken_ignored(), params))
        } else {
            info!(
                params.output_snd,
                "Dependency steps are broken for step {}", params.step.name
            );
            Ok((s.dependency_steps_finished_broken(), params))
        }
    } else {
        debug!(
            params.output_snd,
            "Dependency steps are running for step {}", params.step.name
        );
        Ok((s.dependency_steps_running(), params))
    }
}

fn s_no_need_to_run_f_thorough_diffs_not_changed<'a>(
    s: &NoNeedToRunState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    info!(
        params.output_snd,
        "Dependencies for step {} hasn't changed. Skipping.", params.step.name
    );
    Ok((s.completed_without_running_step(), params))
}

fn s_no_need_to_run_f_has_no_newer_dependencies<'a>(
    s: &NoNeedToRunState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    info!(
        params.output_snd,
        "Step {} has no newer dependencies. Skipping.", params.step.name
    );
    Ok((s.completed_without_running_step(), params))
}

fn s_checking_thorough_diffs<'a>(
    s: &CheckingThoroughDiffsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    let step_e = params.step_e;
    watch!(step_e);
    let deps = params.step_dependencies;
    watch!(deps);

    watch!(deps.is_empty());
    // Normally this should be checked in the previous state, but we check it here just in case
    if deps.is_empty() {
        return Ok((s.thorough_diffs_changed(), params));
    }

    // Calculate all diffs for dependencies for this step
    let step_dependency_diffs: HStore<Diff<XvcDependency>> = params
        .step_dependencies
        .iter()
        .map(|(dep_e, dep)| {
            let cmp_diff = uwr!(compare_dependency(&params, *dep_e), params.output_snd);
            (*dep_e, cmp_diff)
        })
        .collect();

    if step_dependency_diffs.iter().any(|(_, d)| d.changed()) {
        Ok((s.thorough_diffs_changed(), params))
    } else {
        Ok((s.thorough_diffs_not_changed(), params))
    }
}

fn s_checking_shallow_diffs<'a>(
    s: &CheckingShallowDiffsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    if params.run_conditions.ignore_diffs {
        return Ok((s.diffs_ignored(), params));
    }

    let xvc_root = params.xvc_root;
    let step_e = params.step_e;
    let pipeline_rundir = params.pipeline_rundir;
    let deps = params.step_dependencies;
    let outs = params.step_outputs;
    let pmm = params.pmm.clone();

    let step_dependency_diffs: HStore<Diff<XvcDependency>> = params
        .step_dependencies
        .iter()
        .map(|(dep_e, dep)| {
            let cmp_diff = uwr!(compare_dependency(&params, *dep_e), params.output_snd);
            (*dep_e, cmp_diff)
        })
        .collect();
    let dep_paths = deps
        .iter()
        .fold(XvcPathMetadataMap::new(), |mut collected, (_, dep)| {
            collected.extend(dependency_paths(
                xvc_root,
                uwr!(&pmm.read(), params.output_snd),
                pipeline_rundir,
                dep,
            ));
            collected
        });

    // no dependency paths means no newer dependency paths
    if dep_paths.is_empty() {
        return Ok((s.has_no_newer_dependencies(), params));
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
        let pmm = pmm.clone();
        let pmm = uwr!(pmm.read(), params.output_snd);
        let out_paths = outs.iter().map(|(_, out)| {
            let path = XvcPath::from(out);
            let md = pmm.get(&path);
            (path, md.cloned())
        });

        let min_out_ts = out_paths.fold(
            Some((chrono::DateTime::<Utc>::MAX_UTC).into()),
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
                Ok((s.has_newer_dependencies(), params))
            } else {
                Ok((s.has_no_newer_dependencies(), params))
            }
        } else {
            Ok((s.has_newer_dependencies(), params))
        }
    } else {
        // We can return an error in this case but this shouldn't happen anyway
        Ok((s.has_newer_dependencies(), params))
    }
}

fn s_waiting_dependency_steps<'a>(
    s: &WaitingDependencyStepsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    if !params.run_conditions.wait_running_dep_steps {
        return Ok((s.dependency_steps_finished_successfully(), params));
    }
    let dep_states = params.dependency_states.clone();
    // if there are no dependencies, we can claim successfully finished
    if dep_states.read()?.len() == 0 {
        return Ok((s.dependency_steps_finished_successfully(), params));
    }

    // if all dependencies are completed somehow (Done or Broken) move to checking run conditions
    if dep_states
        .read()?
        .iter()
        .all(|(_, dep_state)| matches!(dep_state, &XvcStepState::Done(_)))
    {
        Ok((s.dependency_steps_finished_successfully(), params))
    } else if dep_states.read()?.iter().all(|(_, dep_state)| {
        matches!(dep_state, &XvcStepState::Done(_)) || matches!(dep_state, &XvcStepState::Broken(_))
    }) {
        if params.run_conditions.ignore_broken_dep_steps {
            Ok((s.dependency_steps_finished_broken_ignored(), params))
        } else {
            Ok((s.dependency_steps_finished_broken(), params))
        }
    } else {
        Ok((s.dependency_steps_running(), params))
    }
}

fn s_running_f_start_process<'a>(
    s: &RunningState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    let command_process = params.command_process.clone();
    let mut command_process = command_process.write()?;
    command_process.run()?;
    Ok((s.wait_process(), params))
}

fn s_running_f_wait_process<'a>(
    s: &RunningState,
    mut params: StepStateParams<'a>,
) -> StateTransition<'a> {
    // Check whether the process is still running
    let mut command_process = params.command_process.write()?;

    command_process.update_output_channels()?;

    // We currently pass all the output to the main thread
    // In the future, these can be passed to different channels.
    let output_snd = params.output_snd;
    command_process
        .stderr_receiver
        .try_iter()
        .for_each(|out| warn!(output_snd, "{}", out));

    command_process
        .stdout_receiver
        .try_iter()
        .for_each(|out| output!(output_snd, "{}", out));

    let mut process = command_process
        .process
        .ok_or(anyhow!("Process not found?"))?;
    let birth = command_process
        .birth
        .ok_or(anyhow!("Process birth not found"))?;
    let timeout = params.step_timeout;

    match process.poll() {
        // Still running:
        None => {
            if birth.elapsed() < *timeout {
                Ok((s.wait_process(), params))
            } else {
                if params.terminate_timeout_processes {
                    error!(
                        output_snd,
                        "Process timeout for step {} with command {} ",
                        command_process.step.name,
                        command_process.step_command
                    );
                    process.terminate()?;
                }
                Ok((s.process_timeout(), params))
            }
        }

        Some(exit_code) => match exit_code {
            ExitStatus::Exited(0) => {
                info!(output_snd, "Step {} finished successfully with command {}", command_process.step.name, command_process.step_command);
                Ok((s.process_completed_successfully(), params))
            }
            ,
            // we don't handle other variants in the state machine. Either it exited
            // successfully or died for some reason.
            //
            _ => {
                error!(output_snd, "Step {} finished UNSUCCESSFULLY with command {}", command_process.step.name, command_process.step_command);
                Ok((s.process_returned_non_zero(), params))
            },
        },
    }
}

fn s_waiting_to_run_f_process_pool_full<'a>(
    s: &WaitingToRunState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    if params.runnable_process_count.read()?.gt(&0) {
        Ok((s.start_process(), params))
    } else {
        Ok((s.process_pool_full(), params))
    }
}

fn s_waiting_to_run_f_diffs_ignored<'a>(
    s: &WaitingToRunState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    todo!()
}

fn s_waiting_to_run_f_thorough_diffs_changed<'a>(
    s: &WaitingToRunState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    todo!()
}

/// Broken stays always Broken
fn s_broken_f_process_returned_non_zero<'a>(
    s: &BrokenState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    Ok((s.keep_broken(), params))
}

fn s_broken_f_process_timeout<'a>(s: &BrokenState, params: StepStateParams) -> StateTransition<'a> {
    Ok((s.keep_broken(), params))
}

fn s_broken_f_dependency_steps_finished_broken<'a>(
    s: &BrokenState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    Ok((s.keep_broken(), params))
}

fn s_broken_f_has_missing_dependencies<'a>(
    s: &BrokenState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    Ok((s.keep_broken(), params))
}

fn s_broken_f_cannot_start_process<'a>(
    s: &BrokenState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    Ok((s.keep_broken(), params))
}

fn s_begin_f_init<'a>(s: &BeginState, params: StepStateParams<'a>) -> StateTransition<'a> {
    if params.run_conditions.never {
        Ok((s.run_never(), params)) // s_no_need_to_run_f_run_never
    } else {
        Ok((s.run_conditional(), params)) // s_waiting_dependency_steps_f_run_conditional
    }
}

fn s_waiting_dependency_steps_f_run_conditional<'a>(
    s: &WaitingDependencyStepsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    let dependency_states = params.dependency_states;

    if dependency_states.read()?.len() == 0 {
        info!(
            params.output_snd,
            "No dependency steps for step {}", params.step.name
        );
        Ok((s.dependency_steps_finished_successfully(), params)) // s_waiting_to_run_f_dependency_steps_finished_successfully
    } else {
        info!(
            params.output_snd,
            "Waiting for dependency steps for step {}", params.step.name
        );
        Ok((s.dependency_steps_running(), params)) // s_waiting_dependency_steps_f_dependency_steps_running
    }
}

fn s_no_need_to_run_f_run_never<'a>(
    s: &NoNeedToRunState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    info!(
        params.output_snd,
        "Step {} has run_never set to true. Skipping.", params.step.name
    );
    Ok((s.completed_without_running_step(), params)) // s_done_f_completed_without_running_step
}

/// Terminal state: Waits till the end of times
fn s_done<'a>(s: &DoneState, params: StepStateParams<'a>) -> StateTransition<'a> {
    debug!(
        params.output_snd,
        "Step {} is done. You shouldn't see this more than once.", params.step.name
    );
    Ok((s.keep_done(), params))
}

fn s_done_f_completed_without_running_step<'a>(
    s: &DoneState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    info!(
        params.output_snd,
        "Step {} is completed without running.", params.step.name
    );
    s_done(s, params)
}

fn s_done_f_process_completed_successfully<'a>(
    s: &DoneState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    s_done(s, params)
}

fn s_done_f_has_done<'a>(s: &DoneState, params: StepStateParams<'a>) -> StateTransition<'a> {
    s_done(s, params)
}
