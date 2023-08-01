pub mod api;
pub mod command;
pub mod deps;
pub mod outs;
pub mod schema;
pub mod step;

use self::command::XvcStepCommand;
use self::deps::compare::superficial_compare_dependency;
use self::deps::XvcDependency;
use self::outs::XvcOutput;
use self::step::XvcStep;
use anyhow::anyhow;
use clap::Command;
use xvc_file::CHANNEL_CAPACITY;

use crate::deps::compare::{thorough_compare_dependency, DependencyComparisonParams};
use crate::deps::{dependencies_to_path, dependency_paths};
use crate::error::{Error, Result};
use crate::pipeline::command::CommandProcess;
use crate::{XvcPipeline, XvcPipelineRunDir};

use chrono::Utc;
use crossbeam_channel::{bounded, select, Receiver, Select, Sender};
use petgraph::Direction;
use xvc_logging::{debug, error, info, output, uwr, warn, watch, XvcOutputSender};
use xvc_walker::notify::{make_watcher, PathEvent};

use petgraph::algo::toposort;
use petgraph::data::Build;
use petgraph::dot::Dot;
use petgraph::prelude::DiGraphMap;

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::Sub;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread::{self, sleep, JoinHandle, ScopedJoinHandle};
use std::time::{Duration, Instant, SystemTime};
use strum_macros::{Display, EnumString};
use xvc_config::FromConfigKey;
use xvc_core::{
    all_paths_and_metadata, apply_diff, update_with_actual, AttributeDigest, ContentDigest, Diff,
    HashAlgorithm, PathCollectionDigest, TextOrBinary, XvcDigests, XvcFileType, XvcMetadata,
    XvcPath, XvcPathMetadataMap, XvcRoot,
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
    always: bool,
    wait_running_dep_steps: bool,
    ignore_broken_dep_steps: bool,
    ignore_missing_dependencies: bool,
    ignore_superficial_diffs: bool,
    ignore_thorough_diffs: bool,
    ignore_missing_outputs: bool,
}

