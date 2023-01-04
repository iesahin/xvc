# xvc file list


## Synopsis 

```console
$ xvc file list --help
List tracked and untracked elements in the workspace

Usage: xvc file list [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to list.
          
          If not supplied, lists all files under the current directory.

Options:
  -f, --format <FORMAT>
          A string for each row of the output table
          
          The following are the keys for each row: - {{acd}}:  actual content digest. The hash of the workspace file's content.
          
          - {{aft}}:  actual file type. Whether the entry is a file (F), directory (D), symlink (S), hardlink (H) or reflink (R).
          
          - {{asz}}:  actual size. The size of the workspace file in bytes. It uses MB, GB and TB to represent sizes larger than 1MB.
          
          - {{ats}}:  actual timestamp. The timestamp of the workspace file.
          
          - {{name}}: The name of the file or directory.
          
          - {{cst}}:  cache status. One of "=", ">", "<", "X", or "?" to show whether the file timestamp is the same as the cached timestamp, newer, older, not cached or not tracked.
          
          - {{rcd}}:  recorded content digest. The hash of the cached content.
          
          - {{rct}}:  recorded cache type. Whether the entry is linked to the workspace as a copy (C), symlink (S), hardlink (H) or reflink (R).
          
          - {{rsz}}:  recorded size. The size of the cached content in bytes. It uses MB, GB and TB to represent sizes larged than 1MB.
          
          - {{rts}}:  recorded timestamp. The timestamp of the cached content.
          
          The default format can be set with file.list.format in the config file.

  -s, --sort-criteria <SORT_CRITERIA>
          Sort column.
          
          It can be one of none (default), name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
          
          The default option can be set with file.list.sort in the config file.

      --no-summary
          Don't show total number and size of the listed files.
          
          The default option can be set with file.list.no_summary in the config file.

  -h, --help
          Print help information (use `-h` for a summary)

```

## Examples

For these examples, we'll create a directory tree with five directories, each
having a file.

```console
$ xvc-test-helper create-directory-tree --directories 5 --files 5

$ tree
.
├── dir-0001
│   ├── file-0001.bin
│   ├── file-0002.bin
│   ├── file-0003.bin
│   ├── file-0004.bin
│   └── file-0005.bin
├── dir-0002
│   ├── file-0001.bin
│   ├── file-0002.bin
│   ├── file-0003.bin
│   ├── file-0004.bin
│   └── file-0005.bin
├── dir-0003
│   ├── file-0001.bin
│   ├── file-0002.bin
│   ├── file-0003.bin
│   ├── file-0004.bin
│   └── file-0005.bin
├── dir-0004
│   ├── file-0001.bin
│   ├── file-0002.bin
│   ├── file-0003.bin
│   ├── file-0004.bin
│   └── file-0005.bin
└── dir-0005
    ├── file-0001.bin
    ├── file-0002.bin
    ├── file-0003.bin
    ├── file-0004.bin
    └── file-0005.bin

6 directories, 25 files

```

`xvc file list` command works only in Xvc repositories. As we didn't initialize
a repository yet, it lists nothing.

```console
$ xvc file list 
```

Let's initialize the repository. 

```console
$ git init
...

$ xvc init

```

Now it lists all files and directories.

```console
$ xvc file list
FX        1005 2023-01-04 03:32:50   dir-0005/file-0005.bin           487ebc55
FX        1004 2023-01-04 03:32:50   dir-0005/file-0004.bin           853fe74b
FX        1003 2023-01-04 03:32:50   dir-0005/file-0003.bin           0030d630
FX        1002 2023-01-04 03:32:50   dir-0005/file-0002.bin           d5029243
FX        1001 2023-01-04 03:32:50   dir-0005/file-0001.bin           367a7c54
DX         224 2023-01-04 03:32:50   dir-0005                   
FX        1005 2023-01-04 03:32:50   dir-0004/file-0005.bin           2a5eef53
FX        1004 2023-01-04 03:32:50   dir-0004/file-0004.bin           1f84f6e2
FX        1003 2023-01-04 03:32:50   dir-0004/file-0003.bin           96115170
FX        1002 2023-01-04 03:32:50   dir-0004/file-0002.bin           4f6b7c9f
FX        1001 2023-01-04 03:32:50   dir-0004/file-0001.bin           0404b2b0
DX         224 2023-01-04 03:32:50   dir-0004                   
FX        1005 2023-01-04 03:32:50   dir-0003/file-0005.bin           f9851adb
FX        1004 2023-01-04 03:32:50   dir-0003/file-0004.bin           912afa81
FX        1003 2023-01-04 03:32:50   dir-0003/file-0003.bin           e527dfcc
FX        1002 2023-01-04 03:32:50   dir-0003/file-0002.bin           dac815ea
FX        1001 2023-01-04 03:32:50   dir-0003/file-0001.bin           9eb98899
DX         224 2023-01-04 03:32:50   dir-0003                   
FX        1005 2023-01-04 03:32:50   dir-0002/file-0005.bin           bb45e512
FX        1004 2023-01-04 03:32:50   dir-0002/file-0004.bin           0aab47aa
FX        1003 2023-01-04 03:32:50   dir-0002/file-0003.bin           61982600
FX        1002 2023-01-04 03:32:50   dir-0002/file-0002.bin           1653a440
FX        1001 2023-01-04 03:32:50   dir-0002/file-0001.bin           1f7dc765
DX         224 2023-01-04 03:32:50   dir-0002                   
FX        1005 2023-01-04 03:32:50   dir-0001/file-0005.bin           ab676cbf
FX        1004 2023-01-04 03:32:50   dir-0001/file-0004.bin           efefad08
FX        1003 2023-01-04 03:32:50   dir-0001/file-0003.bin           345cabd6
FX        1002 2023-01-04 03:32:50   dir-0001/file-0002.bin           141d9f90
FX        1001 2023-01-04 03:32:50   dir-0001/file-0001.bin           772f8dcd
DX         224 2023-01-04 03:32:50   dir-0001                   
FX         130 2023-01-04 03:32:50   .xvcignore           ac46bf74
FX         107 2023-01-04 03:32:50   .gitignore           ce9fcf30
Total #: 32 Workspace Size:       26432 Cached Size:           0


```

With the default output format, the first two letters show the path type and
cache type, respectively. 

For example, if you track `dir-0001` as `copy`, the first letter is `F` for the
files and `D` for the directories. The second letter is `C` for files, meaning
the file is a copy of the cached file, and it's `X` for directories that means
they are not in the cache. Similar to Git, Xvc doesn't track only files and
directories are considered as collection of files.

```console
$ xvc file track dir-0001/

$ xvc file list dir-0001/
FX        1005 2023-01-04 03:32:50   dir-0001/file-0005.bin           ab676cbf
FX        1004 2023-01-04 03:32:50   dir-0001/file-0004.bin           efefad08
FX        1003 2023-01-04 03:32:50   dir-0001/file-0003.bin           345cabd6
FX        1002 2023-01-04 03:32:50   dir-0001/file-0002.bin           141d9f90
FX        1001 2023-01-04 03:32:50   dir-0001/file-0001.bin           772f8dcd
FX         149 2023-01-04 03:32:51   dir-0001/.gitignore           bb09cedb
Total #: 6 Workspace Size:        5164 Cached Size:           0


```

If you add another set of files as hardlinks to the cached copies, it will
print the second letter as `H`.

