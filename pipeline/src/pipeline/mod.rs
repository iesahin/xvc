//! Xvc Pipelines implementation
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

use itertools::Itertools;
use xvc_core::XvcPathMetadataProvider;
use xvc_file::CHANNEL_CAPACITY;

use crate::deps::compare::thorough_compare_dependency;
use crate::deps::dependencies_to_path;
use crate::error::{Error, Result};
use crate::pipeline::command::CommandProcess;
use crate::{XvcPipeline, XvcPipelineRunDir};

use crossbeam_channel::{bounded, Receiver, Select, Sender};

use xvc_logging::{debug, error, info, output, uwr, warn, watch, XvcOutputSender};

use petgraph::algo::toposort;
use petgraph::data::Build;
use petgraph::dot::Dot;
use petgraph::prelude::DiGraphMap;

use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::fmt::Debug;

use std::sync::{Arc, RwLock};
use std::thread::{self, sleep, ScopedJoinHandle};
use std::time::Duration;
use strum_macros::{Display, EnumString};
use xvc_config::FromConfigKey;
use xvc_core::{update_with_actual, Diff, HashAlgorithm, XvcPath, XvcRoot};

use xvc_ecs::{persist, HStore, R1NStore, XvcEntity, XvcStore};

use sp::ExitStatus;
use subprocess as sp;

/// The option whether to consider a step changed
#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    Serialize,
    Deserialize,
    Default,
)]
#[strum(serialize_all = "snake_case")]
pub enum XvcStepInvalidate {
    /// Change when dependencies change or outputs missing
    #[default]
    ByDependencies,
    /// Always consider changed
    Always,
    /// Never consider changed,
    Never,
}

persist!(XvcStepInvalidate, "xvc-step-invalidate");

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
#[allow(clippy::too_many_arguments)]
pub fn add_implicit_dependencies(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    pmp: &XvcPathMetadataProvider,
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
                dependencies_to_path(xvc_root, pmp, pipeline_rundir, &all_deps.children, to_path)
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
    ignore_broken_dep_steps: bool,
    ignore_missing_outputs: bool,
}

/// Parameters supplied to a step state
#[derive(Debug, Clone)]
pub struct StepStateParams<'a> {
    xvc_root: &'a XvcRoot,
    output_snd: &'a XvcOutputSender,
    pmp: &'a XvcPathMetadataProvider,
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

    all_steps: &'a HStore<XvcStep>,
    recorded_dependencies: &'a R1NStore<XvcStep, XvcDependency>,
    step_dependencies: &'a HashSet<XvcEntity>,
    step_outputs: &'a HStore<XvcOutput>,
}

/// This structure is passed to step_threads as a parameter.
/// It contains all the information needed to run a step.
#[derive(Debug, Clone)]
struct StepThreadParams<'a> {
    xvc_root: &'a XvcRoot,
    output_snd: &'a XvcOutputSender,
    pipeline_rundir: &'a XvcPath,
    state_sender: Sender<Option<XvcStepState>>,
    current_states: Arc<RwLock<HStore<XvcStepState>>>,
    step_commands: &'a XvcStore<XvcStepCommand>,
    steps: &'a HStore<XvcStep>,
    dependency_graph: &'a DependencyGraph,
    step_timeout: &'a Duration,
    run_conditions: &'a RunConditions,
    terminate_on_timeout: bool,
    algorithm: HashAlgorithm,
    pmp: &'a XvcPathMetadataProvider,
    process_pool_size: usize,
    recorded_dependencies: &'a R1NStore<XvcStep, XvcDependency>,
    recorded_outputs: &'a R1NStore<XvcStep, XvcOutput>,

    // TODO: We can convert these to HStore<Arc<RwLock<...>>>
    dependency_diffs: Arc<RwLock<HStore<Diff<XvcDependency>>>>,
    output_diffs: Arc<RwLock<HStore<Diff<XvcOutput>>>>,
}