#[derive(Debug, Clone)]
pub struct StepStateParams<'a> {
    xvc_root: &'a XvcRoot,
    output_snd: &'a XvcOutputSender,
    pmm: Arc<RwLock<XvcPathMetadataMap>>,
    run_conditions: &'a RunConditions,
    pipeline_rundir: &'a XvcPath,
    terminate_timeout_processes: bool,
    algorithm: HashAlgorithm,

    command_process: Arc<RwLock<CommandProcess>>,
    available_process_slots: Arc<RwLock<usize>>,
    process_poll_milliseconds: u64,

    dependency_diffs: Arc<RwLock<HStore<Diff<XvcDependency>>>>,
    output_diffs: Arc<RwLock<HStore<Diff<XvcOutput>>>>,

    step_e: XvcEntity,
    step: &'a XvcStep,
    step_command: &'a XvcStepCommand,
    current_states: Arc<RwLock<HStore<XvcStepState>>>,
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
    state_sender: Sender<Option<XvcStepState>>,
    state_notifier: Receiver<Option<(XvcEntity, XvcStepState)>>,
    current_states: Arc<RwLock<HStore<XvcStepState>>>,
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

    // TODO: We can convert these to HStore<Arc<RwLock<...>>>
    dependency_diffs: Arc<RwLock<HStore<Diff<XvcDependency>>>>,
    output_diffs: Arc<RwLock<HStore<Diff<XvcOutput>>>>,
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
        always: false,
        ignore_missing_outputs: false,
        ignore_missing_dependencies: false,
        wait_running_dep_steps: false,
        ignore_broken_dep_steps: false,
        ignore_superficial_diffs: false,
        ignore_thorough_diffs: false,
    };

    //  This is the DVC behavior. It doesn't run when _only_ dependency timestamp changed. For
    //  Makefile behavior `dependencies_new` can be set to `true`.
    let run_calculated = RunConditions {
        never: false,
        always: false,
        wait_running_dep_steps: true,
        ignore_broken_dep_steps: false,
        ignore_missing_outputs: true,
        ignore_missing_dependencies: false,
        ignore_superficial_diffs: false,
        ignore_thorough_diffs: false,
    };

    let run_always = RunConditions {
        never: false,
        always: true,
        ignore_missing_outputs: true,
        ignore_missing_dependencies: true,
        wait_running_dep_steps: true,
        ignore_broken_dep_steps: true,
        ignore_superficial_diffs: true,
        ignore_thorough_diffs: true,
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

    let step_states = Arc::new(RwLock::new(
        pipeline_steps
            .iter()
            .map(|(step_e, _)| (*step_e, step::XvcStepState::begin()))
            .collect(),
    ));

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

    let state_channels: HStore<(Sender<_>, Receiver<_>)> = sorted_steps
        .iter()
        .map(|step_e| (*step_e, bounded::<Option<XvcStepState>>(CHANNEL_CAPACITY)))
        .collect();

    let state_senders: Vec<_> = state_channels
        .iter()
        .map(|(step_e, (s, _))| (*step_e, s.clone()))
        .collect();

    let state_receivers: Vec<_> = state_channels
        .iter()
        .map(|(step_e, (_, r))| (*step_e, r.clone()))
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

    let dependency_diffs = Arc::new(RwLock::new(HStore::new()));
    let output_diffs = Arc::new(RwLock::new(HStore::new()));

    let (state_bulletin_sender, state_bulletin_receiver) =
        crossbeam_channel::bounded::<Option<(XvcEntity, XvcStepState)>>(CHANNEL_CAPACITY);
    let (kill_signal_sender, kill_signal_receiver) = crossbeam_channel::bounded::<bool>(1);
    // Create a thread for each of the steps
    // We create these in reverse topological order.
    // Dependent steps block on dependency events, so we need to create them first.
    let done_successfully: Result<bool> = thread::scope(|s| {
        let pmp_updater = s.spawn(|| {
            update_pmp(
                xvc_root,
                fs_receiver,
                &mut pmm,
                kill_signal_receiver.clone(),
            )
        });

        let state_updater = s.spawn(|| {
            step_state_bulletin(
                state_receivers.clone(),
                step_states.clone(),
                state_bulletin_sender.clone(),
                kill_signal_receiver.clone(),
            )
        });

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
                            state_sender: state_channels[step_e].0.clone(),
                            current_states: step_states.clone(),
                            state_notifier: state_bulletin_receiver.clone(),
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
                            dependency_diffs: dependency_diffs.clone(),
                            output_diffs: output_diffs.clone(),
                        };
                        step_state_handler(*step_e, step_thread_params)
                    }),
                )
            })
            .collect();

        watch!(&step_thread_store);

        // Join threads in the order we created
        step_thread_store.into_iter().for_each(|(step_e, jh)| {
            watch!((step_e, &jh));
            if let Err(e) = jh.join() {
                error!(output_snd, "Error in step thread: {:?}", e);
            }
        });

        kill_signal_sender.send(true)?;
        watch!("Before state updater");
        state_updater.join().unwrap().unwrap();
        pmp_updater.join().unwrap().unwrap();

        // if all of the steps are done, we can end
        if step_states.read()?.iter().all(|(_, step_s)| {
            matches!(
                step_s,
                XvcStepState::DoneByRunning(_) | XvcStepState::DoneWithoutRunning(_)
            )
        }) {
            watch!(step_states);
            Ok(true)
        } else {
            watch!(step_states);
            Ok(false)
        }
    });
    watch!(done_successfully);
    // We only save the stores if the pipeline was run successfully
    if let Ok(true) = done_successfully {
        xvc_root.with_store_mut(|store: &mut XvcStore<XvcDependency>| {
            dependency_diffs.read().as_deref().and_then(|diffs| {
                update_with_actual(store, diffs, true, true);
                Ok(())
            });

            Ok(())
        })?;

        xvc_root.with_store_mut(|store: &mut XvcStore<XvcOutput>| {
            output_diffs.read().as_deref().and_then(|output_diffs| {
                update_with_actual(store, output_diffs, true, true);
                watch!(output_diffs);
                watch!(store);
                Ok(())
            })?;
            Ok(())
        })?;
    }
    Ok(())
}

fn dependencies(
    step_e: XvcEntity,
    dependency_graph: &DependencyGraph,
) -> Result<HashSet<XvcEntity>> {
    let dep_neighbors = dependency_graph.neighbors(step_e);
    let mut dependencies = HashSet::new();
    for dep_neighbor in dep_neighbors {
        dependencies.insert(dep_neighbor);
    }
    Ok(dependencies)
}

