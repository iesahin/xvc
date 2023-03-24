# xvc pipeline step dependency

## Purpose

Define a dependency to an existing step in the pipeline.

## Synopsis

```console
$ xvc pipeline step dependency --help
Add a dependency to a step

Usage: xvc pipeline step dependency [OPTIONS] --step-name <STEP_NAME>

Options:
  -s, --step-name <STEP_NAME>    Name of the step to add the dependency to
      --generic <GENERICS>       Add a generic command output as a dependency. Can be used multiple times. Please delimit the command with ' ' to avoid shell expansion
      --url <URLS>               Add a URL dependency to the step. Can be used multiple times
      --file <FILES>             Add a file dependency to the step. Can be used multiple times
      --step <STEPS>             Add a step dependency to a step. Can be used multiple times. Steps are referred with their names
      --directory <DIRECTORIES>  Add a directory dependency to the step. Can be used multiple times
      --glob <GLOBS>             Add a glob dependency to the step. Can be used multiple times
      --param <PARAMS>           Add a parameter dependency to the step in the form filename.yaml::model.units . Can be used multiple times
      --regex <REGEXPS>          Add a regex dependency in the form filename.txt:/^regex/ . Can be used multiple times
      --line <LINES>             Add a line dependency in the form filename.txt::123-234
  -h, --help                     Print help

```

## Examples

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Begin by adding a new step.

```console
$ xvc pipeline step new --step-name file-dependency --command "echo data.txt has changed"
```

Add a file dependency to the step.

```console
$ xvc pipeline step dependency --step-name file-dependency --file data.txt
```

When you run the command, it will print `data.txt has changed` if the file `data.txt` has changed.

```console
$ xvc pipeline run

```

You can add multiple dependencies to a step with multiple invocations.

```console
$ xvc pipeline step dependency --step-name file-dependency --file data2.txt
```

A step will run if any of its dependencies have changed.

```console
$ xvc pipeline run

```

Normally, they are not run if none of the dependencies have changed.

```console
$ xvc pipeline run

```

However, if you want to run the step even if none of the dependencies have changed, you can set the `--when` option to `always`.

```console
$ xvc pipeline step update --step-name file-dependency --when always
```

Now the step will run even if none of the dependencies have changed.

```console
$ xvc pipeline run
[OUT] data.txt has changed

[OUT] [EXIT] Successfully

```

### Step Dependencies

You can add a step dependency to a step. These steps specify dependency relationships explicitly, without relying on
changed files or directories.

```console
$ xvc pipeline step new --step-name world --command "echo world"
$ xvc pipeline step new --step-name hello --command "echo hello"
$ xvc pipeline step dependency --step-name world --step hello
$ xvc pipeline step dependency --step-name hello --step file-dependency
```

When run, the dependency will be run first and the step will be run after.

```console
$ xvc pipeline run
[OUT] data.txt has changed

[OUT] [EXIT] Successfully

```

### Generic Command Dependencies

You can use the output of a command as a dependency to a step. When the command is run, the output hash is saved to
compare and to invalidate the step when the output has changed.

You can use this for any command that outputs a string.

```console
$ xvc pipeline new --name generic

$ xvc pipeline --name generic step new --step-name yearly --command "echo 'Happy New Year! Welcome `(date +%Y)`!'"

$ xvc  pipeline --name generic step dependency --step-name yearly --generic 'date +%Y'

```

```console
$ xvc pipeline --name generic export
{
  "name": "generic",
  "steps": [
    {
      "command": "echo 'Happy New Year! Welcome `(date +%Y)`!'",
      "dependencies": [
        {
          "Generic": {
            "generic_command": "date +%Y"
          }
        }
      ],
      "invalidate": "ByDependencies",
      "name": "yearly",
      "outputs": []
    }
  ],
  "version": 1,
  "workdir": ""
}

```

When the year changes, the step is invalidated and run again.

