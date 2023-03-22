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
[OUT] data.txt has changed

[OUT] [EXIT] Successfully

```

You can add multiple dependencies to a step with multiple invocations.

```console
$ xvc pipeline step dependency --step-name file-dependency --file data2.txt
```

A step will run if any of its dependencies have changed.

```console
$ xvc pipeline run
[OUT] data.txt has changed

[OUT] [EXIT] Successfully

```

Normally, they are not run if none of the dependencies have changed.

```console
$ xvc pipeline run
[OUT] data.txt has changed

[OUT] [EXIT] Successfully

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
[OUT] world

[OUT] [EXIT] Successfully

```

### Generic Command Dependencies

You can use the output of a command as a dependency to a step. When the command is run, the output hash is saved to
compare and to invalidate the step when the output has changed.

You can use this for any command that outputs a string.

```console
$ xvc -vvvv pipeline new --name generic
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
                "git.auto_commit": Boolean(
                    true,
                ),
                "git.command": String(
                    "git",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "core.guid": String(
                    "24f5156df5a09f2a",
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "core.guid": String(
                    "98bd54681c37c79e",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "git.command": String(
                    "git",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
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
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "98bd54681c37c79e",
            ),
        },
        "git.auto_commit": XvcConfigValue {
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
        "file.carry-in.no_parallel": XvcConfigValue {
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
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
        "file.track.no_parallel": XvcConfigValue {
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
        "git.auto_stage": XvcConfigValue {
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
        "file.track.no_commit": XvcConfigValue {
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
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
            ),
        },
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
            ),
        },
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"24f5156df5a09f2a/"
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
    "[CWD]/.xvc/ec/1679513009416068",
    "[CWD]/.xvc/ec/1679513009419294",
    "[CWD]/.xvc/ec/1679513009553734",
    "[CWD]/.xvc/ec/1679513010169833",
    "[CWD]/.xvc/ec/1679513011519021",
    "[CWD]/.xvc/ec/1679513013379322",
    "[CWD]/.xvc/ec/1679513013479969",
    "[CWD]/.xvc/ec/1679513013611130",
    "[CWD]/.xvc/ec/1679513013695200",
]
[TRACE][pipeline/src/lib.rs::309] name: Some(
    "default",
)
[TRACE][lib/src/cli/mod.rs::381] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::384] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline new --name generic"
[TRACE][lib/src/cli/mod.rs::433] args: [
    "-C",
    "[CWD]",
    "diff",
    "--name-only",
    "--cached",
]
[DEBUG] Using Git: /opt/homebrew/bin/git
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
[TRACE][lib/src/cli/mod.rs::582] git_add_output: "add '.xvc/ec/1679513014443765'
add '.xvc/store/xvc-pipeline-store/1679513014443445.json'
"
[TRACE][lib/src/cli/mod.rs::433] args: [
    "-C",
    "[CWD]",
    "commit",
    "-m",
    "Xvc auto-commit after /'/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline new --name generic/'",
]
[DEBUG] Committing .xvc/ to git: [main c45277d] Xvc auto-commit after '/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline new --name generic'
 2 files changed, 2 insertions(+)
 create mode 100644 .xvc/ec/1679513014443765
 create mode 100644 .xvc/store/xvc-pipeline-store/1679513014443445.json

[DEBUG] Command completed successfully.

$ xvc -vvvv pipeline --name generic step new --step-name yearly --command "echo 'Happy new year! Welcome `(date +%Y)`!'"
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
                "core.guid": String(
                    "710fa196bbd077dc",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "git.auto_stage": Boolean(
                    false,
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
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "git.use_git": Boolean(
                    true,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "git.command": String(
                    "git",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "core.guid": String(
                    "98bd54681c37c79e",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
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
                    "debug",
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
                "debug",
            ),
        },
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
            ),
        },
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
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
        "file.carry-in.no_parallel": XvcConfigValue {
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
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
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
        "file.list.recursive": XvcConfigValue {
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
        "git.auto_stage": XvcConfigValue {
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
        "git.auto_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "98bd54681c37c79e",
            ),
        },
        "file.track.text_or_binary": XvcConfigValue {
            source: Project,
            value: String(
                "auto",
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
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"710fa196bbd077dc/"
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
    "[CWD]/.xvc/ec/1679513009416068",
    "[CWD]/.xvc/ec/1679513009419294",
    "[CWD]/.xvc/ec/1679513009553734",
    "[CWD]/.xvc/ec/1679513010169833",
    "[CWD]/.xvc/ec/1679513011519021",
    "[CWD]/.xvc/ec/1679513013379322",
    "[CWD]/.xvc/ec/1679513013479969",
    "[CWD]/.xvc/ec/1679513013611130",
    "[CWD]/.xvc/ec/1679513013695200",
    "[CWD]/.xvc/ec/1679513014443765",
]
[TRACE][pipeline/src/lib.rs::309] name: Some(
    "generic",
)
[TRACE][lib/src/cli/mod.rs::381] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::384] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline --name generic step new --step-name yearly --command echo 'Happy new year! Welcome `(date +%Y)`!'"
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
[DEBUG] Using Git: /opt/homebrew/bin/git
[TRACE][lib/src/cli/mod.rs::582] git_add_output: "add '.xvc/ec/1679513014531375'
add '.xvc/store/xvc-step-command-store/1679513014530736.json'
add '.xvc/store/xvc-step-invalidate-store/1679513014530554.json'
add '.xvc/store/xvc-step-store/1679513014530361.json'
add '.xvc/store/xvc-step-store/1679513014531080.json'
add '.xvc/store/xvc-step-xvc-pipeline-r1n-store/1679513014531156.json'
"
[TRACE][lib/src/cli/mod.rs::433] args: [
    "-C",
    "[CWD]",
    "commit",
    "-m",
    "Xvc auto-commit after /'/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline --name generic step new --step-name yearly --command echo /'Happy new year! Welcome `(date +%Y)`!/'/'",
]
[DEBUG] Committing .xvc/ to git: [main 29b7ff2] Xvc auto-commit after '/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline --name generic step new --step-name yearly --command echo 'Happy new year! Welcome `(date +%Y)`!''
 6 files changed, 6 insertions(+)
 create mode 100644 .xvc/ec/1679513014531375
 create mode 100644 .xvc/store/xvc-step-command-store/1679513014530736.json
 create mode 100644 .xvc/store/xvc-step-invalidate-store/1679513014530554.json
 create mode 100644 .xvc/store/xvc-step-store/1679513014530361.json
 create mode 100644 .xvc/store/xvc-step-store/1679513014531080.json
 create mode 100644 .xvc/store/xvc-step-xvc-pipeline-r1n-store/1679513014531156.json

[DEBUG] Command completed successfully.

$ xvc -vvvv pipeline --name generic step dependency --step-name yearly --generic 'date +%Y'
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
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "git.command": String(
                    "git",
                ),
                "core.guid": String(
                    "a661e13728ab7914",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "git.command": String(
                    "git",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "core.guid": String(
                    "98bd54681c37c79e",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "pipeline.default": String(
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
                    "debug",
                ),
                "core.quiet": Boolean(
                    false,
                ),
            },
        },
    ],
    the_config: {
        "git.auto_stage": XvcConfigValue {
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
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "98bd54681c37c79e",
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
        "file.list.no_summary": XvcConfigValue {
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
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
            ),
        },
        "file.track.text_or_binary": XvcConfigValue {
            source: Project,
            value: String(
                "auto",
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
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
            ),
        },
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
            ),
        },
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
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
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
            ),
        },
        "file.carry-in.force": XvcConfigValue {
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
guid = /"a661e13728ab7914/"
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
    "[CWD]/.xvc/ec/1679513009416068",
    "[CWD]/.xvc/ec/1679513009419294",
    "[CWD]/.xvc/ec/1679513009553734",
    "[CWD]/.xvc/ec/1679513010169833",
    "[CWD]/.xvc/ec/1679513011519021",
    "[CWD]/.xvc/ec/1679513013379322",
    "[CWD]/.xvc/ec/1679513013479969",
    "[CWD]/.xvc/ec/1679513013611130",
    "[CWD]/.xvc/ec/1679513013695200",
    "[CWD]/.xvc/ec/1679513014443765",
    "[CWD]/.xvc/ec/1679513014531375",
]
[TRACE][pipeline/src/lib.rs::309] name: Some(
    "generic",
)
[DEBUG] Adding Generic { generic_command: "date +%Y" }
[TRACE][lib/src/cli/mod.rs::381] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::384] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline --name generic step dependency --step-name yearly --generic date +%Y"
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
[TRACE][lib/src/cli/mod.rs::582] git_add_output: "add '.xvc/ec/1679513014612263'
add '.xvc/store/xvc-dependency-store/1679513014611832.json'
add '.xvc/store/xvc-dependency-xvc-step-r1n-store/1679513014612024.json'
"
[TRACE][lib/src/cli/mod.rs::433] args: [
    "-C",
    "[CWD]",
    "commit",
    "-m",
    "Xvc auto-commit after /'/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline --name generic step dependency --step-name yearly --generic date +%Y/'",
]
[DEBUG] Committing .xvc/ to git: [main 7722487] Xvc auto-commit after '/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline --name generic step dependency --step-name yearly --generic date +%Y'
 3 files changed, 3 insertions(+)
 create mode 100644 .xvc/ec/1679513014612263
 create mode 100644 .xvc/store/xvc-dependency-store/1679513014611832.json
 create mode 100644 .xvc/store/xvc-dependency-xvc-step-r1n-store/1679513014612024.json

