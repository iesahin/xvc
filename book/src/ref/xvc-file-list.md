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
FX        1005 2023-01-04 04:11:13   dir-0005/file-0005.bin           d3ebabb4
FX        1004 2023-01-04 04:11:13   dir-0005/file-0004.bin           0e26a0b5
FX        1003 2023-01-04 04:11:13   dir-0005/file-0003.bin           056f1bf6
FX        1002 2023-01-04 04:11:13   dir-0005/file-0002.bin           b05143ac
FX        1001 2023-01-04 04:11:13   dir-0005/file-0001.bin           c53ab4d1
DX         224 2023-01-04 04:11:13   dir-0005                   
FX        1005 2023-01-04 04:11:13   dir-0004/file-0005.bin           be7abd14
FX        1004 2023-01-04 04:11:13   dir-0004/file-0004.bin           c4e894d5
FX        1003 2023-01-04 04:11:13   dir-0004/file-0003.bin           d089b435
FX        1002 2023-01-04 04:11:13   dir-0004/file-0002.bin           9226dbc7
FX        1001 2023-01-04 04:11:13   dir-0004/file-0001.bin           df6d23df
DX         224 2023-01-04 04:11:13   dir-0004                   
FX        1005 2023-01-04 04:11:13   dir-0003/file-0005.bin           b51e3707
FX        1004 2023-01-04 04:11:13   dir-0003/file-0004.bin           a2ae45c2
FX        1003 2023-01-04 04:11:13   dir-0003/file-0003.bin           f51b3ab1
FX        1002 2023-01-04 04:11:13   dir-0003/file-0002.bin           0d375f73
FX        1001 2023-01-04 04:11:13   dir-0003/file-0001.bin           6b4fd141
DX         224 2023-01-04 04:11:13   dir-0003                   
FX        1005 2023-01-04 04:11:13   dir-0002/file-0005.bin           904fa393
FX        1004 2023-01-04 04:11:13   dir-0002/file-0004.bin           7e1b3fb5
FX        1003 2023-01-04 04:11:13   dir-0002/file-0003.bin           066424b8
FX        1002 2023-01-04 04:11:13   dir-0002/file-0002.bin           af69b20c
FX        1001 2023-01-04 04:11:13   dir-0002/file-0001.bin           96ab56c3
DX         224 2023-01-04 04:11:13   dir-0002                   
FX        1005 2023-01-04 04:11:13   dir-0001/file-0005.bin           ab2b5272
FX        1004 2023-01-04 04:11:13   dir-0001/file-0004.bin           9222852e
FX        1003 2023-01-04 04:11:13   dir-0001/file-0003.bin           ff91985e
FX        1002 2023-01-04 04:11:13   dir-0001/file-0002.bin           2506a16b
FX        1001 2023-01-04 04:11:13   dir-0001/file-0001.bin           21738bac
DX         224 2023-01-04 04:11:13   dir-0001                   
FX         130 2023-01-04 04:11:13   .xvcignore           ac46bf74
FX         107 2023-01-04 04:11:13   .gitignore           ce9fcf30
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
$ xvc -vvvv file track dir-0001/
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
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "cache.type": String(
                    "copy",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "git.command": String(
                    "git",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "core.guid": String(
                    "91a5f5fa37f5ef83",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "core.guid": String(
                    "92d078e73ecbc71f",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "cache.type": String(
                    "copy",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "git.use_git": Boolean(
                    true,
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
                    "debug",
                ),
                "core.quiet": Boolean(
                    false,
                ),
            },
        },
    ],
    the_config: {
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
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
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
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
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
        "cache.type": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
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
                "92d078e73ecbc71f",
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
        default_configuration: "/n[core]/n# The repository id. Please do not delete or change it. /n# This is used to identify the repository and generate paths in storages. /n# In the future it may be used to in other ways. /nguid = /"91a5f5fa37f5ef83/"/n# Default verbosity level. /n# One of /"error/", /"warn/", /"info/"/nverbosity = /"error/"/n/n[git]/n# Automate git operations. /n# Turning this off leads Xvc to behave as if it's not in a Git repository./n# Not recommended unless you're really not using Git/nuse_git = true/n# Command to run Git process./n# You can set this to an absolute path to specify an executable/n# If set to a non-absolute path, the executable will be searched in $PATH./ncommand = /"git/"/n/n# Commit changes in .xvc/ directory after commands./n# You can set this to false if you want to commit manually. /nauto_commit = true/n/n# Stage changes in .xvc/ directory without committing./n# auto_commit implies auto_stage. /n# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true. /nauto_stage = false/n/n[cache]/n# The cache type for XVC. It may take copy, hardlink, symlink, reflink as values./n# The default is copy to make sure the options is portable./n# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file./n# Note that hardlink and symlink are read-only as they link the files in cache. /ntype = /"copy/"/n# The hash algorithm used for the cache. /n# It may take blake3, blake2, sha2 or sha3 as values. /n# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc./n# The cache path is produced by prepending algorithm name to the cache. /n# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc. /nalgorithm = /"blake3/"/n/n[file]/n/n[file.track]/n/n# Don't move file content to cache after xvc file track/nno_commit = false/n# Force to track files even if they are already tracked./nforce = false/n/n# Xvc calculates file content digest differently for text and binary files./n# This option controls whether to treat files as text or binary./n# It may take auto, text or binary as values./n# Auto check each file individually and treat it as text if it's text./ntext_or_binary = /"auto/"/n/n# Don't use parallelism in track operations. /n# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations./nno_parallel = false/n/n[file.list]/n/n# Format for `xvc file list` rows. You can reorder or remove columns./n# The following are the keys for each row: /n# - {acd64}:  actual content digest. All 64 digits from the workspace file's content./n# - {acd8}:  actual content digest. First 8 digits the file content digest. /n# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),/n#   symlink (S), hardlink (H) or reflink (R). /n# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,/n#   GB and TB to represent sizes larger than 1MB. /n# - {ats}:  actual timestamp. The timestamp of the workspace file./n# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show/n#   whether the file timestamp is the same as the cached timestamp, newer,/n#   older, not cached or not tracked./n# - {name}: The name of the file or directory./n# - {rcd64}:  recorded content digest. All 64 digits./n# - {rcd8}:  recorded content digest. First 8 digits./n# - {rct}:  recorded cache type. Whether the entry is linked to the workspace/n#   as a copy (C), symlink (S), hardlink (H) or reflink (R)./n# - {rsz}:  recorded size. The size of the cached content in bytes. It uses/n#   MB, GB and TB to represent sizes larged than 1MB./n# - {rts}:  recorded timestamp. The timestamp of the cached content./n# /n# There are no escape sequences in the format string. /n# If you want to add a tab, type it to the string./n# If you want to add a literal double curly brace, open an issue. /nformat = /"{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}/"/n/n# Default sort order for `xvc file list`./n# Valid values are/n# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc./nsort = /"name-desc/"/n/n# Do not show a summary for as the final row for `xvc file list`./nno_summary = false/n/n# List files recursively always./nrecursive = false/n/n[file.carry-in]/n# Carry-in the files to cache always, even if they are already present./nforce = false/n/n# Don't use parallel move/copy in carry-in/nno_parallel = false/n/n[pipeline]/n# Name of the current pipeline to run/ncurrent_pipeline = /"default/"/n# Name of the default pipeline/ndefault = /"default/"/n# Name of the default params file name/ndefault_params_file = /"params.yaml/"/n/n",
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
    "[CWD]/.xvc/ec/1672805473987094",
    "[CWD]/.xvc/ec/1672805473989971",
    "[CWD]/.xvc/ec/1672805474274848",
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
            cache_type: None,
            no_commit: false,
            text_or_binary: None,
            force: false,
            no_parallel: false,
            targets: Some(
                [
                    "dir-0001/",
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
        "dir-0005",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            224,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672805473,
                tv_nsec: 863333971,
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
                tv_sec: 1672805473,
                tv_nsec: 863021176,
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
                tv_sec: 1672805473,
                tv_nsec: 858566502,
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
                tv_sec: 1672805473,
                tv_nsec: 862036915,
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
                tv_sec: 1672805473,
                tv_nsec: 861701370,
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
                tv_sec: 1672805473,
                tv_nsec: 858018412,
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
                tv_sec: 1672805473,
                tv_nsec: 860531690,
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
                tv_sec: 1672805473,
                tv_nsec: 987301584,
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
                tv_sec: 1672805473,
                tv_nsec: 860000684,
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
                tv_sec: 1672805473,
                tv_nsec: 987228334,
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
                tv_sec: 1672805473,
                tv_nsec: 860605524,
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
                tv_sec: 1672805473,
                tv_nsec: 863268304,
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
                tv_sec: 1672805473,
                tv_nsec: 860286521,
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
                tv_sec: 1672805473,
                tv_nsec: 856958318,
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
                tv_sec: 1672805473,
                tv_nsec: 862552462,
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
                tv_sec: 1672805473,
                tv_nsec: 858838213,
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
                tv_sec: 1672805473,
                tv_nsec: 859717098,
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
                tv_sec: 1672805473,
                tv_nsec: 862249084,
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
                tv_sec: 1672805473,
                tv_nsec: 862783048,
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
                tv_sec: 1672805473,
                tv_nsec: 861979956,
            },
        ),
    },
    XvcPath(
        "dir-0001",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            224,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672805473,
                tv_nsec: 857837452,
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
                tv_sec: 1672805473,
                tv_nsec: 863524890,
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
                tv_sec: 1672805473,
                tv_nsec: 859103591,
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
                tv_sec: 1672805473,
                tv_nsec: 859158592,
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
                tv_sec: 1672805473,
                tv_nsec: 857780202,
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
                tv_sec: 1672805473,
                tv_nsec: 859377302,
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
                tv_sec: 1672805473,
                tv_nsec: 858310999,
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
                tv_sec: 1672805473,
                tv_nsec: 861439659,
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
                tv_sec: 1672805473,
                tv_nsec: 860819152,
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
                tv_sec: 1672805473,
                tv_nsec: 857526824,
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
                tv_sec: 1672805473,
                tv_nsec: 861163530,
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
                tv_sec: 1672805473,
                tv_nsec: 857263571,
            },
        ),
    },
}
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 1 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs:307] glob_matcher: GlobSet {
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
[TRACE][file/src/track/mod.rs:125] targets: {
    XvcPath(
        "dir-0001/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672805473,
                tv_nsec: 856958318,
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
                tv_sec: 1672805473,
                tv_nsec: 857526824,
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
                tv_sec: 1672805473,
                tv_nsec: 857780202,
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
                tv_sec: 1672805473,
                tv_nsec: 858018412,
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
                tv_sec: 1672805473,
                tv_nsec: 857263571,
            },
        ),
    },
}
[TRACE][file/src/common/compare.rs:77] pmm: {
    XvcPath(
        "dir-0001/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672805473,
                tv_nsec: 856958318,
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
                tv_sec: 1672805473,
                tv_nsec: 857526824,
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
                tv_sec: 1672805473,
                tv_nsec: 857780202,
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
                tv_sec: 1672805473,
                tv_nsec: 858018412,
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
                tv_sec: 1672805473,
                tv_nsec: 857263571,
            },
        ),
    },
}
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0001/file-0001.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    2,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    3,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0001/file-0004.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    4,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0001/file-0005.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    5,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    6,
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0002.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0003.bin",
    ),
}
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0004.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0001/file-0004.bin",
)
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0001/file-0004.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0001/file-0003.bin",
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                37,
                6,
                161,
                107,
                233,
                11,
                249,
                105,
                12,
                226,
                153,
                200,
                23,
                223,
                161,
                157,
                74,
                152,
                182,
                39,
                203,
                116,
                238,
                32,
                177,
                241,
                212,
                20,
                241,
                221,
                249,
                97,
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
                37,
                6,
                161,
                107,
                233,
                11,
                249,
                105,
                12,
                226,
                153,
                200,
                23,
                223,
                161,
                157,
                74,
                152,
                182,
                39,
                203,
                116,
                238,
                32,
                177,
                241,
                212,
                20,
                241,
                221,
                249,
                97,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                146,
                34,
                133,
                46,
                249,
                220,
                88,
                76,
                183,
                72,
                44,
                11,
                232,
                214,
                212,
                177,
                76,
                25,
                197,
                87,
                24,
                0,
                245,
                209,
                218,
                68,
                232,
                177,
                54,
                162,
                58,
                80,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                255,
                145,
                152,
                94,
                90,
                220,
                226,
                165,
                155,
                54,
                177,
                253,
                133,
                239,
                2,
                145,
                178,
                88,
                236,
                20,
                200,
                161,
                189,
                61,
                234,
                164,
                10,
                156,
                225,
                28,
                14,
                182,
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
                255,
                145,
                152,
                94,
                90,
                220,
                226,
                165,
                155,
                54,
                177,
                253,
                133,
                239,
                2,
                145,
                178,
                88,
                236,
                20,
                200,
                161,
                189,
                61,
                234,
                164,
                10,
                156,
                225,
                28,
                14,
                182,
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
                    37,
                    6,
                    161,
                    107,
                    233,
                    11,
                    249,
                    105,
                    12,
                    226,
                    153,
                    200,
                    23,
                    223,
                    161,
                    157,
                    74,
                    152,
                    182,
                    39,
                    203,
                    116,
                    238,
                    32,
                    177,
                    241,
                    212,
                    20,
                    241,
                    221,
                    249,
                    97,
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
                146,
                34,
                133,
                46,
                249,
                220,
                88,
                76,
                183,
                72,
                44,
                11,
                232,
                214,
                212,
                177,
                76,
                25,
                197,
                87,
                24,
                0,
                245,
                209,
                218,
                68,
                232,
                177,
                54,
                162,
                58,
                80,
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
                    255,
                    145,
                    152,
                    94,
                    90,
                    220,
                    226,
                    165,
                    155,
                    54,
                    177,
                    253,
                    133,
                    239,
                    2,
                    145,
                    178,
                    88,
                    236,
                    20,
                    200,
                    161,
                    189,
                    61,
                    234,
                    164,
                    10,
                    156,
                    225,
                    28,
                    14,
                    182,
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
                    146,
                    34,
                    133,
                    46,
                    249,
                    220,
                    88,
                    76,
                    183,
                    72,
                    44,
                    11,
                    232,
                    214,
                    212,
                    177,
                    76,
                    25,
                    197,
                    87,
                    24,
                    0,
                    245,
                    209,
                    218,
                    68,
                    232,
                    177,
                    54,
                    162,
                    58,
                    80,
                ],
            },
        ),
    ),
}
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0001.bin",
    ),
}
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0005.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0001/file-0005.bin",
)
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0001/file-0001.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0001/file-0005.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                171,
                43,
                82,
                114,
                123,
                200,
                204,
                44,
                125,
                210,
                135,
                57,
                150,
                217,
                99,
                147,
                218,
                213,
                31,
                45,
                3,
                70,
                107,
                211,
                137,
                15,
                64,
                28,
                231,
                240,
                121,
                236,
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
                33,
                115,
                139,
                172,
                85,
                72,
                78,
                88,
                180,
                206,
                164,
                78,
                192,
                172,
                18,
                101,
                219,
                37,
                146,
                163,
                138,
                26,
                126,
                133,
                68,
                92,
                97,
                118,
                217,
                73,
                169,
                189,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                171,
                43,
                82,
                114,
                123,
                200,
                204,
                44,
                125,
                210,
                135,
                57,
                150,
                217,
                99,
                147,
                218,
                213,
                31,
                45,
                3,
                70,
                107,
                211,
                137,
                15,
                64,
                28,
                231,
                240,
                121,
                236,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:169] stored_content_digest: None
