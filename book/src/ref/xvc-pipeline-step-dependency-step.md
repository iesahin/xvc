
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
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "core.guid": String(
                    "5bfe06c36204b7f2",
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "git.command": String(
                    "git",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "core.guid": String(
                    "5230caa6f592bd1a",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "git.use_git": Boolean(
                    true,
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
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
            ),
        },
        "file.list.recursive": XvcConfigValue {
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
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
            ),
        },
        "pipeline.default": XvcConfigValue {
            source: Project,
            value: String(
                "default",
            ),
        },
        "pipeline.current_pipeline": XvcConfigValue {
            source: Project,
            value: String(
                "default",
            ),
        },
        "file.track.force": XvcConfigValue {
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
        "file.track.no_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "5230caa6f592bd1a",
            ),
        },
        "file.recheck.method": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
            ),
        },
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
            ),
        },
        "file.carry-in.no_parallel": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "quiet",
            ),
        },
        "file.list.no_summary": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "file.track.no_parallel": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
            ),
        },
        "file.carry-in.force": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "core.quiet": XvcConfigValue {
            source: CommandLine,
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
        "file.track.text_or_binary": XvcConfigValue {
            source: Project,
            value: String(
                "auto",
            ),
        },
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
            ),
        },
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"5bfe06c36204b7f2/"
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
    "[CWD]/.xvc/ec/1690882797050490",
    "[CWD]/.xvc/ec/1690882797054352",
    "[CWD]/.xvc/ec/1690882797127100",
    "[CWD]/.xvc/ec/1690882797203079",
    "[CWD]/.xvc/ec/1690882797284398",
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
    paths: 0x000060000235c000,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000600001c44150,
    runloop: Some(
        (
            0x0000600001140100,
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
        18285583588640195063,
    ): [
        (
            XvcEntity(
                2,
                6211911852685607906,
            ),
            Incoming,
        ),
    ],
    XvcEntity(
        2,
        6211911852685607906,
    ): [
        (
            XvcEntity(
                3,
                18285583588640195063,
            ),
            Outgoing,
        ),
    ],
}
[INFO][pipeline/src/pipeline/mod.rs::343] Pipeline Graph:
digraph {
    0 [ label = "(3, 18285583588640195063)" ]
    1 [ label = "(2, 6211911852685607906)" ]
    1 -> 0 [ label = "Step" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::420] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                6211911852685607906,
            ): Begin(
                FromInit,
            ),
            XvcEntity(
                3,
                18285583588640195063,
            ): Begin(
                FromInit,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::546] &step_thread_store: HStore {
    map: {
        XvcEntity(
            2,
            6211911852685607906,
        ): ScopedJoinHandle { .. },
        XvcEntity(
            3,
            18285583588640195063,
        ): ScopedJoinHandle { .. },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::550] (step_e, &jh): (
    XvcEntity(
        2,
        6211911852685607906,
    ),
    ScopedJoinHandle { .. },
)
thread '<unnamed>' panicked at 'no entry found for key', pipeline/src/pipeline/mod.rs:664:24
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[ERROR] Error in step thread: Any { .. }
[TRACE][pipeline/src/pipeline/mod.rs::550] (step_e, &jh): (
    XvcEntity(
        3,
        18285583588640195063,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::700] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::818] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::819] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::821] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::700] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::818] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::819] &r_next_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::821] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::700] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::818] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::819] &r_next_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::821] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::700] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::1103] params.step_dependencies.len(): 0