fn step_state_bulletin(
    state_senders: Vec<(XvcEntity, Receiver<Option<XvcStepState>>)>,
    current_states: Arc<RwLock<HStore<XvcStepState>>>,
    notifier: Sender<Option<(XvcEntity, XvcStepState)>>,
    kill_signal_receiver: Receiver<bool>,
) -> Result<()> {
    let mut select = Select::new();
    for (_, r) in state_senders.iter() {
        select.recv(r);
    }
    loop {
        watch!(select);
        if let Ok(index) = select.ready_timeout(Duration::from_millis(1000)) {
            let res = state_senders[index].1.recv()?;
            if let Some(state) = res {
                let step_e = state_senders[index].0;
                current_states.write()?.insert(step_e, state.clone());
                notifier.send(Some((step_e, state)))?;
            }
        } else {
            if current_states.read()?.iter().all(|(_, s)| {
                matches!(
                    s,
                    XvcStepState::DoneByRunning(_)
                        | XvcStepState::DoneWithoutRunning(_)
                        | XvcStepState::Broken(_)
                )
            }) {
                return Ok(());
            }

            if kill_signal_receiver.try_recv().is_ok() {
                return Ok(());
            }
        }
    }
}

fn step_state_handler(step_e: XvcEntity, params: StepThreadParams) -> Result<()> {
    // We check all other steps states in Select.
    // If we only block on this step's dependencies, two parallel steps will block each other forever.
    let other_steps: Vec<XvcEntity> = params
        .steps
        .iter()
        .filter_map(|(e, _)| if *e != step_e { Some(*e) } else { None })
        .collect();

    let state_notifier = params.state_notifier.clone();
    let step_state_sender = params.state_sender;
    let current_states = params.current_states.clone();
    let mut step_state = XvcStepState::begin();
    let step_dependencies = params.recorded_dependencies.children_of(&step_e)?;
    let step_outputs = params.recorded_outputs.children_of(&step_e)?;
    let step_xvc_digests = params.recorded_xvc_digests.children_of(&step_e)?;
    let step = &params.steps[&step_e];
    let step_command = &params.step_commands[&step_e];
    let command_process = Arc::new(RwLock::new(CommandProcess::new(step, step_command)));
    let process_poll_milliseconds = 10;

    let mut step_params = StepStateParams {
        step_e,
        step,
        output_snd: &params.output_snd,
        algorithm: params.algorithm,
        step_command,
        command_process,
        // TODO: Convert this to AtomicUsize
        available_process_slots: Arc::new(RwLock::new(1)),
        terminate_timeout_processes: params.terminate_on_timeout,
        current_states,
        step_timeout: params.step_timeout,
        run_conditions: &params.run_conditions,
        xvc_root: params.xvc_root,
        pipeline_rundir: params.pipeline_rundir,
        pmm: params.current_pmm,
        step_dependencies: &step_dependencies,
        step_outputs: &step_outputs,
        step_xvc_digests: &step_xvc_digests,
        dependency_diffs: params.dependency_diffs,
        output_diffs: params.output_diffs,
        process_poll_milliseconds,
    };

    loop {
        // Send the state first
        step_state_sender.send(Some(step_state.clone()))?;
        watch!(&step_state);
        if matches!(
            step_state,
            XvcStepState::DoneByRunning(_)
                | XvcStepState::DoneWithoutRunning(_)
                | XvcStepState::Broken(_)
        ) {
            // We're done. We can return.
            return Ok(());
        }

        let (r_next_state, next_params) = match &step_state {
            XvcStepState::Begin(s) => match s {
                BeginState::FromInit => s_begin_f_init(s, step_params)?,
            },

            XvcStepState::DoneWithoutRunning(s) => match s {
                DoneWithoutRunningState::FromRunNever => {
                    s_no_need_to_run_f_run_never(s, step_params)?
                }
                DoneWithoutRunningState::FromDiffsHasNotChanged => {
                    s_no_need_to_run_f_diffs_not_changed(s, step_params)?
                }
                DoneWithoutRunningState::FromKeepDone => {
                    (XvcStepState::DoneWithoutRunning(s.clone()), step_params)
                }
            },
            XvcStepState::WaitingDependencySteps(s) => match s {
                WaitingDependencyStepsState::FromDependencyStepsRunning => {
                    s_waiting_dependency_steps_f_dependency_steps_running(s, step_params)?
                }
                WaitingDependencyStepsState::FromRunConditional => {
                    s_waiting_dependency_steps_f_run_conditional(s, step_params)?
                }
            },

            XvcStepState::CheckingOutputs(s) => match s {
                CheckingOutputsState::FromDependencyStepsFinishedBrokenIgnored => {
                    s_checking_outputs_f_dependency_steps_finished_ignored(s, step_params)?
                }
                CheckingOutputsState::FromDependencyStepsFinishedSuccessfully => {
                    s_checking_outputs_f_dependency_steps_finished_successfully(s, step_params)?
                }
            },
            XvcStepState::CheckingSuperficialDiffs(s) => match s {
                CheckingSuperficialDiffsState::FromOutputsIgnored => {
                    s_checking_superficial_diffs_f_missing_outputs_ignored(s, step_params)?
                }
                CheckingSuperficialDiffsState::FromCheckedOutputs => {
                    s_checking_superficial_diffs(s, step_params)?
                }
            },
            XvcStepState::CheckingThoroughDiffs(s) => match s {
                CheckingThoroughDiffsState::FromSuperficialDiffsChanged => {
                    s_checking_thorough_diffs_f_superficial_diffs_changed(s, step_params)?
                }
                CheckingThoroughDiffsState::FromSuperficialDiffsIgnored => {
                    s_checking_thorough_diffs_f_superficial_diffs_ignored(s, step_params)?
                }
            },
            XvcStepState::ComparingDiffsAndOutputs(s) => match s {
                ComparingDiffsAndOutputsState::FromSuperficialDiffsNotChanged => {
                    s_comparing_diffs_and_outputs_f_superficial_diffs_not_changed(s, step_params)?
                }
                ComparingDiffsAndOutputsState::FromThoroughDiffsNotChanged => {
                    s_comparing_diffs_and_outputs_f_thorough_diffs_not_changed(s, step_params)?
                }
                ComparingDiffsAndOutputsState::FromThoroughDiffsChanged => {
                    s_comparing_diffs_and_outputs_f_thorough_diffs_changed(s, step_params)?
                }
            },
            XvcStepState::WaitingToRun(s) => match s {
                WaitingToRunState::FromDiffsHasChanged => {
                    s_waiting_to_run_f_diffs_has_changed(s, step_params)?
                }
                WaitingToRunState::FromProcessPoolFull => {
                    s_waiting_to_run_f_process_pool_full(s, step_params)?
                }
                WaitingToRunState::FromRunAlways => s_waiting_to_run_f_run_always(s, step_params)?,
            },
            XvcStepState::Running(s) => match s {
                RunningState::FromStartProcess => s_running_f_start_process(s, step_params)?,
                RunningState::FromWaitProcess => s_running_f_wait_process(s, step_params)?,
            },
            XvcStepState::Broken(s) => match s {
                BrokenState::FromCannotStartProcess => {
                    s_broken_f_cannot_start_process(s, step_params)?
                }
                BrokenState::FromHasMissingDependencies => {
                    s_broken_f_has_missing_dependencies(s, step_params)?
                }
                BrokenState::FromDependencyStepsFinishedBroken => {
                    s_broken_f_dependency_steps_finished_broken(s, step_params)?
                }
                BrokenState::FromProcessTimeout => s_broken_f_process_timeout(s, step_params)?,
                BrokenState::FromProcessReturnedNonZero => {
                    s_broken_f_process_returned_non_zero(s, step_params)?
                }
                BrokenState::FromKeepBroken => (XvcStepState::Broken(s.clone()), step_params),
            },
            XvcStepState::DoneByRunning(s) => match s {
                DoneByRunningState::FromProcessCompletedSuccessfully => {
                    s_done_f_process_completed_successfully(s, step_params)?
                }
                DoneByRunningState::FromKeepDone => {
                    (XvcStepState::DoneByRunning(s.clone()), step_params)
                }
            },
            XvcStepState::CheckingSuperficialDiffs(s) => match s {
                CheckingSuperficialDiffsState::FromCheckedOutputs => {
                    s_checking_superficial_diffs(s, step_params)?
                }
                CheckingSuperficialDiffsState::FromOutputsIgnored => {
                    s_checking_superficial_diffs_f_missing_outputs_ignored(s, step_params)?
                }
            },
        };

        watch!(step.name);
        watch!(&r_next_state);
        step_state = r_next_state;
        watch!(&step_state);
        step_params = next_params;
    }
}