/// # Xvc Pipeline Dependency Graph Rules
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
    watch!(pipeline_e);

    let pipeline_steps = xvc_root
        .load_r1nstore::<XvcPipeline, XvcStep>()?
        .children_of(&pipeline_e)?;
    watch!(pipeline_steps);

    let consider_changed = xvc_root.load_store::<XvcStepInvalidate>()?;
    watch!(consider_changed);

    let all_deps = xvc_root.load_r1nstore::<XvcStep, XvcDependency>()?;
    watch!(all_deps.parents.len());
    watch!(all_deps.children.len());
    let all_outs = xvc_root.load_r1nstore::<XvcStep, XvcOutput>()?;
    watch!(all_outs.parents.len());
    watch!(all_outs.children.len());
    let pmp = XvcPathMetadataProvider::new(output_snd, xvc_root)?;
    watch!(&pmp);

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
        &pmp,
        &pipeline_rundir,
        &all_deps,
        &all_outs,
        &pipeline_steps,
        &mut dependency_graph,
    )?;

    watch!(&dependency_graph);

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
        ignore_broken_dep_steps: false,
    };

    //  This is the DVC behavior. It doesn't run when _only_ dependency timestamp changed. For
    //  Makefile behavior `dependencies_new` can be set to `true`.
    let run_calculated = RunConditions {
        never: false,
        always: false,
        ignore_broken_dep_steps: false,
        ignore_missing_outputs: true,
    };

    let run_always = RunConditions {
        never: false,
        always: true,
        ignore_missing_outputs: true,
        ignore_broken_dep_steps: true,
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

    let process_pool_size: usize = xvc_root
        .config()
        .get_int("pipeline.process_pool_size")?
        .option
        .try_into()?;
    let default_step_timeout: u64 = 10000;
    let terminate_on_timeout = true;
    let _step_timeouts: HStore<Duration> = pipeline_steps
        .keys()
        .map(|step_e| (*step_e, Duration::from_secs(default_step_timeout)))
        .collect();

    let step_commands = xvc_root.load_store::<XvcStepCommand>()?;

    let algorithm = HashAlgorithm::from_conf(config);

    let state_channels: HStore<(Sender<_>, Receiver<_>)> = sorted_steps
        .iter()
        .map(|step_e| (*step_e, bounded::<Option<XvcStepState>>(CHANNEL_CAPACITY)))
        .collect();

    let _state_senders: Vec<_> = state_channels
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

    let dependency_diffs = Arc::new(RwLock::new(HStore::new()));
    let output_diffs = Arc::new(RwLock::new(HStore::new()));

    // FIXME: Why don't we use state_bulletin_receiver here?
    #[allow(unused_variables)]
    let (state_bulletin_sender, state_bulletin_receiver) =
        crossbeam_channel::bounded::<Option<(XvcEntity, XvcStepState)>>(CHANNEL_CAPACITY);
    let (kill_signal_sender, kill_signal_receiver) = crossbeam_channel::bounded::<bool>(1);
    // Create a thread for each of the steps
    // We create these in reverse topological order.
    // Dependent steps block on dependency events, so we need to create them first.
    let done_successfully: Result<bool> = thread::scope(|s| {
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
                            state_sender: state_channels[step_e].0.clone(),
                            current_states: step_states.clone(),
                            dependency_graph: &dependency_graph,
                            step_timeout: &step_timeout,
                            run_conditions: &run_conditions[step_e],
                            terminate_on_timeout,
                            pmp: &pmp,
                            output_snd: &output_snd,
                            step_commands: &step_commands,
                            steps: &pipeline_steps,
                            process_pool_size,
                            algorithm,
                            recorded_dependencies: &recorded_dependencies,
                            recorded_outputs: &recorded_outputs,
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
            dependency_diffs
                .read()
                .as_deref()
                .map(|diffs| update_with_actual(store, diffs, true, true))?
        })?;

        xvc_root.with_store_mut(|store: &mut XvcStore<XvcOutput>| {
            output_diffs
                .read()
                .as_deref()
                .map(|output_diffs| update_with_actual(store, output_diffs, true, true))?
        })?;
    }
    Ok(())
}

