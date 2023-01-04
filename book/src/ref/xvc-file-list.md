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
FX        1005 2023-01-04 04:00:10   dir-0005/file-0005.bin           47b34b7c
FX        1004 2023-01-04 04:00:10   dir-0005/file-0004.bin           c7b14e20
FX        1003 2023-01-04 04:00:10   dir-0005/file-0003.bin           1ad87768
FX        1002 2023-01-04 04:00:10   dir-0005/file-0002.bin           9d4df302
FX        1001 2023-01-04 04:00:10   dir-0005/file-0001.bin           a0ac1d7f
DX         224 2023-01-04 04:00:10   dir-0005                   
FX        1005 2023-01-04 04:00:10   dir-0004/file-0005.bin           f228c85b
FX        1004 2023-01-04 04:00:10   dir-0004/file-0004.bin           84be8f56
FX        1003 2023-01-04 04:00:10   dir-0004/file-0003.bin           da0d95ce
FX        1002 2023-01-04 04:00:10   dir-0004/file-0002.bin           c83b5874
FX        1001 2023-01-04 04:00:10   dir-0004/file-0001.bin           9a9518f3
DX         224 2023-01-04 04:00:10   dir-0004                   
FX        1005 2023-01-04 04:00:10   dir-0003/file-0005.bin           637a4490
FX        1004 2023-01-04 04:00:10   dir-0003/file-0004.bin           a24e04ba
FX        1003 2023-01-04 04:00:10   dir-0003/file-0003.bin           317b3f2d
FX        1002 2023-01-04 04:00:10   dir-0003/file-0002.bin           7d957804
FX        1001 2023-01-04 04:00:10   dir-0003/file-0001.bin           3a9fe6da
DX         224 2023-01-04 04:00:10   dir-0003                   
FX        1005 2023-01-04 04:00:10   dir-0002/file-0005.bin           83ef6109
FX        1004 2023-01-04 04:00:10   dir-0002/file-0004.bin           f35b15e1
FX        1003 2023-01-04 04:00:10   dir-0002/file-0003.bin           fbf289c2
FX        1002 2023-01-04 04:00:10   dir-0002/file-0002.bin           fea4d989
FX        1001 2023-01-04 04:00:10   dir-0002/file-0001.bin           e699116a
DX         224 2023-01-04 04:00:10   dir-0002                   
FX        1005 2023-01-04 04:00:10   dir-0001/file-0005.bin           8a0486b1
FX        1004 2023-01-04 04:00:10   dir-0001/file-0004.bin           16c668f1
FX        1003 2023-01-04 04:00:10   dir-0001/file-0003.bin           d02ab6a4
FX        1002 2023-01-04 04:00:10   dir-0001/file-0002.bin           6466c242
FX        1001 2023-01-04 04:00:10   dir-0001/file-0001.bin           791c27d8
DX         224 2023-01-04 04:00:10   dir-0001                   
FX         130 2023-01-04 04:00:10   .xvcignore           ac46bf74
FX         107 2023-01-04 04:00:10   .gitignore           ce9fcf30
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
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "git.command": String(
                    "git",
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
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
                "git.use_git": Boolean(
                    true,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "cache.type": String(
                    "copy",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "core.guid": String(
                    "c1a1a5200b0c0f48",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "pipeline.current_pipeline": String(
                    "default",
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
                "cache.algorithm": String(
                    "blake3",
                ),
                "git.command": String(
                    "git",
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
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "core.guid": String(
                    "5d298331955f516a",
                ),
                "cache.type": String(
                    "copy",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.carry-in.force": Boolean(
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
        "file.track.force": XvcConfigValue {
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
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "5d298331955f516a",
            ),
        },
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
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
        "file.track.no_commit": XvcConfigValue {
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
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
            ),
        },
        "file.carry-in.force": XvcConfigValue {
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
        "file.track.no_parallel": XvcConfigValue {
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
    },
    init_params: XvcConfigInitParams {
        default_configuration: "/n[core]/n# The repository id. Please do not delete or change it. /n# This is used to identify the repository and generate paths in storages. /n# In the future it may be used to in other ways. /nguid = /"c1a1a5200b0c0f48/"/n# Default verbosity level. /n# One of /"error/", /"warn/", /"info/"/nverbosity = /"error/"/n/n[git]/n# Automate git operations. /n# Turning this off leads Xvc to behave as if it's not in a Git repository./n# Not recommended unless you're really not using Git/nuse_git = true/n# Command to run Git process./n# You can set this to an absolute path to specify an executable/n# If set to a non-absolute path, the executable will be searched in $PATH./ncommand = /"git/"/n/n# Commit changes in .xvc/ directory after commands./n# You can set this to false if you want to commit manually. /nauto_commit = true/n/n# Stage changes in .xvc/ directory without committing./n# auto_commit implies auto_stage. /n# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true. /nauto_stage = false/n/n[cache]/n# The cache type for XVC. It may take copy, hardlink, symlink, reflink as values./n# The default is copy to make sure the options is portable./n# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file./n# Note that hardlink and symlink are read-only as they link the files in cache. /ntype = /"copy/"/n# The hash algorithm used for the cache. /n# It may take blake3, blake2, sha2 or sha3 as values. /n# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc./n# The cache path is produced by prepending algorithm name to the cache. /n# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc. /nalgorithm = /"blake3/"/n/n[file]/n/n[file.track]/n/n# Don't move file content to cache after xvc file track/nno_commit = false/n# Force to track files even if they are already tracked./nforce = false/n/n# Xvc calculates file content digest differently for text and binary files./n# This option controls whether to treat files as text or binary./n# It may take auto, text or binary as values./n# Auto check each file individually and treat it as text if it's text./ntext_or_binary = /"auto/"/n/n# Don't use parallelism in track operations. /n# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations./nno_parallel = false/n/n[file.list]/n/n# Format for `xvc file list` rows. You can reorder or remove columns./n# The following are the keys for each row: /n# - {acd64}:  actual content digest. All 64 digits from the workspace file's content./n# - {acd8}:  actual content digest. First 8 digits the file content digest. /n# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),/n#   symlink (S), hardlink (H) or reflink (R). /n# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,/n#   GB and TB to represent sizes larger than 1MB. /n# - {ats}:  actual timestamp. The timestamp of the workspace file./n# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show/n#   whether the file timestamp is the same as the cached timestamp, newer,/n#   older, not cached or not tracked./n# - {name}: The name of the file or directory./n# - {rcd64}:  recorded content digest. All 64 digits./n# - {rcd8}:  recorded content digest. First 8 digits./n# - {rct}:  recorded cache type. Whether the entry is linked to the workspace/n#   as a copy (C), symlink (S), hardlink (H) or reflink (R)./n# - {rsz}:  recorded size. The size of the cached content in bytes. It uses/n#   MB, GB and TB to represent sizes larged than 1MB./n# - {rts}:  recorded timestamp. The timestamp of the cached content./n# /n# There are no escape sequences in the format string. /n# If you want to add a tab, type it to the string./n# If you want to add a literal double curly brace, open an issue. /nformat = /"{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}/"/n/n# Default sort order for `xvc file list`./n# Valid values are/n# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc./nsort = /"name-desc/"/n/n# Do not show a summary for as the final row for `xvc file list`./nno_summary = false/n/n# List files recursively always./nrecursive = false/n/n[file.carry-in]/n# Carry-in the files to cache always, even if they are already present./nforce = false/n/n# Don't use parallel move/copy in carry-in/nno_parallel = false/n/n[pipeline]/n# Name of the current pipeline to run/ncurrent_pipeline = /"default/"/n# Name of the default pipeline/ndefault = /"default/"/n# Name of the default params file name/ndefault_params_file = /"params.yaml/"/n/n",
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
    "[CWD]/.xvc/ec/1672804810404539",
    "[CWD]/.xvc/ec/1672804810407926",
    "[CWD]/.xvc/ec/1672804810727734",
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
        "dir-0004/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672804810,
                tv_nsec: 273050703,
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
                tv_sec: 1672804810,
                tv_nsec: 272709449,
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
                tv_sec: 1672804810,
                tv_nsec: 404894487,
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
                tv_sec: 1672804810,
                tv_nsec: 273880129,
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
                tv_sec: 1672804810,
                tv_nsec: 268238150,
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
                tv_sec: 1672804810,
                tv_nsec: 273327415,
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
                tv_sec: 1672804810,
                tv_nsec: 269993253,
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
                tv_sec: 1672804810,
                tv_nsec: 270768137,
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
                tv_sec: 1672804810,
                tv_nsec: 277259708,
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
                tv_sec: 1672804810,
                tv_nsec: 271430769,
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
                tv_sec: 1672804810,
                tv_nsec: 268967408,
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
                tv_sec: 1672804810,
                tv_nsec: 275719941,
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
                tv_sec: 1672804810,
                tv_nsec: 271772898,
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
                tv_sec: 1672804810,
                tv_nsec: 268894449,
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
                tv_sec: 1672804810,
                tv_nsec: 271019889,
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
                tv_sec: 1672804810,
                tv_nsec: 276994663,
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
                tv_sec: 1672804810,
                tv_nsec: 274746514,
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
                tv_sec: 1672804810,
                tv_nsec: 274463636,
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
                tv_sec: 1672804810,
                tv_nsec: 404999030,
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
                tv_sec: 1672804810,
                tv_nsec: 272403321,
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
                tv_sec: 1672804810,
                tv_nsec: 276843953,
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
                tv_sec: 1672804810,
                tv_nsec: 268572154,
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
                tv_sec: 1672804810,
                tv_nsec: 267847188,
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
                tv_sec: 1672804810,
                tv_nsec: 269223703,
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
                tv_sec: 1672804810,
                tv_nsec: 269648916,
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
                tv_sec: 1672804810,
                tv_nsec: 273607709,
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
                tv_sec: 1672804810,
                tv_nsec: 274143424,
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
                tv_sec: 1672804810,
                tv_nsec: 272485072,
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
                tv_sec: 1672804810,
                tv_nsec: 270348715,
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
                tv_sec: 1672804810,
                tv_nsec: 272115985,
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
                tv_sec: 1672804810,
                tv_nsec: 270688719,
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
                tv_sec: 1672804810,
                tv_nsec: 273937338,
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
        "dir-0001/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672804810,
                tv_nsec: 268238150,
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
                tv_sec: 1672804810,
                tv_nsec: 267847188,
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
                tv_sec: 1672804810,
                tv_nsec: 268894449,
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
                tv_sec: 1672804810,
                tv_nsec: 268572154,
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
                tv_sec: 1672804810,
                tv_nsec: 269223703,
            },
        ),
    },
}
[TRACE][file/src/common/compare.rs:77] pmm: {
    XvcPath(
        "dir-0001/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672804810,
                tv_nsec: 268238150,
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
                tv_sec: 1672804810,
                tv_nsec: 267847188,
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
                tv_sec: 1672804810,
                tv_nsec: 268894449,
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
                tv_sec: 1672804810,
                tv_nsec: 268572154,
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
                tv_sec: 1672804810,
                tv_nsec: 269223703,
            },
        ),
    },
}
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    2,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0001/file-0001.bin",
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
    "dir-0001/file-0003.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    5,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0001/file-0005.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    6,
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0002.bin",
    ),
}
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0001.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0001/file-0001.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0004.bin",
    ),
}
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0001/file-0004.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0003.bin",
    ),
}
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
                100,
                102,
                194,
                66,
                37,
                180,
                237,
                222,
                246,
                56,
                22,
                45,
                48,
                134,
                150,
                204,
                4,
                12,
                234,
                73,
                126,
                179,
                140,
                110,
                166,
                81,
                81,
                163,
                166,
                254,
                187,
                102,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0005.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0001/file-0005.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0001/file-0005.bin",
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                121,
                28,
                39,
                216,
                38,
                238,
                160,
                30,
                209,
                212,
                135,
                1,
                38,
                25,
                238,
                146,
                130,
                221,
                46,
                23,
                48,
                210,
                82,
                134,
                240,
                126,
                142,
                5,
                40,
                49,
                149,
                8,
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
                121,
                28,
                39,
                216,
                38,
                238,
                160,
                30,
                209,
                212,
                135,
                1,
                38,
                25,
                238,
                146,
                130,
                221,
                46,
                23,
                48,
                210,
                82,
                134,
                240,
                126,
                142,
                5,
                40,
                49,
                149,
                8,
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
                    121,
                    28,
                    39,
                    216,
                    38,
                    238,
                    160,
                    30,
                    209,
                    212,
                    135,
                    1,
                    38,
                    25,
                    238,
                    146,
                    130,
                    221,
                    46,
                    23,
                    48,
                    210,
                    82,
                    134,
                    240,
                    126,
                    142,
                    5,
                    40,
                    49,
                    149,
                    8,
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
                100,
                102,
                194,
                66,
                37,
                180,
                237,
                222,
                246,
                56,
                22,
                45,
                48,
                134,
                150,
                204,
                4,
                12,
                234,
                73,
                126,
                179,
                140,
                110,
                166,
                81,
                81,
                163,
                166,
                254,
                187,
                102,
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
                198,
                104,
                241,
                59,
                48,
                230,
                193,
                181,
                143,
                109,
                36,
                212,
                242,
                174,
                98,
                92,
                57,
                201,
                53,
                154,
                123,
                226,
                6,
                247,
                146,
                133,
                155,
                133,
                69,
                115,
                41,
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
                208,
                42,
                182,
                164,
                52,
                115,
                181,
                48,
                224,
                114,
                210,
                214,
                201,
                248,
                114,
                64,
                207,
                111,
                174,
                237,
                200,
                189,
                195,
                53,
                61,
                118,
                169,
                206,
                25,
                24,
                116,
                237,
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
                    100,
                    102,
                    194,
                    66,
                    37,
                    180,
                    237,
                    222,
                    246,
                    56,
                    22,
                    45,
                    48,
                    134,
                    150,
                    204,
                    4,
                    12,
                    234,
                    73,
                    126,
                    179,
                    140,
                    110,
                    166,
                    81,
                    81,
                    163,
                    166,
                    254,
                    187,
                    102,
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
                198,
                104,
                241,
                59,
                48,
                230,
                193,
                181,
                143,
                109,
                36,
                212,
                242,
                174,
                98,
                92,
                57,
                201,
                53,
                154,
                123,
                226,
                6,
                247,
                146,
                133,
                155,
                133,
                69,
                115,
                41,
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
                    22,
                    198,
                    104,
                    241,
                    59,
                    48,
                    230,
                    193,
                    181,
                    143,
                    109,
                    36,
                    212,
                    242,
                    174,
                    98,
                    92,
                    57,
                    201,
                    53,
                    154,
                    123,
                    226,
                    6,
                    247,
                    146,
                    133,
                    155,
                    133,
                    69,
                    115,
                    41,
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
                208,
                42,
                182,
                164,
                52,
                115,
                181,
                48,
                224,
                114,
                210,
                214,
                201,
                248,
                114,
                64,
                207,
                111,
                174,
                237,
                200,
                189,
                195,
                53,
                61,
                118,
                169,
                206,
                25,
                24,
                116,
                237,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                138,
                4,
                134,
                177,
                63,
                28,
                124,
                237,
                203,
                56,
                6,
                199,
                142,
                157,
                3,
                198,
                246,
                131,
                29,
                86,
                76,
                32,
                78,
                218,
                3,
                120,
                237,
                226,
                96,
                8,
                21,
                128,
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
                    208,
                    42,
                    182,
                    164,
                    52,
                    115,
                    181,
                    48,
                    224,
                    114,
                    210,
                    214,
                    201,
                    248,
                    114,
                    64,
                    207,
                    111,
                    174,
                    237,
                    200,
                    189,
                    195,
                    53,
                    61,
                    118,
                    169,
                    206,
                    25,
                    24,
                    116,
                    237,
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
                138,
                4,
                134,
                177,
                63,
                28,
                124,
                237,
                203,
                56,
                6,
                199,
                142,
                157,
                3,
                198,
                246,
                131,
                29,
                86,
                76,
                32,
                78,
                218,
                3,
                120,
                237,
                226,
                96,
                8,
                21,
                128,
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
                    138,
                    4,
                    134,
                    177,
                    63,
                    28,
                    124,
                    237,
                    203,
                    56,
                    6,
                    199,
                    142,
                    157,
                    3,
                    198,
                    246,
                    131,
                    29,
                    86,
                    76,
                    32,
                    78,
                    218,
                    3,
                    120,
                    237,
                    226,
                    96,
                    8,
                    21,
                    128,
                ],
            },
        ),
    ),
}
[TRACE][file/src/track/mod.rs:188] content_digest_diff: HStore {
    map: {
        XvcEntity(
            2,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            100,
                            102,
                            194,
                            66,
                            37,
                            180,
                            237,
                            222,
                            246,
                            56,
                            22,
                            45,
                            48,
                            134,
                            150,
                            204,
                            4,
                            12,
                            234,
                            73,
                            126,
                            179,
                            140,
                            110,
                            166,
                            81,
                            81,
                            163,
                            166,
                            254,
                            187,
                            102,
                        ],
                    },
                ),
            ),
        },
        XvcEntity(
            6,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            138,
                            4,
                            134,
                            177,
                            63,
                            28,
                            124,
                            237,
                            203,
                            56,
                            6,
                            199,
                            142,
                            157,
                            3,
                            198,
                            246,
                            131,
                            29,
                            86,
                            76,
                            32,
                            78,
                            218,
                            3,
                            120,
                            237,
                            226,
                            96,
                            8,
                            21,
                            128,
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
                            121,
                            28,
                            39,
                            216,
                            38,
                            238,
                            160,
                            30,
                            209,
                            212,
                            135,
                            1,
                            38,
                            25,
                            238,
                            146,
                            130,
                            221,
                            46,
                            23,
                            48,
                            210,
                            82,
                            134,
                            240,
                            126,
                            142,
                            5,
                            40,
                            49,
                            149,
                            8,
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
                            208,
                            42,
                            182,
                            164,
                            52,
                            115,
                            181,
                            48,
                            224,
                            114,
                            210,
                            214,
                            201,
                            248,
                            114,
                            64,
                            207,
                            111,
                            174,
                            237,
                            200,
                            189,
                            195,
                            53,
                            61,
                            118,
                            169,
                            206,
                            25,
                            24,
                            116,
                            237,
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
                            22,
                            198,
                            104,
                            241,
                            59,
                            48,
                            230,
                            193,
                            181,
                            143,
                            109,
                            36,
                            212,
                            242,
                            174,
                            98,
                            92,
                            57,
                            201,
                            53,
                            154,
                            123,
                            226,
                            6,
                            247,
                            146,
                            133,
                            155,
                            133,
                            69,
                            115,
                            41,
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
    "[CWD]/dir-0001/file-0003.bin": "[CWD]/dir-0001/.gitignore",
    "[CWD]/dir-0001/file-0004.bin": "[CWD]/dir-0001/.gitignore",
    "[CWD]/dir-0001/file-0005.bin": "[CWD]/dir-0001/.gitignore",
    "[CWD]/dir-0001/file-0002.bin": "[CWD]/dir-0001/.gitignore",
    "[CWD]/dir-0001/file-0001.bin": "[CWD]/dir-0001/.gitignore",
}
[TRACE][file/src/common/mod.rs:482] cache_dir: "[CWD]/.xvc/b3/16c/668/f13b30e6c1b58f6d24d4f2ae625c39c9359a7be206f792859b85457329"
[TRACE][file/src/common/mod.rs:482] cache_dir: "[CWD]/.xvc/b3/d02/ab6/a43473b530e072d2d6c9f87240cf6faeedc8bdc3353d76a9ce191874ed"
[TRACE][file/src/common/mod.rs:482] cache_dir: "[CWD]/.xvc/b3/646/6c2/4225b4eddef638162d308696cc040cea497eb38c6ea65151a3a6febb66"
[TRACE][file/src/common/mod.rs:482] cache_dir: "[CWD]/.xvc/b3/8a0/486/b13f1c7cedcb3806c78e9d03c6f6831d564c204eda0378ede260081580"
[TRACE][file/src/common/mod.rs:482] cache_dir: "[CWD]/.xvc/b3/791/c27/d826eea01ed1d487012619ee9282dd2e1730d25286f07e8e0528319508"
[TRACE][file/src/common/mod.rs:484] path: AbsolutePath(
    "[CWD]/dir-0001/file-0005.bin",
)
[TRACE][file/src/common/mod.rs:485] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/8a0/486/b13f1c7cedcb3806c78e9d03c6f6831d564c204eda0378ede260081580/0.bin",
)
[TRACE][file/src/common/mod.rs:484] path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/mod.rs:485] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/646/6c2/4225b4eddef638162d308696cc040cea497eb38c6ea65151a3a6febb66/0.bin",
)
[TRACE][file/src/common/mod.rs:484] path: AbsolutePath(
    "[CWD]/dir-0001/file-0004.bin",
)
[TRACE][file/src/common/mod.rs:485] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/16c/668/f13b30e6c1b58f6d24d4f2ae625c39c9359a7be206f792859b85457329/0.bin",
)
[TRACE][file/src/common/mod.rs:484] path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][file/src/common/mod.rs:485] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/791/c27/d826eea01ed1d487012619ee9282dd2e1730d25286f07e8e0528319508/0.bin",
)
[TRACE][file/src/common/mod.rs:484] path: AbsolutePath(
    "[CWD]/dir-0001/file-0003.bin",
)
[TRACE][file/src/common/mod.rs:485] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/d02/ab6/a43473b530e072d2d6c9f87240cf6faeedc8bdc3353d76a9ce191874ed/0.bin",
)
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0005.bin -> b3/8a0/486/b13f1c7cedcb3806c78e9d03c6f6831d564c204eda0378ede260081580/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0002.bin -> b3/646/6c2/4225b4eddef638162d308696cc040cea497eb38c6ea65151a3a6febb66/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0004.bin -> b3/16c/668/f13b30e6c1b58f6d24d4f2ae625c39c9359a7be206f792859b85457329/0.bin
[TRACE][file/src/common/mod.rs:430] path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/mod.rs:430] path: AbsolutePath(
    "[CWD]/dir-0001/file-0004.bin",
)
[TRACE][file/src/common/mod.rs:431] cache_type: Copy
[TRACE][file/src/common/mod.rs:431] cache_type: Copy
[TRACE][file/src/common/mod.rs:430] path: AbsolutePath(
    "[CWD]/dir-0001/file-0005.bin",
)
[TRACE][file/src/common/mod.rs:431] cache_type: Copy
[TRACE][file/src/common/mod.rs:430] path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][file/src/common/mod.rs:431] cache_type: Copy
[TRACE][file/src/common/mod.rs:430] path: AbsolutePath(
    "[CWD]/dir-0001/file-0003.bin",
)
[TRACE][file/src/common/mod.rs:431] cache_type: Copy
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0001.bin -> b3/791/c27/d826eea01ed1d487012619ee9282dd2e1730d25286f07e8e0528319508/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0003.bin -> b3/d02/ab6/a43473b530e072d2d6c9f87240cf6faeedc8bdc3353d76a9ce191874ed/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/791/c27/d826eea01ed1d487012619ee9282dd2e1730d25286f07e8e0528319508/0.bin -> [CWD]/dir-0001/file-0001.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/16c/668/f13b30e6c1b58f6d24d4f2ae625c39c9359a7be206f792859b85457329/0.bin -> [CWD]/dir-0001/file-0004.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/646/6c2/4225b4eddef638162d308696cc040cea497eb38c6ea65151a3a6febb66/0.bin -> [CWD]/dir-0001/file-0002.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/d02/ab6/a43473b530e072d2d6c9f87240cf6faeedc8bdc3353d76a9ce191874ed/0.bin -> [CWD]/dir-0001/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/16c/668/f13b30e6c1b58f6d24d4f2ae625c39c9359a7be206f792859b85457329/0.bin -> dir-0001/file-0004.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/8a0/486/b13f1c7cedcb3806c78e9d03c6f6831d564c204eda0378ede260081580/0.bin -> [CWD]/dir-0001/file-0005.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/791/c27/d826eea01ed1d487012619ee9282dd2e1730d25286f07e8e0528319508/0.bin -> dir-0001/file-0001.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/d02/ab6/a43473b530e072d2d6c9f87240cf6faeedc8bdc3353d76a9ce191874ed/0.bin -> dir-0001/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/646/6c2/4225b4eddef638162d308696cc040cea497eb38c6ea65151a3a6febb66/0.bin -> dir-0001/file-0002.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/8a0/486/b13f1c7cedcb3806c78e9d03c6f6831d564c204eda0378ede260081580/0.bin -> dir-0001/file-0005.bin
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
[DEBUG][lib/src/cli/mod.rs:516] Committing .xvc/ to git: [main 2f746dd] Xvc auto-commit after ''
 7 files changed, 12 insertions(+)
 create mode 100644 .xvc/ec/1672804811031408
 create mode 100644 .xvc/store/cache-type-store/1672804811025179.json
 create mode 100644 .xvc/store/content-digest-store/1672804811025496.json
 create mode 100644 .xvc/store/file-text-or-binary-store/1672804811025333.json
 create mode 100644 .xvc/store/xvc-metadata-store/1672804811024081.json
 create mode 100644 .xvc/store/xvc-path-store/1672804811023866.json
 create mode 100644 dir-0001/.gitignore


