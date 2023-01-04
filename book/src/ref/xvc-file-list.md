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
FX        1005 2023-01-04 04:26:06   dir-0005/file-0005.bin           8c765363
FX        1004 2023-01-04 04:26:06   dir-0005/file-0004.bin           8242cf46
FX        1003 2023-01-04 04:26:06   dir-0005/file-0003.bin           476f3e9c
FX        1002 2023-01-04 04:26:06   dir-0005/file-0002.bin           ed9f4d58
FX        1001 2023-01-04 04:26:06   dir-0005/file-0001.bin           7662c7bf
DX         224 2023-01-04 04:26:06   dir-0005                   
FX        1005 2023-01-04 04:26:06   dir-0004/file-0005.bin           2de166cf
FX        1004 2023-01-04 04:26:06   dir-0004/file-0004.bin           7897b573
FX        1003 2023-01-04 04:26:06   dir-0004/file-0003.bin           c0c58c3c
FX        1002 2023-01-04 04:26:06   dir-0004/file-0002.bin           d1884c50
FX        1001 2023-01-04 04:26:06   dir-0004/file-0001.bin           449c28f8
DX         224 2023-01-04 04:26:06   dir-0004                   
FX        1005 2023-01-04 04:26:06   dir-0003/file-0005.bin           278b9d08
FX        1004 2023-01-04 04:26:06   dir-0003/file-0004.bin           852335b0
FX        1003 2023-01-04 04:26:06   dir-0003/file-0003.bin           9fd3b556
FX        1002 2023-01-04 04:26:06   dir-0003/file-0002.bin           fe5e94d0
FX        1001 2023-01-04 04:26:06   dir-0003/file-0001.bin           511dcccf
DX         224 2023-01-04 04:26:06   dir-0003                   
FX        1005 2023-01-04 04:26:06   dir-0002/file-0005.bin           3c977657
FX        1004 2023-01-04 04:26:06   dir-0002/file-0004.bin           d828e889
FX        1003 2023-01-04 04:26:06   dir-0002/file-0003.bin           331842a4
FX        1002 2023-01-04 04:26:06   dir-0002/file-0002.bin           1a373487
FX        1001 2023-01-04 04:26:06   dir-0002/file-0001.bin           e356a062
DX         224 2023-01-04 04:26:06   dir-0002                   
FX        1005 2023-01-04 04:26:06   dir-0001/file-0005.bin           6b659b1c
FX        1004 2023-01-04 04:26:06   dir-0001/file-0004.bin           a6f00fd5
FX        1003 2023-01-04 04:26:06   dir-0001/file-0003.bin           97f28c8f
FX        1002 2023-01-04 04:26:06   dir-0001/file-0002.bin           5d40d4ef
FX        1001 2023-01-04 04:26:06   dir-0001/file-0001.bin           6ea545b5
DX         224 2023-01-04 04:26:06   dir-0001                   
FX         130 2023-01-04 04:26:06   .xvcignore           ac46bf74
FX         107 2023-01-04 04:26:06   .gitignore           ce9fcf30
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