```console
$ xvc file track dir-0002 --cache-type hardlink
[DEBUG][logging/src/lib.rs:222] Terminal logger enabled with level: Trace
[TRACE][core/src/types/xvcroot.rs:204] "."
[DEBUG][core/src/types/xvcroot.rs:210] XVC DIR: "[CWD]"
[DEBUG][config/src/error.rs:72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs:72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][config/src/lib.rs:534] cli_config: [
    "core.verbosity = debug",
    "core.quiet = false",
]
[TRACE][config/src/lib.rs:538] map: {
    "core.quiet": Boolean(
        false,
    ),
    "core.verbosity": String(
        "debug",
    ),
}
[TRACE][config/src/lib.rs:541] conf: XvcConfig {
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
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "core.guid": String(
                    "d178fac9cb760689",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "git.command": String(
                    "git",
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "cache.type": String(
                    "copy",
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "core.guid": String(
                    "7e311213f28d602e",
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
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
                "cache.algorithm": String(
                    "blake3",
                ),
                "cache.type": String(
                    "copy",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.sort": String(
                    "name-desc",
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
                "7e311213f28d602e",
            ),
        },
        "file.track.text_or_binary": XvcConfigValue {
            source: Project,
            value: String(
                "auto",
            ),
        },
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "debug",
            ),
        },
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
            ),
        },
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
            ),
        },
        "pipeline.current_pipeline": XvcConfigValue {
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
        "git.auto_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "file.track.no_parallel": XvcConfigValue {
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
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
            ),
        },
        "cache.type": XvcConfigValue {
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
        "pipeline.default": XvcConfigValue {
            source: Project,
            value: String(
                "default",
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
    },
    init_params: XvcConfigInitParams {
        default_configuration: "/n[core]/n# The repository id. Please do not delete or change it. /n# This is used to identify the repository and generate paths in storages. /n# In the future it may be used to in other ways. /nguid = /"d178fac9cb760689/"/n# Default verbosity level. /n# One of /"error/", /"warn/", /"info/"/nverbosity = /"error/"/n/n[git]/n# Automate git operations. /n# Turning this off leads Xvc to behave as if it's not in a Git repository./n# Not recommended unless you're really not using Git/nuse_git = true/n# Command to run Git process./n# You can set this to an absolute path to specify an executable/n# If set to a non-absolute path, the executable will be searched in $PATH./ncommand = /"git/"/n/n# Commit changes in .xvc/ directory after commands./n# You can set this to false if you want to commit manually. /nauto_commit = true/n/n# Stage changes in .xvc/ directory without committing./n# auto_commit implies auto_stage. /n# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true. /nauto_stage = false/n/n[cache]/n# The cache type for XVC. It may take copy, hardlink, symlink, reflink as values./n# The default is copy to make sure the options is portable./n# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file./n# Note that hardlink and symlink are read-only as they link the files in cache. /ntype = /"copy/"/n# The hash algorithm used for the cache. /n# It may take blake3, blake2, sha2 or sha3 as values. /n# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc./n# The cache path is produced by prepending algorithm name to the cache. /n# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc. /nalgorithm = /"blake3/"/n/n[file]/n/n[file.track]/n/n# Don't move file content to cache after xvc file track/nno_commit = false/n# Force to track files even if they are already tracked./nforce = false/n/n# Xvc calculates file content digest differently for text and binary files./n# This option controls whether to treat files as text or binary./n# It may take auto, text or binary as values./n# Auto check each file individually and treat it as text if it's text./ntext_or_binary = /"auto/"/n/n# Don't use parallelism in track operations. /n# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations./nno_parallel = false/n/n[file.list]/n/n# Format for `xvc file list` rows. You can reorder or remove columns./n# The following are the keys for each row: /n# - {acd64}:  actual content digest. All 64 digits from the workspace file's content./n# - {acd8}:  actual content digest. First 8 digits the file content digest. /n# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),/n#   symlink (S), hardlink (H) or reflink (R). /n# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,/n#   GB and TB to represent sizes larger than 1MB. /n# - {ats}:  actual timestamp. The timestamp of the workspace file./n# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show/n#   whether the file timestamp is the same as the cached timestamp, newer,/n#   older, not cached or not tracked./n# - {name}: The name of the file or directory./n# - {rcd64}:  recorded content digest. All 64 digits./n# - {rcd8}:  recorded content digest. First 8 digits./n# - {rct}:  recorded cache type. Whether the entry is linked to the workspace/n#   as a copy (C), symlink (S), hardlink (H) or reflink (R)./n# - {rsz}:  recorded size. The size of the cached content in bytes. It uses/n#   MB, GB and TB to represent sizes larged than 1MB./n# - {rts}:  recorded timestamp. The timestamp of the cached content./n# /n# There are no escape sequences in the format string. /n# If you want to add a tab, type it to the string./n# If you want to add a literal double curly brace, open an issue. /nformat = /"{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}/"/n/n# Default sort order for `xvc file list`./n# Valid values are/n# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc./nsort = /"name-desc/"/n/n# Do not show a summary for as the final row for `xvc file list`./nno_summary = false/n/n# List files recursively always./nrecursive = false/n/n[file.carry-in]/n# Carry-in the files to cache always, even if they are already present./nforce = false/n/n# Don't use parallel move/copy in carry-in/nno_parallel = false/n/n[pipeline]/n# Name of the current pipeline to run/ncurrent_pipeline = /"default/"/n# Name of the default pipeline/ndefault = /"default/"/n# Name of the default params file name/ndefault_params_file = /"params.yaml/"/n/n",
        current_dir: AbsolutePath(
            "[CWD]",
        ),
        include_system_config: true,
        include_user_config: true,
        project_config_path: Some(
            "[CWD]/.xvc/config.toml",
        ),
        local_config_path: Some(
            "[CWD]/.xvc/config.local.toml",
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
[TRACE][ecs/src/ecs/mod.rs:184] dir: "[CWD]/.xvc/ec"
[TRACE][ecs/src/ecs/mod.rs:194] files: [
    "[CWD]/.xvc/ec/1672803170802632",
    "[CWD]/.xvc/ec/1672803170805486",
    "[CWD]/.xvc/ec/1672803171102478",
    "[CWD]/.xvc/ec/1672803171412292",
    "[CWD]/.xvc/ec/1672803171706934",
]
[TRACE][file/src/lib.rs:130] opts: XvcFileCLI {
    verbosity: 0,
    quiet: false,
    workdir: ".",
    config: None,
    no_system_config: false,
    no_user_config: false,
    no_project_config: false,
    no_local_config: false,
    no_env_config: false,
    subcommand: Track(
        TrackCLI {
            cache_type: Some(
                Hardlink,
            ),
            no_commit: false,
            text_or_binary: None,
            force: false,
            no_parallel: false,
            targets: Some(
                [
                    "dir-0002",
                ],
            ),
        },
    ),
}
[TRACE][file/src/common/mod.rs:268] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][file/src/common/mod.rs:269] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs:285] all_paths: {
    XvcPath(
        "dir-0002/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 674103338,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 671536435,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 672792199,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 676693867,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 675237892,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 677776337,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 674858597,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 677018370,
            },
        ),
    },
    XvcPath(
        "dir-0001/.gitignore",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            149,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803171,
                tv_nsec: 411090776,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 676363238,
            },
        ),
    },
    XvcPath(
        "dir-0004",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            224,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 676895994,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 677602085,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 677248831,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 676526698,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 673470998,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 676852660,
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
                tv_sec: 1672803170,
                tv_nsec: 802839254,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 677429958,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 672508821,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 675565396,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 675823274,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 674435925,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 676104902,
            },
        ),
    },
    XvcPath(
        "dir-0002",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            224,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 674196798,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 672204651,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 678299176,
            },
        ),
    },
    XvcPath(
        "dir-0001",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            256,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803171,
                tv_nsec: 411973203,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 673181245,
            },
        ),
    },
    XvcPath(
        "dir-0005",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            224,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 677814171,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 673783876,
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
                tv_sec: 1672803170,
                tv_nsec: 802775295,
            },
        ),
    },
    XvcPath(
        "dir-0003",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            224,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 675887816,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 671882064,
            },
        ),
    },
}
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 1 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/track/mod.rs:125] targets: {
    XvcPath(
        "dir-0002/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 673470998,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 673181245,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 673783876,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 674435925,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 674103338,
            },
        ),
    },
}
[TRACE][file/src/common/compare.rs:77] pmm: {
    XvcPath(
        "dir-0002/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 673470998,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 673181245,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 673783876,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 674435925,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 674103338,
            },
        ),
    },
}
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0002/file-0002.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    7,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0002/file-0001.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    8,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0002/file-0003.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    9,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0002/file-0005.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    10,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0002/file-0004.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    11,
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0002/file-0005.bin",
    ),
}
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0002/file-0003.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0002/file-0003.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0002/file-0002.bin",
    ),
}
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0002/file-0004.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0002/file-0002.bin",
)
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0002/file-0004.bin",
)
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0002/file-0005.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0002/file-0003.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0002/file-0002.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0002/file-0004.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0002/file-0005.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0002/file-0001.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0002/file-0001.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0002/file-0001.bin",
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                97,
                152,
                38,
                0,
                86,
                129,
                132,
                190,
                237,
                139,
                100,
                201,
                17,
                51,
                61,
                224,
                9,
                177,
                189,
                179,
                1,
                44,
                45,
                188,
                171,
                140,
                15,
                225,
                198,
                26,
                194,
                52,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                31,
                125,
                199,
                101,
                89,
                221,
                105,
                75,
                69,
                103,
                64,
                221,
                254,
                177,
                54,
                252,
                121,
                128,
                128,
                163,
                66,
                102,
                42,
                225,
                169,
                13,
                106,
                108,
                164,
                126,
                179,
                96,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:169] stored_content_digest: None
[TRACE][file/src/common/compare.rs:169] stored_content_digest: None
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                187,
                69,
                229,
                18,
                195,
                165,
                90,
                187,
                160,
                26,
                78,
                15,
                65,
                201,
                46,
                36,
                68,
                214,
                13,
                190,
                12,
                33,
                228,
                247,
                66,
                90,
                124,
                206,
                11,
                254,
                101,
                225,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                10,
                171,
                71,
                170,
                54,
                203,
                23,
                211,
                47,
                29,
                36,
                159,
                17,
                182,
                51,
                167,
                223,
                109,
                105,
                158,
                37,
                93,
                167,
                204,
                176,
                55,
                173,
                21,
                70,
                162,
                24,
                181,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:169] stored_content_digest: None
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                97,
                152,
                38,
                0,
                86,
                129,
                132,
                190,
                237,
                139,
                100,
                201,
                17,
                51,
                61,
                224,
                9,
                177,
                189,
                179,
                1,
                44,
                45,
                188,
                171,
                140,
                15,
                225,
                198,
                26,
                194,
                52,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                22,
                83,
                164,
                64,
                9,
                246,
                106,
                71,
                104,
                95,
                28,
                68,
                28,
                165,
                208,
                183,
                107,
                6,
                154,
                130,
                134,
                170,
                155,
                240,
                250,
                36,
                254,
                91,
                99,
                104,
                43,
                226,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                31,
                125,
                199,
                101,
                89,
                221,
                105,
                75,
                69,
                103,
                64,
                221,
                254,
                177,
                54,
                252,
                121,
                128,
                128,
                163,
                66,
                102,
                42,
                225,
                169,
                13,
                106,
                108,
                164,
                126,
                179,
                96,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                187,
                69,
                229,
                18,
                195,
                165,
                90,
                187,
                160,
                26,
                78,
                15,
                65,
                201,
                46,
                36,
                68,
                214,
                13,
                190,
                12,
                33,
                228,
                247,
                66,
                90,
                124,
                206,
                11,
                254,
                101,
                225,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:234] res: RecordMissing {
    actual: ContentDigest(
        Some(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    97,
                    152,
                    38,
                    0,
                    86,
                    129,
                    132,
                    190,
                    237,
                    139,
                    100,
                    201,
                    17,
                    51,
                    61,
                    224,
                    9,
                    177,
                    189,
                    179,
                    1,
                    44,
                    45,
                    188,
                    171,
                    140,
                    15,
                    225,
                    198,
                    26,
                    194,
                    52,
                ],
            },
        ),
    ),
}
[TRACE][file/src/common/compare.rs:169] stored_content_digest: None
[TRACE][file/src/common/compare.rs:234] res: RecordMissing {
    actual: ContentDigest(
        Some(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    187,
                    69,
                    229,
                    18,
                    195,
                    165,
                    90,
                    187,
                    160,
                    26,
                    78,
                    15,
                    65,
                    201,
                    46,
                    36,
                    68,
                    214,
                    13,
                    190,
                    12,
                    33,
                    228,
                    247,
                    66,
                    90,
                    124,
                    206,
                    11,
                    254,
                    101,
                    225,
                ],
            },
        ),
    ),
}
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                22,
                83,
                164,
                64,
                9,
                246,
                106,
                71,
                104,
                95,
                28,
                68,
                28,
                165,
                208,
                183,
                107,
                6,
                154,
                130,
                134,
                170,
                155,
                240,
                250,
                36,
                254,
                91,
                99,
                104,
                43,
                226,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:234] res: RecordMissing {
    actual: ContentDigest(
        Some(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    31,
                    125,
                    199,
                    101,
                    89,
                    221,
                    105,
                    75,
                    69,
                    103,
                    64,
                    221,
                    254,
                    177,
                    54,
                    252,
                    121,
                    128,
                    128,
                    163,
                    66,
                    102,
                    42,
                    225,
                    169,
                    13,
                    106,
                    108,
                    164,
                    126,
                    179,
                    96,
                ],
            },
        ),
    ),
}
[TRACE][file/src/common/compare.rs:234] res: RecordMissing {
    actual: ContentDigest(
        Some(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    22,
                    83,
                    164,
                    64,
                    9,
                    246,
                    106,
                    71,
                    104,
                    95,
                    28,
                    68,
                    28,
                    165,
                    208,
                    183,
                    107,
                    6,
                    154,
                    130,
                    134,
                    170,
                    155,
                    240,
                    250,
                    36,
                    254,
                    91,
                    99,
                    104,
                    43,
                    226,
                ],
            },
        ),
    ),
}
[TRACE][file/src/common/compare.rs:169] stored_content_digest: None
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                10,
                171,
                71,
                170,
                54,
                203,
                23,
                211,
                47,
                29,
                36,
                159,
                17,
                182,
                51,
                167,
                223,
                109,
                105,
                158,
                37,
                93,
                167,
                204,
                176,
                55,
                173,
                21,
                70,
                162,
                24,
                181,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:234] res: RecordMissing {
    actual: ContentDigest(
        Some(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    10,
                    171,
                    71,
                    170,
                    54,
                    203,
                    23,
                    211,
                    47,
                    29,
                    36,
                    159,
                    17,
                    182,
                    51,
                    167,
                    223,
                    109,
                    105,
                    158,
                    37,
                    93,
                    167,
                    204,
                    176,
                    55,
                    173,
                    21,
                    70,
                    162,
                    24,
                    181,
                ],
            },
        ),
    ),
}
[TRACE][file/src/track/mod.rs:188] content_digest_diff: HStore {
    map: {
        XvcEntity(
            7,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            22,
                            83,
                            164,
                            64,
                            9,
                            246,
                            106,
                            71,
                            104,
                            95,
                            28,
                            68,
                            28,
                            165,
                            208,
                            183,
                            107,
                            6,
                            154,
                            130,
                            134,
                            170,
                            155,
                            240,
                            250,
                            36,
                            254,
                            91,
                            99,
                            104,
                            43,
                            226,
                        ],
                    },
                ),
            ),
        },
        XvcEntity(
            11,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            10,
                            171,
                            71,
                            170,
                            54,
                            203,
                            23,
                            211,
                            47,
                            29,
                            36,
                            159,
                            17,
                            182,
                            51,
                            167,
                            223,
                            109,
                            105,
                            158,
                            37,
                            93,
                            167,
                            204,
                            176,
                            55,
                            173,
                            21,
                            70,
                            162,
                            24,
                            181,
                        ],
                    },
                ),
            ),
        },
        XvcEntity(
            9,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            97,
                            152,
                            38,
                            0,
                            86,
                            129,
                            132,
                            190,
                            237,
                            139,
                            100,
                            201,
                            17,
                            51,
                            61,
                            224,
                            9,
                            177,
                            189,
                            179,
                            1,
                            44,
                            45,
                            188,
                            171,
                            140,
                            15,
                            225,
                            198,
                            26,
                            194,
                            52,
                        ],
                    },
                ),
            ),
        },
        XvcEntity(
            8,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            31,
                            125,
                            199,
                            101,
                            89,
                            221,
                            105,
                            75,
                            69,
                            103,
                            64,
                            221,
                            254,
                            177,
                            54,
                            252,
                            121,
                            128,
                            128,
                            163,
                            66,
                            102,
                            42,
                            225,
                            169,
                            13,
                            106,
                            108,
                            164,
                            126,
                            179,
                            96,
                        ],
                    },
                ),
            ),
        },
        XvcEntity(
            10,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            187,
                            69,
                            229,
                            18,
                            195,
                            165,
                            90,
                            187,
                            160,
                            26,
                            78,
                            15,
                            65,
                            201,
                            46,
                            36,
                            68,
                            214,
                            13,
                            190,
                            12,
                            33,
                            228,
                            247,
                            66,
                            90,
                            124,
                            206,
                            11,
                            254,
                            101,
                            225,
                        ],
                    },
                ),
            ),
        },
    },
}
[TRACE][file/src/common/mod.rs:508] records.len(): 5
[TRACE][file/src/common/mod.rs:510] new_store.len(): 10
[TRACE][file/src/common/mod.rs:508] records.len(): 5
[TRACE][file/src/common/mod.rs:510] new_store.len(): 10
[TRACE][file/src/common/mod.rs:508] records.len(): 5
[TRACE][file/src/common/mod.rs:510] new_store.len(): 10
[TRACE][file/src/common/mod.rs:508] records.len(): 5
[TRACE][file/src/common/mod.rs:510] new_store.len(): 10
[TRACE][file/src/common/mod.rs:508] records.len(): 5
[TRACE][file/src/common/mod.rs:510] new_store.len(): 10
[TRACE][file/src/track/mod.rs:198] current_xvc_metadata_store.len(): 10
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 1 regexes
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 1 regexes
[TRACE][file/src/track/mod.rs:420] dir_map: {}
[TRACE][file/src/track/mod.rs:455] file_map: {
    "[CWD]/dir-0002/file-0005.bin": "[CWD]/dir-0002/.gitignore",
    "[CWD]/dir-0002/file-0002.bin": "[CWD]/dir-0002/.gitignore",
    "[CWD]/dir-0002/file-0003.bin": "[CWD]/dir-0002/.gitignore",
    "[CWD]/dir-0002/file-0001.bin": "[CWD]/dir-0002/.gitignore",
    "[CWD]/dir-0002/file-0004.bin": "[CWD]/dir-0002/.gitignore",
}
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/0aa/b47/aa36cb17d32f1d249f11b633a7df6d699e255da7ccb037ad1546a218b5"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/1f7/dc7/6559dd694b456740ddfeb136fc798080a342662ae1a90d6a6ca47eb360"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/bb4/5e5/12c3a55abba01a4e0f41c92e2444d60dbe0c21e4f7425a7cce0bfe65e1"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/619/826/00568184beed8b64c911333de009b1bdb3012c2dbcab8c0fe1c61ac234"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/165/3a4/4009f66a47685f1c441ca5d0b76b069a8286aa9bf0fa24fe5b63682be2"
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0002/file-0001.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/1f7/dc7/6559dd694b456740ddfeb136fc798080a342662ae1a90d6a6ca47eb360/0.bin",
)
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0002/file-0004.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/0aa/b47/aa36cb17d32f1d249f11b633a7df6d699e255da7ccb037ad1546a218b5/0.bin",
)
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0002/file-0005.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/bb4/5e5/12c3a55abba01a4e0f41c92e2444d60dbe0c21e4f7425a7cce0bfe65e1/0.bin",
)
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0002/file-0002.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/165/3a4/4009f66a47685f1c441ca5d0b76b069a8286aa9bf0fa24fe5b63682be2/0.bin",
)
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0002/file-0003.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/619/826/00568184beed8b64c911333de009b1bdb3012c2dbcab8c0fe1c61ac234/0.bin",
)
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0002/file-0001.bin -> b3/1f7/dc7/6559dd694b456740ddfeb136fc798080a342662ae1a90d6a6ca47eb360/0.bin
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0002/file-0001.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Hardlink
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0002/file-0004.bin -> b3/0aa/b47/aa36cb17d32f1d249f11b633a7df6d699e255da7ccb037ad1546a218b5/0.bin
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0002/file-0004.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Hardlink
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0002/file-0005.bin -> b3/bb4/5e5/12c3a55abba01a4e0f41c92e2444d60dbe0c21e4f7425a7cce0bfe65e1/0.bin
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0002/file-0005.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Hardlink
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0002/file-0003.bin -> b3/619/826/00568184beed8b64c911333de009b1bdb3012c2dbcab8c0fe1c61ac234/0.bin
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0002/file-0003.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Hardlink
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0002/file-0002.bin -> b3/165/3a4/4009f66a47685f1c441ca5d0b76b069a8286aa9bf0fa24fe5b63682be2/0.bin
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0002/file-0002.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Hardlink
[INFO][lib/src/cli/mod.rs:362] [INFO] [HARDLINK] [CWD]/.xvc/b3/1f7/dc7/6559dd694b456740ddfeb136fc798080a342662ae1a90d6a6ca47eb360/0.bin -> [CWD]/dir-0002/file-0001.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/1f7/dc7/6559dd694b456740ddfeb136fc798080a342662ae1a90d6a6ca47eb360/0.bin -> dir-0002/file-0001.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [HARDLINK] [CWD]/.xvc/b3/bb4/5e5/12c3a55abba01a4e0f41c92e2444d60dbe0c21e4f7425a7cce0bfe65e1/0.bin -> [CWD]/dir-0002/file-0005.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/bb4/5e5/12c3a55abba01a4e0f41c92e2444d60dbe0c21e4f7425a7cce0bfe65e1/0.bin -> dir-0002/file-0005.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [HARDLINK] [CWD]/.xvc/b3/0aa/b47/aa36cb17d32f1d249f11b633a7df6d699e255da7ccb037ad1546a218b5/0.bin -> [CWD]/dir-0002/file-0004.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/0aa/b47/aa36cb17d32f1d249f11b633a7df6d699e255da7ccb037ad1546a218b5/0.bin -> dir-0002/file-0004.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [HARDLINK] [CWD]/.xvc/b3/619/826/00568184beed8b64c911333de009b1bdb3012c2dbcab8c0fe1c61ac234/0.bin -> [CWD]/dir-0002/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/619/826/00568184beed8b64c911333de009b1bdb3012c2dbcab8c0fe1c61ac234/0.bin -> dir-0002/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [HARDLINK] [CWD]/.xvc/b3/165/3a4/4009f66a47685f1c441ca5d0b76b069a8286aa9bf0fa24fe5b63682be2/0.bin -> [CWD]/dir-0002/file-0002.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/165/3a4/4009f66a47685f1c441ca5d0b76b069a8286aa9bf0fa24fe5b63682be2/0.bin -> dir-0002/file-0002.bin
[TRACE][lib/src/cli/mod.rs:294] "Before handle_git_automation": "Before handle_git_automation"
[DEBUG][lib/src/cli/mod.rs:491] Using Git: /opt/homebrew/bin/git
[TRACE][lib/src/cli/mod.rs:398] args: [
    "-C",
    "[CWD]",
    "diff",
    "--name-only",
    "--cached",
]
[TRACE][lib/src/cli/mod.rs:424] git_diff_staged_out: ""
[TRACE][lib/src/cli/mod.rs:398] args: [
    "-C",
    "[CWD]",
    "add",
    "[CWD]/.xvc",
    "*.gitignore",
    "*.xvcignore",
]
[DEBUG][lib/src/cli/mod.rs:506] Adding .xvc/ to git: 
[TRACE][lib/src/cli/mod.rs:398] args: [
    "-C",
    "[CWD]",
    "commit",
    "-m",
    "Xvc auto-commit after /'/'",
]
[DEBUG][lib/src/cli/mod.rs:516] Committing .xvc/ to git: [main 0eca3d4] Xvc auto-commit after ''
 7 files changed, 12 insertions(+)
 create mode 100644 .xvc/ec/1672803172008106
 create mode 100644 .xvc/store/cache-type-store/1672803171996589.json
 create mode 100644 .xvc/store/content-digest-store/1672803171996970.json
 create mode 100644 .xvc/store/file-text-or-binary-store/1672803171996761.json
 create mode 100644 .xvc/store/xvc-metadata-store/1672803171996359.json
 create mode 100644 .xvc/store/xvc-path-store/1672803171996057.json
 create mode 100644 dir-0002/.gitignore


$ xvc file list
FX        1005 2023-01-04 03:32:50   dir-0005/file-0005.bin           487ebc55
FX        1004 2023-01-04 03:32:50   dir-0005/file-0004.bin           853fe74b
FX        1003 2023-01-04 03:32:50   dir-0005/file-0003.bin           0030d630
FX        1002 2023-01-04 03:32:50   dir-0005/file-0002.bin           d5029243
FX        1001 2023-01-04 03:32:50   dir-0005/file-0001.bin           367a7c54
DX         224 2023-01-04 03:32:50   dir-0005                   
FX        1005 2023-01-04 03:32:50   dir-0004/file-0005.bin           2a5eef53
FX        1004 2023-01-04 03:32:50   dir-0004/file-0004.bin           1f84f6e2
FX        1003 2023-01-04 03:32:50   dir-0004/file-0003.bin           96115170
FX        1002 2023-01-04 03:32:50   dir-0004/file-0002.bin           4f6b7c9f
FX        1001 2023-01-04 03:32:50   dir-0004/file-0001.bin           0404b2b0
DX         224 2023-01-04 03:32:50   dir-0004                   
FX        1005 2023-01-04 03:32:50   dir-0003/file-0005.bin           f9851adb
FX        1004 2023-01-04 03:32:50   dir-0003/file-0004.bin           912afa81
FX        1003 2023-01-04 03:32:50   dir-0003/file-0003.bin           e527dfcc
FX        1002 2023-01-04 03:32:50   dir-0003/file-0002.bin           dac815ea
FX        1001 2023-01-04 03:32:50   dir-0003/file-0001.bin           9eb98899
DX         224 2023-01-04 03:32:50   dir-0003                   
FH        1005 2023-01-04 03:32:50   dir-0002/file-0005.bin  bb45e512 bb45e512
FH        1004 2023-01-04 03:32:50   dir-0002/file-0004.bin  0aab47aa 0aab47aa
FH        1003 2023-01-04 03:32:50   dir-0002/file-0003.bin  61982600 61982600
FH        1002 2023-01-04 03:32:50   dir-0002/file-0002.bin  1653a440 1653a440
FH        1001 2023-01-04 03:32:50   dir-0002/file-0001.bin  1f7dc765 1f7dc765
FX         149 2023-01-04 03:32:52   dir-0002/.gitignore           2ee28ca0
DX         256 2023-01-04 03:32:52   dir-0002                   
FC        1005 2023-01-04 03:32:50   dir-0001/file-0005.bin  ab676cbf ab676cbf
FC        1004 2023-01-04 03:32:50   dir-0001/file-0004.bin  efefad08 efefad08
FC        1003 2023-01-04 03:32:50   dir-0001/file-0003.bin  345cabd6 345cabd6
FC        1002 2023-01-04 03:32:50   dir-0001/file-0002.bin  141d9f90 141d9f90
FC        1001 2023-01-04 03:32:50   dir-0001/file-0001.bin  772f8dcd 772f8dcd
FX         149 2023-01-04 03:32:51   dir-0001/.gitignore           bb09cedb
DX         256 2023-01-04 03:32:51   dir-0001                   
FX         130 2023-01-04 03:32:50   .xvcignore           ac46bf74
FX         107 2023-01-04 03:32:50   .gitignore           ce9fcf30
Total #: 34 Workspace Size:       26794 Cached Size:       10030


```

