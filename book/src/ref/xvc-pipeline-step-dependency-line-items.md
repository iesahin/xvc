# Line Item Dependencies

You can make your steps to depend on lines of text files. The lines are defined by starting and ending indices.

When the text in those lines change, the step is invalidated.

Unlike line dependencies, this dependency type keeps track of the lines in the
file. You can use `${XVC_LINE_ALL_ITEMS}`, `${XVC_LINE_ADDED_ITEMS}`, and
`${XVC_LINE_REMOVED_ITEMS}` environment variables in the command. Please be
aware that for large set of lines, this dependency can take up considerable
space to keep track of all lines and if you don't need to keep track of changed
lines, you can use `--lines` dependency.

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

We'll use a sample CSV file in this example:

```console
$ cat people.csv
"Name",     "Sex", "Age", "Height (in)", "Weight (lbs)"
"Alex",       "M",   41,       74,      170
"Bert",       "M",   42,       68,      166
"Carl",       "M",   32,       70,      155
"Dave",       "M",   39,       72,      167
"Elly",       "F",   30,       66,      124
"Fran",       "F",   33,       66,      115
"Gwen",       "F",   26,       64,      121
"Hank",       "M",   30,       71,      158
"Ivan",       "M",   53,       72,      175
"Jake",       "M",   32,       69,      143
"Kate",       "F",   47,       69,      139
"Luke",       "M",   34,       72,      163
"Myra",       "F",   23,       62,       98
"Neil",       "M",   36,       75,      160
"Omar",       "M",   38,       70,      145
"Page",       "F",   31,       67,      135
"Quin",       "M",   29,       71,      176
"Ruth",       "F",   28,       65,      131


```

Let's a step to show the first 10 lines of the file:

```console
$ xvc pipeline step new --step-name print-top-10 --command 'echo "Added Lines:\n ${XVC_LINE_ADDED_ITEMS}\nRemoved Lines:\n${XVC_LINE_REMOVED_ITEMS}"'

```

The command is run only when those lines change.

```console
$ xvc pipeline step dependency --step-name print-top-10 --line-items 'people.csv::1-10'

```

When you run the pipeline initially, the step is run.