$ xvc -vvvv file list dir-0001/
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
    "core.verbosity": String(
        "debug",
    ),
    "core.quiet": Boolean(
        false,
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
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "cache.type": String(
                    "copy",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "core.guid": String(
                    "6bab4e1c9c344391",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
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
                "core.verbosity": String(
                    "error",
                ),
                "file.track.text_or_binary": String(
                    "auto",
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
                "cache.type": String(
                    "copy",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "core.guid": String(
                    "c4faeba569e76d56",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
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
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
            ),
        },
        "git.auto_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "cache.type": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
            ),
        },
        "pipeline.current_pipeline": XvcConfigValue {
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
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
            ),
        },
        "file.track.force": XvcConfigValue {
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
        "file.track.no_commit": XvcConfigValue {
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
        "file.carry-in.force": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "c4faeba569e76d56",
            ),
        },
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
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
        "file.list.recursive": XvcConfigValue {
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
        default_configuration: "/n[core]/n# The repository id. Please do not delete or change it. /n# This is used to identify the repository and generate paths in storages. /n# In the future it may be used to in other ways. /nguid = /"6bab4e1c9c344391/"/n# Default verbosity level. /n# One of /"error/", /"warn/", /"info/"/nverbosity = /"error/"/n/n[git]/n# Automate git operations. /n# Turning this off leads Xvc to behave as if it's not in a Git repository./n# Not recommended unless you're really not using Git/nuse_git = true/n# Command to run Git process./n# You can set this to an absolute path to specify an executable/n# If set to a non-absolute path, the executable will be searched in $PATH./ncommand = /"git/"/n/n# Commit changes in .xvc/ directory after commands./n# You can set this to false if you want to commit manually. /nauto_commit = true/n/n# Stage changes in .xvc/ directory without committing./n# auto_commit implies auto_stage. /n# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true. /nauto_stage = false/n/n[cache]/n# The cache type for XVC. It may take copy, hardlink, symlink, reflink as values./n# The default is copy to make sure the options is portable./n# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file./n# Note that hardlink and symlink are read-only as they link the files in cache. /ntype = /"copy/"/n# The hash algorithm used for the cache. /n# It may take blake3, blake2, sha2 or sha3 as values. /n# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc./n# The cache path is produced by prepending algorithm name to the cache. /n# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc. /nalgorithm = /"blake3/"/n/n[file]/n/n[file.track]/n/n# Don't move file content to cache after xvc file track/nno_commit = false/n# Force to track files even if they are already tracked./nforce = false/n/n# Xvc calculates file content digest differently for text and binary files./n# This option controls whether to treat files as text or binary./n# It may take auto, text or binary as values./n# Auto check each file individually and treat it as text if it's text./ntext_or_binary = /"auto/"/n/n# Don't use parallelism in track operations. /n# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations./nno_parallel = false/n/n[file.list]/n/n# Format for `xvc file list` rows. You can reorder or remove columns./n# The following are the keys for each row: /n# - {acd64}:  actual content digest. All 64 digits from the workspace file's content./n# - {acd8}:  actual content digest. First 8 digits the file content digest. /n# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),/n#   symlink (S), hardlink (H) or reflink (R). /n# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,/n#   GB and TB to represent sizes larger than 1MB. /n# - {ats}:  actual timestamp. The timestamp of the workspace file./n# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show/n#   whether the file timestamp is the same as the cached timestamp, newer,/n#   older, not cached or not tracked./n# - {name}: The name of the file or directory./n# - {rcd64}:  recorded content digest. All 64 digits./n# - {rcd8}:  recorded content digest. First 8 digits./n# - {rct}:  recorded cache type. Whether the entry is linked to the workspace/n#   as a copy (C), symlink (S), hardlink (H) or reflink (R)./n# - {rsz}:  recorded size. The size of the cached content in bytes. It uses/n#   MB, GB and TB to represent sizes larged than 1MB./n# - {rts}:  recorded timestamp. The timestamp of the cached content./n# /n# There are no escape sequences in the format string. /n# If you want to add a tab, type it to the string./n# If you want to add a literal double curly brace, open an issue. /nformat = /"{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}/"/n/n# Default sort order for `xvc file list`./n# Valid values are/n# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc./nsort = /"name-desc/"/n/n# Do not show a summary for as the final row for `xvc file list`./nno_summary = false/n/n# List files recursively always./nrecursive = false/n/n[file.carry-in]/n# Carry-in the files to cache always, even if they are already present./nforce = false/n/n# Don't use parallel move/copy in carry-in/nno_parallel = false/n/n[pipeline]/n# Name of the current pipeline to run/ncurrent_pipeline = /"default/"/n# Name of the default pipeline/ndefault = /"default/"/n# Name of the default params file name/ndefault_params_file = /"params.yaml/"/n/n",
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
    "[CWD]/.xvc/ec/1672806366902986",
    "[CWD]/.xvc/ec/1672806366905837",
    "[CWD]/.xvc/ec/1672806367191511",
    "[CWD]/.xvc/ec/1672806367486636",
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
    subcommand: List(
        ListCLI {
            format: None,
            sort_criteria: None,
            no_summary: false,
            targets: Some(
                [
                    "dir-0001/",
                ],
            ),
        },
    ),
}
[TRACE][file/src/common/mod.rs:275] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][file/src/common/mod.rs:276] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs:292] all_paths: {
    XvcPath(
        "dir-0005/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672806366,
                tv_nsec: 774416879,
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
                tv_sec: 1672806366,
                tv_nsec: 772345481,
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
                tv_sec: 1672806367,
                tv_nsec: 485446281,
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
                tv_sec: 1672806366,
                tv_nsec: 771021008,
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
                tv_sec: 1672806366,
                tv_nsec: 772607150,
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
                tv_sec: 1672806367,
                tv_nsec: 486282040,
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
                tv_sec: 1672806366,
                tv_nsec: 774227001,
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
                tv_sec: 1672806366,
                tv_nsec: 903135294,
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
                tv_sec: 1672806366,
                tv_nsec: 769871245,
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
                tv_sec: 1672806366,
                tv_nsec: 770237124,
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
                tv_sec: 1672806366,
                tv_nsec: 774272252,
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
                tv_sec: 1672806366,
                tv_nsec: 771393804,
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
                tv_sec: 1672806366,
                tv_nsec: 903200295,
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
                tv_sec: 1672806366,
                tv_nsec: 771999935,
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
                tv_sec: 1672806366,
                tv_nsec: 770638254,
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
                tv_sec: 1672806366,
                tv_nsec: 773830539,
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
                tv_sec: 1672806366,
                tv_nsec: 773636537,
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
                tv_sec: 1672806366,
                tv_nsec: 771626181,
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
                tv_sec: 1672806366,
                tv_nsec: 771208593,
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
                tv_sec: 1672806366,
                tv_nsec: 773196574,
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
                tv_sec: 1672806366,
                tv_nsec: 770048914,
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
                tv_sec: 1672806366,
                tv_nsec: 773391451,
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
                tv_sec: 1672806366,
                tv_nsec: 771815267,
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
                tv_sec: 1672806366,
                tv_nsec: 769649576,
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
                tv_sec: 1672806366,
                tv_nsec: 771249844,
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
                tv_sec: 1672806366,
                tv_nsec: 770830256,
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
                tv_sec: 1672806366,
                tv_nsec: 773243532,
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
                tv_sec: 1672806366,
                tv_nsec: 772195479,
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
                tv_sec: 1672806366,
                tv_nsec: 774023666,
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
                tv_sec: 1672806366,
                tv_nsec: 770414251,
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
                tv_sec: 1672806366,
                tv_nsec: 772799236,
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
                tv_sec: 1672806366,
                tv_nsec: 772155312,
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
                tv_sec: 1672806366,
                tv_nsec: 773000530,
            },
        ),
    },
}
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 1 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs:314] glob_matcher: GlobSet {
    len: 1,
    strats: [
        Extension(
            ExtensionStrategy(
                {},
            ),
        ),
        BasenameLiteral(
            BasenameLiteralStrategy(
                {},
            ),
        ),
        Literal(
            LiteralStrategy(
                {},
            ),
        ),
        Suffix(
            SuffixStrategy {
                matcher: AhoCorasick {
                    imp: DFA(
                        PremultipliedByteClass(
                            PremultipliedByteClass(
                                Repr {
                                    match_kind: Standard,
                                    anchored: false,
                                    premultiplied: true,
                                    start_id: 2,
                                    max_pattern_len: 0,
                                    pattern_count: 0,
                                    state_count: 3,
                                    max_match: 1,
                                    heap_bytes: 96,
                                    prefilter: None,
                                    byte_classes: ByteClasses( 0 => [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255]),
                                    trans: [
                                        2,
                                        1,
                                        2,
                                    ],
                                    matches: [
                                        [],
                                        [],
                                        [],
                                    ],
                                },
                            ),
                        ),
                    ),
                    match_kind: Standard,
                },
                map: [],
                longest: 0,
            },
        ),
        Prefix(
            PrefixStrategy {
                matcher: AhoCorasick {
                    imp: DFA(
                        PremultipliedByteClass(
                            PremultipliedByteClass(
                                Repr {
                                    match_kind: Standard,
                                    anchored: false,
                                    premultiplied: true,
                                    start_id: 143,
                                    max_pattern_len: 9,
                                    pattern_count: 1,
                                    state_count: 12,
                                    max_match: 26,
                                    heap_bytes: 1552,
                                    prefilter: Some(
                                        PrefilterObj(
                                            StartBytesOne {
                                                byte1: 100,
                                            },
                                        ),
                                    ),
                                    byte_classes: ByteClasses( 0 => [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44] 1 => [45] 2 => [46] 3 => [47] 4 => [48] 5 => [49] 6 => [50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99] 7 => [100] 8 => [101, 102, 103, 104] 9 => [105] 10 => [106, 107, 108, 109, 110, 111, 112, 113] 11 => [114] 12 => [115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255]),
                                    trans: [
                                        11,
                                        11,
                                        11,
                                        11,
                                        11,
                                        11,
                                        11,
                                        3,
                                        11,
                                        11,
                                        11,
                                        11,
                                        11,
                                        1,
                                        1,
                                        1,
                                        1,
                                        1,
                                        1,
                                        1,
                                        1,
                                        1,
                                        1,
                                        1,
                                        1,
                                        1,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        39,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        39,
                                        143,
                                        52,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        39,
                                        143,
                                        143,
                                        143,
                                        65,
                                        143,
                                        143,
                                        78,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        39,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        91,
                                        143,
                                        143,
                                        39,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        104,
                                        143,
                                        143,
                                        39,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        117,
                                        143,
                                        143,
                                        39,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        130,
                                        143,
                                        39,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        26,
                                        143,
                                        143,
                                        143,
                                        39,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                        39,
                                        143,
                                        143,
                                        143,
                                        143,
                                        143,
                                    ],
                                    matches: [
                                        [],
                                        [],
                                        [
                                            (
                                                0,
                                                9,
                                            ),
                                        ],
                                        [],
                                        [],
                                        [],
                                        [],
                                        [],
                                        [],
                                        [],
                                        [],
                                        [],
                                    ],
                                },
                            ),
                        ),
                    ),
                    match_kind: Standard,
                },
                map: [
                    0,
                ],
                longest: 9,
            },
        ),
        RequiredExtension(
            RequiredExtensionStrategy(
                {},
            ),
        ),
        Regex(
            RegexSetStrategy {
                matcher: RegexSet([]),
                map: [],
            },
        ),
    ],
}
[TRACE][file/src/list/mod.rs:543] from_disk: {
    XvcPath(
        "dir-0001/.gitignore",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            149,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672806367,
                tv_nsec: 485446281,
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
                tv_sec: 1672806366,
                tv_nsec: 769649576,
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
                tv_sec: 1672806366,
                tv_nsec: 770414251,
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
                tv_sec: 1672806366,
                tv_nsec: 770237124,
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
                tv_sec: 1672806366,
                tv_nsec: 769871245,
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
                tv_sec: 1672806366,
                tv_nsec: 770048914,
            },
        ),
    },
}
[TRACE][file/src/common/mod.rs:221] targets: Some(
    [
        "dir-0001/",
    ],
)
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 1 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs:234] paths: HStore {
    map: {},
}
[TRACE][file/src/common/mod.rs:236] metadata: HStore {
    map: {},
}
[TRACE][file/src/common/mod.rs:239] dir_md: HStore {
    map: {},
}
[TRACE][file/src/common/mod.rs:241] dir_paths: HStore {
    map: {},
}
[TRACE][file/src/common/mod.rs:248] paths: HStore {
    map: {},
}
[TRACE][file/src/list/mod.rs:545] from_store: HStore {
    map: {},
}
[TRACE][file/src/list/mod.rs:624] matches: [
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0001/.gitignore",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    149,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1672806367,
                        tv_nsec: 485446281,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_cache_type: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0001/file-0001.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    1001,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1672806366,
                        tv_nsec: 769649576,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_cache_type: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0001/file-0005.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    1005,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1672806366,
                        tv_nsec: 770414251,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_cache_type: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0001/file-0004.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    1004,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1672806366,
                        tv_nsec: 770237124,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_cache_type: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0001/file-0002.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    1002,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1672806366,
                        tv_nsec: 769871245,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_cache_type: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0001/file-0003.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    1003,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1672806366,
                        tv_nsec: 770048914,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_cache_type: None,
    },
]
[TRACE][file/src/list/mod.rs:207] &path_prefix: ""
[TRACE][file/src/list/mod.rs:209] ap: XvcPath(
    "dir-0001/.gitignore",
)
[TRACE][file/src/list/mod.rs:207] &path_prefix: ""
[TRACE][file/src/list/mod.rs:209] ap: XvcPath(
    "dir-0001/file-0001.bin",
)
[TRACE][file/src/list/mod.rs:207] &path_prefix: ""
[TRACE][file/src/list/mod.rs:209] ap: XvcPath(
    "dir-0001/file-0005.bin",
)
[TRACE][file/src/list/mod.rs:207] &path_prefix: ""
[TRACE][file/src/list/mod.rs:209] ap: XvcPath(
    "dir-0001/file-0004.bin",
)
[TRACE][file/src/list/mod.rs:207] &path_prefix: ""
[TRACE][file/src/list/mod.rs:209] ap: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][file/src/list/mod.rs:207] &path_prefix: ""
[TRACE][file/src/list/mod.rs:209] ap: XvcPath(
    "dir-0001/file-0003.bin",
)
FX        1005 2023-01-04 04:26:06   dir-0001/file-0005.bin           6b659b1c
FX        1004 2023-01-04 04:26:06   dir-0001/file-0004.bin           a6f00fd5
FX        1003 2023-01-04 04:26:06   dir-0001/file-0003.bin           97f28c8f
FX        1002 2023-01-04 04:26:06   dir-0001/file-0002.bin           5d40d4ef
FX        1001 2023-01-04 04:26:06   dir-0001/file-0001.bin           6ea545b5
FX         149 2023-01-04 04:26:07   dir-0001/.gitignore           38fa3174
Total #: 6 Workspace Size:        5164 Cached Size:           0
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
[DEBUG][lib/src/cli/mod.rs:516] Committing .xvc/ to git: [main 6bd23b2] Xvc auto-commit after ''
 1 file changed, 1 insertion(+)
 create mode 100644 .xvc/ec/1672806367782662