[TRACE][pipeline/src/pipeline/mod.rs::1106] params.step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::818] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::819] &r_next_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::821] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::700] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::1154] deps.is_empty(): true
[TRACE][pipeline/src/pipeline/mod.rs::818] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::819] &r_next_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::821] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::700] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::818] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::819] &r_next_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::821] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::700] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::1445] params: StepStateParams {
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
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "core.guid": String(
                            "5bfe06c36204b7f2",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "5230caa6f592bd1a",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "git.use_git": Boolean(
                            true,
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
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
                    ),
                },
                "file.list.recursive": XvcConfigValue {
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "file.track.force": XvcConfigValue {
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
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "5230caa6f592bd1a",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "quiet",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"5bfe06c36204b7f2/"
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
            random: 14682102138141470512,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                ".xvcignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    130,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1690882797,
                        tv_nsec: 50745056,
                    },
                ),
            },
            XvcPath(
                ".gitignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    107,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1690882797,
                        tv_nsec: 50929468,
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
        18285583588640195063,
    ),
    step: XvcStep {
        name: "hello",
    },
    step_command: XvcStepCommand {
        command: "echo hello",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    6211911852685607906,
                ): Begin(
                    FromInit,
                ),
                XvcEntity(
                    3,
                    18285583588640195063,
                ): Begin(
                    FromInit,
                ),
            },
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
[TRACE][pipeline/src/pipeline/mod.rs::818] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::819] &r_next_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::821] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::700] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::818] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::819] &r_next_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::821] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::700] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1334] params: StepStateParams {
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
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "core.guid": String(
                            "5bfe06c36204b7f2",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "5230caa6f592bd1a",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "git.use_git": Boolean(
                            true,
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
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
                    ),
                },
                "file.list.recursive": XvcConfigValue {
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "file.track.force": XvcConfigValue {
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
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "5230caa6f592bd1a",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "quiet",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"5bfe06c36204b7f2/"
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
            random: 14682102138141470512,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                ".xvcignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    130,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1690882797,
                        tv_nsec: 50745056,
                    },
                ),
            },
            XvcPath(
                ".gitignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    107,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1690882797,
                        tv_nsec: 50929468,
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
                    tv_sec: 436307,
                    tv_nsec: 274511125,
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
                        pid: 67732,
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
        18285583588640195063,
    ),
    step: XvcStep {
        name: "hello",
    },
    step_command: XvcStepCommand {
        command: "echo hello",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    6211911852685607906,
                ): Begin(
                    FromInit,
                ),
                XvcEntity(
                    3,
                    18285583588640195063,
                ): Running(
                    FromWaitProcess,
                ),
            },
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
 