fn update_pmp(
    xvc_root: &XvcRoot,
    fs_receiver: Receiver<Option<PathEvent>>,
    pmm: &mut XvcPathMetadataMap,
    kill_signal_receiver: Receiver<bool>,
) -> Result<()> {
    let mut handle_fs_event = |fs_event| match fs_event {
        PathEvent::Create { path, metadata } => {
            let xvc_path = XvcPath::new(xvc_root, xvc_root, &path).unwrap();
            let xvc_md = XvcMetadata::from(metadata);
            pmm.insert(xvc_path, xvc_md);
        }
        PathEvent::Update { path, metadata } => {
            let xvc_path = XvcPath::new(xvc_root, xvc_root, &path).unwrap();
            let xvc_md = XvcMetadata::from(metadata);
            pmm.insert(xvc_path, xvc_md);
        }
        PathEvent::Delete { path } => {
            let xvc_path = XvcPath::new(xvc_root, xvc_root, &path).unwrap();
            let xvc_md = XvcMetadata {
                file_type: XvcFileType::Missing,
                size: None,
                modified: None,
            };
            pmm.insert(xvc_path, xvc_md);
        }
    };

    loop {
        select! {
            recv(fs_receiver) -> fs_event => match fs_event {
                Ok(Some(fs_event)) => {
                    handle_fs_event(fs_event);
                }
                Ok(None) => {
                    return Ok(())
                }
                Err(e) => {
                    error!("Error in fs_receiver: {:?}", e);
                    return Err(anyhow!("Error in fs_receiver: {:?}", e).into())
                }
            },

            recv(kill_signal_receiver) -> kill_signal => {
                if let Ok(true) = kill_signal {
                    return Ok(())
                }
            },
        }
    }
}