```console
$ xvc pipeline --name generic run
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
    "core.quiet": Boolean(
        false,
    ),
    "core.verbosity": String(
        "debug",
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
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "core.guid": String(
                    "4e1b2b04f256091c",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.command": String(
                    "git",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
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
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.guid": String(
                    "847ba0dff8fb492c",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.command": String(
                    "git",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.no_summary": Boolean(
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
                    "debug",
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
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "debug",
            ),
        },
        "file.track.no_parallel": XvcConfigValue {
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
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
            ),
        },
        "file.carry-in.force": XvcConfigValue {
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
        "git.auto_stage": XvcConfigValue {
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
        "core.quiet": XvcConfigValue {
            source: CommandLine,
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
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
            ),
        },
        "file.carry-in.no_parallel": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "847ba0dff8fb492c",
            ),
        },
        "file.track.no_commit": XvcConfigValue {
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
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
            ),
        },
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
            ),
        },
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"4e1b2b04f256091c/"
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
    "[CWD]/.xvc/ec/1679695685169289",
    "[CWD]/.xvc/ec/1679695685172420",
    "[CWD]/.xvc/ec/1679695685341733",
    "[CWD]/.xvc/ec/1679695685454724",
    "[CWD]/.xvc/ec/1679695686605799",
    "[CWD]/.xvc/ec/1679695688355824",
    "[CWD]/.xvc/ec/1679695688434820",
    "[CWD]/.xvc/ec/1679695688515293",
    "[CWD]/.xvc/ec/1679695688597331",
    "[CWD]/.xvc/ec/1679695689244639",
    "[CWD]/.xvc/ec/1679695689317409",
    "[CWD]/.xvc/ec/1679695689389725",
]
[TRACE][pipeline/src/lib.rs::309] name: Some(
    "generic",
)
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
    paths: 0x000000012a904ef0,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x000000012a904e40,
    runloop: Some(
        (
            0x00000001437055b0,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::263] pipeline_len: 1
[TRACE][pipeline/src/pipeline/mod.rs::289] &dependency_graph: {
    XvcEntity(
        10,
        9552538657297686406,
    ): [],
}
[INFO][pipeline/src/pipeline/mod.rs::303] Pipeline Graph:
digraph {
    0 [ label = "(10, 9552538657297686406)" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::375] step_states: HStore {
    map: {
        XvcEntity(
            10,
            9552538657297686406,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::409] (step_e, step_s): (
    XvcEntity(
        10,
        9552538657297686406,
    ),
    Begin(
        FromInit,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::474] dependency_diffs: HStore {
    map: {},
}
[TRACE][pipeline/src/pipeline/mod.rs::409] (step_e, step_s): (
    XvcEntity(
        10,
        9552538657297686406,
    ),
    WaitingDependencySteps(
        FromRunConditional,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::474] dependency_diffs: HStore {
    map: {},
}
[TRACE][pipeline/src/pipeline/mod.rs::409] (step_e, step_s): (
    XvcEntity(
        10,
        9552538657297686406,
    ),
    CheckingMissingDependencies(
        FromDependencyStepsFinishedSuccessfully,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::474] dependency_diffs: HStore {
    map: {},
}
[TRACE][pipeline/src/pipeline/mod.rs::409] (step_e, step_s): (
    XvcEntity(
        10,
        9552538657297686406,
    ),
    CheckingMissingOutputs(
        FromNoMissingDependencies,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::474] dependency_diffs: HStore {
    map: {},
}
[TRACE][pipeline/src/pipeline/mod.rs::409] (step_e, step_s): (
    XvcEntity(
        10,
        9552538657297686406,
    ),
    CheckingTimestamps(
        FromHasNoMissingOutputs,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::474] dependency_diffs: HStore {
    map: {},
}
[TRACE][pipeline/src/pipeline/mod.rs::409] (step_e, step_s): (
    XvcEntity(
        10,
        9552538657297686406,
    ),
    CheckingDependencyContentDigest(
        FromHasNoNewerDependencies,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::450] dependency_comparison_params: DependencyComparisonParams {
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
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "core.guid": String(
                            "4e1b2b04f256091c",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
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
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "847ba0dff8fb492c",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
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
                            "debug",
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
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "debug",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
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
                "git.auto_stage": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "847ba0dff8fb492c",
                    ),
                },
                "file.track.no_commit": XvcConfigValue {
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"4e1b2b04f256091c/"
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
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 12,
            random: 2513896018348605072,
            dirty: false,
        },
    },
    pipeline_rundir: XvcPath(
        "",
    ),
    pmm: {
        XvcPath(
            ".gitignore",
        ): XvcMetadata {
            file_type: File,
            size: Some(
                107,
            ),
            modified: Some(
                SystemTime {
                    tv_sec: 1679695685,
                    tv_nsec: 169574219,
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
                    tv_sec: 1679695685,
                    tv_nsec: 169487760,
                },
            ),
        },
    },
    algorithm: Blake3,
    all_dependencies: XvcStore {
        map: {
            XvcEntity(
                3,
                3502701794558230782,
            ): File {
                path: XvcPath(
                    "data.txt",
                ),
            },
            XvcEntity(
                4,
                8470562819807696088,
            ): File {
                path: XvcPath(
                    "data2.txt",
                ),
            },
            XvcEntity(
                7,
                10074615450653110740,
            ): Step {
                name: "hello",
            },
            XvcEntity(
                8,
                14930448192085249654,
            ): Step {
                name: "file-dependency",
            },
            XvcEntity(
                11,
                17309432153816108988,
            ): Generic {
                generic_command: "date +%Y",
            },
        },
        entity_index: {
            Generic {
                generic_command: "date +%Y",
            }: [
                XvcEntity(
                    11,
                    17309432153816108988,
                ),
            ],
            Step {
                name: "file-dependency",
            }: [
                XvcEntity(
                    8,
                    14930448192085249654,
                ),
            ],
            Step {
                name: "hello",
            }: [
                XvcEntity(
                    7,
                    10074615450653110740,
                ),
            ],
            File {
                path: XvcPath(
                    "data.txt",
                ),
            }: [
                XvcEntity(
                    3,
                    3502701794558230782,
                ),
            ],
            File {
                path: XvcPath(
                    "data2.txt",
                ),
            }: [
                XvcEntity(
                    4,
                    8470562819807696088,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        3502701794558230782,
                    ),
                    value: File {
                        path: XvcPath(
                            "data.txt",
                        ),
                    },
                },
                Add {
                    entity: XvcEntity(
                        4,
                        8470562819807696088,
                    ),
                    value: File {
                        path: XvcPath(
                            "data2.txt",
                        ),
                    },
                },
                Add {
                    entity: XvcEntity(
                        7,
                        10074615450653110740,
                    ),
                    value: Step {
                        name: "hello",
                    },
                },
                Add {
                    entity: XvcEntity(
                        8,
                        14930448192085249654,
                    ),
                    value: Step {
                        name: "file-dependency",
                    },
                },
                Add {
                    entity: XvcEntity(
                        11,
                        17309432153816108988,
                    ),
                    value: Generic {
                        generic_command: "date +%Y",
                    },
                },
            ],
        ),
        current: EventLog(
            [],
        ),
    },
    dependency_paths: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    3,
                    3502701794558230782,
                ): File {
                    path: XvcPath(
                        "data.txt",
                    ),
                },
                XvcEntity(
                    4,
                    8470562819807696088,
                ): File {
                    path: XvcPath(
                        "data2.txt",
                    ),
                },
                XvcEntity(
                    7,
                    10074615450653110740,
                ): Step {
                    name: "hello",
                },
                XvcEntity(
                    8,
                    14930448192085249654,
                ): Step {
                    name: "file-dependency",
                },
                XvcEntity(
                    11,
                    17309432153816108988,
                ): Generic {
                    generic_command: "date +%Y",
                },
            },
            entity_index: {
                Generic {
                    generic_command: "date +%Y",
                }: [
                    XvcEntity(
                        11,
                        17309432153816108988,
                    ),
                ],
                Step {
                    name: "file-dependency",
                }: [
                    XvcEntity(
                        8,
                        14930448192085249654,
                    ),
                ],
                Step {
                    name: "hello",
                }: [
                    XvcEntity(
                        7,
                        10074615450653110740,
                    ),
                ],
                File {
                    path: XvcPath(
                        "data.txt",
                    ),
                }: [
                    XvcEntity(
                        3,
                        3502701794558230782,
                    ),
                ],
                File {
                    path: XvcPath(
                        "data2.txt",
                    ),
                }: [
                    XvcEntity(
                        4,
                        8470562819807696088,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            3502701794558230782,
                        ),
                        value: File {
                            path: XvcPath(
                                "data.txt",
                            ),
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            4,
                            8470562819807696088,
                        ),
                        value: File {
                            path: XvcPath(
                                "data2.txt",
                            ),
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            10074615450653110740,
                        ),
                        value: Step {
                            name: "hello",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            14930448192085249654,
                        ),
                        value: Step {
                            name: "file-dependency",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            11,
                            17309432153816108988,
                        ),
                        value: Generic {
                            generic_command: "date +%Y",
                        },
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        children: XvcStore {
            map: {},
            entity_index: {},
            previous: EventLog(
                [],
            ),
            current: EventLog(
                [],
            ),
        },
        child_parents: XvcStore {
            map: {},
            entity_index: {},
            previous: EventLog(
                [],
            ),
            current: EventLog(
                [],
            ),
        },
    },
    xvc_path_store: XvcStore {
        map: {},
        entity_index: {},
        previous: EventLog(
            [],
        ),
        current: EventLog(
            [],
        ),
    },
    xvc_metadata_store: XvcStore {
        map: {},
        entity_index: {},
        previous: EventLog(
            [],
        ),
        current: EventLog(
            [],
        ),
    },
    xvc_digests_store: XvcStore {
        map: {},
        entity_index: {},
        previous: EventLog(
            [],
        ),
        current: EventLog(
            [],
        ),
    },
    text_files: XvcStore {
        map: {},
        entity_index: {},
        previous: EventLog(
            [],
        ),
        current: EventLog(
            [],
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::617] dependency_diffs: HStore {
    map: {},
}
[TRACE][pipeline/src/pipeline/mod.rs::618] params.run_conditions.ignore_content_digest_comparison: false
[TRACE][pipeline/src/pipeline/mod.rs::624] step_e: XvcEntity(
    10,
    9552538657297686406,
)
[TRACE][pipeline/src/pipeline/mod.rs::627] deps: HStore {
    map: {
        XvcEntity(
            11,
            17309432153816108988,
        ): Generic {
            generic_command: "date +%Y",
        },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::629] deps.is_empty(): false
[TRACE][pipeline/src/pipeline/mod.rs::635] _comparison_results: HStore {
    map: {},
}
[TRACE][pipeline/src/pipeline/mod.rs::639] cmp_params: DependencyComparisonParams {
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
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "core.guid": String(
                            "4e1b2b04f256091c",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
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
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "847ba0dff8fb492c",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
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
                            "debug",
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
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "debug",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
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
                "git.auto_stage": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "847ba0dff8fb492c",
                    ),
                },
                "file.track.no_commit": XvcConfigValue {
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"4e1b2b04f256091c/"
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
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 12,
            random: 2513896018348605072,
            dirty: false,
        },
    },
    pipeline_rundir: XvcPath(
        "",
    ),
    pmm: {
        XvcPath(
            ".gitignore",
        ): XvcMetadata {
            file_type: File,
            size: Some(
                107,
            ),
            modified: Some(
                SystemTime {
                    tv_sec: 1679695685,
                    tv_nsec: 169574219,
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
                    tv_sec: 1679695685,
                    tv_nsec: 169487760,
                },
            ),
        },
    },
    algorithm: Blake3,
    all_dependencies: XvcStore {
        map: {
            XvcEntity(
                3,
                3502701794558230782,
            ): File {
                path: XvcPath(
                    "data.txt",
                ),
            },
            XvcEntity(
                4,
                8470562819807696088,
            ): File {
                path: XvcPath(
                    "data2.txt",
                ),
            },
            XvcEntity(
                7,
                10074615450653110740,
            ): Step {
                name: "hello",
            },
            XvcEntity(
                8,
                14930448192085249654,
            ): Step {
                name: "file-dependency",
            },
            XvcEntity(
                11,
                17309432153816108988,
            ): Generic {
                generic_command: "date +%Y",
            },
        },
        entity_index: {
            Generic {
                generic_command: "date +%Y",
            }: [
                XvcEntity(
                    11,
                    17309432153816108988,
                ),
            ],
            Step {
                name: "file-dependency",
            }: [
                XvcEntity(
                    8,
                    14930448192085249654,
                ),
            ],
            Step {
                name: "hello",
            }: [
                XvcEntity(
                    7,
                    10074615450653110740,
                ),
            ],
            File {
                path: XvcPath(
                    "data.txt",
                ),
            }: [
                XvcEntity(
                    3,
                    3502701794558230782,
                ),
            ],
            File {
                path: XvcPath(
                    "data2.txt",
                ),
            }: [
                XvcEntity(
                    4,
                    8470562819807696088,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        3502701794558230782,
                    ),
                    value: File {
                        path: XvcPath(
                            "data.txt",
                        ),
                    },
                },
                Add {
                    entity: XvcEntity(
                        4,
                        8470562819807696088,
                    ),
                    value: File {
                        path: XvcPath(
                            "data2.txt",
                        ),
                    },
                },
                Add {
                    entity: XvcEntity(
                        7,
                        10074615450653110740,
                    ),
                    value: Step {
                        name: "hello",
                    },
                },
                Add {
                    entity: XvcEntity(
                        8,
                        14930448192085249654,
                    ),
                    value: Step {
                        name: "file-dependency",
                    },
                },
                Add {
                    entity: XvcEntity(
                        11,
                        17309432153816108988,
                    ),
                    value: Generic {
                        generic_command: "date +%Y",
                    },
                },
            ],
        ),
        current: EventLog(
            [],
        ),
    },
    dependency_paths: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    3,
                    3502701794558230782,
                ): File {
                    path: XvcPath(
                        "data.txt",
                    ),
                },
                XvcEntity(
                    4,
                    8470562819807696088,
                ): File {
                    path: XvcPath(
                        "data2.txt",
                    ),
                },
                XvcEntity(
                    7,
                    10074615450653110740,
                ): Step {
                    name: "hello",
                },
                XvcEntity(
                    8,
                    14930448192085249654,
                ): Step {
                    name: "file-dependency",
                },
                XvcEntity(
                    11,
                    17309432153816108988,
                ): Generic {
                    generic_command: "date +%Y",
                },
            },
            entity_index: {
                Generic {
                    generic_command: "date +%Y",
                }: [
                    XvcEntity(
                        11,
                        17309432153816108988,
                    ),
                ],
                Step {
                    name: "file-dependency",
                }: [
                    XvcEntity(
                        8,
                        14930448192085249654,
                    ),
                ],
                Step {
                    name: "hello",
                }: [
                    XvcEntity(
                        7,
                        10074615450653110740,
                    ),
                ],
                File {
                    path: XvcPath(
                        "data.txt",
                    ),
                }: [
                    XvcEntity(
                        3,
                        3502701794558230782,
                    ),
                ],
                File {
                    path: XvcPath(
                        "data2.txt",
                    ),
                }: [
                    XvcEntity(
                        4,
                        8470562819807696088,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            3502701794558230782,
                        ),
                        value: File {
                            path: XvcPath(
                                "data.txt",
                            ),
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            4,
                            8470562819807696088,
                        ),
                        value: File {
                            path: XvcPath(
                                "data2.txt",
                            ),
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            7,
                            10074615450653110740,
                        ),
                        value: Step {
                            name: "hello",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            8,
                            14930448192085249654,
                        ),
                        value: Step {
                            name: "file-dependency",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            11,
                            17309432153816108988,
                        ),
                        value: Generic {
                            generic_command: "date +%Y",
                        },
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        children: XvcStore {
            map: {},
            entity_index: {},
            previous: EventLog(
                [],
            ),
            current: EventLog(
                [],
            ),
        },
        child_parents: XvcStore {
            map: {},
            entity_index: {},
            previous: EventLog(
                [],
            ),
            current: EventLog(
                [],
            ),
        },
    },
    xvc_path_store: XvcStore {
        map: {},
        entity_index: {},
        previous: EventLog(
            [],
        ),
        current: EventLog(
            [],
        ),
    },
    xvc_metadata_store: XvcStore {
        map: {},
        entity_index: {},
        previous: EventLog(
            [],
        ),
        current: EventLog(
            [],
        ),
    },
    xvc_digests_store: XvcStore {
        map: {},
        entity_index: {},
        previous: EventLog(
            [],
        ),
        current: EventLog(
            [],
        ),
    },
    text_files: XvcStore {
        map: {},
        entity_index: {},
        previous: EventLog(
            [],
        ),
        current: EventLog(
            [],
        ),
    },
}
[TRACE][pipeline/src/pipeline/deps/compare.rs::187] collected_diffs: Diffs {
    xvc_dependency_diff: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    xvc_digests_diff: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    xvc_metadata_diff: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    xvc_path_diff: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
}
[TRACE][pipeline/src/pipeline/deps/compare.rs::235] stdout: "2023
"
[TRACE][pipeline/src/pipeline/deps/compare.rs::237] stderr: ""
[TRACE][pipeline/src/pipeline/mod.rs::645] collected_diffs: Diffs {
    xvc_dependency_diff: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    xvc_digests_diff: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    11,
                    17309432153816108988,
                ): RecordMissing {
                    actual: XvcDigests(
                        {
                            "stdout-digest": XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    0,
                                    25,
                                    102,
                                    177,
                                    28,
                                    165,
                                    102,
                                    239,
                                    199,
                                    24,
                                    108,
                                    167,
                                    117,
                                    135,
                                    7,
                                    186,
                                    162,
                                    135,
                                    192,
                                    222,
                                    59,
                                    86,
                                    22,
                                    53,
                                    101,
                                    177,
                                    233,
                                    127,
                                    6,
                                    117,
                                    31,
                                    37,
                                ],
                            },
                        },
                    ),
                },
            },
        },
        poisoned: false,
        ..
    },
    xvc_metadata_diff: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    xvc_path_diff: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::647] &collected_diffs: Diffs {
    xvc_dependency_diff: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    xvc_digests_diff: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    11,
                    17309432153816108988,
                ): RecordMissing {
                    actual: XvcDigests(
                        {
                            "stdout-digest": XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    0,
                                    25,
                                    102,
                                    177,
                                    28,
                                    165,
                                    102,
                                    239,
                                    199,
                                    24,
                                    108,
                                    167,
                                    117,
                                    135,
                                    7,
                                    186,
                                    162,
                                    135,
                                    192,
                                    222,
                                    59,
                                    86,
                                    22,
                                    53,
                                    101,
                                    177,
                                    233,
                                    127,
                                    6,
                                    117,
                                    31,
                                    37,
                                ],
                            },
                        },
                    ),
                },
            },
        },
        poisoned: false,
        ..
    },
    xvc_metadata_diff: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    xvc_path_diff: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::474] dependency_diffs: HStore {
    map: {
        XvcEntity(
            10,
            9552538657297686406,
        ): Diffs {
            xvc_dependency_diff: RwLock {
                data: HStore {
                    map: {},
                },
                poisoned: false,
                ..
            },
            xvc_digests_diff: RwLock {
                data: HStore {
                    map: {
                        XvcEntity(
                            11,
                            17309432153816108988,
                        ): RecordMissing {
                            actual: XvcDigests(
                                {
                                    "stdout-digest": XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            0,
                                            25,
                                            102,
                                            177,
                                            28,
                                            165,
                                            102,
                                            239,
                                            199,
                                            24,
                                            108,
                                            167,
                                            117,
                                            135,
                                            7,
                                            186,
                                            162,
                                            135,
                                            192,
                                            222,
                                            59,
                                            86,
                                            22,
                                            53,
                                            101,
                                            177,
                                            233,
                                            127,
                                            6,
                                            117,
                                            31,
                                            37,
                                        ],
                                    },
                                },
                            ),
                        },
                    },
                },
                poisoned: false,
                ..
            },
            xvc_metadata_diff: RwLock {
                data: HStore {
                    map: {},
                },
                poisoned: false,
                ..
            },
            xvc_path_diff: RwLock {
                data: HStore {
                    map: {},
                },
                poisoned: false,
                ..
            },
        },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::480] &xvc_digests_diffs: HStore {
    map: {
        XvcEntity(
            11,
            17309432153816108988,
        ): RecordMissing {
            actual: XvcDigests(
                {
                    "stdout-digest": XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            0,
                            25,
                            102,
                            177,
                            28,
                            165,
                            102,
                            239,
                            199,
                            24,
                            108,
                            167,
                            117,
                            135,
                            7,
                            186,
                            162,
                            135,
                            192,
                            222,
                            59,
                            86,
                            22,
                            53,
                            101,
                            177,
                            233,
                            127,
                            6,
                            117,
                            31,
                            37,
                        ],
                    },
                },
            ),
        },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::493] &xvc_dependency_diffs: HStore {
    map: {},
}
[TRACE][pipeline/src/pipeline/mod.rs::506] &xvc_metadata_diffs: HStore {
    map: {},
}
[TRACE][pipeline/src/pipeline/mod.rs::524] &xvc_path_diffs: HStore {
    map: {},
}
[TRACE][pipeline/src/pipeline/mod.rs::409] (step_e, step_s): (
    XvcEntity(
        10,
        9552538657297686406,
    ),
    WaitingToRun(
        FromContentDigestChanged,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::474] dependency_diffs: HStore {
    map: {},
}
[TRACE][pipeline/src/pipeline/mod.rs::409] (step_e, step_s): (
    XvcEntity(
        10,
        9552538657297686406,
    ),
    Running(
        FromStartProcess,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::474] dependency_diffs: HStore {
    map: {},
}
[OUT] Happy New Year! Welcome `(date +%Y)`!

[OUT] [EXIT] Successfully
[TRACE][pipeline/src/pipeline/mod.rs::597] &updated_xvc_digests_store: XvcStore {
    map: {
        XvcEntity(
            11,
            17309432153816108988,
        ): XvcDigests(
            {
                "stdout-digest": XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        0,
                        25,
                        102,
                        177,
                        28,
                        165,
                        102,
                        239,
                        199,
                        24,
                        108,
                        167,
                        117,
                        135,
                        7,
                        186,
                        162,
                        135,
                        192,
                        222,
                        59,
                        86,
                        22,
                        53,
                        101,
                        177,
                        233,
                        127,
                        6,
                        117,
                        31,
                        37,
                    ],
                },
            },
        ),
    },
    entity_index: {
        XvcDigests(
            {
                "stdout-digest": XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        0,
                        25,
                        102,
                        177,
                        28,
                        165,
                        102,
                        239,
                        199,
                        24,
                        108,
                        167,
                        117,
                        135,
                        7,
                        186,
                        162,
                        135,
                        192,
                        222,
                        59,
                        86,
                        22,
                        53,
                        101,
                        177,
                        233,
                        127,
                        6,
                        117,
                        31,
                        37,
                    ],
                },
            },
        ): [
            XvcEntity(
                11,
                17309432153816108988,
            ),
        ],
    },
    previous: EventLog(
        [],
    ),
    current: EventLog(
        [
            Add {
                entity: XvcEntity(
                    11,
                    17309432153816108988,
                ),
                value: XvcDigests(
                    {
                        "stdout-digest": XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                0,
                                25,
                                102,
                                177,
                                28,
                                165,
                                102,
                                239,
                                199,
                                24,
                                108,
                                167,
                                117,
                                135,
                                7,
                                186,
                                162,
                                135,
                                192,
                                222,
                                59,
                                86,
                                22,
                                53,
                                101,
                                177,
                                233,
                                127,
                                6,
                                117,
                                31,
                                37,
                            ],
                        },
                    },
                ),
            },
        ],
    ),
}
[TRACE][lib/src/cli/mod.rs::381] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::384] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline --name generic run"
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
[TRACE][lib/src/cli/mod.rs::582] git_add_output: "add '.xvc/store/xvc-digests-store/1679695689870412.json'
"
[TRACE][lib/src/cli/mod.rs::433] args: [
    "-C",
    "[CWD]",
    "commit",
    "-m",
    "Xvc auto-commit after /'/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline --name generic run/'",
]
[DEBUG] Committing .xvc/ to git: [main 4076c6f] Xvc auto-commit after '/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline --name generic run'
 1 file changed, 1 insertion(+)
 create mode 100644 .xvc/store/xvc-digests-store/1679695689870412.json

[DEBUG] Command completed successfully.

```

The step won't run until the next year.

```console
$ xvc pipeline --name generic run

```


## Caveats

## Tips

Most shells support editing longer commands with an editor. For bash, you can use `Ctrl+X Ctrl+E`.

Pipeline commands can get longer quickly. You can use [xvc aliases](/ref/xvc-aliases.md) for shorter
versions. Type `source $(xvc aliases)` to load the aliases into your shell.
