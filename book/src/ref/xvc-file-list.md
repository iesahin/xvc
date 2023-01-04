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
FX        1005 2023-01-04 03:36:29   dir-0005/file-0005.bin           d7d79775
FX        1004 2023-01-04 03:36:29   dir-0005/file-0004.bin           c3e3bab7
FX        1003 2023-01-04 03:36:29   dir-0005/file-0003.bin           e0836d2d
FX        1002 2023-01-04 03:36:29   dir-0005/file-0002.bin           aff18dec
FX        1001 2023-01-04 03:36:29   dir-0005/file-0001.bin           7a3ee3ba
DX         224 2023-01-04 03:36:29   dir-0005                   
FX        1005 2023-01-04 03:36:29   dir-0004/file-0005.bin           099abc2b
FX        1004 2023-01-04 03:36:29   dir-0004/file-0004.bin           28743952
FX        1003 2023-01-04 03:36:29   dir-0004/file-0003.bin           0cef04e5
FX        1002 2023-01-04 03:36:28   dir-0004/file-0002.bin           3bff03fb
FX        1001 2023-01-04 03:36:28   dir-0004/file-0001.bin           504797ef
DX         224 2023-01-04 03:36:29   dir-0004                   
FX        1005 2023-01-04 03:36:28   dir-0003/file-0005.bin           06baaa70
FX        1004 2023-01-04 03:36:28   dir-0003/file-0004.bin           e4ee699f
FX        1003 2023-01-04 03:36:28   dir-0003/file-0003.bin           d9d20c79
FX        1002 2023-01-04 03:36:28   dir-0003/file-0002.bin           216a9bb4
FX        1001 2023-01-04 03:36:28   dir-0003/file-0001.bin           13d41ebb
DX         224 2023-01-04 03:36:28   dir-0003                   
FX        1005 2023-01-04 03:36:28   dir-0002/file-0005.bin           8101636d
FX        1004 2023-01-04 03:36:28   dir-0002/file-0004.bin           3feb82a3
FX        1003 2023-01-04 03:36:28   dir-0002/file-0003.bin           2833e2e8
FX        1002 2023-01-04 03:36:28   dir-0002/file-0002.bin           acbbaecf
FX        1001 2023-01-04 03:36:28   dir-0002/file-0001.bin           1eae21eb
DX         224 2023-01-04 03:36:28   dir-0002                   
FX        1005 2023-01-04 03:36:28   dir-0001/file-0005.bin           131ce7a0
FX        1004 2023-01-04 03:36:28   dir-0001/file-0004.bin           b05d4f90
FX        1003 2023-01-04 03:36:28   dir-0001/file-0003.bin           de0a3809
FX        1002 2023-01-04 03:36:28   dir-0001/file-0002.bin           ae6ab4e7
FX        1001 2023-01-04 03:36:28   dir-0001/file-0001.bin           fa5384a9
DX         224 2023-01-04 03:36:28   dir-0001                   
FX         130 2023-01-04 03:36:29   .xvcignore           ac46bf74
FX         107 2023-01-04 03:36:29   .gitignore           ce9fcf30
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
FX        1005 2023-01-04 03:36:28   dir-0001/file-0005.bin           131ce7a0
FX        1004 2023-01-04 03:36:28   dir-0001/file-0004.bin           b05d4f90
FX        1003 2023-01-04 03:36:28   dir-0001/file-0003.bin           de0a3809
FX        1002 2023-01-04 03:36:28   dir-0001/file-0002.bin           ae6ab4e7
FX        1001 2023-01-04 03:36:28   dir-0001/file-0001.bin           fa5384a9
FX         149 2023-01-04 03:36:29   dir-0001/.gitignore           d5a2c9f7
Total #: 6 Workspace Size:        5164 Cached Size:           0


```

If you add another set of files as hardlinks to the cached copies, it will
print the second letter as `H`.

```console
$ xvc file track dir-0002 --cache-type hardlink