fn s_begin_f_init<'a>(s: &BeginState, params: StepStateParams<'a>) -> StateTransition<'a> {
    if params.run_conditions.never {
        Ok((s.run_never(), params)) // s_no_need_to_run_f_run_never
    } else {
        Ok((s.run_conditional(), params)) // s_waiting_dependency_steps_f_run_conditional
    }
}

fn s_no_need_to_run_f_run_never<'a>(
    s: &DoneWithoutRunningState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    info!(
        params.output_snd,
        "Step {} has run_never set to true. Skipping.", params.step.name
    );
    Ok((s.keep_done(), params))
}

fn s_no_need_to_run_f_diffs_not_changed<'a>(
    s: &DoneWithoutRunningState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    info!(
        params.output_snd,
        "Dependencies for step {} hasn't changed. Skipping.", params.step.name
    );
    Ok((s.keep_done(), params))
}

fn s_waiting_dependency_steps_f_dependency_steps_running<'a>(
    s: &WaitingDependencyStepsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    loop {
        let dependencies = params.step_dependencies;
        // We'll update these states internally and check for actions
        let current_states = params.current_states.clone();
        let dep_states = current_states
            .read()?
            .iter()
            .filter_map(|(step_e, state)| {
                if dependencies.contains_key(step_e) {
                    Some((*step_e, state.clone()))
                } else {
                    None
                }
            })
            .collect::<HStore<_>>();

        watch!(dep_states);

        // if all dependencies are completed somehow (Done or Broken) move to checking run conditions
        if dep_states.iter().all(|(_, dep_state)| {
            matches!(
                dep_state,
                &XvcStepState::DoneByRunning(_) | &XvcStepState::DoneWithoutRunning(_)
            )
        }) {
            info!(
                params.output_snd,
                "Dependency steps completed successfully for step {}", params.step.name
            );
            return Ok((s.dependency_steps_finished_successfully(), params));
        } else if dep_states
            .iter()
            .all(|(_, dep_state)| matches!(dep_state, &XvcStepState::Broken(_)))
        {
            if params.run_conditions.ignore_broken_dep_steps {
                info!(
                    params.output_snd,
                    "Dependency steps completed for step {} (ignoring broken steps)",
                    params.step.name
                );
                return Ok((s.dependency_steps_finished_broken_ignored(), params));
            } else {
                info!(
                    params.output_snd,
                    "Dependency steps are broken for step {}", params.step.name
                );
                return Ok((s.dependency_steps_finished_broken(), params));
            }
        } else {
            debug!(
                params.output_snd,
                "Dependency steps are running for step {}", params.step.name
            );
            watch!(params.step.name);
            sleep(Duration::from_millis(params.process_poll_milliseconds));
        }
    }
}

