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
          
          The following are the keys for each row:
          
          - {{acd8}}:  actual content digest from the workspace file. First 8 digits.
          - {{acd64}}:  actual content digest. All 64 digits.
          - {{aft}}:  actual file type. Whether the entry is a file (F), directory (D),
            symlink (S), hardlink (H) or reflink (R).
          - {{asz}}:  actual size. The size of the workspace file in bytes. It uses MB,
            GB and TB to represent sizes larger than 1MB.
          - {{ats}}:  actual timestamp. The timestamp of the workspace file.
          - {{name}}: The name of the file or directory.
          - {{cst}}:  cache status. One of "=", ">", "<", "X", or "?" to show
            whether the file timestamp is the same as the cached timestamp, newer,
            older, not cached or not tracked.
          - {{rcd8}}:  recorded content digest stored in the cache. First 8 digits.
          - {{rcd64}}:  recorded content digest stored in the cache. All 64 digits.
          - {{rrm}}:  recorded recheck method. Whether the entry is linked to the workspace
            as a copy (C), symlink (S), hardlink (H) or reflink (R).
          - {{rsz}}:  recorded size. The size of the cached content in bytes. It uses
            MB, GB and TB to represent sizes larged than 1MB.
          - {{rts}}:  recorded timestamp. The timestamp of the cached content.
          
          The default format can be set with file.list.format in the config file.

  -s, --sort <SORT>
          Sort criteria.
          
          It can be one of none (default), name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
          
          The default option can be set with file.list.sort in the config file.

      --no-summary
          Don't show total number and size of the listed files.
          
          The default option can be set with file.list.no_summary in the config file.

  -a, --show-dot-files
          Don't hide dot files
          
          If not supplied, hides dot files like .gitignore and .xvcignore

  -h, --help
          Print help (see a summary with '-h')

```

## Examples

For these examples, we'll create a directory tree with five directories, each
having a file.

```console
$ xvc-test-helper create-directory-tree --directories 5 --files 5 --seed 20230213

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

[..] directories, 25 files

```

`xvc file list` command works only in Xvc repositories. As we didn't initialize
a repository yet, it reports an error.

```console
$ xvc file list
[ERROR] File Error: [E2004] Requires xvc repository.

```

Let's initialize the repository.

```console
$ git init
...

$ xvc init

```

Now it lists all files and directories.

```console
$ xvc file list --sort name-asc
DX         224 [..]                   dir-0001
FX        2001 [..]          1953f05d dir-0001/file-0001.bin
FX        2002 [..]          7e807161 dir-0001/file-0002.bin
FX        2003 [..]          d2432259 dir-0001/file-0003.bin
FX        2004 [..]          63535612 dir-0001/file-0004.bin
FX        2005 [..]          447933dc dir-0001/file-0005.bin
DX         224 [..]                   dir-0002
FX        2001 [..]          1953f05d dir-0002/file-0001.bin
FX        2002 [..]          7e807161 dir-0002/file-0002.bin
FX        2003 [..]          d2432259 dir-0002/file-0003.bin
FX        2004 [..]          63535612 dir-0002/file-0004.bin
FX        2005 [..]          447933dc dir-0002/file-0005.bin
DX         224 [..]                   dir-0003
FX        2001 [..]          1953f05d dir-0003/file-0001.bin
FX        2002 [..]          7e807161 dir-0003/file-0002.bin
FX        2003 [..]          d2432259 dir-0003/file-0003.bin
FX        2004 [..]          63535612 dir-0003/file-0004.bin
FX        2005 [..]          447933dc dir-0003/file-0005.bin
DX         224 [..]                   dir-0004
FX        2001 [..]          1953f05d dir-0004/file-0001.bin
FX        2002 [..]          7e807161 dir-0004/file-0002.bin
FX        2003 [..]          d2432259 dir-0004/file-0003.bin
FX        2004 [..]          63535612 dir-0004/file-0004.bin
FX        2005 [..]          447933dc dir-0004/file-0005.bin
DX         224 [..]                   dir-0005
FX        2001 [..]          1953f05d dir-0005/file-0001.bin
FX        2002 [..]          7e807161 dir-0005/file-0002.bin
FX        2003 [..]          d2432259 dir-0005/file-0003.bin
FX        2004 [..]          63535612 dir-0005/file-0004.bin
FX        2005 [..]          447933dc dir-0005/file-0005.bin
Total #: 30 Workspace Size:       51195 Cached Size:           0


