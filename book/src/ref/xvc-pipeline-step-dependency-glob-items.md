### Glob Items Dependency

A step can depend on multiple files specified with globs. When any of the files change, or a new file is added or
removed from the files specified by glob, the step is invalidated.

Unline glob dependency, glob items dependency keeps track of the individual files that belong to a glob. If your
command run with the list of files from a glob and you want to track added and removed files, use this. Otherwise if
your command for all the files in a glob and don't need to track which files have changed, use the glob dependency.

This one injects `${XVC_GLOB_ADDED_ITEMS}`, `${XVC_GLOB_REMOVED_ITEMS}`, `${XVC_GLOB_CHANGED_ITEMS}` and `${XVC_GLOB_ALL_ITEMS}` to the command
environment.

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Let's create a set of files:

```console
$ xvc-test-helper create-directory-tree --directories 2 --files 3 --seed 2023

$ tree
.
├── dir-0001
│   ├── file-0001.bin
│   ├── file-0002.bin
│   └── file-0003.bin
└── dir-0002
    ├── file-0001.bin
    ├── file-0002.bin
    └── file-0003.bin

3 directories, 6 files

```

Add a step to list the added files.

```console
$ xvc pipeline step new --step-name files-changed --command 'echo "### Added Files:\n${XVC_GLOB_ADDED_ITEMS}\n### Removed Files:\n${XVC_GLOB_REMOVED_ITEMS}\n### Changed Files:\n${XVC_GLOB_CHANGED_ITEMS}"'

$ xvc pipeline step dependency --step-name files-changed --glob-items 'dir-*/*'

```

The step is invalidated when a file described by the glob is added, removed or changed.

