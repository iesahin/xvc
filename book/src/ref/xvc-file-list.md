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
FX        1005 2023-01-04 03:52:31   dir-0005/file-0005.bin           f4a711ea
FX        1004 2023-01-04 03:52:31   dir-0005/file-0004.bin           975db44f
FX        1003 2023-01-04 03:52:31   dir-0005/file-0003.bin           ffbbc504
FX        1002 2023-01-04 03:52:31   dir-0005/file-0002.bin           b536c5b4
FX        1001 2023-01-04 03:52:31   dir-0005/file-0001.bin           afcb8943
DX         224 2023-01-04 03:52:31   dir-0005                   
FX        1005 2023-01-04 03:52:31   dir-0004/file-0005.bin           e985bedc
FX        1004 2023-01-04 03:52:31   dir-0004/file-0004.bin           4bd798f0
FX        1003 2023-01-04 03:52:31   dir-0004/file-0003.bin           c9e53382
FX        1002 2023-01-04 03:52:31   dir-0004/file-0002.bin           f953a9bb
FX        1001 2023-01-04 03:52:31   dir-0004/file-0001.bin           073cafcf
DX         224 2023-01-04 03:52:31   dir-0004                   
FX        1005 2023-01-04 03:52:31   dir-0003/file-0005.bin           ec4eb143
FX        1004 2023-01-04 03:52:31   dir-0003/file-0004.bin           f9f11977
FX        1003 2023-01-04 03:52:31   dir-0003/file-0003.bin           dcb09f8e
FX        1002 2023-01-04 03:52:31   dir-0003/file-0002.bin           248e6b42
FX        1001 2023-01-04 03:52:31   dir-0003/file-0001.bin           ec5083c5
DX         224 2023-01-04 03:52:31   dir-0003                   
FX        1005 2023-01-04 03:52:31   dir-0002/file-0005.bin           00deb432
FX        1004 2023-01-04 03:52:31   dir-0002/file-0004.bin           8b1bcc78
FX        1003 2023-01-04 03:52:31   dir-0002/file-0003.bin           3ebe96b3
FX        1002 2023-01-04 03:52:31   dir-0002/file-0002.bin           a1009ead
FX        1001 2023-01-04 03:52:31   dir-0002/file-0001.bin           7015119d
DX         224 2023-01-04 03:52:31   dir-0002                   
FX        1005 2023-01-04 03:52:31   dir-0001/file-0005.bin           e0a44b54
FX        1004 2023-01-04 03:52:31   dir-0001/file-0004.bin           dbba2dff
FX        1003 2023-01-04 03:52:31   dir-0001/file-0003.bin           243ce2b9
FX        1002 2023-01-04 03:52:31   dir-0001/file-0002.bin           3b062e10
FX        1001 2023-01-04 03:52:31   dir-0001/file-0001.bin           4e582703
DX         224 2023-01-04 03:52:31   dir-0001                   
FX         130 2023-01-04 03:52:31   .xvcignore           ac46bf74
FX         107 2023-01-04 03:52:31   .gitignore           ce9fcf30
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
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.command": String(
                    "git",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "core.guid": String(
                    "1d7c43732765ae48",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "cache.type": String(
                    "copy",
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "git.use_git": Boolean(
                    true,
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.guid": String(
                    "960f4e414de9b742",
                ),
                "cache.type": String(
                    "copy",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.track.text_or_binary": String(
                    "auto",
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
        "cache.type": XvcConfigValue {
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
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
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
        "pipeline.current_pipeline": XvcConfigValue {
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
        "file.track.force": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "960f4e414de9b742",
            ),
        },
        "file.track.text_or_binary": XvcConfigValue {
            source: Project,
            value: String(
                "auto",
            ),
        },
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "file.track.no_commit": XvcConfigValue {
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
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
            ),
        },
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "debug",
            ),
        },
        "git.auto_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
    },
    init_params: XvcConfigInitParams {
        default_configuration: "/n[core]/n# The repository id. Please do not delete or change it. /n# This is used to identify the repository and generate paths in storages. /n# In the future it may be used to in other ways. /nguid = /"1d7c43732765ae48/"/n# Default verbosity level. /n# One of /"error/", /"warn/", /"info/"/nverbosity = /"error/"/n/n[git]/n# Automate git operations. /n# Turning this off leads Xvc to behave as if it's not in a Git repository./n# Not recommended unless you're really not using Git/nuse_git = true/n# Command to run Git process./n# You can set this to an absolute path to specify an executable/n# If set to a non-absolute path, the executable will be searched in $PATH./ncommand = /"git/"/n/n# Commit changes in .xvc/ directory after commands./n# You can set this to false if you want to commit manually. /nauto_commit = true/n/n# Stage changes in .xvc/ directory without committing./n# auto_commit implies auto_stage. /n# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true. /nauto_stage = false/n/n[cache]/n# The cache type for XVC. It may take copy, hardlink, symlink, reflink as values./n# The default is copy to make sure the options is portable./n# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file./n# Note that hardlink and symlink are read-only as they link the files in cache. /ntype = /"copy/"/n# The hash algorithm used for the cache. /n# It may take blake3, blake2, sha2 or sha3 as values. /n# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc./n# The cache path is produced by prepending algorithm name to the cache. /n# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc. /nalgorithm = /"blake3/"/n/n[file]/n/n[file.track]/n/n# Don't move file content to cache after xvc file track/nno_commit = false/n# Force to track files even if they are already tracked./nforce = false/n/n# Xvc calculates file content digest differently for text and binary files./n# This option controls whether to treat files as text or binary./n# It may take auto, text or binary as values./n# Auto check each file individually and treat it as text if it's text./ntext_or_binary = /"auto/"/n/n# Don't use parallelism in track operations. /n# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations./nno_parallel = false/n/n[file.list]/n/n# Format for `xvc file list` rows. You can reorder or remove columns./n# The following are the keys for each row: /n# - {acd64}:  actual content digest. All 64 digits from the workspace file's content./n# - {acd8}:  actual content digest. First 8 digits the file content digest. /n# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),/n#   symlink (S), hardlink (H) or reflink (R). /n# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,/n#   GB and TB to represent sizes larger than 1MB. /n# - {ats}:  actual timestamp. The timestamp of the workspace file./n# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show/n#   whether the file timestamp is the same as the cached timestamp, newer,/n#   older, not cached or not tracked./n# - {name}: The name of the file or directory./n# - {rcd64}:  recorded content digest. All 64 digits./n# - {rcd8}:  recorded content digest. First 8 digits./n# - {rct}:  recorded cache type. Whether the entry is linked to the workspace/n#   as a copy (C), symlink (S), hardlink (H) or reflink (R)./n# - {rsz}:  recorded size. The size of the cached content in bytes. It uses/n#   MB, GB and TB to represent sizes larged than 1MB./n# - {rts}:  recorded timestamp. The timestamp of the cached content./n# /n# There are no escape sequences in the format string. /n# If you want to add a tab, type it to the string./n# If you want to add a literal double curly brace, open an issue. /nformat = /"{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}/"/n/n# Default sort order for `xvc file list`./n# Valid values are/n# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc./nsort = /"name-desc/"/n/n# Do not show a summary for as the final row for `xvc file list`./nno_summary = false/n/n# List files recursively always./nrecursive = false/n/n[file.carry-in]/n# Carry-in the files to cache always, even if they are already present./nforce = false/n/n# Don't use parallel move/copy in carry-in/nno_parallel = false/n/n[pipeline]/n# Name of the current pipeline to run/ncurrent_pipeline = /"default/"/n# Name of the default pipeline/ndefault = /"default/"/n# Name of the default params file name/ndefault_params_file = /"params.yaml/"/n/n",
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
    "[CWD]/.xvc/ec/1672804351475799",
    "[CWD]/.xvc/ec/1672804351478676",
    "[CWD]/.xvc/ec/1672804351766008",
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
        "dir-0003/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672804351,
                tv_nsec: 356303896,
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
                tv_sec: 1672804351,
                tv_nsec: 354347291,
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
                tv_sec: 1672804351,
                tv_nsec: 356645275,
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
                tv_sec: 1672804351,
                tv_nsec: 357349574,
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
                tv_sec: 1672804351,
                tv_nsec: 357705120,
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
                tv_sec: 1672804351,
                tv_nsec: 354956465,
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
                tv_sec: 1672804351,
                tv_nsec: 356965112,
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
                tv_sec: 1672804351,
                tv_nsec: 358731131,
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
                tv_sec: 1672804351,
                tv_nsec: 356475773,
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
                tv_sec: 1672804351,
                tv_nsec: 355557763,
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
                tv_sec: 1672804351,
                tv_nsec: 356790193,
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
                tv_sec: 1672804351,
                tv_nsec: 355736265,
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
                tv_sec: 1672804351,
                tv_nsec: 354764379,
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
                tv_sec: 1672804351,
                tv_nsec: 358315960,
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
                tv_sec: 1672804351,
                tv_nsec: 357524868,
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
                tv_sec: 1672804351,
                tv_nsec: 358680256,
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
                tv_sec: 1672804351,
                tv_nsec: 357883789,
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
                tv_sec: 1672804351,
                tv_nsec: 358127916,
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
                tv_sec: 1672804351,
                tv_nsec: 475937004,
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
                tv_sec: 1672804351,
                tv_nsec: 358496129,
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
                tv_sec: 1672804351,
                tv_nsec: 355951934,
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
                tv_sec: 1672804351,
                tv_nsec: 358865174,
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
                tv_sec: 1672804351,
                tv_nsec: 355139967,
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
                tv_sec: 1672804351,
                tv_nsec: 357753246,
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
                tv_sec: 1672804351,
                tv_nsec: 355914184,
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
                tv_sec: 1672804351,
                tv_nsec: 356085602,
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
                tv_sec: 1672804351,
                tv_nsec: 354572919,
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
                tv_sec: 1672804351,
                tv_nsec: 475996088,
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
                tv_sec: 1672804351,
                tv_nsec: 356828277,
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
                tv_sec: 1672804351,
                tv_nsec: 357179322,
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
                tv_sec: 1672804351,
                tv_nsec: 355369219,
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
                tv_sec: 1672804351,
                tv_nsec: 354999799,
            },
        ),
    },
}
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 1 prefixes, 0 suffixes, 0 required extensions, 0 regexes
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
                tv_sec: 1672804351,
                tv_nsec: 354347291,
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
                tv_sec: 1672804351,
                tv_nsec: 355139967,
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
                tv_sec: 1672804351,
                tv_nsec: 354572919,
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
                tv_sec: 1672804351,
                tv_nsec: 354956465,
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
                tv_sec: 1672804351,
                tv_nsec: 354764379,
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
                tv_sec: 1672804351,
                tv_nsec: 354347291,
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
                tv_sec: 1672804351,
                tv_nsec: 355139967,
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
                tv_sec: 1672804351,
                tv_nsec: 354572919,
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
                tv_sec: 1672804351,
                tv_nsec: 354956465,
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
                tv_sec: 1672804351,
                tv_nsec: 354764379,
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
    "dir-0001/file-0005.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    3,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    4,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0001/file-0004.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    5,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0001/file-0003.bin",
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
        "dir-0001/file-0004.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0001.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0001/file-0004.bin",
)
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0001/file-0001.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0001/file-0004.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0005.bin",
    ),
}
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0003.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0001/file-0005.bin",
)
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0001/file-0003.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0001/file-0005.bin",
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                219,
                186,
                45,
                255,
                148,
                79,
                108,
                123,
                112,
                232,
                222,
                90,
                223,
                5,
                228,
                229,
                21,
                224,
                254,
                227,
                102,
                193,
                78,
                248,
                227,
                186,
                108,
                212,
                24,
                179,
                144,
                105,
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
                78,
                88,
                39,
                3,
                39,
                101,
                55,
                6,
                177,
                122,
                19,
                163,
                2,
                50,
                212,
                129,
                201,
                0,
                175,
                246,
                5,
                105,
                8,
                123,
                137,
                106,
                50,
                116,
                128,
                128,
                106,
                219,
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
                36,
                60,
                226,
                185,
                19,
                185,
                205,
                208,
                89,
                187,
                136,
                232,
                166,
                134,
                93,
                86,
                174,
                82,
                61,
                241,
                112,
                234,
                247,
                38,
                62,
                253,
                252,
                3,
                26,
                176,
                75,
                186,
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
                224,
                164,
                75,
                84,
                45,
                224,
                212,
                171,
                40,
                205,
                230,
                249,
                68,
                136,
                73,
                249,
                67,
                217,
                152,
                117,
                206,
                193,
                209,
                110,
                93,
                97,
                52,
                246,
                191,
                125,
                17,
                67,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                78,
                88,
                39,
                3,
                39,
                101,
                55,
                6,
                177,
                122,
                19,
                163,
                2,
                50,
                212,
                129,
                201,
                0,
                175,
                246,
                5,
                105,
                8,
                123,
                137,
                106,
                50,
                116,
                128,
                128,
                106,
                219,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                59,
                6,
                46,
                16,
                105,
                66,
                24,
                17,
                140,
                223,
                242,
                133,
                8,
                162,
                74,
                31,
                2,
                227,
                24,
                171,
                107,
                129,
                136,
                75,
                19,
                89,
                4,
                109,
                239,
                216,
                128,
                53,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                219,
                186,
                45,
                255,
                148,
                79,
                108,
                123,
                112,
                232,
                222,
                90,
                223,
                5,
                228,
                229,
                21,
                224,
                254,
                227,
                102,
                193,
                78,
                248,
                227,
                186,
                108,
                212,
                24,
                179,
                144,
                105,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                36,
                60,
                226,
                185,
                19,
                185,
                205,
                208,
                89,
                187,
                136,
                232,
                166,
                134,
                93,
                86,
                174,
                82,
                61,
                241,
                112,
                234,
                247,
                38,
                62,
                253,
                252,
                3,
                26,
                176,
                75,
                186,
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
                    78,
                    88,
                    39,
                    3,
                    39,
                    101,
                    55,
                    6,
                    177,
                    122,
                    19,
                    163,
                    2,
                    50,
                    212,
                    129,
                    201,
                    0,
                    175,
                    246,
                    5,
                    105,
                    8,
                    123,
                    137,
                    106,
                    50,
                    116,
                    128,
                    128,
                    106,
                    219,
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
                224,
                164,
                75,
                84,
                45,
                224,
                212,
                171,
                40,
                205,
                230,
                249,
                68,
                136,
                73,
                249,
                67,
                217,
                152,
                117,
                206,
                193,
                209,
                110,
                93,
                97,
                52,
                246,
                191,
                125,
                17,
                67,
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
                    219,
                    186,
                    45,
                    255,
                    148,
                    79,
                    108,
                    123,
                    112,
                    232,
                    222,
                    90,
                    223,
                    5,
                    228,
                    229,
                    21,
                    224,
                    254,
                    227,
                    102,
                    193,
                    78,
                    248,
                    227,
                    186,
                    108,
                    212,
                    24,
                    179,
                    144,
                    105,
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
                    36,
                    60,
                    226,
                    185,
                    19,
                    185,
                    205,
                    208,
                    89,
                    187,
                    136,
                    232,
                    166,
                    134,
                    93,
                    86,
                    174,
                    82,
                    61,
                    241,
                    112,
                    234,
                    247,
                    38,
                    62,
                    253,
                    252,
                    3,
                    26,
                    176,
                    75,
                    186,
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
                    224,
                    164,
                    75,
                    84,
                    45,
                    224,
                    212,
                    171,
                    40,
                    205,
                    230,
                    249,
                    68,
                    136,
                    73,
                    249,
                    67,
                    217,
                    152,
                    117,
                    206,
                    193,
                    209,
                    110,
                    93,
                    97,
                    52,
                    246,
                    191,
                    125,
                    17,
                    67,
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
                59,
                6,
                46,
                16,
                105,
                66,
                24,
                17,
                140,
                223,
                242,
                133,
                8,
                162,
                74,
                31,
                2,
                227,
                24,
                171,
                107,
                129,
                136,
                75,
                19,
                89,
                4,
                109,
                239,
                216,
                128,
                53,
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
                    59,
                    6,
                    46,
                    16,
                    105,
                    66,
                    24,
                    17,
                    140,
                    223,
                    242,
                    133,
                    8,
                    162,
                    74,
                    31,
                    2,
                    227,
                    24,
                    171,
                    107,
                    129,
                    136,
                    75,
                    19,
                    89,
                    4,
                    109,
                    239,
                    216,
                    128,
                    53,
                ],
            },
        ),
    ),
}
[TRACE][file/src/track/mod.rs:188] content_digest_diff: HStore {
    map: {
        XvcEntity(
            5,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            219,
                            186,
                            45,
                            255,
                            148,
                            79,
                            108,
                            123,
                            112,
                            232,
                            222,
                            90,
                            223,
                            5,
                            228,
                            229,
                            21,
                            224,
                            254,
                            227,
                            102,
                            193,
                            78,
                            248,
                            227,
                            186,
                            108,
                            212,
                            24,
                            179,
                            144,
                            105,
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
                            36,
                            60,
                            226,
                            185,
                            19,
                            185,
                            205,
                            208,
                            89,
                            187,
                            136,
                            232,
                            166,
                            134,
                            93,
                            86,
                            174,
                            82,
                            61,
                            241,
                            112,
                            234,
                            247,
                            38,
                            62,
                            253,
                            252,
                            3,
                            26,
                            176,
                            75,
                            186,
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
                            78,
                            88,
                            39,
                            3,
                            39,
                            101,
                            55,
                            6,
                            177,
                            122,
                            19,
                            163,
                            2,
                            50,
                            212,
                            129,
                            201,
                            0,
                            175,
                            246,
                            5,
                            105,
                            8,
                            123,
                            137,
                            106,
                            50,
                            116,
                            128,
                            128,
                            106,
                            219,
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
                            224,
                            164,
                            75,
                            84,
                            45,
                            224,
                            212,
                            171,
                            40,
                            205,
                            230,
                            249,
                            68,
                            136,
                            73,
                            249,
                            67,
                            217,
                            152,
                            117,
                            206,
                            193,
                            209,
                            110,
                            93,
                            97,
                            52,
                            246,
                            191,
                            125,
                            17,
                            67,
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
                            59,
                            6,
                            46,
                            16,
                            105,
                            66,
                            24,
                            17,
                            140,
                            223,
                            242,
                            133,
                            8,
                            162,
                            74,
                            31,
                            2,
                            227,
                            24,
                            171,
                            107,
                            129,
                            136,
                            75,
                            19,
                            89,
                            4,
                            109,
                            239,
                            216,
                            128,
                            53,
                        ],
                    },
                ),
            ),
        },
    },
}
[TRACE][file/src/common/mod.rs:508] records.len(): 0
[TRACE][file/src/common/mod.rs:510] new_store.len(): 5
[TRACE][file/src/common/mod.rs:508] records.len(): 0
[TRACE][file/src/common/mod.rs:510] new_store.len(): 5
[TRACE][file/src/common/mod.rs:508] records.len(): 0
[TRACE][file/src/common/mod.rs:510] new_store.len(): 5
[TRACE][file/src/common/mod.rs:508] records.len(): 0
[TRACE][file/src/common/mod.rs:510] new_store.len(): 5
[TRACE][file/src/common/mod.rs:508] records.len(): 0
[TRACE][file/src/common/mod.rs:510] new_store.len(): 5
[TRACE][file/src/track/mod.rs:198] current_xvc_metadata_store.len(): 5
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 1 regexes
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[TRACE][file/src/track/mod.rs:420] dir_map: {}
[TRACE][file/src/track/mod.rs:455] file_map: {
    "[CWD]/dir-0001/file-0004.bin": "[CWD]/dir-0001/.gitignore",
    "[CWD]/dir-0001/file-0001.bin": "[CWD]/dir-0001/.gitignore",
    "[CWD]/dir-0001/file-0003.bin": "[CWD]/dir-0001/.gitignore",
    "[CWD]/dir-0001/file-0002.bin": "[CWD]/dir-0001/.gitignore",
    "[CWD]/dir-0001/file-0005.bin": "[CWD]/dir-0001/.gitignore",
}
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/3b0/62e/10694218118cdff28508a24a1f02e318ab6b81884b1359046defd88035"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/243/ce2/b913b9cdd059bb88e8a6865d56ae523df170eaf7263efdfc031ab04bba"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/dbb/a2d/ff944f6c7b70e8de5adf05e4e515e0fee366c14ef8e3ba6cd418b39069"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/4e5/827/0327653706b17a13a30232d481c900aff60569087b896a327480806adb"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/e0a/44b/542de0d4ab28cde6f9448849f943d99875cec1d16e5d6134f6bf7d1143"
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/3b0/62e/10694218118cdff28508a24a1f02e318ab6b81884b1359046defd88035/0.bin",
)
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0001/file-0003.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/243/ce2/b913b9cdd059bb88e8a6865d56ae523df170eaf7263efdfc031ab04bba/0.bin",
)
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0001/file-0004.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/dbb/a2d/ff944f6c7b70e8de5adf05e4e515e0fee366c14ef8e3ba6cd418b39069/0.bin",
)
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/4e5/827/0327653706b17a13a30232d481c900aff60569087b896a327480806adb/0.bin",
)
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Copy
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0001/file-0003.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Copy
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0001/file-0004.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Copy
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0002.bin -> b3/3b0/62e/10694218118cdff28508a24a1f02e318ab6b81884b1359046defd88035/0.bin
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0001/file-0005.bin",
)
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/e0a/44b/542de0d4ab28cde6f9448849f943d99875cec1d16e5d6134f6bf7d1143/0.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Copy
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0003.bin -> b3/243/ce2/b913b9cdd059bb88e8a6865d56ae523df170eaf7263efdfc031ab04bba/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0004.bin -> b3/dbb/a2d/ff944f6c7b70e8de5adf05e4e515e0fee366c14ef8e3ba6cd418b39069/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0001.bin -> b3/4e5/827/0327653706b17a13a30232d481c900aff60569087b896a327480806adb/0.bin
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0001/file-0005.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Copy
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/3b0/62e/10694218118cdff28508a24a1f02e318ab6b81884b1359046defd88035/0.bin -> [CWD]/dir-0001/file-0002.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/243/ce2/b913b9cdd059bb88e8a6865d56ae523df170eaf7263efdfc031ab04bba/0.bin -> [CWD]/dir-0001/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0001/file-0005.bin -> b3/e0a/44b/542de0d4ab28cde6f9448849f943d99875cec1d16e5d6134f6bf7d1143/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/4e5/827/0327653706b17a13a30232d481c900aff60569087b896a327480806adb/0.bin -> [CWD]/dir-0001/file-0001.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/dbb/a2d/ff944f6c7b70e8de5adf05e4e515e0fee366c14ef8e3ba6cd418b39069/0.bin -> [CWD]/dir-0001/file-0004.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/4e5/827/0327653706b17a13a30232d481c900aff60569087b896a327480806adb/0.bin -> dir-0001/file-0001.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/3b0/62e/10694218118cdff28508a24a1f02e318ab6b81884b1359046defd88035/0.bin -> dir-0001/file-0002.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/dbb/a2d/ff944f6c7b70e8de5adf05e4e515e0fee366c14ef8e3ba6cd418b39069/0.bin -> dir-0001/file-0004.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [COPY] [CWD]/.xvc/b3/e0a/44b/542de0d4ab28cde6f9448849f943d99875cec1d16e5d6134f6bf7d1143/0.bin -> [CWD]/dir-0001/file-0005.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/243/ce2/b913b9cdd059bb88e8a6865d56ae523df170eaf7263efdfc031ab04bba/0.bin -> dir-0001/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/e0a/44b/542de0d4ab28cde6f9448849f943d99875cec1d16e5d6134f6bf7d1143/0.bin -> dir-0001/file-0005.bin
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
[DEBUG][lib/src/cli/mod.rs:516] Committing .xvc/ to git: [main 716b2f9] Xvc auto-commit after ''
 7 files changed, 12 insertions(+)
 create mode 100644 .xvc/ec/1672804352068645
 create mode 100644 .xvc/store/cache-type-store/1672804352062855.json
 create mode 100644 .xvc/store/content-digest-store/1672804352063288.json
 create mode 100644 .xvc/store/file-text-or-binary-store/1672804352063036.json
 create mode 100644 .xvc/store/xvc-metadata-store/1672804352062589.json
 create mode 100644 .xvc/store/xvc-path-store/1672804352062323.json
 create mode 100644 dir-0001/.gitignore


