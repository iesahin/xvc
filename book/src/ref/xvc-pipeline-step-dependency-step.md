
### Step Dependencies

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

You can add a step dependency to a step. These steps specify dependency relationships explicitly, without relying on
 changed files or directories.

```console
$ xvc pipeline step new --step-name world --command "echo world"
$ xvc pipeline step new --step-name hello --command "echo hello"
$ xvc pipeline step dependency --step-name world --step hello
```

When run, the dependency will be run first and the step will be run after.

```console
$ xvc --debug pipeline run
[DEBUG][logging/src/lib.rs::236] Terminal logger enabled with level: Error
[DEBUG][logging/src/lib.rs::239] File logger enabled with level: Trace to "/var/folders/tk/3vn311ps4kqdhgykj3jg_p8r0000gn/T//xvc.log"
[TRACE][core/src/types/xvcroot.rs::247] "."
[DEBUG][core/src/types/xvcroot.rs::253] XVC DIR: "[CWD]"
[DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][config/src/lib.rs::536] cli_config: [
    "core.verbosity = quiet",
    "core.quiet = false",
]
[TRACE][config/src/lib.rs::540] map: {
    "core.verbosity": String(
        "quiet",
    ),
    "core.quiet": Boolean(
        false,
    ),
}
[TRACE][config/src/lib.rs::543] conf: XvcConfig {
    current_dir: XvcConfigOption {
        source: Runtime,
        option: AbsolutePath(
            "[CWD]",
        ),
    },
    config_maps: [
        XvcConfigMap {
            source: Default,
            map: {
                "file.recheck.method": String(
                    "copy",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "core.guid": String(
                    "dc2cce7e8228a107",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "git.command": String(
                    "git",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "core.guid": String(
                    "66e7d5b26275a147",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
            },
        },
        XvcConfigMap {
            source: Local,
            map: {},
        },
        XvcConfigMap {
            source: Environment,
            map: {},
        },
        XvcConfigMap {
            source: CommandLine,
            map: {
                "core.verbosity": String(
                    "quiet",
                ),
                "core.quiet": Boolean(
                    false,
                ),
            },
        },
    ],
    the_config: {
        "file.track.text_or_binary": XvcConfigValue {
            source: Project,
            value: String(
                "auto",
            ),
        },
        "file.list.no_summary": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "file.track.force": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "file.list.recursive": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "file.recheck.method": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
            ),
        },
        "file.track.no_parallel": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "git.auto_stage": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "file.track.no_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "git.auto_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "core.quiet": XvcConfigValue {
            source: CommandLine,
            value: Boolean(
                false,
            ),
        },
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
            ),
        },
        "pipeline.default": XvcConfigValue {
            source: Project,
            value: String(
                "default",
            ),
        },
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
            ),
        },
        "pipeline.current_pipeline": XvcConfigValue {
            source: Project,
            value: String(
                "default",
            ),
        },
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "66e7d5b26275a147",
            ),
        },
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
            ),
        },
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "quiet",
            ),
        },
        "file.carry-in.force": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "file.carry-in.no_parallel": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
            ),
        },
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"dc2cce7e8228a107/"
# Default verbosity level.
# One of /"error/", /"warn/", /"info/"
verbosity = /"error/"

[git]
# Automate git operations.
# Turning this off leads Xvc to behave as if it's not in a Git repository.
# Not recommended unless you're really not using Git
use_git = true
# Command to run Git process.
# You can set this to an absolute path to specify an executable
# If set to a non-absolute path, the executable will be searched in $PATH.
command = /"git/"

# Commit changes in .xvc/ directory after commands.
# You can set this to false if you want to commit manually.
auto_commit = true

# Stage changes in .xvc/ directory without committing.
# auto_commit implies auto_stage.
# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
auto_stage = false

[cache]
# The hash algorithm used for the cache.
# It may take blake3, blake2, sha2 or sha3 as values.
# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# The cache path is produced by prepending algorithm name to the cache.
# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
algorithm = /"blake3/"

[file]

[file.track]

# Don't move file content to cache after xvc file track
no_commit = false
# Force to track files even if they are already tracked.
force = false

# Xvc calculates file content digest differently for text and binary files.
# This option controls whether to treat files as text or binary.
# It may take auto, text or binary as values.
# Auto check each file individually and treat it as text if it's text.
text_or_binary = /"auto/"

# Don't use parallelism in track operations.
# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
no_parallel = false

[file.list]

# Format for `xvc file list` rows. You can reorder or remove columns.
# The following are the keys for each row:
# - {acd64}:  actual content digest. All 64 digits from the workspace file's content.
# - {acd8}:  actual content digest. First 8 digits the file content digest.
# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),
#   symlink (S), hardlink (H) or reflink (R).
# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,
#   GB and TB to represent sizes larger than 1MB.
# - {ats}:  actual timestamp. The timestamp of the workspace file.
# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show
#   whether the file timestamp is the same as the cached timestamp, newer,
#   older, not cached or not tracked.
# - {name}: The name of the file or directory.
# - {rcd64}:  recorded content digest. All 64 digits.
# - {rcd8}:  recorded content digest. First 8 digits.
# - {rrm}:  recorded recheck method. Whether the entry is linked to the workspace
#   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# - {rsz}:  recorded size. The size of the cached content in bytes. It uses
#   MB, GB and TB to represent sizes larged than 1MB.
# - {rts}:  recorded timestamp. The timestamp of the cached content.
#
# There are no escape sequences in the format string.
# If you want to add a tab, type it to the string.
# If you want to add a literal double curly brace, open an issue.
format = /"{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"

# Default sort order for `xvc file list`.
# Valid values are
# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
sort = /"name-desc/"

# Do not show a summary for as the final row for `xvc file list`.
no_summary = false

# List files recursively always.
recursive = false

[file.carry-in]
# Carry-in the files to cache always, even if they are already present.
force = false

# Don't use parallel move/copy in carry-in
no_parallel = false

[file.recheck]
# The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# The default is copy to make sure the options is portable.
# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# Note that hardlink and symlink are read-only as they link the files in cache.
method = /"copy/"

[pipeline]
# Name of the current pipeline to run
current_pipeline = /"default/"
# Name of the default pipeline
default = /"default/"
# Name of the default params file name
default_params_file = /"params.yaml/"

",
        current_dir: AbsolutePath(
            "[CWD]",
        ),
        include_system_config: true,
        include_user_config: true,
        project_config_path: Some(
            AbsolutePath(
                "[CWD]/.xvc/config.toml",
            ),
        ),
        local_config_path: Some(
            AbsolutePath(
                "[CWD]/.xvc/config.local.toml",
            ),
        ),
        include_environment_config: true,
        command_line_config: Some(
            [
                "core.verbosity = quiet",
                "core.quiet = false",
            ],
        ),
    },
}
[TRACE][ecs/src/ecs/mod.rs::229] dir: "[CWD]/.xvc/ec"
[TRACE][ecs/src/ecs/mod.rs::239] files: [
    "[CWD]/.xvc/ec/1689789075618228",
    "[CWD]/.xvc/ec/1689789075621709",
    "[CWD]/.xvc/ec/1689789075693403",
    "[CWD]/.xvc/ec/1689789075765633",
    "[CWD]/.xvc/ec/1689789075836888",
]
[TRACE][pipeline/src/lib.rs::350] name: Some(
    "default",
)
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/.gitignore"
[TRACE][core/src/types/xvcpath.rs::88] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::89] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/.xvcignore"
[TRACE][core/src/types/xvcpath.rs::88] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::89] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/notify.rs::160] watcher: FsEventWatcher {
    paths: 0x0000600000000000,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000600003f1c1f0,
    runloop: Some(
        (
            0x000060000321c800,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::303] pipeline_len: 2
[TRACE][pipeline/src/pipeline/mod.rs::329] &dependency_graph: {
    XvcEntity(
        3,
        5012156582048127371,
    ): [
        (
            XvcEntity(
                2,
                12869007782330632556,
            ),
            Incoming,
        ),
    ],
    XvcEntity(
        2,
        12869007782330632556,
    ): [
        (
            XvcEntity(
                3,
                5012156582048127371,
            ),
            Outgoing,
        ),
    ],
}
[INFO][pipeline/src/pipeline/mod.rs::343] Pipeline Graph:
digraph {
    0 [ label = "(3, 5012156582048127371)" ]
    1 [ label = "(2, 12869007782330632556)" ]
    1 -> 0 [ label = "Step" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::420] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                3,
                5012156582048127371,
            ): Begin(
                FromInit,
            ),
            XvcEntity(
                2,
                12869007782330632556,
            ): Begin(
                FromInit,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::546] &step_thread_store: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): ScopedJoinHandle { .. },
        XvcEntity(
            2,
            12869007782330632556,
        ): ScopedJoinHandle { .. },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::550] (step_e, &jh): (
    XvcEntity(
        3,
        5012156582048127371,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::702] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::702] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::811] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::812] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::814] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::702] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::811] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::811] step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::812] &r_next_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::814] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::812] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::702] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::814] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::702] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::811] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::812] &r_next_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::811] step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::814] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::812] &r_next_state: WaitingDependencySteps(
    FromDependencyStepsRunning,
)
[TRACE][pipeline/src/pipeline/mod.rs::702] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::1061] params.step_dependencies.len(): 0
[TRACE][pipeline/src/pipeline/mod.rs::1064] params.step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::811] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::812] &r_next_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::814] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::702] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::1112] deps.is_empty(): true
[TRACE][pipeline/src/pipeline/mod.rs::814] &step_state: WaitingDependencySteps(
    FromDependencyStepsRunning,
)
[TRACE][pipeline/src/pipeline/mod.rs::811] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::812] &r_next_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::814] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::702] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::811] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::812] &r_next_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::814] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::702] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::702] &step_state: WaitingDependencySteps(
    FromDependencyStepsRunning,
)
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::1403] params: StepStateParams {
    xvc_root: XvcRootInner {
        absolute_path: AbsolutePath(
            "[CWD]",
        ),
        xvc_dir: AbsolutePath(
            "[CWD]/.xvc",
        ),
        store_dir: AbsolutePath(
            "[CWD]/.xvc/store",
        ),
        config: XvcConfig {
            current_dir: XvcConfigOption {
                source: Runtime,
                option: AbsolutePath(
                    "[CWD]",
                ),
            },
            config_maps: [
                XvcConfigMap {
                    source: Default,
                    map: {
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "core.guid": String(
                            "dc2cce7e8228a107",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.command": String(
                            "git",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "core.guid": String(
                            "66e7d5b26275a147",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Local,
                    map: {},
                },
                XvcConfigMap {
                    source: Environment,
                    map: {},
                },
                XvcConfigMap {
                    source: CommandLine,
                    map: {
                        "core.verbosity": String(
                            "quiet",
                        ),
                        "core.quiet": Boolean(
                            false,
                        ),
                    },
                },
            ],
            the_config: {
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.recursive": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "git.auto_stage": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
                    value: Boolean(
                        false,
                    ),
                },
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
                    ),
                },
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "66e7d5b26275a147",
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "quiet",
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"dc2cce7e8228a107/"
# Default verbosity level.
# One of /"error/", /"warn/", /"info/"
verbosity = /"error/"

[git]
# Automate git operations.
# Turning this off leads Xvc to behave as if it's not in a Git repository.
# Not recommended unless you're really not using Git
use_git = true
# Command to run Git process.
# You can set this to an absolute path to specify an executable
# If set to a non-absolute path, the executable will be searched in $PATH.
command = /"git/"

# Commit changes in .xvc/ directory after commands.
# You can set this to false if you want to commit manually.
auto_commit = true

# Stage changes in .xvc/ directory without committing.
# auto_commit implies auto_stage.
# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
auto_stage = false

[cache]
# The hash algorithm used for the cache.
# It may take blake3, blake2, sha2 or sha3 as values.
# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# The cache path is produced by prepending algorithm name to the cache.
# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
algorithm = /"blake3/"

[file]

[file.track]

# Don't move file content to cache after xvc file track
no_commit = false
# Force to track files even if they are already tracked.
force = false

# Xvc calculates file content digest differently for text and binary files.
# This option controls whether to treat files as text or binary.
# It may take auto, text or binary as values.
# Auto check each file individually and treat it as text if it's text.
text_or_binary = /"auto/"

# Don't use parallelism in track operations.
# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
no_parallel = false

[file.list]

# Format for `xvc file list` rows. You can reorder or remove columns.
# The following are the keys for each row:
# - {acd64}:  actual content digest. All 64 digits from the workspace file's content.
# - {acd8}:  actual content digest. First 8 digits the file content digest.
# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),
#   symlink (S), hardlink (H) or reflink (R).
# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,
#   GB and TB to represent sizes larger than 1MB.
# - {ats}:  actual timestamp. The timestamp of the workspace file.
# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show
#   whether the file timestamp is the same as the cached timestamp, newer,
#   older, not cached or not tracked.
# - {name}: The name of the file or directory.
# - {rcd64}:  recorded content digest. All 64 digits.
# - {rcd8}:  recorded content digest. First 8 digits.
# - {rrm}:  recorded recheck method. Whether the entry is linked to the workspace
#   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# - {rsz}:  recorded size. The size of the cached content in bytes. It uses
#   MB, GB and TB to represent sizes larged than 1MB.
# - {rts}:  recorded timestamp. The timestamp of the cached content.
#
# There are no escape sequences in the format string.
# If you want to add a tab, type it to the string.
# If you want to add a literal double curly brace, open an issue.
format = /"{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"

# Default sort order for `xvc file list`.
# Valid values are
# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
sort = /"name-desc/"

# Do not show a summary for as the final row for `xvc file list`.
no_summary = false

# List files recursively always.
recursive = false

[file.carry-in]
# Carry-in the files to cache always, even if they are already present.
force = false

# Don't use parallel move/copy in carry-in
no_parallel = false

[file.recheck]
# The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# The default is copy to make sure the options is portable.
# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# Note that hardlink and symlink are read-only as they link the files in cache.
method = /"copy/"

[pipeline]
# Name of the current pipeline to run
current_pipeline = /"default/"
# Name of the default pipeline
default = /"default/"
# Name of the default params file name
default_params_file = /"params.yaml/"

",
                current_dir: AbsolutePath(
                    "[CWD]",
                ),
                include_system_config: true,
                include_user_config: true,
                project_config_path: Some(
                    AbsolutePath(
                        "[CWD]/.xvc/config.toml",
                    ),
                ),
                local_config_path: Some(
                    AbsolutePath(
                        "[CWD]/.xvc/config.local.toml",
                    ),
                ),
                include_environment_config: true,
                command_line_config: Some(
                    [
                        "core.verbosity = quiet",
                        "core.quiet = false",
                    ],
                ),
            },
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 5,
            random: 4914651369345530668,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                ".gitignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    107,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1689789075,
                        tv_nsec: 618569215,
                    },
                ),
            },
            XvcPath(
                ".xvcignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    130,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1689789075,
                        tv_nsec: 618480798,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
        always: true,
        wait_running_dep_steps: true,
        ignore_broken_dep_steps: true,
        ignore_missing_dependencies: true,
        ignore_superficial_diffs: true,
        ignore_thorough_diffs: true,
        ignore_missing_outputs: true,
    },
    pipeline_rundir: XvcPath(
        "",
    ),
    terminate_timeout_processes: true,
    algorithm: Blake3,
    command_process: RwLock {
        data: CommandProcess {
            environment: {},
            step: XvcStep {
                name: "hello",
            },
            step_command: XvcStepCommand {
                command: "echo hello",
            },
            birth: None,
            process: None,
            stdout_sender: Sender { .. },
            stderr_sender: Sender { .. },
            stdout_receiver: Receiver { .. },
            stderr_receiver: Receiver { .. },
        },
        poisoned: false,
        ..
    },
    available_process_slots: RwLock {
        data: 1,
        poisoned: false,
        ..
    },
    process_poll_milliseconds: 10,
    dependency_diffs: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    output_diffs: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    step_e: XvcEntity(
        3,
        5012156582048127371,
    ),
    step: XvcStep {
        name: "hello",
    },
    step_command: XvcStepCommand {
        command: "echo hello",
    },
    dependency_states: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    step_timeout: 10000s,
    step_dependencies: HStore {
        map: {},
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::811] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::812] &r_next_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::814] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::702] &step_state: Running(
    FromStartProcess,
)
[TRACE][walker/src/notify.rs::56] event: Ok(
    Event {
        kind: Create(
            Folder,
        ),
        paths: [
            "[CWD]/.xvc/store/xvc-pipeline-run-dir-store",
        ],
        attr:tracker: None,
        attr:flag: None,
        attr:info: None,
        attr:source: None,
    },
)
[TRACE][walker/src/notify.rs::56] event: Ok(
    Event {
        kind: Create(
            Folder,
        ),
        paths: [
            "[CWD]/.xvc/store/xvc-path-store",
        ],
        attr:tracker: None,
        attr:flag: None,
        attr:info: None,
        attr:source: None,
    },
)
[TRACE][walker/src/notify.rs::56] event: Ok(
    Event {
        kind: Create(
            Folder,
        ),
        paths: [
            "[CWD]/.xvc/store/xvc-path-xvc-dependency-r1n-store",
        ],
        attr:tracker: None,
        attr:flag: None,
        attr:info: None,
        attr:source: None,
    },
)
[TRACE][walker/src/notify.rs::56] event: Ok(
    Event {
        kind: Create(
            Folder,
        ),
        paths: [
            "[CWD]/.xvc/store/xvc-metadata-store",
        ],
        attr:tracker: None,
        attr:flag: None,
        attr:info: None,
        attr:source: None,
    },
)
[TRACE][walker/src/notify.rs::56] event: Ok(
    Event {
        kind: Create(
            Folder,
        ),
        paths: [
            "[CWD]/.xvc/store/xvc-digests-store",
        ],
        attr:tracker: None,
        attr:flag: None,
        attr:info: None,
        attr:source: None,
    },
)
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/.xvc/store/xvc-pipeline-run-dir-store"
[TRACE][walker/src/notify.rs::56] event: Ok(
    Event {
        kind: Create(
            Folder,
        ),
        paths: [
            "[CWD]/.xvc/store/text-or-binary-store",
        ],
        attr:tracker: None,
        attr:flag: None,
        attr:info: None,
        attr:source: None,
    },
)
[TRACE][core/src/types/xvcpath.rs::88] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/notify.rs::56] event: Ok(
    Event {
        kind: Create(
            Folder,
        ),
        paths: [
            "[CWD]/.xvc/store/xvc-digests-xvc-step-r1n-store",
        ],
        attr:tracker: None,
        attr:flag: None,
        attr:info: None,
        attr:source: None,
    },
)
[TRACE][core/src/types/xvcpath.rs::89] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/.xvc/store/xvc-path-store"
[TRACE][core/src/types/xvcpath.rs::88] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::89] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/.xvc/store/xvc-path-xvc-dependency-r1n-store"
[TRACE][core/src/types/xvcpath.rs::88] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::89] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/.xvc/store/xvc-metadata-store"
[TRACE][core/src/types/xvcpath.rs::88] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::89] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/.xvc/store/xvc-digests-store"
[TRACE][core/src/types/xvcpath.rs::88] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::89] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/.xvc/store/text-or-binary-store"
[TRACE][core/src/types/xvcpath.rs::88] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::89] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/.xvc/store/xvc-digests-xvc-step-r1n-store"
[TRACE][core/src/types/xvcpath.rs::88] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::89] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::811] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::812] &r_next_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::814] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::702] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1292] params: StepStateParams {
    xvc_root: XvcRootInner {
        absolute_path: AbsolutePath(
            "[CWD]",
        ),
        xvc_dir: AbsolutePath(
            "[CWD]/.xvc",
        ),
        store_dir: AbsolutePath(
            "[CWD]/.xvc/store",
        ),
        config: XvcConfig {
            current_dir: XvcConfigOption {
                source: Runtime,
                option: AbsolutePath(
                    "[CWD]",
                ),
            },
            config_maps: [
                XvcConfigMap {
                    source: Default,
                    map: {
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "core.guid": String(
                            "dc2cce7e8228a107",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.command": String(
                            "git",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "core.guid": String(
                            "66e7d5b26275a147",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Local,
                    map: {},
                },
                XvcConfigMap {
                    source: Environment,
                    map: {},
                },
                XvcConfigMap {
                    source: CommandLine,
                    map: {
                        "core.verbosity": String(
                            "quiet",
                        ),
                        "core.quiet": Boolean(
                            false,
                        ),
                    },
                },
            ],
            the_config: {
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.recursive": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "git.auto_stage": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
                    value: Boolean(
                        false,
                    ),
                },
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
                    ),
                },
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "66e7d5b26275a147",
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "quiet",
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"dc2cce7e8228a107/"
# Default verbosity level.
# One of /"error/", /"warn/", /"info/"
verbosity = /"error/"

[git]
# Automate git operations.
# Turning this off leads Xvc to behave as if it's not in a Git repository.
# Not recommended unless you're really not using Git
use_git = true
# Command to run Git process.
# You can set this to an absolute path to specify an executable
# If set to a non-absolute path, the executable will be searched in $PATH.
command = /"git/"

# Commit changes in .xvc/ directory after commands.
# You can set this to false if you want to commit manually.
auto_commit = true

# Stage changes in .xvc/ directory without committing.
# auto_commit implies auto_stage.
# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
auto_stage = false

[cache]
# The hash algorithm used for the cache.
# It may take blake3, blake2, sha2 or sha3 as values.
# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# The cache path is produced by prepending algorithm name to the cache.
# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
algorithm = /"blake3/"

[file]

[file.track]

# Don't move file content to cache after xvc file track
no_commit = false
# Force to track files even if they are already tracked.
force = false

# Xvc calculates file content digest differently for text and binary files.
# This option controls whether to treat files as text or binary.
# It may take auto, text or binary as values.
# Auto check each file individually and treat it as text if it's text.
text_or_binary = /"auto/"

# Don't use parallelism in track operations.
# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
no_parallel = false

[file.list]

# Format for `xvc file list` rows. You can reorder or remove columns.
# The following are the keys for each row:
# - {acd64}:  actual content digest. All 64 digits from the workspace file's content.
# - {acd8}:  actual content digest. First 8 digits the file content digest.
# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),
#   symlink (S), hardlink (H) or reflink (R).
# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,
#   GB and TB to represent sizes larger than 1MB.
# - {ats}:  actual timestamp. The timestamp of the workspace file.
# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show
#   whether the file timestamp is the same as the cached timestamp, newer,
#   older, not cached or not tracked.
# - {name}: The name of the file or directory.
# - {rcd64}:  recorded content digest. All 64 digits.
# - {rcd8}:  recorded content digest. First 8 digits.
# - {rrm}:  recorded recheck method. Whether the entry is linked to the workspace
#   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# - {rsz}:  recorded size. The size of the cached content in bytes. It uses
#   MB, GB and TB to represent sizes larged than 1MB.
# - {rts}:  recorded timestamp. The timestamp of the cached content.
#
# There are no escape sequences in the format string.
# If you want to add a tab, type it to the string.
# If you want to add a literal double curly brace, open an issue.
format = /"{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"

# Default sort order for `xvc file list`.
# Valid values are
# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
sort = /"name-desc/"

# Do not show a summary for as the final row for `xvc file list`.
no_summary = false

# List files recursively always.
recursive = false

[file.carry-in]
# Carry-in the files to cache always, even if they are already present.
force = false

# Don't use parallel move/copy in carry-in
no_parallel = false

[file.recheck]
# The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# The default is copy to make sure the options is portable.
# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# Note that hardlink and symlink are read-only as they link the files in cache.
method = /"copy/"

[pipeline]
# Name of the current pipeline to run
current_pipeline = /"default/"
# Name of the default pipeline
default = /"default/"
# Name of the default params file name
default_params_file = /"params.yaml/"

",
                current_dir: AbsolutePath(
                    "[CWD]",
                ),
                include_system_config: true,
                include_user_config: true,
                project_config_path: Some(
                    AbsolutePath(
                        "[CWD]/.xvc/config.toml",
                    ),
                ),
                local_config_path: Some(
                    AbsolutePath(
                        "[CWD]/.xvc/config.local.toml",
                    ),
                ),
                include_environment_config: true,
                command_line_config: Some(
                    [
                        "core.verbosity = quiet",
                        "core.quiet = false",
                    ],
                ),
            },
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 5,
            random: 4914651369345530668,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                ".gitignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    107,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1689789075,
                        tv_nsec: 618569215,
                    },
                ),
            },
            XvcPath(
                ".xvcignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    130,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1689789075,
                        tv_nsec: 618480798,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
        always: true,
        wait_running_dep_steps: true,
        ignore_broken_dep_steps: true,
        ignore_missing_dependencies: true,
        ignore_superficial_diffs: true,
        ignore_thorough_diffs: true,
        ignore_missing_outputs: true,
    },
    pipeline_rundir: XvcPath(
        "",
    ),
    terminate_timeout_processes: true,
    algorithm: Blake3,
    command_process: RwLock {
        data: CommandProcess {
            environment: {},
            step: XvcStep {
                name: "hello",
            },
            step_command: XvcStepCommand {
                command: "echo hello",
            },
            birth: Some(
                Instant {
                    tv_sec: 399578,
                    tv_nsec: 534956916,
                },
            ),
            process: Some(
                Popen {
                    stdin: None,
                    stdout: Some(
                        File {
                            fd: 7,
                            read: true,
                            write: false,
                        },
                    ),
                    stderr: Some(
                        File {
                            fd: 9,
                            read: true,
                            write: false,
                        },
                    ),
                    child_state: Running {
                        pid: 27317,
                        ext: (),
                    },
                    detached: true,
                },
            ),
            stdout_sender: Sender { .. },
            stderr_sender: Sender { .. },
            stdout_receiver: Receiver { .. },
            stderr_receiver: Receiver { .. },
        },
        poisoned: false,
        ..
    },
    available_process_slots: RwLock {
        data: 0,
        poisoned: false,
        ..
    },
    process_poll_milliseconds: 10,
    dependency_diffs: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    output_diffs: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    step_e: XvcEntity(
        3,
        5012156582048127371,
    ),
    step: XvcStep {
        name: "hello",
    },
    step_command: XvcStepCommand {
        command: "echo hello",
    },
    dependency_states: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    step_timeout: 10000s,
    step_dependencies: HStore {
        map: {},
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[OUT] [hello] hello
 
[TRACE][pipeline/src/pipeline/mod.rs::1324] command_process: CommandProcess {
    environment: {},
    step: XvcStep {
        name: "hello",
    },
    step_command: XvcStepCommand {
        command: "echo hello",
    },
    birth: Some(
        Instant {
            tv_sec: 399578,
            tv_nsec: 534956916,
        },
    ),
    process: Some(
        Popen {
            stdin: None,
            stdout: Some(
                File {
                    fd: 7,
                    read: true,
                    write: false,
                },
            ),
            stderr: Some(
                File {
                    fd: 9,
                    read: true,
                    write: false,
                },
            ),
            child_state: Running {
                pid: 27317,
                ext: (),
            },
            detached: true,
        },
    ),
    stdout_sender: Sender { .. },
    stderr_sender: Sender { .. },
    stdout_receiver: Receiver { .. },
    stderr_receiver: Receiver { .. },
}
[TRACE][pipeline/src/pipeline/mod.rs::1330] &process: Popen {
    stdin: None,
    stdout: Some(
        File {
            fd: 7,
            read: true,
            write: false,
        },
    ),
    stderr: Some(
        File {
            fd: 9,
            read: true,
            write: false,
        },
    ),
    child_state: Running {
        pid: 27317,
        ext: (),
    },
    detached: true,
}
[TRACE][pipeline/src/pipeline/mod.rs::1377] return_state: Some(
    Done(
        FromProcessCompletedSuccessfully,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::1383] params: StepStateParams {
    xvc_root: XvcRootInner {
        absolute_path: AbsolutePath(
            "[CWD]",
        ),
        xvc_dir: AbsolutePath(
            "[CWD]/.xvc",
        ),
        store_dir: AbsolutePath(
            "[CWD]/.xvc/store",
        ),
        config: XvcConfig {
            current_dir: XvcConfigOption {
                source: Runtime,
                option: AbsolutePath(
                    "[CWD]",
                ),
            },
            config_maps: [
                XvcConfigMap {
                    source: Default,
                    map: {
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "core.guid": String(
                            "dc2cce7e8228a107",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.command": String(
                            "git",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "core.guid": String(
                            "66e7d5b26275a147",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Local,
                    map: {},
                },
                XvcConfigMap {
                    source: Environment,
                    map: {},
                },
                XvcConfigMap {
                    source: CommandLine,
                    map: {
                        "core.verbosity": String(
                            "quiet",
                        ),
                        "core.quiet": Boolean(
                            false,
                        ),
                    },
                },
            ],
            the_config: {
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.recursive": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "git.auto_stage": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
                    value: Boolean(
                        false,
                    ),
                },
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
                    ),
                },
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "66e7d5b26275a147",
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "quiet",
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"dc2cce7e8228a107/"
# Default verbosity level.
# One of /"error/", /"warn/", /"info/"
verbosity = /"error/"

[git]
# Automate git operations.
# Turning this off leads Xvc to behave as if it's not in a Git repository.
# Not recommended unless you're really not using Git
use_git = true
# Command to run Git process.
# You can set this to an absolute path to specify an executable
# If set to a non-absolute path, the executable will be searched in $PATH.
command = /"git/"

# Commit changes in .xvc/ directory after commands.
# You can set this to false if you want to commit manually.
auto_commit = true

# Stage changes in .xvc/ directory without committing.
# auto_commit implies auto_stage.
# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
auto_stage = false

[cache]
# The hash algorithm used for the cache.
# It may take blake3, blake2, sha2 or sha3 as values.
# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# The cache path is produced by prepending algorithm name to the cache.
# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
algorithm = /"blake3/"

[file]

[file.track]

# Don't move file content to cache after xvc file track
no_commit = false
# Force to track files even if they are already tracked.
force = false

# Xvc calculates file content digest differently for text and binary files.
# This option controls whether to treat files as text or binary.
# It may take auto, text or binary as values.
# Auto check each file individually and treat it as text if it's text.
text_or_binary = /"auto/"

# Don't use parallelism in track operations.
# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
no_parallel = false

[file.list]

# Format for `xvc file list` rows. You can reorder or remove columns.
# The following are the keys for each row:
# - {acd64}:  actual content digest. All 64 digits from the workspace file's content.
# - {acd8}:  actual content digest. First 8 digits the file content digest.
# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),
#   symlink (S), hardlink (H) or reflink (R).
# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,
#   GB and TB to represent sizes larger than 1MB.
# - {ats}:  actual timestamp. The timestamp of the workspace file.
# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show
#   whether the file timestamp is the same as the cached timestamp, newer,
#   older, not cached or not tracked.
# - {name}: The name of the file or directory.
# - {rcd64}:  recorded content digest. All 64 digits.
# - {rcd8}:  recorded content digest. First 8 digits.
# - {rrm}:  recorded recheck method. Whether the entry is linked to the workspace
#   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# - {rsz}:  recorded size. The size of the cached content in bytes. It uses
#   MB, GB and TB to represent sizes larged than 1MB.
# - {rts}:  recorded timestamp. The timestamp of the cached content.
#
# There are no escape sequences in the format string.
# If you want to add a tab, type it to the string.
# If you want to add a literal double curly brace, open an issue.
format = /"{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"

# Default sort order for `xvc file list`.
# Valid values are
# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
sort = /"name-desc/"

# Do not show a summary for as the final row for `xvc file list`.
no_summary = false

# List files recursively always.
recursive = false

[file.carry-in]
# Carry-in the files to cache always, even if they are already present.
force = false

# Don't use parallel move/copy in carry-in
no_parallel = false

[file.recheck]
# The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# The default is copy to make sure the options is portable.
# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# Note that hardlink and symlink are read-only as they link the files in cache.
method = /"copy/"

[pipeline]
# Name of the current pipeline to run
current_pipeline = /"default/"
# Name of the default pipeline
default = /"default/"
# Name of the default params file name
default_params_file = /"params.yaml/"

",
                current_dir: AbsolutePath(
                    "[CWD]",
                ),
                include_system_config: true,
                include_user_config: true,
                project_config_path: Some(
                    AbsolutePath(
                        "[CWD]/.xvc/config.toml",
                    ),
                ),
                local_config_path: Some(
                    AbsolutePath(
                        "[CWD]/.xvc/config.local.toml",
                    ),
                ),
                include_environment_config: true,
                command_line_config: Some(
                    [
                        "core.verbosity = quiet",
                        "core.quiet = false",
                    ],
                ),
            },
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 5,
            random: 4914651369345530668,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                ".gitignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    107,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1689789075,
                        tv_nsec: 618569215,
                    },
                ),
            },
            XvcPath(
                ".xvcignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    130,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1689789075,
                        tv_nsec: 618480798,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
        always: true,
        wait_running_dep_steps: true,
        ignore_broken_dep_steps: true,
        ignore_missing_dependencies: true,
        ignore_superficial_diffs: true,
        ignore_thorough_diffs: true,
        ignore_missing_outputs: true,
    },
    pipeline_rundir: XvcPath(
        "",
    ),
    terminate_timeout_processes: true,
    algorithm: Blake3,
    command_process: RwLock {
        data: CommandProcess {
            environment: {},
            step: XvcStep {
                name: "hello",
            },
            step_command: XvcStepCommand {
                command: "echo hello",
            },
            birth: Some(
                Instant {
                    tv_sec: 399578,
                    tv_nsec: 534956916,
                },
            ),
            process: Some(
                Popen {
                    stdin: None,
                    stdout: Some(
                        File {
                            fd: 7,
                            read: true,
                            write: false,
                        },
                    ),
                    stderr: Some(
                        File {
                            fd: 9,
                            read: true,
                            write: false,
                        },
                    ),
                    child_state: Finished(
                        Exited(
                            0,
                        ),
                    ),
                    detached: true,
                },
            ),
            stdout_sender: Sender { .. },
            stderr_sender: Sender { .. },
            stdout_receiver: Receiver { .. },
            stderr_receiver: Receiver { .. },
        },
        poisoned: false,
        ..
    },
    available_process_slots: RwLock {
        data: <locked>,
        poisoned: false,
        ..
    },
    process_poll_milliseconds: 10,
    dependency_diffs: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    output_diffs: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    step_e: XvcEntity(
        3,
        5012156582048127371,
    ),
    step: XvcStep {
        name: "hello",
    },
    step_command: XvcStepCommand {
        command: "echo hello",
    },
    dependency_states: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    step_timeout: 10000s,
    step_dependencies: HStore {
        map: {},
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::811] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::812] &r_next_state: Done(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::814] &step_state: Done(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::702] &step_state: Done(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::550] (step_e, &jh): (
    XvcEntity(
        2,
        12869007782330632556,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"
[TRACE][pipeline/src/pipeline/mod.rs::909] dep_states: HStore {
    map: {
        XvcEntity(
            3,
            5012156582048127371,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::944] params.step.name: "world"

```
