### Generic Command Dependencies

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

You can use the output of a shell command as a dependency to a step.
When the command is run, the output hash is saved to compare and the step is invalidated when the output of the command changed.

You can use this for any command that outputs a string.

```console
$ xvc pipeline step new --step-name morning-message --command "echo 'Good Morning!'"

$ xvc  pipeline step dependency --step-name morning-message --generic 'date +%F'

```

The step is invalidated when the date changes and the step is run again.

```console
$ xvc pipeline run
[OUT] [morning-message] Good Morning!


```

The step won't run until tomorrow, when `date +%F` changes.

```console
$ xvc pipeline run

```

You can mimic all kinds of pipeline behavior with this generic dependency.

For example, if you want to run a command when directory contents change, you can depend on the output of `ls -lR`:

```console
$ xvc pipeline step new --step-name directory-contents --command "echo 'Files changed'"
$ xvc pipeline step dependency --step-name directory-contents --generic 'ls -lR'
$ xvc pipeline run
[OUT] [directory-contents] Files changed


```

When you add a file to the directory, the step is invalidated and run again:

```console
$ xvc pipeline run
$ xvc-test-helper generate-random-file new-file.txt
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
                "file.track.force": Boolean(
                    false,
                ),
                "git.command": String(
                    "git",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
                "core.guid": String(
                    "1f64aa5454722741",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "git.command": String(
                    "git",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "core.guid": String(
                    "72cbecd96de5ac2a",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "pipeline.current_pipeline": String(
                    "default",
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
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "quiet",
            ),
        },
        "git.auto_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "72cbecd96de5ac2a",
            ),
        },
        "file.track.force": XvcConfigValue {
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
        "file.list.no_summary": XvcConfigValue {
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
        "pipeline.current_pipeline": XvcConfigValue {
            source: Project,
            value: String(
                "default",
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
        "file.track.no_commit": XvcConfigValue {
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
        "file.track.no_parallel": XvcConfigValue {
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
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
            ),
        },
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
            ),
        },
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
            ),
        },
        "git.auto_stage": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
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
guid = /"1f64aa5454722741/"
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
    "[CWD]/.xvc/ec/1691846344870902",
    "[CWD]/.xvc/ec/1691846344873741",
    "[CWD]/.xvc/ec/1691846344947267",
    "[CWD]/.xvc/ec/1691846345021370",
    "[CWD]/.xvc/ec/1691846347706030",
    "[CWD]/.xvc/ec/1691846347776236",
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
    paths: 0x0000600002258000,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000600001d54150,
    runloop: Some(
        (
            0x0000600001058100,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::305] pipeline_len: 2
[TRACE][pipeline/src/pipeline/mod.rs::331] &dependency_graph: {
    XvcEntity(
        4,
        11031608649320549610,
    ): [],
    XvcEntity(
        2,
        14428876823979097321,
    ): [],
}
[TRACE][pipeline/src/pipeline/mod.rs::343] &dependency_graph: {
    XvcEntity(
        4,
        11031608649320549610,
    ): [],
    XvcEntity(
        2,
        14428876823979097321,
    ): [],
}
[INFO][pipeline/src/pipeline/mod.rs::347] Pipeline Graph:
digraph {
    0 [ label = "(4, 11031608649320549610)" ]
    1 [ label = "(2, 14428876823979097321)" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::424] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                14428876823979097321,
            ): Begin(
                FromInit,
            ),
            XvcEntity(
                4,
                11031608649320549610,
            ): Begin(
                FromInit,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::550] &step_thread_store: HStore {
    map: {
        XvcEntity(
            2,
            14428876823979097321,
        ): ScopedJoinHandle { .. },
        XvcEntity(
            4,
            11031608649320549610,
        ): ScopedJoinHandle { .. },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::554] (step_e, &jh): (
    XvcEntity(
        2,
        14428876823979097321,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::670] params.recorded_dependencies: R1NStore {
    parents: XvcStore {
        map: {
            XvcEntity(
                2,
                14428876823979097321,
            ): XvcStep {
                name: "morning-message",
            },
            XvcEntity(
                4,
                11031608649320549610,
            ): XvcStep {
                name: "directory-contents",
            },
        },
        entity_index: {
            XvcStep {
                name: "directory-contents",
            }: [
                XvcEntity(
                    4,
                    11031608649320549610,
                ),
            ],
            XvcStep {
                name: "morning-message",
            }: [
                XvcEntity(
                    2,
                    14428876823979097321,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        2,
                        14428876823979097321,
                    ),
                    value: XvcStep {
                        name: "morning-message",
                    },
                },
                Add {
                    entity: XvcEntity(
                        2,
                        14428876823979097321,
                    ),
                    value: XvcStep {
                        name: "morning-message",
                    },
                },
                Add {
                    entity: XvcEntity(
                        4,
                        11031608649320549610,
                    ),
                    value: XvcStep {
                        name: "directory-contents",
                    },
                },
                Add {
                    entity: XvcEntity(
                        4,
                        11031608649320549610,
                    ),
                    value: XvcStep {
                        name: "directory-contents",
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
                9554303650483606518,
            ): Generic(
                GenericDep {
                    generic_command: "date +%F",
                    output_digest: Some(
                        StdoutDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    50,
                                    73,
                                    75,
                                    188,
                                    90,
                                    228,
                                    145,
                                    158,
                                    166,
                                    111,
                                    43,
                                    244,
                                    44,
                                    137,
                                    182,
                                    122,
                                    255,
                                    80,
                                    237,
                                    43,
                                    182,
                                    75,
                                    3,
                                    239,
                                    175,
                                    227,
                                    26,
                                    66,
                                    250,
                                    189,
                                    163,
                                    117,
                                ],
                            },
                        ),
                    ),
                },
            ),
            XvcEntity(
                5,
                18084731218162541931,
            ): Generic(
                GenericDep {
                    generic_command: "ls -lR",
                    output_digest: Some(
                        StdoutDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    113,
                                    31,
                                    8,
                                    96,
                                    119,
                                    69,
                                    101,
                                    63,
                                    176,
                                    181,
                                    172,
                                    2,
                                    130,
                                    247,
                                    47,
                                    77,
                                    102,
                                    42,
                                    152,
                                    148,
                                    86,
                                    113,
                                    240,
                                    14,
                                    93,
                                    176,
                                    37,
                                    101,
                                    152,
                                    96,
                                    202,
                                    223,
                                ],
                            },
                        ),
                    ),
                },
            ),
        },
        entity_index: {
            Generic(
                GenericDep {
                    generic_command: "date +%F",
                    output_digest: Some(
                        StdoutDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    50,
                                    73,
                                    75,
                                    188,
                                    90,
                                    228,
                                    145,
                                    158,
                                    166,
                                    111,
                                    43,
                                    244,
                                    44,
                                    137,
                                    182,
                                    122,
                                    255,
                                    80,
                                    237,
                                    43,
                                    182,
                                    75,
                                    3,
                                    239,
                                    175,
                                    227,
                                    26,
                                    66,
                                    250,
                                    189,
                                    163,
                                    117,
                                ],
                            },
                        ),
                    ),
                },
            ): [
                XvcEntity(
                    3,
                    9554303650483606518,
                ),
            ],
            Generic(
                GenericDep {
                    generic_command: "ls -lR",
                    output_digest: Some(
                        StdoutDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    113,
                                    31,
                                    8,
                                    96,
                                    119,
                                    69,
                                    101,
                                    63,
                                    176,
                                    181,
                                    172,
                                    2,
                                    130,
                                    247,
                                    47,
                                    77,
                                    102,
                                    42,
                                    152,
                                    148,
                                    86,
                                    113,
                                    240,
                                    14,
                                    93,
                                    176,
                                    37,
                                    101,
                                    152,
                                    96,
                                    202,
                                    223,
                                ],
                            },
                        ),
                    ),
                },
            ): [
                XvcEntity(
                    5,
                    18084731218162541931,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        9554303650483606518,
                    ),
                    value: Generic(
                        GenericDep {
                            generic_command: "date +%F",
                            output_digest: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        3,
                        9554303650483606518,
                    ),
                    value: Generic(
                        GenericDep {
                            generic_command: "date +%F",
                            output_digest: Some(
                                StdoutDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            50,
                                            73,
                                            75,
                                            188,
                                            90,
                                            228,
                                            145,
                                            158,
                                            166,
                                            111,
                                            43,
                                            244,
                                            44,
                                            137,
                                            182,
                                            122,
                                            255,
                                            80,
                                            237,
                                            43,
                                            182,
                                            75,
                                            3,
                                            239,
                                            175,
                                            227,
                                            26,
                                            66,
                                            250,
                                            189,
                                            163,
                                            117,
                                        ],
                                    },
                                ),
                            ),
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        18084731218162541931,
                    ),
                    value: Generic(
                        GenericDep {
                            generic_command: "ls -lR",
                            output_digest: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        18084731218162541931,
                    ),
                    value: Generic(
                        GenericDep {
                            generic_command: "ls -lR",
                            output_digest: Some(
                                StdoutDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            113,
                                            31,
                                            8,
                                            96,
                                            119,
                                            69,
                                            101,
                                            63,
                                            176,
                                            181,
                                            172,
                                            2,
                                            130,
                                            247,
                                            47,
                                            77,
                                            102,
                                            42,
                                            152,
                                            148,
                                            86,
                                            113,
                                            240,
                                            14,
                                            93,
                                            176,
                                            37,
                                            101,
                                            152,
                                            96,
                                            202,
                                            223,
                                        ],
                                    },
                                ),
                            ),
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
                9554303650483606518,
            ): ChildEntity(
                XvcEntity(
                    2,
                    14428876823979097321,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
            XvcEntity(
                5,
                18084731218162541931,
            ): ChildEntity(
                XvcEntity(
                    4,
                    11031608649320549610,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
        },
        entity_index: {
            ChildEntity(
                XvcEntity(
                    2,
                    14428876823979097321,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    3,
                    9554303650483606518,
                ),
            ],
            ChildEntity(
                XvcEntity(
                    4,
                    11031608649320549610,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    5,
                    18084731218162541931,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        9554303650483606518,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            14428876823979097321,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        18084731218162541931,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            4,
                            11031608649320549610,
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
    14428876823979097321,
)
[TRACE][pipeline/src/pipeline/mod.rs::670] params.recorded_dependencies: R1NStore {
    parents: XvcStore {
        map: {
            XvcEntity(
                2,
                14428876823979097321,
            ): XvcStep {
                name: "morning-message",
            },
            XvcEntity(
                4,
                11031608649320549610,
            ): XvcStep {
                name: "directory-contents",
            },
        },
        entity_index: {
            XvcStep {
                name: "directory-contents",
            }: [
                XvcEntity(
                    4,
                    11031608649320549610,
                ),
            ],
            XvcStep {
                name: "morning-message",
            }: [
                XvcEntity(
                    2,
                    14428876823979097321,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        2,
                        14428876823979097321,
                    ),
                    value: XvcStep {
                        name: "morning-message",
                    },
                },
                Add {
                    entity: XvcEntity(
                        2,
                        14428876823979097321,
                    ),
                    value: XvcStep {
                        name: "morning-message",
                    },
                },
                Add {
                    entity: XvcEntity(
                        4,
                        11031608649320549610,
                    ),
                    value: XvcStep {
                        name: "directory-contents",
                    },
                },
                Add {
                    entity: XvcEntity(
                        4,
                        11031608649320549610,
                    ),
                    value: XvcStep {
                        name: "directory-contents",
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
                9554303650483606518,
            ): Generic(
                GenericDep {
                    generic_command: "date +%F",
                    output_digest: Some(
                        StdoutDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    50,
                                    73,
                                    75,
                                    188,
                                    90,
                                    228,
                                    145,
                                    158,
                                    166,
                                    111,
                                    43,
                                    244,
                                    44,
                                    137,
                                    182,
                                    122,
                                    255,
                                    80,
                                    237,
                                    43,
                                    182,
                                    75,
                                    3,
                                    239,
                                    175,
                                    227,
                                    26,
                                    66,
                                    250,
                                    189,
                                    163,
                                    117,
                                ],
                            },
                        ),
                    ),
                },
            ),
            XvcEntity(
                5,
                18084731218162541931,
            ): Generic(
                GenericDep {
                    generic_command: "ls -lR",
                    output_digest: Some(
                        StdoutDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    113,
                                    31,
                                    8,
                                    96,
                                    119,
                                    69,
                                    101,
                                    63,
                                    176,
                                    181,
                                    172,
                                    2,
                                    130,
                                    247,
                                    47,
                                    77,
                                    102,
                                    42,
                                    152,
                                    148,
                                    86,
                                    113,
                                    240,
                                    14,
                                    93,
                                    176,
                                    37,
                                    101,
                                    152,
                                    96,
                                    202,
                                    223,
                                ],
                            },
                        ),
                    ),
                },
            ),
        },
        entity_index: {
            Generic(
                GenericDep {
                    generic_command: "date +%F",
                    output_digest: Some(
                        StdoutDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    50,
                                    73,
                                    75,
                                    188,
                                    90,
                                    228,
                                    145,
                                    158,
                                    166,
                                    111,
                                    43,
                                    244,
                                    44,
                                    137,
                                    182,
                                    122,
                                    255,
                                    80,
                                    237,
                                    43,
                                    182,
                                    75,
                                    3,
                                    239,
                                    175,
                                    227,
                                    26,
                                    66,
                                    250,
                                    189,
                                    163,
                                    117,
                                ],
                            },
                        ),
                    ),
                },
            ): [
                XvcEntity(
                    3,
                    9554303650483606518,
                ),
            ],
            Generic(
                GenericDep {
                    generic_command: "ls -lR",
                    output_digest: Some(
                        StdoutDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    113,
                                    31,
                                    8,
                                    96,
                                    119,
                                    69,
                                    101,
                                    63,
                                    176,
                                    181,
                                    172,
                                    2,
                                    130,
                                    247,
                                    47,
                                    77,
                                    102,
                                    42,
                                    152,
                                    148,
                                    86,
                                    113,
                                    240,
                                    14,
                                    93,
                                    176,
                                    37,
                                    101,
                                    152,
                                    96,
                                    202,
                                    223,
                                ],
                            },
                        ),
                    ),
                },
            ): [
                XvcEntity(
                    5,
                    18084731218162541931,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        9554303650483606518,
                    ),
                    value: Generic(
                        GenericDep {
                            generic_command: "date +%F",
                            output_digest: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        3,
                        9554303650483606518,
                    ),
                    value: Generic(
                        GenericDep {
                            generic_command: "date +%F",
                            output_digest: Some(
                                StdoutDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            50,
                                            73,
                                            75,
                                            188,
                                            90,
                                            228,
                                            145,
                                            158,
                                            166,
                                            111,
                                            43,
                                            244,
                                            44,
                                            137,
                                            182,
                                            122,
                                            255,
                                            80,
                                            237,
                                            43,
                                            182,
                                            75,
                                            3,
                                            239,
                                            175,
                                            227,
                                            26,
                                            66,
                                            250,
                                            189,
                                            163,
                                            117,
                                        ],
                                    },
                                ),
                            ),
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        18084731218162541931,
                    ),
                    value: Generic(
                        GenericDep {
                            generic_command: "ls -lR",
                            output_digest: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        18084731218162541931,
                    ),
                    value: Generic(
                        GenericDep {
                            generic_command: "ls -lR",
                            output_digest: Some(
                                StdoutDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            113,
                                            31,
                                            8,
                                            96,
                                            119,
                                            69,
                                            101,
                                            63,
                                            176,
                                            181,
                                            172,
                                            2,
                                            130,
                                            247,
                                            47,
                                            77,
                                            102,
                                            42,
                                            152,
                                            148,
                                            86,
                                            113,
                                            240,
                                            14,
                                            93,
                                            176,
                                            37,
                                            101,
                                            152,
                                            96,
                                            202,
                                            223,
                                        ],
                                    },
                                ),
                            ),
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
                9554303650483606518,
            ): ChildEntity(
                XvcEntity(
                    2,
                    14428876823979097321,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
            XvcEntity(
                5,
                18084731218162541931,
            ): ChildEntity(
                XvcEntity(
                    4,
                    11031608649320549610,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
        },
        entity_index: {
            ChildEntity(
                XvcEntity(
                    2,
                    14428876823979097321,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    3,
                    9554303650483606518,
                ),
            ],
            ChildEntity(
                XvcEntity(
                    4,
                    11031608649320549610,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    5,
                    18084731218162541931,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        9554303650483606518,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            14428876823979097321,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        18084731218162541931,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            4,
                            11031608649320549610,
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
[TRACE][pipeline/src/pipeline/mod.rs::671] step_e: XvcEntity(
    4,
    11031608649320549610,
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
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::829] step.name: "directory-contents"
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::829] step.name: "morning-message"
[TRACE][pipeline/src/pipeline/mod.rs::830] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::832] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::829] step.name: "morning-message"
[TRACE][pipeline/src/pipeline/mod.rs::830] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::832] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::830] &r_next_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::832] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::829] step.name: "morning-message"
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::829] step.name: "directory-contents"
[TRACE][pipeline/src/pipeline/mod.rs::830] &r_next_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::832] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::830] &r_next_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::832] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::829] step.name: "directory-contents"
[TRACE][pipeline/src/pipeline/mod.rs::1119] deps: HStore {
    map: {
        XvcEntity(
            3,
            9554303650483606518,
        ): Generic(
            GenericDep {
                generic_command: "date +%F",
                output_digest: Some(
                    StdoutDigest(
                        XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                50,
                                73,
                                75,
                                188,
                                90,
                                228,
                                145,
                                158,
                                166,
                                111,
                                43,
                                244,
                                44,
                                137,
                                182,
                                122,
                                255,
                                80,
                                237,
                                43,
                                182,
                                75,
                                3,
                                239,
                                175,
                                227,
                                26,
                                66,
                                250,
                                189,
                                163,
                                117,
                            ],
                        },
                    ),
                ),
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::830] &r_next_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::832] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/deps/compare.rs::404] generic: GenericDep {
    generic_command: "date +%F",
    output_digest: Some(
        StdoutDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    50,
                    73,
                    75,
                    188,
                    90,
                    228,
                    145,
                    158,
                    166,
                    111,
                    43,
                    244,
                    44,
                    137,
                    182,
                    122,
                    255,
                    80,
                    237,
                    43,
                    182,
                    75,
                    3,
                    239,
                    175,
                    227,
                    26,
                    66,
                    250,
                    189,
                    163,
                    117,
                ],
            },
        ),
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/deps/generic.rs::35] generic_command: "date +%F"
[TRACE][pipeline/src/pipeline/mod.rs::1119] deps: HStore {
    map: {
        XvcEntity(
            5,
            18084731218162541931,
        ): Generic(
            GenericDep {
                generic_command: "ls -lR",
                output_digest: Some(
                    StdoutDigest(
                        XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                113,
                                31,
                                8,
                                96,
                                119,
                                69,
                                101,
                                63,
                                176,
                                181,
                                172,
                                2,
                                130,
                                247,
                                47,
                                77,
                                102,
                                42,
                                152,
                                148,
                                86,
                                113,
                                240,
                                14,
                                93,
                                176,
                                37,
                                101,
                                152,
                                96,
                                202,
                                223,
                            ],
                        },
                    ),
                ),
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/deps/compare.rs::404] generic: GenericDep {
    generic_command: "ls -lR",
    output_digest: Some(
        StdoutDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    113,
                    31,
                    8,
                    96,
                    119,
                    69,
                    101,
                    63,
                    176,
                    181,
                    172,
                    2,
                    130,
                    247,
                    47,
                    77,
                    102,
                    42,
                    152,
                    148,
                    86,
                    113,
                    240,
                    14,
                    93,
                    176,
                    37,
                    101,
                    152,
                    96,
                    202,
                    223,
                ],
            },
        ),
    ),
}
[TRACE][pipeline/src/pipeline/deps/generic.rs::35] generic_command: "ls -lR"
[TRACE][pipeline/src/pipeline/deps/generic.rs::38] command_output: CaptureData {
    stdout: [
        50,
        48,
        50,
        51,
        45,
        48,
        56,
        45,
        49,
        50,
        10,
    ],
    stderr: [],
    exit_status: Exited(
        0,
    ),
}
[TRACE][pipeline/src/pipeline/deps/generic.rs::38] command_output: CaptureData {
    stdout: [
        116,
        111,
        116,
        97,
        108,
        32,
        48,
        10,
    ],
    stderr: [],
    exit_status: Exited(
        0,
    ),
}
[TRACE][pipeline/src/pipeline/deps/generic.rs::41] stdout: "2023-08-12
"
[TRACE][pipeline/src/pipeline/deps/generic.rs::42] stderr: ""
[TRACE][pipeline/src/pipeline/deps/generic.rs::41] stdout: "total 0
"
[TRACE][pipeline/src/pipeline/deps/generic.rs::42] stderr: ""
[TRACE][pipeline/src/pipeline/deps/generic.rs::67] record: GenericDep {
    generic_command: "date +%F",
    output_digest: Some(
        StdoutDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    50,
                    73,
                    75,
                    188,
                    90,
                    228,
                    145,
                    158,
                    166,
                    111,
                    43,
                    244,
                    44,
                    137,
                    182,
                    122,
                    255,
                    80,
                    237,
                    43,
                    182,
                    75,
                    3,
                    239,
                    175,
                    227,
                    26,
                    66,
                    250,
                    189,
                    163,
                    117,
                ],
            },
        ),
    ),
}
[TRACE][pipeline/src/pipeline/deps/generic.rs::67] record: GenericDep {
    generic_command: "ls -lR",
    output_digest: Some(
        StdoutDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    113,
                    31,
                    8,
                    96,
                    119,
                    69,
                    101,
                    63,
                    176,
                    181,
                    172,
                    2,
                    130,
                    247,
                    47,
                    77,
                    102,
                    42,
                    152,
                    148,
                    86,
                    113,
                    240,
                    14,
                    93,
                    176,
                    37,
                    101,
                    152,
                    96,
                    202,
                    223,
                ],
            },
        ),
    ),
}
[TRACE][pipeline/src/pipeline/deps/generic.rs::68] actual: GenericDep {
    generic_command: "date +%F",
    output_digest: Some(
        StdoutDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    50,
                    73,
                    75,
                    188,
                    90,
                    228,
                    145,
                    158,
                    166,
                    111,
                    43,
                    244,
                    44,
                    137,
                    182,
                    122,
                    255,
                    80,
                    237,
                    43,
                    182,
                    75,
                    3,
                    239,
                    175,
                    227,
                    26,
                    66,
                    250,
                    189,
                    163,
                    117,
                ],
            },
        ),
    ),
}
[TRACE][pipeline/src/pipeline/deps/generic.rs::68] actual: GenericDep {
    generic_command: "ls -lR",
    output_digest: Some(
        StdoutDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    113,
                    31,
                    8,
                    96,
                    119,
                    69,
                    101,
                    63,
                    176,
                    181,
                    172,
                    2,
                    130,
                    247,
                    47,
                    77,
                    102,
                    42,
                    152,
                    148,
                    86,
                    113,
                    240,
                    14,
                    93,
                    176,
                    37,
                    101,
                    152,
                    96,
                    202,
                    223,
                ],
            },
        ),
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::1136] step_dependency_diffs: HStore {
    map: {
        XvcEntity(
            5,
            18084731218162541931,
        ): Identical,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1136] step_dependency_diffs: HStore {
    map: {
        XvcEntity(
            3,
            9554303650483606518,
        ): Identical,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1146] changed: false
[TRACE][pipeline/src/pipeline/mod.rs::1146] changed: false
[TRACE][pipeline/src/pipeline/mod.rs::829] step.name: "directory-contents"
[TRACE][pipeline/src/pipeline/mod.rs::830] &r_next_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::829] step.name: "morning-message"
[TRACE][pipeline/src/pipeline/mod.rs::832] &step_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::830] &r_next_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::832] &step_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::1066] params.step_dependencies: {}
[TRACE][pipeline/src/pipeline/mod.rs::1066] params.step_dependencies: {}
[TRACE][pipeline/src/pipeline/mod.rs::1104] changed: false
[TRACE][pipeline/src/pipeline/mod.rs::1104] changed: false
[TRACE][pipeline/src/pipeline/mod.rs::829] step.name: "morning-message"
[TRACE][pipeline/src/pipeline/mod.rs::830] &r_next_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::829] step.name: "directory-contents"
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::832] &step_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::830] &r_next_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::832] &step_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::711] &step_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::631] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::554] (step_e, &jh): (
    XvcEntity(
        4,
        11031608649320549610,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::561] "Before state updater": "Before state updater"
[TRACE][pipeline/src/pipeline/mod.rs::572] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                14428876823979097321,
            ): DoneWithoutRunning(
                FromDiffsHasNotChanged,
            ),
            XvcEntity(
                4,
                11031608649320549610,
            ): DoneWithoutRunning(
                FromDiffsHasNotChanged,
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
[TRACE][lib/src/cli/mod.rs::582] git_add_output: ""

```
