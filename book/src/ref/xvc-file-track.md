# xvc file track

## Purpose

`xvc file track` is used to register any kind of file to Xvc for tracking versions.

## Synopsis 

```console
$ xvc file track --help
Add file and directories to Xvc

Usage: xvc file track [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to track

Options:
      --recheck-method <RECHECK_METHOD>
          How to track the file contents in cache: One of copy, symlink, hardlink, reflink.
          
          Note: Reflink uses copy if the underlying file system doesn't support it.

      --no-commit
          Do not copy/link added files to the file cache

      --text-or-binary <TEXT_OR_BINARY>
          Calculate digests as text or binary file without checking contents, or by automatically. (Default: auto)

      --force
          Add targets even if they are already tracked

      --no-parallel
          Don't use parallelism

  -h, --help
          Print help (see a summary with '-h')

```

## Examples


File tracking works only in Xvc repositories.

```console
$ git init
...
$ xvc init

```
Let's create a directory tree for these examples. 

```console
$ xvc-test-helper create-directory-tree --directories 4 --files 3  --seed 20231021
$ tree
.
├── dir-0001
│   ├── file-0001.bin
│   ├── file-0002.bin
│   └── file-0003.bin
├── dir-0002
│   ├── file-0001.bin
│   ├── file-0002.bin
│   └── file-0003.bin
├── dir-0003
│   ├── file-0001.bin
│   ├── file-0002.bin
│   └── file-0003.bin
└── dir-0004
    ├── file-0001.bin
    ├── file-0002.bin
    └── file-0003.bin

5 directories, 12 files

```

By default, the command runs similar to `git add` and `git commit`. 

You can track individual files.

```console
$ xvc file track dir-0001/file-0001.bin
```

You can track directories with the same command. 

```console
$ xvc file track dir-0002/
```

You can specify more than one target in a single command. 

```console
$ xvc file track dir-0001/file-0002.bin dir-0001/file-0003.bin
```

When you track a file, Xvc moves the file to the cache directory under `.xvc/`
and _connects_ the workspace file with the cached file. This _connection_ is
called rechecking and analogous to Git checkout. For example, the above
commands create a directory tree under `.xvc` as follows: 

```console
$ tree .xvc/b3
.xvc/b3
├── 493
│   └── eeb
│       └── 6525ea5e94e1e760371108e4a525c696c773a774a4818e941fd6d1af79
│           └── 0.bin
├── ab3
│   └── 619
│       └── 814cae0456a5a291e4d5c8d339a8389630e476f9f9e8d3a09accc919f0
│           └── 0.bin
└── e51
    └── 7d6
        └── b9a3617fdcd96bd128142a39f1eca26ed77a338d2b93ba4921a0116c70
            └── 0.bin

10 directories, 3 files

```

There are different _recheck (checkout) methods_ that Xvc connects the
workspace file to the cache. The default method for this is copying the file to
the workspace. This way a separate copy of the cache file is created in the workspace. 

If you want to make this connection with symbolic links, you can specify it with `--recheck-method` option. 

```console
$ xvc file track --recheck-method symlink dir-0003/file-0001.bin
$ ls -l dir-0003/file-0001.bin
lrwxr-xr-x  1 iex  staff  181 Oct  9 12:17 dir-0003/file-0001.bin -> [CWD]/.xvc/b3/e51/7d6/b9a3617fdcd96bd128142a39f1eca26ed77a338d2b93ba4921a0116c70/0.bin

```

You can also use `--hardlink` and `--reflink` options. Please see [`xvc file recheck`](/ref/xvc-file-recheck/) reference for details.  

```console
$ xvc file track --recheck-method hardlink dir-0003/file-0002.bin
$ xvc file track --recheck-method reflink dir-0003/file-0003.bin
$ ls -l dir-0003/
total 16
lrwxr-xr-x  1 iex  staff   181 Oct  9 12:17 file-0001.bin -> [CWD]/.xvc/b3/e51/7d6/b9a3617fdcd96bd128142a39f1eca26ed77a338d2b93ba4921a0116c70/0.bin
-r--r--r--  2 iex  staff  2002 Oct  9 12:16 file-0002.bin
-r--r--r--  1 iex  staff  2003 Oct  9 12:16 file-0003.bin

```