fn s_waiting_dependency_steps_f_run_conditional<'a>(
    s: &WaitingDependencyStepsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    let dependencies = params.step_dependencies;
    if dependencies.len() == 0 {
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

fn s_comparing_diffs_and_outputs_f_thorough_diffs_changed<'a>(
    s: &ComparingDiffsAndOutputsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    info!(
        params.output_snd,
        "[{}] Dependencies has changed", params.step.name
    );
    // TODO: Update MISSING_OUTPUTS environment variable if there is
    Ok((s.diffs_has_changed(), params))
}

fn s_comparing_diffs_and_outputs_f_thorough_diffs_not_changed<'a>(
    s: &ComparingDiffsAndOutputsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    if params.run_conditions.always {
        return Ok((s.run_always(), params));
    }

    // Run if we have missing outputs, or dependencies have changed, or run conditions require to run always.
    let mut changed = false;
    {
        let output_diffs = params.output_diffs.read()?;
        if output_diffs
            .iter()
            .any(|(_, diff)| matches!(diff, Diff::ActualMissing { .. }))
        {
            // TODO: Update MISSING_OUTPUTS environment variable
            info!(params.output_snd, "[{}] Missing Outputs", params.step.name);
            changed = true;
        } else {
            info!(
                params.output_snd,
                "[{}] No missing Outputs and no changed dependencies", params.step.name
            );
            changed = false;
        }
    }

    if changed {
        Ok((s.diffs_has_changed(), params))
    } else {
        Ok((s.diffs_has_not_changed(), params))
    }
}

fn s_comparing_diffs_and_outputs_f_superficial_diffs_not_changed<'a>(
    s: &ComparingDiffsAndOutputsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    if params.run_conditions.always {
        return Ok((s.run_always(), params));
    }

    info!(
        params.output_snd,
        "[{}] No changed dependencies. Skipping thorough comparison.", params.step.name
    );

    let mut changed = false;

    // Check if the step dependencies have run
    {
        let done_by_running_dependencies = params
            .step_dependencies
            .iter()
            .filter_map(
                |(xe, xd)| match params.current_states.read().ok()?.get(xe) {
                    Some(XvcStepState::DoneByRunning(_)) => Some((*xe, xd.clone())),
                    _ => None,
                },
            )
            .collect::<HStore<_>>();

        if done_by_running_dependencies.len() > 0 {
            changed = true;
        }
    }

    {
        let output_diffs = params.output_diffs.read()?;
        if output_diffs
            .iter()
            .any(|(_, diff)| matches!(diff, Diff::ActualMissing { .. }))
        {
            // TODO: Update MISSING_OUTPUTS environment variable
            info!(params.output_snd, "[{}] Missing Outputs", params.step.name);
            changed = true;
        } else {
            info!(
                params.output_snd,
                "[{}] No missing Outputs and no changed dependencies", params.step.name
            );
            changed = false;
        }
    }

    if changed {
        Ok((s.diffs_has_changed(), params))
    } else {
        Ok((s.diffs_has_not_changed(), params))
    }
}

fn s_checking_superficial_diffs<'a>(
    s: &CheckingSuperficialDiffsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    watch!(params.step_dependencies.len());
    // if no dependencies, we assume the step needs to run always.
    if params.step_dependencies.is_empty() {
        watch!(params.step.name);
        return Ok((s.superficial_diffs_changed(), params));
    }

    let step_dependency_diffs: HStore<Diff<XvcDependency>> = params
        .step_dependencies
        .iter()
        .map(|(dep_e, dep)| {
            let cmp_diff = uwr!(
                superficial_compare_dependency(&params, *dep_e),
                params.output_snd
            );
            (*dep_e, cmp_diff)
        })
        .collect();
    let mut changed = false;

    {
        let mut dependency_diffs = params.dependency_diffs.write()?;
        for (dep_e, diff) in step_dependency_diffs.into_iter() {
            changed = changed | &diff.changed();
            dependency_diffs.insert(dep_e, diff);
        }
    }
    watch!(changed);
    if changed {
        Ok((s.superficial_diffs_changed(), params))
    } else {
        Ok((s.superficial_diffs_not_changed(), params))
    }
}

fn s_checking_superficial_diffs_f_missing_outputs_ignored<'a>(
    s: &CheckingSuperficialDiffsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    info!(
        params.output_snd,
        "[{}] Ignored Missing Outputs", params.step.name
    );
    s_checking_superficial_diffs(s, params)
}

