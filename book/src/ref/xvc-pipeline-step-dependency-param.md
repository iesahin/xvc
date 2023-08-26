# (Hyper-)Parameter Dependencies

You may be keeping pipeline-wide parameters in structured text files. You can specify such parameters found in JSON,
TOML and YAML files as dependencies.


This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Suppose we have a YAML file that we specify various parameters for the whole connection.

```yaml
param: value
database:
  server: example.com
  port: 5432
  connection:
    timeout: 5000
numeric_param: 13
```

```
$ xvc pipeline step new --step-name read-database-config --command 'echo "Updated Database Configuration"'

$ xvc pipeline step new --step-name read-hyperparams --command 'echo "Update Hyperparameters"'

```

Let's create different steps for various pieces of this parameters file:

```console
$ xvc pipeline step dependency --step-name read-database-config --param 'myparams.yaml::database.port' --param 'myparams.yaml::database.server' --param 'myparams.yaml::database.connection'

$ xvc pipeline step dependency --step-name read-hyperparams --param 'myparams.yaml::param' --param 'myparams.yaml::numeric_param'

```

Run for the first time, as initially all dependencies are invalid:

```console
$ xvc pipeline run
[OUT] [read-hyperparams] Update Hyperparameters
 
[OUT] [read-database-config] Updated Database Configuration
 

```

For the second time, it won't read the configuration as nothing is changed:

```console
$ xvc pipeline run

```

When you update a value in this file, it will only invalidate the steps that depend on the value, not other dependencies
that rely on the same file.

Let's update the database port:

```console
$ perl -pi -e 's/5432/9876/g' myparams.yaml

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
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "core.guid": String(
                    "b699699fc944c69d",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "core.guid": String(
                    "e8b927ad14610598",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "git.command": String(
                    "git",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
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
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "e8b927ad14610598",
            ),
        },
        "git.auto_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
            ),
        },
        "file.track.text_or_binary": XvcConfigValue {
            source: Project,
            value: String(
                "auto",
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
        "file.carry-in.no_parallel": XvcConfigValue {
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
        "core.quiet": XvcConfigValue {
            source: CommandLine,
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
        "git.auto_stage": XvcConfigValue {
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
        "file.track.no_commit": XvcConfigValue {
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
        "file.carry-in.force": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
        "file.list.no_summary": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"b699699fc944c69d/"
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
    "[CWD]/.xvc/ec/1693046651521551",
    "[CWD]/.xvc/ec/1693046651523581",
    "[CWD]/.xvc/ec/1693046651589599",
    "[CWD]/.xvc/ec/1693046651657383",
    "[CWD]/.xvc/ec/1693046651740881",
    "[CWD]/.xvc/ec/1693046651814948",
]
[TRACE][pipeline/src/lib.rs::358] name: Some(
    "default",
)
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/myparams.toml"
[TRACE][core/src/types/xvcpath.rs::88] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::89] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/myparams.yaml"
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
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/myparams.json"
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
    paths: 0x000060000107c030,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000600002f74150,
    runloop: Some(
        (
            0x0000600002274800,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::306] pipeline_len: 2
[TRACE][pipeline/src/pipeline/mod.rs::332] &dependency_graph: {
    XvcEntity(
        3,
        3369321840027960889,
    ): [],
    XvcEntity(
        2,
        4244321473927733562,
    ): [],
}
[TRACE][pipeline/src/pipeline/mod.rs::344] &dependency_graph: {
    XvcEntity(
        3,
        3369321840027960889,
    ): [],
    XvcEntity(
        2,
        4244321473927733562,
    ): [],
}
[INFO][pipeline/src/pipeline/mod.rs::348] Pipeline Graph:
digraph {
    0 [ label = "(3, 3369321840027960889)" ]
    1 [ label = "(2, 4244321473927733562)" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::425] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                3,
                3369321840027960889,
            ): Begin(
                FromInit,
            ),
            XvcEntity(
                2,
                4244321473927733562,
            ): Begin(
                FromInit,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::551] &step_thread_store: HStore {
    map: {
        XvcEntity(
            2,
            4244321473927733562,
        ): ScopedJoinHandle { .. },
        XvcEntity(
            3,
            3369321840027960889,
        ): ScopedJoinHandle { .. },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::555] (step_e, &jh): (
    XvcEntity(
        2,
        4244321473927733562,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::671] params.recorded_dependencies: R1NStore {
    parents: XvcStore {
        map: {
            XvcEntity(
                2,
                4244321473927733562,
            ): XvcStep {
                name: "read-database-config",
            },
            XvcEntity(
                3,
                3369321840027960889,
            ): XvcStep {
                name: "read-hyperparams",
            },
        },
        entity_index: {
            XvcStep {
                name: "read-database-config",
            }: [
                XvcEntity(
                    2,
                    4244321473927733562,
                ),
            ],
            XvcStep {
                name: "read-hyperparams",
            }: [
                XvcEntity(
                    3,
                    3369321840027960889,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    value: XvcStep {
                        name: "read-database-config",
                    },
                },
                Add {
                    entity: XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    value: XvcStep {
                        name: "read-database-config",
                    },
                },
                Add {
                    entity: XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    value: XvcStep {
                        name: "read-hyperparams",
                    },
                },
                Add {
                    entity: XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    value: XvcStep {
                        name: "read-hyperparams",
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
                4,
                2200857687599595040,
            ): Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.port",
                    value: Some(
                        Yaml(
                            Number(5432),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
            XvcEntity(
                5,
                2200857687599595040,
            ): Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.server",
                    value: Some(
                        Yaml(
                            String("example.com"),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
            XvcEntity(
                6,
                2200857687599595040,
            ): Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.connection",
                    value: Some(
                        Yaml(
                            Mapping {
                                "timeout": Number(5000),
                            },
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
            XvcEntity(
                7,
                7714117151203501825,
            ): Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "param",
                    value: Some(
                        Yaml(
                            String("value"),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
            XvcEntity(
                8,
                7714117151203501825,
            ): Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "numeric_param",
                    value: Some(
                        Yaml(
                            Number(13),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
        },
        entity_index: {
            Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.connection",
                    value: Some(
                        Yaml(
                            Mapping {
                                "timeout": Number(5000),
                            },
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ): [
                XvcEntity(
                    6,
                    2200857687599595040,
                ),
            ],
            Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.port",
                    value: Some(
                        Yaml(
                            Number(5432),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ): [
                XvcEntity(
                    4,
                    2200857687599595040,
                ),
            ],
            Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.server",
                    value: Some(
                        Yaml(
                            String("example.com"),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ): [
                XvcEntity(
                    5,
                    2200857687599595040,
                ),
            ],
            Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "numeric_param",
                    value: Some(
                        Yaml(
                            Number(13),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ): [
                XvcEntity(
                    8,
                    7714117151203501825,
                ),
            ],
            Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "param",
                    value: Some(
                        Yaml(
                            String("value"),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ): [
                XvcEntity(
                    7,
                    7714117151203501825,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: None,
                            xvc_metadata: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: None,
                            xvc_metadata: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: None,
                            xvc_metadata: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: None,
                            xvc_metadata: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: None,
                            xvc_metadata: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: Some(
                                Yaml(
                                    String("example.com"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: Some(
                                Yaml(
                                    Number(5432),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: Some(
                                Yaml(
                                    Mapping {
                                        "timeout": Number(5000),
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: Some(
                                Yaml(
                                    Number(13),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: Some(
                                Yaml(
                                    String("value"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
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
                4,
                2200857687599595040,
            ): ChildEntity(
                XvcEntity(
                    2,
                    4244321473927733562,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
            XvcEntity(
                5,
                2200857687599595040,
            ): ChildEntity(
                XvcEntity(
                    2,
                    4244321473927733562,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
            XvcEntity(
                6,
                2200857687599595040,
            ): ChildEntity(
                XvcEntity(
                    2,
                    4244321473927733562,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
            XvcEntity(
                7,
                7714117151203501825,
            ): ChildEntity(
                XvcEntity(
                    3,
                    3369321840027960889,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
            XvcEntity(
                8,
                7714117151203501825,
            ): ChildEntity(
                XvcEntity(
                    3,
                    3369321840027960889,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
        },
        entity_index: {
            ChildEntity(
                XvcEntity(
                    2,
                    4244321473927733562,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    4,
                    2200857687599595040,
                ),
                XvcEntity(
                    5,
                    2200857687599595040,
                ),
                XvcEntity(
                    6,
                    2200857687599595040,
                ),
            ],
            ChildEntity(
                XvcEntity(
                    3,
                    3369321840027960889,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    7,
                    7714117151203501825,
                ),
                XvcEntity(
                    8,
                    7714117151203501825,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
                Add {
                    entity: XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
                Add {
                    entity: XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            3,
                            3369321840027960889,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
                Add {
                    entity: XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            3,
                            3369321840027960889,
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
[TRACE][pipeline/src/pipeline/mod.rs::672] step_e: XvcEntity(
    2,
    4244321473927733562,
)
[TRACE][pipeline/src/pipeline/mod.rs::671] params.recorded_dependencies: R1NStore {
    parents: XvcStore {
        map: {
            XvcEntity(
                2,
                4244321473927733562,
            ): XvcStep {
                name: "read-database-config",
            },
            XvcEntity(
                3,
                3369321840027960889,
            ): XvcStep {
                name: "read-hyperparams",
            },
        },
        entity_index: {
            XvcStep {
                name: "read-database-config",
            }: [
                XvcEntity(
                    2,
                    4244321473927733562,
                ),
            ],
            XvcStep {
                name: "read-hyperparams",
            }: [
                XvcEntity(
                    3,
                    3369321840027960889,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    value: XvcStep {
                        name: "read-database-config",
                    },
                },
                Add {
                    entity: XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    value: XvcStep {
                        name: "read-database-config",
                    },
                },
                Add {
                    entity: XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    value: XvcStep {
                        name: "read-hyperparams",
                    },
                },
                Add {
                    entity: XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    value: XvcStep {
                        name: "read-hyperparams",
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
                4,
                2200857687599595040,
            ): Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.port",
                    value: Some(
                        Yaml(
                            Number(5432),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
            XvcEntity(
                5,
                2200857687599595040,
            ): Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.server",
                    value: Some(
                        Yaml(
                            String("example.com"),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
            XvcEntity(
                6,
                2200857687599595040,
            ): Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.connection",
                    value: Some(
                        Yaml(
                            Mapping {
                                "timeout": Number(5000),
                            },
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
            XvcEntity(
                7,
                7714117151203501825,
            ): Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "param",
                    value: Some(
                        Yaml(
                            String("value"),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
            XvcEntity(
                8,
                7714117151203501825,
            ): Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "numeric_param",
                    value: Some(
                        Yaml(
                            Number(13),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
        },
        entity_index: {
            Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.connection",
                    value: Some(
                        Yaml(
                            Mapping {
                                "timeout": Number(5000),
                            },
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ): [
                XvcEntity(
                    6,
                    2200857687599595040,
                ),
            ],
            Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.port",
                    value: Some(
                        Yaml(
                            Number(5432),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ): [
                XvcEntity(
                    4,
                    2200857687599595040,
                ),
            ],
            Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.server",
                    value: Some(
                        Yaml(
                            String("example.com"),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ): [
                XvcEntity(
                    5,
                    2200857687599595040,
                ),
            ],
            Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "numeric_param",
                    value: Some(
                        Yaml(
                            Number(13),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ): [
                XvcEntity(
                    8,
                    7714117151203501825,
                ),
            ],
            Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "param",
                    value: Some(
                        Yaml(
                            String("value"),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ): [
                XvcEntity(
                    7,
                    7714117151203501825,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: None,
                            xvc_metadata: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: None,
                            xvc_metadata: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: None,
                            xvc_metadata: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: None,
                            xvc_metadata: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: None,
                            xvc_metadata: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: Some(
                                Yaml(
                                    String("example.com"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: Some(
                                Yaml(
                                    Number(5432),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: Some(
                                Yaml(
                                    Mapping {
                                        "timeout": Number(5000),
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: Some(
                                Yaml(
                                    Number(13),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                    value: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: Some(
                                Yaml(
                                    String("value"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
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
                4,
                2200857687599595040,
            ): ChildEntity(
                XvcEntity(
                    2,
                    4244321473927733562,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
            XvcEntity(
                5,
                2200857687599595040,
            ): ChildEntity(
                XvcEntity(
                    2,
                    4244321473927733562,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
            XvcEntity(
                6,
                2200857687599595040,
            ): ChildEntity(
                XvcEntity(
                    2,
                    4244321473927733562,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
            XvcEntity(
                7,
                7714117151203501825,
            ): ChildEntity(
                XvcEntity(
                    3,
                    3369321840027960889,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
            XvcEntity(
                8,
                7714117151203501825,
            ): ChildEntity(
                XvcEntity(
                    3,
                    3369321840027960889,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
        },
        entity_index: {
            ChildEntity(
                XvcEntity(
                    2,
                    4244321473927733562,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    4,
                    2200857687599595040,
                ),
                XvcEntity(
                    5,
                    2200857687599595040,
                ),
                XvcEntity(
                    6,
                    2200857687599595040,
                ),
            ],
            ChildEntity(
                XvcEntity(
                    3,
                    3369321840027960889,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    7,
                    7714117151203501825,
                ),
                XvcEntity(
                    8,
                    7714117151203501825,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
                Add {
                    entity: XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
                Add {
                    entity: XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            3,
                            3369321840027960889,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
                Add {
                    entity: XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            3,
                            3369321840027960889,
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
[TRACE][pipeline/src/pipeline/mod.rs::613] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::672] step_e: XvcEntity(
    3,
    3369321840027960889,
)
[TRACE][pipeline/src/pipeline/mod.rs::613] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::673] dependency_steps(step_e, params.dependency_graph)?: {}
[TRACE][pipeline/src/pipeline/mod.rs::613] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::673] dependency_steps(step_e, params.dependency_graph)?: {}
[TRACE][pipeline/src/pipeline/mod.rs::613] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-hyperparams"
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-database-config"
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-hyperparams"
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-hyperparams"
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-database-config"
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1110] parent_entity: XvcEntity(
    3,
    3369321840027960889,
)
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-database-config"
[TRACE][pipeline/src/pipeline/mod.rs::1113] deps: HStore {
    map: {
        XvcEntity(
            8,
            7714117151203501825,
        ): Param(
            ParamDep {
                format: YAML,
                path: XvcPath(
                    "myparams.yaml",
                ),
                key: "numeric_param",
                value: Some(
                    Yaml(
                        Number(13),
                    ),
                ),
                xvc_metadata: Some(
                    XvcMetadata {
                        file_type: File,
                        size: Some(
                            108,
                        ),
                        modified: Some(
                            SystemTime {
                                tv_sec: 1693042660,
                                tv_nsec: 806716570,
                            },
                        ),
                    },
                ),
            },
        ),
        XvcEntity(
            7,
            7714117151203501825,
        ): Param(
            ParamDep {
                format: YAML,
                path: XvcPath(
                    "myparams.yaml",
                ),
                key: "param",
                value: Some(
                    Yaml(
                        String("value"),
                    ),
                ),
                xvc_metadata: Some(
                    XvcMetadata {
                        file_type: File,
                        size: Some(
                            108,
                        ),
                        modified: Some(
                            SystemTime {
                                tv_sec: 1693042660,
                                tv_nsec: 806716570,
                            },
                        ),
                    },
                ),
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/deps/compare.rs::428] &stored: Param(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "numeric_param",
        value: Some(
            Yaml(
                Number(13),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042660,
                        tv_nsec: 806716570,
                    },
                ),
            },
        ),
    },
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::1110] parent_entity: XvcEntity(
    2,
    4244321473927733562,
)
[TRACE][pipeline/src/pipeline/deps/param.rs::76] record: ParamDep {
    format: YAML,
    path: XvcPath(
        "myparams.yaml",
    ),
    key: "numeric_param",
    value: Some(
        Yaml(
            Number(13),
        ),
    ),
    xvc_metadata: Some(
        XvcMetadata {
            file_type: File,
            size: Some(
                108,
            ),
            modified: Some(
                SystemTime {
                    tv_sec: 1693042660,
                    tv_nsec: 806716570,
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/deps/param.rs::77] actual: ParamDep {
    format: YAML,
    path: XvcPath(
        "myparams.yaml",
    ),
    key: "numeric_param",
    value: None,
    xvc_metadata: Some(
        XvcMetadata {
            file_type: File,
            size: Some(
                108,
            ),
            modified: Some(
                SystemTime {
                    tv_sec: 1693046654,
                    tv_nsec: 465778895,
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/deps/compare.rs::428] &stored: Param(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "param",
        value: Some(
            Yaml(
                String("value"),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042660,
                        tv_nsec: 806716570,
                    },
                ),
            },
        ),
    },
)
[TRACE][pipeline/src/pipeline/mod.rs::1113] deps: HStore {
    map: {
        XvcEntity(
            5,
            2200857687599595040,
        ): Param(
            ParamDep {
                format: YAML,
                path: XvcPath(
                    "myparams.yaml",
                ),
                key: "database.server",
                value: Some(
                    Yaml(
                        String("example.com"),
                    ),
                ),
                xvc_metadata: Some(
                    XvcMetadata {
                        file_type: File,
                        size: Some(
                            108,
                        ),
                        modified: Some(
                            SystemTime {
                                tv_sec: 1693042660,
                                tv_nsec: 806716570,
                            },
                        ),
                    },
                ),
            },
        ),
        XvcEntity(
            4,
            2200857687599595040,
        ): Param(
            ParamDep {
                format: YAML,
                path: XvcPath(
                    "myparams.yaml",
                ),
                key: "database.port",
                value: Some(
                    Yaml(
                        Number(5432),
                    ),
                ),
                xvc_metadata: Some(
                    XvcMetadata {
                        file_type: File,
                        size: Some(
                            108,
                        ),
                        modified: Some(
                            SystemTime {
                                tv_sec: 1693042660,
                                tv_nsec: 806716570,
                            },
                        ),
                    },
                ),
            },
        ),
        XvcEntity(
            6,
            2200857687599595040,
        ): Param(
            ParamDep {
                format: YAML,
                path: XvcPath(
                    "myparams.yaml",
                ),
                key: "database.connection",
                value: Some(
                    Yaml(
                        Mapping {
                            "timeout": Number(5000),
                        },
                    ),
                ),
                xvc_metadata: Some(
                    XvcMetadata {
                        file_type: File,
                        size: Some(
                            108,
                        ),
                        modified: Some(
                            SystemTime {
                                tv_sec: 1693042660,
                                tv_nsec: 806716570,
                            },
                        ),
                    },
                ),
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/deps/param.rs::76] record: ParamDep {
    format: YAML,
    path: XvcPath(
        "myparams.yaml",
    ),
    key: "param",
    value: Some(
        Yaml(
            String("value"),
        ),
    ),
    xvc_metadata: Some(
        XvcMetadata {
            file_type: File,
            size: Some(
                108,
            ),
            modified: Some(
                SystemTime {
                    tv_sec: 1693042660,
                    tv_nsec: 806716570,
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/deps/param.rs::77] actual: ParamDep {
    format: YAML,
    path: XvcPath(
        "myparams.yaml",
    ),
    key: "param",
    value: None,
    xvc_metadata: Some(
        XvcMetadata {
            file_type: File,
            size: Some(
                108,
            ),
            modified: Some(
                SystemTime {
                    tv_sec: 1693046654,
                    tv_nsec: 465778895,
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/deps/compare.rs::428] &stored: Param(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "database.server",
        value: Some(
            Yaml(
                String("example.com"),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042660,
                        tv_nsec: 806716570,
                    },
                ),
            },
        ),
    },
)
[TRACE][pipeline/src/pipeline/deps/param.rs::76] record: ParamDep {
    format: YAML,
    path: XvcPath(
        "myparams.yaml",
    ),
    key: "database.server",
    value: Some(
        Yaml(
            String("example.com"),
        ),
    ),
    xvc_metadata: Some(
        XvcMetadata {
            file_type: File,
            size: Some(
                108,
            ),
            modified: Some(
                SystemTime {
                    tv_sec: 1693042660,
                    tv_nsec: 806716570,
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/deps/param.rs::77] actual: ParamDep {
    format: YAML,
    path: XvcPath(
        "myparams.yaml",
    ),
    key: "database.server",
    value: None,
    xvc_metadata: Some(
        XvcMetadata {
            file_type: File,
            size: Some(
                108,
            ),
            modified: Some(
                SystemTime {
                    tv_sec: 1693046654,
                    tv_nsec: 465778895,
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::1130] step_dependency_diffs: HStore {
    map: {
        XvcEntity(
            8,
            7714117151203501825,
        ): Different {
            record: Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "numeric_param",
                    value: Some(
                        Yaml(
                            Number(13),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
            actual: Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "numeric_param",
                    value: None,
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693046654,
                                    tv_nsec: 465778895,
                                },
                            ),
                        },
                    ),
                },
            ),
        },
        XvcEntity(
            7,
            7714117151203501825,
        ): Different {
            record: Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "param",
                    value: Some(
                        Yaml(
                            String("value"),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
            actual: Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "param",
                    value: None,
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693046654,
                                    tv_nsec: 465778895,
                                },
                            ),
                        },
                    ),
                },
            ),
        },
    },
}
[TRACE][pipeline/src/pipeline/deps/compare.rs::428] &stored: Param(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "database.port",
        value: Some(
            Yaml(
                Number(5432),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042660,
                        tv_nsec: 806716570,
                    },
                ),
            },
        ),
    },
)
[TRACE][pipeline/src/pipeline/mod.rs::1136] diff: Different {
    record: Param(
        ParamDep {
            format: YAML,
            path: XvcPath(
                "myparams.yaml",
            ),
            key: "numeric_param",
            value: Some(
                Yaml(
                    Number(13),
                ),
            ),
            xvc_metadata: Some(
                XvcMetadata {
                    file_type: File,
                    size: Some(
                        108,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1693042660,
                            tv_nsec: 806716570,
                        },
                    ),
                },
            ),
        },
    ),
    actual: Param(
        ParamDep {
            format: YAML,
            path: XvcPath(
                "myparams.yaml",
            ),
            key: "numeric_param",
            value: None,
            xvc_metadata: Some(
                XvcMetadata {
                    file_type: File,
                    size: Some(
                        108,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1693046654,
                            tv_nsec: 465778895,
                        },
                    ),
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/deps/param.rs::76] record: ParamDep {
    format: YAML,
    path: XvcPath(
        "myparams.yaml",
    ),
    key: "database.port",
    value: Some(
        Yaml(
            Number(5432),
        ),
    ),
    xvc_metadata: Some(
        XvcMetadata {
            file_type: File,
            size: Some(
                108,
            ),
            modified: Some(
                SystemTime {
                    tv_sec: 1693042660,
                    tv_nsec: 806716570,
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::1137] diff.changed(): true
[TRACE][pipeline/src/pipeline/deps/param.rs::77] actual: ParamDep {
    format: YAML,
    path: XvcPath(
        "myparams.yaml",
    ),
    key: "database.port",
    value: None,
    xvc_metadata: Some(
        XvcMetadata {
            file_type: File,
            size: Some(
                108,
            ),
            modified: Some(
                SystemTime {
                    tv_sec: 1693046654,
                    tv_nsec: 465778895,
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/deps/compare.rs::428] &stored: Param(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "database.connection",
        value: Some(
            Yaml(
                Mapping {
                    "timeout": Number(5000),
                },
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042660,
                        tv_nsec: 806716570,
                    },
                ),
            },
        ),
    },
)
[TRACE][pipeline/src/pipeline/mod.rs::1136] diff: Different {
    record: Param(
        ParamDep {
            format: YAML,
            path: XvcPath(
                "myparams.yaml",
            ),
            key: "param",
            value: Some(
                Yaml(
                    String("value"),
                ),
            ),
            xvc_metadata: Some(
                XvcMetadata {
                    file_type: File,
                    size: Some(
                        108,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1693042660,
                            tv_nsec: 806716570,
                        },
                    ),
                },
            ),
        },
    ),
    actual: Param(
        ParamDep {
            format: YAML,
            path: XvcPath(
                "myparams.yaml",
            ),
            key: "param",
            value: None,
            xvc_metadata: Some(
                XvcMetadata {
                    file_type: File,
                    size: Some(
                        108,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1693046654,
                            tv_nsec: 465778895,
                        },
                    ),
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::1137] diff.changed(): true
[TRACE][pipeline/src/pipeline/deps/param.rs::76] record: ParamDep {
    format: YAML,
    path: XvcPath(
        "myparams.yaml",
    ),
    key: "database.connection",
    value: Some(
        Yaml(
            Mapping {
                "timeout": Number(5000),
            },
        ),
    ),
    xvc_metadata: Some(
        XvcMetadata {
            file_type: File,
            size: Some(
                108,
            ),
            modified: Some(
                SystemTime {
                    tv_sec: 1693042660,
                    tv_nsec: 806716570,
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::1142] changed: true
[TRACE][pipeline/src/pipeline/deps/param.rs::77] actual: ParamDep {
    format: YAML,
    path: XvcPath(
        "myparams.yaml",
    ),
    key: "database.connection",
    value: None,
    xvc_metadata: Some(
        XvcMetadata {
            file_type: File,
            size: Some(
                108,
            ),
            modified: Some(
                SystemTime {
                    tv_sec: 1693046654,
                    tv_nsec: 465778895,
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-hyperparams"
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1167] deps: HStore {
    map: {
        XvcEntity(
            7,
            7714117151203501825,
        ): Param(
            ParamDep {
                format: YAML,
                path: XvcPath(
                    "myparams.yaml",
                ),
                key: "param",
                value: Some(
                    Yaml(
                        String("value"),
                    ),
                ),
                xvc_metadata: Some(
                    XvcMetadata {
                        file_type: File,
                        size: Some(
                            108,
                        ),
                        modified: Some(
                            SystemTime {
                                tv_sec: 1693042660,
                                tv_nsec: 806716570,
                            },
                        ),
                    },
                ),
            },
        ),
        XvcEntity(
            8,
            7714117151203501825,
        ): Param(
            ParamDep {
                format: YAML,
                path: XvcPath(
                    "myparams.yaml",
                ),
                key: "numeric_param",
                value: Some(
                    Yaml(
                        Number(13),
                    ),
                ),
                xvc_metadata: Some(
                    XvcMetadata {
                        file_type: File,
                        size: Some(
                            108,
                        ),
                        modified: Some(
                            SystemTime {
                                tv_sec: 1693042660,
                                tv_nsec: 806716570,
                            },
                        ),
                    },
                ),
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1130] step_dependency_diffs: HStore {
    map: {
        XvcEntity(
            5,
            2200857687599595040,
        ): Different {
            record: Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.server",
                    value: Some(
                        Yaml(
                            String("example.com"),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
            actual: Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.server",
                    value: None,
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693046654,
                                    tv_nsec: 465778895,
                                },
                            ),
                        },
                    ),
                },
            ),
        },
        XvcEntity(
            6,
            2200857687599595040,
        ): Different {
            record: Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.connection",
                    value: Some(
                        Yaml(
                            Mapping {
                                "timeout": Number(5000),
                            },
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
            actual: Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.connection",
                    value: None,
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693046654,
                                    tv_nsec: 465778895,
                                },
                            ),
                        },
                    ),
                },
            ),
        },
        XvcEntity(
            4,
            2200857687599595040,
        ): Different {
            record: Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.port",
                    value: Some(
                        Yaml(
                            Number(5432),
                        ),
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693042660,
                                    tv_nsec: 806716570,
                                },
                            ),
                        },
                    ),
                },
            ),
            actual: Param(
                ParamDep {
                    format: YAML,
                    path: XvcPath(
                        "myparams.yaml",
                    ),
                    key: "database.port",
                    value: None,
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                108,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1693046654,
                                    tv_nsec: 465778895,
                                },
                            ),
                        },
                    ),
                },
            ),
        },
    },
}
[TRACE][core/src/types/diff.rs::341] record: Some(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "param",
        value: Some(
            Yaml(
                String("value"),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042660,
                        tv_nsec: 806716570,
                    },
                ),
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::342] actual: Some(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "param",
        value: Some(
            Yaml(
                String("value"),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693046654,
                        tv_nsec: 465778895,
                    },
                ),
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::341] record: Some(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "numeric_param",
        value: Some(
            Yaml(
                Number(13),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042660,
                        tv_nsec: 806716570,
                    },
                ),
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::342] actual: Some(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "numeric_param",
        value: Some(
            Yaml(
                Number(13),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693046654,
                        tv_nsec: 465778895,
                    },
                ),
            },
        ),
    },
)
[TRACE][pipeline/src/pipeline/mod.rs::1136] diff: Different {
    record: Param(
        ParamDep {
            format: YAML,
            path: XvcPath(
                "myparams.yaml",
            ),
            key: "database.server",
            value: Some(
                Yaml(
                    String("example.com"),
                ),
            ),
            xvc_metadata: Some(
                XvcMetadata {
                    file_type: File,
                    size: Some(
                        108,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1693042660,
                            tv_nsec: 806716570,
                        },
                    ),
                },
            ),
        },
    ),
    actual: Param(
        ParamDep {
            format: YAML,
            path: XvcPath(
                "myparams.yaml",
            ),
            key: "database.server",
            value: None,
            xvc_metadata: Some(
                XvcMetadata {
                    file_type: File,
                    size: Some(
                        108,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1693046654,
                            tv_nsec: 465778895,
                        },
                    ),
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::1137] diff.changed(): true
[TRACE][pipeline/src/pipeline/mod.rs::1136] diff: Different {
    record: Param(
        ParamDep {
            format: YAML,
            path: XvcPath(
                "myparams.yaml",
            ),
            key: "database.connection",
            value: Some(
                Yaml(
                    Mapping {
                        "timeout": Number(5000),
                    },
                ),
            ),
            xvc_metadata: Some(
                XvcMetadata {
                    file_type: File,
                    size: Some(
                        108,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1693042660,
                            tv_nsec: 806716570,
                        },
                    ),
                },
            ),
        },
    ),
    actual: Param(
        ParamDep {
            format: YAML,
            path: XvcPath(
                "myparams.yaml",
            ),
            key: "database.connection",
            value: None,
            xvc_metadata: Some(
                XvcMetadata {
                    file_type: File,
                    size: Some(
                        108,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1693046654,
                            tv_nsec: 465778895,
                        },
                    ),
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::1137] diff.changed(): true
[TRACE][pipeline/src/pipeline/mod.rs::1136] diff: Different {
    record: Param(
        ParamDep {
            format: YAML,
            path: XvcPath(
                "myparams.yaml",
            ),
            key: "database.port",
            value: Some(
                Yaml(
                    Number(5432),
                ),
            ),
            xvc_metadata: Some(
                XvcMetadata {
                    file_type: File,
                    size: Some(
                        108,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1693042660,
                            tv_nsec: 806716570,
                        },
                    ),
                },
            ),
        },
    ),
    actual: Param(
        ParamDep {
            format: YAML,
            path: XvcPath(
                "myparams.yaml",
            ),
            key: "database.port",
            value: None,
            xvc_metadata: Some(
                XvcMetadata {
                    file_type: File,
                    size: Some(
                        108,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1693046654,
                            tv_nsec: 465778895,
                        },
                    ),
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::1137] diff.changed(): true
[TRACE][pipeline/src/pipeline/mod.rs::1142] changed: true
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-database-config"
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-hyperparams"
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-hyperparams"
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::1167] deps: HStore {
    map: {
        XvcEntity(
            5,
            2200857687599595040,
        ): Param(
            ParamDep {
                format: YAML,
                path: XvcPath(
                    "myparams.yaml",
                ),
                key: "database.server",
                value: Some(
                    Yaml(
                        String("example.com"),
                    ),
                ),
                xvc_metadata: Some(
                    XvcMetadata {
                        file_type: File,
                        size: Some(
                            108,
                        ),
                        modified: Some(
                            SystemTime {
                                tv_sec: 1693042660,
                                tv_nsec: 806716570,
                            },
                        ),
                    },
                ),
            },
        ),
        XvcEntity(
            4,
            2200857687599595040,
        ): Param(
            ParamDep {
                format: YAML,
                path: XvcPath(
                    "myparams.yaml",
                ),
                key: "database.port",
                value: Some(
                    Yaml(
                        Number(5432),
                    ),
                ),
                xvc_metadata: Some(
                    XvcMetadata {
                        file_type: File,
                        size: Some(
                            108,
                        ),
                        modified: Some(
                            SystemTime {
                                tv_sec: 1693042660,
                                tv_nsec: 806716570,
                            },
                        ),
                    },
                ),
            },
        ),
        XvcEntity(
            6,
            2200857687599595040,
        ): Param(
            ParamDep {
                format: YAML,
                path: XvcPath(
                    "myparams.yaml",
                ),
                key: "database.connection",
                value: Some(
                    Yaml(
                        Mapping {
                            "timeout": Number(5000),
                        },
                    ),
                ),
                xvc_metadata: Some(
                    XvcMetadata {
                        file_type: File,
                        size: Some(
                            108,
                        ),
                        modified: Some(
                            SystemTime {
                                tv_sec: 1693042660,
                                tv_nsec: 806716570,
                            },
                        ),
                    },
                ),
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][core/src/types/diff.rs::341] record: Some(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "database.connection",
        value: Some(
            Yaml(
                Mapping {
                    "timeout": Number(5000),
                },
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042660,
                        tv_nsec: 806716570,
                    },
                ),
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::342] actual: Some(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "database.connection",
        value: Some(
            Yaml(
                Mapping {
                    "timeout": Number(5000),
                },
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693046654,
                        tv_nsec: 465778895,
                    },
                ),
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::341] record: Some(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "database.port",
        value: Some(
            Yaml(
                Number(5432),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042660,
                        tv_nsec: 806716570,
                    },
                ),
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::342] actual: Some(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "database.port",
        value: Some(
            Yaml(
                Number(9876),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693046654,
                        tv_nsec: 465778895,
                    },
                ),
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::341] record: Some(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "param",
        value: Some(
            Yaml(
                String("value"),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042660,
                        tv_nsec: 806716570,
                    },
                ),
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::342] actual: Some(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "param",
        value: Some(
            Yaml(
                String("value"),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693046654,
                        tv_nsec: 465778895,
                    },
                ),
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::341] record: Some(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "database.server",
        value: Some(
            Yaml(
                String("example.com"),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042660,
                        tv_nsec: 806716570,
                    },
                ),
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::342] actual: Some(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "database.server",
        value: Some(
            Yaml(
                String("example.com"),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693046654,
                        tv_nsec: 465778895,
                    },
                ),
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::341] record: Some(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "numeric_param",
        value: Some(
            Yaml(
                Number(13),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042660,
                        tv_nsec: 806716570,
                    },
                ),
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::342] actual: Some(
    ParamDep {
        format: YAML,
        path: XvcPath(
            "myparams.yaml",
        ),
        key: "numeric_param",
        value: Some(
            Yaml(
                Number(13),
            ),
        ),
        xvc_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693046654,
                        tv_nsec: 465778895,
                    },
                ),
            },
        ),
    },
)
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-database-config"
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-database-config"
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1632] params: StepStateParams {
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
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "core.guid": String(
                            "b699699fc944c69d",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "core.guid": String(
                            "e8b927ad14610598",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "e8b927ad14610598",
                    ),
                },
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
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
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.auto_stage": XvcConfigValue {
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
                "file.track.no_commit": XvcConfigValue {
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
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"b699699fc944c69d/"
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
            counter: 9,
            random: 18409169647092427842,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "myparams.toml",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    259,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042586,
                        tv_nsec: 730145224,
                    },
                ),
            },
            XvcPath(
                "myparams.yaml",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693046654,
                        tv_nsec: 465778895,
                    },
                ),
            },
            XvcPath(
                "myparams.json",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    162,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042629,
                        tv_nsec: 880895440,
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
                        tv_sec: 1693046651,
                        tv_nsec: 521695443,
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
                        tv_sec: 1693046651,
                        tv_nsec: 521760985,
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
                name: "read-hyperparams",
            },
            step_command: XvcStepCommand {
                command: "echo /"Update Hyperparameters/"",
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
                    6,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: Some(
                                Yaml(
                                    Mapping {
                                        "timeout": Number(5000),
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: None,
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    4,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: Some(
                                Yaml(
                                    Number(5432),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: None,
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    7,
                    7714117151203501825,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: Some(
                                Yaml(
                                    String("value"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: Some(
                                Yaml(
                                    String("value"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    5,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: Some(
                                Yaml(
                                    String("example.com"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: None,
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    8,
                    7714117151203501825,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: Some(
                                Yaml(
                                    Number(13),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: Some(
                                Yaml(
                                    Number(13),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
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
        3,
        3369321840027960889,
    ),
    step: XvcStep {
        name: "read-hyperparams",
    },
    step_command: XvcStepCommand {
        command: "echo /"Update Hyperparameters/"",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    3,
                    3369321840027960889,
                ): WaitingToRun(
                    FromDiffsHasChanged,
                ),
                XvcEntity(
                    2,
                    4244321473927733562,
                ): CheckingThoroughDiffs(
                    FromSuperficialDiffsChanged,
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
                3,
                3369321840027960889,
            ): XvcStep {
                name: "read-hyperparams",
            },
            XvcEntity(
                2,
                4244321473927733562,
            ): XvcStep {
                name: "read-database-config",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    4244321473927733562,
                ): XvcStep {
                    name: "read-database-config",
                },
                XvcEntity(
                    3,
                    3369321840027960889,
                ): XvcStep {
                    name: "read-hyperparams",
                },
            },
            entity_index: {
                XvcStep {
                    name: "read-database-config",
                }: [
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                ],
                XvcStep {
                    name: "read-hyperparams",
                }: [
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        value: XvcStep {
                            name: "read-database-config",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        value: XvcStep {
                            name: "read-database-config",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            3369321840027960889,
                        ),
                        value: XvcStep {
                            name: "read-hyperparams",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            3369321840027960889,
                        ),
                        value: XvcStep {
                            name: "read-hyperparams",
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
                    4,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.port",
                        value: Some(
                            Yaml(
                                Number(5432),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    5,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.server",
                        value: Some(
                            Yaml(
                                String("example.com"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    6,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.connection",
                        value: Some(
                            Yaml(
                                Mapping {
                                    "timeout": Number(5000),
                                },
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    7,
                    7714117151203501825,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "param",
                        value: Some(
                            Yaml(
                                String("value"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    8,
                    7714117151203501825,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "numeric_param",
                        value: Some(
                            Yaml(
                                Number(13),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
            },
            entity_index: {
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.connection",
                        value: Some(
                            Yaml(
                                Mapping {
                                    "timeout": Number(5000),
                                },
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.port",
                        value: Some(
                            Yaml(
                                Number(5432),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.server",
                        value: Some(
                            Yaml(
                                String("example.com"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "numeric_param",
                        value: Some(
                            Yaml(
                                Number(13),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "param",
                        value: Some(
                            Yaml(
                                String("value"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.port",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.server",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.connection",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "param",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "numeric_param",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.server",
                                value: Some(
                                    Yaml(
                                        String("example.com"),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.port",
                                value: Some(
                                    Yaml(
                                        Number(5432),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.connection",
                                value: Some(
                                    Yaml(
                                        Mapping {
                                            "timeout": Number(5000),
                                        },
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "numeric_param",
                                value: Some(
                                    Yaml(
                                        Number(13),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "param",
                                value: Some(
                                    Yaml(
                                        String("value"),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
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
                    4,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    5,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    6,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    7,
                    7714117151203501825,
                ): ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    8,
                    7714117151203501825,
                ): ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                    XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                    XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                ],
                ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                    XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                3,
                                3369321840027960889,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                3,
                                3369321840027960889,
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
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-hyperparams"
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/command.rs::81] self.environment: {}
[TRACE][pipeline/src/pipeline/mod.rs::1632] params: StepStateParams {
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
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "core.guid": String(
                            "b699699fc944c69d",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "core.guid": String(
                            "e8b927ad14610598",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "e8b927ad14610598",
                    ),
                },
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
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
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.auto_stage": XvcConfigValue {
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
                "file.track.no_commit": XvcConfigValue {
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
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"b699699fc944c69d/"
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
            counter: 9,
            random: 18409169647092427842,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "myparams.toml",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    259,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042586,
                        tv_nsec: 730145224,
                    },
                ),
            },
            XvcPath(
                "myparams.yaml",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693046654,
                        tv_nsec: 465778895,
                    },
                ),
            },
            XvcPath(
                "myparams.json",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    162,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042629,
                        tv_nsec: 880895440,
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
                        tv_sec: 1693046651,
                        tv_nsec: 521695443,
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
                        tv_sec: 1693046651,
                        tv_nsec: 521760985,
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
                name: "read-database-config",
            },
            step_command: XvcStepCommand {
                command: "echo /"Updated Database Configuration/"",
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
                    6,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: Some(
                                Yaml(
                                    Mapping {
                                        "timeout": Number(5000),
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: Some(
                                Yaml(
                                    Mapping {
                                        "timeout": Number(5000),
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    4,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: Some(
                                Yaml(
                                    Number(5432),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: Some(
                                Yaml(
                                    Number(9876),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    7,
                    7714117151203501825,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: Some(
                                Yaml(
                                    String("value"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: Some(
                                Yaml(
                                    String("value"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    5,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: Some(
                                Yaml(
                                    String("example.com"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: Some(
                                Yaml(
                                    String("example.com"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    8,
                    7714117151203501825,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: Some(
                                Yaml(
                                    Number(13),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: Some(
                                Yaml(
                                    Number(13),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
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
        4244321473927733562,
    ),
    step: XvcStep {
        name: "read-database-config",
    },
    step_command: XvcStepCommand {
        command: "echo /"Updated Database Configuration/"",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    3,
                    3369321840027960889,
                ): Running(
                    FromStartProcess,
                ),
                XvcEntity(
                    2,
                    4244321473927733562,
                ): WaitingToRun(
                    FromDiffsHasChanged,
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
                3,
                3369321840027960889,
            ): XvcStep {
                name: "read-hyperparams",
            },
            XvcEntity(
                2,
                4244321473927733562,
            ): XvcStep {
                name: "read-database-config",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    4244321473927733562,
                ): XvcStep {
                    name: "read-database-config",
                },
                XvcEntity(
                    3,
                    3369321840027960889,
                ): XvcStep {
                    name: "read-hyperparams",
                },
            },
            entity_index: {
                XvcStep {
                    name: "read-database-config",
                }: [
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                ],
                XvcStep {
                    name: "read-hyperparams",
                }: [
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        value: XvcStep {
                            name: "read-database-config",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        value: XvcStep {
                            name: "read-database-config",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            3369321840027960889,
                        ),
                        value: XvcStep {
                            name: "read-hyperparams",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            3369321840027960889,
                        ),
                        value: XvcStep {
                            name: "read-hyperparams",
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
                    4,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.port",
                        value: Some(
                            Yaml(
                                Number(5432),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    5,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.server",
                        value: Some(
                            Yaml(
                                String("example.com"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    6,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.connection",
                        value: Some(
                            Yaml(
                                Mapping {
                                    "timeout": Number(5000),
                                },
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    7,
                    7714117151203501825,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "param",
                        value: Some(
                            Yaml(
                                String("value"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    8,
                    7714117151203501825,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "numeric_param",
                        value: Some(
                            Yaml(
                                Number(13),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
            },
            entity_index: {
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.connection",
                        value: Some(
                            Yaml(
                                Mapping {
                                    "timeout": Number(5000),
                                },
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.port",
                        value: Some(
                            Yaml(
                                Number(5432),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.server",
                        value: Some(
                            Yaml(
                                String("example.com"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "numeric_param",
                        value: Some(
                            Yaml(
                                Number(13),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "param",
                        value: Some(
                            Yaml(
                                String("value"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.port",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.server",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.connection",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "param",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "numeric_param",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.server",
                                value: Some(
                                    Yaml(
                                        String("example.com"),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.port",
                                value: Some(
                                    Yaml(
                                        Number(5432),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.connection",
                                value: Some(
                                    Yaml(
                                        Mapping {
                                            "timeout": Number(5000),
                                        },
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "numeric_param",
                                value: Some(
                                    Yaml(
                                        Number(13),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "param",
                                value: Some(
                                    Yaml(
                                        String("value"),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
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
                    4,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    5,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    6,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    7,
                    7714117151203501825,
                ): ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    8,
                    7714117151203501825,
                ): ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                    XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                    XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                ],
                ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                    XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                3,
                                3369321840027960889,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                3,
                                3369321840027960889,
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
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-database-config"
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/command.rs::81] self.environment: {}
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-hyperparams"
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1521] params: StepStateParams {
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
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "core.guid": String(
                            "b699699fc944c69d",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "core.guid": String(
                            "e8b927ad14610598",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "e8b927ad14610598",
                    ),
                },
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
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
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.auto_stage": XvcConfigValue {
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
                "file.track.no_commit": XvcConfigValue {
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
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"b699699fc944c69d/"
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
            counter: 9,
            random: 18409169647092427842,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "myparams.toml",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    259,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042586,
                        tv_nsec: 730145224,
                    },
                ),
            },
            XvcPath(
                "myparams.yaml",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693046654,
                        tv_nsec: 465778895,
                    },
                ),
            },
            XvcPath(
                "myparams.json",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    162,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042629,
                        tv_nsec: 880895440,
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
                        tv_sec: 1693046651,
                        tv_nsec: 521695443,
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
                        tv_sec: 1693046651,
                        tv_nsec: 521760985,
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
                name: "read-hyperparams",
            },
            step_command: XvcStepCommand {
                command: "echo /"Update Hyperparameters/"",
            },
            birth: Some(
                Instant {
                    tv_sec: 1642812,
                    tv_nsec: 1948250,
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
                        pid: 72752,
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
                    6,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: Some(
                                Yaml(
                                    Mapping {
                                        "timeout": Number(5000),
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: Some(
                                Yaml(
                                    Mapping {
                                        "timeout": Number(5000),
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    4,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: Some(
                                Yaml(
                                    Number(5432),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: Some(
                                Yaml(
                                    Number(9876),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    7,
                    7714117151203501825,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: Some(
                                Yaml(
                                    String("value"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: Some(
                                Yaml(
                                    String("value"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    5,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: Some(
                                Yaml(
                                    String("example.com"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: Some(
                                Yaml(
                                    String("example.com"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    8,
                    7714117151203501825,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: Some(
                                Yaml(
                                    Number(13),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: Some(
                                Yaml(
                                    Number(13),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
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
        3,
        3369321840027960889,
    ),
    step: XvcStep {
        name: "read-hyperparams",
    },
    step_command: XvcStepCommand {
        command: "echo /"Update Hyperparameters/"",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    3,
                    3369321840027960889,
                ): Running(
                    FromWaitProcess,
                ),
                XvcEntity(
                    2,
                    4244321473927733562,
                ): Running(
                    FromStartProcess,
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
                3,
                3369321840027960889,
            ): XvcStep {
                name: "read-hyperparams",
            },
            XvcEntity(
                2,
                4244321473927733562,
            ): XvcStep {
                name: "read-database-config",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    4244321473927733562,
                ): XvcStep {
                    name: "read-database-config",
                },
                XvcEntity(
                    3,
                    3369321840027960889,
                ): XvcStep {
                    name: "read-hyperparams",
                },
            },
            entity_index: {
                XvcStep {
                    name: "read-database-config",
                }: [
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                ],
                XvcStep {
                    name: "read-hyperparams",
                }: [
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        value: XvcStep {
                            name: "read-database-config",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        value: XvcStep {
                            name: "read-database-config",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            3369321840027960889,
                        ),
                        value: XvcStep {
                            name: "read-hyperparams",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            3369321840027960889,
                        ),
                        value: XvcStep {
                            name: "read-hyperparams",
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
                    4,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.port",
                        value: Some(
                            Yaml(
                                Number(5432),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    5,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.server",
                        value: Some(
                            Yaml(
                                String("example.com"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    6,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.connection",
                        value: Some(
                            Yaml(
                                Mapping {
                                    "timeout": Number(5000),
                                },
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    7,
                    7714117151203501825,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "param",
                        value: Some(
                            Yaml(
                                String("value"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    8,
                    7714117151203501825,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "numeric_param",
                        value: Some(
                            Yaml(
                                Number(13),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
            },
            entity_index: {
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.connection",
                        value: Some(
                            Yaml(
                                Mapping {
                                    "timeout": Number(5000),
                                },
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.port",
                        value: Some(
                            Yaml(
                                Number(5432),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.server",
                        value: Some(
                            Yaml(
                                String("example.com"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "numeric_param",
                        value: Some(
                            Yaml(
                                Number(13),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "param",
                        value: Some(
                            Yaml(
                                String("value"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.port",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.server",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.connection",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "param",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "numeric_param",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.server",
                                value: Some(
                                    Yaml(
                                        String("example.com"),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.port",
                                value: Some(
                                    Yaml(
                                        Number(5432),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.connection",
                                value: Some(
                                    Yaml(
                                        Mapping {
                                            "timeout": Number(5000),
                                        },
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "numeric_param",
                                value: Some(
                                    Yaml(
                                        Number(13),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "param",
                                value: Some(
                                    Yaml(
                                        String("value"),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
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
                    4,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    5,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    6,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    7,
                    7714117151203501825,
                ): ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    8,
                    7714117151203501825,
                ): ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                    XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                    XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                ],
                ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                    XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                3,
                                3369321840027960889,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                3,
                                3369321840027960889,
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
[TRACE][pipeline/src/pipeline/mod.rs::1553] command_process: CommandProcess {
    environment: {},
    step: XvcStep {
        name: "read-hyperparams",
    },
    step_command: XvcStepCommand {
        command: "echo /"Update Hyperparameters/"",
    },
    birth: Some(
        Instant {
            tv_sec: 1642812,
            tv_nsec: 1948250,
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
                pid: 72752,
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
[OUT] [read-hyperparams] Update Hyperparameters
 
[TRACE][pipeline/src/pipeline/mod.rs::1559] &process: Popen {
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
        pid: 72752,
        ext: (),
    },
    detached: true,
}
[TRACE][pipeline/src/pipeline/mod.rs::1606] return_state: Some(
    DoneByRunning(
        FromProcessCompletedSuccessfully,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::1612] params: StepStateParams {
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
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "core.guid": String(
                            "b699699fc944c69d",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "core.guid": String(
                            "e8b927ad14610598",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "e8b927ad14610598",
                    ),
                },
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
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
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.auto_stage": XvcConfigValue {
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
                "file.track.no_commit": XvcConfigValue {
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
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"b699699fc944c69d/"
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
            counter: 9,
            random: 18409169647092427842,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "myparams.toml",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    259,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042586,
                        tv_nsec: 730145224,
                    },
                ),
            },
            XvcPath(
                "myparams.yaml",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693046654,
                        tv_nsec: 465778895,
                    },
                ),
            },
            XvcPath(
                "myparams.json",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    162,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042629,
                        tv_nsec: 880895440,
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
                        tv_sec: 1693046651,
                        tv_nsec: 521695443,
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
                        tv_sec: 1693046651,
                        tv_nsec: 521760985,
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
                name: "read-hyperparams",
            },
            step_command: XvcStepCommand {
                command: "echo /"Update Hyperparameters/"",
            },
            birth: Some(
                Instant {
                    tv_sec: 1642812,
                    tv_nsec: 1948250,
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
                    6,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: Some(
                                Yaml(
                                    Mapping {
                                        "timeout": Number(5000),
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: Some(
                                Yaml(
                                    Mapping {
                                        "timeout": Number(5000),
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    4,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: Some(
                                Yaml(
                                    Number(5432),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: Some(
                                Yaml(
                                    Number(9876),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    7,
                    7714117151203501825,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: Some(
                                Yaml(
                                    String("value"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: Some(
                                Yaml(
                                    String("value"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    5,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: Some(
                                Yaml(
                                    String("example.com"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: Some(
                                Yaml(
                                    String("example.com"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    8,
                    7714117151203501825,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: Some(
                                Yaml(
                                    Number(13),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: Some(
                                Yaml(
                                    Number(13),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
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
        3,
        3369321840027960889,
    ),
    step: XvcStep {
        name: "read-hyperparams",
    },
    step_command: XvcStepCommand {
        command: "echo /"Update Hyperparameters/"",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    3,
                    3369321840027960889,
                ): Running(
                    FromWaitProcess,
                ),
                XvcEntity(
                    2,
                    4244321473927733562,
                ): Running(
                    FromStartProcess,
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
                3,
                3369321840027960889,
            ): XvcStep {
                name: "read-hyperparams",
            },
            XvcEntity(
                2,
                4244321473927733562,
            ): XvcStep {
                name: "read-database-config",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    4244321473927733562,
                ): XvcStep {
                    name: "read-database-config",
                },
                XvcEntity(
                    3,
                    3369321840027960889,
                ): XvcStep {
                    name: "read-hyperparams",
                },
            },
            entity_index: {
                XvcStep {
                    name: "read-database-config",
                }: [
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                ],
                XvcStep {
                    name: "read-hyperparams",
                }: [
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        value: XvcStep {
                            name: "read-database-config",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        value: XvcStep {
                            name: "read-database-config",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            3369321840027960889,
                        ),
                        value: XvcStep {
                            name: "read-hyperparams",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            3369321840027960889,
                        ),
                        value: XvcStep {
                            name: "read-hyperparams",
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
                    4,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.port",
                        value: Some(
                            Yaml(
                                Number(5432),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    5,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.server",
                        value: Some(
                            Yaml(
                                String("example.com"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    6,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.connection",
                        value: Some(
                            Yaml(
                                Mapping {
                                    "timeout": Number(5000),
                                },
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    7,
                    7714117151203501825,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "param",
                        value: Some(
                            Yaml(
                                String("value"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    8,
                    7714117151203501825,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "numeric_param",
                        value: Some(
                            Yaml(
                                Number(13),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
            },
            entity_index: {
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.connection",
                        value: Some(
                            Yaml(
                                Mapping {
                                    "timeout": Number(5000),
                                },
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.port",
                        value: Some(
                            Yaml(
                                Number(5432),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.server",
                        value: Some(
                            Yaml(
                                String("example.com"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "numeric_param",
                        value: Some(
                            Yaml(
                                Number(13),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "param",
                        value: Some(
                            Yaml(
                                String("value"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.port",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.server",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.connection",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "param",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "numeric_param",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.server",
                                value: Some(
                                    Yaml(
                                        String("example.com"),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.port",
                                value: Some(
                                    Yaml(
                                        Number(5432),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.connection",
                                value: Some(
                                    Yaml(
                                        Mapping {
                                            "timeout": Number(5000),
                                        },
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "numeric_param",
                                value: Some(
                                    Yaml(
                                        Number(13),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "param",
                                value: Some(
                                    Yaml(
                                        String("value"),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
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
                    4,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    5,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    6,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    7,
                    7714117151203501825,
                ): ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    8,
                    7714117151203501825,
                ): ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                    XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                    XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                ],
                ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                    XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                3,
                                3369321840027960889,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                3,
                                3369321840027960889,
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
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-hyperparams"
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-database-config"
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::1521] params: StepStateParams {
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
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "core.guid": String(
                            "b699699fc944c69d",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "core.guid": String(
                            "e8b927ad14610598",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "e8b927ad14610598",
                    ),
                },
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
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
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.auto_stage": XvcConfigValue {
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
                "file.track.no_commit": XvcConfigValue {
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
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"b699699fc944c69d/"
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
            counter: 9,
            random: 18409169647092427842,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "myparams.toml",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    259,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042586,
                        tv_nsec: 730145224,
                    },
                ),
            },
            XvcPath(
                "myparams.yaml",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693046654,
                        tv_nsec: 465778895,
                    },
                ),
            },
            XvcPath(
                "myparams.json",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    162,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042629,
                        tv_nsec: 880895440,
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
                        tv_sec: 1693046651,
                        tv_nsec: 521695443,
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
                        tv_sec: 1693046651,
                        tv_nsec: 521760985,
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
                name: "read-database-config",
            },
            step_command: XvcStepCommand {
                command: "echo /"Updated Database Configuration/"",
            },
            birth: Some(
                Instant {
                    tv_sec: 1642812,
                    tv_nsec: 10312583,
                },
            ),
            process: Some(
                Popen {
                    stdin: None,
                    stdout: Some(
                        File {
                            fd: 10,
                            read: true,
                            write: false,
                        },
                    ),
                    stderr: Some(
                        File {
                            fd: 12,
                            read: true,
                            write: false,
                        },
                    ),
                    child_state: Running {
                        pid: 72753,
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
                    6,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: Some(
                                Yaml(
                                    Mapping {
                                        "timeout": Number(5000),
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: Some(
                                Yaml(
                                    Mapping {
                                        "timeout": Number(5000),
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    4,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: Some(
                                Yaml(
                                    Number(5432),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: Some(
                                Yaml(
                                    Number(9876),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    7,
                    7714117151203501825,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: Some(
                                Yaml(
                                    String("value"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: Some(
                                Yaml(
                                    String("value"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    5,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: Some(
                                Yaml(
                                    String("example.com"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: Some(
                                Yaml(
                                    String("example.com"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    8,
                    7714117151203501825,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: Some(
                                Yaml(
                                    Number(13),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: Some(
                                Yaml(
                                    Number(13),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
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
        4244321473927733562,
    ),
    step: XvcStep {
        name: "read-database-config",
    },
    step_command: XvcStepCommand {
        command: "echo /"Updated Database Configuration/"",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    3,
                    3369321840027960889,
                ): Running(
                    FromWaitProcess,
                ),
                XvcEntity(
                    2,
                    4244321473927733562,
                ): Running(
                    FromStartProcess,
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
                3,
                3369321840027960889,
            ): XvcStep {
                name: "read-hyperparams",
            },
            XvcEntity(
                2,
                4244321473927733562,
            ): XvcStep {
                name: "read-database-config",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    4244321473927733562,
                ): XvcStep {
                    name: "read-database-config",
                },
                XvcEntity(
                    3,
                    3369321840027960889,
                ): XvcStep {
                    name: "read-hyperparams",
                },
            },
            entity_index: {
                XvcStep {
                    name: "read-database-config",
                }: [
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                ],
                XvcStep {
                    name: "read-hyperparams",
                }: [
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        value: XvcStep {
                            name: "read-database-config",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        value: XvcStep {
                            name: "read-database-config",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            3369321840027960889,
                        ),
                        value: XvcStep {
                            name: "read-hyperparams",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            3369321840027960889,
                        ),
                        value: XvcStep {
                            name: "read-hyperparams",
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
                    4,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.port",
                        value: Some(
                            Yaml(
                                Number(5432),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    5,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.server",
                        value: Some(
                            Yaml(
                                String("example.com"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    6,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.connection",
                        value: Some(
                            Yaml(
                                Mapping {
                                    "timeout": Number(5000),
                                },
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    7,
                    7714117151203501825,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "param",
                        value: Some(
                            Yaml(
                                String("value"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    8,
                    7714117151203501825,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "numeric_param",
                        value: Some(
                            Yaml(
                                Number(13),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
            },
            entity_index: {
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.connection",
                        value: Some(
                            Yaml(
                                Mapping {
                                    "timeout": Number(5000),
                                },
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.port",
                        value: Some(
                            Yaml(
                                Number(5432),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.server",
                        value: Some(
                            Yaml(
                                String("example.com"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "numeric_param",
                        value: Some(
                            Yaml(
                                Number(13),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "param",
                        value: Some(
                            Yaml(
                                String("value"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.port",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.server",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.connection",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "param",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "numeric_param",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.server",
                                value: Some(
                                    Yaml(
                                        String("example.com"),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.port",
                                value: Some(
                                    Yaml(
                                        Number(5432),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.connection",
                                value: Some(
                                    Yaml(
                                        Mapping {
                                            "timeout": Number(5000),
                                        },
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "numeric_param",
                                value: Some(
                                    Yaml(
                                        Number(13),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "param",
                                value: Some(
                                    Yaml(
                                        String("value"),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
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
                    4,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    5,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    6,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    7,
                    7714117151203501825,
                ): ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    8,
                    7714117151203501825,
                ): ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                    XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                    XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                ],
                ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                    XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                3,
                                3369321840027960889,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                3,
                                3369321840027960889,
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
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1553] command_process: CommandProcess {
    environment: {},
    step: XvcStep {
        name: "read-database-config",
    },
    step_command: XvcStepCommand {
        command: "echo /"Updated Database Configuration/"",
    },
    birth: Some(
        Instant {
            tv_sec: 1642812,
            tv_nsec: 10312583,
        },
    ),
    process: Some(
        Popen {
            stdin: None,
            stdout: Some(
                File {
                    fd: 10,
                    read: true,
                    write: false,
                },
            ),
            stderr: Some(
                File {
                    fd: 12,
                    read: true,
                    write: false,
                },
            ),
            child_state: Running {
                pid: 72753,
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
[OUT] [read-database-config] Updated Database Configuration
 
[TRACE][pipeline/src/pipeline/mod.rs::1559] &process: Popen {
    stdin: None,
    stdout: Some(
        File {
            fd: 10,
            read: true,
            write: false,
        },
    ),
    stderr: Some(
        File {
            fd: 12,
            read: true,
            write: false,
        },
    ),
    child_state: Running {
        pid: 72753,
        ext: (),
    },
    detached: true,
}
[TRACE][pipeline/src/pipeline/mod.rs::1606] return_state: Some(
    DoneByRunning(
        FromProcessCompletedSuccessfully,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::1612] params: StepStateParams {
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
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "core.guid": String(
                            "b699699fc944c69d",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "core.guid": String(
                            "e8b927ad14610598",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "e8b927ad14610598",
                    ),
                },
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
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
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.auto_stage": XvcConfigValue {
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
                "file.track.no_commit": XvcConfigValue {
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
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"b699699fc944c69d/"
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
            counter: 9,
            random: 18409169647092427842,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "myparams.toml",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    259,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042586,
                        tv_nsec: 730145224,
                    },
                ),
            },
            XvcPath(
                "myparams.yaml",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    108,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693046654,
                        tv_nsec: 465778895,
                    },
                ),
            },
            XvcPath(
                "myparams.json",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    162,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1693042629,
                        tv_nsec: 880895440,
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
                        tv_sec: 1693046651,
                        tv_nsec: 521695443,
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
                        tv_sec: 1693046651,
                        tv_nsec: 521760985,
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
                name: "read-database-config",
            },
            step_command: XvcStepCommand {
                command: "echo /"Updated Database Configuration/"",
            },
            birth: Some(
                Instant {
                    tv_sec: 1642812,
                    tv_nsec: 10312583,
                },
            ),
            process: Some(
                Popen {
                    stdin: None,
                    stdout: Some(
                        File {
                            fd: 10,
                            read: true,
                            write: false,
                        },
                    ),
                    stderr: Some(
                        File {
                            fd: 12,
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
                    6,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: Some(
                                Yaml(
                                    Mapping {
                                        "timeout": Number(5000),
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.connection",
                            value: Some(
                                Yaml(
                                    Mapping {
                                        "timeout": Number(5000),
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    4,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: Some(
                                Yaml(
                                    Number(5432),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.port",
                            value: Some(
                                Yaml(
                                    Number(9876),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    7,
                    7714117151203501825,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: Some(
                                Yaml(
                                    String("value"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "param",
                            value: Some(
                                Yaml(
                                    String("value"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    5,
                    2200857687599595040,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: Some(
                                Yaml(
                                    String("example.com"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "database.server",
                            value: Some(
                                Yaml(
                                    String("example.com"),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                XvcEntity(
                    8,
                    7714117151203501825,
                ): Different {
                    record: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: Some(
                                Yaml(
                                    Number(13),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693042660,
                                            tv_nsec: 806716570,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    actual: Param(
                        ParamDep {
                            format: YAML,
                            path: XvcPath(
                                "myparams.yaml",
                            ),
                            key: "numeric_param",
                            value: Some(
                                Yaml(
                                    Number(13),
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        108,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1693046654,
                                            tv_nsec: 465778895,
                                        },
                                    ),
                                },
                            ),
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
        4244321473927733562,
    ),
    step: XvcStep {
        name: "read-database-config",
    },
    step_command: XvcStepCommand {
        command: "echo /"Updated Database Configuration/"",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    3,
                    3369321840027960889,
                ): DoneByRunning(
                    FromProcessCompletedSuccessfully,
                ),
                XvcEntity(
                    2,
                    4244321473927733562,
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
                3,
                3369321840027960889,
            ): XvcStep {
                name: "read-hyperparams",
            },
            XvcEntity(
                2,
                4244321473927733562,
            ): XvcStep {
                name: "read-database-config",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    4244321473927733562,
                ): XvcStep {
                    name: "read-database-config",
                },
                XvcEntity(
                    3,
                    3369321840027960889,
                ): XvcStep {
                    name: "read-hyperparams",
                },
            },
            entity_index: {
                XvcStep {
                    name: "read-database-config",
                }: [
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                ],
                XvcStep {
                    name: "read-hyperparams",
                }: [
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        value: XvcStep {
                            name: "read-database-config",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            4244321473927733562,
                        ),
                        value: XvcStep {
                            name: "read-database-config",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            3369321840027960889,
                        ),
                        value: XvcStep {
                            name: "read-hyperparams",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            3369321840027960889,
                        ),
                        value: XvcStep {
                            name: "read-hyperparams",
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
                    4,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.port",
                        value: Some(
                            Yaml(
                                Number(5432),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    5,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.server",
                        value: Some(
                            Yaml(
                                String("example.com"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    6,
                    2200857687599595040,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.connection",
                        value: Some(
                            Yaml(
                                Mapping {
                                    "timeout": Number(5000),
                                },
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    7,
                    7714117151203501825,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "param",
                        value: Some(
                            Yaml(
                                String("value"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
                XvcEntity(
                    8,
                    7714117151203501825,
                ): Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "numeric_param",
                        value: Some(
                            Yaml(
                                Number(13),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ),
            },
            entity_index: {
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.connection",
                        value: Some(
                            Yaml(
                                Mapping {
                                    "timeout": Number(5000),
                                },
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.port",
                        value: Some(
                            Yaml(
                                Number(5432),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "database.server",
                        value: Some(
                            Yaml(
                                String("example.com"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "numeric_param",
                        value: Some(
                            Yaml(
                                Number(13),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                ],
                Param(
                    ParamDep {
                        format: YAML,
                        path: XvcPath(
                            "myparams.yaml",
                        ),
                        key: "param",
                        value: Some(
                            Yaml(
                                String("value"),
                            ),
                        ),
                        xvc_metadata: Some(
                            XvcMetadata {
                                file_type: File,
                                size: Some(
                                    108,
                                ),
                                modified: Some(
                                    SystemTime {
                                        tv_sec: 1693042660,
                                        tv_nsec: 806716570,
                                    },
                                ),
                            },
                        ),
                    },
                ): [
                    XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.port",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.server",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.connection",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "param",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "numeric_param",
                                value: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.server",
                                value: Some(
                                    Yaml(
                                        String("example.com"),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.port",
                                value: Some(
                                    Yaml(
                                        Number(5432),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "database.connection",
                                value: Some(
                                    Yaml(
                                        Mapping {
                                            "timeout": Number(5000),
                                        },
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "numeric_param",
                                value: Some(
                                    Yaml(
                                        Number(13),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: Param(
                            ParamDep {
                                format: YAML,
                                path: XvcPath(
                                    "myparams.yaml",
                                ),
                                key: "param",
                                value: Some(
                                    Yaml(
                                        String("value"),
                                    ),
                                ),
                                xvc_metadata: Some(
                                    XvcMetadata {
                                        file_type: File,
                                        size: Some(
                                            108,
                                        ),
                                        modified: Some(
                                            SystemTime {
                                                tv_sec: 1693042660,
                                                tv_nsec: 806716570,
                                            },
                                        ),
                                    },
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
                    4,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    5,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    6,
                    2200857687599595040,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    7,
                    7714117151203501825,
                ): ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    8,
                    7714117151203501825,
                ): ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        4244321473927733562,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        4,
                        2200857687599595040,
                    ),
                    XvcEntity(
                        5,
                        2200857687599595040,
                    ),
                    XvcEntity(
                        6,
                        2200857687599595040,
                    ),
                ],
                ChildEntity(
                    XvcEntity(
                        3,
                        3369321840027960889,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        7,
                        7714117151203501825,
                    ),
                    XvcEntity(
                        8,
                        7714117151203501825,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            4,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            6,
                            2200857687599595040,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                4244321473927733562,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            7714117151203501825,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                3,
                                3369321840027960889,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            7714117151203501825,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                3,
                                3369321840027960889,
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
[TRACE][pipeline/src/pipeline/mod.rs::822] step.name: "read-database-config"
[TRACE][pipeline/src/pipeline/mod.rs::823] &r_next_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::825] &step_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::712] &step_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::632] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::555] (step_e, &jh): (
    XvcEntity(
        3,
        3369321840027960889,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::562] "Before state updater": "Before state updater"
[TRACE][pipeline/src/pipeline/mod.rs::573] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                3,
                3369321840027960889,
            ): DoneByRunning(
                FromProcessCompletedSuccessfully,
            ),
            XvcEntity(
                2,
                4244321473927733562,
            ): DoneByRunning(
                FromProcessCompletedSuccessfully,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::580] done_successfully: Ok(
    true,
)
[TRACE][pipeline/src/pipeline/mod.rs::595] output_diffs: HStore {
    map: {},
}
[TRACE][pipeline/src/pipeline/mod.rs::596] store: XvcStore {
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
[TRACE][lib/src/cli/mod.rs::582] git_add_output: "add '.xvc/store/xvc-dependency-store/1693046655732589.json'
"
[TRACE][lib/src/cli/mod.rs::433] args: [
    "-C",
    "[CWD]",
    "commit",
    "-m",
    "Xvc auto-commit after /'/Users/iex/github.com/iesahin/xvc/target/debug/xvc --debug pipeline run/'",
]

```