```admonish info
Note that, unlike DVC that specifies checkout/recheck option repository wide,
Xvc lets you specify per file. You can recheck files data files as symbolic
links (which are non-writable) and save space and make model files as copies of
the cached original and commit (carry-in) every time they change.

```

When you track a file in Xvc, it's automatically commit (carry-in) to the cache
directory. If you want to postpone this operation and don't need a cached copy
for a file, you can use `--no-commit` option. You can later use [xvc file
carry-in](/ref/xvc-file-carry-in) command to move these files to the repository
cache.  

```console
$ xvc file track --no-commit --recheck-method symlink dir-0004/
$ ls -l dir-0004/
total 24
-rw-r--r--  1 iex  staff  2001 Oct  9 12:16 file-0001.bin
-rw-r--r--  1 iex  staff  2002 Oct  9 12:16 file-0002.bin
-rw-r--r--  1 iex  staff  2003 Oct  9 12:16 file-0003.bin

$ xvc file list dir-0004/
FS        2003 2023-10-09 09:16:59 ab361981 ab361981 dir-0004/file-0003.bin
FS        2002 2023-10-09 09:16:59 493eeb65 493eeb65 dir-0004/file-0002.bin
FS        2001 2023-10-09 09:16:59 e517d6b9 e517d6b9 dir-0004/file-0001.bin
Total #: 3 Workspace Size:        6006 Cached Size:        6006


```
You can carry-in (commit) these files to the cache with `xvc file carry-in` command. 