fn s_checking_thorough_diffs_f_superficial_diffs_changed<'a>(
    s: &CheckingThoroughDiffsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    let deps = params.step_dependencies;
    watch!(deps.is_empty());
    // Normally this should be checked in the previous state, but we check it here just in case
    if deps.is_empty() {
        return Ok((s.thorough_diffs_changed(), params));
    }

    // Calculate diffs for changed dependencies
    let step_dependency_diffs: HStore<Diff<XvcDependency>> = params
        .dependency_diffs
        .read()?
        .iter()
        .map(|(dep_e, dep)| {
            if dep.changed() {
                let cmp_diff = uwr!(
                    thorough_compare_dependency(&params, *dep_e),
                    params.output_snd
                );
                (*dep_e, cmp_diff)
            } else {
                (*dep_e, Diff::Skipped)
            }
        })
        .collect();
    let mut changed = false;

    {
        let mut dependency_diffs = params.dependency_diffs.write()?;
        for (dep_e, diff) in step_dependency_diffs.into_iter() {
            changed = changed | &diff.changed();
            dependency_diffs.insert(dep_e, diff);
        }
    }

    if changed {
        Ok((s.thorough_diffs_changed(), params))
    } else {
        Ok((s.thorough_diffs_not_changed(), params))
    }
}

fn s_checking_thorough_diffs_f_superficial_diffs_ignored<'a>(
    s: &CheckingThoroughDiffsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    let step_e = params.step_e;
    let deps = params.step_dependencies;
    watch!(deps.is_empty());
    // Normally this should be checked in the previous state, but we check it here just in case
    if deps.is_empty() {
        return Ok((s.thorough_diffs_changed(), params));
    }

    // Calculate diffs for all dependencies
    let step_dependency_diffs: HStore<Diff<XvcDependency>> = params
        .step_dependencies
        .iter()
        .map(|(dep_e, dep)| {
            let cmp_diff = uwr!(
                thorough_compare_dependency(&params, *dep_e),
                params.output_snd
            );
            (*dep_e, cmp_diff)
        })
        .collect();

    let mut changed = false;

    {
        let mut dependency_diffs = params.dependency_diffs.write()?;
        for (dep_e, diff) in step_dependency_diffs.into_iter() {
            changed = changed | &diff.changed();
            dependency_diffs.insert(dep_e, diff);
        }
    }

    if changed {
        Ok((s.thorough_diffs_changed(), params))
    } else {
        Ok((s.thorough_diffs_not_changed(), params))
    }
}

fn s_checking_outputs_f_dependency_steps_finished_successfully<'a>(
    s: &CheckingOutputsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    s_checking_missing_outputs(s, params)
}

fn s_checking_outputs_f_dependency_steps_finished_ignored<'a>(
    s: &CheckingOutputsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    s_checking_missing_outputs(s, params)
}

fn compare_output(params: &StepStateParams, out_e: XvcEntity) -> Result<Diff<XvcOutput>> {
    let output = params.step_outputs.get(&out_e).unwrap();

    match output {
        record @ XvcOutput::File { path } => {
            let path = path.to_absolute_path(params.xvc_root);
            if path.exists() {
                Ok(Diff::Identical)
            } else {
                Ok(Diff::ActualMissing {
                    record: record.clone(),
                })
            }
        }
        record @ XvcOutput::Metric { path, .. } => {
            let path = path.to_absolute_path(params.xvc_root);
            if path.exists() {
                Ok(Diff::Identical)
            } else {
                Ok(Diff::ActualMissing {
                    record: record.clone(),
                })
            }
        }
        record @ XvcOutput::Image { path } => {
            let path = path.to_absolute_path(params.xvc_root);
            if path.exists() {
                Ok(Diff::Identical)
            } else {
                Ok(Diff::ActualMissing {
                    record: record.clone(),
                })
            }
        }
    }
}

fn s_checking_missing_outputs<'a>(
    s: &CheckingOutputsState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    let run_conditions = params.run_conditions;
    let step_outs = params.step_outputs;
    let pmm = params.pmm.clone();

    if run_conditions.ignore_missing_outputs {
        return Ok((s.checked_outputs(), params));
    }

    let mut missing = false;

    params.output_diffs.write()?.extend(
        step_outs
            .iter()
            .map(|(out_e, out)| {
                let out_diff = uwr!(compare_output(&params, *out_e), params.output_snd);
                if matches!(out_diff, Diff::ActualMissing { .. }) {
                    missing = true;
                }
                (*out_e, out_diff)
            })
            .collect::<HStore<Diff<XvcOutput>>>(),
    );

    Ok((s.checked_outputs(), params))
}

