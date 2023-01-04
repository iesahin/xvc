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
FX        1005 2023-01-04 08:07:50   dir-0005/file-0005.bin           e23e79a0
FX        1004 2023-01-04 08:07:50   dir-0005/file-0004.bin           3640687a
FX        1003 2023-01-04 08:07:50   dir-0005/file-0003.bin           2856fe70
FX        1002 2023-01-04 08:07:50   dir-0005/file-0002.bin           8c079454
FX        1001 2023-01-04 08:07:50   dir-0005/file-0001.bin           189fa49f
DX         224 2023-01-04 08:07:50   dir-0005                   
FX        1005 2023-01-04 08:07:50   dir-0004/file-0005.bin           e23e79a0
FX        1004 2023-01-04 08:07:50   dir-0004/file-0004.bin           3640687a
FX        1003 2023-01-04 08:07:50   dir-0004/file-0003.bin           2856fe70
FX        1002 2023-01-04 08:07:50   dir-0004/file-0002.bin           8c079454
FX        1001 2023-01-04 08:07:50   dir-0004/file-0001.bin           189fa49f
DX         224 2023-01-04 08:07:50   dir-0004                   
FX        1005 2023-01-04 08:07:50   dir-0003/file-0005.bin           e23e79a0
FX        1004 2023-01-04 08:07:50   dir-0003/file-0004.bin           3640687a
FX        1003 2023-01-04 08:07:50   dir-0003/file-0003.bin           2856fe70
FX        1002 2023-01-04 08:07:50   dir-0003/file-0002.bin           8c079454
FX        1001 2023-01-04 08:07:50   dir-0003/file-0001.bin           189fa49f
DX         224 2023-01-04 08:07:50   dir-0003                   
FX        1005 2023-01-04 08:07:50   dir-0002/file-0005.bin           e23e79a0
FX        1004 2023-01-04 08:07:50   dir-0002/file-0004.bin           3640687a
FX        1003 2023-01-04 08:07:50   dir-0002/file-0003.bin           2856fe70
FX        1002 2023-01-04 08:07:50   dir-0002/file-0002.bin           8c079454
FX        1001 2023-01-04 08:07:50   dir-0002/file-0001.bin           189fa49f
DX         224 2023-01-04 08:07:50   dir-0002                   
FX        1005 2023-01-04 08:07:50   dir-0001/file-0005.bin           e23e79a0
FX        1004 2023-01-04 08:07:50   dir-0001/file-0004.bin           3640687a
FX        1003 2023-01-04 08:07:50   dir-0001/file-0003.bin           2856fe70
FX        1002 2023-01-04 08:07:50   dir-0001/file-0002.bin           8c079454
FX        1001 2023-01-04 08:07:50   dir-0001/file-0001.bin           189fa49f
DX         224 2023-01-04 08:07:50   dir-0001                   
FX         130 2023-01-04 08:07:50   .xvcignore           ac46bf74
FX         107 2023-01-04 08:07:50   .gitignore           ce9fcf30
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
FC        1005 2023-01-04 08:07:50   dir-0001/file-0005.bin  e23e79a0 e23e79a0
FC        1004 2023-01-04 08:07:50   dir-0001/file-0004.bin  3640687a 3640687a
FC        1003 2023-01-04 08:07:50   dir-0001/file-0003.bin  2856fe70 2856fe70
FC        1002 2023-01-04 08:07:50   dir-0001/file-0002.bin  8c079454 8c079454
FC        1001 2023-01-04 08:07:50   dir-0001/file-0001.bin  189fa49f 189fa49f
FX         149 2023-01-04 08:07:51   dir-0001/.gitignore           f8a3e270
Total #: 6 Workspace Size:        5164 Cached Size:        5015


```

If you add another set of files as hardlinks to the cached copies, it will
print the second letter as `H`.

```console
$ xvc file track dir-0002 --cache-type hardlink