```console
$ xvc pipeline run
[DEBUG][logging/src/lib.rs::237] Terminal logger enabled with level: Error
[DEBUG][logging/src/lib.rs::240] File logger enabled with level: Trace to "/var/folders/tk/3vn311ps4kqdhgykj3jg_p8r0000gn/T//xvc.log"
[TRACE][core/src/types/xvcroot.rs::247] "."
[DEBUG][core/src/types/xvcroot.rs::253] XVC DIR: "[CWD]"
[DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][config/src/lib.rs::527] env_config: {
    "TRYCMD_TESTS": String(
        "pipeline",
    ),
    "TRYCMD_DURATION": Integer(
        300,
    ),
}
[TRACE][config/src/lib.rs::537] cli_config: [
    "core.verbosity = quiet",
    "core.quiet = false",
]
[TRACE][config/src/lib.rs::541] map: {
    "core.quiet": Boolean(
        false,
    ),
    "core.verbosity": String(
        "quiet",
    ),
}
[TRACE][config/src/lib.rs::544] conf: XvcConfig {
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
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.list.show_dot_files": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.process_pool_size": Integer(
                    4,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "core.guid": String(
                    "c1d62877762ce1f5",
                ),
                "file.track.force": Boolean(
                    false,
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "pipeline.process_pool_size": Integer(
                    4,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
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
                "file.list.show_dot_files": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "git.command": String(
                    "git",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "core.guid": String(
                    "694154b363eacd9a",
                ),
            },
        },
        XvcConfigMap {
            source: Local,
            map: {},
        },
        XvcConfigMap {
            source: Environment,
            map: {
                "TRYCMD_TESTS": String(
                    "pipeline",
                ),
                "TRYCMD_DURATION": Integer(
                    300,
                ),
            },
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
        "pipeline.default": XvcConfigValue {
            source: Project,
            value: String(
                "default",
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
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
            ),
        },
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
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
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "694154b363eacd9a",
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
        "TRYCMD_DURATION": XvcConfigValue {
            source: Environment,
            value: Integer(
                300,
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
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
        "pipeline.current_pipeline": XvcConfigValue {
            source: Project,
            value: String(
                "default",
            ),
        },
        "TRYCMD_TESTS": XvcConfigValue {
            source: Environment,
            value: String(
                "pipeline",
            ),
        },
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "quiet",
            ),
        },
        "file.track.no_parallel": XvcConfigValue {
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
        "pipeline.process_pool_size": XvcConfigValue {
            source: Project,
            value: Integer(
                4,
            ),
        },
        "file.list.recursive": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "file.list.show_dot_files": XvcConfigValue {
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
guid = /"c1d62877762ce1f5/"
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

# Show dot files like .gitignore
show_dot_files = false

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
# Number of command processes to run concurrently
process_pool_size = 4
# 

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
    "[CWD]/.xvc/ec/1703585566129843",
    "[CWD]/.xvc/ec/1703585566136774",
    "[CWD]/.xvc/ec/1703585566459409",
    "[CWD]/.xvc/ec/1703585566535369",
]
[TRACE][pipeline/src/lib.rs::104] name: Some(
    "default",
)
[TRACE][walker/src/lib.rs::594] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::595] ignore_path: "[CWD]/.xvcignore"
[TRACE][walker/src/lib.rs::603] &content: "
# Add patterns of files xvc should ignore, which could improve
# the performance.
# It's in the same format as .gitignore files.

.DS_Store
"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 1 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0001"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0001"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/ec"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/ec"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-output-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-output-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-output-xvc-step-r1n-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-output-xvc-step-r1n-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-step-xvc-pipeline-r1n-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-step-xvc-pipeline-r1n-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-step-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-step-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-step-command-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-step-command-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-dependency-xvc-step-r1n-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-dependency-xvc-step-r1n-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-pipeline-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-pipeline-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-step-invalidate-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-step-invalidate-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-dependency-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-dependency-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0002"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0002"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/0d"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/0d"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/68"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/68"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/56"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/56"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/0b"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/0b"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/9d"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/9d"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/d9"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/d9"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/bd"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/bd"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/d1"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/d1"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/ae"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/ae"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/d8"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/d8"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/c0"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/c0"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/c9"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/c9"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/cf"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/cf"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/e4"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/e4"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/ec"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/ec"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/pack"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/pack"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/7d"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/7d"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/7e"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/7e"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/4c"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/4c"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/75"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/75"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/86"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/86"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/44"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/44"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/info"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/info"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/3a"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/3a"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/bf"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/bf"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/d3"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/d3"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/e1"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/e1"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/f9"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/f9"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/f6"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/f6"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/79"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/79"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/41"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/41"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/48"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/48"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/8d"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/8d"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/15"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/15"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/12"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/12"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/76"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/76"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/8b"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/8b"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/info"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/info"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/logs"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/logs"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/logs/refs"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/logs/refs"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/logs/refs/heads"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/logs/refs/heads"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/hooks"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/hooks"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/refs"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/refs"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/refs/heads"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/refs/heads"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/refs/tags"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/refs/tags"
[TRACE][walker/src/notify.rs::170] watcher: FsEventWatcher {
    paths: 0x00006000016ac000,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000600003390010,
    runloop: Some(
        (
            0x0000600002d9c0c0,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::301] pipeline_len: 1
[TRACE][pipeline/src/pipeline/mod.rs::327] &dependency_graph: {
    XvcEntity(
        2,
        5593323671116635981,
    ): [],
}
[TRACE][pipeline/src/pipeline/mod.rs::339] &dependency_graph: {
    XvcEntity(
        2,
        5593323671116635981,
    ): [],
}
[INFO][pipeline/src/pipeline/mod.rs::343] Pipeline Graph:
digraph {
    0 [ label = "(2, 5593323671116635981)" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::408] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                5593323671116635981,
            ): Begin(
                FromInit,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::504] &step_thread_store: HStore {
    map: {
        XvcEntity(
            2,
            5593323671116635981,
        ): ScopedJoinHandle { .. },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::508] (step_e, &jh): (
    XvcEntity(
        2,
        5593323671116635981,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::619] params.recorded_dependencies: R1NStore {
    parents: XvcStore {
        map: {
            XvcEntity(
                2,
                5593323671116635981,
            ): XvcStep {
                name: "files-changed",
            },
        },
        entity_index: {
            XvcStep {
                name: "files-changed",
            }: [
                XvcEntity(
                    2,
                    5593323671116635981,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        2,
                        5593323671116635981,
                    ),
                    value: XvcStep {
                        name: "files-changed",
                    },
                },
                Add {
                    entity: XvcEntity(
                        2,
                        5593323671116635981,
                    ),
                    value: XvcStep {
                        name: "files-changed",
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
                2750374442685597278,
            ): GlobItems(
                GlobItemsDep {
                    glob: "dir-*/*",
                    xvc_path_metadata_map: {},
                    xvc_path_content_digest_map: {},
                },
            ),
        },
        entity_index: {
            GlobItems(
                GlobItemsDep {
                    glob: "dir-*/*",
                    xvc_path_metadata_map: {},
                    xvc_path_content_digest_map: {},
                },
            ): [
                XvcEntity(
                    3,
                    2750374442685597278,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        2750374442685597278,
                    ),
                    value: GlobItems(
                        GlobItemsDep {
                            glob: "dir-*/*",
                            xvc_path_metadata_map: {},
                            xvc_path_content_digest_map: {},
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
                2750374442685597278,
            ): ChildEntity(
                XvcEntity(
                    2,
                    5593323671116635981,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
        },
        entity_index: {
            ChildEntity(
                XvcEntity(
                    2,
                    5593323671116635981,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    3,
                    2750374442685597278,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        2750374442685597278,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            5593323671116635981,
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
[TRACE][pipeline/src/pipeline/mod.rs::620] step_e: XvcEntity(
    2,
    5593323671116635981,
)
[TRACE][pipeline/src/pipeline/mod.rs::560] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::621] dependency_steps(step_e, params.dependency_graph)?: {}
[TRACE][pipeline/src/pipeline/mod.rs::560] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::579] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::658] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::579] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::768] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::769] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::771] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::658] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::768] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::769] &r_next_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::771] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::658] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::768] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::769] &r_next_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::771] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::658] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::1059] parent_entity: XvcEntity(
    2,
    5593323671116635981,
)
[TRACE][pipeline/src/pipeline/mod.rs::1062] deps: HStore {
    map: {
        XvcEntity(
            3,
            2750374442685597278,
        ): GlobItems(
            GlobItemsDep {
                glob: "dir-*/*",
                xvc_path_metadata_map: {},
                xvc_path_content_digest_map: {},
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/deps/compare.rs::426] &stored: GlobItems(
    GlobItemsDep {
        glob: "dir-*/*",
        xvc_path_metadata_map: {},
        xvc_path_content_digest_map: {},
    },
)
[TRACE][pipeline/src/pipeline/mod.rs::579] select: Select { .. }
[TRACE][core/src/util/file.rs::316] full_glob: "dir-*/*"
[TRACE][pipeline/src/pipeline/mod.rs::579] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::579] select: Select { .. }
[TRACE][walker/src/lib.rs::745] is_abs: false
[TRACE][walker/src/lib.rs::749] path_str: "dir-0001/file-0001.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "dir-0001/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/lib.rs::745] is_abs: false
[TRACE][walker/src/lib.rs::749] path_str: "dir-0001/file-0002.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "dir-0001/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/lib.rs::745] is_abs: false
[TRACE][walker/src/lib.rs::749] path_str: "dir-0001/file-0003.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "dir-0001/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/lib.rs::745] is_abs: false
[TRACE][walker/src/lib.rs::749] path_str: "dir-0002/file-0001.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "dir-0002/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/lib.rs::745] is_abs: false
[TRACE][walker/src/lib.rs::749] path_str: "dir-0002/file-0002.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "dir-0002/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/lib.rs::745] is_abs: false
[TRACE][walker/src/lib.rs::749] path_str: "dir-0002/file-0003.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "dir-0002/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][pipeline/src/pipeline/mod.rs::1079] step_dependency_diffs: HStore {
    map: {
        XvcEntity(
            3,
            2750374442685597278,
        ): Identical,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1085] diff: Identical
[TRACE][pipeline/src/pipeline/mod.rs::1086] diff.changed(): false
[TRACE][pipeline/src/pipeline/mod.rs::1091] changed: false
[TRACE][pipeline/src/pipeline/mod.rs::768] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::769] &r_next_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::771] &step_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::658] &step_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::1008] params.step_dependencies: {}
[TRACE][pipeline/src/pipeline/mod.rs::1046] changed: false
[TRACE][pipeline/src/pipeline/mod.rs::768] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::769] &r_next_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::771] &step_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::658] &step_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::579] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::579] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::515] "Before state updater": "Before state updater"
[TRACE][pipeline/src/pipeline/mod.rs::589] s: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::525] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                5593323671116635981,
            ): DoneWithoutRunning(
                FromDiffsHasNotChanged,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::532] done_successfully: Ok(
    true,
)
[TRACE][lib/src/cli/mod.rs::381] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::384] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc --debug pipeline run"
[TRACE][lib/src/cli/mod.rs::436] args: [
    "-C",
    "[CWD]",
    "diff",
    "--name-only",
    "--cached",
]
[TRACE][lib/src/cli/mod.rs::466] git_diff_staged_out: ""
[TRACE][lib/src/cli/mod.rs::436] args: [
    "-C",
    "[CWD]",
    "add",
    "--verbose",
    "[CWD]/.xvc",
    "*.gitignore",
    "*.xvcignore",
]
[TRACE][lib/src/cli/mod.rs::584] git_add_output: ""

$ xvc --debug pipeline run
[DEBUG][logging/src/lib.rs::237] Terminal logger enabled with level: Error
[DEBUG][logging/src/lib.rs::240] File logger enabled with level: Trace to "/var/folders/tk/3vn311ps4kqdhgykj3jg_p8r0000gn/T//xvc.log"
[TRACE][core/src/types/xvcroot.rs::247] "."
[DEBUG][core/src/types/xvcroot.rs::253] XVC DIR: "[CWD]"
[DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][config/src/lib.rs::527] env_config: {
    "TRYCMD_DURATION": Integer(
        300,
    ),
    "TRYCMD_TESTS": String(
        "pipeline",
    ),
}
[TRACE][config/src/lib.rs::537] cli_config: [
    "core.verbosity = quiet",
    "core.quiet = false",
]
[TRACE][config/src/lib.rs::541] map: {
    "core.verbosity": String(
        "quiet",
    ),
    "core.quiet": Boolean(
        false,
    ),
}
[TRACE][config/src/lib.rs::544] conf: XvcConfig {
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
                "pipeline.default": String(
                    "default",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
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
                "pipeline.process_pool_size": Integer(
                    4,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.list.show_dot_files": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "core.guid": String(
                    "66a4281fdbb632d7",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "git.command": String(
                    "git",
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "file.recheck.method": String(
                    "copy",
                ),
                "core.guid": String(
                    "694154b363eacd9a",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "pipeline.process_pool_size": Integer(
                    4,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
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
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.list.show_dot_files": Boolean(
                    false,
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
            },
        },
        XvcConfigMap {
            source: Local,
            map: {},
        },
        XvcConfigMap {
            source: Environment,
            map: {
                "TRYCMD_DURATION": Integer(
                    300,
                ),
                "TRYCMD_TESTS": String(
                    "pipeline",
                ),
            },
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
        "file.list.show_dot_files": XvcConfigValue {
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
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "694154b363eacd9a",
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
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
            ),
        },
        "TRYCMD_DURATION": XvcConfigValue {
            source: Environment,
            value: Integer(
                300,
            ),
        },
        "TRYCMD_TESTS": XvcConfigValue {
            source: Environment,
            value: String(
                "pipeline",
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
        "pipeline.process_pool_size": XvcConfigValue {
            source: Project,
            value: Integer(
                4,
            ),
        },
        "file.track.no_parallel": XvcConfigValue {
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
        "git.auto_stage": XvcConfigValue {
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
        "file.carry-in.no_parallel": XvcConfigValue {
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
guid = /"66a4281fdbb632d7/"
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

# Show dot files like .gitignore
show_dot_files = false

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
# Number of command processes to run concurrently
process_pool_size = 4
# 

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
    "[CWD]/.xvc/ec/1703585566129843",
    "[CWD]/.xvc/ec/1703585566136774",
    "[CWD]/.xvc/ec/1703585566459409",
    "[CWD]/.xvc/ec/1703585566535369",
]
[TRACE][pipeline/src/lib.rs::104] name: Some(
    "default",
)
[TRACE][walker/src/lib.rs::594] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::595] ignore_path: "[CWD]/.xvcignore"
[TRACE][walker/src/lib.rs::603] &content: "
# Add patterns of files xvc should ignore, which could improve
# the performance.
# It's in the same format as .gitignore files.

.DS_Store
"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 1 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0001"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0001"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/ec"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/ec"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-output-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-output-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-output-xvc-step-r1n-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-output-xvc-step-r1n-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-step-xvc-pipeline-r1n-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-step-xvc-pipeline-r1n-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-step-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-step-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-step-command-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-step-command-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-dependency-xvc-step-r1n-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-dependency-xvc-step-r1n-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-pipeline-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-pipeline-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-step-invalidate-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-step-invalidate-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-pipeline-run-dir-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-pipeline-run-dir-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc/store/xvc-dependency-store"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc/store/xvc-dependency-store"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0002"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0002"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/0d"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/0d"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/68"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/68"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/56"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/56"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/0b"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/0b"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/9d"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/9d"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/d9"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/d9"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/bd"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/bd"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/d1"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/d1"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/ae"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/ae"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/d8"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/d8"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/c0"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/c0"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/c9"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/c9"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/cf"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/cf"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/e4"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/e4"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/ec"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/ec"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/pack"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/pack"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/7d"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/7d"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/7e"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/7e"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/4c"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/4c"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/75"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/75"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/86"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/86"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/44"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/44"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/info"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/info"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/3a"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/3a"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/bf"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/bf"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/d3"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/d3"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/e1"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/e1"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/f9"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/f9"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/f6"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/f6"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/79"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/79"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/41"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/41"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/48"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/48"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/8d"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/8d"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/15"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/15"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/12"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/12"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/76"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/76"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/objects/8b"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/objects/8b"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/info"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/info"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/logs"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/logs"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/logs/refs"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/logs/refs"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/logs/refs/heads"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/logs/refs/heads"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/hooks"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/hooks"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/refs"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/refs"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/refs/heads"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/refs/heads"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git/refs/tags"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git/refs/tags"
[TRACE][walker/src/notify.rs::170] watcher: FsEventWatcher {
    paths: 0x0000600002b78000,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000600000e78010,
    runloop: Some(
        (
            0x00006000010700c0,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::301] pipeline_len: 1
[TRACE][pipeline/src/pipeline/mod.rs::327] &dependency_graph: {
    XvcEntity(
        2,
        5593323671116635981,
    ): [],
}
[TRACE][pipeline/src/pipeline/mod.rs::339] &dependency_graph: {
    XvcEntity(
        2,
        5593323671116635981,
    ): [],
}
[INFO][pipeline/src/pipeline/mod.rs::343] Pipeline Graph:
digraph {
    0 [ label = "(2, 5593323671116635981)" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::408] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                5593323671116635981,
            ): Begin(
                FromInit,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::504] &step_thread_store: HStore {
    map: {
        XvcEntity(
            2,
            5593323671116635981,
        ): ScopedJoinHandle { .. },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::508] (step_e, &jh): (
    XvcEntity(
        2,
        5593323671116635981,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::619] params.recorded_dependencies: R1NStore {
    parents: XvcStore {
        map: {
            XvcEntity(
                2,
                5593323671116635981,
            ): XvcStep {
                name: "files-changed",
            },
        },
        entity_index: {
            XvcStep {
                name: "files-changed",
            }: [
                XvcEntity(
                    2,
                    5593323671116635981,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        2,
                        5593323671116635981,
                    ),
                    value: XvcStep {
                        name: "files-changed",
                    },
                },
                Add {
                    entity: XvcEntity(
                        2,
                        5593323671116635981,
                    ),
                    value: XvcStep {
                        name: "files-changed",
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
                2750374442685597278,
            ): GlobItems(
                GlobItemsDep {
                    glob: "dir-*/*",
                    xvc_path_metadata_map: {},
                    xvc_path_content_digest_map: {},
                },
            ),
        },
        entity_index: {
            GlobItems(
                GlobItemsDep {
                    glob: "dir-*/*",
                    xvc_path_metadata_map: {},
                    xvc_path_content_digest_map: {},
                },
            ): [
                XvcEntity(
                    3,
                    2750374442685597278,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        2750374442685597278,
                    ),
                    value: GlobItems(
                        GlobItemsDep {
                            glob: "dir-*/*",
                            xvc_path_metadata_map: {},
                            xvc_path_content_digest_map: {},
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
                2750374442685597278,
            ): ChildEntity(
                XvcEntity(
                    2,
                    5593323671116635981,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
        },
        entity_index: {
            ChildEntity(
                XvcEntity(
                    2,
                    5593323671116635981,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    3,
                    2750374442685597278,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        2750374442685597278,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            5593323671116635981,
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
[TRACE][pipeline/src/pipeline/mod.rs::620] step_e: XvcEntity(
    2,
    5593323671116635981,
)
[TRACE][pipeline/src/pipeline/mod.rs::560] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::621] dependency_steps(step_e, params.dependency_graph)?: {}
[TRACE][pipeline/src/pipeline/mod.rs::560] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::579] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::658] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::768] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::769] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::771] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::658] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::768] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::769] &r_next_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::771] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::658] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::768] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::769] &r_next_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::771] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::658] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::1059] parent_entity: XvcEntity(
    2,
    5593323671116635981,
)
[TRACE][pipeline/src/pipeline/mod.rs::1062] deps: HStore {
    map: {
        XvcEntity(
            3,
            2750374442685597278,
        ): GlobItems(
            GlobItemsDep {
                glob: "dir-*/*",
                xvc_path_metadata_map: {},
                xvc_path_content_digest_map: {},
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/deps/compare.rs::426] &stored: GlobItems(
    GlobItemsDep {
        glob: "dir-*/*",
        xvc_path_metadata_map: {},
        xvc_path_content_digest_map: {},
    },
)
[TRACE][core/src/util/file.rs::316] full_glob: "dir-*/*"
[TRACE][walker/src/lib.rs::745] is_abs: false
[TRACE][walker/src/lib.rs::749] path_str: "dir-0001/file-0001.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "dir-0001/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/lib.rs::745] is_abs: false
[TRACE][walker/src/lib.rs::749] path_str: "dir-0001/file-0002.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "dir-0001/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/lib.rs::745] is_abs: false
[TRACE][walker/src/lib.rs::749] path_str: "dir-0001/file-0003.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "dir-0001/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/lib.rs::745] is_abs: false
[TRACE][walker/src/lib.rs::749] path_str: "dir-0002/file-0001.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "dir-0002/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/lib.rs::745] is_abs: false
[TRACE][walker/src/lib.rs::749] path_str: "dir-0002/file-0002.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "dir-0002/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/lib.rs::745] is_abs: false
[TRACE][walker/src/lib.rs::749] path_str: "dir-0002/file-0003.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "dir-0002/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][pipeline/src/pipeline/mod.rs::1079] step_dependency_diffs: HStore {
    map: {
        XvcEntity(
            3,
            2750374442685597278,
        ): Identical,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1085] diff: Identical
[TRACE][pipeline/src/pipeline/mod.rs::1086] diff.changed(): false
[TRACE][pipeline/src/pipeline/mod.rs::579] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::579] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::579] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::579] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1091] changed: false
[TRACE][pipeline/src/pipeline/mod.rs::768] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::769] &r_next_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::771] &step_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::658] &step_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::1008] params.step_dependencies: {}
[TRACE][pipeline/src/pipeline/mod.rs::1046] changed: false
[TRACE][pipeline/src/pipeline/mod.rs::768] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::769] &r_next_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::771] &step_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::658] &step_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::579] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::579] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::515] "Before state updater": "Before state updater"
[TRACE][pipeline/src/pipeline/mod.rs::589] s: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::525] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                5593323671116635981,
            ): DoneWithoutRunning(
                FromDiffsHasNotChanged,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::532] done_successfully: Ok(
    true,
)
[TRACE][lib/src/cli/mod.rs::381] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::384] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc --debug pipeline run"
[TRACE][lib/src/cli/mod.rs::436] args: [
    "-C",
    "[CWD]",
    "diff",
    "--name-only",
    "--cached",
]
[TRACE][lib/src/cli/mod.rs::466] git_diff_staged_out: ""
[TRACE][lib/src/cli/mod.rs::436] args: [
    "-C",
    "[CWD]",
    "add",
    "--verbose",
    "[CWD]/.xvc",
    "*.gitignore",
    "*.xvcignore",
]
[TRACE][lib/src/cli/mod.rs::584] git_add_output: ""

```

If you add or remove a file from the files specified by the glob, they are printed.

```console
$ rm dir-0001/file-0001.bin

$ xvc pipeline run
[OUT] [files-changed] ### Added Files:

### Removed Files:
dir-0001/file-0001.bin
### Changed Files:

 
[DONE] files-changed (echo "### Added Files:/n${XVC_GLOB_ADDED_ITEMS}/n### Removed Files:/n${XVC_GLOB_REMOVED_ITEMS}/n### Changed Files:/n${XVC_GLOB_CHANGED_ITEMS}")

```

When you change a file, it's printed in both added and removed files:

```console
$ xvc-test-helper generate-filled-file dir-0001/file-0002.bin

$ xvc pipeline run
[OUT] [files-changed] ### Added Files:

### Removed Files:

### Changed Files:
dir-0001/file-0002.bin
 
[DONE] files-changed (echo "### Added Files:/n${XVC_GLOB_ADDED_ITEMS}/n### Removed Files:/n${XVC_GLOB_REMOVED_ITEMS}/n### Changed Files:/n${XVC_GLOB_CHANGED_ITEMS}")

```