[DEBUG] Command completed successfully.

```

When the year changes, the step is invalidated and run again.

```console
$ xvc -vvvv pipeline --name generic run
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
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "core.guid": String(
                    "504080291385a0ff",
                ),
                "git.command": String(
                    "git",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "git.auto_commit": Boolean(
                    true,
                ),
                "git.command": String(
                    "git",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "pipeline.default": String(
                    "default",
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
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "core.guid": String(
                    "98bd54681c37c79e",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
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
        "file.list.recursive": XvcConfigValue {
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
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
            ),
        },
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
        "file.track.no_commit": XvcConfigValue {
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
        "file.recheck.method": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "98bd54681c37c79e",
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
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
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
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"504080291385a0ff/"
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
    "[CWD]/.xvc/ec/1679513009416068",
    "[CWD]/.xvc/ec/1679513009419294",
    "[CWD]/.xvc/ec/1679513009553734",
    "[CWD]/.xvc/ec/1679513010169833",
    "[CWD]/.xvc/ec/1679513011519021",
    "[CWD]/.xvc/ec/1679513013379322",
    "[CWD]/.xvc/ec/1679513013479969",
    "[CWD]/.xvc/ec/1679513013611130",
    "[CWD]/.xvc/ec/1679513013695200",
    "[CWD]/.xvc/ec/1679513014443765",
    "[CWD]/.xvc/ec/1679513014531375",
    "[CWD]/.xvc/ec/1679513014612263",
]
[TRACE][pipeline/src/lib.rs::309] name: Some(
    "generic",
)
[TRACE][pipeline/src/lib.rs::404] name: None
[TRACE][pipeline/src/pipeline/api/run.rs::23] pipeline_name: "default"
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
    paths: 0x0000000136e045b0,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000000136e04520,
    runloop: Some(
        (
            0x0000000116e04b50,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::263] pipeline_len: 3
[INFO] Found explicit dependency: XvcStep { name: "hello" } -> Step { name: "file-dependency" }
[INFO] Found explicit dependency: XvcStep { name: "world" } -> Step { name: "hello" }
[TRACE][pipeline/src/pipeline/mod.rs::289] &dependency_graph: {
    XvcEntity(
        6,
        13442750732318480994,
    ): [
        (
            XvcEntity(
                2,
                2069264120390389462,
            ),
            Outgoing,
        ),
        (
            XvcEntity(
                5,
                8799652807479790633,
            ),
            Incoming,
        ),
    ],
    XvcEntity(
        5,
        8799652807479790633,
    ): [
        (
            XvcEntity(
                6,
                13442750732318480994,
            ),
            Outgoing,
        ),
    ],
    XvcEntity(
        2,
        2069264120390389462,
    ): [
        (
            XvcEntity(
                6,
                13442750732318480994,
            ),
            Incoming,
        ),
    ],
}
[INFO][pipeline/src/pipeline/mod.rs::303] Pipeline Graph:
digraph {
    0 [ label = "(6, 13442750732318480994)" ]
    1 [ label = "(5, 8799652807479790633)" ]
    2 [ label = "(2, 2069264120390389462)" ]
    0 -> 2 [ label = "Step" ]
    1 -> 0 [ label = "Step" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::354] dependency_graph.edges_directed(*step_e,
            Direction::Incoming).map(|e| e.0).collect::<Vec<XvcEntity>>(): [
    XvcEntity(
        5,
        8799652807479790633,
    ),
]
[TRACE][pipeline/src/pipeline/mod.rs::359] dependency_graph.edges_directed(*step_e,
            Direction::Outgoing).map(|e| e.0).collect::<Vec<XvcEntity>>(): [
    XvcEntity(
        6,
        13442750732318480994,
    ),
]
[TRACE][pipeline/src/pipeline/mod.rs::354] dependency_graph.edges_directed(*step_e,
            Direction::Incoming).map(|e| e.0).collect::<Vec<XvcEntity>>(): []
[TRACE][pipeline/src/pipeline/mod.rs::359] dependency_graph.edges_directed(*step_e,
            Direction::Outgoing).map(|e| e.0).collect::<Vec<XvcEntity>>(): [
    XvcEntity(
        5,
        8799652807479790633,
    ),
]
[TRACE][pipeline/src/pipeline/mod.rs::354] dependency_graph.edges_directed(*step_e,
            Direction::Incoming).map(|e| e.0).collect::<Vec<XvcEntity>>(): [
    XvcEntity(
        6,
        13442750732318480994,
    ),
]
[TRACE][pipeline/src/pipeline/mod.rs::359] dependency_graph.edges_directed(*step_e,
            Direction::Outgoing).map(|e| e.0).collect::<Vec<XvcEntity>>(): []
[TRACE][pipeline/src/pipeline/mod.rs::387] step_states: HStore {
    map: {
        XvcEntity(
            5,
            8799652807479790633,
        ): Begin(
            FromInit,
        ),
        XvcEntity(
            2,
            2069264120390389462,
        ): Begin(
            FromInit,
        ),
        XvcEntity(
            6,
            13442750732318480994,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    Begin(
        FromInit,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Begin(
        FromInit,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    Begin(
        FromInit,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingDependencySteps(
        FromRunConditional,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    WaitingDependencySteps(
        FromRunConditional,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    WaitingDependencySteps(
        FromRunConditional,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    CheckingMissingDependencies(
        FromDependencyStepsFinishedSuccessfully,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    CheckingMissingOutputs(
        FromMissingDependenciesIgnored,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    CheckingTimestamps(
        FromHasNoMissingOutputs,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    CheckingDependencyContentDigest(
        FromTimestampsIgnored,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::558] params.run_conditions: RunConditions {
    never: false,
    wait_running_dep_steps: true,
    ignore_broken_dep_steps: true,
    ignore_missing_dependencies: true,
    ignore_timestamp_comparison: true,
    ignore_content_digest_comparison: true,
    run_when_outputs_missing: true,
}
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    WaitingToRun(
        FromContentDigestIgnored,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Running(
        FromStartProcess,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[OUT] data.txt has changed

[OUT] [EXIT] Successfully
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Done(
        FromProcessCompletedSuccessfully,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    CheckingMissingDependencies(
        FromDependencyStepsFinishedSuccessfully,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    CheckingMissingOutputs(
        FromNoMissingDependencies,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    CheckingTimestamps(
        FromHasNoMissingOutputs,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    CheckingDependencyContentDigest(
        FromHasNoNewerDependencies,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::558] params.run_conditions: RunConditions {
    never: false,
    wait_running_dep_steps: true,
    ignore_broken_dep_steps: false,
    ignore_missing_dependencies: false,
    ignore_timestamp_comparison: false,
    ignore_content_digest_comparison: false,
    run_when_outputs_missing: true,
}
[TRACE][pipeline/src/pipeline/mod.rs::581] &collected_diffs: Diffs {
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
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    NoNeedToRun(
        FromContentDigestNotChanged,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingDependencySteps(
        FromDependencyStepsRunning,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    Done(
        FromCompletedWithoutRunningStep,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    CheckingMissingDependencies(
        FromDependencyStepsFinishedSuccessfully,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    CheckingMissingOutputs(
        FromMissingDependenciesIgnored,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    CheckingTimestamps(
        FromHasNoMissingOutputs,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    CheckingDependencyContentDigest(
        FromTimestampsIgnored,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::558] params.run_conditions: RunConditions {
    never: false,
    wait_running_dep_steps: true,
    ignore_broken_dep_steps: true,
    ignore_missing_dependencies: true,
    ignore_timestamp_comparison: true,
    ignore_content_digest_comparison: true,
    run_when_outputs_missing: true,
}
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    WaitingToRun(
        FromContentDigestIgnored,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        5,
        8799652807479790633,
    ),
    Running(
        FromStartProcess,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        2,
        2069264120390389462,
    ),
    Done(
        FromHasDone,
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::417] (step_e, step_s): (
    XvcEntity(
        6,
        13442750732318480994,
    ),
    Done(
        FromHasDone,
    ),
)
[OUT] world

[OUT] [EXIT] Successfully
[TRACE][lib/src/cli/mod.rs::381] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::384] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline --name generic run"
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
[DEBUG] Using Git: /opt/homebrew/bin/git
[TRACE][lib/src/cli/mod.rs::582] git_add_output: ""
[DEBUG] No files to commit
[DEBUG] Command completed successfully.

```

The step won't run until the next year.

```console
$ xvc pipeline --name generic run
[OUT] data.txt has changed

[OUT] [EXIT] Successfully
[OUT] world

[OUT] [EXIT] Successfully

```


## Caveats

## Tips

Most shells support editing longer commands with an editor. For bash, you can use `Ctrl+X Ctrl+E`.

Pipeline commands can get longer quickly. You can use [xvc aliases](/ref/xvc-aliases.md) for shorter
versions. Type `source $(xvc aliases)` to load the aliases into your shell.