$ xvc file list
FX        1005 2023-01-04 03:36:29   dir-0005/file-0005.bin           d7d79775
FX        1004 2023-01-04 03:36:29   dir-0005/file-0004.bin           c3e3bab7
FX        1003 2023-01-04 03:36:29   dir-0005/file-0003.bin           e0836d2d
FX        1002 2023-01-04 03:36:29   dir-0005/file-0002.bin           aff18dec
FX        1001 2023-01-04 03:36:29   dir-0005/file-0001.bin           7a3ee3ba
DX         224 2023-01-04 03:36:29   dir-0005                   
FX        1005 2023-01-04 03:36:29   dir-0004/file-0005.bin           099abc2b
FX        1004 2023-01-04 03:36:29   dir-0004/file-0004.bin           28743952
FX        1003 2023-01-04 03:36:29   dir-0004/file-0003.bin           0cef04e5
FX        1002 2023-01-04 03:36:28   dir-0004/file-0002.bin           3bff03fb
FX        1001 2023-01-04 03:36:28   dir-0004/file-0001.bin           504797ef
DX         224 2023-01-04 03:36:29   dir-0004                   
FX        1005 2023-01-04 03:36:28   dir-0003/file-0005.bin           06baaa70
FX        1004 2023-01-04 03:36:28   dir-0003/file-0004.bin           e4ee699f
FX        1003 2023-01-04 03:36:28   dir-0003/file-0003.bin           d9d20c79
FX        1002 2023-01-04 03:36:28   dir-0003/file-0002.bin           216a9bb4
FX        1001 2023-01-04 03:36:28   dir-0003/file-0001.bin           13d41ebb
DX         224 2023-01-04 03:36:28   dir-0003                   
FH        1005 2023-01-04 03:36:28   dir-0002/file-0005.bin  8101636d 8101636d
FH        1004 2023-01-04 03:36:28   dir-0002/file-0004.bin  3feb82a3 3feb82a3
FH        1003 2023-01-04 03:36:28   dir-0002/file-0003.bin  2833e2e8 2833e2e8
FH        1002 2023-01-04 03:36:28   dir-0002/file-0002.bin  acbbaecf acbbaecf
FH        1001 2023-01-04 03:36:28   dir-0002/file-0001.bin  1eae21eb 1eae21eb
FX         149 2023-01-04 03:36:30   dir-0002/.gitignore           52a61206
DX         256 2023-01-04 03:36:30   dir-0002                   
FC        1005 2023-01-04 03:36:28   dir-0001/file-0005.bin  131ce7a0 131ce7a0
FC        1004 2023-01-04 03:36:28   dir-0001/file-0004.bin  b05d4f90 b05d4f90
FC        1003 2023-01-04 03:36:28   dir-0001/file-0003.bin  de0a3809 de0a3809
FC        1002 2023-01-04 03:36:28   dir-0001/file-0002.bin  ae6ab4e7 ae6ab4e7
FC        1001 2023-01-04 03:36:28   dir-0001/file-0001.bin  fa5384a9 fa5384a9
FX         149 2023-01-04 03:36:29   dir-0001/.gitignore           d5a2c9f7
DX         256 2023-01-04 03:36:29   dir-0001                   
FX         130 2023-01-04 03:36:29   .xvcignore           ac46bf74
FX         107 2023-01-04 03:36:29   .gitignore           ce9fcf30
Total #: 34 Workspace Size:       26794 Cached Size:       10030


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
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "cache.type": String(
                    "copy",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "core.verbosity": String(
                    "error",
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
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.guid": String(
                    "e3389c8828254919",
                ),
                "git.command": String(
                    "git",
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "pipeline.current_pipeline": String(
                    "default",
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
                "git.command": String(
                    "git",
                ),
                "cache.type": String(
                    "copy",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.guid": String(
                    "e36e24723272f94e",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
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
        "file.track.no_parallel": XvcConfigValue {
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
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "e36e24723272f94e",
            ),
        },
        "cache.type": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
            ),
        },
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "debug",
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
        "file.carry-in.force": XvcConfigValue {
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
        "file.track.no_commit": XvcConfigValue {
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
        "file.carry-in.no_parallel": XvcConfigValue {
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
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}",
            ),
        },
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
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
        default_configuration: "/n[core]/n# The repository id. Please do not delete or change it. /n# This is used to identify the repository and generate paths in storages. /n# In the future it may be used to in other ways. /nguid = /"e3389c8828254919/"/n# Default verbosity level. /n# One of /"error/", /"warn/", /"info/"/nverbosity = /"error/"/n/n[git]/n# Automate git operations. /n# Turning this off leads Xvc to behave as if it's not in a Git repository./n# Not recommended unless you're really not using Git/nuse_git = true/n# Command to run Git process./n# You can set this to an absolute path to specify an executable/n# If set to a non-absolute path, the executable will be searched in $PATH./ncommand = /"git/"/n/n# Commit changes in .xvc/ directory after commands./n# You can set this to false if you want to commit manually. /nauto_commit = true/n/n# Stage changes in .xvc/ directory without committing./n# auto_commit implies auto_stage. /n# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true. /nauto_stage = false/n/n[cache]/n# The cache type for XVC. It may take copy, hardlink, symlink, reflink as values./n# The default is copy to make sure the options is portable./n# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file./n# Note that hardlink and symlink are read-only as they link the files in cache. /ntype = /"copy/"/n# The hash algorithm used for the cache. /n# It may take blake3, blake2, sha2 or sha3 as values. /n# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc./n# The cache path is produced by prepending algorithm name to the cache. /n# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc. /nalgorithm = /"blake3/"/n/n[file]/n/n[file.track]/n/n# Don't move file content to cache after xvc file track/nno_commit = false/n# Force to track files even if they are already tracked./nforce = false/n/n# Xvc calculates file content digest differently for text and binary files./n# This option controls whether to treat files as text or binary./n# It may take auto, text or binary as values./n# Auto check each file individually and treat it as text if it's text./ntext_or_binary = /"auto/"/n/n# Don't use parallelism in track operations. /n# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations./nno_parallel = false/n/n[file.list]/n/n# Format for `xvc file list` rows. You can reorder or remove columns./n# The following are the keys for each row: /n# - {acd64}:  actual content digest. All 64 digits from the workspace file's content./n# - {acd8}:  actual content digest. First 8 digits the file content digest. /n# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),/n#   symlink (S), hardlink (H) or reflink (R). /n# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,/n#   GB and TB to represent sizes larger than 1MB. /n# - {ats}:  actual timestamp. The timestamp of the workspace file./n# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show/n#   whether the file timestamp is the same as the cached timestamp, newer,/n#   older, not cached or not tracked./n# - {name}: The name of the file or directory./n# - {rcd64}:  recorded content digest. All 64 digits./n# - {rcd8}:  recorded content digest. First 8 digits./n# - {rct}:  recorded cache type. Whether the entry is linked to the workspace/n#   as a copy (C), symlink (S), hardlink (H) or reflink (R)./n# - {rsz}:  recorded size. The size of the cached content in bytes. It uses/n#   MB, GB and TB to represent sizes larged than 1MB./n# - {rts}:  recorded timestamp. The timestamp of the cached content./n# /n# There are no escape sequences in the format string. /n# If you want to add a tab, type it to the string./n# If you want to add a literal double curly brace, open an issue. /nformat = /"{{aft}}{{rct}} {{asz}} {{ats}}   {{name}}  {{rcd8}} {{acd8}}/"/n/n# Default sort order for `xvc file list`./n# Valid values are/n# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc./nsort = /"name-desc/"/n/n# Do not show a summary for as the final row for `xvc file list`./nno_summary = false/n/n# List files recursively always./nrecursive = false/n/n[file.carry-in]/n# Carry-in the files to cache always, even if they are already present./nforce = false/n/n# Don't use parallel move/copy in carry-in/nno_parallel = false/n/n[pipeline]/n# Name of the current pipeline to run/ncurrent_pipeline = /"default/"/n# Name of the default pipeline/ndefault = /"default/"/n# Name of the default params file name/ndefault_params_file = /"params.yaml/"/n/n",
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
    "[CWD]/.xvc/ec/1672803389129138",
    "[CWD]/.xvc/ec/1672803389131980",
    "[CWD]/.xvc/ec/1672803389426964",
    "[CWD]/.xvc/ec/1672803389730001",
    "[CWD]/.xvc/ec/1672803390024465",
    "[CWD]/.xvc/ec/1672803390325478",
    "[CWD]/.xvc/ec/1672803390615882",
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
        "dir-0001/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803388,
                tv_nsec: 995803101,
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
                tv_sec: 1672803388,
                tv_nsec: 998569672,
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
                tv_sec: 1672803388,
                tv_nsec: 998399587,
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
                tv_sec: 1672803388,
                tv_nsec: 999337597,
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
                tv_sec: 1672803389,
                tv_nsec: 5703626,
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
                tv_sec: 1672803389,
                tv_nsec: 5857169,
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
                tv_sec: 1672803388,
                tv_nsec: 999694935,
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
                tv_sec: 1672803390,
                tv_nsec: 323678204,
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
                tv_sec: 1672803390,
                tv_nsec: 322183896,
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
                tv_sec: 1672803388,
                tv_nsec: 998163168,
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
                tv_sec: 1672803389,
                tv_nsec: 5176495,
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
                tv_sec: 1672803389,
                tv_nsec: 2168712,
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
                tv_sec: 1672803388,
                tv_nsec: 995653474,
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
                tv_sec: 1672803389,
                tv_nsec: 726371302,
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
                tv_sec: 1672803388,
                tv_nsec: 995495223,
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
                tv_sec: 1672803388,
                tv_nsec: 995947727,
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
                tv_sec: 1672803389,
                tv_nsec: 4938,
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
                tv_sec: 1672803388,
                tv_nsec: 999195721,
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
                tv_sec: 1672803388,
                tv_nsec: 998986760,
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
                tv_sec: 1672803389,
                tv_nsec: 5410789,
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
                tv_sec: 1672803388,
                tv_nsec: 995276470,
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
                tv_sec: 1672803389,
                tv_nsec: 129283110,
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
                tv_sec: 1672803388,
                tv_nsec: 999850020,
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
                tv_sec: 1672803388,
                tv_nsec: 996623526,
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
                tv_sec: 1672803388,
                tv_nsec: 999375639,
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
                tv_sec: 1672803389,
                tv_nsec: 3333475,
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
                tv_sec: 1672803388,
                tv_nsec: 997188240,
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
                tv_sec: 1672803389,
                tv_nsec: 129353027,
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
                tv_sec: 1672803389,
                tv_nsec: 5632875,
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
                tv_sec: 1672803389,
                tv_nsec: 4849366,
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
                tv_sec: 1672803389,
                tv_nsec: 2033127,
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
                tv_sec: 1672803389,
                tv_nsec: 727411147,
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
                tv_sec: 1672803388,
                tv_nsec: 998794258,
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
                tv_sec: 1672803388,
                tv_nsec: 999495974,
            },
        ),
    },
}
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs:431] built glob set; 0 literals, 0 basenames, 0 extensions, 1 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/track/mod.rs:125] targets: {
    XvcPath(
        "dir-0003/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803388,
                tv_nsec: 998794258,
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
                tv_sec: 1672803388,
                tv_nsec: 999195721,
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
                tv_sec: 1672803388,
                tv_nsec: 998986760,
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
                tv_sec: 1672803388,
                tv_nsec: 999337597,
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
                tv_sec: 1672803388,
                tv_nsec: 999495974,
            },
        ),
    },
}
[TRACE][file/src/common/compare.rs:77] pmm: {
    XvcPath(
        "dir-0003/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            1001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1672803388,
                tv_nsec: 998794258,
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
                tv_sec: 1672803388,
                tv_nsec: 999195721,
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
                tv_sec: 1672803388,
                tv_nsec: 998986760,
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
                tv_sec: 1672803388,
                tv_nsec: 999337597,
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
                tv_sec: 1672803388,
                tv_nsec: 999495974,
            },
        ),
    },
}
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0003/file-0001.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    12,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0003/file-0003.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    13,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0003/file-0002.bin",
)
[TRACE][ecs/src/ecs/hstore.rs:111] key: XvcEntity(
    14,
)
[TRACE][ecs/src/ecs/hstore.rs:106] value: XvcPath(
    "dir-0003/file-0004.bin",
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
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0002.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0003/file-0004.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0003/file-0004.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0003/file-0001.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0003/file-0001.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0003/file-0003.bin",
    ),
}
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0004.bin",
)
[TRACE][file/src/common/compare.rs:186] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0003/file-0005.bin",
    ),
}
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0003/file-0005.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0001.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0005.bin",
)
[TRACE][file/src/common/compare.rs:228] actual: XvcPath(
    "dir-0003/file-0003.bin",
)
[TRACE][file/src/common/compare.rs:230] path: AbsolutePath(
    "[CWD]/dir-0003/file-0003.bin",
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                33,
                106,
                155,
                180,
                204,
                238,
                41,
                164,
                10,
                177,
                152,
                55,
                245,
                183,
                246,
                16,
                234,
                231,
                134,
                78,
                128,
                83,
                47,
                38,
                124,
                38,
                61,
                235,
                24,
                89,
                113,
                108,
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
                33,
                106,
                155,
                180,
                204,
                238,
                41,
                164,
                10,
                177,
                152,
                55,
                245,
                183,
                246,
                16,
                234,
                231,
                134,
                78,
                128,
                83,
                47,
                38,
                124,
                38,
                61,
                235,
                24,
                89,
                113,
                108,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:232] actual_digest: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                228,
                238,
                105,
                159,
                116,
                85,
                238,
                166,
                93,
                10,
                136,
                29,
                135,
                237,
                249,
                225,
                134,
                37,
                252,
                178,
                134,
                172,
                195,
                65,
                18,
                193,
                117,
                210,
                8,
                162,
                173,
                12,
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
                6,
                186,
                170,
                112,
                137,
                43,
                53,
                19,
                210,
                214,
                65,
                214,
                238,
                104,
                38,
                13,
                118,
                130,
                3,
                74,
                43,
                126,
                36,
                17,
                184,
                119,
                83,
                182,
                167,
                86,
                108,
                76,
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
                    33,
                    106,
                    155,
                    180,
                    204,
                    238,
                    41,
                    164,
                    10,
                    177,
                    152,
                    55,
                    245,
                    183,
                    246,
                    16,
                    234,
                    231,
                    134,
                    78,
                    128,
                    83,
                    47,
                    38,
                    124,
                    38,
                    61,
                    235,
                    24,
                    89,
                    113,
                    108,
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
                228,
                238,
                105,
                159,
                116,
                85,
                238,
                166,
                93,
                10,
                136,
                29,
                135,
                237,
                249,
                225,
                134,
                37,
                252,
                178,
                134,
                172,
                195,
                65,
                18,
                193,
                117,
                210,
                8,
                162,
                173,
                12,
            ],
        },
    ),
)
[TRACE][file/src/common/compare.rs:170] actual: ContentDigest(
    Some(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                6,
                186,
                170,
                112,
                137,
                43,
                53,
                19,
                210,
                214,
                65,
                214,
                238,
                104,
                38,
                13,
                118,
                130,
                3,
                74,
                43,
                126,
                36,
                17,
                184,
                119,
                83,
                182,
                167,
                86,
                108,
                76,
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
                    228,
                    238,
                    105,
                    159,
                    116,
                    85,
                    238,
                    166,
                    93,
                    10,
                    136,
                    29,
                    135,
                    237,
                    249,
                    225,
                    134,
                    37,
                    252,
                    178,
                    134,
                    172,
                    195,
                    65,
                    18,
                    193,
                    117,
                    210,
                    8,
                    162,
                    173,
                    12,
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
                    6,
                    186,
                    170,
                    112,
                    137,
                    43,
                    53,
                    19,
                    210,
                    214,
                    65,
                    214,
                    238,
                    104,
                    38,
                    13,
                    118,
                    130,
                    3,
                    74,
                    43,
                    126,
                    36,
                    17,
                    184,
                    119,
                    83,
                    182,
                    167,
                    86,
                    108,
                    76,
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
                19,
                212,
                30,
                187,
                49,
                122,
                193,
                203,
                222,
                51,
                245,
                137,
                151,
                122,
                8,
                24,
                177,
                184,
                59,
                43,
                96,
                25,
                253,
                62,
                160,
                140,
                44,
                129,
                108,
                234,
                86,
                99,
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
                19,
                212,
                30,
                187,
                49,
                122,
                193,
                203,
                222,
                51,
                245,
                137,
                151,
                122,
                8,
                24,
                177,
                184,
                59,
                43,
                96,
                25,
                253,
                62,
                160,
                140,
                44,
                129,
                108,
                234,
                86,
                99,
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
                    19,
                    212,
                    30,
                    187,
                    49,
                    122,
                    193,
                    203,
                    222,
                    51,
                    245,
                    137,
                    151,
                    122,
                    8,
                    24,
                    177,
                    184,
                    59,
                    43,
                    96,
                    25,
                    253,
                    62,
                    160,
                    140,
                    44,
                    129,
                    108,
                    234,
                    86,
                    99,
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
                217,
                210,
                12,
                121,
                239,
                204,
                231,
                245,
                69,
                247,
                123,
                162,
                58,
                240,
                143,
                88,
                124,
                239,
                40,
                79,
                108,
                12,
                214,
                229,
                50,
                31,
                216,
                131,
                141,
                242,
                228,
                103,
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
                217,
                210,
                12,
                121,
                239,
                204,
                231,
                245,
                69,
                247,
                123,
                162,
                58,
                240,
                143,
                88,
                124,
                239,
                40,
                79,
                108,
                12,
                214,
                229,
                50,
                31,
                216,
                131,
                141,
                242,
                228,
                103,
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
                    217,
                    210,
                    12,
                    121,
                    239,
                    204,
                    231,
                    245,
                    69,
                    247,
                    123,
                    162,
                    58,
                    240,
                    143,
                    88,
                    124,
                    239,
                    40,
                    79,
                    108,
                    12,
                    214,
                    229,
                    50,
                    31,
                    216,
                    131,
                    141,
                    242,
                    228,
                    103,
                ],
            },
        ),
    ),
}
[TRACE][file/src/track/mod.rs:188] content_digest_diff: HStore {
    map: {
        XvcEntity(
            16,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            6,
                            186,
                            170,
                            112,
                            137,
                            43,
                            53,
                            19,
                            210,
                            214,
                            65,
                            214,
                            238,
                            104,
                            38,
                            13,
                            118,
                            130,
                            3,
                            74,
                            43,
                            126,
                            36,
                            17,
                            184,
                            119,
                            83,
                            182,
                            167,
                            86,
                            108,
                            76,
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
                            228,
                            238,
                            105,
                            159,
                            116,
                            85,
                            238,
                            166,
                            93,
                            10,
                            136,
                            29,
                            135,
                            237,
                            249,
                            225,
                            134,
                            37,
                            252,
                            178,
                            134,
                            172,
                            195,
                            65,
                            18,
                            193,
                            117,
                            210,
                            8,
                            162,
                            173,
                            12,
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
                            217,
                            210,
                            12,
                            121,
                            239,
                            204,
                            231,
                            245,
                            69,
                            247,
                            123,
                            162,
                            58,
                            240,
                            143,
                            88,
                            124,
                            239,
                            40,
                            79,
                            108,
                            12,
                            214,
                            229,
                            50,
                            31,
                            216,
                            131,
                            141,
                            242,
                            228,
                            103,
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
                            19,
                            212,
                            30,
                            187,
                            49,
                            122,
                            193,
                            203,
                            222,
                            51,
                            245,
                            137,
                            151,
                            122,
                            8,
                            24,
                            177,
                            184,
                            59,
                            43,
                            96,
                            25,
                            253,
                            62,
                            160,
                            140,
                            44,
                            129,
                            108,
                            234,
                            86,
                            99,
                        ],
                    },
                ),
            ),
        },
        XvcEntity(
            14,
        ): RecordMissing {
            actual: ContentDigest(
                Some(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            33,
                            106,
                            155,
                            180,
                            204,
                            238,
                            41,
                            164,
                            10,
                            177,
                            152,
                            55,
                            245,
                            183,
                            246,
                            16,
                            234,
                            231,
                            134,
                            78,
                            128,
                            83,
                            47,
                            38,
                            124,
                            38,
                            61,
                            235,
                            24,
                            89,
                            113,
                            108,
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
    "[CWD]/dir-0003/file-0005.bin": "[CWD]/dir-0003/.gitignore",
    "[CWD]/dir-0003/file-0001.bin": "[CWD]/dir-0003/.gitignore",
    "[CWD]/dir-0003/file-0004.bin": "[CWD]/dir-0003/.gitignore",
    "[CWD]/dir-0003/file-0003.bin": "[CWD]/dir-0003/.gitignore",
    "[CWD]/dir-0003/file-0002.bin": "[CWD]/dir-0003/.gitignore",
}
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/06b/aaa/70892b3513d2d641d6ee68260d7682034a2b7e2411b87753b6a7566c4c"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/d9d/20c/79efcce7f545f77ba23af08f587cef284f6c0cd6e5321fd8838df2e467"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/216/a9b/b4ccee29a40ab19837f5b7f610eae7864e80532f267c263deb1859716c"
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/e4e/e69/9f7455eea65d0a881d87edf9e18625fcb286acc34112c175d208a2ad0c"
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0003/file-0005.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/06b/aaa/70892b3513d2d641d6ee68260d7682034a2b7e2411b87753b6a7566c4c/0.bin",
)
[TRACE][file/src/common/mod.rs:481] cache_dir: "[CWD]/.xvc/b3/13d/41e/bb317ac1cbde33f589977a0818b1b83b2b6019fd3ea08c2c816cea5663"
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0003/file-0003.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/d9d/20c/79efcce7f545f77ba23af08f587cef284f6c0cd6e5321fd8838df2e467/0.bin",
)
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0003/file-0002.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/216/a9b/b4ccee29a40ab19837f5b7f610eae7864e80532f267c263deb1859716c/0.bin",
)
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0003/file-0005.bin -> b3/06b/aaa/70892b3513d2d641d6ee68260d7682034a2b7e2411b87753b6a7566c4c/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0003/file-0003.bin -> b3/d9d/20c/79efcce7f545f77ba23af08f587cef284f6c0cd6e5321fd8838df2e467/0.bin
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0003/file-0005.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Symlink
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0003/file-0003.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Symlink
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0003/file-0004.bin",
)
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0003/file-0002.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/e4e/e69/9f7455eea65d0a881d87edf9e18625fcb286acc34112c175d208a2ad0c/0.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Symlink
[TRACE][file/src/common/mod.rs:483] path: AbsolutePath(
    "[CWD]/dir-0003/file-0001.bin",
)
[TRACE][file/src/common/mod.rs:484] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/13d/41e/bb317ac1cbde33f589977a0818b1b83b2b6019fd3ea08c2c816cea5663/0.bin",
)
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0003/file-0004.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Symlink
[TRACE][file/src/common/mod.rs:429] path: AbsolutePath(
    "[CWD]/dir-0003/file-0001.bin",
)
[TRACE][file/src/common/mod.rs:430] cache_type: Symlink
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0003/file-0002.bin -> b3/216/a9b/b4ccee29a40ab19837f5b7f610eae7864e80532f267c263deb1859716c/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SYMLINK] [CWD]/.xvc/b3/d9d/20c/79efcce7f545f77ba23af08f587cef284f6c0cd6e5321fd8838df2e467/0.bin -> [CWD]/dir-0003/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/d9d/20c/79efcce7f545f77ba23af08f587cef284f6c0cd6e5321fd8838df2e467/0.bin -> dir-0003/file-0003.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SYMLINK] [CWD]/.xvc/b3/06b/aaa/70892b3513d2d641d6ee68260d7682034a2b7e2411b87753b6a7566c4c/0.bin -> [CWD]/dir-0003/file-0005.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/06b/aaa/70892b3513d2d641d6ee68260d7682034a2b7e2411b87753b6a7566c4c/0.bin -> dir-0003/file-0005.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SYMLINK] [CWD]/.xvc/b3/216/a9b/b4ccee29a40ab19837f5b7f610eae7864e80532f267c263deb1859716c/0.bin -> [CWD]/dir-0003/file-0002.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/216/a9b/b4ccee29a40ab19837f5b7f610eae7864e80532f267c263deb1859716c/0.bin -> dir-0003/file-0002.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0003/file-0004.bin -> b3/e4e/e69/9f7455eea65d0a881d87edf9e18625fcb286acc34112c175d208a2ad0c/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [CARRY] dir-0003/file-0001.bin -> b3/13d/41e/bb317ac1cbde33f589977a0818b1b83b2b6019fd3ea08c2c816cea5663/0.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SYMLINK] [CWD]/.xvc/b3/e4e/e69/9f7455eea65d0a881d87edf9e18625fcb286acc34112c175d208a2ad0c/0.bin -> [CWD]/dir-0003/file-0004.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/e4e/e69/9f7455eea65d0a881d87edf9e18625fcb286acc34112c175d208a2ad0c/0.bin -> dir-0003/file-0004.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [SYMLINK] [CWD]/.xvc/b3/13d/41e/bb317ac1cbde33f589977a0818b1b83b2b6019fd3ea08c2c816cea5663/0.bin -> [CWD]/dir-0003/file-0001.bin
[INFO][lib/src/cli/mod.rs:362] [INFO] [RECHECK] b3/13d/41e/bb317ac1cbde33f589977a0818b1b83b2b6019fd3ea08c2c816cea5663/0.bin -> dir-0003/file-0001.bin
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
[DEBUG][lib/src/cli/mod.rs:516] Committing .xvc/ to git: [main 342153b] Xvc auto-commit after ''
 7 files changed, 12 insertions(+)
 create mode 100644 .xvc/ec/1672803390917376
 create mode 100644 .xvc/store/cache-type-store/1672803390896265.json
 create mode 100644 .xvc/store/content-digest-store/1672803390896730.json
 create mode 100644 .xvc/store/file-text-or-binary-store/1672803390896469.json
 create mode 100644 .xvc/store/xvc-metadata-store/1672803390895988.json
 create mode 100644 .xvc/store/xvc-path-store/1672803390895624.json
 create mode 100644 dir-0003/.gitignore


