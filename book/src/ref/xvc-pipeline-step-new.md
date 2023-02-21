# xvc pipeline step new

## Purpose

Create a new step in the pipeline.

## Synopsis

```console
$ xvc pipeline step new --help
Add a new step

Usage: xvc pipeline step new [OPTIONS] --step-name <STEP_NAME> --command <COMMAND>

Options:
  -s, --step-name <STEP_NAME>  Name of the new step
  -c, --command <COMMAND>      Step command to run
      --when <WHEN>            When to run the command. One of always, never, by_dependencies (default). This is used to freeze or invalidate a step manually
  -h, --help                   Print help

```

## Examples

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

You can create a new step with a name and a command.

```console
$ xvc pipeline step new --step-name hello --command "echo hello"
```

By default a step will run only if its dependencies have changed. (`--when by_dependencies`).

If you want to run the command always, regardless of the changes in dependencies, you can set `--when` to `always`.

```console
$ xvc pipeline step new --step-name world --command "echo world" --when always
```

If you want a step to never run, you can set `--when` to `never`.

```console
$ xvc pipeline step new --step-name never --command "echo never" --when never
```

You can update when the step will run with [`xvc pipeline step update`](/ref/xvc-pipeline-step-update.md).

You can get the list of steps in the pipeline with `export` or `dag`.