```

If you add another set of files as hardlinks to the cached copies, it will
print the second letter as `H`.

```console
$ xvc file track dir-0002 --cache-type hardlink

$ xvc file list dir-0002
FX        1005 2023-01-04 04:26:06   dir-0002/file-0005.bin           3c977657
FX        1004 2023-01-04 04:26:06   dir-0002/file-0004.bin           d828e889
FX        1003 2023-01-04 04:26:06   dir-0002/file-0003.bin           331842a4
FX        1002 2023-01-04 04:26:06   dir-0002/file-0002.bin           1a373487
FX        1001 2023-01-04 04:26:06   dir-0002/file-0001.bin           e356a062
FX         149 2023-01-04 04:26:08   dir-0002/.gitignore           3e4f64a0
Total #: 6 Workspace Size:        5164 Cached Size:           0


```

Note, as hardlinks are actually files with the same inode in the file system
with alternative paths, they are detected as `F`. 

Symbolic links are typically reported as `SS` in the first letters. 
It means they are symbolic links on the file system and their cache type is also
symbolic links. 

```console
$ xvc file track dir-0003 --cache-type symlink

$ xvc file list dir-0003
SX         180 [..]   dir-0003/file-0005.bin                   
SX         180 [..]   dir-0003/file-0004.bin                   
SX         180 [..]   dir-0003/file-0003.bin                   
SX         180 [..]   dir-0003/file-0002.bin                   
SX         180 [..]   dir-0003/file-0001.bin                   
FX         149 2023-01-04 04:26:08   dir-0003/.gitignore           68e6f7c7
Total #: 6 Workspace Size:        1049 Cached Size:           0


```

Although not all filesystems support, `R` represents reflinks. 

### Sort options

You may sort `xvc file list` output by name, by modification time and by file
size. 