$ xvc file list
FX        1005 2023-01-04 03:36:29   dir-0005/file-0005.bin           d7d79775
FX        1004 2023-01-04 03:36:29   dir-0005/file-0004.bin           c3e3bab7
FX        1003 2023-01-04 03:36:29   dir-0005/file-0003.bin           e0836d2d
FX        1002 2023-01-04 03:36:29   dir-0005/file-0002.bin           aff18dec
FX        1001 2023-01-04 03:36:29   dir-0005/file-0001.bin           7a3ee3ba
DX         224 2023-01-04 03:36:29   dir-0005                   
FX        1005 2023-01-04 03:36:29   dir-0004/file-0005.bin           099abc2b
FX        1004 2023-01-04 03:36:29   dir-0004/file-0004.bin           28743952
FX        1003 2023-01-04 03:36:29   dir-0004/file-0003.bin           0cef04e5
FX        1002 2023-01-04 03:36:28   dir-0004/file-0002.bin           3bff03fb
FX        1001 2023-01-04 03:36:28   dir-0004/file-0001.bin           504797ef
DX         224 2023-01-04 03:36:29   dir-0004                   
SS         180 2023-01-04 03:36:30   dir-0003/file-0005.bin  06baaa70         
SS         180 2023-01-04 03:36:30   dir-0003/file-0004.bin  e4ee699f         
SS         180 2023-01-04 03:36:30   dir-0003/file-0003.bin  d9d20c79         
SS         180 2023-01-04 03:36:30   dir-0003/file-0002.bin  216a9bb4         
SS         180 2023-01-04 03:36:30   dir-0003/file-0001.bin  13d41ebb         
FX         149 2023-01-04 03:36:30   dir-0003/.gitignore           ad2a6a85
DX         256 2023-01-04 03:36:30   dir-0003                   
FH        1005 2023-01-04 03:36:28   dir-0002/file-0005.bin  8101636d 8101636d
FH        1004 2023-01-04 03:36:28   dir-0002/file-0004.bin  3feb82a3 3feb82a3
FH        1003 2023-01-04 03:36:28   dir-0002/file-0003.bin  2833e2e8 2833e2e8
FH        1002 2023-01-04 03:36:28   dir-0002/file-0002.bin  acbbaecf acbbaecf
FH        1001 2023-01-04 03:36:28   dir-0002/file-0001.bin  1eae21eb 1eae21eb
FX         149 2023-01-04 03:36:30   dir-0002/.gitignore           52a61206
DX         256 2023-01-04 03:36:30   dir-0002                   
FC        1005 2023-01-04 03:36:28   dir-0001/file-0005.bin  131ce7a0 131ce7a0
FC        1004 2023-01-04 03:36:28   dir-0001/file-0004.bin  b05d4f90 b05d4f90
FC        1003 2023-01-04 03:36:28   dir-0001/file-0003.bin  de0a3809 de0a3809
FC        1002 2023-01-04 03:36:28   dir-0001/file-0002.bin  ae6ab4e7 ae6ab4e7
FC        1001 2023-01-04 03:36:28   dir-0001/file-0001.bin  fa5384a9 fa5384a9
FX         149 2023-01-04 03:36:29   dir-0001/.gitignore           d5a2c9f7
DX         256 2023-01-04 03:36:29   dir-0001                   
FX         130 2023-01-04 03:36:29   .xvcignore           ac46bf74
FX         107 2023-01-04 03:36:29   .gitignore           ce9fcf30
Total #: 35 Workspace Size:       22860 Cached Size:       15045


```

Although not all filesystems support, `R` represents reflinks. 

### Sort options

You may sort `xvc file list` output by name, by modification time and by file
size. 