# xvc pipeline run

## Synopsis

```console
$ xvc pipeline run --help
Run a pipeline

Usage: xvc pipeline run [OPTIONS]

Options:
  -n, --name <NAME>  Name of the pipeline to run
  -h, --help         Print help

```

## Examples

Pipelines require Xvc to be initialized before running.

```console
$ git init
...
$ xvc init
```

Xvc defines a default pipeline and any steps added without specifying the pipeline will be added to it.

```console
$ xvc pipeline list
+---------+---------+
| Name    | Run Dir |
+===================+
| default |         |
+---------+---------+

```

Create a new step in this pipeline with [`xvc pipeline step new`](/ref/xvc-pipeline-step-new.md) command.

```console
$ xvc pipeline step new --step-name hello --command "echo hello"
```

```console
$ xvc pipeline dag
digraph {
    0 [ label = "step: START (always, )" ]
    1 [ label = "step: hello (by_dependencies, echo hello)" ]
    2 [ label = "step: END (never, )" ]
    0 -> 1 [ label = "" ]
    1 -> 2 [ label = "" ]
}


```


You can run the default pipeline without specifying its name.

```console
$ xvc -vvvvv pipeline run
[DEBUG][logging/src/lib.rs::236] Terminal logger enabled with level: Trace
[TRACE][core/src/types/xvcroot.rs::247] "."
[DEBUG][core/src/types/xvcroot.rs::253] XVC DIR: "[CWD]"
[DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][config/src/lib.rs::536] cli_config: [
    "core.verbosity = trace",
    "core.quiet = false",
]
[TRACE][config/src/lib.rs::540] map: {
    "core.quiet": Boolean(
        false,
    ),
    "core.verbosity": String(
        "trace",
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
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "core.guid": String(
                    "e72352e77a756c0c",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "file.track.force": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "git.command": String(
                    "git",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "core.guid": String(
                    "383f12a03ca53df3",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.no_parallel": Boolean(
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
                    "trace",
                ),
            },
        },
    ],
    the_config: {
        "file.list.no_summary": XvcConfigValue {
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
        "file.track.no_parallel": XvcConfigValue {
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
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "file.list.recursive": XvcConfigValue {
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
        "file.carry-in.no_parallel": XvcConfigValue {
            source: Project,
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
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "trace",
            ),
        },
        "git.auto_stage": XvcConfigValue {
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
        "file.recheck.method": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
            ),
        },
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
            ),
        },
        "pipeline.default": XvcConfigValue {
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
        "file.track.no_commit": XvcConfigValue {
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
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
            ),
        },
        "file.track.text_or_binary": XvcConfigValue {
            source: Project,
            value: String(
                "auto",
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "383f12a03ca53df3",
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
guid = /"e72352e77a756c0c/"
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
                "core.verbosity = trace",
                "core.quiet = false",
            ],
        ),
    },
}
[TRACE][ecs/src/ecs/mod.rs::229] dir: "[CWD]/.xvc/ec"
[TRACE][ecs/src/ecs/mod.rs::239] files: [
    "[CWD]/.xvc/ec/1678959758660121",
    "[CWD]/.xvc/ec/1678959758665974",
    "[CWD]/.xvc/ec/1678959758934720",
]
[TRACE][pipeline/src/pipeline/api/run.rs::19] pipeline_name: "default"
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/.gitignore"
[TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/.xvcignore"
[TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/notify.rs::160] watcher: FsEventWatcher {
    paths: 0x0000000127204a10,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000000127204900,
    runloop: Some(
        (
            0x000000012361b3a0,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::256] pipeline_len: 1
[TRACE][pipeline/src/pipeline/mod.rs::272] &dependency_graph: {}
[INFO][pipeline/src/pipeline/mod.rs::285] Pipeline Graph:
digraph {
}


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
            "[CWD]/.xvc/store/content-digest-store",
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
            "[CWD]/.xvc/store/collection-digest-store",
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
[TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/.xvc/store/xvc-path-store"
[TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/.xvc/store/xvc-path-xvc-dependency-r1n-store"
[TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/.xvc/store/xvc-metadata-store"
[TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/.xvc/store/xvc-digests-store"
[TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/.xvc/store/content-digest-store"
[TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/.xvc/store/collection-digest-store"
[TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/.xvc/store/text-or-binary-store"
[TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][lib/src/cli/mod.rs::381] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::384] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvvv pipeline run"
[DEBUG] Using Git: /opt/homebrew/bin/git
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
[DEBUG] No files to commit
[DEBUG] Command completed successfully.

```