fn s_running_f_start_process<'a>(
    s: &RunningState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    let command_process = params.command_process.clone();
    let mut command_process = command_process.write()?;
    command_process.run()?;
    let available_slots = params.available_process_slots.clone();
    let mut available_slots = available_slots.write()?;
    *available_slots -= 1;
    Ok((s.wait_process(), params))
}

fn s_running_f_wait_process<'a>(
    s: &RunningState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    watch!(params);
    let mut return_state: Option<XvcStepState> = None;
    let command_process = params.command_process.clone();
    let timeout = params.step_timeout;
    let step = params.step.clone();
    let step_command = params.step_command.clone();
    let birth = command_process
        .read()?
        .birth
        .ok_or(anyhow!("Process birth not found"))?;
    let sleep_duration = Duration::from_millis(params.process_poll_milliseconds);
    loop {
        // We put process operations in an inner scope not to interfere with the process while sleeping
        {
            // Check whether the process is still running
            let mut command_process = command_process.write()?;

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

            watch!(command_process);

            let process = command_process
                .process
                .as_mut()
                .ok_or_else(|| anyhow!("Cannot find process"))?;
            watch!(&process);
            let poll_result = process.poll();
            match poll_result {
                // Still running:
                None => {
                    watch!(process);
                    if birth.elapsed() < *timeout {
                        debug!(
                            output_snd,
                            "Step {} with command {} is still running", &step.name, &step_command
                        );
                    } else {
                        if params.terminate_timeout_processes {
                            error!(
                                output_snd,
                                "Process timeout for step {} with command {} ",
                                &step.name,
                                &step_command
                            );
                            process.terminate().ok();
                        }
                        return_state = Some(s.process_timeout());
                        break;
                    }
                }

                Some(exit_code) => match exit_code {
            ExitStatus::Exited(0) => {
                info!(output_snd, "Step {} finished successfully with command {}", step.name, step_command);
                return_state = Some(s.process_completed_successfully());
                break;
            }
            ,
            // we don't handle other variants in the state machine. Either it exited
            // successfully or died for some reason.
            //
            _ => {
                error!(output_snd, "Step {} finished UNSUCCESSFULLY with command {}", step.name, step_command);
                return_state = Some(s.process_returned_non_zero());
                break;
            },
        },
            }
        }
        sleep(sleep_duration);
    }

    watch!(return_state);

    let available_slots = params.available_process_slots.clone();
    let mut available_slots = available_slots.write().unwrap();
    *available_slots += 1;

    watch!(params);

    Ok((return_state.unwrap(), params))
}

fn s_waiting_to_run_f_process_pool_full<'a>(
    s: &WaitingToRunState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    watch!(params);
    if params.available_process_slots.read()?.gt(&0) {
        Ok((s.start_process(), params))
    } else {
        Ok((s.process_pool_full(), params))
    }
}
fn s_waiting_to_run_f_diffs_has_changed<'a>(
    s: &WaitingToRunState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    watch!(params);
    if params.available_process_slots.read()?.gt(&0) {
        Ok((s.start_process(), params))
    } else {
        Ok((s.process_pool_full(), params))
    }
}

fn s_waiting_to_run_f_run_always<'a>(
    s: &WaitingToRunState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    watch!(params);
    if params.available_process_slots.read()?.gt(&0) {
        Ok((s.start_process(), params))
    } else {
        Ok((s.process_pool_full(), params))
    }
}

/// Broken stays always Broken
fn s_broken_f_process_returned_non_zero<'a>(
    s: &BrokenState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    Ok((s.keep_broken(), params))
}

fn s_broken_f_process_timeout<'a>(
    s: &BrokenState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
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

/// Terminal state: Waits till the end of times
fn s_done<'a>(s: &DoneByRunningState, params: StepStateParams<'a>) -> StateTransition<'a> {
    debug!(
        params.output_snd,
        "Step {} is done. You shouldn't see this more than once.", params.step.name
    );
    Ok((s.keep_done(), params))
}

fn s_done_f_process_completed_successfully<'a>(
    s: &DoneByRunningState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    s_done(s, params)
}