```console
$ xvc file carry-in --force dir-0004/
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
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "core.guid": String(
                    "11d5ac05849d8d74",
                ),
                "file.track.text_or_binary": String(
                    "auto",
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
                "file.recheck.method": String(
                    "copy",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "pipeline.process_pool_size": Integer(
                    4,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "git.command": String(
                    "git",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "file.recheck.method": String(
                    "copy",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "core.guid": String(
                    "1b8ee2c266971fae",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
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
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "git.command": String(
                    "git",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.use_git": Boolean(
                    true,
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
                    "quiet",
                ),
                "core.quiet": Boolean(
                    false,
                ),
            },
        },
    ],
    the_config: {
        "pipeline.process_pool_size": XvcConfigValue {
            source: Project,
            value: Integer(
                4,
            ),
        },
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
            ),
        },
        "file.list.recursive": XvcConfigValue {
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
        "git.auto_stage": XvcConfigValue {
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
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
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
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
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
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "1b8ee2c266971fae",
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
        "file.list.no_summary": XvcConfigValue {
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
guid = /"11d5ac05849d8d74/"
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
    "[CWD]/.xvc/ec/1696843019321783",
    "[CWD]/.xvc/ec/1696843019324143",
    "[CWD]/.xvc/ec/1696843019827290",
    "[CWD]/.xvc/ec/1696843020211923",
    "[CWD]/.xvc/ec/1696843020580610",
    "[CWD]/.xvc/ec/1696843020917025",
    "[CWD]/.xvc/ec/1696843021250333",
    "[CWD]/.xvc/ec/1696843021596820",
    "[CWD]/.xvc/ec/1696843021905712",
]
[TRACE][file/src/lib.rs::157] opts: XvcFileCLI {
    verbosity: 0,
    quiet: false,
    workdir: ".",
    config: None,
    no_system_config: false,
    no_user_config: false,
    no_project_config: false,
    no_local_config: false,
    no_env_config: false,
    subcommand: CarryIn(
        CarryInCLI {
            text_or_binary: None,
            force: true,
            no_parallel: false,
            targets: Some(
                [
                    "dir-0004/",
                ],
            ),
        },
    ),
}
[TRACE][file/src/carry_in/mod.rs::109] cli_opts: CarryInCLI {
    text_or_binary: None,
    force: true,
    no_parallel: false,
    targets: Some(
        [
            "dir-0004/",
        ],
    ),
}
[TRACE][file/src/carry_in/mod.rs::110] xvc_root: XvcRootInner {
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
                    "pipeline.default_params_file": String(
                        "params.yaml",
                    ),
                    "pipeline.current_pipeline": String(
                        "default",
                    ),
                    "core.guid": String(
                        "11d5ac05849d8d74",
                    ),
                    "file.track.text_or_binary": String(
                        "auto",
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
                    "file.recheck.method": String(
                        "copy",
                    ),
                    "file.list.recursive": Boolean(
                        false,
                    ),
                    "core.verbosity": String(
                        "error",
                    ),
                    "pipeline.process_pool_size": Integer(
                        4,
                    ),
                    "file.list.format": String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                    "git.use_git": Boolean(
                        true,
                    ),
                    "file.track.no_commit": Boolean(
                        false,
                    ),
                    "git.auto_commit": Boolean(
                        true,
                    ),
                    "file.track.no_parallel": Boolean(
                        false,
                    ),
                    "file.track.force": Boolean(
                        false,
                    ),
                    "pipeline.default": String(
                        "default",
                    ),
                    "git.command": String(
                        "git",
                    ),
                    "file.list.no_summary": Boolean(
                        false,
                    ),
                    "file.carry-in.no_parallel": Boolean(
                        false,
                    ),
                    "file.carry-in.force": Boolean(
                        false,
                    ),
                },
            },
            XvcConfigMap {
                source: Project,
                map: {
                    "file.recheck.method": String(
                        "copy",
                    ),
                    "file.carry-in.no_parallel": Boolean(
                        false,
                    ),
                    "cache.algorithm": String(
                        "blake3",
                    ),
                    "pipeline.default_params_file": String(
                        "params.yaml",
                    ),
                    "file.list.sort": String(
                        "name-desc",
                    ),
                    "file.carry-in.force": Boolean(
                        false,
                    ),
                    "file.track.no_commit": Boolean(
                        false,
                    ),
                    "core.guid": String(
                        "1b8ee2c266971fae",
                    ),
                    "file.track.force": Boolean(
                        false,
                    ),
                    "git.auto_stage": Boolean(
                        false,
                    ),
                    "file.track.text_or_binary": String(
                        "auto",
                    ),
                    "file.list.no_summary": Boolean(
                        false,
                    ),
                    "pipeline.default": String(
                        "default",
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
                    "file.track.no_parallel": Boolean(
                        false,
                    ),
                    "file.list.format": String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                    "git.command": String(
                        "git",
                    ),
                    "pipeline.current_pipeline": String(
                        "default",
                    ),
                    "git.use_git": Boolean(
                        true,
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
                        "quiet",
                    ),
                    "core.quiet": Boolean(
                        false,
                    ),
                },
            },
        ],
        the_config: {
            "pipeline.process_pool_size": XvcConfigValue {
                source: Project,
                value: Integer(
                    4,
                ),
            },
            "cache.algorithm": XvcConfigValue {
                source: Project,
                value: String(
                    "blake3",
                ),
            },
            "file.list.recursive": XvcConfigValue {
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
            "git.auto_stage": XvcConfigValue {
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
            "file.list.sort": XvcConfigValue {
                source: Project,
                value: String(
                    "name-desc",
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
            "git.use_git": XvcConfigValue {
                source: Project,
                value: Boolean(
                    true,
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
            "pipeline.default_params_file": XvcConfigValue {
                source: Project,
                value: String(
                    "params.yaml",
                ),
            },
            "core.guid": XvcConfigValue {
                source: Project,
                value: String(
                    "1b8ee2c266971fae",
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
            "file.list.no_summary": XvcConfigValue {
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
guid = /"11d5ac05849d8d74/"
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
    },
    local_config_path: AbsolutePath(
        "[CWD]/.xvc/config.local.toml",
    ),
    project_config_path: AbsolutePath(
        "[CWD]/.xvc/config.toml",
    ),
    entity_generator: XvcEntityGenerator {
        counter: 14,
        random: 12517610610491036851,
        dirty: false,
    },
}
[TRACE][file/src/carry_in/mod.rs::113] opts: CarryInCLI {
    text_or_binary: Some(
        FileTextOrBinary(
            Auto,
        ),
    ),
    force: true,
    no_parallel: false,
    targets: Some(
        [
            "dir-0004/",
        ],
    ),
}
[TRACE][file/src/common/mod.rs::139] targets: Some(
    [
        "dir-0004/",
    ],
)
[TRACE][file/src/common/mod.rs::156] t: "dir-0004/"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 1 literals, 0 basenames, 0 extensions, 1 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs::174] paths: HStore {
    map: {
        XvcEntity(
            13,
            6826636397237250639,
        ): XvcPath(
            "dir-0004/file-0003.bin",
        ),
        XvcEntity(
            11,
            6826636397237250639,
        ): XvcPath(
            "dir-0004/file-0002.bin",
        ),
        XvcEntity(
            12,
            6826636397237250639,
        ): XvcPath(
            "dir-0004/file-0001.bin",
        ),
    },
}
[TRACE][file/src/common/mod.rs::144] paths: HStore {
    map: {
        XvcEntity(
            13,
            6826636397237250639,
        ): XvcPath(
            "dir-0004/file-0003.bin",
        ),
        XvcEntity(
            11,
            6826636397237250639,
        ): XvcPath(
            "dir-0004/file-0002.bin",
        ),
        XvcEntity(
            12,
            6826636397237250639,
        ): XvcPath(
            "dir-0004/file-0001.bin",
        ),
    },
}
[TRACE][file/src/carry_in/mod.rs::116] targets: HStore {
    map: {
        XvcEntity(
            13,
            6826636397237250639,
        ): XvcPath(
            "dir-0004/file-0003.bin",
        ),
        XvcEntity(
            11,
            6826636397237250639,
        ): XvcPath(
            "dir-0004/file-0002.bin",
        ),
        XvcEntity(
            12,
            6826636397237250639,
        ): XvcPath(
            "dir-0004/file-0001.bin",
        ),
    },
}
[TRACE][file/src/common/compare.rs::38] pmm: {
    XvcPath(
        "dir-0004/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1696843019,
                tv_nsec: 469258001,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1696843019,
                tv_nsec: 469467407,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1696843019,
                tv_nsec: 469068802,
            },
        ),
    },
}
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "dir-0004/file-0002.bin",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    11,
    6826636397237250639,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "dir-0004/file-0003.bin",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    13,
    6826636397237250639,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "dir-0004/file-0001.bin",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    12,
    6826636397237250639,
)
[TRACE][file/src/carry_in/mod.rs::155] content_digest_diff: HStore {
    map: {
        XvcEntity(
            13,
            6826636397237250639,
        ): Skipped,
        XvcEntity(
            11,
            6826636397237250639,
        ): Skipped,
        XvcEntity(
            12,
            6826636397237250639,
        ): Skipped,
    },
}
[TRACE][file/src/carry_in/mod.rs::200] xvc_paths_to_carry: HStore {
    map: {
        XvcEntity(
            12,
            6826636397237250639,
        ): XvcPath(
            "dir-0004/file-0001.bin",
        ),
        XvcEntity(
            11,
            6826636397237250639,
        ): XvcPath(
            "dir-0004/file-0002.bin",
        ),
        XvcEntity(
            13,
            6826636397237250639,
        ): XvcPath(
            "dir-0004/file-0003.bin",
        ),
    },
}
[TRACE][file/src/carry_in/mod.rs::249] abs_cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/493/eeb/6525ea5e94e1e760371108e4a525c696c773a774a4818e941fd6d1af79/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::250] file_perm: Permissions(
    FilePermissions {
        mode: 33060,
    },
)
[TRACE][file/src/carry_in/mod.rs::249] abs_cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/e51/7d6/b9a3617fdcd96bd128142a39f1eca26ed77a338d2b93ba4921a0116c70/0.bin",
)
[TRACE][file/src/common/mod.rs::423] cache_dir: "[CWD]/.xvc/b3/493/eeb/6525ea5e94e1e760371108e4a525c696c773a774a4818e941fd6d1af79"
[TRACE][file/src/carry_in/mod.rs::249] abs_cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/ab3/619/814cae0456a5a291e4d5c8d339a8389630e476f9f9e8d3a09accc919f0/0.bin",
)
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[TRACE][file/src/carry_in/mod.rs::250] file_perm: Permissions(
    FilePermissions {
        mode: 33060,
    },
)
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/dir-0002/**", re: "(?-u)^(?:/|/.*/)dir//-0002/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('d'), Literal('i'), Literal('r'), Literal('-'), Literal('0'), Literal('0'), Literal('0'), Literal('2'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/dir-0004/**", re: "(?-u)^(?:/|/.*/)dir//-0004/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('d'), Literal('i'), Literal('r'), Literal('-'), Literal('0'), Literal('0'), Literal('0'), Literal('4'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 3 regexes
[TRACE][file/src/common/mod.rs::423] cache_dir: "[CWD]/.xvc/b3/ab3/619/814cae0456a5a291e4d5c8d339a8389630e476f9f9e8d3a09accc919f0"
[TRACE][file/src/carry_in/mod.rs::273] target_path: AbsolutePath(
    "[CWD]/dir-0004/file-0002.bin",
)
[TRACE][file/src/carry_in/mod.rs::250] file_perm: Permissions(
    FilePermissions {
        mode: 33060,
    },
)
[TRACE][file/src/common/mod.rs::299] parent_dir: AbsolutePath(
    "[CWD]/dir-0004",
)
[TRACE][file/src/common/mod.rs::318] path: AbsolutePath(
    "[CWD]/dir-0004/file-0002.bin",
)
[TRACE][file/src/common/mod.rs::319] recheck_method: Symlink
[TRACE][file/src/common/mod.rs::366] "Before return": "Before return"
[TRACE][file/src/carry_in/mod.rs::273] target_path: AbsolutePath(
    "[CWD]/dir-0004/file-0003.bin",
)
[TRACE][file/src/common/mod.rs::299] parent_dir: AbsolutePath(
    "[CWD]/dir-0004",
)
[TRACE][file/src/common/mod.rs::318] path: AbsolutePath(
    "[CWD]/dir-0004/file-0003.bin",
)
[TRACE][file/src/common/mod.rs::319] recheck_method: Symlink
[TRACE][file/src/common/mod.rs::423] cache_dir: "[CWD]/.xvc/b3/e51/7d6/b9a3617fdcd96bd128142a39f1eca26ed77a338d2b93ba4921a0116c70"
[TRACE][file/src/common/mod.rs::366] "Before return": "Before return"
[TRACE][file/src/carry_in/mod.rs::273] target_path: AbsolutePath(
    "[CWD]/dir-0004/file-0001.bin",
)
[TRACE][file/src/common/mod.rs::299] parent_dir: AbsolutePath(
    "[CWD]/dir-0004",
)
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[TRACE][file/src/common/mod.rs::318] path: AbsolutePath(
    "[CWD]/dir-0004/file-0001.bin",
)
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[TRACE][file/src/common/mod.rs::319] recheck_method: Symlink
[TRACE][file/src/common/mod.rs::366] "Before return": "Before return"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/dir-0002/**", re: "(?-u)^(?:/|/.*/)dir//-0002/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('d'), Literal('i'), Literal('r'), Literal('-'), Literal('0'), Literal('0'), Literal('0'), Literal('2'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/dir-0004/**", re: "(?-u)^(?:/|/.*/)dir//-0004/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('d'), Literal('i'), Literal('r'), Literal('-'), Literal('0'), Literal('0'), Literal('0'), Literal('4'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 3 regexes
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/dir-0002/**", re: "(?-u)^(?:/|/.*/)dir//-0002/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('d'), Literal('i'), Literal('r'), Literal('-'), Literal('0'), Literal('0'), Literal('0'), Literal('2'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/dir-0004/**", re: "(?-u)^(?:/|/.*/)dir//-0004/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('d'), Literal('i'), Literal('r'), Literal('-'), Literal('0'), Literal('0'), Literal('0'), Literal('4'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 3 regexes
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/dir-0002/**", re: "(?-u)^(?:/|/.*/)dir//-0002/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('d'), Literal('i'), Literal('r'), Literal('-'), Literal('0'), Literal('0'), Literal('0'), Literal('2'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/dir-0004/**", re: "(?-u)^(?:/|/.*/)dir//-0004/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('d'), Literal('i'), Literal('r'), Literal('-'), Literal('0'), Literal('0'), Literal('0'), Literal('4'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 3 regexes
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/dir-0002/**", re: "(?-u)^(?:/|/.*/)dir//-0002/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('d'), Literal('i'), Literal('r'), Literal('-'), Literal('0'), Literal('0'), Literal('0'), Literal('2'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/dir-0004/**", re: "(?-u)^(?:/|/.*/)dir//-0004/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('d'), Literal('i'), Literal('r'), Literal('-'), Literal('0'), Literal('0'), Literal('0'), Literal('4'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 3 regexes
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/dir-0002/**", re: "(?-u)^(?:/|/.*/)dir//-0002/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('d'), Literal('i'), Literal('r'), Literal('-'), Literal('0'), Literal('0'), Literal('0'), Literal('2'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/dir-0004/**", re: "(?-u)^(?:/|/.*/)dir//-0004/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('d'), Literal('i'), Literal('r'), Literal('-'), Literal('0'), Literal('0'), Literal('0'), Literal('4'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 3 regexes
[TRACE][file/src/common/mod.rs::467] records.len(): 12
[TRACE][file/src/common/mod.rs::469] new_store.len(): 12
[TRACE][file/src/common/mod.rs::467] records.len(): 12
[TRACE][file/src/common/mod.rs::469] new_store.len(): 12
[TRACE][file/src/common/mod.rs::467] records.len(): 12
[TRACE][file/src/common/mod.rs::469] new_store.len(): 12
[TRACE][lib/src/cli/mod.rs::381] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::384] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc --debug file carry-in --force dir-0004/"
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

$ ls -l dir-0004/
total 0
lrwxr-xr-x  1 iex  staff  181 Oct  9 12:17 file-0001.bin -> [CWD]/.xvc/b3/e51/7d6/b9a3617fdcd96bd128142a39f1eca26ed77a338d2b93ba4921a0116c70/0.bin
lrwxr-xr-x  1 iex  staff  181 Oct  9 12:17 file-0002.bin -> [CWD]/.xvc/b3/493/eeb/6525ea5e94e1e760371108e4a525c696c773a774a4818e941fd6d1af79/0.bin
lrwxr-xr-x  1 iex  staff  181 Oct  9 12:17 file-0003.bin -> [CWD]/.xvc/b3/ab3/619/814cae0456a5a291e4d5c8d339a8389630e476f9f9e8d3a09accc919f0/0.bin

```


## Caveats

- This command doesn't discriminate symbolic links or hardlinks. 
Links are followed and any broken links may cause errors. 

- Under the hood, Xvc tracks only the files, not directories. 
Directories are considered as path collections.
It doesn't matter if you track a directory or files in it separately.

## Technical Details

- Detecting changes in files and directories employ different kinds of [associated digests](/concepts/associated-digest.md).
If a file has different metadata digest, its content digest is calculated.
If file's content digest has changed, the file is considered changed.
A directory that contains different set of files, or files with changed content is considered changed.