$ xvc -vvvv file list dir-0001/
FX        1005 2023-01-04 04:00:10   dir-0001/file-0005.bin           8a0486b1
FX        1004 2023-01-04 04:00:10   dir-0001/file-0004.bin           16c668f1
FX        1003 2023-01-04 04:00:10   dir-0001/file-0003.bin           d02ab6a4
FX        1002 2023-01-04 04:00:10   dir-0001/file-0002.bin           6466c242
FX        1001 2023-01-04 04:00:10   dir-0001/file-0001.bin           791c27d8
FX         149 2023-01-04 04:00:11   dir-0001/.gitignore           043f15e3
Total #: 6 Workspace Size:        5164 Cached Size:           0


```

If you add another set of files as hardlinks to the cached copies, it will
print the second letter as `H`.

```console
$ xvc file track dir-0002 --cache-type hardlink

$ xvc file list dir-0002
FX        1005 2023-01-04 04:00:10   dir-0002/file-0005.bin           83ef6109
FX        1004 2023-01-04 04:00:10   dir-0002/file-0004.bin           f35b15e1
FX        1003 2023-01-04 04:00:10   dir-0002/file-0003.bin           fbf289c2
FX        1002 2023-01-04 04:00:10   dir-0002/file-0002.bin           fea4d989
FX        1001 2023-01-04 04:00:10   dir-0002/file-0001.bin           e699116a
FX         149 2023-01-04 04:00:11   dir-0002/.gitignore           d4e6813c
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
FX         149 2023-01-04 04:00:12   dir-0003/.gitignore           6bcc1497
Total #: 6 Workspace Size:        1049 Cached Size:           0


```

Although not all filesystems support, `R` represents reflinks. 

### Sort options

You may sort `xvc file list` output by name, by modification time and by file
size. 