$ xvc file list dir-0002
FH        1005 2023-01-04 08:07:50   dir-0002/file-0005.bin  e23e79a0 e23e79a0
FH        1004 2023-01-04 08:07:50   dir-0002/file-0004.bin  3640687a 3640687a
FH        1003 2023-01-04 08:07:50   dir-0002/file-0003.bin  2856fe70 2856fe70
FH        1002 2023-01-04 08:07:50   dir-0002/file-0002.bin  8c079454 8c079454
FH        1001 2023-01-04 08:07:50   dir-0002/file-0001.bin  189fa49f 189fa49f
FX         149 2023-01-04 08:07:52   dir-0002/.gitignore           7a499f10
Total #: 6 Workspace Size:        5164 Cached Size:        5015


```

Note, as hardlinks are actually files with the same inode in the file system
with alternative paths, they are detected as `F`. 

Symbolic links are typically reported as `SS` in the first letters. 
It means they are symbolic links on the file system and their cache type is also
symbolic links. 

```console
$ xvc file track dir-0003 --cache-type symlink
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
                "git.command": String(
                    "git",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "cache.type": String(
                    "copy",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "core.guid": String(
                    "fb48413aeb00aae5",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "pipeline.default": String(
                    "default",
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
                "git.command": String(
                    "git",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "core.guid": String(
                    "bee701fc631b3414",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "cache.type": String(
                    "copy",
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
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
            ),
        },
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
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
        "file.list.recursive": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "bee701fc631b3414",
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
        "cache.type": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
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
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
    },
    init_params: XvcConfigInitParams {
        default_configuration: "/n[core]/n# The repository id. Please do not delete or change it. /n# This is used to identify the repository and generate paths in storages. /n# In the future it may be used to in other ways. /nguid = /"fb48413aeb00aae5/"/n# Default verbosity level. /n# One of /"error/", /"warn/", /"info/"/nverbosity = /"error/"/n/n[git]/n# Automate git operations. /n# Turning this off leads Xvc to behave as if it's not in a Git repository./n# Not recommended unless you're really not using Git/nuse_git = true/n# Command to run Git process./n# You can set this to an absolute path to specify an executable/n# If set to a non-absolute path, the executable will be searched in $PATH./ncommand = /"git/"/n/n# Commit changes in .xvc/ directory after commands./n# You can set this to false if you want to commit manually. /nauto_commit = true/n/n# Stage changes in .xvc/ directory without committing./n# auto_commit implies auto_stage. /n# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true. /nauto_stage = false/n/n[cache]/n# The cache type for XVC. It may take copy, hardlink, symlink, reflink as values./n# The default is copy to make sure the options is portable./n# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file./n# Note that hardlink and symlink are read-only as they link the files in cache. /ntype = /"copy/"/n# The hash algorithm used for the cache. /n# It may take blake3, blake2, sha2 or sha3 as values. /n# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc./n# The cache path is produced by prepending algorithm name to the cache. /n# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc. /nalgorithm = /"blake3/"/n/n[file]/n/n[file.track]/n/n# Don't move file content to cache after xvc file track/nno_commit = false/n# Force to track files even if they are already tracked./nforce = false/n/n# Xvc calculates file content digest differently for text and binary files./n# This option controls whether to treat files as text or binary./n# It may take auto, text or binary as values./n# Auto check each file individually and treat it as text if it's text./ntext_or_binary = /"auto/"/n/n# Don't use parallelism in track operations. /n# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations./nno_parallel = false/n/n[file.list]/n/n# Format for `xvc file list` rows. You can reorder or remove columns./n# The following are the keys for each row: /n# - {acd64}:  actual content digest. All 64 digits from the workspace file's content./n# - {acd8}:  actual content digest. First 8 digits the file content digest. /n# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),/n#   symlink (S), hardlink (H) or reflink (R). /n# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,/n#   GB and TB to represent sizes larger than 1MB. /n# - {ats}:  actual timestamp. The timestamp of the workspace file./n# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show/n#   whether the file timestamp is the same as the cached timestamp, newer,/n#   older, not cached or not tracked./n# - {name}: The name of the file or directory./n# - {rcd64}:  recorded content digest. All 64 digits./n# - {rcd8}:  recorded content digest. First 8 digits./n# - {rct}:  recorded cache type. Whether the entry is linked to the workspace/n#   as a copy (C), symlink (S), hardlink (H) or reflink (R)./n# - {rsz}:  recorded size. The size of the cached content in bytes. It uses/n#   MB, GB and TB to represent sizes larged than 1MB./n# - {rts}:  recorded timestamp. The timestamp of the cached content./n# /n# There are no escape sequences in the format string. /n# If you want to add a tab, type it to the string./n# If you want to add a literal double curly brace, open an issue. /nformat = /"{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}/"/n/n# Default sort order for `xvc file list`./n# Valid values are/n# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc./nsort = /"name-desc/"/n/n# Do not show a summary for as the final row for `xvc file list`./nno_summary = false/n/n# List files recursively always./nrecursive = false/n/n[file.carry-in]/n# Carry-in the files to cache always, even if they are already present./nforce = false/n/n# Don't use parallel move/copy in carry-in/nno_parallel = false/n/n[pipeline]/n# Name of the current pipeline to run/ncurrent_pipeline = /"default/"/n# Name of the default pipeline/ndefault = /"default/"/n# Name of the default params file name/ndefault_params_file = /"params.yaml/"/n/n",
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
    "[CWD]/.xvc/ec/1672819670917611",
    "[CWD]/.xvc/ec/1672819670920916",
    "[CWD]/.xvc/ec/1672819671218848",
    "[CWD]/.xvc/ec/1672819671527057",
    "[CWD]/.xvc/ec/1672819671822766",
    "[CWD]/.xvc/ec/1672819672121993",
    "[CWD]/.xvc/ec/1672819672410806",
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
[TRACE][file/src/common/mod.rs:288] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][file/src/common/mod.rs:289] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs:305] all_paths: {
    XvcPath(
        "dir-0005/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672819670,
                tv_nsec: 777663436,
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
                tv_sec: 1672819671,
                tv_nsec: 526015916,
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
                tv_sec: 1672819670,
                tv_nsec: 777225565,
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
                tv_sec: 1672819670,
                tv_nsec: 776696236,
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
                tv_sec: 1672819670,
                tv_nsec: 776759152,
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
                tv_sec: 1672819670,
                tv_nsec: 777335105,
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
                tv_sec: 1672819672,
                tv_nsec: 119678028,
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
                tv_sec: 1672819670,
                tv_nsec: 776965067,
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
                tv_sec: 1672819670,
                tv_nsec: 776539529,
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
                tv_sec: 1672819670,
                tv_nsec: 776593362,
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
                tv_sec: 1672819670,
                tv_nsec: 776814027,
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
                tv_sec: 1672819670,
                tv_nsec: 777108607,
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
                tv_sec: 1672819670,
                tv_nsec: 777437729,
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
                tv_sec: 1672819670,
                tv_nsec: 776265365,
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
                tv_sec: 1672819670,
                tv_nsec: 776907817,
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
                tv_sec: 1672819671,
                tv_nsec: 524118433,
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
                tv_sec: 1672819670,
                tv_nsec: 776449738,
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
                tv_sec: 1672819670,
                tv_nsec: 777461479,
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
                tv_sec: 1672819670,
                tv_nsec: 777761185,
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
                tv_sec: 1672819670,
                tv_nsec: 777281147,
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
                tv_sec: 1672819670,
                tv_nsec: 776394405,
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
                tv_sec: 1672819670,
                tv_nsec: 777547145,
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
                tv_sec: 1672819670,
                tv_nsec: 777723435,
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
                tv_sec: 1672819670,
                tv_nsec: 777075149,
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
                tv_sec: 1672819670,
                tv_nsec: 777139982,
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
                tv_sec: 1672819670,
                tv_nsec: 777020691,
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
                tv_sec: 1672819670,
                tv_nsec: 917776309,
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
                tv_sec: 1672819672,
                tv_nsec: 119622695,
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
                tv_sec: 1672819670,
                tv_nsec: 776155741,
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
                tv_sec: 1672819670,
                tv_nsec: 777408771,
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
                tv_sec: 1672819670,
                tv_nsec: 776336531,
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
                tv_sec: 1672819670,
                tv_nsec: 777608353,
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
                tv_sec: 1672819670,
                tv_nsec: 917849183,
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
                tv_sec: 1672819670,
                tv_nsec: 777784143,
            },
        ),
    },
}
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 1 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs:327] glob_matcher: GlobSet {
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
                                    start_id: 154,
                                    max_pattern_len: 9,
                                    pattern_count: 1,
                                    state_count: 12,
                                    max_match: 28,
                                    heap_bytes: 1648,
                                    prefilter: Some(
                                        PrefilterObj(
                                            StartBytesOne {
                                                byte1: 100,
                                            },
                                        ),
                                    ),
                                    byte_classes: ByteClasses( 0 => [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44] 1 => [45] 2 => [46] 3 => [47] 4 => [48] 5 => [49, 50] 6 => [51] 7 => [52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99] 8 => [100] 9 => [101, 102, 103, 104] 10 => [105] 11 => [106, 107, 108, 109, 110, 111, 112, 113] 12 => [114] 13 => [115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255]),
                                    trans: [
                                        11,
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
                                        1,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        42,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        42,
                                        154,
                                        56,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        42,
                                        154,
                                        154,
                                        154,
                                        70,
                                        154,
                                        154,
                                        84,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        42,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        98,
                                        154,
                                        154,
                                        154,
                                        42,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        112,
                                        154,
                                        154,
                                        154,
                                        42,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        126,
                                        154,
                                        154,
                                        154,
                                        42,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        140,
                                        154,
                                        42,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        28,
                                        154,
                                        154,
                                        154,
                                        154,
                                        42,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
                                        42,
                                        154,
                                        154,
                                        154,
                                        154,
                                        154,
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
        "dir-0003/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672819670,
                tv_nsec: 776965067,
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
                tv_sec: 1672819670,
                tv_nsec: 776907817,
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
                tv_sec: 1672819670,
                tv_nsec: 777075149,
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
                tv_sec: 1672819670,
                tv_nsec: 777020691,
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
                tv_sec: 1672819670,
                tv_nsec: 777139982,
            },
        ),
    },
}
[TRACE][file/src/common/compare.rs:77] pmm: {
    XvcPath(
        "dir-0003/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672819670,
                tv_nsec: 776965067,
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
                tv_sec: 1672819670,
                tv_nsec: 776907817,
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
                tv_sec: 1672819670,
                tv_nsec: 777075149,
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
                tv_sec: 1672819670,
                tv_nsec: 777020691,
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
                tv_sec: 1672819670,
                tv_nsec: 777139982,
            },
        ),
    },
}
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0003/file-0002.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    12,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0003/file-0001.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    13,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0003/file-0004.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    14,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0003/file-0003.bin",
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
        "dir-0003/file-0002.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0003/file-0002.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0003/file-0003.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0003/file-0003.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0003/file-0001.bin",
    ),
}
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0002.bin",
)
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0003/file-0001.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0003.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0003/file-0004.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0003/file-0004.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0001.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0003/file-0005.bin",
    ),
}
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0004.bin",
)
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0003/file-0005.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0005.bin",
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                24,
                159,
                164,
                159,
                169,
                65,
                161,
                103,
                12,
                135,
                82,
                25,
                244,
                96,
                73,
                116,
                69,
                244,
                178,
                116,
                164,
                55,
                78,
                6,
                65,
                1,
                100,
                252,
                97,
                89,
                172,
                78,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                140,
                7,
                148,
                84,
                126,
                211,
                222,
                142,
                46,
                151,
                25,
                130,
                69,
                35,
                188,
                184,
                180,
                188,
                80,
                6,
                127,
                7,
                219,
                171,
                100,
                139,
                46,
                110,
                110,
                168,
                220,
                175,
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
                40,
                86,
                254,
                112,
                162,
                183,
                80,
                103,
                80,
                254,
                78,
                197,
                30,
                252,
                188,
                179,
                246,
                198,
                28,
                188,
                213,
                148,
                250,
                186,
                185,
                50,
                48,
                146,
                187,
                110,
                121,
                245,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                24,
                159,
                164,
                159,
                169,
                65,
                161,
                103,
                12,
                135,
                82,
                25,
                244,
                96,
                73,
                116,
                69,
                244,
                178,
                116,
                164,
                55,
                78,
                6,
                65,
                1,
                100,
                252,
                97,
                89,
                172,
                78,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                54,
                64,
                104,
                122,
                43,
                46,
                180,
                76,
                201,
                168,
                30,
                71,
                108,
                65,
                212,
                169,
                9,
                191,
                72,
                100,
                91,
                56,
                43,
                153,
                138,
                112,
                199,
                136,
                152,
                115,
                196,
                136,
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
                    24,
                    159,
                    164,
                    159,
                    169,
                    65,
                    161,
                    103,
                    12,
                    135,
                    82,
                    25,
                    244,
                    96,
                    73,
                    116,
                    69,
                    244,
                    178,
                    116,
                    164,
                    55,
                    78,
                    6,
                    65,
                    1,
                    100,
                    252,
                    97,
                    89,
                    172,
                    78,
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
                140,
                7,
                148,
                84,
                126,
                211,
                222,
                142,
                46,
                151,
                25,
                130,
                69,
                35,
                188,
                184,
                180,
                188,
                80,
                6,
                127,
                7,
                219,
                171,
                100,
                139,
                46,
                110,
                110,
                168,
                220,
                175,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                226,
                62,
                121,
                160,
                46,
                26,
                251,
                17,
                168,
                143,
                20,
                205,
                47,
                40,
                42,
                111,
                25,
                210,
                127,
                111,
                103,
                105,
                95,
                164,
                142,
                243,
                131,
                76,
                149,
                186,
                61,
                43,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:169] stored_content_digest: None