Note, as hardlinks are actually files with the same inode in the file system
with alternative paths, they are detected as `F`. 

Symbolic links are typically reported as `SS` in the first letters. 
It means they are symbolic links on the file system and their cache type is also
symbolic links. 

```console
$ xvc -vvvv file track dir-0003 --cache-type symlink
[DEBUG][logging/src/lib.rs:222] Terminal logger enabled with level: Trace
[TRACE][core/src/types/xvcroot.rs:204] "."
[DEBUG][core/src/types/xvcroot.rs:210] XVC DIR: "[CWD]"
[DEBUG][config/src/error.rs:72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs:72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][config/src/lib.rs:534] cli_config: [
    "core.verbosity = debug",
    "core.quiet = false",
]
[TRACE][config/src/lib.rs:538] map: {
    "core.quiet": Boolean(
        false,
    ),
    "core.verbosity": String(
        "debug",
    ),
}
[TRACE][config/src/lib.rs:541] conf: XvcConfig {
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
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "cache.type": String(
                    "copy",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "git.command": String(
                    "git",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "core.guid": String(
                    "81e59a87b4d1e95f",
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "git.command": String(
                    "git",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "core.guid": String(
                    "7e311213f28d602e",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "cache.type": String(
                    "copy",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.track.no_commit": Boolean(
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
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "7e311213f28d602e",
            ),
        },
        "git.auto_stage": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
            ),
        },
        "file.list.recursive": XvcConfigValue {
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
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
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
        "file.list.no_summary": XvcConfigValue {
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
        "file.track.text_or_binary": XvcConfigValue {
            source: Project,
            value: String(
                "auto",
            ),
        },
        "git.auto_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "file.track.no_parallel": XvcConfigValue {
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
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
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
        "cache.type": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
            ),
        },
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
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
    },
    init_params: XvcConfigInitParams {
        default_configuration: "/n[core]/n# The repository id. Please do not delete or change it. /n# This is used to identify the repository and generate paths in storages. /n# In the future it may be used to in other ways. /nguid = /"81e59a87b4d1e95f/"/n# Default verbosity level. /n# One of /"error/", /"warn/", /"info/"/nverbosity = /"error/"/n/n[git]/n# Automate git operations. /n# Turning this off leads Xvc to behave as if it's not in a Git repository./n# Not recommended unless you're really not using Git/nuse_git = true/n# Command to run Git process./n# You can set this to an absolute path to specify an executable/n# If set to a non-absolute path, the executable will be searched in $PATH./ncommand = /"git/"/n/n# Commit changes in .xvc/ directory after commands./n# You can set this to false if you want to commit manually. /nauto_commit = true/n/n# Stage changes in .xvc/ directory without committing./n# auto_commit implies auto_stage. /n# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true. /nauto_stage = false/n/n[cache]/n# The cache type for XVC. It may take copy, hardlink, symlink, reflink as values./n# The default is copy to make sure the options is portable./n# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file./n# Note that hardlink and symlink are read-only as they link the files in cache. /ntype = /"copy/"/n# The hash algorithm used for the cache. /n# It may take blake3, blake2, sha2 or sha3 as values. /n# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc./n# The cache path is produced by prepending algorithm name to the cache. /n# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc. /nalgorithm = /"blake3/"/n/n[file]/n/n[file.track]/n/n# Don't move file content to cache after xvc file track/nno_commit = false/n# Force to track files even if they are already tracked./nforce = false/n/n# Xvc calculates file content digest differently for text and binary files./n# This option controls whether to treat files as text or binary./n# It may take auto, text or binary as values./n# Auto check each file individually and treat it as text if it's text./ntext_or_binary = /"auto/"/n/n# Don't use parallelism in track operations. /n# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations./nno_parallel = false/n/n[file.list]/n/n# Format for `xvc file list` rows. You can reorder or remove columns./n# The following are the keys for each row: /n# - {acd64}:  actual content digest. All 64 digits from the workspace file's content./n# - {acd8}:  actual content digest. First 8 digits the file content digest. /n# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),/n#   symlink (S), hardlink (H) or reflink (R). /n# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,/n#   GB and TB to represent sizes larger than 1MB. /n# - {ats}:  actual timestamp. The timestamp of the workspace file./n# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show/n#   whether the file timestamp is the same as the cached timestamp, newer,/n#   older, not cached or not tracked./n# - {name}: The name of the file or directory./n# - {rcd64}:  recorded content digest. All 64 digits./n# - {rcd8}:  recorded content digest. First 8 digits./n# - {rct}:  recorded cache type. Whether the entry is linked to the workspace/n#   as a copy (C), symlink (S), hardlink (H) or reflink (R)./n# - {rsz}:  recorded size. The size of the cached content in bytes. It uses/n#   MB, GB and TB to represent sizes larged than 1MB./n# - {rts}:  recorded timestamp. The timestamp of the cached content./n# /n# There are no escape sequences in the format string. /n# If you want to add a tab, type it to the string./n# If you want to add a literal double curly brace, open an issue. /nformat = /"{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}/"/n/n# Default sort order for `xvc file list`./n# Valid values are/n# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc./nsort = /"name-desc/"/n/n# Do not show a summary for as the final row for `xvc file list`./nno_summary = false/n/n# List files recursively always./nrecursive = false/n/n[file.carry-in]/n# Carry-in the files to cache always, even if they are already present./nforce = false/n/n# Don't use parallel move/copy in carry-in/nno_parallel = false/n/n[pipeline]/n# Name of the current pipeline to run/ncurrent_pipeline = /"default/"/n# Name of the default pipeline/ndefault = /"default/"/n# Name of the default params file name/ndefault_params_file = /"params.yaml/"/n/n",
        current_dir: AbsolutePath(
            "[CWD]",
        ),
        include_system_config: true,
        include_user_config: true,
        project_config_path: Some(
            "[CWD]/.xvc/config.toml",
        ),
        local_config_path: Some(
            "[CWD]/.xvc/config.local.toml",
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
[TRACE][ecs/src/ecs/mod.rs:184] dir: "[CWD]/.xvc/ec"
[TRACE][ecs/src/ecs/mod.rs:194] files: [
    "[CWD]/.xvc/ec/1672803170802632",
    "[CWD]/.xvc/ec/1672803170805486",
    "[CWD]/.xvc/ec/1672803171102478",
    "[CWD]/.xvc/ec/1672803171412292",
    "[CWD]/.xvc/ec/1672803171706934",
    "[CWD]/.xvc/ec/1672803172008106",
    "[CWD]/.xvc/ec/1672803172348391",
]
[TRACE][file/src/lib.rs:130] opts: XvcFileCLI {
    verbosity: 0,
    quiet: false,
    workdir: ".",
    config: None,
    no_system_config: false,
    no_user_config: false,
    no_project_config: false,
    no_local_config: false,
    no_env_config: false,
    subcommand: Track(
        TrackCLI {
            cache_type: Some(
                Symlink,
            ),
            no_commit: false,
            text_or_binary: None,
            force: false,
            no_parallel: false,
            targets: Some(
                [
                    "dir-0003",
                ],
            ),
        },
    ),
}
[TRACE][file/src/common/mod.rs:268] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][file/src/common/mod.rs:269] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs:285] all_paths: {
    XvcPath(
        "dir-0004/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 676526698,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 676104902,
            },
        ),
    },
    XvcPath(
        "dir-0001/.gitignore",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            149,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803171,
                tv_nsec: 411090776,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 676363238,
            },
        ),
    },
    XvcPath(
        "dir-0002/.gitignore",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            149,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803172,
                tv_nsec: 6312155,
            },
        ),
    },
    XvcPath(
        "dir-0001",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            256,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803171,
                tv_nsec: 411973203,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 672508821,
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
                tv_sec: 1672803170,
                tv_nsec: 802839254,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 678299176,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 677018370,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 672792199,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 671882064,
            },
        ),
    },
    XvcPath(
        "dir-0003",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            224,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 675887816,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 671536435,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 677429958,
            },
        ),
    },
    XvcPath(
        "dir-0005",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            224,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 677814171,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 672204651,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 676852660,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 674435925,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 675823274,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 677248831,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 674858597,
            },
        ),
    },
    XvcPath(
        "dir-0004",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            224,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 676895994,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 673181245,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 677602085,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 677776337,
            },
        ),
    },
    XvcPath(
        "dir-0002",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            256,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803172,
                tv_nsec: 7734588,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 674103338,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 676693867,
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
                tv_sec: 1672803170,
                tv_nsec: 802775295,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 675237892,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 675565396,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 673470998,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 673783876,
            },
        ),
    },
}
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 1 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/track/mod.rs:125] targets: {
    XvcPath(
        "dir-0003/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 675565396,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 675823274,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 674858597,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 675237892,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 676104902,
            },
        ),
    },
}
[TRACE][file/src/common/compare.rs:77] pmm: {
    XvcPath(
        "dir-0003/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 675565396,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 675823274,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 674858597,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 675237892,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803170,
                tv_nsec: 676104902,
            },
        ),
    },
}
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0003/file-0003.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    12,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0003/file-0004.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    13,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0003/file-0001.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    14,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0003/file-0002.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    15,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0003/file-0005.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    16,
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0003/file-0005.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0003/file-0005.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0003/file-0001.bin",
    ),
}
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0005.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0003/file-0003.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0003/file-0003.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0003.bin",
)
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0003/file-0001.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0003/file-0004.bin",
    ),
}
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0003/file-0002.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0003/file-0002.bin",
)
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0003/file-0004.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0002.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0001.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0004.bin",
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                229,
                39,
                223,
                204,
                69,
                223,
                228,
                26,
                206,
                105,
                2,
                247,
                154,
                187,
                130,
                84,
                71,
                233,
                236,
                254,
                187,
                72,
                28,
                55,
                11,
                15,
                2,
                249,
                168,
                77,
                113,
                220,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:169] stored_content_digest: None
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                249,
                133,
                26,
                219,
                25,
                83,
                98,
                193,
                219,
                164,
                26,
                37,
                20,
                174,
                99,
                101,
                79,
                89,
                239,
                94,
                231,
                109,
                78,
                104,
                52,
                177,
                185,
                148,
                126,
                79,
                70,
                158,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:169] stored_content_digest: None
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                229,
                39,
                223,
                204,
                69,
                223,
                228,
                26,
                206,
                105,
                2,
                247,
                154,
                187,
                130,
                84,
                71,
                233,
                236,
                254,
                187,
                72,
                28,
                55,
                11,
                15,
                2,
                249,
                168,
                77,
                113,
                220,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                218,
                200,
                21,
                234,
                228,
                225,
                189,
                192,
                39,
                92,
                168,
                5,
                139,
                148,
                171,
                97,
                118,
                235,
                225,
                1,
                26,
                125,
                217,
                97,
                167,
                73,
                147,
                71,
                171,
                127,
                132,
                204,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:169] stored_content_digest: None
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                249,
                133,
                26,
                219,
                25,
                83,
                98,
                193,
                219,
                164,
                26,
                37,
                20,
                174,
                99,
                101,
                79,
                89,
                239,
                94,
                231,
                109,
                78,
                104,
                52,
                177,
                185,
                148,
                126,
                79,
                70,
                158,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:234] res: RecordMissing {
    actual: ContentDigest(
        Some(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    229,
                    39,
                    223,
                    204,
                    69,
                    223,
                    228,
                    26,
                    206,
                    105,
                    2,
                    247,
                    154,
                    187,
                    130,
                    84,
                    71,
                    233,
                    236,
                    254,
                    187,
                    72,
                    28,
                    55,
                    11,
                    15,
                    2,
                    249,
                    168,
                    77,
                    113,
                    220,
                ],
            },
        ),
    ),
}
[TRACE][file/src/common/compare.rs:234] res: RecordMissing {
    actual: ContentDigest(
        Some(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    249,
                    133,
                    26,
                    219,
                    25,
                    83,
                    98,
                    193,
                    219,
                    164,
                    26,
                    37,
                    20,
                    174,
                    99,
                    101,
                    79,
                    89,
                    239,
                    94,
                    231,
                    109,
                    78,
                    104,
                    52,
                    177,
                    185,
                    148,
                    126,
                    79,
                    70,
                    158,
                ],
            },
        ),
    ),
}
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                218,
                200,
                21,
                234,
                228,
                225,
                189,
                192,
                39,
                92,
                168,
                5,
                139,
                148,
                171,
                97,
                118,
                235,
                225,
                1,
                26,
                125,
                217,
                97,
                167,
                73,
                147,
                71,
                171,
                127,
                132,
                204,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:234] res: RecordMissing {
    actual: ContentDigest(
        Some(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    218,
                    200,
                    21,
                    234,
                    228,
                    225,
                    189,
                    192,
                    39,
                    92,
                    168,
                    5,
                    139,
                    148,
                    171,
                    97,
                    118,
                    235,
                    225,
                    1,
                    26,
                    125,
                    217,
                    97,
                    167,
                    73,
                    147,
                    71,
                    171,
                    127,
                    132,
                    204,
                ],
            },
        ),
    ),
}
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                145,
                42,
                250,
                129,
                67,
                83,
                135,
                163,
                219,
                175,
                62,
                197,
                147,
                176,
                106,
                140,
                31,
                150,
                125,
                221,
                204,
                136,
                90,
                206,
                225,
                211,
                91,
                7,
                10,
                133,
                30,
                144,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                158,
                185,
                136,
                153,
                150,
                54,
                191,
                252,
                177,
                12,
                123,
                215,
                161,
                70,
                171,
                210,
                21,
                122,
                56,
                210,
                112,
                129,
                22,
                239,
                12,
                154,
                115,
                114,
                37,
                2,
                210,
                232,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:169] stored_content_digest: None
[TRACE][file/src/common/compare.rs:169] stored_content_digest: None
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                145,
                42,
                250,
                129,
                67,
                83,
                135,
                163,
                219,
                175,
                62,
                197,
                147,
                176,
                106,
                140,
                31,
                150,
                125,
                221,
                204,
                136,
                90,
                206,
                225,
                211,
                91,
                7,
                10,
                133,
                30,
                144,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                158,
                185,
                136,
                153,
                150,
                54,
                191,
                252,
                177,
                12,
                123,
                215,
                161,
                70,
                171,
                210,
                21,
                122,
                56,
                210,
                112,
                129,
                22,
                239,
                12,
                154,
                115,
                114,
                37,
                2,
                210,
                232,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:234] res: RecordMissing {
    actual: ContentDigest(
        Some(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    145,
                    42,
                    250,
                    129,
                    67,
                    83,
                    135,
                    163,
                    219,
                    175,
                    62,
                    197,
                    147,
                    176,
                    106,
                    140,
                    31,
                    150,
                    125,
                    221,
                    204,
                    136,
                    90,
                    206,
                    225,
                    211,
                    91,
                    7,
                    10,
                    133,
                    30,
                    144,
                ],
            },
        ),
    ),
}
[TRACE][file/src/common/compare.rs:234] res: RecordMissing {
    actual: ContentDigest(
        Some(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    158,
                    185,
                    136,
                    153,
                    150,
                    54,
                    191,
                    252,
                    177,
                    12,
                    123,
                    215,
                    161,
                    70,
                    171,
                    210,
                    21,
                    122,
                    56,
                    210,
                    112,
                    129,
                    22,
                    239,
                    12,
                    154,
                    115,
                    114,
                    37,
                    2,
                    210,
                    232,
                ],
            },
        ),
    ),
}
[TRACE][file/src/track/mod.rs:188] content_digest_diff: HStore {
    map: {
        XvcEntity(
            14,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            158,
                            185,
                            136,
                            153,
                            150,
                            54,
                            191,
                            252,
                            177,
                            12,
                            123,
                            215,
                            161,
                            70,
                            171,
                            210,
                            21,
                            122,
                            56,
                            210,
                            112,
                            129,
                            22,
                            239,
                            12,
                            154,
                            115,
                            114,
                            37,
                            2,
                            210,
                            232,
                        ],
                    },
                ),
            ),
        },
        XvcEntity(
            13,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            145,
                            42,
                            250,
                            129,
                            67,
                            83,
                            135,
                            163,
                            219,
                            175,
                            62,
                            197,
                            147,
                            176,
                            106,
                            140,
                            31,
                            150,
                            125,
                            221,
                            204,
                            136,
                            90,
                            206,
                            225,
                            211,
                            91,
                            7,
                            10,
                            133,
                            30,
                            144,
                        ],
                    },
                ),
            ),
        },
        XvcEntity(
            15,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            218,
                            200,
                            21,
                            234,
                            228,
                            225,
                            189,
                            192,
                            39,
                            92,
                            168,
                            5,
                            139,
                            148,
                            171,
                            97,
                            118,
                            235,
                            225,
                            1,
                            26,
                            125,
                            217,
                            97,
                            167,
                            73,
                            147,
                            71,
                            171,
                            127,
                            132,
                            204,
                        ],
                    },
                ),
            ),
        },
        XvcEntity(
            16,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            249,
                            133,
                            26,
                            219,
                            25,
                            83,
                            98,
                            193,
                            219,
                            164,
                            26,
                            37,
                            20,
                            174,
                            99,
                            101,
                            79,
                            89,
                            239,
                            94,
                            231,
                            109,
                            78,
                            104,
                            52,
                            177,
                            185,
                            148,
                            126,
                            79,
                            70,
                            158,
                        ],
                    },
                ),
            ),
        },
        XvcEntity(
            12,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            229,
                            39,
                            223,
                            204,
                            69,
                            223,
                            228,
                            26,
                            206,
                            105,
                            2,
                            247,
                            154,
                            187,
                            130,
                            84,
                            71,
                            233,
                            236,
                            254,
                            187,
                            72,
                            28,
                            55,
                            11,
                            15,
                            2,
                            249,
                            168,
                            77,
                            113,
                            220,
                        ],
                    },
                ),
            ),
        },
    },
}
[TRACE][file/src/common/mod.rs:508] records.len(): 10
[TRACE][file/src/common/mod.rs:510] new_store.len(): 15
[TRACE][file/src/common/mod.rs:508] records.len(): 10
[TRACE][file/src/common/mod.rs:510] new_store.len(): 15
[TRACE][file/src/common/mod.rs:508] records.len(): 10
[TRACE][file/src/common/mod.rs:510] new_store.len(): 15
[TRACE][file/src/common/mod.rs:508] records.len(): 10
[TRACE][file/src/common/mod.rs:510] new_store.len(): 15
[TRACE][file/src/common/mod.rs:508] records.len(): 10
[TRACE][file/src/common/mod.rs:510] new_store.len(): 15
[TRACE][file/src/track/mod.rs:198] current_xvc_metadata_store.len(): 15
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 1 regexes
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 1 regexes
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 1 regexes
[TRACE][file/src/track/mod.rs:420] dir_map: {}
[TRACE][file/src/track/mod.rs:455] file_map: {
    "[CWD]/dir-0003/file-0004.bin": "[CWD]/dir-0003/.gitignore",
    "[CWD]/dir-0003/file-0001.bin": "[CWD]/dir-0003/.gitignore",
    "[CWD]/dir-0003/file-0002.bin": "[CWD]/dir-0003/.gitignore",
    "[CWD]/dir-0003/file-0003.bin": "[CWD]/dir-0003/.gitignore",
    "[CWD]/dir-0003/file-0005.bin": "[CWD]/dir-0003/.gitignore",
}
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/f98/51a/db195362c1dba41a2514ae63654f59ef5ee76d4e6834b1b9947e4f469e"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/912/afa/81435387a3dbaf3ec593b06a8c1f967dddcc885acee1d35b070a851e90"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/9eb/988/999636bffcb10c7bd7a146abd2157a38d2708116ef0c9a73722502d2e8"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/dac/815/eae4e1bdc0275ca8058b94ab6176ebe1011a7dd961a7499347ab7f84cc"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/e52/7df/cc45dfe41ace6902f79abb825447e9ecfebb481c370b0f02f9a84d71dc"
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0003/file-0005.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/f98/51a/db195362c1dba41a2514ae63654f59ef5ee76d4e6834b1b9947e4f469e/0.bin",
)
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0003/file-0001.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/9eb/988/999636bffcb10c7bd7a146abd2157a38d2708116ef0c9a73722502d2e8/0.bin",
)
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0003/file-0004.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/912/afa/81435387a3dbaf3ec593b06a8c1f967dddcc885acee1d35b070a851e90/0.bin",
)
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0003/file-0005.bin -> b3/f98/51a/db195362c1dba41a2514ae63654f59ef5ee76d4e6834b1b9947e4f469e/0.bin
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0003/file-0005.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Symlink
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0003/file-0003.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/e52/7df/cc45dfe41ace6902f79abb825447e9ecfebb481c370b0f02f9a84d71dc/0.bin",
)
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0003/file-0002.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/dac/815/eae4e1bdc0275ca8058b94ab6176ebe1011a7dd961a7499347ab7f84cc/0.bin",
)
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0003/file-0001.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Symlink
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0003/file-0004.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Symlink
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0003/file-0003.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Symlink
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0003/file-0002.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Symlink
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0003/file-0001.bin -> b3/9eb/988/999636bffcb10c7bd7a146abd2157a38d2708116ef0c9a73722502d2e8/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0003/file-0004.bin -> b3/912/afa/81435387a3dbaf3ec593b06a8c1f967dddcc885acee1d35b070a851e90/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SYMLINK] [CWD]/.xvc/b3/f98/51a/db195362c1dba41a2514ae63654f59ef5ee76d4e6834b1b9947e4f469e/0.bin -> [CWD]/dir-0003/file-0005.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/f98/51a/db195362c1dba41a2514ae63654f59ef5ee76d4e6834b1b9947e4f469e/0.bin -> dir-0003/file-0005.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0003/file-0003.bin -> b3/e52/7df/cc45dfe41ace6902f79abb825447e9ecfebb481c370b0f02f9a84d71dc/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SYMLINK] [CWD]/.xvc/b3/9eb/988/999636bffcb10c7bd7a146abd2157a38d2708116ef0c9a73722502d2e8/0.bin -> [CWD]/dir-0003/file-0001.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/9eb/988/999636bffcb10c7bd7a146abd2157a38d2708116ef0c9a73722502d2e8/0.bin -> dir-0003/file-0001.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0003/file-0002.bin -> b3/dac/815/eae4e1bdc0275ca8058b94ab6176ebe1011a7dd961a7499347ab7f84cc/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SYMLINK] [CWD]/.xvc/b3/912/afa/81435387a3dbaf3ec593b06a8c1f967dddcc885acee1d35b070a851e90/0.bin -> [CWD]/dir-0003/file-0004.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/912/afa/81435387a3dbaf3ec593b06a8c1f967dddcc885acee1d35b070a851e90/0.bin -> dir-0003/file-0004.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SYMLINK] [CWD]/.xvc/b3/e52/7df/cc45dfe41ace6902f79abb825447e9ecfebb481c370b0f02f9a84d71dc/0.bin -> [CWD]/dir-0003/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/e52/7df/cc45dfe41ace6902f79abb825447e9ecfebb481c370b0f02f9a84d71dc/0.bin -> dir-0003/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SYMLINK] [CWD]/.xvc/b3/dac/815/eae4e1bdc0275ca8058b94ab6176ebe1011a7dd961a7499347ab7f84cc/0.bin -> [CWD]/dir-0003/file-0002.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/dac/815/eae4e1bdc0275ca8058b94ab6176ebe1011a7dd961a7499347ab7f84cc/0.bin -> dir-0003/file-0002.bin
[TRACE][lib/src/cli/mod.rs:294] "Before handle_git_automation": "Before handle_git_automation"
[DEBUG][lib/src/cli/mod.rs:491] Using Git: /opt/homebrew/bin/git
[TRACE][lib/src/cli/mod.rs:398] args: [
    "-C",
    "[CWD]",
    "diff",
    "--name-only",
    "--cached",
]
[TRACE][lib/src/cli/mod.rs:424] git_diff_staged_out: ""
[TRACE][lib/src/cli/mod.rs:398] args: [
    "-C",
    "[CWD]",
    "add",
    "[CWD]/.xvc",
    "*.gitignore",
    "*.xvcignore",
]
[DEBUG][lib/src/cli/mod.rs:506] Adding .xvc/ to git: 
[TRACE][lib/src/cli/mod.rs:398] args: [
    "-C",
    "[CWD]",
    "commit",
    "-m",
    "Xvc auto-commit after /'/'",
]
[DEBUG][lib/src/cli/mod.rs:516] Committing .xvc/ to git: [main 865a2d3] Xvc auto-commit after ''
 7 files changed, 12 insertions(+)
 create mode 100644 .xvc/ec/1672803172645707
 create mode 100644 .xvc/store/cache-type-store/1672803172624387.json
 create mode 100644 .xvc/store/content-digest-store/1672803172624833.json
 create mode 100644 .xvc/store/file-text-or-binary-store/1672803172624573.json
 create mode 100644 .xvc/store/xvc-metadata-store/1672803172624127.json
 create mode 100644 .xvc/store/xvc-path-store/1672803172623791.json
 create mode 100644 dir-0003/.gitignore


$ xvc file list
FX        1005 2023-01-04 03:32:50   dir-0005/file-0005.bin           487ebc55
FX        1004 2023-01-04 03:32:50   dir-0005/file-0004.bin           853fe74b
FX        1003 2023-01-04 03:32:50   dir-0005/file-0003.bin           0030d630
FX        1002 2023-01-04 03:32:50   dir-0005/file-0002.bin           d5029243
FX        1001 2023-01-04 03:32:50   dir-0005/file-0001.bin           367a7c54
DX         224 2023-01-04 03:32:50   dir-0005                   
FX        1005 2023-01-04 03:32:50   dir-0004/file-0005.bin           2a5eef53
FX        1004 2023-01-04 03:32:50   dir-0004/file-0004.bin           1f84f6e2
FX        1003 2023-01-04 03:32:50   dir-0004/file-0003.bin           96115170
FX        1002 2023-01-04 03:32:50   dir-0004/file-0002.bin           4f6b7c9f
FX        1001 2023-01-04 03:32:50   dir-0004/file-0001.bin           0404b2b0
DX         224 2023-01-04 03:32:50   dir-0004                   
SS         180 2023-01-04 03:32:52   dir-0003/file-0005.bin  f9851adb         
SS         180 2023-01-04 03:32:52   dir-0003/file-0004.bin  912afa81         
SS         180 2023-01-04 03:32:52   dir-0003/file-0003.bin  e527dfcc         
SS         180 2023-01-04 03:32:52   dir-0003/file-0002.bin  dac815ea         
SS         180 2023-01-04 03:32:52   dir-0003/file-0001.bin  9eb98899         
FX         149 2023-01-04 03:32:52   dir-0003/.gitignore           57788f76
DX         256 2023-01-04 03:32:52   dir-0003                   
FH        1005 2023-01-04 03:32:50   dir-0002/file-0005.bin  bb45e512 bb45e512
FH        1004 2023-01-04 03:32:50   dir-0002/file-0004.bin  0aab47aa 0aab47aa
FH        1003 2023-01-04 03:32:50   dir-0002/file-0003.bin  61982600 61982600
FH        1002 2023-01-04 03:32:50   dir-0002/file-0002.bin  1653a440 1653a440
FH        1001 2023-01-04 03:32:50   dir-0002/file-0001.bin  1f7dc765 1f7dc765
FX         149 2023-01-04 03:32:52   dir-0002/.gitignore           2ee28ca0
DX         256 2023-01-04 03:32:52   dir-0002                   
FC        1005 2023-01-04 03:32:50   dir-0001/file-0005.bin  ab676cbf ab676cbf
FC        1004 2023-01-04 03:32:50   dir-0001/file-0004.bin  efefad08 efefad08
FC        1003 2023-01-04 03:32:50   dir-0001/file-0003.bin  345cabd6 345cabd6
FC        1002 2023-01-04 03:32:50   dir-0001/file-0002.bin  141d9f90 141d9f90
FC        1001 2023-01-04 03:32:50   dir-0001/file-0001.bin  772f8dcd 772f8dcd
FX         149 2023-01-04 03:32:51   dir-0001/.gitignore           bb09cedb
DX         256 2023-01-04 03:32:51   dir-0001                   
FX         130 2023-01-04 03:32:50   .xvcignore           ac46bf74
FX         107 2023-01-04 03:32:50   .gitignore           ce9fcf30
Total #: 35 Workspace Size:       22860 Cached Size:       15045


```

Although not all filesystems support, `R` represents reflinks. 

### Sort options

You may sort `xvc file list` output by name, by modification time and by file
size. 