```

By default the command hides dotfiles. If you also want to show them, you can use `--show-dot-files`/`-a` flag. 

```console
$ xvc file list --sort name-asc --show-dot-files
[DEBUG][logging/src/lib.rs::237] Terminal logger enabled with level: Error
[DEBUG][logging/src/lib.rs::240] File logger enabled with level: Trace to "/var/folders/tk/3vn311ps4kqdhgykj3jg_p8r0000gn/T//xvc.log"
[TRACE][core/src/types/xvcroot.rs::247] "."
[DEBUG][core/src/types/xvcroot.rs::253] XVC DIR: "[CWD]"
[DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][config/src/lib.rs::527] env_config: {
    "TRYCMD_TESTS": String(
        "file",
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
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "core.guid": String(
                    "ac58365fd1e06ebf",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "pipeline.process_pool_size": Integer(
                    4,
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.list.show_dot_files": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.track.no_parallel": Boolean(
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
                "file.track.no_parallel": Boolean(
                    false,
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
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "core.guid": String(
                    "9190a7aea7c8508a",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.list.show_dot_files": Boolean(
                    false,
                ),
                "pipeline.process_pool_size": Integer(
                    4,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.command": String(
                    "git",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
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
            source: Local,
            map: {},
        },
        XvcConfigMap {
            source: Environment,
            map: {
                "TRYCMD_TESTS": String(
                    "file",
                ),
                "TRYCMD_DURATION": Integer(
                    300,
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
        "pipeline.process_pool_size": XvcConfigValue {
            source: Project,
            value: Integer(
                4,
            ),
        },
        "file.track.text_or_binary": XvcConfigValue {
            source: Project,
            value: String(
                "auto",
            ),
        },
        "core.quiet": XvcConfigValue {
            source: CommandLine,
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
        "file.carry-in.force": XvcConfigValue {
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
        "TRYCMD_TESTS": XvcConfigValue {
            source: Environment,
            value: String(
                "file",
            ),
        },
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "quiet",
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
        "TRYCMD_DURATION": XvcConfigValue {
            source: Environment,
            value: Integer(
                300,
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
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "9190a7aea7c8508a",
            ),
        },
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
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
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"ac58365fd1e06ebf/"
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
    "[CWD]/.xvc/ec/1703493614865519",
    "[CWD]/.xvc/ec/1703493614870034",
]
[TRACE][file/src/lib.rs::169] opts: XvcFileCLI {
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
            sort: Some(
                NameAsc,
            ),
            no_summary: false,
            show_dot_files: true,
            targets: None,
        },
    ),
}
[TRACE][file/src/common/mod.rs::193] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][file/src/common/mod.rs::194] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][walker/src/lib.rs::594] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::595] ignore_path: "[CWD]/.xvcignore"
[TRACE][walker/src/lib.rs::603] &content: "
# Add patterns of files xvc should ignore, which could improve
# the performance.
# It's in the same format as .gitignore files.

.DS_Store
"
[TRACE][walker/src/lib.rs::258] new_patterns: [
    Pattern {
        pattern: Glob {
            glob: "**/.DS_Store",
            re: "(?-u)^(?:/?|.*/)//.DS_Store$",
            opts: GlobOptions {
                case_insensitive: false,
                literal_separator: false,
                backslash_escape: true,
                empty_alternates: false,
            },
            tokens: Tokens(
                [
                    RecursivePrefix,
                    Literal(
                        '.',
                    ),
                    Literal(
                        'D',
                    ),
                    Literal(
                        'S',
                    ),
                    Literal(
                        '_',
                    ),
                    Literal(
                        'S',
                    ),
                    Literal(
                        't',
                    ),
                    Literal(
                        'o',
                    ),
                    Literal(
                        'r',
                    ),
                    Literal(
                        'e',
                    ),
                ],
            ),
        },
        original: ".DS_Store",
        source: File {
            path: ".xvcignore",
            line: 6,
        },
        effect: Ignore,
        relativity: Anywhere,
        path_kind: Any,
    },
]
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 3 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][walker/src/lib.rs::260] ignore_rules: RwLock {
    data: IgnoreRules {
        root: "[CWD]",
        ignore_patterns: RwLock {
            data: [
                Pattern {
                    pattern: Glob {
                        glob: "**/.xvc",
                        re: "(?-u)^(?:/?|.*/)//.xvc$",
                        opts: GlobOptions {
                            case_insensitive: false,
                            literal_separator: false,
                            backslash_escape: true,
                            empty_alternates: false,
                        },
                        tokens: Tokens(
                            [
                                RecursivePrefix,
                                Literal(
                                    '.',
                                ),
                                Literal(
                                    'x',
                                ),
                                Literal(
                                    'v',
                                ),
                                Literal(
                                    'c',
                                ),
                            ],
                        ),
                    },
                    original: ".xvc",
                    source: Global,
                    effect: Ignore,
                    relativity: Anywhere,
                    path_kind: Any,
                },
                Pattern {
                    pattern: Glob {
                        glob: "**/.git",
                        re: "(?-u)^(?:/?|.*/)//.git$",
                        opts: GlobOptions {
                            case_insensitive: false,
                            literal_separator: false,
                            backslash_escape: true,
                            empty_alternates: false,
                        },
                        tokens: Tokens(
                            [
                                RecursivePrefix,
                                Literal(
                                    '.',
                                ),
                                Literal(
                                    'g',
                                ),
                                Literal(
                                    'i',
                                ),
                                Literal(
                                    't',
                                ),
                            ],
                        ),
                    },
                    original: ".git",
                    source: Global,
                    effect: Ignore,
                    relativity: Anywhere,
                    path_kind: Any,
                },
                Pattern {
                    pattern: Glob {
                        glob: "**/.DS_Store",
                        re: "(?-u)^(?:/?|.*/)//.DS_Store$",
                        opts: GlobOptions {
                            case_insensitive: false,
                            literal_separator: false,
                            backslash_escape: true,
                            empty_alternates: false,
                        },
                        tokens: Tokens(
                            [
                                RecursivePrefix,
                                Literal(
                                    '.',
                                ),
                                Literal(
                                    'D',
                                ),
                                Literal(
                                    'S',
                                ),
                                Literal(
                                    '_',
                                ),
                                Literal(
                                    'S',
                                ),
                                Literal(
                                    't',
                                ),
                                Literal(
                                    'o',
                                ),
                                Literal(
                                    'r',
                                ),
                                Literal(
                                    'e',
                                ),
                            ],
                        ),
                    },
                    original: ".DS_Store",
                    source: File {
                        path: ".xvcignore",
                        line: 6,
                    },
                    effect: Ignore,
                    relativity: Anywhere,
                    path_kind: Any,
                },
            ],
            poisoned: false,
            ..
        },
        whitelist_patterns: RwLock {
            data: [],
            poisoned: false,
            ..
        },
        whitelist_set: RwLock {
            data: GlobSet {
                len: 0,
                strats: [],
            },
            poisoned: false,
            ..
        },
        ignore_set: RwLock {
            data: GlobSet {
                len: 3,
                strats: [
                    Extension(
                        ExtensionStrategy(
                            {},
                        ),
                    ),
                    BasenameLiteral(
                        BasenameLiteralStrategy(
                            {
                                [
                                    46,
                                    68,
                                    83,
                                    95,
                                    83,
                                    116,
                                    111,
                                    114,
                                    101,
                                ]: [
                                    2,
                                ],
                                [
                                    46,
                                    103,
                                    105,
                                    116,
                                ]: [
                                    1,
                                ],
                                [
                                    46,
                                    120,
                                    118,
                                    99,
                                ]: [
                                    0,
                                ],
                            },
                        ),
                    ),
                    Literal(
                        LiteralStrategy(
                            {},
                        ),
                    ),
                    Suffix(
                        SuffixStrategy {
                            matcher: AhoCorasick(
                                dfa::DFA(
                                D 000000: /x00 => 0
                                F 000001:
                                 >000002: /x00 => 2
                                  000003: /x00 => 0
                                match kind: Standard
                                prefilter: false
                                state length: 4
                                pattern length: 0
                                shortest pattern length: 18446744073709551615
                                longest pattern length: 0
                                alphabet length: 1
                                stride: 1
                                byte classes: ByteClasses(0 => [0-255])
                                memory usage: 16
                                )
                                ,
                            ),
                            map: [],
                            longest: 0,
                        },
                    ),
                    Prefix(
                        PrefixStrategy {
                            matcher: AhoCorasick(
                                dfa::DFA(
                                D 000000: /x00 => 0
                                F 000001:
                                 >000002: /x00 => 2
                                  000003: /x00 => 0
                                match kind: Standard
                                prefilter: false
                                state length: 4
                                pattern length: 0
                                shortest pattern length: 18446744073709551615
                                longest pattern length: 0
                                alphabet length: 1
                                stride: 1
                                byte classes: ByteClasses(0 => [0-255])
                                memory usage: 16
                                )
                                ,
                            ),
                            map: [],
                            longest: 0,
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
            },
            poisoned: false,
            ..
        },
    },
    poisoned: false,
    ..
}
[TRACE][walker/src/lib.rs::271] child_paths: [
    PathMetadata {
        path: "[CWD]/dir-0001",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 16877,
                },
            ),
            is_dir: true,
            is_file: false,
            permissions: Permissions(
                FilePermissions {
                    mode: 16877,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 760025351,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 771678796,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 758664289,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/.xvc",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 16877,
                },
            ),
            is_dir: true,
            is_file: false,
            permissions: Permissions(
                FilePermissions {
                    mode: 16877,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 865669339,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 927112137,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 858561449,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/.gitignore",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 866280538,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 994790921,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 865779379,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0005",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 16877,
                },
            ),
            is_dir: true,
            is_file: false,
            permissions: Permissions(
                FilePermissions {
                    mode: 16877,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 766181381,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 778090405,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 765316686,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0002",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 16877,
                },
            ),
            is_dir: true,
            is_file: false,
            permissions: Permissions(
                FilePermissions {
                    mode: 16877,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 762368732,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 772179955,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 760351680,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0003",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 16877,
                },
            ),
            is_dir: true,
            is_file: false,
            permissions: Permissions(
                FilePermissions {
                    mode: 16877,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 763953916,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 777816617,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 762893640,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0004",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 16877,
                },
            ),
            is_dir: true,
            is_file: false,
            permissions: Permissions(
                FilePermissions {
                    mode: 16877,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 765118606,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 777961073,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 764181412,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/.git",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 16877,
                },
            ),
            is_dir: true,
            is_file: false,
            permissions: Permissions(
                FilePermissions {
                    mode: 16877,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 470502257,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 810634442,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 810634442,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/.xvcignore",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 865745046,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 995075875,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 865700089,
                },
            ),
            ..
        },
    },
]
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0001"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0001"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0001"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvc"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvc"
[TRACE][walker/src/lib.rs::288] child_path.path: "[CWD]/.xvc"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.gitignore"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.gitignore"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/.gitignore"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0005"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0005"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0005"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0002"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0002"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0002"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0003"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0003"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0003"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0004"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0004"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0004"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.git"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.git"
[TRACE][walker/src/lib.rs::288] child_path.path: "[CWD]/.git"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/.xvcignore"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/.xvcignore"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/.xvcignore"
[TRACE][walker/src/lib.rs::338] path_sender: Sender { .. }
[TRACE][walker/src/lib.rs::338] path_sender: Sender { .. }
[TRACE][walker/src/lib.rs::339] ignore_sender: Sender { .. }
[TRACE][walker/src/lib.rs::338] path_sender: Sender { .. }
[TRACE][walker/src/lib.rs::339] ignore_sender: Sender { .. }
[TRACE][walker/src/lib.rs::338] path_sender: Sender { .. }
[TRACE][walker/src/lib.rs::338] path_sender: Sender { .. }
[TRACE][walker/src/lib.rs::339] ignore_sender: Sender { .. }
[TRACE][walker/src/lib.rs::338] path_sender: Sender { .. }
[TRACE][walker/src/lib.rs::339] ignore_sender: Sender { .. }
[TRACE][walker/src/lib.rs::339] ignore_sender: Sender { .. }
[TRACE][walker/src/lib.rs::271] child_paths: [
    PathMetadata {
        path: "[CWD]/dir-0001/file-0002.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 759469818,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 442364068,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 759297446,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0001/file-0003.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 759689731,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 456635514,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 759501568,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0001/file-0001.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 759253738,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 456049231,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 758824995,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0001/file-0004.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 759969186,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 441885867,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 759732439,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0001/file-0005.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 760303055,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 455956274,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 760015935,
                },
            ),
            ..
        },
    },
]
[TRACE][walker/src/lib.rs::271] child_paths: [
    PathMetadata {
        path: "[CWD]/dir-0002/file-0002.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 761451579,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 455593947,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 761173875,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0002/file-0003.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 761796032,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 455857526,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 761490870,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0002/file-0001.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 761121043,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 443474635,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 760418054,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0002/file-0004.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 762306483,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 455685320,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 761857031,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0002/file-0005.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 762826266,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 456135688,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 762348107,
                },
            ),
            ..
        },
    },
]
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0002/file-0002.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0002/file-0002.bin"
[TRACE][walker/src/lib.rs::271] child_paths: [
    PathMetadata {
        path: "[CWD]/dir-0003/file-0002.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 763414382,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 456882510,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 763181344,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0003/file-0003.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 763664795,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 456389268,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 763455965,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0003/file-0001.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 763139470,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 456550599,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 762943931,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0003/file-0004.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 763904750,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 441998116,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 763705128,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0003/file-0005.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 764147162,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 456470516,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 763945291,
                },
            ),
            ..
        },
    },
]
[TRACE][walker/src/lib.rs::338] path_sender: Sender { .. }
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0002/file-0002.bin"
[TRACE][walker/src/lib.rs::338] path_sender: Sender { .. }
[TRACE][walker/src/lib.rs::271] child_paths: [
    PathMetadata {
        path: "[CWD]/dir-0004/file-0002.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 764633613,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 456300061,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 764453575,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0004/file-0003.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 764841568,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 457056549,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 764665530,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0004/file-0001.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 764412367,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 456795928,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 764228870,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0004/file-0004.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 765068398,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 441734828,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 764882026,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0004/file-0005.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 765283812,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 455495656,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 765104564,
                },
            ),
            ..
        },
    },
]
[TRACE][walker/src/lib.rs::271] child_paths: [
    PathMetadata {
        path: "[CWD]/dir-0005/file-0002.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 765734805,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 443676090,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 765564099,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0005/file-0003.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 765945968,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 456215479,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 765768512,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0005/file-0001.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 765529891,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 455772652,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 765357644,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0005/file-0004.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 766149465,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 456963967,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 765979801,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/dir-0005/file-0005.bin",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33188,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33188,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 766346295,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1703493615,
                    tv_nsec: 456714763,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1703493614,
                    tv_nsec: 766178339,
                },
            ),
            ..
        },
    },
]
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0003/file-0002.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::339] ignore_sender: Sender { .. }
[TRACE][walker/src/lib.rs::339] ignore_sender: Sender { .. }
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0001/file-0002.bin"
[TRACE][walker/src/lib.rs::358] "End of thread {}": "End of thread {}"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::773] path: "/dir-0003/file-0002.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::358] "End of thread {}": "End of thread {}"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::339] ignore_sender: Sender { .. }
[TRACE][walker/src/lib.rs::358] thread_i: 7
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0005/file-0002.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0004/file-0002.bin"
[TRACE][walker/src/lib.rs::358] thread_i: 3
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0002/file-0003.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0003/file-0002.bin"
[TRACE][walker/src/lib.rs::773] path: "/dir-0002/file-0003.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0003/file-0003.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0003/file-0003.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0003/file-0003.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0003/file-0001.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0003/file-0001.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0003/file-0001.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0003/file-0004.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0003/file-0004.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0003/file-0004.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::358] "End of thread {}": "End of thread {}"
[TRACE][walker/src/lib.rs::358] thread_i: 5
[TRACE][walker/src/lib.rs::773] path: "/dir-0005/file-0002.bin"
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0003/file-0005.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0005/file-0002.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0005/file-0003.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0005/file-0003.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0005/file-0003.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0005/file-0001.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0005/file-0001.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0005/file-0001.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0005/file-0004.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0005/file-0004.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0005/file-0004.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0005/file-0005.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0005/file-0005.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0005/file-0005.bin"
[TRACE][walker/src/lib.rs::358] "End of thread {}": "End of thread {}"
[TRACE][walker/src/lib.rs::358] thread_i: 2
[TRACE][walker/src/lib.rs::773] path: "/dir-0001/file-0002.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0004/file-0002.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0002/file-0003.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0002/file-0001.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0002/file-0001.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0004/file-0002.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0002/file-0001.bin"
[TRACE][walker/src/lib.rs::773] path: "/dir-0003/file-0005.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0003/file-0005.bin"
[TRACE][walker/src/lib.rs::358] "End of thread {}": "End of thread {}"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0004/file-0003.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::773] path: "/dir-0004/file-0003.bin"
[TRACE][walker/src/lib.rs::358] thread_i: 4
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0002/file-0004.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0002/file-0004.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0002/file-0004.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0001/file-0002.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0001/file-0003.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0004/file-0003.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::773] path: "/dir-0001/file-0003.bin"
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0004/file-0001.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0001/file-0003.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0002/file-0005.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0002/file-0005.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0002/file-0005.bin"
[TRACE][walker/src/lib.rs::358] "End of thread {}": "End of thread {}"
[TRACE][walker/src/lib.rs::358] thread_i: 6
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0004/file-0001.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0004/file-0001.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0004/file-0004.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0004/file-0004.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0004/file-0004.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0004/file-0005.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0004/file-0005.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0004/file-0005.bin"
[TRACE][walker/src/lib.rs::358] "End of thread {}": "End of thread {}"
[TRACE][walker/src/lib.rs::358] thread_i: 1
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0001/file-0001.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0001/file-0001.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0001/file-0001.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0001/file-0004.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0001/file-0004.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0001/file-0004.bin"
[TRACE][walker/src/lib.rs::745] is_abs: true
[TRACE][walker/src/lib.rs::749] path_str: "[CWD]/dir-0001/file-0005.bin"
[TRACE][walker/src/lib.rs::751] final_slash: false
[TRACE][walker/src/lib.rs::773] path: "/dir-0001/file-0005.bin"
[TRACE][walker/src/lib.rs::276] child_path.path: "[CWD]/dir-0001/file-0005.bin"
[TRACE][walker/src/lib.rs::358] "End of thread {}": "End of thread {}"
[TRACE][walker/src/lib.rs::358] thread_i: 0
[TRACE][walker/src/lib.rs::364] "End of walk_parallel": "End of walk_parallel"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/.gitignore"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0005"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0003"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0004"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/.xvcignore"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0003/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0003/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0003/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0003/file-0004.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0005/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0005/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0005/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0005/file-0004.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0005/file-0005.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0003/file-0005.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0004/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0004/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0004.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0005.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0004/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0004/file-0004.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0004/file-0005.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0004.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0005.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 3 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs::210] all_paths: {
    XvcPath(
        "dir-0003/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 763414382,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 763139470,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 765068398,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 762826266,
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
                tv_sec: 1703493614,
                tv_nsec: 763953916,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 761121043,
            },
        ),
    },
    XvcPath(
        ".xvcignore",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            141,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 865745046,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 759469818,
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
                tv_sec: 1703493614,
                tv_nsec: 764412367,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 759253738,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 762306483,
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
                tv_sec: 1703493614,
                tv_nsec: 766181381,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 765734805,
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
                tv_sec: 1703493614,
                tv_nsec: 764841568,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 759689731,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 765283812,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 759969186,
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
                tv_sec: 1703493614,
                tv_nsec: 760025351,
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
                tv_sec: 1703493614,
                tv_nsec: 765118606,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 766346295,
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
                tv_sec: 1703493614,
                tv_nsec: 866280538,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 764633613,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 763904750,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 766149465,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 761451579,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 761796032,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 765945968,
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
                tv_sec: 1703493614,
                tv_nsec: 762368732,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 764147162,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 760303055,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 763664795,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 765529891,
            },
        ),
    },
}
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0003/file-0002.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0003/file-0001.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0004/file-0004.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0002/file-0005.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0003"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0002/file-0001.bin"
[TRACE][file/src/list/mod.rs::542] path_str: ".xvcignore"
[TRACE][file/src/list/mod.rs::544] "ignored": "ignored"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0001/file-0002.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0004/file-0001.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0001/file-0001.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0002/file-0004.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0005"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0005/file-0002.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0004/file-0003.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0001/file-0003.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0004/file-0005.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0001/file-0004.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0001"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0004"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0005/file-0005.bin"
[TRACE][file/src/list/mod.rs::542] path_str: ".gitignore"
[TRACE][file/src/list/mod.rs::544] "ignored": "ignored"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0004/file-0002.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0003/file-0004.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0005/file-0004.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0002/file-0002.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0002/file-0003.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0005/file-0003.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0002"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0003/file-0005.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0001/file-0005.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0003/file-0003.bin"
[TRACE][file/src/list/mod.rs::542] path_str: "dir-0005/file-0001.bin"
[TRACE][file/src/list/mod.rs::553] from_disk: {
    XvcPath(
        "dir-0005/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 766149465,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 759253738,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 765945968,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 760303055,
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
                tv_sec: 1703493614,
                tv_nsec: 766181381,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 765068398,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 761451579,
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
                tv_sec: 1703493614,
                tv_nsec: 764841568,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 764147162,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 763904750,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 761796032,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 765734805,
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
                tv_sec: 1703493614,
                tv_nsec: 762368732,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 762306483,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 765283812,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 763664795,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 762826266,
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
                tv_sec: 1703493614,
                tv_nsec: 763953916,
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
                tv_sec: 1703493614,
                tv_nsec: 760025351,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 759469818,
            },
        ),
    },
    XvcPath(
        "dir-0002/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 761121043,
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
                tv_sec: 1703493614,
                tv_nsec: 764412367,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 763139470,
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
                tv_sec: 1703493614,
                tv_nsec: 765118606,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0001.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 765529891,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 759689731,
            },
        ),
    },
    XvcPath(
        "dir-0001/file-0004.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2004,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 759969186,
            },
        ),
    },
    XvcPath(
        "dir-0003/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 763414382,
            },
        ),
    },
    XvcPath(
        "dir-0004/file-0002.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 764633613,
            },
        ),
    },
    XvcPath(
        "dir-0005/file-0005.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2005,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703493614,
                tv_nsec: 766346295,
            },
        ),
    },
}
[TRACE][file/src/common/mod.rs::134] targets: None
[TRACE][file/src/list/mod.rs::555] from_store: HStore {
    map: {},
}
[TRACE][file/src/list/mod.rs::634] matches: [
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0005/file-0004.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2004,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 766149465,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
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
                    2001,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 759253738,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0005/file-0003.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2003,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 765945968,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
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
                    2005,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 760303055,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0005",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: Directory,
                size: Some(
                    224,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 766181381,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0004/file-0004.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2004,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 765068398,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0002/file-0002.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2002,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 761451579,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0004/file-0003.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2003,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 764841568,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0003/file-0005.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2005,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 764147162,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0003/file-0004.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2004,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 763904750,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0002/file-0003.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2003,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 761796032,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0005/file-0002.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2002,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 765734805,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0002",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: Directory,
                size: Some(
                    224,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 762368732,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0002/file-0004.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2004,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 762306483,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0004/file-0005.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2005,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 765283812,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0003/file-0003.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2003,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 763664795,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0002/file-0005.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2005,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 762826266,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0003",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: Directory,
                size: Some(
                    224,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 763953916,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0001",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: Directory,
                size: Some(
                    224,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 760025351,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
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
                    2002,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 759469818,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0002/file-0001.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2001,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 761121043,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0004/file-0001.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2001,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 764412367,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0003/file-0001.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2001,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 763139470,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0004",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: Directory,
                size: Some(
                    224,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 765118606,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0005/file-0001.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2001,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 765529891,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
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
                    2003,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 759689731,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
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
                    2004,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 759969186,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0003/file-0002.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2002,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 763414382,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0004/file-0002.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2002,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 764633613,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
    PathMatch {
        xvc_entity: None,
        actual_path: Some(
            XvcPath(
                "dir-0005/file-0005.bin",
            ),
        ),
        actual_metadata: Some(
            XvcMetadata {
                file_type: File,
                size: Some(
                    2005,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703493614,
                        tv_nsec: 766346295,
                    },
                ),
            },
        ),
        actual_digest: None,
        recorded_path: None,
        recorded_metadata: None,
        recorded_digest: None,
        recorded_recheck_method: None,
    },
]
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0005/file-0004.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0001/file-0001.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0005/file-0003.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0001/file-0005.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0005",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0004/file-0004.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0002/file-0002.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0004/file-0003.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0003/file-0005.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0003/file-0004.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0002/file-0003.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0005/file-0002.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0002",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0002/file-0004.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0004/file-0005.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0003/file-0003.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0002/file-0005.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0003",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0001",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0002/file-0001.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0004/file-0001.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0003/file-0001.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0004",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0005/file-0001.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0001/file-0004.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0003/file-0002.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0004/file-0002.bin",
)
[TRACE][file/src/list/mod.rs::193] &path_prefix: ""
[TRACE][file/src/list/mod.rs::195] ap: XvcPath(
    "dir-0005/file-0005.bin",
)
DX         224 2023-12-25 08:40:14                   dir-0001
FX        2001 2023-12-25 08:40:14          1953f05d dir-0001/file-0001.bin
FX        2002 2023-12-25 08:40:14          7e807161 dir-0001/file-0002.bin
FX        2003 2023-12-25 08:40:14          d2432259 dir-0001/file-0003.bin
FX        2004 2023-12-25 08:40:14          63535612 dir-0001/file-0004.bin
FX        2005 2023-12-25 08:40:14          447933dc dir-0001/file-0005.bin
DX         224 2023-12-25 08:40:14                   dir-0002
FX        2001 2023-12-25 08:40:14          1953f05d dir-0002/file-0001.bin
FX        2002 2023-12-25 08:40:14          7e807161 dir-0002/file-0002.bin
FX        2003 2023-12-25 08:40:14          d2432259 dir-0002/file-0003.bin
FX        2004 2023-12-25 08:40:14          63535612 dir-0002/file-0004.bin
FX        2005 2023-12-25 08:40:14          447933dc dir-0002/file-0005.bin
DX         224 2023-12-25 08:40:14                   dir-0003
FX        2001 2023-12-25 08:40:14          1953f05d dir-0003/file-0001.bin
FX        2002 2023-12-25 08:40:14          7e807161 dir-0003/file-0002.bin
FX        2003 2023-12-25 08:40:14          d2432259 dir-0003/file-0003.bin
FX        2004 2023-12-25 08:40:14          63535612 dir-0003/file-0004.bin
FX        2005 2023-12-25 08:40:14          447933dc dir-0003/file-0005.bin
DX         224 2023-12-25 08:40:14                   dir-0004
FX        2001 2023-12-25 08:40:14          1953f05d dir-0004/file-0001.bin
FX        2002 2023-12-25 08:40:14          7e807161 dir-0004/file-0002.bin
FX        2003 2023-12-25 08:40:14          d2432259 dir-0004/file-0003.bin
FX        2004 2023-12-25 08:40:14          63535612 dir-0004/file-0004.bin
FX        2005 2023-12-25 08:40:14          447933dc dir-0004/file-0005.bin
DX         224 2023-12-25 08:40:14                   dir-0005
FX        2001 2023-12-25 08:40:14          1953f05d dir-0005/file-0001.bin
FX        2002 2023-12-25 08:40:14          7e807161 dir-0005/file-0002.bin
FX        2003 2023-12-25 08:40:14          d2432259 dir-0005/file-0003.bin
FX        2004 2023-12-25 08:40:14          63535612 dir-0005/file-0004.bin
FX        2005 2023-12-25 08:40:14          447933dc dir-0005/file-0005.bin
Total #: 30 Workspace Size:       51195 Cached Size:           0

[TRACE][lib/src/cli/mod.rs::381] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::384] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc --debug file list --sort name-asc --show-dot-files"
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

## Output Format

With the default output format, the first two letters show the path type and
recheck method, respectively.

For example, if you track `dir-0001` as `copy`, the first letter is `F` for the
files and `D` for the directories. The second letter is `C` for files, meaning
the file is a copy of the cached file, and it's `X` for directories that means
they are not in the cache. Similar to Git, Xvc doesn't track only files and
directories are considered as collection of files.

```console
$ xvc file track dir-0001/

$ xvc file list dir-0001/
FC        2005 [..] 447933dc 447933dc dir-0001/file-0005.bin
FC        2004 [..] 63535612 63535612 dir-0001/file-0004.bin
FC        2003 [..] d2432259 d2432259 dir-0001/file-0003.bin
FC        2002 [..] 7e807161 7e807161 dir-0001/file-0002.bin
FC        2001 [..] 1953f05d 1953f05d dir-0001/file-0001.bin
Total #: 5 Workspace Size:       10015 Cached Size:       10015


```

If you add another set of files as hardlinks to the cached copies, it will
print the second letter as `H`.

```console
$ xvc file track dir-0002 --recheck-method hardlink

$ xvc file list dir-0002
FH        2005 [..] 447933dc 447933dc dir-0002/file-0005.bin
FH        2004 [..] 63535612 63535612 dir-0002/file-0004.bin
FH        2003 [..] d2432259 d2432259 dir-0002/file-0003.bin
FH        2002 [..] 7e807161 7e807161 dir-0002/file-0002.bin
FH        2001 [..] 1953f05d 1953f05d dir-0002/file-0001.bin
Total #: 5 Workspace Size:       10015 Cached Size:       10015


```

Note, as hardlinks are files with the same inode in the file system
with alternative paths, they are detected as `F`.

Symbolic links are typically reported as `SS` in the first letters.
It means they are symbolic links on the file system and their recheck method is also
symbolic links.

```console
$ xvc file track dir-0003 --recheck-method symlink

$ xvc file list dir-0003
SS         [..] 447933dc          dir-0003/file-0005.bin
SS         [..] 63535612          dir-0003/file-0004.bin
SS         [..] d2432259          dir-0003/file-0003.bin
SS         [..] 7e807161          dir-0003/file-0002.bin
SS         [..] 1953f05d          dir-0003/file-0001.bin
Total #: 5 Workspace Size:         [..] Cached Size:       10015


```

Although not all filesystems support it, `R` represents reflinks.

## Globs

You may use globs to list files.

```console
$ xvc file list 'dir-*/*-0001.bin'
FX        2001 [..]          1953f05d dir-0005/file-0001.bin
FX        2001 [..]          1953f05d dir-0004/file-0001.bin
SS         [..] 1953f05d          dir-0003/file-0001.bin
FH        2[..] 1953f05d 1953f05d dir-0002/file-0001.bin
FC        2[..] 1953f05d 1953f05d dir-0001/file-0001.bin
Total #: 5 Workspace Size:        [..] Cached Size:        2001


```

Note that all these files are identical. They are cached once, and only one of
them takes space in the cache.

You can also use multiple targets as globs.

```console
$ xvc file list '*/*-0001.bin' '*/*-0002.bin'
FX        2002 [..]          7e807161 dir-0005/file-0002.bin
FX        2001 [..]          1953f05d dir-0005/file-0001.bin
FX        2002 [..]          7e807161 dir-0004/file-0002.bin
FX        2001 [..]          1953f05d dir-0004/file-0001.bin
SS        [..] 7e807161          dir-0003/file-0002.bin
SS        [..] 1953f05d          dir-0003/file-0001.bin
FH        [..] 7e807161 7e807161 dir-0002/file-0002.bin
FH        [..] 1953f05d 1953f05d dir-0002/file-0001.bin
FC        [..] 7e807161 7e807161 dir-0001/file-0002.bin
FC        [..] 1953f05d 1953f05d dir-0001/file-0001.bin
Total #: 10 Workspace Size:       [..] Cached Size:        4003


```

## Sorting

You may sort `xvc file list` output by name, by modification time and by file
size.

Use `--sort` option to specify the sort criteria.

```console
$ xvc file list --sort name-desc dir-0001/
FC        2005 [..] 447933dc 447933dc dir-0001/file-0005.bin
FC        2004 [..] 63535612 63535612 dir-0001/file-0004.bin
FC        2003 [..] d2432259 d2432259 dir-0001/file-0003.bin
FC        2002 [..] 7e807161 7e807161 dir-0001/file-0002.bin
FC        2001 [..] 1953f05d 1953f05d dir-0001/file-0001.bin
Total #: 5 Workspace Size:       10015 Cached Size:       10015


```

```console
$ xvc file list --sort name-asc dir-0001/
FC        2001 [..] 1953f05d 1953f05d dir-0001/file-0001.bin
FC        2002 [..] 7e807161 7e807161 dir-0001/file-0002.bin
FC        2003 [..] d2432259 d2432259 dir-0001/file-0003.bin
FC        2004 [..] 63535612 63535612 dir-0001/file-0004.bin
FC        2005 [..] 447933dc 447933dc dir-0001/file-0005.bin
Total #: 5 Workspace Size:       10015 Cached Size:       10015


```

## Column Format

You can specify the columns that the command prints.

For example, if you only want to see the file names, use `{{name}}` as the
format string.

The following command sorts all files with their sizes in the workspace, and
prints their size and name.

```console
$ xvc file list --format '{{asz}} {{name}}' --sort size-desc dir-0001/
       2005 dir-0001/file-0005.bin
       2004 dir-0001/file-0004.bin
       2003 dir-0001/file-0003.bin
       2002 dir-0001/file-0002.bin
       2001 dir-0001/file-0001.bin
Total #: 5 Workspace Size:       10015 Cached Size:       10015


```

If you want to compare the recorded (cached) hashes and actual hashes in the workspace, you can use `{{acd}} {{rcd}} {{name}}` format string.

```console
$ xvc file list --format '{{acd8}} {{rcd8}} {{name}}' --sort ts-asc dir-0001
1953f05d 1953f05d dir-0001/file-0001.bin
7e807161 7e807161 dir-0001/file-0002.bin
d2432259 d2432259 dir-0001/file-0003.bin
63535612 63535612 dir-0001/file-0004.bin
447933dc 447933dc dir-0001/file-0005.bin
Total #: 5 Workspace Size:       10015 Cached Size:       10015


```

```admonish info
If `{{acd8}}` or `{{acd64}}` is not present in the format string, Xvc doesn't calculate these hashes. If you have large number of files where the default format (that includes actual content hashes) runs slowly, you may customize it to not to include these columns.
```

If you want to get a quick glimpse of what needs to carried in, or rechecked,
you can use cache status `{{cst}}` column.

```console
$ xvc-test-helper generate-random-file --size 100 dir-0001/a-new-file.bin

$ xvc file list --format '{{cst}} {{name}}' dir-0001/
= dir-0001/file-0005.bin
= dir-0001/file-0004.bin
= dir-0001/file-0003.bin
= dir-0001/file-0002.bin
= dir-0001/file-0001.bin
X dir-0001/a-new-file.bin
Total #: 6 Workspace Size:       10115 Cached Size:       10015


```

The cache status column shows `=` for unchanged files in the cache, `X` for
untracked files, `>` for files that there is newer version in the cache, and `<`
for files that there is a newer version in the workspace. The comparison is done
between recorded timestamp and actual timestamp with an accuracy of 1 second.