[TRACE][file/src/common/compare.rs:234] res: RecordMissing {
    actual: ContentDigest(
        Some(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    171,
                    43,
                    82,
                    114,
                    123,
                    200,
                    204,
                    44,
                    125,
                    210,
                    135,
                    57,
                    150,
                    217,
                    99,
                    147,
                    218,
                    213,
                    31,
                    45,
                    3,
                    70,
                    107,
                    211,
                    137,
                    15,
                    64,
                    28,
                    231,
                    240,
                    121,
                    236,
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
                33,
                115,
                139,
                172,
                85,
                72,
                78,
                88,
                180,
                206,
                164,
                78,
                192,
                172,
                18,
                101,
                219,
                37,
                146,
                163,
                138,
                26,
                126,
                133,
                68,
                92,
                97,
                118,
                217,
                73,
                169,
                189,
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
                    33,
                    115,
                    139,
                    172,
                    85,
                    72,
                    78,
                    88,
                    180,
                    206,
                    164,
                    78,
                    192,
                    172,
                    18,
                    101,
                    219,
                    37,
                    146,
                    163,
                    138,
                    26,
                    126,
                    133,
                    68,
                    92,
                    97,
                    118,
                    217,
                    73,
                    169,
                    189,
                ],
            },
        ),
    ),
}
[TRACE][file/src/track/mod.rs:188] content_digest_diff: HStore {
    map: {
        XvcEntity(
            6,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            37,
                            6,
                            161,
                            107,
                            233,
                            11,
                            249,
                            105,
                            12,
                            226,
                            153,
                            200,
                            23,
                            223,
                            161,
                            157,
                            74,
                            152,
                            182,
                            39,
                            203,
                            116,
                            238,
                            32,
                            177,
                            241,
                            212,
                            20,
                            241,
                            221,
                            249,
                            97,
                        ],
                    },
                ),
            ),
        },
        XvcEntity(
            4,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            146,
                            34,
                            133,
                            46,
                            249,
                            220,
                            88,
                            76,
                            183,
                            72,
                            44,
                            11,
                            232,
                            214,
                            212,
                            177,
                            76,
                            25,
                            197,
                            87,
                            24,
                            0,
                            245,
                            209,
                            218,
                            68,
                            232,
                            177,
                            54,
                            162,
                            58,
                            80,
                        ],
                    },
                ),
            ),
        },
        XvcEntity(
            2,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            33,
                            115,
                            139,
                            172,
                            85,
                            72,
                            78,
                            88,
                            180,
                            206,
                            164,
                            78,
                            192,
                            172,
                            18,
                            101,
                            219,
                            37,
                            146,
                            163,
                            138,
                            26,
                            126,
                            133,
                            68,
                            92,
                            97,
                            118,
                            217,
                            73,
                            169,
                            189,
                        ],
                    },
                ),
            ),
        },
        XvcEntity(
            5,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            171,
                            43,
                            82,
                            114,
                            123,
                            200,
                            204,
                            44,
                            125,
                            210,
                            135,
                            57,
                            150,
                            217,
                            99,
                            147,
                            218,
                            213,
                            31,
                            45,
                            3,
                            70,
                            107,
                            211,
                            137,
                            15,
                            64,
                            28,
                            231,
                            240,
                            121,
                            236,
                        ],
                    },
                ),
            ),
        },
        XvcEntity(
            3,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            255,
                            145,
                            152,
                            94,
                            90,
                            220,
                            226,
                            165,
                            155,
                            54,
                            177,
                            253,
                            133,
                            239,
                            2,
                            145,
                            178,
                            88,
                            236,
                            20,
                            200,
                            161,
                            189,
                            61,
                            234,
                            164,
                            10,
                            156,
                            225,
                            28,
                            14,
                            182,
                        ],
                    },
                ),
            ),
        },
    },
}
[TRACE][file/src/common/mod.rs:509] records.len(): 0
[TRACE][file/src/common/mod.rs:511] new_store.len(): 5
[TRACE][file/src/common/mod.rs:509] records.len(): 0
[TRACE][file/src/common/mod.rs:511] new_store.len(): 5
[TRACE][file/src/common/mod.rs:509] records.len(): 0
[TRACE][file/src/common/mod.rs:511] new_store.len(): 5
[TRACE][file/src/common/mod.rs:509] records.len(): 0
[TRACE][file/src/common/mod.rs:511] new_store.len(): 5
[TRACE][file/src/common/mod.rs:509] records.len(): 0
[TRACE][file/src/common/mod.rs:511] new_store.len(): 5
[TRACE][file/src/track/mod.rs:198] current_xvc_metadata_store.len(): 5
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 1 regexes
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[TRACE][file/src/track/mod.rs:420] dir_map: {}
[TRACE][file/src/track/mod.rs:455] file_map: {
    "[CWD]/dir-0001/file-0002.bin": "[CWD]/dir-0001/.gitignore",
    "[CWD]/dir-0001/file-0004.bin": "[CWD]/dir-0001/.gitignore",
    "[CWD]/dir-0001/file-0005.bin": "[CWD]/dir-0001/.gitignore",
    "[CWD]/dir-0001/file-0003.bin": "[CWD]/dir-0001/.gitignore",
    "[CWD]/dir-0001/file-0001.bin": "[CWD]/dir-0001/.gitignore",
}
[TRACE][file/src/common/mod.rs:482] cache_dir: "[CWD]/.xvc/b3/250/6a1/6be90bf9690ce299c817dfa19d4a98b627cb74ee20b1f1d414f1ddf961"
[TRACE][file/src/common/mod.rs:482] cache_dir: "[CWD]/.xvc/b3/ab2/b52/727bc8cc2c7dd2873996d96393dad51f2d03466bd3890f401ce7f079ec"
[TRACE][file/src/common/mod.rs:482] cache_dir: "[CWD]/.xvc/b3/922/285/2ef9dc584cb7482c0be8d6d4b14c19c5571800f5d1da44e8b136a23a50"
[TRACE][file/src/common/mod.rs:482] cache_dir: "[CWD]/.xvc/b3/217/38b/ac55484e58b4cea44ec0ac1265db2592a38a1a7e85445c6176d949a9bd"
[TRACE][file/src/common/mod.rs:482] cache_dir: "[CWD]/.xvc/b3/ff9/198/5e5adce2a59b36b1fd85ef0291b258ec14c8a1bd3deaa40a9ce11c0eb6"
[TRACE][file/src/common/mod.rs:484] path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][file/src/common/mod.rs:484] path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/mod.rs:485] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/217/38b/ac55484e58b4cea44ec0ac1265db2592a38a1a7e85445c6176d949a9bd/0.bin",
)
[TRACE][file/src/common/mod.rs:485] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/250/6a1/6be90bf9690ce299c817dfa19d4a98b627cb74ee20b1f1d414f1ddf961/0.bin",
)
[TRACE][file/src/common/mod.rs:484] path: AbsolutePath(
    "[CWD]/dir-0001/file-0003.bin",
)
[TRACE][file/src/common/mod.rs:484] path: AbsolutePath(
    "[CWD]/dir-0001/file-0004.bin",
)
[TRACE][file/src/common/mod.rs:485] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/922/285/2ef9dc584cb7482c0be8d6d4b14c19c5571800f5d1da44e8b136a23a50/0.bin",
)
[TRACE][file/src/common/mod.rs:484] path: AbsolutePath(
    "[CWD]/dir-0001/file-0005.bin",
)
[TRACE][file/src/common/mod.rs:485] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/ff9/198/5e5adce2a59b36b1fd85ef0291b258ec14c8a1bd3deaa40a9ce11c0eb6/0.bin",
)
[TRACE][file/src/common/mod.rs:485] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/ab2/b52/727bc8cc2c7dd2873996d96393dad51f2d03466bd3890f401ce7f079ec/0.bin",
)
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0002.bin -> b3/250/6a1/6be90bf9690ce299c817dfa19d4a98b627cb74ee20b1f1d414f1ddf961/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0001.bin -> b3/217/38b/ac55484e58b4cea44ec0ac1265db2592a38a1a7e85445c6176d949a9bd/0.bin
[TRACE][file/src/common/mod.rs:430] path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][file/src/common/mod.rs:431] cache_type: Copy
[TRACE][file/src/common/mod.rs:430] path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/mod.rs:431] cache_type: Copy
[TRACE][file/src/common/mod.rs:430] path: AbsolutePath(
    "[CWD]/dir-0001/file-0004.bin",
)
[TRACE][file/src/common/mod.rs:431] cache_type: Copy
[TRACE][file/src/common/mod.rs:430] path: AbsolutePath(
    "[CWD]/dir-0001/file-0003.bin",
)
[TRACE][file/src/common/mod.rs:431] cache_type: Copy
[TRACE][file/src/common/mod.rs:430] path: AbsolutePath(
    "[CWD]/dir-0001/file-0005.bin",
)
[TRACE][file/src/common/mod.rs:431] cache_type: Copy
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0004.bin -> b3/922/285/2ef9dc584cb7482c0be8d6d4b14c19c5571800f5d1da44e8b136a23a50/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0003.bin -> b3/ff9/198/5e5adce2a59b36b1fd85ef0291b258ec14c8a1bd3deaa40a9ce11c0eb6/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0005.bin -> b3/ab2/b52/727bc8cc2c7dd2873996d96393dad51f2d03466bd3890f401ce7f079ec/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/217/38b/ac55484e58b4cea44ec0ac1265db2592a38a1a7e85445c6176d949a9bd/0.bin -> [CWD]/dir-0001/file-0001.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/922/285/2ef9dc584cb7482c0be8d6d4b14c19c5571800f5d1da44e8b136a23a50/0.bin -> [CWD]/dir-0001/file-0004.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/ff9/198/5e5adce2a59b36b1fd85ef0291b258ec14c8a1bd3deaa40a9ce11c0eb6/0.bin -> [CWD]/dir-0001/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/250/6a1/6be90bf9690ce299c817dfa19d4a98b627cb74ee20b1f1d414f1ddf961/0.bin -> [CWD]/dir-0001/file-0002.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/ab2/b52/727bc8cc2c7dd2873996d96393dad51f2d03466bd3890f401ce7f079ec/0.bin -> [CWD]/dir-0001/file-0005.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/250/6a1/6be90bf9690ce299c817dfa19d4a98b627cb74ee20b1f1d414f1ddf961/0.bin -> dir-0001/file-0002.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/217/38b/ac55484e58b4cea44ec0ac1265db2592a38a1a7e85445c6176d949a9bd/0.bin -> dir-0001/file-0001.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/ff9/198/5e5adce2a59b36b1fd85ef0291b258ec14c8a1bd3deaa40a9ce11c0eb6/0.bin -> dir-0001/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/ab2/b52/727bc8cc2c7dd2873996d96393dad51f2d03466bd3890f401ce7f079ec/0.bin -> dir-0001/file-0005.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/922/285/2ef9dc584cb7482c0be8d6d4b14c19c5571800f5d1da44e8b136a23a50/0.bin -> dir-0001/file-0004.bin
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
[DEBUG][lib/src/cli/mod.rs:516] Committing .xvc/ to git: [main f4eb2cd] Xvc auto-commit after ''
 7 files changed, 12 insertions(+)
 create mode 100644 .xvc/ec/1672805474575050
 create mode 100644 .xvc/store/cache-type-store/1672805474569653.json
 create mode 100644 .xvc/store/content-digest-store/1672805474570051.json
 create mode 100644 .xvc/store/file-text-or-binary-store/1672805474569850.json
 create mode 100644 .xvc/store/xvc-metadata-store/1672805474569363.json
 create mode 100644 .xvc/store/xvc-path-store/1672805474569094.json
 create mode 100644 dir-0001/.gitignore


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
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "cache.type": String(
                    "copy",
                ),
                "core.guid": String(
                    "071296b2004ca238",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "core.guid": String(
                    "92d078e73ecbc71f",
                ),
                "git.command": String(
                    "git",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "cache.type": String(
                    "copy",
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
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
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "debug",
            ),
        },
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
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
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
            ),
        },
        "file.carry-in.no_parallel": XvcConfigValue {
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
        "file.list.recursive": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "92d078e73ecbc71f",
            ),
        },
        "pipeline.current_pipeline": XvcConfigValue {
            source: Project,
            value: String(
                "default",
            ),
        },
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
            ),
        },
        "core.quiet": XvcConfigValue {
            source: CommandLine,
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
        "cache.type": XvcConfigValue {
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
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
            ),
        },
    },
    init_params: XvcConfigInitParams {
        default_configuration: "/n[core]/n# The repository id. Please do not delete or change it. /n# This is used to identify the repository and generate paths in storages. /n# In the future it may be used to in other ways. /nguid = /"071296b2004ca238/"/n# Default verbosity level. /n# One of /"error/", /"warn/", /"info/"/nverbosity = /"error/"/n/n[git]/n# Automate git operations. /n# Turning this off leads Xvc to behave as if it's not in a Git repository./n# Not recommended unless you're really not using Git/nuse_git = true/n# Command to run Git process./n# You can set this to an absolute path to specify an executable/n# If set to a non-absolute path, the executable will be searched in $PATH./ncommand = /"git/"/n/n# Commit changes in .xvc/ directory after commands./n# You can set this to false if you want to commit manually. /nauto_commit = true/n/n# Stage changes in .xvc/ directory without committing./n# auto_commit implies auto_stage. /n# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true. /nauto_stage = false/n/n[cache]/n# The cache type for XVC. It may take copy, hardlink, symlink, reflink as values./n# The default is copy to make sure the options is portable./n# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file./n# Note that hardlink and symlink are read-only as they link the files in cache. /ntype = /"copy/"/n# The hash algorithm used for the cache. /n# It may take blake3, blake2, sha2 or sha3 as values. /n# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc./n# The cache path is produced by prepending algorithm name to the cache. /n# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc. /nalgorithm = /"blake3/"/n/n[file]/n/n[file.track]/n/n# Don't move file content to cache after xvc file track/nno_commit = false/n# Force to track files even if they are already tracked./nforce = false/n/n# Xvc calculates file content digest differently for text and binary files./n# This option controls whether to treat files as text or binary./n# It may take auto, text or binary as values./n# Auto check each file individually and treat it as text if it's text./ntext_or_binary = /"auto/"/n/n# Don't use parallelism in track operations. /n# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations./nno_parallel = false/n/n[file.list]/n/n# Format for `xvc file list` rows. You can reorder or remove columns./n# The following are the keys for each row: /n# - {acd64}:  actual content digest. All 64 digits from the workspace file's content./n# - {acd8}:  actual content digest. First 8 digits the file content digest. /n# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),/n#   symlink (S), hardlink (H) or reflink (R). /n# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,/n#   GB and TB to represent sizes larger than 1MB. /n# - {ats}:  actual timestamp. The timestamp of the workspace file./n# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show/n#   whether the file timestamp is the same as the cached timestamp, newer,/n#   older, not cached or not tracked./n# - {name}: The name of the file or directory./n# - {rcd64}:  recorded content digest. All 64 digits./n# - {rcd8}:  recorded content digest. First 8 digits./n# - {rct}:  recorded cache type. Whether the entry is linked to the workspace/n#   as a copy (C), symlink (S), hardlink (H) or reflink (R)./n# - {rsz}:  recorded size. The size of the cached content in bytes. It uses/n#   MB, GB and TB to represent sizes larged than 1MB./n# - {rts}:  recorded timestamp. The timestamp of the cached content./n# /n# There are no escape sequences in the format string. /n# If you want to add a tab, type it to the string./n# If you want to add a literal double curly brace, open an issue. /nformat = /"{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}/"/n/n# Default sort order for `xvc file list`./n# Valid values are/n# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc./nsort = /"name-desc/"/n/n# Do not show a summary for as the final row for `xvc file list`./nno_summary = false/n/n# List files recursively always./nrecursive = false/n/n[file.carry-in]/n# Carry-in the files to cache always, even if they are already present./nforce = false/n/n# Don't use parallel move/copy in carry-in/nno_parallel = false/n/n[pipeline]/n# Name of the current pipeline to run/ncurrent_pipeline = /"default/"/n# Name of the default pipeline/ndefault = /"default/"/n# Name of the default params file name/ndefault_params_file = /"params.yaml/"/n/n",
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
    "[CWD]/.xvc/ec/1672805473987094",
    "[CWD]/.xvc/ec/1672805473989971",
    "[CWD]/.xvc/ec/1672805474274848",
    "[CWD]/.xvc/ec/1672805474575050",
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
[TRACE][file/src/common/mod.rs:268] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][file/src/common/mod.rs:269] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs:285] all_paths: {
    XvcPath(
        "dir-0002/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672805473,
                tv_nsec: 859377302,
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
                tv_sec: 1672805473,
                tv_nsec: 862552462,
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
                tv_sec: 1672805473,
                tv_nsec: 859717098,
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
                tv_sec: 1672805473,
                tv_nsec: 858018412,
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
                tv_sec: 1672805473,
                tv_nsec: 861163530,
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
                tv_sec: 1672805473,
                tv_nsec: 856958318,
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
                tv_sec: 1672805473,
                tv_nsec: 860605524,
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
                tv_sec: 1672805473,
                tv_nsec: 858838213,
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
                tv_sec: 1672805473,
                tv_nsec: 859103591,
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
                tv_sec: 1672805473,
                tv_nsec: 862783048,
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
                tv_sec: 1672805473,
                tv_nsec: 858310999,
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
                tv_sec: 1672805473,
                tv_nsec: 987301584,
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
                tv_sec: 1672805473,
                tv_nsec: 863524890,
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
                tv_sec: 1672805473,
                tv_nsec: 860286521,
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
                tv_sec: 1672805473,
                tv_nsec: 858566502,
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
                tv_sec: 1672805473,
                tv_nsec: 857263571,
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
                tv_sec: 1672805473,
                tv_nsec: 860000684,
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
                tv_sec: 1672805473,
                tv_nsec: 857780202,
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
                tv_sec: 1672805473,
                tv_nsec: 861701370,
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
                tv_sec: 1672805474,
                tv_nsec: 574583959,
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
                tv_sec: 1672805473,
                tv_nsec: 861439659,
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
                tv_sec: 1672805473,
                tv_nsec: 859158592,
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
                tv_sec: 1672805473,
                tv_nsec: 857526824,
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
                tv_sec: 1672805473,
                tv_nsec: 862249084,
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
                tv_sec: 1672805473,
                tv_nsec: 863333971,
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
                tv_sec: 1672805474,
                tv_nsec: 573683199,
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
                tv_sec: 1672805473,
                tv_nsec: 863021176,
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
                tv_sec: 1672805473,
                tv_nsec: 861979956,
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
                tv_sec: 1672805473,
                tv_nsec: 987228334,
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
                tv_sec: 1672805473,
                tv_nsec: 863268304,
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
                tv_sec: 1672805473,
                tv_nsec: 860531690,
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
                tv_sec: 1672805473,
                tv_nsec: 860819152,
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
                tv_sec: 1672805473,
                tv_nsec: 862036915,
            },
        ),
    },
}
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 1 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs:307] glob_matcher: GlobSet {
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
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 1 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
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
    "dir-0001/.gitignore",
)
[TRACE][file/src/list/mod.rs:207] &path_prefix: ""
[TRACE][file/src/list/mod.rs:209] ap: XvcPath(
    "dir-0001/file-0001.bin",
)
[TRACE][file/src/list/mod.rs:207] &path_prefix: ""
[TRACE][file/src/list/mod.rs:209] ap: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][file/src/list/mod.rs:207] &path_prefix: ""
[TRACE][file/src/list/mod.rs:209] ap: XvcPath(
    "dir-0001/file-0002.bin",
)
FX        1005 2023-01-04 04:11:13   dir-0001/file-0005.bin           ab2b5272
FX        1004 2023-01-04 04:11:13   dir-0001/file-0004.bin           9222852e
FX        1003 2023-01-04 04:11:13   dir-0001/file-0003.bin           ff91985e
FX        1002 2023-01-04 04:11:13   dir-0001/file-0002.bin           2506a16b
FX        1001 2023-01-04 04:11:13   dir-0001/file-0001.bin           21738bac
FX         149 2023-01-04 04:11:14   dir-0001/.gitignore           6fa45fe8
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
[DEBUG][lib/src/cli/mod.rs:516] Committing .xvc/ to git: [main 1c909aa] Xvc auto-commit after ''
 1 file changed, 1 insertion(+)
 create mode 100644 .xvc/ec/1672805474884913


```

If you add another set of files as hardlinks to the cached copies, it will
print the second letter as `H`.

```console
$ xvc file track dir-0002 --cache-type hardlink

$ xvc file list dir-0002
FX        1005 2023-01-04 04:11:13   dir-0002/file-0005.bin           904fa393
FX        1004 2023-01-04 04:11:13   dir-0002/file-0004.bin           7e1b3fb5
FX        1003 2023-01-04 04:11:13   dir-0002/file-0003.bin           066424b8
FX        1002 2023-01-04 04:11:13   dir-0002/file-0002.bin           af69b20c
FX        1001 2023-01-04 04:11:13   dir-0002/file-0001.bin           96ab56c3
FX         149 2023-01-04 04:11:15   dir-0002/.gitignore           eafe3a8a
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
FX         149 2023-01-04 04:11:15   dir-0003/.gitignore           d2ff7852
Total #: 6 Workspace Size:        1049 Cached Size:           0


```

Although not all filesystems support, `R` represents reflinks. 

### Sort options

You may sort `xvc file list` output by name, by modification time and by file
size. 