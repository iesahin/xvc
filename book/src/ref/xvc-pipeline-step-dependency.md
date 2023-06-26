# xvc pipeline step dependency

## Purpose

Define a dependency to an existing step in the pipeline.

## Synopsis

```console
$ xvc pipeline step dependency --help
Add a dependency to a step

Usage: xvc pipeline step dependency [OPTIONS] --step-name <STEP_NAME>

Options:
  -s, --step-name <STEP_NAME>
          Name of the step to add the dependency to

      --generic <GENERICS>
          Add a generic command output as a dependency. Can be used multiple times. Please delimit the command with ' ' to avoid shell expansion

      --url <URLS>
          Add a URL dependency to the step. Can be used multiple times

      --file <FILES>
          Add a file dependency to the step. Can be used multiple times

      --step <STEPS>
          Add a step dependency to a step. Can be used multiple times. Steps are referred with their names

      --glob <GLOBS>
          Add a glob dependency to the step. Can be used multiple times.

          The difference between this and the glob-digest option is that the glob option keeps track of all matching files, but glob-digest only keeps track of the matched files' digest. When you want to use ${[ALL_GLOB_FILES]} or ${[CHANGED_GLOB_FILES]} options in the step command, use the glob option. Otherwise, you can use the glob-digest option to save disk space.

      --glob_digest <GLOB_DIGESTS>
          Add a glob digest dependency to the step. Can be used multiple times.

          The difference between this and the glob option is that the glob option keeps track of all matching files, but glob-digest only keeps track of the matched files' digest. When you want to use ${[ALL_GLOB_FILES]} or ${[CHANGED_GLOB_FILES]} options in the step command, use the glob option. Otherwise, you can use the glob-digest option to save disk space.

      --param <PARAMS>
          Add a parameter dependency to the step in the form filename.yaml::model.units . Can be used multiple times

      --regex <REGEXPS>
          Add a regex dependency in the form filename.txt:/^regex/ . Can be used multiple times. The difference between this and the regex-digest option is that the regex option keeps track of all matching lines, but regex-digest only keeps track of the matched lines' digest. When you want to use ${[ALL_REGEX_LINES]} or ${[CHANGED_REGEX_LINES]} options in the step command, use the regex option. Otherwise, you can use the regex-digest option to save disk space

      --regex_digest <REGEXP_DIGESTS>
          Add a regex dependency in the form filename.txt:/^regex/ . Can be used multiple times.

          The difference between this and the regex option is that the regex option keeps track of all matching lines that can be used in the step command. This option only keeps track of the matched lines' digest.

      --line <LINES>
          Add a line dependency in the form filename.txt::123-234

          The difference between this and the line-digest option is that the line option keeps track of all matching lines that can be used in the step command. This option only keeps track of the matched lines' digest. When you want to use ${[ALL_LINES]} or ${[CHANGED_LINES]} options in the step command, use the line option. Otherwise, you can use the line-digest option to save disk space.

      --line_digest <LINE_DIGESTS>
          Add a line digest dependency in the form filename.txt::123-234

          The difference between this and the line option is that the line option keeps track of all matching lines that can be used in the step command. This option only keeps track of the matched lines' digest. When you want to use ${[ALL_LINES]} or ${[CHANGED_LINES]} options in the step command, use the line option. Otherwise, you can use the line-digest option to save disk space.

  -h, --help
          Print help (see a summary with '-h')

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
$ xvc --debug pipeline run
[OUT] [file-dependency] data.txt has changed


```

You can add multiple dependencies to a step with multiple invocations.

```console
$ xvc pipeline step dependency --step-name file-dependency --file data2.txt
```

A step will run if any of its dependencies have changed.

```console
$ xvc pipeline run
[OUT] [file-dependency] data.txt has changed


```