$ xvc file list dir-0001/
FX        1005 2023-01-04 03:52:31   dir-0001/file-0005.bin           e0a44b54
FX        1004 2023-01-04 03:52:31   dir-0001/file-0004.bin           dbba2dff
FX        1003 2023-01-04 03:52:31   dir-0001/file-0003.bin           243ce2b9
FX        1002 2023-01-04 03:52:31   dir-0001/file-0002.bin           3b062e10
FX        1001 2023-01-04 03:52:31   dir-0001/file-0001.bin           4e582703
FX         149 2023-01-04 03:52:32   dir-0001/.gitignore           2e9baffd
Total #: 6 Workspace Size:        5164 Cached Size:           0


```

If you add another set of files as hardlinks to the cached copies, it will
print the second letter as `H`.

```console
$ xvc file track dir-0002 --cache-type hardlink

$ xvc file list dir-0002
FX        1005 2023-01-04 03:52:31   dir-0002/file-0005.bin           00deb432
FX        1004 2023-01-04 03:52:31   dir-0002/file-0004.bin           8b1bcc78
FX        1003 2023-01-04 03:52:31   dir-0002/file-0003.bin           3ebe96b3
FX        1002 2023-01-04 03:52:31   dir-0002/file-0002.bin           a1009ead
FX        1001 2023-01-04 03:52:31   dir-0002/file-0001.bin           7015119d
FX         149 2023-01-04 03:52:32   dir-0002/.gitignore           53793b57
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
FX         149 2023-01-04 03:52:33   dir-0003/.gitignore           d10fc6f3
Total #: 6 Workspace Size:        1049 Cached Size:           0


```

Although not all filesystems support, `R` represents reflinks. 

### Sort options

You may sort `xvc file list` output by name, by modification time and by file
size. 