```console
$ xvc pipeline export
{
  "name": "default",
  "steps": [
    {
      "command": "echo hello",
      "dependencies": [],
      "invalidate": "ByDependencies",
      "name": "hello",
      "outputs": []
    },
    {
      "command": "echo world",
      "dependencies": [],
      "invalidate": "Always",
      "name": "world",
      "outputs": []
    },
    {
      "command": "echo never",
      "dependencies": [],
      "invalidate": "Never",
      "name": "never",
      "outputs": []
    }
  ],
  "version": 1,
  "workdir": ""
}

$ xvc -vvvv pipeline dag --format mermaid
[DEBUG][logging/src/lib.rs::236] Terminal logger enabled with level: Trace
[TRACE][core/src/types/xvcroot.rs::247] "."
[DEBUG][core/src/types/xvcroot.rs::253] XVC DIR: "[CWD]"
[DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][config/src/lib.rs::536] cli_config: [
    "core.verbosity = debug",
    "core.quiet = false",
]
[TRACE][config/src/lib.rs::540] map: {
    "core.verbosity": String(
        "debug",
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
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "git.command": String(
                    "git",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "core.guid": String(
                    "48968af62abe9fe9",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.sort": String(
                    "name-desc",
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
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.guid": String(
                    "70f65f1f36672a0d",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "core.verbosity": String(
                    "error",
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
                    "debug",
                ),
                "core.quiet": Boolean(
                    false,
                ),
            },
        },
    ],
    the_config: {
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
        "git.auto_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "file.carry-in.force": XvcConfigValue {
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
        "file.track.force": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "debug",
            ),
        },
        "file.track.text_or_binary": XvcConfigValue {
            source: Project,
            value: String(
                "auto",
            ),
        },
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
            ),
        },
        "pipeline.current_pipeline": XvcConfigValue {
            source: Project,
            value: String(
                "default",
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "70f65f1f36672a0d",
            ),
        },
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
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
        "file.track.no_parallel": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "file.list.no_summary": XvcConfigValue {
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
        "core.quiet": XvcConfigValue {
            source: CommandLine,
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
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"48968af62abe9fe9/"
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
                "core.verbosity = debug",
                "core.quiet = false",
            ],
        ),
    },
}
[TRACE][ecs/src/ecs/mod.rs::229] dir: "[CWD]/.xvc/ec"
[TRACE][ecs/src/ecs/mod.rs::239] files: [
    "[CWD]/.xvc/ec/1676969101881988",
    "[CWD]/.xvc/ec/1676969101884839",
    "[CWD]/.xvc/ec/1676969101970702",
    "[CWD]/.xvc/ec/1676969102051943",
    "[CWD]/.xvc/ec/1676969102286398",
]
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][core/src/types/xvcpath.rs::83] abs_path: "[CWD]/.gitignore"
[TRACE][core/src/types/xvcpath.rs::84] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::85] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::83] abs_path: "[CWD]/.xvcignore"
[TRACE][core/src/types/xvcpath.rs::84] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::85] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][pipeline/src/pipeline/api/dag.rs::176] pipeline_steps: HStore {
    map: {
        XvcEntity(
            2,
            12607682588247016128,
        ): XvcStep {
            name: "hello",
        },
        XvcEntity(
            4,
            1578801984158423751,
        ): XvcStep {
            name: "never",
        },
        XvcEntity(
            3,
            4414197723370443061,
        ): XvcStep {
            name: "world",
        },
        XvcEntity(
            0,
            0,
        ): XvcStep {
            name: "START",
        },
        XvcEntity(
            18446744073709551615,
            0,
        ): XvcStep {
            name: "END",
        },
    },
}
[TRACE][pipeline/src/pipeline/api/dag.rs::188] dependency_graph: {
    XvcEntity(
        0,
        0,
    ): [
        (
            XvcEntity(
                2,
                12607682588247016128,
            ),
            Outgoing,
        ),
        (
            XvcEntity(
                4,
                1578801984158423751,
            ),
            Outgoing,
        ),
        (
            XvcEntity(
                3,
                4414197723370443061,
            ),
            Outgoing,
        ),
    ],
    XvcEntity(
        2,
        12607682588247016128,
    ): [
        (
            XvcEntity(
                0,
                0,
            ),
            Incoming,
        ),
        (
            XvcEntity(
                18446744073709551615,
                0,
            ),
            Outgoing,
        ),
    ],
    XvcEntity(
        18446744073709551615,
        0,
    ): [
        (
            XvcEntity(
                2,
                12607682588247016128,
            ),
            Incoming,
        ),
        (
            XvcEntity(
                4,
                1578801984158423751,
            ),
            Incoming,
        ),
        (
            XvcEntity(
                3,
                4414197723370443061,
            ),
            Incoming,
        ),
    ],
    XvcEntity(
        4,
        1578801984158423751,
    ): [
        (
            XvcEntity(
                0,
                0,
            ),
            Incoming,
        ),
        (
            XvcEntity(
                18446744073709551615,
                0,
            ),
            Outgoing,
        ),
    ],
    XvcEntity(
        3,
        4414197723370443061,
    ): [
        (
            XvcEntity(
                0,
                0,
            ),
            Incoming,
        ),
        (
            XvcEntity(
                18446744073709551615,
                0,
            ),
            Outgoing,
        ),
    ],
}
[TRACE][pipeline/src/pipeline/api/dag.rs::49] step_e: XvcEntity(
    2,
    12607682588247016128,
)
[TRACE][pipeline/src/pipeline/api/dag.rs::49] start_e: XvcEntity(
    0,
    0,
)
[TRACE][pipeline/src/pipeline/api/dag.rs::49] end_e: XvcEntity(
    18446744073709551615,
    0,
)
[TRACE][pipeline/src/pipeline/api/dag.rs::49] step_e: XvcEntity(
    4,
    1578801984158423751,
)
[TRACE][pipeline/src/pipeline/api/dag.rs::49] start_e: XvcEntity(
    0,
    0,
)
[TRACE][pipeline/src/pipeline/api/dag.rs::49] end_e: XvcEntity(
    18446744073709551615,
    0,
)
[TRACE][pipeline/src/pipeline/api/dag.rs::49] step_e: XvcEntity(
    3,
    4414197723370443061,
)
[TRACE][pipeline/src/pipeline/api/dag.rs::49] start_e: XvcEntity(
    0,
    0,
)
[TRACE][pipeline/src/pipeline/api/dag.rs::49] end_e: XvcEntity(
    18446744073709551615,
    0,
)
[TRACE][pipeline/src/pipeline/api/dag.rs::49] step_e: XvcEntity(
    0,
    0,
)
[TRACE][pipeline/src/pipeline/api/dag.rs::49] start_e: XvcEntity(
    0,
    0,
)
[TRACE][pipeline/src/pipeline/api/dag.rs::49] end_e: XvcEntity(
    18446744073709551615,
    0,
)
[TRACE][pipeline/src/pipeline/api/dag.rs::49] step_e: XvcEntity(
    18446744073709551615,
    0,
)
[TRACE][pipeline/src/pipeline/api/dag.rs::49] start_e: XvcEntity(
    0,
    0,
)
[TRACE][pipeline/src/pipeline/api/dag.rs::49] end_e: XvcEntity(
    18446744073709551615,
    0,
)
graph TD
	step:_START_always__[step: START (always, )] --> |step: hello (by_dependencies, echo hello)|step:_hello_by_dependencies__echo_hello[step: hello (by_dependencies, echo hello)]
	step:_START_always__[step: START (always, )] --> |step: never (never, echo never)|step:_never_never__echo_never[step: never (never, echo never)]
	step:_START_always__[step: START (always, )] --> |step: world (always, echo world)|step:_world_always__echo_world[step: world (always, echo world)]
	step:_hello_by_dependencies__echo_hello[step: hello (by_dependencies, echo hello)] --> |step: END (never, )|step:_END_never__[step: END (never, )]
	step:_never_never__echo_never[step: never (never, echo never)] --> |step: END (never, )|step:_END_never__[step: END (never, )]
	step:_world_always__echo_world[step: world (always, echo world)] --> |step: END (never, )|step:_END_never__[step: END (never, )]

[TRACE][lib/src/cli/mod.rs::381] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::384] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline dag --format mermaid"
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
[DEBUG] No files to commit
[DEBUG] Command completed successfully.

```