/// Return steps that `step_e` depends on the dependency_graph.
/// Note that, _non step_ dependencies are retrieved by R1NStore::<XvcStep, XvcDependency>::children
/// as a standard 1-N relationship.
fn dependency_steps(
    step_e: XvcEntity,
    dependency_graph: &DependencyGraph,
) -> Result<HashSet<XvcEntity>> {
    let dep_neighbors = dependency_graph.neighbors(step_e);
    watch!(dep_neighbors);
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
        if let Ok(index) = select.ready_timeout(Duration::from_millis(10)) {
            let res = state_senders[index].1.recv()?;
            if let Some(state) = res {
                let step_e = state_senders[index].0;
                current_states.write()?.insert(step_e, state.clone());
                notifier.send(Some((step_e, state)))?;
            }
        } else {
            if current_states.read()?.iter().all(|(_, s)| {
                watch!(s);
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
    let _other_steps: Vec<XvcEntity> = params
        .steps
        .iter()
        .filter_map(|(e, _)| if *e != step_e { Some(*e) } else { None })
        .collect();

    let step_state_sender = params.state_sender;
    let current_states = params.current_states.clone();
    let mut step_state = XvcStepState::begin();
    watch!(params.recorded_dependencies);
    watch!(step_e);
    watch!(dependency_steps(step_e, params.dependency_graph)?);
    let step_dependencies = dependency_steps(step_e, params.dependency_graph)?;
    let step_outputs = params.recorded_outputs.children_of(&step_e)?;
    let step = &params.steps[&step_e];
    let step_command = &params.step_commands[&step_e];
    let command_process = Arc::new(RwLock::new(CommandProcess::new(step, step_command)));
    let process_poll_milliseconds = 10;

    let mut step_params = StepStateParams {
        step_e,
        step,
        output_snd: params.output_snd,
        algorithm: params.algorithm,
        step_command,
        command_process,
        // TODO: Convert this to AtomicUsize
        available_process_slots: Arc::new(RwLock::new(params.process_pool_size)),
        terminate_timeout_processes: params.terminate_on_timeout,
        current_states,
        step_timeout: params.step_timeout,
        run_conditions: params.run_conditions,
        xvc_root: params.xvc_root,
        pipeline_rundir: params.pipeline_rundir,
        pmp: params.pmp,

        all_steps: params.steps,
        recorded_dependencies: params.recorded_dependencies,
        step_dependencies: &step_dependencies,
        step_outputs: &step_outputs,
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
        };

        watch!(step.name);
        watch!(&r_next_state);
        step_state = r_next_state;
        watch!(&step_state);
        step_params = next_params;
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
                if dependencies.contains(step_e) {
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
    if dependencies.is_empty() {
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
    let changed: bool;

    // Create a new scope to read params.output_diffs
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
        watch!(params.step_dependencies);
        // Check if there are any step dependencies that we depend and done by running
        changed = params.step_dependencies.iter().any(|xe| {
            if let Ok(hstore) = params.current_states.read() {
                if let Some(s) = hstore.get(xe) {
                    if matches!(s, XvcStepState::DoneByRunning(_)) {
                        true
                    } else {
                        changed
                    }
                } else {
                    changed
                }
            } else {
                changed
            }
        });
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
            // We don't update the state
            // changed = changed;
        }
    }

    watch!(changed);
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
    let parent_entity = params.step_e;
    watch!(parent_entity);
    let deps = params.recorded_dependencies.children_of(&parent_entity)?;

    watch!(deps);
    // if no dependencies, we assume the step needs to run always.
    if deps.is_empty() {
        watch!(params.step.name);
        return Ok((s.superficial_diffs_changed(), params));
    }

    let step_dependency_diffs: HStore<Diff<XvcDependency>> = deps
        .iter()
        .map(|(dep_e, _dep)| {
            let cmp_diff = uwr!(
                superficial_compare_dependency(&params, *dep_e),
                params.output_snd
            );
            (*dep_e, cmp_diff)
        })
        .collect();
    watch!(step_dependency_diffs);
    let mut changed = false;

    {
        let mut dependency_diffs = params.dependency_diffs.write()?;
        for (dep_e, diff) in step_dependency_diffs.into_iter() {
            watch!(diff);
            watch!(diff.changed());
            changed |= &diff.changed();
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
    let parent_entity = params.step_e;
    let deps = params.recorded_dependencies.children_of(&parent_entity)?;
    watch!(deps);
    // Normally this should be checked in the previous state, but we check it here just in case
    if deps.is_empty() {
        return Ok((s.thorough_diffs_changed(), params));
    }

    // Calculate diffs for superficially changed dependencies.
    // This allows to skip thorough comparison for dependencies that haven't changed.
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
            changed |= &diff.changed();
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
    let parent_entity = params.step_e;
    let deps = params.recorded_dependencies.children_of(&parent_entity)?;
    watch!(deps);
    // Normally this should be checked in the previous state, but we check it here just in case
    if deps.is_empty() {
        return Ok((s.thorough_diffs_changed(), params));
    }

    // Calculate diffs for all dependencies as we skipped superficial comparison
    let step_dependency_diffs: HStore<Diff<XvcDependency>> = deps
        .iter()
        .map(|(dep_e, _dep)| {
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
            changed |= &diff.changed();
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

    if run_conditions.ignore_missing_outputs {
        return Ok((s.checked_outputs(), params));
    }

    let mut missing = false;

    params.output_diffs.write()?.extend(
        step_outs
            .iter()
            .map(|(out_e, _out)| {
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
    update_command_environment(params.command_process.clone(), &params)?;
    let command_process = params.command_process.clone();
    let mut command_process = command_process.write()?;
    command_process.run()?;
    let available_slots = params.available_process_slots.clone();
    let mut available_slots = available_slots.write()?;
    *available_slots -= 1;
    Ok((s.wait_process(), params))
}

fn update_command_environment(
    command_process: Arc<RwLock<CommandProcess>>,
    params: &StepStateParams<'_>,
) -> Result<()> {
    let parent_entity = params.step_e;
    let deps = params.recorded_dependencies.children_of(&parent_entity)?;
    let update_env = |key, values: &[String]| {
        let value = values.join("\n");
        let mut command_process = command_process.write()?;
        command_process.add_environment_variable(key, &value)?;
        Ok(())
    };

    let update_from_record_missing = |actual: &XvcDependency| -> Result<()> {
        if let Some(items) = actual.items() {
            match actual {
                XvcDependency::GlobItems(_) => {
                    update_env("XVC_ADDED_GLOB_ITEMS", &items)?;
                    update_env("XVC_REMOVED_GLOB_ITEMS", &[])?;
                    update_env("XVC_ALL_GLOB_ITEMS", &items)
                }
                XvcDependency::RegexItems(_) => {
                    update_env("XVC_ADDED_REGEX_ITEMS", &items)?;
                    update_env("XVC_REMOVED_REGEX_ITEMS", &[])?;
                    update_env("XVC_ALL_REGEX_ITEMS", &items)
                }
                XvcDependency::LineItems(_) => {
                    update_env("XVC_ADDED_ITEMS", &items)?;
                    update_env("XVC_REMOVED_LINE_ITEMS", &[])?;
                    update_env("XVC_ALL_LINE_ITEMS", &items)
                }
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    };

    let update_from_actual_missing = |record: &XvcDependency| -> Result<()> {
        if let Some(items) = record.items() {
            match record {
                XvcDependency::GlobItems(_dep) => {
                    update_env("XVC_ADDED_GLOB_ITEMS", &[])?;
                    update_env("XVC_REMOVED_GLOB_ITEMS", &items)?;
                    update_env("XVC_ALL_GLOB_ITEMS", &items)
                }
                XvcDependency::RegexItems(_dep) => {
                    update_env("XVC_ADDED_REGEX_ITEMS", &[])?;
                    update_env("XVC_REMOVED_REGEX_ITEMS", &items)?;
                    update_env("XVC_ALL_REGEX_ITEMS", &items)
                }
                XvcDependency::LineItems(_dep) => {
                    update_env("XVC_ADDED_LINE_ITEMS", &[])?;
                    update_env("XVC_REMOVED_LINE_ITEMS", &items)?;
                    update_env("XVC_ALL_LINE_ITEMS", &items)
                }
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    };

    let update_from_different = |record: &XvcDependency, actual: &XvcDependency| -> Result<()> {
        if let Some(record_items) = record.items() {
            if let Some(actual_items) = actual.items() {
                let record_items = record_items.into_iter().collect::<HashSet<_>>();
                let actual_items = actual_items.into_iter().collect::<HashSet<_>>();
                let added_items = actual_items
                    .difference(&record_items)
                    .cloned()
                    .sorted()
                    .collect::<Vec<String>>();

                let removed_items = record_items
                    .difference(&actual_items)
                    .cloned()
                    .sorted()
                    .collect::<Vec<String>>();
                let all_items = actual_items
                    .union(&record_items)
                    .cloned()
                    .sorted()
                    .collect::<Vec<String>>();

                match record {
                    XvcDependency::GlobItems(record_dep) => {
                        if let XvcDependency::GlobItems(actual_dep) = actual {
                            let mut changed_items = vec![];
                            for (record_xp, record_digest) in
                                record_dep.xvc_path_content_digest_map.iter()
                            {
                                if let Some(actual_digest) =
                                    actual_dep.xvc_path_content_digest_map.get(record_xp)
                                {
                                    if actual_digest != record_digest {
                                        changed_items.push(record_xp.to_string());
                                    }
                                }
                            }
                            update_env("XVC_CHANGED_GLOB_ITEMS", &changed_items)?;
                        }

                        update_env("XVC_ADDED_GLOB_ITEMS", &added_items)?;
                        update_env("XVC_REMOVED_GLOB_ITEMS", &removed_items)?;
                        update_env("XVC_ALL_GLOB_ITEMS", &all_items)
                    }
                    XvcDependency::RegexItems(_dep) => {
                        update_env("XVC_ADDED_REGEX_ITEMS", &added_items)?;
                        update_env("XVC_REMOVED_REGEX_ITEMS", &removed_items)?;
                        update_env("XVC_ALL_REGEX_ITEMS", &all_items)
                    }
                    XvcDependency::LineItems(_dep) => {
                        update_env("XVC_ADDED_LINE_ITEMS", &added_items)?;
                        update_env("XVC_REMOVED_LINE_ITEMS", &removed_items)?;
                        update_env("XVC_ALL_LINE_ITEMS", &all_items)
                    }
                    _ => Ok(()),
                }
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    };

    let update_from_identical = |dep: XvcDependency| -> Result<()> {
        if let Some(items) = dep.items() {
            match dep {
                XvcDependency::GlobItems(_) => {
                    update_env("XVC_ADDED_GLOB_ITEMS", &[])?;
                    update_env("XVC_REMOVED_GLOB_ITEMS", &[])?;
                    update_env("XVC_ALL_GLOB_ITEMS", &items)
                }
                XvcDependency::RegexItems(_) => {
                    update_env("XVC_ADDED_REGEX_ITEMS", &[])?;
                    update_env("XVC_REMOVED_REGEX_ITEMS", &[])?;
                    update_env("XVC_ALL_REGEX_ITEMS", &items)
                }
                XvcDependency::LineItems(_) => {
                    update_env("XVC_ADDED_LINE_ITEMS", &[])?;
                    update_env("XVC_REMOVED_LINE_ITEMS", &[])?;
                    update_env("XVC_ALL_LINE_ITEMS", &items)
                }
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    };

    for (dep_e, dep) in deps.into_iter() {
        if let Some(diff) = params.dependency_diffs.read()?.get(&dep_e) {
            match diff {
                Diff::Identical => uwr!(update_from_identical(dep), params.output_snd),
                Diff::RecordMissing { actual } => {
                    uwr!(update_from_record_missing(actual), params.output_snd)
                }
                Diff::ActualMissing { record } => {
                    uwr!(update_from_actual_missing(record), params.output_snd)
                }
                Diff::Different { record, actual } => {
                    uwr!(update_from_different(record, actual), params.output_snd)
                }
                Diff::Skipped => update_from_identical(dep)?,
            }
        }
    }

    Ok(())
}

fn s_running_f_wait_process<'a>(
    s: &RunningState,
    params: StepStateParams<'a>,
) -> StateTransition<'a> {
    watch!(params);
    let mut return_state: Option<XvcStepState>;
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
        let send_output = |cp: Arc<RwLock<CommandProcess>>| -> Result<()> {
            let mut cp = cp.write()?;
            cp.update_output_channels()?;
            // We currently pass all the output to the main thread
            // In the future, these can be passed to different channels.
            let output_snd = params.output_snd;
            cp.stderr_receiver.try_iter().for_each(|out| {
                if !out.is_empty() {
                    warn!(output_snd, "{}", out)
                }
            });

            cp.stdout_receiver.try_iter().for_each(|out| {
                if !out.is_empty() {
                    output!(output_snd, "{}", out);
                }
            });

            Ok(())
        };

        // We put process operations in an inner scope not to interfere with the process while sleeping
        // Check whether the process is still running

        {
            let cp = command_process.clone();
            let mut cp = cp.write()?;
            let process = cp
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
                            params.output_snd,
                            "Step {} with command {} is still running", &step.name, &step_command
                        );
                        return_state = None;
                    } else {
                        if params.terminate_timeout_processes {
                            error!(
                                params.output_snd,
                                "Process timeout for step {} with command {} ",
                                &step.name,
                                &step_command
                            );
                            process.terminate().ok();
                        }
                        return_state = Some(s.process_timeout());
                    }
                }

                Some(exit_code) => match exit_code {
            ExitStatus::Exited(0) => {
                output!(params.output_snd, "[DONE] {} ({})\n", step.name, step_command);
                return_state = Some(s.process_completed_successfully());
            }
            ,
            // we don't handle other variants in the state machine. Either it exited
            // successfully or died for some reason.
            //
            _ => {
    // We get the remaining output at the end;
                error!(params.output_snd, "Step {} finished UNSUCCESSFULLY with command {}", step.name, step_command);
                return_state = Some(s.process_returned_non_zero());
            },
        },
            }
        }

        send_output(command_process.clone())?;
        if return_state.is_some() {
            break;
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