By default, they are not run if none of the dependencies have changed.

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
                "core.verbosity": String(
                    "error",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.command": String(
                    "git",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "core.guid": String(
                    "2b897277af478900",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "core.guid": String(
                    "09bf77d8601acdb7",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "git.command": String(
                    "git",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
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
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "09bf77d8601acdb7",
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
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
            ),
        },
        "file.recheck.method": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
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
        "git.auto_stage": XvcConfigValue {
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
        "file.list.recursive": XvcConfigValue {
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
        "core.quiet": XvcConfigValue {
            source: CommandLine,
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
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
            ),
        },
        "file.track.no_parallel": XvcConfigValue {
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
guid = /"2b897277af478900/"
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
    "[CWD]/.xvc/ec/1687255206389690",
    "[CWD]/.xvc/ec/1687255206393426",
    "[CWD]/.xvc/ec/1687255206457600",
    "[CWD]/.xvc/ec/1687255206522847",
    "[CWD]/.xvc/ec/1687255206907538",
]
[TRACE][pipeline/src/lib.rs::350] name: Some(
    "default",
)
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/.gitignore"
[TRACE][core/src/types/xvcpath.rs::88] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::89] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/data2.txt"
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
[TRACE][core/src/types/xvcpath.rs::87] abs_path: "[CWD]/data.txt"
[TRACE][core/src/types/xvcpath.rs::88] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::89] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/notify.rs::160] watcher: FsEventWatcher {
    paths: 0x00006000036980c0,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x00006000009983d0,
    runloop: Some(
        (
            0x0000600000498800,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::302] pipeline_len: 1
[TRACE][pipeline/src/pipeline/mod.rs::328] &dependency_graph: {
    XvcEntity(
        2,
        8083115080617033,
    ): [],
}
[INFO][pipeline/src/pipeline/mod.rs::342] Pipeline Graph:
digraph {
    0 [ label = "(2, 8083115080617033)" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::416] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                8083115080617033,
            ): Begin(
                FromInit,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::542] &step_thread_store: HStore {
    map: {
        XvcEntity(
            2,
            8083115080617033,
        ): ScopedJoinHandle { .. },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::546] (step_e, &jh): (
    XvcEntity(
        2,
        8083115080617033,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::708] &step_params: StepStateParams {
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
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "2b897277af478900",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "core.guid": String(
                            "09bf77d8601acdb7",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "09bf77d8601acdb7",
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
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
                "git.auto_stage": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
guid = /"2b897277af478900/"
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
            random: 14447771076703443834,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "data.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686297661,
                        tv_nsec: 509304211,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389941775,
                    },
                ),
            },
            XvcPath(
                "data2.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686298446,
                        tv_nsec: 762404426,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389867650,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
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
                name: "file-dependency",
            },
            step_command: XvcStepCommand {
                command: "echo data.txt has changed",
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
        2,
        8083115080617033,
    ),
    step: XvcStep {
        name: "file-dependency",
    },
    step_command: XvcStepCommand {
        command: "echo data.txt has changed",
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
        map: {
            XvcEntity(
                3,
                12968638473026154098,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
            XvcEntity(
                4,
                5730163350291362975,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data2.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
        },
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::708] &step_params: StepStateParams {
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
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "2b897277af478900",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "core.guid": String(
                            "09bf77d8601acdb7",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "09bf77d8601acdb7",
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
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
                "git.auto_stage": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
guid = /"2b897277af478900/"
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
            random: 14447771076703443834,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "data.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686297661,
                        tv_nsec: 509304211,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389941775,
                    },
                ),
            },
            XvcPath(
                "data2.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686298446,
                        tv_nsec: 762404426,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389867650,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
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
                name: "file-dependency",
            },
            step_command: XvcStepCommand {
                command: "echo data.txt has changed",
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
        2,
        8083115080617033,
    ),
    step: XvcStep {
        name: "file-dependency",
    },
    step_command: XvcStepCommand {
        command: "echo data.txt has changed",
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
        map: {
            XvcEntity(
                3,
                12968638473026154098,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
            XvcEntity(
                4,
                5730163350291362975,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data2.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
        },
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::708] &step_params: StepStateParams {
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
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "2b897277af478900",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "core.guid": String(
                            "09bf77d8601acdb7",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "09bf77d8601acdb7",
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
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
                "git.auto_stage": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
guid = /"2b897277af478900/"
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
            random: 14447771076703443834,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "data.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686297661,
                        tv_nsec: 509304211,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389941775,
                    },
                ),
            },
            XvcPath(
                "data2.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686298446,
                        tv_nsec: 762404426,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389867650,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
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
                name: "file-dependency",
            },
            step_command: XvcStepCommand {
                command: "echo data.txt has changed",
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
        2,
        8083115080617033,
    ),
    step: XvcStep {
        name: "file-dependency",
    },
    step_command: XvcStepCommand {
        command: "echo data.txt has changed",
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
        map: {
            XvcEntity(
                3,
                12968638473026154098,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
            XvcEntity(
                4,
                5730163350291362975,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data2.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
        },
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::708] &step_params: StepStateParams {
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
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "2b897277af478900",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "core.guid": String(
                            "09bf77d8601acdb7",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "09bf77d8601acdb7",
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
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
                "git.auto_stage": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
guid = /"2b897277af478900/"
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
            random: 14447771076703443834,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "data.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686297661,
                        tv_nsec: 509304211,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389941775,
                    },
                ),
            },
            XvcPath(
                "data2.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686298446,
                        tv_nsec: 762404426,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389867650,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
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
                name: "file-dependency",
            },
            step_command: XvcStepCommand {
                command: "echo data.txt has changed",
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
        2,
        8083115080617033,
    ),
    step: XvcStep {
        name: "file-dependency",
    },
    step_command: XvcStepCommand {
        command: "echo data.txt has changed",
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
        map: {
            XvcEntity(
                3,
                12968638473026154098,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
            XvcEntity(
                4,
                5730163350291362975,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data2.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
        },
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::708] &step_params: StepStateParams {
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
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "2b897277af478900",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "core.guid": String(
                            "09bf77d8601acdb7",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "09bf77d8601acdb7",
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
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
                "git.auto_stage": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
guid = /"2b897277af478900/"
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
            random: 14447771076703443834,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "data.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686297661,
                        tv_nsec: 509304211,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389941775,
                    },
                ),
            },
            XvcPath(
                "data2.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686298446,
                        tv_nsec: 762404426,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389867650,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
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
                name: "file-dependency",
            },
            step_command: XvcStepCommand {
                command: "echo data.txt has changed",
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
                    4,
                    5730163350291362975,
                ): RecordMissing {
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686298446,
                                            tv_nsec: 762404426,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
                        },
                    ),
                },
                XvcEntity(
                    3,
                    12968638473026154098,
                ): RecordMissing {
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686297661,
                                            tv_nsec: 509304211,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
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
        8083115080617033,
    ),
    step: XvcStep {
        name: "file-dependency",
    },
    step_command: XvcStepCommand {
        command: "echo data.txt has changed",
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
        map: {
            XvcEntity(
                3,
                12968638473026154098,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
            XvcEntity(
                4,
                5730163350291362975,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data2.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
        },
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1136] deps.is_empty(): false
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::708] &step_params: StepStateParams {
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
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "2b897277af478900",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "core.guid": String(
                            "09bf77d8601acdb7",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "09bf77d8601acdb7",
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
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
                "git.auto_stage": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
guid = /"2b897277af478900/"
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
            random: 14447771076703443834,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "data.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686297661,
                        tv_nsec: 509304211,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389941775,
                    },
                ),
            },
            XvcPath(
                "data2.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686298446,
                        tv_nsec: 762404426,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389867650,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
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
                name: "file-dependency",
            },
            step_command: XvcStepCommand {
                command: "echo data.txt has changed",
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
                    4,
                    5730163350291362975,
                ): Different {
                    record: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686298446,
                                            tv_nsec: 762404426,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
                        },
                    ),
                },
                XvcEntity(
                    3,
                    12968638473026154098,
                ): Different {
                    record: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686297661,
                                            tv_nsec: 509304211,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
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
        8083115080617033,
    ),
    step: XvcStep {
        name: "file-dependency",
    },
    step_command: XvcStepCommand {
        command: "echo data.txt has changed",
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
        map: {
            XvcEntity(
                3,
                12968638473026154098,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
            XvcEntity(
                4,
                5730163350291362975,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data2.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
        },
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::708] &step_params: StepStateParams {
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
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "2b897277af478900",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "core.guid": String(
                            "09bf77d8601acdb7",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "09bf77d8601acdb7",
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
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
                "git.auto_stage": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
guid = /"2b897277af478900/"
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
            random: 14447771076703443834,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "data.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686297661,
                        tv_nsec: 509304211,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389941775,
                    },
                ),
            },
            XvcPath(
                "data2.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686298446,
                        tv_nsec: 762404426,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389867650,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
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
                name: "file-dependency",
            },
            step_command: XvcStepCommand {
                command: "echo data.txt has changed",
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
                    4,
                    5730163350291362975,
                ): Different {
                    record: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686298446,
                                            tv_nsec: 762404426,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
                        },
                    ),
                },
                XvcEntity(
                    3,
                    12968638473026154098,
                ): Different {
                    record: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686297661,
                                            tv_nsec: 509304211,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
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
        8083115080617033,
    ),
    step: XvcStep {
        name: "file-dependency",
    },
    step_command: XvcStepCommand {
        command: "echo data.txt has changed",
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
        map: {
            XvcEntity(
                3,
                12968638473026154098,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
            XvcEntity(
                4,
                5730163350291362975,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data2.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
        },
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1426] params: StepStateParams {
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
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "2b897277af478900",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "core.guid": String(
                            "09bf77d8601acdb7",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "09bf77d8601acdb7",
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
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
                "git.auto_stage": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
guid = /"2b897277af478900/"
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
            random: 14447771076703443834,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "data.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686297661,
                        tv_nsec: 509304211,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389941775,
                    },
                ),
            },
            XvcPath(
                "data2.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686298446,
                        tv_nsec: 762404426,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389867650,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
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
                name: "file-dependency",
            },
            step_command: XvcStepCommand {
                command: "echo data.txt has changed",
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
                    4,
                    5730163350291362975,
                ): Different {
                    record: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686298446,
                                            tv_nsec: 762404426,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
                        },
                    ),
                },
                XvcEntity(
                    3,
                    12968638473026154098,
                ): Different {
                    record: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686297661,
                                            tv_nsec: 509304211,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
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
        8083115080617033,
    ),
    step: XvcStep {
        name: "file-dependency",
    },
    step_command: XvcStepCommand {
        command: "echo data.txt has changed",
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
        map: {
            XvcEntity(
                3,
                12968638473026154098,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
            XvcEntity(
                4,
                5730163350291362975,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data2.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
        },
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::708] &step_params: StepStateParams {
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
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "2b897277af478900",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "core.guid": String(
                            "09bf77d8601acdb7",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "09bf77d8601acdb7",
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
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
                "git.auto_stage": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
guid = /"2b897277af478900/"
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
            random: 14447771076703443834,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "data.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686297661,
                        tv_nsec: 509304211,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389941775,
                    },
                ),
            },
            XvcPath(
                "data2.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686298446,
                        tv_nsec: 762404426,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389867650,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
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
                name: "file-dependency",
            },
            step_command: XvcStepCommand {
                command: "echo data.txt has changed",
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
                    4,
                    5730163350291362975,
                ): Different {
                    record: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686298446,
                                            tv_nsec: 762404426,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
                        },
                    ),
                },
                XvcEntity(
                    3,
                    12968638473026154098,
                ): Different {
                    record: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686297661,
                                            tv_nsec: 509304211,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
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
        8083115080617033,
    ),
    step: XvcStep {
        name: "file-dependency",
    },
    step_command: XvcStepCommand {
        command: "echo data.txt has changed",
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
        map: {
            XvcEntity(
                3,
                12968638473026154098,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
            XvcEntity(
                4,
                5730163350291362975,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data2.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
        },
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::622] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::708] &step_params: StepStateParams {
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
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "2b897277af478900",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "core.guid": String(
                            "09bf77d8601acdb7",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "09bf77d8601acdb7",
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
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
                "git.auto_stage": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
guid = /"2b897277af478900/"
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
            random: 14447771076703443834,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "data.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686297661,
                        tv_nsec: 509304211,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389941775,
                    },
                ),
            },
            XvcPath(
                "data2.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686298446,
                        tv_nsec: 762404426,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389867650,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
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
                name: "file-dependency",
            },
            step_command: XvcStepCommand {
                command: "echo data.txt has changed",
            },
            birth: Some(
                Instant {
                    tv_sec: 944401,
                    tv_nsec: 426255916,
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
                        pid: 1898,
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
                    4,
                    5730163350291362975,
                ): Different {
                    record: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686298446,
                                            tv_nsec: 762404426,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
                        },
                    ),
                },
                XvcEntity(
                    3,
                    12968638473026154098,
                ): Different {
                    record: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686297661,
                                            tv_nsec: 509304211,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
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
        8083115080617033,
    ),
    step: XvcStep {
        name: "file-dependency",
    },
    step_command: XvcStepCommand {
        command: "echo data.txt has changed",
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
        map: {
            XvcEntity(
                3,
                12968638473026154098,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
            XvcEntity(
                4,
                5730163350291362975,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data2.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
        },
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1316] params: StepStateParams {
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
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "2b897277af478900",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "core.guid": String(
                            "09bf77d8601acdb7",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "09bf77d8601acdb7",
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
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
                "git.auto_stage": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
guid = /"2b897277af478900/"
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
            random: 14447771076703443834,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "data.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686297661,
                        tv_nsec: 509304211,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389941775,
                    },
                ),
            },
            XvcPath(
                "data2.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686298446,
                        tv_nsec: 762404426,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389867650,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
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
                name: "file-dependency",
            },
            step_command: XvcStepCommand {
                command: "echo data.txt has changed",
            },
            birth: Some(
                Instant {
                    tv_sec: 944401,
                    tv_nsec: 426255916,
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
                        pid: 1898,
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
                    4,
                    5730163350291362975,
                ): Different {
                    record: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686298446,
                                            tv_nsec: 762404426,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
                        },
                    ),
                },
                XvcEntity(
                    3,
                    12968638473026154098,
                ): Different {
                    record: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686297661,
                                            tv_nsec: 509304211,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
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
        8083115080617033,
    ),
    step: XvcStep {
        name: "file-dependency",
    },
    step_command: XvcStepCommand {
        command: "echo data.txt has changed",
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
        map: {
            XvcEntity(
                3,
                12968638473026154098,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
            XvcEntity(
                4,
                5730163350291362975,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data2.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
        },
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1348] command_process: CommandProcess {
    environment: {},
    step: XvcStep {
        name: "file-dependency",
    },
    step_command: XvcStepCommand {
        command: "echo data.txt has changed",
    },
    birth: Some(
        Instant {
            tv_sec: 944401,
            tv_nsec: 426255916,
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
                pid: 1898,
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
[OUT] [file-dependency] data.txt has changed

[TRACE][pipeline/src/pipeline/mod.rs::1354] &process: Popen {
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
        pid: 1898,
        ext: (),
    },
    detached: true,
}
[TRACE][pipeline/src/pipeline/mod.rs::1400] return_state: Some(
    Done(
        FromProcessCompletedSuccessfully,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::1406] params: StepStateParams {
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
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "2b897277af478900",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "core.guid": String(
                            "09bf77d8601acdb7",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.sort": String(
                            "name-desc",
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
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "09bf77d8601acdb7",
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
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
                "git.auto_stage": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
guid = /"2b897277af478900/"
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
            random: 14447771076703443834,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                "data.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686297661,
                        tv_nsec: 509304211,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389941775,
                    },
                ),
            },
            XvcPath(
                "data2.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    19,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1686298446,
                        tv_nsec: 762404426,
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
                        tv_sec: 1687255206,
                        tv_nsec: 389867650,
                    },
                ),
            },
        },
        poisoned: false,
        ..
    },
    run_conditions: RunConditions {
        never: false,
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
                name: "file-dependency",
            },
            step_command: XvcStepCommand {
                command: "echo data.txt has changed",
            },
            birth: Some(
                Instant {
                    tv_sec: 944401,
                    tv_nsec: 426255916,
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
                    4,
                    5730163350291362975,
                ): Different {
                    record: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data2.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686298446,
                                            tv_nsec: 762404426,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
                        },
                    ),
                },
                XvcEntity(
                    3,
                    12968638473026154098,
                ): Different {
                    record: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "data.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        19,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1686297661,
                                            tv_nsec: 509304211,
                                        },
                                    ),
                                },
                            ),
                            content_digest: None,
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
        8083115080617033,
    ),
    step: XvcStep {
        name: "file-dependency",
    },
    step_command: XvcStepCommand {
        command: "echo data.txt has changed",
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
        map: {
            XvcEntity(
                3,
                12968638473026154098,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
            XvcEntity(
                4,
                5730163350291362975,
            ): File(
                FileDep {
                    path: XvcPath(
                        "data2.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
        },
    },
    step_outputs: HStore {
        map: {},
    },
    step_xvc_digests: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::553] "Before state updater": "Before state updater"
[TRACE][pipeline/src/pipeline/mod.rs::566] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                8083115080617033,
            ): Running(
                FromWaitProcess,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::570] done_successfully: Ok(
    false,
)
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

However, if you want to run the step even if none of the dependencies have changed, you can set the `--when` option to `always`.

```console
$ xvc pipeline step update --step-name file-dependency --when always
```

# Now the step will run even if none of the dependencies have changed.
#
# ```console
# $ xvc pipeline run
# [OUT] data.txt has changed
#
# [OUT] [EXIT] Successfully
#
# ```
#
# ### Step Dependencies
#
# You can add a step dependency to a step. These steps specify dependency relationships explicitly, without relying on
# changed files or directories.
#
# ```console
# $ xvc pipeline step new --step-name world --command "echo world"
# $ xvc pipeline step new --step-name hello --command "echo hello"
# $ xvc pipeline step dependency --step-name world --step hello
# $ xvc pipeline step dependency --step-name hello --step file-dependency
# ```
#
# When run, the dependency will be run first and the step will be run after.
#
# ```console
# $ xvc pipeline run
# [OUT] data.txt has changed
#
# [OUT] [EXIT] Successfully
#
# ```
#
# ### Generic Command Dependencies
#
# You can use the output of a command as a dependency to a step. When the command is run, the output hash is saved to
# compare and to invalidate the step when the output has changed.
#
# You can use this for any command that outputs a string.
#
# ```console
# $ xvc pipeline new --name generic
#
# $ xvc pipeline --name generic step new --step-name yearly --command "echo 'Happy New Year! Welcome `(date +%Y)`!'"
#
# $ xvc  pipeline --name generic step dependency --step-name yearly --generic 'date +%Y'
#
# ```
#
# ```console
# $ xvc pipeline --name generic export
# {
#   "name": "generic",
#   "steps": [
#     {
#       "command": "echo 'Happy New Year! Welcome `(date +%Y)`!'",
#       "dependencies": [
#         {
#           "Generic": {
#             "generic_command": "date +%Y"
#           }
#         }
#       ],
#       "invalidate": "ByDependencies",
#       "name": "yearly",
#       "outputs": []
#     }
#   ],
#   "version": 1,
#   "workdir": ""
# }
#
# ```
#
# When the year changes, the step is invalidated and run again.
#
# ```console
# $ xvc pipeline --name generic run
# [OUT] Happy New Year! Welcome `(date +%Y)`!
#
# [OUT] [EXIT] Successfully
#
# ```
#
# The step won't run until the next year.
#
# ```console
# $ xvc pipeline --name generic run
#
# ```
#
# ### Directory Dependencies
#
# You can specify a directory in the Xvc repository as a dependency to a step. When the directory changes, the step is
# invalidated and run again.
#
# We'll run the following commands in the `examples` directory.
#
# ```console
# $ xvc-test-helper create-directory-tree --directories 2 --files 3 --seed 20230323
# $ tree
# .
# ├── dir-0001
# │   ├── file-0001.bin
# │   ├── file-0002.bin
# │   └── file-0003.bin
# └── dir-0002
#     ├── file-0001.bin
#     ├── file-0002.bin
#     └── file-0003.bin
#
# 3 directories, 6 files
#
# ```
#
# ```console
# $ xvc pipeline new --name directory-example
# $ xvc pipeline --name directory-example step new --step-name directory-step --command "echo 'Directory has changed'"
# $ xvc pipeline --name directory-example step dependency --step-name directory-step --directory dir-0001/
# ```
#
# When you define the pipeline for the first time, it will run the step.
#
# ```console
# $ xvc pipeline --name directory-example run
# [DEBUG][logging/src/lib.rs::236] Terminal logger enabled with level: Trace
# [TRACE][core/src/types/xvcroot.rs::247] "."
# [DEBUG][core/src/types/xvcroot.rs::253] XVC DIR: "[CWD]"
# [DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
# [DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
# [TRACE][config/src/lib.rs::536] cli_config: [
#     "core.verbosity = debug",
#     "core.quiet = false",
# ]
# [TRACE][config/src/lib.rs::540] map: {
#     "core.quiet": Boolean(
#         false,
#     ),
#     "core.verbosity": String(
#         "debug",
#     ),
# }
# [TRACE][config/src/lib.rs::543] conf: XvcConfig {
#     current_dir: XvcConfigOption {
#         source: Runtime,
#         option: AbsolutePath(
#             "[CWD]",
#         ),
#     },
#     config_maps: [
#         XvcConfigMap {
#             source: Default,
#             map: {
#                 "file.carry-in.no_parallel": Boolean(
#                     false,
#                 ),
#                 "pipeline.current_pipeline": String(
#                     "default",
#                 ),
#                 "file.track.no_parallel": Boolean(
#                     false,
#                 ),
#                 "file.track.no_commit": Boolean(
#                     false,
#                 ),
#                 "git.auto_commit": Boolean(
#                     true,
#                 ),
#                 "pipeline.default_params_file": String(
#                     "params.yaml",
#                 ),
#                 "pipeline.default": String(
#                     "default",
#                 ),
#                 "git.auto_stage": Boolean(
#                     false,
#                 ),
#                 "file.list.format": String(
#                     "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
#                 ),
#                 "file.carry-in.force": Boolean(
#                     false,
#                 ),
#                 "file.track.text_or_binary": String(
#                     "auto",
#                 ),
#                 "core.guid": String(
#                     "81c9c84d280d4ec9",
#                 ),
#                 "file.recheck.method": String(
#                     "copy",
#                 ),
#                 "file.list.no_summary": Boolean(
#                     false,
#                 ),
#                 "file.list.sort": String(
#                     "name-desc",
#                 ),
#                 "file.track.force": Boolean(
#                     false,
#                 ),
#                 "core.verbosity": String(
#                     "error",
#                 ),
#                 "git.use_git": Boolean(
#                     true,
#                 ),
#                 "git.command": String(
#                     "git",
#                 ),
#                 "file.list.recursive": Boolean(
#                     false,
#                 ),
#                 "cache.algorithm": String(
#                     "blake3",
#                 ),
#             },
#         },
#         XvcConfigMap {
#             source: Project,
#             map: {
#                 "file.track.force": Boolean(
#                     false,
#                 ),
#                 "pipeline.default_params_file": String(
#                     "params.yaml",
#                 ),
#                 "file.track.no_commit": Boolean(
#                     false,
#                 ),
#                 "file.list.no_summary": Boolean(
#                     false,
#                 ),
#                 "file.list.format": String(
#                     "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
#                 ),
#                 "file.list.recursive": Boolean(
#                     false,
#                 ),
#                 "file.carry-in.no_parallel": Boolean(
#                     false,
#                 ),
#                 "cache.algorithm": String(
#                     "blake3",
#                 ),
#                 "core.guid": String(
#                     "29f321b4bb945201",
#                 ),
#                 "pipeline.current_pipeline": String(
#                     "default",
#                 ),
#                 "git.auto_commit": Boolean(
#                     true,
#                 ),
#                 "file.list.sort": String(
#                     "name-desc",
#                 ),
#                 "core.verbosity": String(
#                     "error",
#                 ),
#                 "file.carry-in.force": Boolean(
#                     false,
#                 ),
#                 "git.use_git": Boolean(
#                     true,
#                 ),
#                 "git.auto_stage": Boolean(
#                     false,
#                 ),
#                 "git.command": String(
#                     "git",
#                 ),
#                 "file.track.text_or_binary": String(
#                     "auto",
#                 ),
#                 "pipeline.default": String(
#                     "default",
#                 ),
#                 "file.track.no_parallel": Boolean(
#                     false,
#                 ),
#                 "file.recheck.method": String(
#                     "copy",
#                 ),
#             },
#         },
#         XvcConfigMap {
#             source: Local,
#             map: {},
#         },
#         XvcConfigMap {
#             source: Environment,
#             map: {},
#         },
#         XvcConfigMap {
#             source: CommandLine,
#             map: {
#                 "core.quiet": Boolean(
#                     false,
#                 ),
#                 "core.verbosity": String(
#                     "debug",
#                 ),
#             },
#         },
#     ],
#     the_config: {
#         "cache.algorithm": XvcConfigValue {
#             source: Project,
#             value: String(
#                 "blake3",
#             ),
#         },
#         "git.use_git": XvcConfigValue {
#             source: Project,
#             value: Boolean(
#                 true,
#             ),
#         },
#         "file.carry-in.force": XvcConfigValue {
#             source: Project,
#             value: Boolean(
#                 false,
#             ),
#         },
#         "pipeline.default": XvcConfigValue {
#             source: Project,
#             value: String(
#                 "default",
#             ),
#         },
#         "git.command": XvcConfigValue {
#             source: Project,
#             value: String(
#                 "git",
#             ),
#         },
#         "core.quiet": XvcConfigValue {
#             source: CommandLine,
#             value: Boolean(
#                 false,
#             ),
#         },
#         "file.track.text_or_binary": XvcConfigValue {
#             source: Project,
#             value: String(
#                 "auto",
#             ),
#         },
#         "file.carry-in.no_parallel": XvcConfigValue {
#             source: Project,
#             value: Boolean(
#                 false,
#             ),
#         },
#         "git.auto_stage": XvcConfigValue {
#             source: Project,
#             value: Boolean(
#                 false,
#             ),
#         },
#         "file.track.no_commit": XvcConfigValue {
#             source: Project,
#             value: Boolean(
#                 false,
#             ),
#         },
#         "file.track.no_parallel": XvcConfigValue {
#             source: Project,
#             value: Boolean(
#                 false,
#             ),
#         },
#         "file.recheck.method": XvcConfigValue {
#             source: Project,
#             value: String(
#                 "copy",
#             ),
#         },
#         "file.list.no_summary": XvcConfigValue {
#             source: Project,
#             value: Boolean(
#                 false,
#             ),
#         },
#         "git.auto_commit": XvcConfigValue {
#             source: Project,
#             value: Boolean(
#                 true,
#             ),
#         },
#         "pipeline.default_params_file": XvcConfigValue {
#             source: Project,
#             value: String(
#                 "params.yaml",
#             ),
#         },
#         "core.verbosity": XvcConfigValue {
#             source: CommandLine,
#             value: String(
#                 "debug",
#             ),
#         },
#         "file.list.sort": XvcConfigValue {
#             source: Project,
#             value: String(
#                 "name-desc",
#             ),
#         },
#         "file.list.format": XvcConfigValue {
#             source: Project,
#             value: String(
#                 "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
#             ),
#         },
#         "pipeline.current_pipeline": XvcConfigValue {
#             source: Project,
#             value: String(
#                 "default",
#             ),
#         },
#         "file.track.force": XvcConfigValue {
#             source: Project,
#             value: Boolean(
#                 false,
#             ),
#         },
#         "core.guid": XvcConfigValue {
#             source: Project,
#             value: String(
#                 "29f321b4bb945201",
#             ),
#         },
#         "file.list.recursive": XvcConfigValue {
#             source: Project,
#             value: Boolean(
#                 false,
#             ),
#         },
#     },
#     init_params: XvcConfigInitParams {
#         default_configuration: "
# [core]
# # The repository id. Please do not delete or change it.
# # This is used to identify the repository and generate paths in storages.
# # In the future it may be used to in other ways.
# guid = /"81c9c84d280d4ec9/"
# # Default verbosity level.
# # One of /"error/", /"warn/", /"info/"
# verbosity = /"error/"
#
# [git]
# # Automate git operations.
# # Turning this off leads Xvc to behave as if it's not in a Git repository.
# # Not recommended unless you're really not using Git
# use_git = true
# # Command to run Git process.
# # You can set this to an absolute path to specify an executable
# # If set to a non-absolute path, the executable will be searched in $PATH.
# command = /"git/"
#
# # Commit changes in .xvc/ directory after commands.
# # You can set this to false if you want to commit manually.
# auto_commit = true
#
# # Stage changes in .xvc/ directory without committing.
# # auto_commit implies auto_stage.
# # If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
# auto_stage = false
#
# [cache]
# # The hash algorithm used for the cache.
# # It may take blake3, blake2, sha2 or sha3 as values.
# # All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# # The cache path is produced by prepending algorithm name to the cache.
# # Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
# algorithm = /"blake3/"
#
# [file]
#
# [file.track]
#
# # Don't move file content to cache after xvc file track
# no_commit = false
# # Force to track files even if they are already tracked.
# force = false
#
# # Xvc calculates file content digest differently for text and binary files.
# # This option controls whether to treat files as text or binary.
# # It may take auto, text or binary as values.
# # Auto check each file individually and treat it as text if it's text.
# text_or_binary = /"auto/"
#
# # Don't use parallelism in track operations.
# # Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
# no_parallel = false
#
# [file.list]
#
# # Format for `xvc file list` rows. You can reorder or remove columns.
# # The following are the keys for each row:
# # - {acd64}:  actual content digest. All 64 digits from the workspace file's content.
# # - {acd8}:  actual content digest. First 8 digits the file content digest.
# # - {aft}:  actual file type. Whether the entry is a file (F), directory (D),
# #   symlink (S), hardlink (H) or reflink (R).
# # - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,
# #   GB and TB to represent sizes larger than 1MB.
# # - {ats}:  actual timestamp. The timestamp of the workspace file.
# # - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show
# #   whether the file timestamp is the same as the cached timestamp, newer,
# #   older, not cached or not tracked.
# # - {name}: The name of the file or directory.
# # - {rcd64}:  recorded content digest. All 64 digits.
# # - {rcd8}:  recorded content digest. First 8 digits.
# # - {rrm}:  recorded recheck method. Whether the entry is linked to the workspace
# #   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# # - {rsz}:  recorded size. The size of the cached content in bytes. It uses
# #   MB, GB and TB to represent sizes larged than 1MB.
# # - {rts}:  recorded timestamp. The timestamp of the cached content.
# #
# # There are no escape sequences in the format string.
# # If you want to add a tab, type it to the string.
# # If you want to add a literal double curly brace, open an issue.
# format = /"{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"
#
# # Default sort order for `xvc file list`.
# # Valid values are
# # none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
# sort = /"name-desc/"
#
# # Do not show a summary for as the final row for `xvc file list`.
# no_summary = false
#
# # List files recursively always.
# recursive = false
#
# [file.carry-in]
# # Carry-in the files to cache always, even if they are already present.
# force = false
#
# # Don't use parallel move/copy in carry-in
# no_parallel = false
#
# [file.recheck]
# # The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# # The default is copy to make sure the options is portable.
# # Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# # Note that hardlink and symlink are read-only as they link the files in cache.
# method = /"copy/"
#
# [pipeline]
# # Name of the current pipeline to run
# current_pipeline = /"default/"
# # Name of the default pipeline
# default = /"default/"
# # Name of the default params file name
# default_params_file = /"params.yaml/"
#
# ",
#         current_dir: AbsolutePath(
#             "[CWD]",
#         ),
#         include_system_config: true,
#         include_user_config: true,
#         project_config_path: Some(
#             AbsolutePath(
#                 "[CWD]/.xvc/config.toml",
#             ),
#         ),
#         local_config_path: Some(
#             AbsolutePath(
#                 "[CWD]/.xvc/config.local.toml",
#             ),
#         ),
#         include_environment_config: true,
#         command_line_config: Some(
#             [
#                 "core.verbosity = debug",
#                 "core.quiet = false",
#             ],
#         ),
#     },
# }
# [TRACE][ecs/src/ecs/mod.rs::229] dir: "[CWD]/.xvc/ec"
# [TRACE][ecs/src/ecs/mod.rs::239] files: [
#     "[CWD]/.xvc/ec/1679697331513529",
#     "[CWD]/.xvc/ec/1679697331523891",
#     "[CWD]/.xvc/ec/1679697331682830",
#     "[CWD]/.xvc/ec/1679697331826445",
#     "[CWD]/.xvc/ec/1679697333061487",
#     "[CWD]/.xvc/ec/1679697335349615",
#     "[CWD]/.xvc/ec/1679697335517844",
#     "[CWD]/.xvc/ec/1679697335648099",
#     "[CWD]/.xvc/ec/1679697335761987",
#     "[CWD]/.xvc/ec/1679697336594123",
#     "[CWD]/.xvc/ec/1679697336718861",
#     "[CWD]/.xvc/ec/1679697336825808",
#     "[CWD]/.xvc/ec/1679697338189209",
#     "[CWD]/.xvc/ec/1679697338266097",
#     "[CWD]/.xvc/ec/1679697338357110",
# ]
# [TRACE][pipeline/src/lib.rs::309] name: Some(
#     "directory-example",
# )
# [DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
# [TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/dir-0001"
# [TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/.gitignore"
# [TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/dir-0002"
# [TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/.xvcignore"
# [TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/dir-0001/file-0002.bin"
# [TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/dir-0001/file-0003.bin"
# [TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/dir-0001/file-0001.bin"
# [TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/dir-0002/file-0002.bin"
# [TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/dir-0002/file-0003.bin"
# [TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::85] abs_path: "[CWD]/dir-0002/file-0001.bin"
# [TRACE][core/src/types/xvcpath.rs::86] current_dir: AbsolutePath(
#     "[CWD]",
# )
# [TRACE][core/src/types/xvcpath.rs::87] xvc_root.absolute_path(): AbsolutePath(
#     "[CWD]",
# )
# [TRACE][walker/src/notify.rs::160] watcher: FsEventWatcher {
#     paths: 0x0000000140905580,
#     since_when: 18446744073709551615,
#     latency: 0.0,
#     flags: 18,
#     event_handler: 0x0000000140904d50,
#     runloop: Some(
#         (
#             0x0000000140a04d30,
#             JoinHandle { .. },
#         ),
#     ),
#     recursive_info: {
#         "[CWD]": true,
#     },
# }
# [TRACE][pipeline/src/pipeline/mod.rs::263] pipeline_len: 1
# [TRACE][pipeline/src/pipeline/mod.rs::289] &dependency_graph: {
#     XvcEntity(
#         13,
#         232039266569345540,
#     ): [],
# }
# [INFO][pipeline/src/pipeline/mod.rs::303] Pipeline Graph:
# digraph {
#     0 [ label = "(13, 232039266569345540)" ]
# }
#
#
# [TRACE][pipeline/src/pipeline/mod.rs::375] step_states: HStore {
#     map: {
#         XvcEntity(
#             13,
#             232039266569345540,
#         ): Begin(
#             FromInit,
#         ),
#     },
# }
# [TRACE][pipeline/src/pipeline/mod.rs::409] (step_e, step_s): (
#     XvcEntity(
#         13,
#         232039266569345540,
#     ),
#     Begin(
#         FromInit,
#     ),
# )
# [TRACE][pipeline/src/pipeline/mod.rs::474] dependency_diffs: HStore {
#     map: {},
# }
# [TRACE][pipeline/src/pipeline/mod.rs::409] (step_e, step_s): (
#     XvcEntity(
#         13,
#         232039266569345540,
#     ),
#     WaitingDependencySteps(
#         FromRunConditional,
#     ),
# )
# [TRACE][pipeline/src/pipeline/mod.rs::474] dependency_diffs: HStore {
#     map: {},
# }
# [TRACE][pipeline/src/pipeline/mod.rs::409] (step_e, step_s): (
#     XvcEntity(
#         13,
#         232039266569345540,
#     ),
#     CheckingMissingDependencies(
#         FromDependencyStepsFinishedSuccessfully,
#     ),
# )
# [TRACE][pipeline/src/pipeline/mod.rs::474] dependency_diffs: HStore {
#     map: {},
# }
# [TRACE][pipeline/src/pipeline/mod.rs::409] (step_e, step_s): (
#     XvcEntity(
#         13,
#         232039266569345540,
#     ),
#     CheckingMissingOutputs(
#         FromNoMissingDependencies,
#     ),
# )
# [TRACE][pipeline/src/pipeline/mod.rs::474] dependency_diffs: HStore {
#     map: {},
# }
# [TRACE][pipeline/src/pipeline/mod.rs::409] (step_e, step_s): (
#     XvcEntity(
#         13,
#         232039266569345540,
#     ),
#     CheckingTimestamps(
#         FromHasNoMissingOutputs,
#     ),
# )
# [TRACE][pipeline/src/pipeline/mod.rs::474] dependency_diffs: HStore {
#     map: {},
# }
# [TRACE][pipeline/src/pipeline/mod.rs::409] (step_e, step_s): (
#     XvcEntity(
#         13,
#         232039266569345540,
#     ),
#     CheckingDependencyContentDigest(
#         FromHasNoNewerDependencies,
#     ),
# )
# [TRACE][pipeline/src/pipeline/mod.rs::450] dependency_comparison_params: DependencyComparisonParams {
#     xvc_root: XvcRootInner {
#         absolute_path: AbsolutePath(
#             "[CWD]",
#         ),
#         xvc_dir: AbsolutePath(
#             "[CWD]/.xvc",
#         ),
#         store_dir: AbsolutePath(
#             "[CWD]/.xvc/store",
#         ),
#         config: XvcConfig {
#             current_dir: XvcConfigOption {
#                 source: Runtime,
#                 option: AbsolutePath(
#                     "[CWD]",
#                 ),
#             },
#             config_maps: [
#                 XvcConfigMap {
#                     source: Default,
#                     map: {
#                         "file.carry-in.no_parallel": Boolean(
#                             false,
#                         ),
#                         "pipeline.current_pipeline": String(
#                             "default",
#                         ),
#                         "file.track.no_parallel": Boolean(
#                             false,
#                         ),
#                         "file.track.no_commit": Boolean(
#                             false,
#                         ),
#                         "git.auto_commit": Boolean(
#                             true,
#                         ),
#                         "pipeline.default_params_file": String(
#                             "params.yaml",
#                         ),
#                         "pipeline.default": String(
#                             "default",
#                         ),
#                         "git.auto_stage": Boolean(
#                             false,
#                         ),
#                         "file.list.format": String(
#                             "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
#                         ),
#                         "file.carry-in.force": Boolean(
#                             false,
#                         ),
#                         "file.track.text_or_binary": String(
#                             "auto",
#                         ),
#                         "core.guid": String(
#                             "81c9c84d280d4ec9",
#                         ),
#                         "file.recheck.method": String(
#                             "copy",
#                         ),
#                         "file.list.no_summary": Boolean(
#                             false,
#                         ),
#                         "file.list.sort": String(
#                             "name-desc",
#                         ),
#                         "file.track.force": Boolean(
#                             false,
#                         ),
#                         "core.verbosity": String(
#                             "error",
#                         ),
#                         "git.use_git": Boolean(
#                             true,
#                         ),
#                         "git.command": String(
#                             "git",
#                         ),
#                         "file.list.recursive": Boolean(
#                             false,
#                         ),
#                         "cache.algorithm": String(
#                             "blake3",
#                         ),
#                     },
#                 },
#                 XvcConfigMap {
#                     source: Project,
#                     map: {
#                         "file.track.force": Boolean(
#                             false,
#                         ),
#                         "pipeline.default_params_file": String(
#                             "params.yaml",
#                         ),
#                         "file.track.no_commit": Boolean(
#                             false,
#                         ),
#                         "file.list.no_summary": Boolean(
#                             false,
#                         ),
#                         "file.list.format": String(
#                             "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
#                         ),
#                         "file.list.recursive": Boolean(
#                             false,
#                         ),
#                         "file.carry-in.no_parallel": Boolean(
#                             false,
#                         ),
#                         "cache.algorithm": String(
#                             "blake3",
#                         ),
#                         "core.guid": String(
#                             "29f321b4bb945201",
#                         ),
#                         "pipeline.current_pipeline": String(
#                             "default",
#                         ),
#                         "git.auto_commit": Boolean(
#                             true,
#                         ),
#                         "file.list.sort": String(
#                             "name-desc",
#                         ),
#                         "core.verbosity": String(
#                             "error",
#                         ),
#                         "file.carry-in.force": Boolean(
#                             false,
#                         ),
#                         "git.use_git": Boolean(
#                             true,
#                         ),
#                         "git.auto_stage": Boolean(
#                             false,
#                         ),
#                         "git.command": String(
#                             "git",
#                         ),
#                         "file.track.text_or_binary": String(
#                             "auto",
#                         ),
#                         "pipeline.default": String(
#                             "default",
#                         ),
#                         "file.track.no_parallel": Boolean(
#                             false,
#                         ),
#                         "file.recheck.method": String(
#                             "copy",
#                         ),
#                     },
#                 },
#                 XvcConfigMap {
#                     source: Local,
#                     map: {},
#                 },
#                 XvcConfigMap {
#                     source: Environment,
#                     map: {},
#                 },
#                 XvcConfigMap {
#                     source: CommandLine,
#                     map: {
#                         "core.quiet": Boolean(
#                             false,
#                         ),
#                         "core.verbosity": String(
#                             "debug",
#                         ),
#                     },
#                 },
#             ],
#             the_config: {
#                 "cache.algorithm": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "blake3",
#                     ),
#                 },
#                 "git.use_git": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         true,
#                     ),
#                 },
#                 "file.carry-in.force": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "pipeline.default": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "default",
#                     ),
#                 },
#                 "git.command": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "git",
#                     ),
#                 },
#                 "core.quiet": XvcConfigValue {
#                     source: CommandLine,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "file.track.text_or_binary": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "auto",
#                     ),
#                 },
#                 "file.carry-in.no_parallel": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "git.auto_stage": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "file.track.no_commit": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "file.track.no_parallel": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "file.recheck.method": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "copy",
#                     ),
#                 },
#                 "file.list.no_summary": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "git.auto_commit": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         true,
#                     ),
#                 },
#                 "pipeline.default_params_file": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "params.yaml",
#                     ),
#                 },
#                 "core.verbosity": XvcConfigValue {
#                     source: CommandLine,
#                     value: String(
#                         "debug",
#                     ),
#                 },
#                 "file.list.sort": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "name-desc",
#                     ),
#                 },
#                 "file.list.format": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
#                     ),
#                 },
#                 "pipeline.current_pipeline": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "default",
#                     ),
#                 },
#                 "file.track.force": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "core.guid": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "29f321b4bb945201",
#                     ),
#                 },
#                 "file.list.recursive": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#             },
#             init_params: XvcConfigInitParams {
#                 default_configuration: "
# [core]
# # The repository id. Please do not delete or change it.
# # This is used to identify the repository and generate paths in storages.
# # In the future it may be used to in other ways.
# guid = /"81c9c84d280d4ec9/"
# # Default verbosity level.
# # One of /"error/", /"warn/", /"info/"
# verbosity = /"error/"
#
# [git]
# # Automate git operations.
# # Turning this off leads Xvc to behave as if it's not in a Git repository.
# # Not recommended unless you're really not using Git
# use_git = true
# # Command to run Git process.
# # You can set this to an absolute path to specify an executable
# # If set to a non-absolute path, the executable will be searched in $PATH.
# command = /"git/"
#
# # Commit changes in .xvc/ directory after commands.
# # You can set this to false if you want to commit manually.
# auto_commit = true
#
# # Stage changes in .xvc/ directory without committing.
# # auto_commit implies auto_stage.
# # If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
# auto_stage = false
#
# [cache]
# # The hash algorithm used for the cache.
# # It may take blake3, blake2, sha2 or sha3 as values.
# # All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# # The cache path is produced by prepending algorithm name to the cache.
# # Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
# algorithm = /"blake3/"
#
# [file]
#
# [file.track]
#
# # Don't move file content to cache after xvc file track
# no_commit = false
# # Force to track files even if they are already tracked.
# force = false
#
# # Xvc calculates file content digest differently for text and binary files.
# # This option controls whether to treat files as text or binary.
# # It may take auto, text or binary as values.
# # Auto check each file individually and treat it as text if it's text.
# text_or_binary = /"auto/"
#
# # Don't use parallelism in track operations.
# # Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
# no_parallel = false
#
# [file.list]
#
# # Format for `xvc file list` rows. You can reorder or remove columns.
# # The following are the keys for each row:
# # - {acd64}:  actual content digest. All 64 digits from the workspace file's content.
# # - {acd8}:  actual content digest. First 8 digits the file content digest.
# # - {aft}:  actual file type. Whether the entry is a file (F), directory (D),
# #   symlink (S), hardlink (H) or reflink (R).
# # - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,
# #   GB and TB to represent sizes larger than 1MB.
# # - {ats}:  actual timestamp. The timestamp of the workspace file.
# # - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show
# #   whether the file timestamp is the same as the cached timestamp, newer,
# #   older, not cached or not tracked.
# # - {name}: The name of the file or directory.
# # - {rcd64}:  recorded content digest. All 64 digits.
# # - {rcd8}:  recorded content digest. First 8 digits.
# # - {rrm}:  recorded recheck method. Whether the entry is linked to the workspace
# #   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# # - {rsz}:  recorded size. The size of the cached content in bytes. It uses
# #   MB, GB and TB to represent sizes larged than 1MB.
# # - {rts}:  recorded timestamp. The timestamp of the cached content.
# #
# # There are no escape sequences in the format string.
# # If you want to add a tab, type it to the string.
# # If you want to add a literal double curly brace, open an issue.
# format = /"{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"
#
# # Default sort order for `xvc file list`.
# # Valid values are
# # none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
# sort = /"name-desc/"
#
# # Do not show a summary for as the final row for `xvc file list`.
# no_summary = false
#
# # List files recursively always.
# recursive = false
#
# [file.carry-in]
# # Carry-in the files to cache always, even if they are already present.
# force = false
#
# # Don't use parallel move/copy in carry-in
# no_parallel = false
#
# [file.recheck]
# # The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# # The default is copy to make sure the options is portable.
# # Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# # Note that hardlink and symlink are read-only as they link the files in cache.
# method = /"copy/"
#
# [pipeline]
# # Name of the current pipeline to run
# current_pipeline = /"default/"
# # Name of the default pipeline
# default = /"default/"
# # Name of the default params file name
# default_params_file = /"params.yaml/"
#
# ",
#                 current_dir: AbsolutePath(
#                     "[CWD]",
#                 ),
#                 include_system_config: true,
#                 include_user_config: true,
#                 project_config_path: Some(
#                     AbsolutePath(
#                         "[CWD]/.xvc/config.toml",
#                     ),
#                 ),
#                 local_config_path: Some(
#                     AbsolutePath(
#                         "[CWD]/.xvc/config.local.toml",
#                     ),
#                 ),
#                 include_environment_config: true,
#                 command_line_config: Some(
#                     [
#                         "core.verbosity = debug",
#                         "core.quiet = false",
#                     ],
#                 ),
#             },
#         },
#         local_config_path: AbsolutePath(
#             "[CWD]/.xvc/config.local.toml",
#         ),
#         project_config_path: AbsolutePath(
#             "[CWD]/.xvc/config.toml",
#         ),
#         entity_generator: XvcEntityGenerator {
#             counter: 15,
#             random: 9956448680558698430,
#             dirty: false,
#         },
#     },
#     pipeline_rundir: XvcPath(
#         "",
#     ),
#     pmm: {
#         XvcPath(
#             ".xvcignore",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 130,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697331,
#                     tv_nsec: 513702324,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0001",
#         ): XvcMetadata {
#             file_type: Directory,
#             size: Some(
#                 160,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 141528876,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0001/file-0003.bin",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 1003,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 141628877,
#                 },
#             ),
#         },
#         XvcPath(
#             ".gitignore",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 107,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697331,
#                     tv_nsec: 513831701,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0001/file-0001.bin",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 1001,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 141322957,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0002",
#         ): XvcMetadata {
#             file_type: Directory,
#             size: Some(
#                 160,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 141961507,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0002/file-0001.bin",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 1001,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 141796130,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0002/file-0003.bin",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 1003,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 142058633,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0001/file-0002.bin",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 1002,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 141494376,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0002/file-0002.bin",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 1002,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 141929507,
#                 },
#             ),
#         },
#     },
#     algorithm: Blake3,
#     all_dependencies: XvcStore {
#         map: {
#             XvcEntity(
#                 3,
#                 5108033101573966617,
#             ): File {
#                 path: XvcPath(
#                     "data.txt",
#                 ),
#             },
#             XvcEntity(
#                 4,
#                 7100715853232020034,
#             ): File {
#                 path: XvcPath(
#                     "data2.txt",
#                 ),
#             },
#             XvcEntity(
#                 7,
#                 13794036064244473358,
#             ): Step {
#                 name: "hello",
#             },
#             XvcEntity(
#                 8,
#                 4916406749229324471,
#             ): Step {
#                 name: "file-dependency",
#             },
#             XvcEntity(
#                 11,
#                 2286334430150808884,
#             ): Generic {
#                 generic_command: "date +%Y",
#             },
#             XvcEntity(
#                 14,
#                 16038783588203922350,
#             ): Directory {
#                 path: XvcPath(
#                     "dir-0001",
#                 ),
#             },
#         },
#         entity_index: {
#             Generic {
#                 generic_command: "date +%Y",
#             }: [
#                 XvcEntity(
#                     11,
#                     2286334430150808884,
#                 ),
#             ],
#             Step {
#                 name: "file-dependency",
#             }: [
#                 XvcEntity(
#                     8,
#                     4916406749229324471,
#                 ),
#             ],
#             Step {
#                 name: "hello",
#             }: [
#                 XvcEntity(
#                     7,
#                     13794036064244473358,
#                 ),
#             ],
#             File {
#                 path: XvcPath(
#                     "data.txt",
#                 ),
#             }: [
#                 XvcEntity(
#                     3,
#                     5108033101573966617,
#                 ),
#             ],
#             File {
#                 path: XvcPath(
#                     "data2.txt",
#                 ),
#             }: [
#                 XvcEntity(
#                     4,
#                     7100715853232020034,
#                 ),
#             ],
#             Directory {
#                 path: XvcPath(
#                     "dir-0001",
#                 ),
#             }: [
#                 XvcEntity(
#                     14,
#                     16038783588203922350,
#                 ),
#             ],
#         },
#         previous: EventLog(
#             [
#                 Add {
#                     entity: XvcEntity(
#                         3,
#                         5108033101573966617,
#                     ),
#                     value: File {
#                         path: XvcPath(
#                             "data.txt",
#                         ),
#                     },
#                 },
#                 Add {
#                     entity: XvcEntity(
#                         4,
#                         7100715853232020034,
#                     ),
#                     value: File {
#                         path: XvcPath(
#                             "data2.txt",
#                         ),
#                     },
#                 },
#                 Add {
#                     entity: XvcEntity(
#                         7,
#                         13794036064244473358,
#                     ),
#                     value: Step {
#                         name: "hello",
#                     },
#                 },
#                 Add {
#                     entity: XvcEntity(
#                         8,
#                         4916406749229324471,
#                     ),
#                     value: Step {
#                         name: "file-dependency",
#                     },
#                 },
#                 Add {
#                     entity: XvcEntity(
#                         11,
#                         2286334430150808884,
#                     ),
#                     value: Generic {
#                         generic_command: "date +%Y",
#                     },
#                 },
#                 Add {
#                     entity: XvcEntity(
#                         14,
#                         16038783588203922350,
#                     ),
#                     value: Directory {
#                         path: XvcPath(
#                             "dir-0001",
#                         ),
#                     },
#                 },
#             ],
#         ),
#         current: EventLog(
#             [],
#         ),
#     },
#     dependency_paths: R1NStore {
#         parents: XvcStore {
#             map: {
#                 XvcEntity(
#                     3,
#                     5108033101573966617,
#                 ): File {
#                     path: XvcPath(
#                         "data.txt",
#                     ),
#                 },
#                 XvcEntity(
#                     4,
#                     7100715853232020034,
#                 ): File {
#                     path: XvcPath(
#                         "data2.txt",
#                     ),
#                 },
#                 XvcEntity(
#                     7,
#                     13794036064244473358,
#                 ): Step {
#                     name: "hello",
#                 },
#                 XvcEntity(
#                     8,
#                     4916406749229324471,
#                 ): Step {
#                     name: "file-dependency",
#                 },
#                 XvcEntity(
#                     11,
#                     2286334430150808884,
#                 ): Generic {
#                     generic_command: "date +%Y",
#                 },
#                 XvcEntity(
#                     14,
#                     16038783588203922350,
#                 ): Directory {
#                     path: XvcPath(
#                         "dir-0001",
#                     ),
#                 },
#             },
#             entity_index: {
#                 Generic {
#                     generic_command: "date +%Y",
#                 }: [
#                     XvcEntity(
#                         11,
#                         2286334430150808884,
#                     ),
#                 ],
#                 Step {
#                     name: "file-dependency",
#                 }: [
#                     XvcEntity(
#                         8,
#                         4916406749229324471,
#                     ),
#                 ],
#                 Step {
#                     name: "hello",
#                 }: [
#                     XvcEntity(
#                         7,
#                         13794036064244473358,
#                     ),
#                 ],
#                 File {
#                     path: XvcPath(
#                         "data.txt",
#                     ),
#                 }: [
#                     XvcEntity(
#                         3,
#                         5108033101573966617,
#                     ),
#                 ],
#                 File {
#                     path: XvcPath(
#                         "data2.txt",
#                     ),
#                 }: [
#                     XvcEntity(
#                         4,
#                         7100715853232020034,
#                     ),
#                 ],
#                 Directory {
#                     path: XvcPath(
#                         "dir-0001",
#                     ),
#                 }: [
#                     XvcEntity(
#                         14,
#                         16038783588203922350,
#                     ),
#                 ],
#             },
#             previous: EventLog(
#                 [
#                     Add {
#                         entity: XvcEntity(
#                             3,
#                             5108033101573966617,
#                         ),
#                         value: File {
#                             path: XvcPath(
#                                 "data.txt",
#                             ),
#                         },
#                     },
#                     Add {
#                         entity: XvcEntity(
#                             4,
#                             7100715853232020034,
#                         ),
#                         value: File {
#                             path: XvcPath(
#                                 "data2.txt",
#                             ),
#                         },
#                     },
#                     Add {
#                         entity: XvcEntity(
#                             7,
#                             13794036064244473358,
#                         ),
#                         value: Step {
#                             name: "hello",
#                         },
#                     },
#                     Add {
#                         entity: XvcEntity(
#                             8,
#                             4916406749229324471,
#                         ),
#                         value: Step {
#                             name: "file-dependency",
#                         },
#                     },
#                     Add {
#                         entity: XvcEntity(
#                             11,
#                             2286334430150808884,
#                         ),
#                         value: Generic {
#                             generic_command: "date +%Y",
#                         },
#                     },
#                     Add {
#                         entity: XvcEntity(
#                             14,
#                             16038783588203922350,
#                         ),
#                         value: Directory {
#                             path: XvcPath(
#                                 "dir-0001",
#                             ),
#                         },
#                     },
#                 ],
#             ),
#             current: EventLog(
#                 [],
#             ),
#         },
#         children: XvcStore {
#             map: {},
#             entity_index: {},
#             previous: EventLog(
#                 [],
#             ),
#             current: EventLog(
#                 [],
#             ),
#         },
#         child_parents: XvcStore {
#             map: {},
#             entity_index: {},
#             previous: EventLog(
#                 [],
#             ),
#             current: EventLog(
#                 [],
#             ),
#         },
#     },
#     xvc_path_store: XvcStore {
#         map: {},
#         entity_index: {},
#         previous: EventLog(
#             [],
#         ),
#         current: EventLog(
#             [],
#         ),
#     },
#     xvc_metadata_store: XvcStore {
#         map: {},
#         entity_index: {},
#         previous: EventLog(
#             [],
#         ),
#         current: EventLog(
#             [],
#         ),
#     },
#     xvc_digests_store: XvcStore {
#         map: {
#             XvcEntity(
#                 11,
#                 2286334430150808884,
#             ): XvcDigests(
#                 {
#                     "stdout-digest": XvcDigest {
#                         algorithm: Blake3,
#                         digest: [
#                             0,
#                             25,
#                             102,
#                             177,
#                             28,
#                             165,
#                             102,
#                             239,
#                             199,
#                             24,
#                             108,
#                             167,
#                             117,
#                             135,
#                             7,
#                             186,
#                             162,
#                             135,
#                             192,
#                             222,
#                             59,
#                             86,
#                             22,
#                             53,
#                             101,
#                             177,
#                             233,
#                             127,
#                             6,
#                             117,
#                             31,
#                             37,
#                         ],
#                     },
#                 },
#             ),
#         },
#         entity_index: {
#             XvcDigests(
#                 {
#                     "stdout-digest": XvcDigest {
#                         algorithm: Blake3,
#                         digest: [
#                             0,
#                             25,
#                             102,
#                             177,
#                             28,
#                             165,
#                             102,
#                             239,
#                             199,
#                             24,
#                             108,
#                             167,
#                             117,
#                             135,
#                             7,
#                             186,
#                             162,
#                             135,
#                             192,
#                             222,
#                             59,
#                             86,
#                             22,
#                             53,
#                             101,
#                             177,
#                             233,
#                             127,
#                             6,
#                             117,
#                             31,
#                             37,
#                         ],
#                     },
#                 },
#             ): [
#                 XvcEntity(
#                     11,
#                     2286334430150808884,
#                 ),
#             ],
#         },
#         previous: EventLog(
#             [
#                 Add {
#                     entity: XvcEntity(
#                         11,
#                         2286334430150808884,
#                     ),
#                     value: XvcDigests(
#                         {
#                             "stdout-digest": XvcDigest {
#                                 algorithm: Blake3,
#                                 digest: [
#                                     0,
#                                     25,
#                                     102,
#                                     177,
#                                     28,
#                                     165,
#                                     102,
#                                     239,
#                                     199,
#                                     24,
#                                     108,
#                                     167,
#                                     117,
#                                     135,
#                                     7,
#                                     186,
#                                     162,
#                                     135,
#                                     192,
#                                     222,
#                                     59,
#                                     86,
#                                     22,
#                                     53,
#                                     101,
#                                     177,
#                                     233,
#                                     127,
#                                     6,
#                                     117,
#                                     31,
#                                     37,
#                                 ],
#                             },
#                         },
#                     ),
#                 },
#             ],
#         ),
#         current: EventLog(
#             [],
#         ),
#     },
#     text_files: XvcStore {
#         map: {},
#         entity_index: {},
#         previous: EventLog(
#             [],
#         ),
#         current: EventLog(
#             [],
#         ),
#     },
# }
# [TRACE][pipeline/src/pipeline/mod.rs::617] dependency_diffs: HStore {
#     map: {},
# }
# [TRACE][pipeline/src/pipeline/mod.rs::618] params.run_conditions.ignore_content_digest_comparison: false
# [TRACE][pipeline/src/pipeline/mod.rs::624] step_e: XvcEntity(
#     13,
#     232039266569345540,
# )
# [TRACE][pipeline/src/pipeline/mod.rs::627] deps: HStore {
#     map: {
#         XvcEntity(
#             14,
#             16038783588203922350,
#         ): Directory {
#             path: XvcPath(
#                 "dir-0001",
#             ),
#         },
#     },
# }
# [TRACE][pipeline/src/pipeline/mod.rs::629] deps.is_empty(): false
# [TRACE][pipeline/src/pipeline/mod.rs::635] _comparison_results: HStore {
#     map: {},
# }
# [TRACE][pipeline/src/pipeline/mod.rs::639] cmp_params: DependencyComparisonParams {
#     xvc_root: XvcRootInner {
#         absolute_path: AbsolutePath(
#             "[CWD]",
#         ),
#         xvc_dir: AbsolutePath(
#             "[CWD]/.xvc",
#         ),
#         store_dir: AbsolutePath(
#             "[CWD]/.xvc/store",
#         ),
#         config: XvcConfig {
#             current_dir: XvcConfigOption {
#                 source: Runtime,
#                 option: AbsolutePath(
#                     "[CWD]",
#                 ),
#             },
#             config_maps: [
#                 XvcConfigMap {
#                     source: Default,
#                     map: {
#                         "file.carry-in.no_parallel": Boolean(
#                             false,
#                         ),
#                         "pipeline.current_pipeline": String(
#                             "default",
#                         ),
#                         "file.track.no_parallel": Boolean(
#                             false,
#                         ),
#                         "file.track.no_commit": Boolean(
#                             false,
#                         ),
#                         "git.auto_commit": Boolean(
#                             true,
#                         ),
#                         "pipeline.default_params_file": String(
#                             "params.yaml",
#                         ),
#                         "pipeline.default": String(
#                             "default",
#                         ),
#                         "git.auto_stage": Boolean(
#                             false,
#                         ),
#                         "file.list.format": String(
#                             "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
#                         ),
#                         "file.carry-in.force": Boolean(
#                             false,
#                         ),
#                         "file.track.text_or_binary": String(
#                             "auto",
#                         ),
#                         "core.guid": String(
#                             "81c9c84d280d4ec9",
#                         ),
#                         "file.recheck.method": String(
#                             "copy",
#                         ),
#                         "file.list.no_summary": Boolean(
#                             false,
#                         ),
#                         "file.list.sort": String(
#                             "name-desc",
#                         ),
#                         "file.track.force": Boolean(
#                             false,
#                         ),
#                         "core.verbosity": String(
#                             "error",
#                         ),
#                         "git.use_git": Boolean(
#                             true,
#                         ),
#                         "git.command": String(
#                             "git",
#                         ),
#                         "file.list.recursive": Boolean(
#                             false,
#                         ),
#                         "cache.algorithm": String(
#                             "blake3",
#                         ),
#                     },
#                 },
#                 XvcConfigMap {
#                     source: Project,
#                     map: {
#                         "file.track.force": Boolean(
#                             false,
#                         ),
#                         "pipeline.default_params_file": String(
#                             "params.yaml",
#                         ),
#                         "file.track.no_commit": Boolean(
#                             false,
#                         ),
#                         "file.list.no_summary": Boolean(
#                             false,
#                         ),
#                         "file.list.format": String(
#                             "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
#                         ),
#                         "file.list.recursive": Boolean(
#                             false,
#                         ),
#                         "file.carry-in.no_parallel": Boolean(
#                             false,
#                         ),
#                         "cache.algorithm": String(
#                             "blake3",
#                         ),
#                         "core.guid": String(
#                             "29f321b4bb945201",
#                         ),
#                         "pipeline.current_pipeline": String(
#                             "default",
#                         ),
#                         "git.auto_commit": Boolean(
#                             true,
#                         ),
#                         "file.list.sort": String(
#                             "name-desc",
#                         ),
#                         "core.verbosity": String(
#                             "error",
#                         ),
#                         "file.carry-in.force": Boolean(
#                             false,
#                         ),
#                         "git.use_git": Boolean(
#                             true,
#                         ),
#                         "git.auto_stage": Boolean(
#                             false,
#                         ),
#                         "git.command": String(
#                             "git",
#                         ),
#                         "file.track.text_or_binary": String(
#                             "auto",
#                         ),
#                         "pipeline.default": String(
#                             "default",
#                         ),
#                         "file.track.no_parallel": Boolean(
#                             false,
#                         ),
#                         "file.recheck.method": String(
#                             "copy",
#                         ),
#                     },
#                 },
#                 XvcConfigMap {
#                     source: Local,
#                     map: {},
#                 },
#                 XvcConfigMap {
#                     source: Environment,
#                     map: {},
#                 },
#                 XvcConfigMap {
#                     source: CommandLine,
#                     map: {
#                         "core.quiet": Boolean(
#                             false,
#                         ),
#                         "core.verbosity": String(
#                             "debug",
#                         ),
#                     },
#                 },
#             ],
#             the_config: {
#                 "cache.algorithm": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "blake3",
#                     ),
#                 },
#                 "git.use_git": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         true,
#                     ),
#                 },
#                 "file.carry-in.force": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "pipeline.default": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "default",
#                     ),
#                 },
#                 "git.command": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "git",
#                     ),
#                 },
#                 "core.quiet": XvcConfigValue {
#                     source: CommandLine,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "file.track.text_or_binary": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "auto",
#                     ),
#                 },
#                 "file.carry-in.no_parallel": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "git.auto_stage": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "file.track.no_commit": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "file.track.no_parallel": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "file.recheck.method": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "copy",
#                     ),
#                 },
#                 "file.list.no_summary": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "git.auto_commit": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         true,
#                     ),
#                 },
#                 "pipeline.default_params_file": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "params.yaml",
#                     ),
#                 },
#                 "core.verbosity": XvcConfigValue {
#                     source: CommandLine,
#                     value: String(
#                         "debug",
#                     ),
#                 },
#                 "file.list.sort": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "name-desc",
#                     ),
#                 },
#                 "file.list.format": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
#                     ),
#                 },
#                 "pipeline.current_pipeline": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "default",
#                     ),
#                 },
#                 "file.track.force": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#                 "core.guid": XvcConfigValue {
#                     source: Project,
#                     value: String(
#                         "29f321b4bb945201",
#                     ),
#                 },
#                 "file.list.recursive": XvcConfigValue {
#                     source: Project,
#                     value: Boolean(
#                         false,
#                     ),
#                 },
#             },
#             init_params: XvcConfigInitParams {
#                 default_configuration: "
# [core]
# # The repository id. Please do not delete or change it.
# # This is used to identify the repository and generate paths in storages.
# # In the future it may be used to in other ways.
# guid = /"81c9c84d280d4ec9/"
# # Default verbosity level.
# # One of /"error/", /"warn/", /"info/"
# verbosity = /"error/"
#
# [git]
# # Automate git operations.
# # Turning this off leads Xvc to behave as if it's not in a Git repository.
# # Not recommended unless you're really not using Git
# use_git = true
# # Command to run Git process.
# # You can set this to an absolute path to specify an executable
# # If set to a non-absolute path, the executable will be searched in $PATH.
# command = /"git/"
#
# # Commit changes in .xvc/ directory after commands.
# # You can set this to false if you want to commit manually.
# auto_commit = true
#
# # Stage changes in .xvc/ directory without committing.
# # auto_commit implies auto_stage.
# # If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
# auto_stage = false
#
# [cache]
# # The hash algorithm used for the cache.
# # It may take blake3, blake2, sha2 or sha3 as values.
# # All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# # The cache path is produced by prepending algorithm name to the cache.
# # Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
# algorithm = /"blake3/"
#
# [file]
#
# [file.track]
#
# # Don't move file content to cache after xvc file track
# no_commit = false
# # Force to track files even if they are already tracked.
# force = false
#
# # Xvc calculates file content digest differently for text and binary files.
# # This option controls whether to treat files as text or binary.
# # It may take auto, text or binary as values.
# # Auto check each file individually and treat it as text if it's text.
# text_or_binary = /"auto/"
#
# # Don't use parallelism in track operations.
# # Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
# no_parallel = false
#
# [file.list]
#
# # Format for `xvc file list` rows. You can reorder or remove columns.
# # The following are the keys for each row:
# # - {acd64}:  actual content digest. All 64 digits from the workspace file's content.
# # - {acd8}:  actual content digest. First 8 digits the file content digest.
# # - {aft}:  actual file type. Whether the entry is a file (F), directory (D),
# #   symlink (S), hardlink (H) or reflink (R).
# # - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,
# #   GB and TB to represent sizes larger than 1MB.
# # - {ats}:  actual timestamp. The timestamp of the workspace file.
# # - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show
# #   whether the file timestamp is the same as the cached timestamp, newer,
# #   older, not cached or not tracked.
# # - {name}: The name of the file or directory.
# # - {rcd64}:  recorded content digest. All 64 digits.
# # - {rcd8}:  recorded content digest. First 8 digits.
# # - {rrm}:  recorded recheck method. Whether the entry is linked to the workspace
# #   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# # - {rsz}:  recorded size. The size of the cached content in bytes. It uses
# #   MB, GB and TB to represent sizes larged than 1MB.
# # - {rts}:  recorded timestamp. The timestamp of the cached content.
# #
# # There are no escape sequences in the format string.
# # If you want to add a tab, type it to the string.
# # If you want to add a literal double curly brace, open an issue.
# format = /"{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"
#
# # Default sort order for `xvc file list`.
# # Valid values are
# # none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
# sort = /"name-desc/"
#
# # Do not show a summary for as the final row for `xvc file list`.
# no_summary = false
#
# # List files recursively always.
# recursive = false
#
# [file.carry-in]
# # Carry-in the files to cache always, even if they are already present.
# force = false
#
# # Don't use parallel move/copy in carry-in
# no_parallel = false
#
# [file.recheck]
# # The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# # The default is copy to make sure the options is portable.
# # Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# # Note that hardlink and symlink are read-only as they link the files in cache.
# method = /"copy/"
#
# [pipeline]
# # Name of the current pipeline to run
# current_pipeline = /"default/"
# # Name of the default pipeline
# default = /"default/"
# # Name of the default params file name
# default_params_file = /"params.yaml/"
#
# ",
#                 current_dir: AbsolutePath(
#                     "[CWD]",
#                 ),
#                 include_system_config: true,
#                 include_user_config: true,
#                 project_config_path: Some(
#                     AbsolutePath(
#                         "[CWD]/.xvc/config.toml",
#                     ),
#                 ),
#                 local_config_path: Some(
#                     AbsolutePath(
#                         "[CWD]/.xvc/config.local.toml",
#                     ),
#                 ),
#                 include_environment_config: true,
#                 command_line_config: Some(
#                     [
#                         "core.verbosity = debug",
#                         "core.quiet = false",
#                     ],
#                 ),
#             },
#         },
#         local_config_path: AbsolutePath(
#             "[CWD]/.xvc/config.local.toml",
#         ),
#         project_config_path: AbsolutePath(
#             "[CWD]/.xvc/config.toml",
#         ),
#         entity_generator: XvcEntityGenerator {
#             counter: 15,
#             random: 9956448680558698430,
#             dirty: false,
#         },
#     },
#     pipeline_rundir: XvcPath(
#         "",
#     ),
#     pmm: {
#         XvcPath(
#             ".xvcignore",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 130,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697331,
#                     tv_nsec: 513702324,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0001",
#         ): XvcMetadata {
#             file_type: Directory,
#             size: Some(
#                 160,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 141528876,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0001/file-0003.bin",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 1003,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 141628877,
#                 },
#             ),
#         },
#         XvcPath(
#             ".gitignore",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 107,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697331,
#                     tv_nsec: 513831701,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0001/file-0001.bin",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 1001,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 141322957,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0002",
#         ): XvcMetadata {
#             file_type: Directory,
#             size: Some(
#                 160,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 141961507,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0002/file-0001.bin",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 1001,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 141796130,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0002/file-0003.bin",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 1003,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 142058633,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0001/file-0002.bin",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 1002,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 141494376,
#                 },
#             ),
#         },
#         XvcPath(
#             "dir-0002/file-0002.bin",
#         ): XvcMetadata {
#             file_type: File,
#             size: Some(
#                 1002,
#             ),
#             modified: Some(
#                 SystemTime {
#                     tv_sec: 1679697338,
#                     tv_nsec: 141929507,
#                 },
#             ),
#         },
#     },
#     algorithm: Blake3,
#     all_dependencies: XvcStore {
#         map: {
#             XvcEntity(
#                 3,
#                 5108033101573966617,
#             ): File {
#                 path: XvcPath(
#                     "data.txt",
#                 ),
#             },
#             XvcEntity(
#                 4,
#                 7100715853232020034,
#             ): File {
#                 path: XvcPath(
#                     "data2.txt",
#                 ),
#             },
#             XvcEntity(
#                 7,
#                 13794036064244473358,
#             ): Step {
#                 name: "hello",
#             },
#             XvcEntity(
#                 8,
#                 4916406749229324471,
#             ): Step {
#                 name: "file-dependency",
#             },
#             XvcEntity(
#                 11,
#                 2286334430150808884,
#             ): Generic {
#                 generic_command: "date +%Y",
#             },
#             XvcEntity(
#                 14,
#                 16038783588203922350,
#             ): Directory {
#                 path: XvcPath(
#                     "dir-0001",
#                 ),
#             },
#         },
#         entity_index: {
#             Generic {
#                 generic_command: "date +%Y",
#             }: [
#                 XvcEntity(
#                     11,
#                     2286334430150808884,
#                 ),
#             ],
#             Step {
#                 name: "file-dependency",
#             }: [
#                 XvcEntity(
#                     8,
#                     4916406749229324471,
#                 ),
#             ],
#             Step {
#                 name: "hello",
#             }: [
#                 XvcEntity(
#                     7,
#                     13794036064244473358,
#                 ),
#             ],
#             File {
#                 path: XvcPath(
#                     "data.txt",
#                 ),
#             }: [
#                 XvcEntity(
#                     3,
#                     5108033101573966617,
#                 ),
#             ],
#             File {
#                 path: XvcPath(
#                     "data2.txt",
#                 ),
#             }: [
#                 XvcEntity(
#                     4,
#                     7100715853232020034,
#                 ),
#             ],
#             Directory {
#                 path: XvcPath(
#                     "dir-0001",
#                 ),
#             }: [
#                 XvcEntity(
#                     14,
#                     16038783588203922350,
#                 ),
#             ],
#         },
#         previous: EventLog(
#             [
#                 Add {
#                     entity: XvcEntity(
#                         3,
#                         5108033101573966617,
#                     ),
#                     value: File {
#                         path: XvcPath(
#                             "data.txt",
#                         ),
#                     },
#                 },
#                 Add {
#                     entity: XvcEntity(
#                         4,
#                         7100715853232020034,
#                     ),
#                     value: File {
#                         path: XvcPath(
#                             "data2.txt",
#                         ),
#                     },
#                 },
#                 Add {
#                     entity: XvcEntity(
#                         7,
#                         13794036064244473358,
#                     ),
#                     value: Step {
#                         name: "hello",
#                     },
#                 },
#                 Add {
#                     entity: XvcEntity(
#                         8,
#                         4916406749229324471,
#                     ),
#                     value: Step {
#                         name: "file-dependency",
#                     },
#                 },
#                 Add {
#                     entity: XvcEntity(
#                         11,
#                         2286334430150808884,
#                     ),
#                     value: Generic {
#                         generic_command: "date +%Y",
#                     },
#                 },
#                 Add {
#                     entity: XvcEntity(
#                         14,
#                         16038783588203922350,
#                     ),
#                     value: Directory {
#                         path: XvcPath(
#                             "dir-0001",
#                         ),
#                     },
#                 },
#             ],
#         ),
#         current: EventLog(
#             [],
#         ),
#     },
#     dependency_paths: R1NStore {
#         parents: XvcStore {
#             map: {
#                 XvcEntity(
#                     3,
#                     5108033101573966617,
#                 ): File {
#                     path: XvcPath(
#                         "data.txt",
#                     ),
#                 },
#                 XvcEntity(
#                     4,
#                     7100715853232020034,
#                 ): File {
#                     path: XvcPath(
#                         "data2.txt",
#                     ),
#                 },
#                 XvcEntity(
#                     7,
#                     13794036064244473358,
#                 ): Step {
#                     name: "hello",
#                 },
#                 XvcEntity(
#                     8,
#                     4916406749229324471,
#                 ): Step {
#                     name: "file-dependency",
#                 },
#                 XvcEntity(
#                     11,
#                     2286334430150808884,
#                 ): Generic {
#                     generic_command: "date +%Y",
#                 },
#                 XvcEntity(
#                     14,
#                     16038783588203922350,
#                 ): Directory {
#                     path: XvcPath(
#                         "dir-0001",
#                     ),
#                 },
#             },
#             entity_index: {
#                 Generic {
#                     generic_command: "date +%Y",
#                 }: [
#                     XvcEntity(
#                         11,
#                         2286334430150808884,
#                     ),
#                 ],
#                 Step {
#                     name: "file-dependency",
#                 }: [
#                     XvcEntity(
#                         8,
#                         4916406749229324471,
#                     ),
#                 ],
#                 Step {
#                     name: "hello",
#                 }: [
#                     XvcEntity(
#                         7,
#                         13794036064244473358,
#                     ),
#                 ],
#                 File {
#                     path: XvcPath(
#                         "data.txt",
#                     ),
#                 }: [
#                     XvcEntity(
#                         3,
#                         5108033101573966617,
#                     ),
#                 ],
#                 File {
#                     path: XvcPath(
#                         "data2.txt",
#                     ),
#                 }: [
#                     XvcEntity(
#                         4,
#                         7100715853232020034,
#                     ),
#                 ],
#                 Directory {
#                     path: XvcPath(
#                         "dir-0001",
#                     ),
#                 }: [
#                     XvcEntity(
#                         14,
#                         16038783588203922350,
#                     ),
#                 ],
#             },
#             previous: EventLog(
#                 [
#                     Add {
#                         entity: XvcEntity(
#                             3,
#                             5108033101573966617,
#                         ),
#                         value: File {
#                             path: XvcPath(
#                                 "data.txt",
#                             ),
#                         },
#                     },
#                     Add {
#                         entity: XvcEntity(
#                             4,
#                             7100715853232020034,
#                         ),
#                         value: File {
#                             path: XvcPath(
#                                 "data2.txt",
#                             ),
#                         },
#                     },
#                     Add {
#                         entity: XvcEntity(
#                             7,
#                             13794036064244473358,
#                         ),
#                         value: Step {
#                             name: "hello",
#                         },
#                     },
#                     Add {
#                         entity: XvcEntity(
#                             8,
#                             4916406749229324471,
#                         ),
#                         value: Step {
#                             name: "file-dependency",
#                         },
#                     },
#                     Add {
#                         entity: XvcEntity(
#                             11,
#                             2286334430150808884,
#                         ),
#                         value: Generic {
#                             generic_command: "date +%Y",
#                         },
#                     },
#                     Add {
#                         entity: XvcEntity(
#                             14,
#                             16038783588203922350,
#                         ),
#                         value: Directory {
#                             path: XvcPath(
#                                 "dir-0001",
#                             ),
#                         },
#                     },
#                 ],
#             ),
#             current: EventLog(
#                 [],
#             ),
#         },
#         children: XvcStore {
#             map: {},
#             entity_index: {},
#             previous: EventLog(
#                 [],
#             ),
#             current: EventLog(
#                 [],
#             ),
#         },
#         child_parents: XvcStore {
#             map: {},
#             entity_index: {},
#             previous: EventLog(
#                 [],
#             ),
#             current: EventLog(
#                 [],
#             ),
#         },
#     },
#     xvc_path_store: XvcStore {
#         map: {},
#         entity_index: {},
#         previous: EventLog(
#             [],
#         ),
#         current: EventLog(
#             [],
#         ),
#     },
#     xvc_metadata_store: XvcStore {
#         map: {},
#         entity_index: {},
#         previous: EventLog(
#             [],
#         ),
#         current: EventLog(
#             [],
#         ),
#     },
#     xvc_digests_store: XvcStore {
#         map: {
#             XvcEntity(
#                 11,
#                 2286334430150808884,
#             ): XvcDigests(
#                 {
#                     "stdout-digest": XvcDigest {
#                         algorithm: Blake3,
#                         digest: [
#                             0,
#                             25,
#                             102,
#                             177,
#                             28,
#                             165,
#                             102,
#                             239,
#                             199,
#                             24,
#                             108,
#                             167,
#                             117,
#                             135,
#                             7,
#                             186,
#                             162,
#                             135,
#                             192,
#                             222,
#                             59,
#                             86,
#                             22,
#                             53,
#                             101,
#                             177,
#                             233,
#                             127,
#                             6,
#                             117,
#                             31,
#                             37,
#                         ],
#                     },
#                 },
#             ),
#         },
#         entity_index: {
#             XvcDigests(
#                 {
#                     "stdout-digest": XvcDigest {
#                         algorithm: Blake3,
#                         digest: [
#                             0,
#                             25,
#                             102,
#                             177,
#                             28,
#                             165,
#                             102,
#                             239,
#                             199,
#                             24,
#                             108,
#                             167,
#                             117,
#                             135,
#                             7,
#                             186,
#                             162,
#                             135,
#                             192,
#                             222,
#                             59,
#                             86,
#                             22,
#                             53,
#                             101,
#                             177,
#                             233,
#                             127,
#                             6,
#                             117,
#                             31,
#                             37,
#                         ],
#                     },
#                 },
#             ): [
#                 XvcEntity(
#                     11,
#                     2286334430150808884,
#                 ),
#             ],
#         },
#         previous: EventLog(
#             [
#                 Add {
#                     entity: XvcEntity(
#                         11,
#                         2286334430150808884,
#                     ),
#                     value: XvcDigests(
#                         {
#                             "stdout-digest": XvcDigest {
#                                 algorithm: Blake3,
#                                 digest: [
#                                     0,
#                                     25,
#                                     102,
#                                     177,
#                                     28,
#                                     165,
#                                     102,
#                                     239,
#                                     199,
#                                     24,
#                                     108,
#                                     167,
#                                     117,
#                                     135,
#                                     7,
#                                     186,
#                                     162,
#                                     135,
#                                     192,
#                                     222,
#                                     59,
#                                     86,
#                                     22,
#                                     53,
#                                     101,
#                                     177,
#                                     233,
#                                     127,
#                                     6,
#                                     117,
#                                     31,
#                                     37,
#                                 ],
#                             },
#                         },
#                     ),
#                 },
#             ],
#         ),
#         current: EventLog(
#             [],
#         ),
#     },
#     text_files: XvcStore {
#         map: {},
#         entity_index: {},
#         previous: EventLog(
#             [],
#         ),
#         current: EventLog(
#             [],
#         ),
#     },
# }
# [TRACE][pipeline/src/pipeline/deps/compare.rs::187] collected_diffs: Diffs {
#     xvc_dependency_diff: RwLock {
#         data: HStore {
#             map: {},
#         },
#         poisoned: false,
#         ..
#     },
#     xvc_digests_diff: RwLock {
#         data: HStore {
#             map: {},
#         },
#         poisoned: false,
#         ..
#     },
#     xvc_metadata_diff: RwLock {
#         data: HStore {
#             map: {},
#         },
#         poisoned: false,
#         ..
#     },
#     xvc_path_diff: RwLock {
#         data: HStore {
#             map: {},
#         },
#         poisoned: false,
#         ..
#     },
# }
# thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: IoError { source: Os { code: 21, kind: IsADirectory, message: "Is a directory" } }', pipeline/src/pipeline/deps/compare.rs:580:26
# stack backtrace:
#    0: _rust_begin_unwind
#    1: core::panicking::panic_fmt
#    2: core::result::unwrap_failed
#    3: core::result::Result<T,E>::unwrap
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/result.rs:1113:23
#    4: xvc_pipeline::pipeline::deps::compare::compare_deps_multiple_paths::{{closure}}::{{closure}}
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:575:53
#    5: <core::slice::iter::Iter<T> as core::iter::traits::iterator::Iterator>::for_each
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/slice/iter/macros.rs:211:21
#    6: xvc_pipeline::pipeline::deps::compare::compare_deps_multiple_paths::{{closure}}
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:564:13
#    7: core::result::Result<T,E>::and_then
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/result.rs:1372:22
#    8: xvc_pipeline::pipeline::deps::compare::compare_deps_multiple_paths
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:560:5
#    9: xvc_pipeline::pipeline::deps::compare::compare_deps_directory
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:447:9
#   10: xvc_pipeline::pipeline::deps::compare::compare_deps
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:205:13
#   11: xvc_pipeline::pipeline::s_checking_dependency_content_digest
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/mod.rs:644:9
#   12: xvc_pipeline::pipeline::the_grand_pipeline_loop
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/mod.rs:451:21
#   13: xvc_pipeline::pipeline::api::run::cmd_run
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/api/run.rs:18:5
#   14: xvc_pipeline::cmd_pipeline
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/lib.rs:405:13
#   15: xvc::cli::dispatch::{{closure}}::{{closure}}
#              at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:349:24
#   16: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
#              at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:438:31
#   17: core::ops::function::FnOnce::call_once{{vtable.shim}}
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/ops/function.rs:507:5
#   18: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/alloc/src/boxed.rs:2000:9
# note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
# thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', lib/src/cli/mod.rs:403:37
# stack backtrace:
#    0: _rust_begin_unwind
#    1: core::panicking::panic_fmt
#    2: core::result::unwrap_failed
#    3: core::result::Result<T,E>::unwrap
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/result.rs:1113:23
#    4: xvc::cli::dispatch::{{closure}}
#              at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:403:15
#    5: crossbeam_utils::thread::scope::{{closure}}
#              at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:65
#    6: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/panic/unwind_safe.rs:271:9
#    7: std::panicking::try::do_call
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panicking.rs:483:40
#    8: ___rust_try
#    9: std::panicking::try
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panicking.rs:447:19
#   10: std::panic::catch_unwind
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panic.rs:137:14
#   11: crossbeam_utils::thread::scope
#              at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:18
#   12: xvc::cli::dispatch
#              at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:239:5
#   13: xvc::main
#              at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:12:5
#   14: core::ops::function::FnOnce::call_once
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/ops/function.rs:507:5
# note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
#
# ```
#
# If you run the pipeline again, it won't run the step because the directory hasn't changed.
#
# ```console
# $ xvc pipeline --name directory-example run
# thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: IoError { source: Os { code: 21, kind: IsADirectory, message: "Is a directory" } }', pipeline/src/pipeline/deps/compare.rs:580:26
# stack backtrace:
#    0: _rust_begin_unwind
#    1: core::panicking::panic_fmt
#    2: core::result::unwrap_failed
#    3: core::result::Result<T,E>::unwrap
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/result.rs:1113:23
#    4: xvc_pipeline::pipeline::deps::compare::compare_deps_multiple_paths::{{closure}}::{{closure}}
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:575:53
#    5: <core::slice::iter::Iter<T> as core::iter::traits::iterator::Iterator>::for_each
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/slice/iter/macros.rs:211:21
#    6: xvc_pipeline::pipeline::deps::compare::compare_deps_multiple_paths::{{closure}}
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:564:13
#    7: core::result::Result<T,E>::and_then
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/result.rs:1372:22
#    8: xvc_pipeline::pipeline::deps::compare::compare_deps_multiple_paths
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:560:5
#    9: xvc_pipeline::pipeline::deps::compare::compare_deps_directory
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:447:9
#   10: xvc_pipeline::pipeline::deps::compare::compare_deps
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:205:13
#   11: xvc_pipeline::pipeline::s_checking_dependency_content_digest
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/mod.rs:644:9
#   12: xvc_pipeline::pipeline::the_grand_pipeline_loop
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/mod.rs:451:21
#   13: xvc_pipeline::pipeline::api::run::cmd_run
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/api/run.rs:18:5
#   14: xvc_pipeline::cmd_pipeline
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/lib.rs:405:13
#   15: xvc::cli::dispatch::{{closure}}::{{closure}}
#              at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:349:24
#   16: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
#              at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:438:31
#   17: core::ops::function::FnOnce::call_once{{vtable.shim}}
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/ops/function.rs:507:5
#   18: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/alloc/src/boxed.rs:2000:9
# note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
# thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', lib/src/cli/mod.rs:403:37
# stack backtrace:
#    0: _rust_begin_unwind
#    1: core::panicking::panic_fmt
#    2: core::result::unwrap_failed
#    3: core::result::Result<T,E>::unwrap
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/result.rs:1113:23
#    4: xvc::cli::dispatch::{{closure}}
#              at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:403:15
#    5: crossbeam_utils::thread::scope::{{closure}}
#              at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:65
#    6: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/panic/unwind_safe.rs:271:9
#    7: std::panicking::try::do_call
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panicking.rs:483:40
#    8: ___rust_try
#    9: std::panicking::try
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panicking.rs:447:19
#   10: std::panic::catch_unwind
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panic.rs:137:14
#   11: crossbeam_utils::thread::scope
#              at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:18
#   12: xvc::cli::dispatch
#              at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:239:5
#   13: xvc::main
#              at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:12:5
#   14: core::ops::function::FnOnce::call_once
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/ops/function.rs:507:5
# note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
#
# ```
#
# If you add, delete or modify a file in the directory, the step will be invalidated and run again.
#
# ```console
# $ touch dir-0001/another-file.txt
# $ xvc pipeline --name directory-example run
# thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: IoError { source: Os { code: 21, kind: IsADirectory, message: "Is a directory" } }', pipeline/src/pipeline/deps/compare.rs:580:26
# stack backtrace:
#    0: _rust_begin_unwind
#    1: core::panicking::panic_fmt
#    2: core::result::unwrap_failed
#    3: core::result::Result<T,E>::unwrap
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/result.rs:1113:23
#    4: xvc_pipeline::pipeline::deps::compare::compare_deps_multiple_paths::{{closure}}::{{closure}}
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:575:53
#    5: <core::slice::iter::Iter<T> as core::iter::traits::iterator::Iterator>::for_each
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/slice/iter/macros.rs:211:21
#    6: xvc_pipeline::pipeline::deps::compare::compare_deps_multiple_paths::{{closure}}
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:564:13
#    7: core::result::Result<T,E>::and_then
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/result.rs:1372:22
#    8: xvc_pipeline::pipeline::deps::compare::compare_deps_multiple_paths
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:560:5
#    9: xvc_pipeline::pipeline::deps::compare::compare_deps_directory
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:447:9
#   10: xvc_pipeline::pipeline::deps::compare::compare_deps
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/deps/compare.rs:205:13
#   11: xvc_pipeline::pipeline::s_checking_dependency_content_digest
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/mod.rs:644:9
#   12: xvc_pipeline::pipeline::the_grand_pipeline_loop
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/mod.rs:451:21
#   13: xvc_pipeline::pipeline::api::run::cmd_run
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/api/run.rs:18:5
#   14: xvc_pipeline::cmd_pipeline
#              at /Users/iex/github.com/iesahin/xvc/pipeline/src/lib.rs:405:13
#   15: xvc::cli::dispatch::{{closure}}::{{closure}}
#              at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:349:24
#   16: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
#              at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:438:31
#   17: core::ops::function::FnOnce::call_once{{vtable.shim}}
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/ops/function.rs:507:5
#   18: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/alloc/src/boxed.rs:2000:9
# note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
# thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', lib/src/cli/mod.rs:403:37
# stack backtrace:
#    0: _rust_begin_unwind
#    1: core::panicking::panic_fmt
#    2: core::result::unwrap_failed
#    3: core::result::Result<T,E>::unwrap
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/result.rs:1113:23
#    4: xvc::cli::dispatch::{{closure}}
#              at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:403:15
#    5: crossbeam_utils::thread::scope::{{closure}}
#              at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:65
#    6: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/panic/unwind_safe.rs:271:9
#    7: std::panicking::try::do_call
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panicking.rs:483:40
#    8: ___rust_try
#    9: std::panicking::try
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panicking.rs:447:19
#   10: std::panic::catch_unwind
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panic.rs:137:14
#   11: crossbeam_utils::thread::scope
#              at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:18
#   12: xvc::cli::dispatch
#              at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:239:5
#   13: xvc::main
#              at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:12:5
#   14: core::ops::function::FnOnce::call_once
#              at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/ops/function.rs:507:5
# note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
#
# ```
#
#
#
#
# ## Caveats
#
# ## Tips
#
# Most shells support editing longer commands with an editor. For bash, you can use `Ctrl+X Ctrl+E`.
#
# Pipeline commands can get longer quickly. You can use [xvc aliases](/ref/xvc-aliases.md) for shorter
# versions. Type `source $(xvc aliases)` to load the aliases into your shell.