[TRACE][pipeline/src/pipeline/mod.rs::1366] command_process: CommandProcess {
    environment: {},
    step: XvcStep {
        name: "hello",
    },
    step_command: XvcStepCommand {
        command: "echo hello",
    },
    birth: Some(
        Instant {
            tv_sec: 436307,
            tv_nsec: 274511125,
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
                pid: 67732,
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
[TRACE][pipeline/src/pipeline/mod.rs::1372] &process: Popen {
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
        pid: 67732,
        ext: (),
    },
    detached: true,
}
[TRACE][pipeline/src/pipeline/mod.rs::1419] return_state: Some(
    DoneByRunning(
        FromProcessCompletedSuccessfully,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::1425] params: StepStateParams {
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
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "core.guid": String(
                            "5bfe06c36204b7f2",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "5230caa6f592bd1a",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "git.use_git": Boolean(
                            true,
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
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
                    ),
                },
                "file.list.recursive": XvcConfigValue {
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "file.track.force": XvcConfigValue {
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
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "5230caa6f592bd1a",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "quiet",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"5bfe06c36204b7f2/"
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
            random: 14682102138141470512,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                ".xvcignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    130,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1690882797,
                        tv_nsec: 50745056,
                    },
                ),
            },
            XvcPath(
                ".gitignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    107,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1690882797,
                        tv_nsec: 50929468,
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
                    tv_sec: 436307,
                    tv_nsec: 274511125,
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
        18285583588640195063,
    ),
    step: XvcStep {
        name: "hello",
    },
    step_command: XvcStepCommand {
        command: "echo hello",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    6211911852685607906,
                ): Begin(
                    FromInit,
                ),
                XvcEntity(
                    3,
                    18285583588640195063,
                ): Running(
                    FromWaitProcess,
                ),
            },
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
[TRACE][pipeline/src/pipeline/mod.rs::818] step.name: "hello"
[TRACE][pipeline/src/pipeline/mod.rs::819] &r_next_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::821] &step_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::700] &step_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::557] "Before state updater": "Before state updater"
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
[WARN][walker/src/error.rs::91] Crossbeam Send Error for Type: "Some(/n    Create {/n        path: /"[CWD]/.xvc/store/xvc-pipeline-run-dir-store/",/n        metadata: Metadata {/n            file_type: FileType(/n                FileType {/n                    mode: 16877,/n                },/n            ),/n            is_dir: true,/n            is_file: false,/n            permissions: Permissions(/n                FilePermissions {/n                    mode: 16877,/n                },/n            ),/n            modified: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558110736,/n                },/n            ),/n            accessed: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558110736,/n                },/n            ),/n            created: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558110736,/n                },/n            ),/n            ../n        },/n    },/n)" "sending on a disconnected channel"
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
[WARN][walker/src/error.rs::91] Crossbeam Send Error for Type: "Some(/n    Create {/n        path: /"[CWD]/.xvc/store/xvc-path-store/",/n        metadata: Metadata {/n            file_type: FileType(/n                FileType {/n                    mode: 16877,/n                },/n            ),/n            is_dir: true,/n            is_file: false,/n            permissions: Permissions(/n                FilePermissions {/n                    mode: 16877,/n                },/n            ),/n            modified: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558582767,/n                },/n            ),/n            accessed: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558582767,/n                },/n            ),/n            created: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558582767,/n                },/n            ),/n            ../n        },/n    },/n)" "sending on a disconnected channel"
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
[WARN][walker/src/error.rs::91] Crossbeam Send Error for Type: "Some(/n    Create {/n        path: /"[CWD]/.xvc/store/xvc-path-xvc-dependency-r1n-store/",/n        metadata: Metadata {/n            file_type: FileType(/n                FileType {/n                    mode: 16877,/n                },/n            ),/n            is_dir: true,/n            is_file: false,/n            permissions: Permissions(/n                FilePermissions {/n                    mode: 16877,/n                },/n            ),/n            modified: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558635724,/n                },/n            ),/n            accessed: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558635724,/n                },/n            ),/n            created: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558635724,/n                },/n            ),/n            ../n        },/n    },/n)" "sending on a disconnected channel"
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
[WARN][walker/src/error.rs::91] Crossbeam Send Error for Type: "Some(/n    Create {/n        path: /"[CWD]/.xvc/store/xvc-metadata-store/",/n        metadata: Metadata {/n            file_type: FileType(/n                FileType {/n                    mode: 16877,/n                },/n            ),/n            is_dir: true,/n            is_file: false,/n            permissions: Permissions(/n                FilePermissions {/n                    mode: 16877,/n                },/n            ),/n            modified: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558704389,/n                },/n            ),/n            accessed: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558704389,/n                },/n            ),/n            created: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558704389,/n                },/n            ),/n            ../n        },/n    },/n)" "sending on a disconnected channel"
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
[WARN][walker/src/error.rs::91] Crossbeam Send Error for Type: "Some(/n    Create {/n        path: /"[CWD]/.xvc/store/xvc-digests-store/",/n        metadata: Metadata {/n            file_type: FileType(/n                FileType {/n                    mode: 16877,/n                },/n            ),/n            is_dir: true,/n            is_file: false,/n            permissions: Permissions(/n                FilePermissions {/n                    mode: 16877,/n                },/n            ),/n            modified: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558744930,/n                },/n            ),/n            accessed: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558744930,/n                },/n            ),/n            created: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558744930,/n                },/n            ),/n            ../n        },/n    },/n)" "sending on a disconnected channel"
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
[WARN][walker/src/error.rs::91] Crossbeam Send Error for Type: "Some(/n    Create {/n        path: /"[CWD]/.xvc/store/text-or-binary-store/",/n        metadata: Metadata {/n            file_type: FileType(/n                FileType {/n                    mode: 16877,/n                },/n            ),/n            is_dir: true,/n            is_file: false,/n            permissions: Permissions(/n                FilePermissions {/n                    mode: 16877,/n                },/n            ),/n            modified: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558783346,/n                },/n            ),/n            accessed: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558783346,/n                },/n            ),/n            created: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 558783346,/n                },/n            ),/n            ../n        },/n    },/n)" "sending on a disconnected channel"
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
[WARN][walker/src/error.rs::91] Crossbeam Send Error for Type: "Some(/n    Create {/n        path: /"[CWD]/.xvc/store/xvc-digests-xvc-step-r1n-store/",/n        metadata: Metadata {/n            file_type: FileType(/n                FileType {/n                    mode: 16877,/n                },/n            ),/n            is_dir: true,/n            is_file: false,/n            permissions: Permissions(/n                FilePermissions {/n                    mode: 16877,/n                },/n            ),/n            modified: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 564056558,/n                },/n            ),/n            accessed: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 564056558,/n                },/n            ),/n            created: Ok(/n                SystemTime {/n                    tv_sec: 1690882797,/n                    tv_nsec: 564056558,/n                },/n            ),/n            ../n        },/n    },/n)" "sending on a disconnected channel"
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] select: Select { .. }

```