```console
$ xvc pipeline run
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
    "core.quiet": Boolean(
        false,
    ),
    "core.verbosity": String(
        "quiet",
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
                "git.use_git": Boolean(
                    true,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "core.guid": String(
                    "be3578434f8f3c3b",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "core.guid": String(
                    "4f787cb5dfdb97c9",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "git.auto_stage": Boolean(
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
                "core.quiet": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "quiet",
                ),
            },
        },
    ],
    the_config: {
        "git.auto_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
            ),
        },
        "file.carry-in.no_parallel": XvcConfigValue {
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
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
            ),
        },
        "core.quiet": XvcConfigValue {
            source: CommandLine,
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
        "pipeline.current_pipeline": XvcConfigValue {
            source: Project,
            value: String(
                "default",
            ),
        },
        "file.recheck.method": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
            ),
        },
        "git.auto_stage": XvcConfigValue {
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
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "4f787cb5dfdb97c9",
            ),
        },
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
            ),
        },
        "file.track.no_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "file.carry-in.force": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "file.track.text_or_binary": XvcConfigValue {
            source: Project,
            value: String(
                "auto",
            ),
        },
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
            ),
        },
        "file.list.no_summary": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "pipeline.default": XvcConfigValue {
            source: Project,
            value: String(
                "default",
            ),
        },
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"be3578434f8f3c3b/"
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
    "[CWD]/.xvc/ec/1692720913969101",
    "[CWD]/.xvc/ec/1692720913971658",
    "[CWD]/.xvc/ec/1692720914086829",
    "[CWD]/.xvc/ec/1692720914172863",
]
[TRACE][pipeline/src/lib.rs::358] name: Some(
    "default",
)
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/people.csv"
[TRACE][core/src/types/xvcpath.rs::88] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::89] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
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
    paths: 0x0000600002aec2d0,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x00006000015e81f0,
    runloop: Some(
        (
            0x00006000018e8100,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::305] pipeline_len: 1
[TRACE][pipeline/src/pipeline/mod.rs::331] &dependency_graph: {
    XvcEntity(
        2,
        14670673438873334973,
    ): [],
}
[TRACE][pipeline/src/pipeline/mod.rs::343] &dependency_graph: {
    XvcEntity(
        2,
        14670673438873334973,
    ): [],
}
[INFO][pipeline/src/pipeline/mod.rs::347] Pipeline Graph:
digraph {
    0 [ label = "(2, 14670673438873334973)" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::424] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                14670673438873334973,
            ): Begin(
                FromInit,
            ),
        },
    },
    poisoned: false,
    ..
}
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
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::550] &step_thread_store: HStore {
    map: {
        XvcEntity(
            2,
            14670673438873334973,
        ): ScopedJoinHandle { .. },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::554] (step_e, &jh): (
    XvcEntity(
        2,
        14670673438873334973,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::670] params.recorded_dependencies: R1NStore {
    parents: XvcStore {
        map: {
            XvcEntity(
                2,
                14670673438873334973,
            ): XvcStep {
                name: "print-top-10",
            },
        },
        entity_index: {
            XvcStep {
                name: "print-top-10",
            }: [
                XvcEntity(
                    2,
                    14670673438873334973,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        2,
                        14670673438873334973,
                    ),
                    value: XvcStep {
                        name: "print-top-10",
                    },
                },
                Add {
                    entity: XvcEntity(
                        2,
                        14670673438873334973,
                    ),
                    value: XvcStep {
                        name: "print-top-10",
                    },
                },
            ],
        ),
        current: EventLog(
            [],
        ),
    },
    children: XvcStore {
        map: {
            XvcEntity(
                3,
                4719233081309553267,
            ): LineItems(
                LineItemsDep {
                    path: XvcPath(
                        "people.csv",
                    ),
                    begin: 1,
                    end: 10,
                    xvc_metadata: None,
                    lines: [],
                },
            ),
        },
        entity_index: {
            LineItems(
                LineItemsDep {
                    path: XvcPath(
                        "people.csv",
                    ),
                    begin: 1,
                    end: 10,
                    xvc_metadata: None,
                    lines: [],
                },
            ): [
                XvcEntity(
                    3,
                    4719233081309553267,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        4719233081309553267,
                    ),
                    value: LineItems(
                        LineItemsDep {
                            path: XvcPath(
                                "people.csv",
                            ),
                            begin: 1,
                            end: 10,
                            xvc_metadata: None,
                            lines: [],
                        },
                    ),
                },
            ],
        ),
        current: EventLog(
            [],
        ),
    },
    child_parents: XvcStore {
        map: {
            XvcEntity(
                3,
                4719233081309553267,
            ): ChildEntity(
                XvcEntity(
                    2,
                    14670673438873334973,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
        },
        entity_index: {
            ChildEntity(
                XvcEntity(
                    2,
                    14670673438873334973,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    3,
                    4719233081309553267,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        4719233081309553267,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            14670673438873334973,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
            ],
        ),
        current: EventLog(
            [],
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::671] step_e: XvcEntity(
    2,
    14670673438873334973,
)
[TRACE][pipeline/src/pipeline/mod.rs::612] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::672] dependency_steps(step_e, params.dependency_graph)?: {}
[TRACE][pipeline/src/pipeline/mod.rs::612] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/.xvc/store/xvc-pipeline-run-dir-store"
[TRACE][core/src/types/xvcpath.rs::88] current_dir: AbsolutePath(
    "[CWD]",
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
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::821] step.name: "print-top-10"
[TRACE][pipeline/src/pipeline/mod.rs::822] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::824] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::821] step.name: "print-top-10"
[TRACE][pipeline/src/pipeline/mod.rs::822] &r_next_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::824] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::821] step.name: "print-top-10"
[TRACE][pipeline/src/pipeline/mod.rs::822] &r_next_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::824] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::1109] parent_entity: XvcEntity(
    2,
    14670673438873334973,
)
[TRACE][pipeline/src/pipeline/mod.rs::1112] deps: HStore {
    map: {
        XvcEntity(
            3,
            4719233081309553267,
        ): LineItems(
            LineItemsDep {
                path: XvcPath(
                    "people.csv",
                ),
                begin: 1,
                end: 10,
                xvc_metadata: None,
                lines: [],
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/deps/compare.rs::421] &stored: LineItems(
    LineItemsDep {
        path: XvcPath(
            "people.csv",
        ),
        begin: 1,
        end: 10,
        xvc_metadata: None,
        lines: [],
    },
)
[TRACE][pipeline/src/pipeline/mod.rs::1129] step_dependency_diffs: HStore {
    map: {
        XvcEntity(
            3,
            4719233081309553267,
        ): RecordMissing {
            actual: LineItems(
                LineItemsDep {
                    path: XvcPath(
                        "people.csv",
                    ),
                    begin: 1,
                    end: 10,
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                849,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1692691126,
                                    tv_nsec: 43723936,
                                },
                            ),
                        },
                    ),
                    lines: [],
                },
            ),
        },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1135] diff: RecordMissing {
    actual: LineItems(
        LineItemsDep {
            path: XvcPath(
                "people.csv",
            ),
            begin: 1,
            end: 10,
            xvc_metadata: Some(
                XvcMetadata {
                    file_type: File,
                    size: Some(
                        849,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1692691126,
                            tv_nsec: 43723936,
                        },
                    ),
                },
            ),
            lines: [],
        },
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::1136] diff.changed(): true
[TRACE][pipeline/src/pipeline/mod.rs::1141] changed: true
[TRACE][pipeline/src/pipeline/mod.rs::821] step.name: "print-top-10"
[TRACE][pipeline/src/pipeline/mod.rs::822] &r_next_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::824] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::1166] deps: HStore {
    map: {
        XvcEntity(
            3,
            4719233081309553267,
        ): LineItems(
            LineItemsDep {
                path: XvcPath(
                    "people.csv",
                ),
                begin: 1,
                end: 10,
                xvc_metadata: None,
                lines: [],
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::821] step.name: "print-top-10"
[TRACE][pipeline/src/pipeline/mod.rs::822] &r_next_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::824] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::821] step.name: "print-top-10"
[TRACE][pipeline/src/pipeline/mod.rs::822] &r_next_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::824] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::1604] params: StepStateParams {
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
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "be3578434f8f3c3b",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "core.guid": String(
                            "4f787cb5dfdb97c9",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
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
                        "core.quiet": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "quiet",
                        ),
                    },
                },
            ],
            the_config: {
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "git.auto_stage": XvcConfigValue {
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "4f787cb5dfdb97c9",
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"be3578434f8f3c3b/"
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
            counter: 4,
            random: 1158293242932922722,
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
                        tv_sec: 1692720913,
                        tv_nsec: 969411180,
                    },
                ),
            },
            XvcPath(
                "people.csv",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    849,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1692691126,
                        tv_nsec: 43723936,
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
                        tv_sec: 1692720913,
                        tv_nsec: 969495640,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
        always: false,
        wait_running_dep_steps: true,
        ignore_broken_dep_steps: false,
        ignore_missing_dependencies: false,
        ignore_superficial_diffs: false,
        ignore_thorough_diffs: false,
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
                name: "print-top-10",
            },
            step_command: XvcStepCommand {
                command: "echo /"Added Lines:/
 ${XVC_LINE_ADDED_ITEMS}/
Removed Lines:/
${XVC_LINE_REMOVED_ITEMS}/"",
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
            map: {
                XvcEntity(
                    3,
                    4719233081309553267,
                ): RecordMissing {
                    actual: LineItems(
                        LineItemsDep {
                            path: XvcPath(
                                "people.csv",
                            ),
                            begin: 1,
                            end: 10,
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        849,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1692691126,
                                            tv_nsec: 43723936,
                                        },
                                    ),
                                },
                            ),
                            lines: [
                                "/"Alex/",       /"M/",   41,       74,      170",
                                "/"Bert/",       /"M/",   42,       68,      166",
                                "/"Carl/",       /"M/",   32,       70,      155",
                                "/"Dave/",       /"M/",   39,       72,      167",
                                "/"Elly/",       /"F/",   30,       66,      124",
                                "/"Fran/",       /"F/",   33,       66,      115",
                                "/"Gwen/",       /"F/",   26,       64,      121",
                                "/"Hank/",       /"M/",   30,       71,      158",
                                "/"Ivan/",       /"M/",   53,       72,      175",
                            ],
                        },
                    ),
                },
            },
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
        2,
        14670673438873334973,
    ),
    step: XvcStep {
        name: "print-top-10",
    },
    step_command: XvcStepCommand {
        command: "echo /"Added Lines:/
 ${XVC_LINE_ADDED_ITEMS}/
Removed Lines:/
${XVC_LINE_REMOVED_ITEMS}/"",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    14670673438873334973,
                ): WaitingDependencySteps(
                    FromRunConditional,
                ),
            },
        },
        poisoned: false,
        ..
    },
    step_timeout: 10000s,
    all_steps: HStore {
        map: {
            XvcEntity(
                2,
                14670673438873334973,
            ): XvcStep {
                name: "print-top-10",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    14670673438873334973,
                ): XvcStep {
                    name: "print-top-10",
                },
            },
            entity_index: {
                XvcStep {
                    name: "print-top-10",
                }: [
                    XvcEntity(
                        2,
                        14670673438873334973,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            14670673438873334973,
                        ),
                        value: XvcStep {
                            name: "print-top-10",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            14670673438873334973,
                        ),
                        value: XvcStep {
                            name: "print-top-10",
                        },
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        children: XvcStore {
            map: {
                XvcEntity(
                    3,
                    4719233081309553267,
                ): LineItems(
                    LineItemsDep {
                        path: XvcPath(
                            "people.csv",
                        ),
                        begin: 1,
                        end: 10,
                        xvc_metadata: None,
                        lines: [],
                    },
                ),
            },
            entity_index: {
                LineItems(
                    LineItemsDep {
                        path: XvcPath(
                            "people.csv",
                        ),
                        begin: 1,
                        end: 10,
                        xvc_metadata: None,
                        lines: [],
                    },
                ): [
                    XvcEntity(
                        3,
                        4719233081309553267,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            4719233081309553267,
                        ),
                        value: LineItems(
                            LineItemsDep {
                                path: XvcPath(
                                    "people.csv",
                                ),
                                begin: 1,
                                end: 10,
                                xvc_metadata: None,
                                lines: [],
                            },
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        child_parents: XvcStore {
            map: {
                XvcEntity(
                    3,
                    4719233081309553267,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        14670673438873334973,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        14670673438873334973,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        3,
                        4719233081309553267,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            4719233081309553267,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                14670673438873334973,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
    },
    step_dependencies: {},
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::821] step.name: "print-top-10"
[TRACE][pipeline/src/pipeline/mod.rs::822] &r_next_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::824] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/command.rs::74] self: CommandProcess {
    environment: {},
    step: XvcStep {
        name: "print-top-10",
    },
    step_command: XvcStepCommand {
        command: "echo /"Added Lines:/
 ${XVC_LINE_ADDED_ITEMS}/
Removed Lines:/
${XVC_LINE_REMOVED_ITEMS}/"",
    },
    birth: None,
    process: None,
    stdout_sender: Sender { .. },
    stderr_sender: Sender { .. },
    stdout_receiver: Receiver { .. },
    stderr_receiver: Receiver { .. },
}
[TRACE][pipeline/src/pipeline/command.rs::76] self: CommandProcess {
    environment: {
        "XVC_LINE_ADDED_ITEMS": "/"Alex/",       /"M/",   41,       74,      170
/"Bert/",       /"M/",   42,       68,      166
/"Carl/",       /"M/",   32,       70,      155
/"Dave/",       /"M/",   39,       72,      167
/"Elly/",       /"F/",   30,       66,      124
/"Fran/",       /"F/",   33,       66,      115
/"Gwen/",       /"F/",   26,       64,      121
/"Hank/",       /"M/",   30,       71,      158
/"Ivan/",       /"M/",   53,       72,      175",
    },
    step: XvcStep {
        name: "print-top-10",
    },
    step_command: XvcStepCommand {
        command: "echo /"Added Lines:/
 ${XVC_LINE_ADDED_ITEMS}/
Removed Lines:/
${XVC_LINE_REMOVED_ITEMS}/"",
    },
    birth: None,
    process: None,
    stdout_sender: Sender { .. },
    stderr_sender: Sender { .. },
    stdout_receiver: Receiver { .. },
    stderr_receiver: Receiver { .. },
}
[TRACE][pipeline/src/pipeline/command.rs::74] self: CommandProcess {
    environment: {
        "XVC_LINE_ADDED_ITEMS": "/"Alex/",       /"M/",   41,       74,      170
/"Bert/",       /"M/",   42,       68,      166
/"Carl/",       /"M/",   32,       70,      155
/"Dave/",       /"M/",   39,       72,      167
/"Elly/",       /"F/",   30,       66,      124
/"Fran/",       /"F/",   33,       66,      115
/"Gwen/",       /"F/",   26,       64,      121
/"Hank/",       /"M/",   30,       71,      158
/"Ivan/",       /"M/",   53,       72,      175",
    },
    step: XvcStep {
        name: "print-top-10",
    },
    step_command: XvcStepCommand {
        command: "echo /"Added Lines:/
 ${XVC_LINE_ADDED_ITEMS}/
Removed Lines:/
${XVC_LINE_REMOVED_ITEMS}/"",
    },
    birth: None,
    process: None,
    stdout_sender: Sender { .. },
    stderr_sender: Sender { .. },
    stdout_receiver: Receiver { .. },
    stderr_receiver: Receiver { .. },
}
[TRACE][pipeline/src/pipeline/command.rs::76] self: CommandProcess {
    environment: {
        "XVC_LINE_REMOVED_ITEMS": "",
        "XVC_LINE_ADDED_ITEMS": "/"Alex/",       /"M/",   41,       74,      170
/"Bert/",       /"M/",   42,       68,      166
/"Carl/",       /"M/",   32,       70,      155
/"Dave/",       /"M/",   39,       72,      167
/"Elly/",       /"F/",   30,       66,      124
/"Fran/",       /"F/",   33,       66,      115
/"Gwen/",       /"F/",   26,       64,      121
/"Hank/",       /"M/",   30,       71,      158
/"Ivan/",       /"M/",   53,       72,      175",
    },
    step: XvcStep {
        name: "print-top-10",
    },
    step_command: XvcStepCommand {
        command: "echo /"Added Lines:/
 ${XVC_LINE_ADDED_ITEMS}/
Removed Lines:/
${XVC_LINE_REMOVED_ITEMS}/"",
    },
    birth: None,
    process: None,
    stdout_sender: Sender { .. },
    stderr_sender: Sender { .. },
    stdout_receiver: Receiver { .. },
    stderr_receiver: Receiver { .. },
}
[TRACE][pipeline/src/pipeline/command.rs::74] self: CommandProcess {
    environment: {
        "XVC_LINE_REMOVED_ITEMS": "",
        "XVC_LINE_ADDED_ITEMS": "/"Alex/",       /"M/",   41,       74,      170
/"Bert/",       /"M/",   42,       68,      166
/"Carl/",       /"M/",   32,       70,      155
/"Dave/",       /"M/",   39,       72,      167
/"Elly/",       /"F/",   30,       66,      124
/"Fran/",       /"F/",   33,       66,      115
/"Gwen/",       /"F/",   26,       64,      121
/"Hank/",       /"M/",   30,       71,      158
/"Ivan/",       /"M/",   53,       72,      175",
    },
    step: XvcStep {
        name: "print-top-10",
    },
    step_command: XvcStepCommand {
        command: "echo /"Added Lines:/
 ${XVC_LINE_ADDED_ITEMS}/
Removed Lines:/
${XVC_LINE_REMOVED_ITEMS}/"",
    },
    birth: None,
    process: None,
    stdout_sender: Sender { .. },
    stderr_sender: Sender { .. },
    stdout_receiver: Receiver { .. },
    stderr_receiver: Receiver { .. },
}
[TRACE][pipeline/src/pipeline/command.rs::76] self: CommandProcess {
    environment: {
        "XVC_LINE_REMOVED_ITEMS": "",
        "XVC_LINE_ADDED_ITEMS": "/"Alex/",       /"M/",   41,       74,      170
/"Bert/",       /"M/",   42,       68,      166
/"Carl/",       /"M/",   32,       70,      155
/"Dave/",       /"M/",   39,       72,      167
/"Elly/",       /"F/",   30,       66,      124
/"Fran/",       /"F/",   33,       66,      115
/"Gwen/",       /"F/",   26,       64,      121
/"Hank/",       /"M/",   30,       71,      158
/"Ivan/",       /"M/",   53,       72,      175",
        "XVC_LINE_ALL_ITEMS": "/"Alex/",       /"M/",   41,       74,      170
/"Bert/",       /"M/",   42,       68,      166
/"Carl/",       /"M/",   32,       70,      155
/"Dave/",       /"M/",   39,       72,      167
/"Elly/",       /"F/",   30,       66,      124
/"Fran/",       /"F/",   33,       66,      115
/"Gwen/",       /"F/",   26,       64,      121
/"Hank/",       /"M/",   30,       71,      158
/"Ivan/",       /"M/",   53,       72,      175",
    },
    step: XvcStep {
        name: "print-top-10",
    },
    step_command: XvcStepCommand {
        command: "echo /"Added Lines:/
 ${XVC_LINE_ADDED_ITEMS}/
Removed Lines:/
${XVC_LINE_REMOVED_ITEMS}/"",
    },
    birth: None,
    process: None,
    stdout_sender: Sender { .. },
    stderr_sender: Sender { .. },
    stdout_receiver: Receiver { .. },
    stderr_receiver: Receiver { .. },
}
[TRACE][pipeline/src/pipeline/command.rs::81] self.environment: {
    "XVC_LINE_REMOVED_ITEMS": "",
    "XVC_LINE_ADDED_ITEMS": "/"Alex/",       /"M/",   41,       74,      170
/"Bert/",       /"M/",   42,       68,      166
/"Carl/",       /"M/",   32,       70,      155
/"Dave/",       /"M/",   39,       72,      167
/"Elly/",       /"F/",   30,       66,      124
/"Fran/",       /"F/",   33,       66,      115
/"Gwen/",       /"F/",   26,       64,      121
/"Hank/",       /"M/",   30,       71,      158
/"Ivan/",       /"M/",   53,       72,      175",
    "XVC_LINE_ALL_ITEMS": "/"Alex/",       /"M/",   41,       74,      170
/"Bert/",       /"M/",   42,       68,      166
/"Carl/",       /"M/",   32,       70,      155
/"Dave/",       /"M/",   39,       72,      167
/"Elly/",       /"F/",   30,       66,      124
/"Fran/",       /"F/",   33,       66,      115
/"Gwen/",       /"F/",   26,       64,      121
/"Hank/",       /"M/",   30,       71,      158
/"Ivan/",       /"M/",   53,       72,      175",
}
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::821] step.name: "print-top-10"
[TRACE][pipeline/src/pipeline/mod.rs::822] &r_next_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::824] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1493] params: StepStateParams {
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
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "be3578434f8f3c3b",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "core.guid": String(
                            "4f787cb5dfdb97c9",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
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
                        "core.quiet": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "quiet",
                        ),
                    },
                },
            ],
            the_config: {
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "git.auto_stage": XvcConfigValue {
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "4f787cb5dfdb97c9",
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"be3578434f8f3c3b/"
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
            counter: 4,
            random: 1158293242932922722,
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
                        tv_sec: 1692720913,
                        tv_nsec: 969411180,
                    },
                ),
            },
            XvcPath(
                "people.csv",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    849,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1692691126,
                        tv_nsec: 43723936,
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
                        tv_sec: 1692720913,
                        tv_nsec: 969495640,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
        always: false,
        wait_running_dep_steps: true,
        ignore_broken_dep_steps: false,
        ignore_missing_dependencies: false,
        ignore_superficial_diffs: false,
        ignore_thorough_diffs: false,
        ignore_missing_outputs: true,
    },
    pipeline_rundir: XvcPath(
        "",
    ),
    terminate_timeout_processes: true,
    algorithm: Blake3,
    command_process: RwLock {
        data: CommandProcess {
            environment: {
                "XVC_LINE_REMOVED_ITEMS": "",
                "XVC_LINE_ADDED_ITEMS": "/"Alex/",       /"M/",   41,       74,      170
/"Bert/",       /"M/",   42,       68,      166
/"Carl/",       /"M/",   32,       70,      155
/"Dave/",       /"M/",   39,       72,      167
/"Elly/",       /"F/",   30,       66,      124
/"Fran/",       /"F/",   33,       66,      115
/"Gwen/",       /"F/",   26,       64,      121
/"Hank/",       /"M/",   30,       71,      158
/"Ivan/",       /"M/",   53,       72,      175",
                "XVC_LINE_ALL_ITEMS": "/"Alex/",       /"M/",   41,       74,      170
/"Bert/",       /"M/",   42,       68,      166
/"Carl/",       /"M/",   32,       70,      155
/"Dave/",       /"M/",   39,       72,      167
/"Elly/",       /"F/",   30,       66,      124
/"Fran/",       /"F/",   33,       66,      115
/"Gwen/",       /"F/",   26,       64,      121
/"Hank/",       /"M/",   30,       71,      158
/"Ivan/",       /"M/",   53,       72,      175",
            },
            step: XvcStep {
                name: "print-top-10",
            },
            step_command: XvcStepCommand {
                command: "echo /"Added Lines:/
 ${XVC_LINE_ADDED_ITEMS}/
Removed Lines:/
${XVC_LINE_REMOVED_ITEMS}/"",
            },
            birth: Some(
                Instant {
                    tv_sec: 1390323,
                    tv_nsec: 627999708,
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
                        pid: 94560,
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
            map: {
                XvcEntity(
                    3,
                    4719233081309553267,
                ): RecordMissing {
                    actual: LineItems(
                        LineItemsDep {
                            path: XvcPath(
                                "people.csv",
                            ),
                            begin: 1,
                            end: 10,
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        849,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1692691126,
                                            tv_nsec: 43723936,
                                        },
                                    ),
                                },
                            ),
                            lines: [
                                "/"Alex/",       /"M/",   41,       74,      170",
                                "/"Bert/",       /"M/",   42,       68,      166",
                                "/"Carl/",       /"M/",   32,       70,      155",
                                "/"Dave/",       /"M/",   39,       72,      167",
                                "/"Elly/",       /"F/",   30,       66,      124",
                                "/"Fran/",       /"F/",   33,       66,      115",
                                "/"Gwen/",       /"F/",   26,       64,      121",
                                "/"Hank/",       /"M/",   30,       71,      158",
                                "/"Ivan/",       /"M/",   53,       72,      175",
                            ],
                        },
                    ),
                },
            },
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
        2,
        14670673438873334973,
    ),
    step: XvcStep {
        name: "print-top-10",
    },
    step_command: XvcStepCommand {
        command: "echo /"Added Lines:/
 ${XVC_LINE_ADDED_ITEMS}/
Removed Lines:/
${XVC_LINE_REMOVED_ITEMS}/"",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    14670673438873334973,
                ): Running(
                    FromWaitProcess,
                ),
            },
        },
        poisoned: false,
        ..
    },
    step_timeout: 10000s,
    all_steps: HStore {
        map: {
            XvcEntity(
                2,
                14670673438873334973,
            ): XvcStep {
                name: "print-top-10",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    14670673438873334973,
                ): XvcStep {
                    name: "print-top-10",
                },
            },
            entity_index: {
                XvcStep {
                    name: "print-top-10",
                }: [
                    XvcEntity(
                        2,
                        14670673438873334973,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            14670673438873334973,
                        ),
                        value: XvcStep {
                            name: "print-top-10",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            14670673438873334973,
                        ),
                        value: XvcStep {
                            name: "print-top-10",
                        },
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        children: XvcStore {
            map: {
                XvcEntity(
                    3,
                    4719233081309553267,
                ): LineItems(
                    LineItemsDep {
                        path: XvcPath(
                            "people.csv",
                        ),
                        begin: 1,
                        end: 10,
                        xvc_metadata: None,
                        lines: [],
                    },
                ),
            },
            entity_index: {
                LineItems(
                    LineItemsDep {
                        path: XvcPath(
                            "people.csv",
                        ),
                        begin: 1,
                        end: 10,
                        xvc_metadata: None,
                        lines: [],
                    },
                ): [
                    XvcEntity(
                        3,
                        4719233081309553267,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            4719233081309553267,
                        ),
                        value: LineItems(
                            LineItemsDep {
                                path: XvcPath(
                                    "people.csv",
                                ),
                                begin: 1,
                                end: 10,
                                xvc_metadata: None,
                                lines: [],
                            },
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        child_parents: XvcStore {
            map: {
                XvcEntity(
                    3,
                    4719233081309553267,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        14670673438873334973,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        14670673438873334973,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        3,
                        4719233081309553267,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            4719233081309553267,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                14670673438873334973,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
    },
    step_dependencies: {},
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1525] command_process: CommandProcess {
    environment: {
        "XVC_LINE_REMOVED_ITEMS": "",
        "XVC_LINE_ADDED_ITEMS": "/"Alex/",       /"M/",   41,       74,      170
/"Bert/",       /"M/",   42,       68,      166
/"Carl/",       /"M/",   32,       70,      155
/"Dave/",       /"M/",   39,       72,      167
/"Elly/",       /"F/",   30,       66,      124
/"Fran/",       /"F/",   33,       66,      115
/"Gwen/",       /"F/",   26,       64,      121
/"Hank/",       /"M/",   30,       71,      158
/"Ivan/",       /"M/",   53,       72,      175",
        "XVC_LINE_ALL_ITEMS": "/"Alex/",       /"M/",   41,       74,      170
/"Bert/",       /"M/",   42,       68,      166
/"Carl/",       /"M/",   32,       70,      155
/"Dave/",       /"M/",   39,       72,      167
/"Elly/",       /"F/",   30,       66,      124
/"Fran/",       /"F/",   33,       66,      115
/"Gwen/",       /"F/",   26,       64,      121
/"Hank/",       /"M/",   30,       71,      158
/"Ivan/",       /"M/",   53,       72,      175",
    },
    step: XvcStep {
        name: "print-top-10",
    },
    step_command: XvcStepCommand {
        command: "echo /"Added Lines:/
 ${XVC_LINE_ADDED_ITEMS}/
Removed Lines:/
${XVC_LINE_REMOVED_ITEMS}/"",
    },
    birth: Some(
        Instant {
            tv_sec: 1390323,
            tv_nsec: 627999708,
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
                pid: 94560,
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
[OUT] [print-top-10] Added Lines:
 "Alex",       "M",   41,       74,      170
"Bert",       "M",   42,       68,      166
"Carl",       "M",   32,       70,      155
"Dave",       "M",   39,       72,      167
"Elly",       "F",   30,       66,      124
"Fran",       "F",   33,       66,      115
"Gwen",       "F",   26,       64,      121
"Hank",       "M",   30,       71,      158
"Ivan",       "M",   53,       72,      175
Removed Lines:


[TRACE][pipeline/src/pipeline/mod.rs::1531] &process: Popen {
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
        pid: 94560,
        ext: (),
    },
    detached: true,
}
[TRACE][pipeline/src/pipeline/mod.rs::1578] return_state: Some(
    DoneByRunning(
        FromProcessCompletedSuccessfully,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::1584] params: StepStateParams {
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
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "be3578434f8f3c3b",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "core.guid": String(
                            "4f787cb5dfdb97c9",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
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
                        "core.quiet": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "quiet",
                        ),
                    },
                },
            ],
            the_config: {
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "git.auto_stage": XvcConfigValue {
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "4f787cb5dfdb97c9",
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"be3578434f8f3c3b/"
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
            counter: 4,
            random: 1158293242932922722,
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
                        tv_sec: 1692720913,
                        tv_nsec: 969411180,
                    },
                ),
            },
            XvcPath(
                "people.csv",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    849,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1692691126,
                        tv_nsec: 43723936,
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
                        tv_sec: 1692720913,
                        tv_nsec: 969495640,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
        always: false,
        wait_running_dep_steps: true,
        ignore_broken_dep_steps: false,
        ignore_missing_dependencies: false,
        ignore_superficial_diffs: false,
        ignore_thorough_diffs: false,
        ignore_missing_outputs: true,
    },
    pipeline_rundir: XvcPath(
        "",
    ),
    terminate_timeout_processes: true,
    algorithm: Blake3,
    command_process: RwLock {
        data: CommandProcess {
            environment: {
                "XVC_LINE_REMOVED_ITEMS": "",
                "XVC_LINE_ADDED_ITEMS": "/"Alex/",       /"M/",   41,       74,      170
/"Bert/",       /"M/",   42,       68,      166
/"Carl/",       /"M/",   32,       70,      155
/"Dave/",       /"M/",   39,       72,      167
/"Elly/",       /"F/",   30,       66,      124
/"Fran/",       /"F/",   33,       66,      115
/"Gwen/",       /"F/",   26,       64,      121
/"Hank/",       /"M/",   30,       71,      158
/"Ivan/",       /"M/",   53,       72,      175",
                "XVC_LINE_ALL_ITEMS": "/"Alex/",       /"M/",   41,       74,      170
/"Bert/",       /"M/",   42,       68,      166
/"Carl/",       /"M/",   32,       70,      155
/"Dave/",       /"M/",   39,       72,      167
/"Elly/",       /"F/",   30,       66,      124
/"Fran/",       /"F/",   33,       66,      115
/"Gwen/",       /"F/",   26,       64,      121
/"Hank/",       /"M/",   30,       71,      158
/"Ivan/",       /"M/",   53,       72,      175",
            },
            step: XvcStep {
                name: "print-top-10",
            },
            step_command: XvcStepCommand {
                command: "echo /"Added Lines:/
 ${XVC_LINE_ADDED_ITEMS}/
Removed Lines:/
${XVC_LINE_REMOVED_ITEMS}/"",
            },
            birth: Some(
                Instant {
                    tv_sec: 1390323,
                    tv_nsec: 627999708,
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
            map: {
                XvcEntity(
                    3,
                    4719233081309553267,
                ): RecordMissing {
                    actual: LineItems(
                        LineItemsDep {
                            path: XvcPath(
                                "people.csv",
                            ),
                            begin: 1,
                            end: 10,
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        849,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1692691126,
                                            tv_nsec: 43723936,
                                        },
                                    ),
                                },
                            ),
                            lines: [
                                "/"Alex/",       /"M/",   41,       74,      170",
                                "/"Bert/",       /"M/",   42,       68,      166",
                                "/"Carl/",       /"M/",   32,       70,      155",
                                "/"Dave/",       /"M/",   39,       72,      167",
                                "/"Elly/",       /"F/",   30,       66,      124",
                                "/"Fran/",       /"F/",   33,       66,      115",
                                "/"Gwen/",       /"F/",   26,       64,      121",
                                "/"Hank/",       /"M/",   30,       71,      158",
                                "/"Ivan/",       /"M/",   53,       72,      175",
                            ],
                        },
                    ),
                },
            },
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
        2,
        14670673438873334973,
    ),
    step: XvcStep {
        name: "print-top-10",
    },
    step_command: XvcStepCommand {
        command: "echo /"Added Lines:/
 ${XVC_LINE_ADDED_ITEMS}/
Removed Lines:/
${XVC_LINE_REMOVED_ITEMS}/"",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    14670673438873334973,
                ): Running(
                    FromWaitProcess,
                ),
            },
        },
        poisoned: false,
        ..
    },
    step_timeout: 10000s,
    all_steps: HStore {
        map: {
            XvcEntity(
                2,
                14670673438873334973,
            ): XvcStep {
                name: "print-top-10",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    14670673438873334973,
                ): XvcStep {
                    name: "print-top-10",
                },
            },
            entity_index: {
                XvcStep {
                    name: "print-top-10",
                }: [
                    XvcEntity(
                        2,
                        14670673438873334973,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            14670673438873334973,
                        ),
                        value: XvcStep {
                            name: "print-top-10",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            14670673438873334973,
                        ),
                        value: XvcStep {
                            name: "print-top-10",
                        },
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        children: XvcStore {
            map: {
                XvcEntity(
                    3,
                    4719233081309553267,
                ): LineItems(
                    LineItemsDep {
                        path: XvcPath(
                            "people.csv",
                        ),
                        begin: 1,
                        end: 10,
                        xvc_metadata: None,
                        lines: [],
                    },
                ),
            },
            entity_index: {
                LineItems(
                    LineItemsDep {
                        path: XvcPath(
                            "people.csv",
                        ),
                        begin: 1,
                        end: 10,
                        xvc_metadata: None,
                        lines: [],
                    },
                ): [
                    XvcEntity(
                        3,
                        4719233081309553267,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            4719233081309553267,
                        ),
                        value: LineItems(
                            LineItemsDep {
                                path: XvcPath(
                                    "people.csv",
                                ),
                                begin: 1,
                                end: 10,
                                xvc_metadata: None,
                                lines: [],
                            },
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        child_parents: XvcStore {
            map: {
                XvcEntity(
                    3,
                    4719233081309553267,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        14670673438873334973,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        14670673438873334973,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        3,
                        4719233081309553267,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            4719233081309553267,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                14670673438873334973,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
    },
    step_dependencies: {},
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::821] step.name: "print-top-10"
[TRACE][pipeline/src/pipeline/mod.rs::822] &r_next_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::824] &step_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::561] "Before state updater": "Before state updater"
[TRACE][pipeline/src/pipeline/mod.rs::572] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                14670673438873334973,
            ): DoneByRunning(
                FromProcessCompletedSuccessfully,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::579] done_successfully: Ok(
    true,
)
[TRACE][pipeline/src/pipeline/mod.rs::594] output_diffs: HStore {
    map: {},
}
[TRACE][pipeline/src/pipeline/mod.rs::595] store: XvcStore {
    map: {},
    entity_index: {},
    previous: EventLog(
        [],
    ),
    current: EventLog(
        [],
    ),
}
[TRACE][lib/src/cli/mod.rs::381] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::384] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc --debug pipeline run"
[TRACE][lib/src/cli/mod.rs::433] args: [
    "-C",
    "[CWD]",
    "diff",
    "--name-only",
    "--cached",
]
[TRACE][lib/src/cli/mod.rs::463] git_diff_staged_out: ""
[TRACE][lib/src/cli/mod.rs::433] args: [
    "-C",
    "[CWD]",
    "add",
    "--verbose",
    "[CWD]/.xvc",
    "*.gitignore",
    "*.xvcignore",
]
[TRACE][lib/src/cli/mod.rs::582] git_add_output: "add '.xvc/store/xvc-dependency-store/1692720916318016.json'
"
[TRACE][lib/src/cli/mod.rs::433] args: [
    "-C",
    "[CWD]",
    "commit",
    "-m",
    "Xvc auto-commit after /'/Users/iex/github.com/iesahin/xvc/target/debug/xvc --debug pipeline run/'",
]

``````

When you run the pipeline again, the step is not run because the specified lines didn't change.

```console
$ xvc pipeline run

``````

When you change a line from the file, the step is invalidated.

```console
$ perl -i -pe 's/Hank/Ferzan/g' people.csv

```

Now, when you run the pipeline, it will print the changed line, with its new and old versions.

```
$ xvc pipeline run
[OUT] [print-top-10] Added Lines:
 "Ferzan",       "M",   30,       71,      158
Removed Lines:
"Hank",       "M",   30,       71,      158


```