[TRACE][file/src/common/compare.rs:169] stored_content_digest: None
[TRACE][file/src/common/compare.rs:234] res: RecordMissing {
    actual: ContentDigest(
        Some(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    140,
                    7,
                    148,
                    84,
                    126,
                    211,
                    222,
                    142,
                    46,
                    151,
                    25,
                    130,
                    69,
                    35,
                    188,
                    184,
                    180,
                    188,
                    80,
                    6,
                    127,
                    7,
                    219,
                    171,
                    100,
                    139,
                    46,
                    110,
                    110,
                    168,
                    220,
                    175,
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
                40,
                86,
                254,
                112,
                162,
                183,
                80,
                103,
                80,
                254,
                78,
                197,
                30,
                252,
                188,
                179,
                246,
                198,
                28,
                188,
                213,
                148,
                250,
                186,
                185,
                50,
                48,
                146,
                187,
                110,
                121,
                245,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                54,
                64,
                104,
                122,
                43,
                46,
                180,
                76,
                201,
                168,
                30,
                71,
                108,
                65,
                212,
                169,
                9,
                191,
                72,
                100,
                91,
                56,
                43,
                153,
                138,
                112,
                199,
                136,
                152,
                115,
                196,
                136,
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
                    40,
                    86,
                    254,
                    112,
                    162,
                    183,
                    80,
                    103,
                    80,
                    254,
                    78,
                    197,
                    30,
                    252,
                    188,
                    179,
                    246,
                    198,
                    28,
                    188,
                    213,
                    148,
                    250,
                    186,
                    185,
                    50,
                    48,
                    146,
                    187,
                    110,
                    121,
                    245,
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
                226,
                62,
                121,
                160,
                46,
                26,
                251,
                17,
                168,
                143,
                20,
                205,
                47,
                40,
                42,
                111,
                25,
                210,
                127,
                111,
                103,
                105,
                95,
                164,
                142,
                243,
                131,
                76,
                149,
                186,
                61,
                43,
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
                    54,
                    64,
                    104,
                    122,
                    43,
                    46,
                    180,
                    76,
                    201,
                    168,
                    30,
                    71,
                    108,
                    65,
                    212,
                    169,
                    9,
                    191,
                    72,
                    100,
                    91,
                    56,
                    43,
                    153,
                    138,
                    112,
                    199,
                    136,
                    152,
                    115,
                    196,
                    136,
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
                    226,
                    62,
                    121,
                    160,
                    46,
                    26,
                    251,
                    17,
                    168,
                    143,
                    20,
                    205,
                    47,
                    40,
                    42,
                    111,
                    25,
                    210,
                    127,
                    111,
                    103,
                    105,
                    95,
                    164,
                    142,
                    243,
                    131,
                    76,
                    149,
                    186,
                    61,
                    43,
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
                            54,
                            64,
                            104,
                            122,
                            43,
                            46,
                            180,
                            76,
                            201,
                            168,
                            30,
                            71,
                            108,
                            65,
                            212,
                            169,
                            9,
                            191,
                            72,
                            100,
                            91,
                            56,
                            43,
                            153,
                            138,
                            112,
                            199,
                            136,
                            152,
                            115,
                            196,
                            136,
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
                            140,
                            7,
                            148,
                            84,
                            126,
                            211,
                            222,
                            142,
                            46,
                            151,
                            25,
                            130,
                            69,
                            35,
                            188,
                            184,
                            180,
                            188,
                            80,
                            6,
                            127,
                            7,
                            219,
                            171,
                            100,
                            139,
                            46,
                            110,
                            110,
                            168,
                            220,
                            175,
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
                            40,
                            86,
                            254,
                            112,
                            162,
                            183,
                            80,
                            103,
                            80,
                            254,
                            78,
                            197,
                            30,
                            252,
                            188,
                            179,
                            246,
                            198,
                            28,
                            188,
                            213,
                            148,
                            250,
                            186,
                            185,
                            50,
                            48,
                            146,
                            187,
                            110,
                            121,
                            245,
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
                            226,
                            62,
                            121,
                            160,
                            46,
                            26,
                            251,
                            17,
                            168,
                            143,
                            20,
                            205,
                            47,
                            40,
                            42,
                            111,
                            25,
                            210,
                            127,
                            111,
                            103,
                            105,
                            95,
                            164,
                            142,
                            243,
                            131,
                            76,
                            149,
                            186,
                            61,
                            43,
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
                            24,
                            159,
                            164,
                            159,
                            169,
                            65,
                            161,
                            103,
                            12,
                            135,
                            82,
                            25,
                            244,
                            96,
                            73,
                            116,
                            69,
                            244,
                            178,
                            116,
                            164,
                            55,
                            78,
                            6,
                            65,
                            1,
                            100,
                            252,
                            97,
                            89,
                            172,
                            78,
                        ],
                    },
                ),
            ),
        },
    },
}
[TRACE][file/src/common/mod.rs:540] records.len(): 10
[TRACE][file/src/common/mod.rs:542] new_store.len(): 15
[TRACE][file/src/common/mod.rs:540] records.len(): 10
[TRACE][file/src/common/mod.rs:542] new_store.len(): 15
[TRACE][file/src/common/mod.rs:540] records.len(): 10
[TRACE][file/src/common/mod.rs:542] new_store.len(): 15
[TRACE][file/src/common/mod.rs:540] records.len(): 10
[TRACE][file/src/common/mod.rs:542] new_store.len(): 15
[TRACE][file/src/common/mod.rs:540] records.len(): 10
[TRACE][file/src/common/mod.rs:542] new_store.len(): 15
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
[TRACE][file/src/track/mod.rs:421] dir_map: {}
[TRACE][file/src/track/mod.rs:456] file_map: {
    "[CWD]/dir-0003/file-0003.bin": "[CWD]/dir-0003/.gitignore",
    "[CWD]/dir-0003/file-0004.bin": "[CWD]/dir-0003/.gitignore",
    "[CWD]/dir-0003/file-0001.bin": "[CWD]/dir-0003/.gitignore",
    "[CWD]/dir-0003/file-0002.bin": "[CWD]/dir-0003/.gitignore",
    "[CWD]/dir-0003/file-0005.bin": "[CWD]/dir-0003/.gitignore",
}
[INFO][lib/src/cli/mod.rs:362] [INFO] [EXISTS] [CWD]/.xvc/b3/285/6fe/70a2b7506750fe4ec51efcbcb3f6c61cbcd594fabab9323092bb6e79f5/0.bin for dir-0003/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [EXISTS] [CWD]/.xvc/b3/364/068/7a2b2eb44cc9a81e476c41d4a909bf48645b382b998a70c7889873c488/0.bin for dir-0003/file-0004.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SKIP] [CWD]/dir-0003/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/285/6fe/70a2b7506750fe4ec51efcbcb3f6c61cbcd594fabab9323092bb6e79f5/0.bin -> dir-0003/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SKIP] [CWD]/dir-0003/file-0004.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/364/068/7a2b2eb44cc9a81e476c41d4a909bf48645b382b998a70c7889873c488/0.bin -> dir-0003/file-0004.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [EXISTS] [CWD]/.xvc/b3/e23/e79/a02e1afb11a88f14cd2f282a6f19d27f6f67695fa48ef3834c95ba3d2b/0.bin for dir-0003/file-0005.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [EXISTS] [CWD]/.xvc/b3/189/fa4/9fa941a1670c875219f460497445f4b274a4374e06410164fc6159ac4e/0.bin for dir-0003/file-0001.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SKIP] [CWD]/dir-0003/file-0005.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/e23/e79/a02e1afb11a88f14cd2f282a6f19d27f6f67695fa48ef3834c95ba3d2b/0.bin -> dir-0003/file-0005.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [EXISTS] [CWD]/.xvc/b3/8c0/794/547ed3de8e2e9719824523bcb8b4bc50067f07dbab648b2e6e6ea8dcaf/0.bin for dir-0003/file-0002.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SKIP] [CWD]/dir-0003/file-0001.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/189/fa4/9fa941a1670c875219f460497445f4b274a4374e06410164fc6159ac4e/0.bin -> dir-0003/file-0001.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SKIP] [CWD]/dir-0003/file-0002.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/8c0/794/547ed3de8e2e9719824523bcb8b4bc50067f07dbab648b2e6e6ea8dcaf/0.bin -> dir-0003/file-0002.bin
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
[DEBUG][lib/src/cli/mod.rs:516] Committing .xvc/ to git: [main b6f688f] Xvc auto-commit after ''
 7 files changed, 12 insertions(+)
 create mode 100644 .xvc/ec/1672819672707825
 create mode 100644 .xvc/store/cache-type-store/1672819672687693.json
 create mode 100644 .xvc/store/content-digest-store/1672819672688155.json
 create mode 100644 .xvc/store/file-text-or-binary-store/1672819672687900.json
 create mode 100644 .xvc/store/xvc-metadata-store/1672819672687412.json
 create mode 100644 .xvc/store/xvc-path-store/1672819672687023.json
 create mode 100644 dir-0003/.gitignore


$ xvc file list dir-0003
FS        1005 [..]   dir-0003/file-0005.bin  e23e79a0 e23e79a0
FS        1004 [..]   dir-0003/file-0004.bin  3640687a 3640687a
FS        1003 [..]   dir-0003/file-0003.bin  2856fe70 2856fe70
FS        1002 [..]   dir-0003/file-0002.bin  8c079454 8c079454
FS        1001 [..]   dir-0003/file-0001.bin  189fa49f 189fa49f
FX         149 [..]   dir-0003/.gitignore           [..]
Total #: 6 Workspace Size:        5164 Cached Size:        5015


```

Although not all filesystems support, `R` represents reflinks. 

### Sort options

You may sort `xvc file list` output by name, by modification time and by file
size. 
