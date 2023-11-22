# xvc file recheck

## Synopsis

```console
$ xvc file recheck --help
Get files from cache by copy or *link

Usage: xvc file recheck [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to recheck

Options:
      --recheck-method <RECHECK_METHOD>
          How to track the file contents in cache: One of copy, symlink, hardlink, reflink.
          
          Note: Reflink uses copy if the underlying file system doesn't support it.

      --no-parallel
          Don't use parallelism

      --force
          Force even if target exists

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

This command has an alias [`xvc file checkout`](/ref/xvc-file-checkout.md) if you feel more at home with Git terminology.

## Examples

Rechecking is analogous to [git checkout](https://git-scm.com/docs/git-checkout).
It copies or links a cached file to the workspace.

Let's create an example directory hierarchy as a showcase. 

```console
$ xvc-test-helper create-directory-tree --directories 1 --files 3 --seed 231123
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

Start by tracking files. 

```console
$ git init
...
$ xvc init

$ xvc file track dir-*
? 101
[DEBUG][logging/src/lib.rs::236] Terminal logger enabled with level: Trace
[TRACE][core/src/types/xvcroot.rs::247] "."
[DEBUG][core/src/types/xvcroot.rs::253] XVC DIR: "[CWD]"
[DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][config/src/lib.rs::536] cli_config: [
    "core.verbosity = debug",
    "core.quiet = false",
]
[TRACE][config/src/lib.rs::540] map: {
    "core.verbosity": String(
        "debug",
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
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
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
                "file.track.force": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "pipeline.process_pool_size": Integer(
                    4,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "git.command": String(
                    "git",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.guid": String(
                    "14497bc4aef463c7",
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "git.command": String(
                    "git",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "core.guid": String(
                    "b2a32bfd318e32eb",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "pipeline.process_pool_size": Integer(
                    4,
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
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.track.no_parallel": Boolean(
                    false,
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
            map: {
                "TRYCMD_DURATION": Integer(
                    30,
                ),
                "TRYCMD_TESTS": String(
                    "file",
                ),
            },
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
        "git.use_git": XvcConfigValue {
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
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
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
        "file.list.recursive": XvcConfigValue {
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
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "b2a32bfd318e32eb",
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
        "TRYCMD_DURATION": XvcConfigValue {
            source: Environment,
            value: Integer(
                30,
            ),
        },
        "TRYCMD_TESTS": XvcConfigValue {
            source: Environment,
            value: String(
                "file",
            ),
        },
        "file.track.force": XvcConfigValue {
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
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
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
        "file.carry-in.no_parallel": XvcConfigValue {
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
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"14497bc4aef463c7/"
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
                "core.verbosity = debug",
                "core.quiet = false",
            ],
        ),
    },
}
[TRACE][ecs/src/ecs/mod.rs::229] dir: "[CWD]/.xvc/ec"
[TRACE][ecs/src/ecs/mod.rs::239] files: [
    "[CWD]/.xvc/ec/1700685641311125",
    "[CWD]/.xvc/ec/1700685641313674",
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
    subcommand: Track(
        TrackCLI {
            recheck_method: None,
            no_commit: false,
            text_or_binary: None,
            force: false,
            no_parallel: false,
            targets: Some(
                [
                    "dir-*",
                ],
            ),
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
[TRACE][walker/src/lib.rs::688] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::689] ignore_path: "[CWD]/.xvcignore"
[TRACE][walker/src/lib.rs::697] &content: "
# Add patterns of files xvc should ignore, which could improve
# the performance.
# It's in the same format as .gitignore files.
"
[TRACE][walker/src/lib.rs::394] new_patterns: []
[TRACE][walker/src/lib.rs::396] ignore_rules: IgnoreRules {
    root: "[CWD]",
    patterns: [
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
    ],
    whitelist_set: GlobSet {
        len: 0,
        strats: [],
    },
    ignore_set: GlobSet {
        len: 2,
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
}
[TRACE][walker/src/lib.rs::407] child_paths: [
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
                    tv_sec: 1700685641,
                    tv_nsec: 268293310,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 272088446,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 267225860,
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
                    tv_sec: 1700685641,
                    tv_nsec: 311246588,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 331656715,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 308696317,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/.keep",
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
                    tv_sec: 1700644173,
                    tv_nsec: 820168674,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700644173,
                    tv_nsec: 820168674,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700644173,
                    tv_nsec: 820168674,
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
                    tv_sec: 1700685641,
                    tv_nsec: 311409837,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 341659093,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 311345712,
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
                    tv_sec: 1700685641,
                    tv_nsec: 269338718,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 363257043,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 268618432,
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
                    tv_sec: 1700685641,
                    tv_nsec: 336021930,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 275014589,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 275014589,
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
                    tv_sec: 1700685641,
                    tv_nsec: 311310213,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 341794675,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 311278171,
                },
            ),
            ..
        },
    },
]
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/dir-0001"
[TRACE][walker/src/lib.rs::424] child_path.path: "[CWD]/.xvc"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/.keep"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/.gitignore"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/dir-0002"
[TRACE][walker/src/lib.rs::424] child_path.path: "[CWD]/.git"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/.xvcignore"
[TRACE][walker/src/lib.rs::436] dwi: IgnoreRules {
    root: "[CWD]",
    patterns: [
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
    ],
    whitelist_set: GlobSet {
        len: 0,
        strats: [],
    },
    ignore_set: GlobSet {
        len: 2,
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
}
[TRACE][walker/src/lib.rs::437] walk_options: WalkOptions {
    ignore_filename: Some(
        ".xvcignore",
    ),
    include_dirs: true,
}
[TRACE][walker/src/lib.rs::438] path_sender: Sender { .. }
[TRACE][walker/src/lib.rs::439] ignore_sender: Sender { .. }
[TRACE][walker/src/lib.rs::436] dwi: IgnoreRules {
    root: "[CWD]",
    patterns: [
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
    ],
    whitelist_set: GlobSet {
        len: 0,
        strats: [],
    },
    ignore_set: GlobSet {
        len: 2,
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
}
[TRACE][walker/src/lib.rs::437] walk_options: WalkOptions {
    ignore_filename: Some(
        ".xvcignore",
    ),
    include_dirs: true,
}
[TRACE][walker/src/lib.rs::438] path_sender: Sender { .. }
[TRACE][walker/src/lib.rs::439] ignore_sender: Sender { .. }
[TRACE][walker/src/lib.rs::407] child_paths: [
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
                    tv_sec: 1700685641,
                    tv_nsec: 269285718,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 269027679,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 269027679,
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
                    tv_sec: 1700685641,
                    tv_nsec: 269580216,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 269329843,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 269329843,
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
                    tv_sec: 1700685641,
                    tv_nsec: 268978096,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 268702848,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 268702848,
                },
            ),
            ..
        },
    },
]
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/dir-0002/file-0002.bin"
[TRACE][walker/src/lib.rs::407] child_paths: [
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
                    tv_sec: 1700685641,
                    tv_nsec: 268074936,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 267737106,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 267737106,
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
                    tv_sec: 1700685641,
                    tv_nsec: 268566974,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 268262435,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 268262435,
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
                    tv_sec: 1700685641,
                    tv_nsec: 267607607,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 267299109,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700685641,
                    tv_nsec: 267299109,
                },
            ),
            ..
        },
    },
]
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/dir-0002/file-0003.bin"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/dir-0002/file-0001.bin"
[TRACE][walker/src/lib.rs::452] "End of walk_parallel": "End of walk_parallel"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/dir-0001/file-0002.bin"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/dir-0001/file-0003.bin"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/dir-0001/file-0001.bin"
[TRACE][walker/src/lib.rs::452] "End of walk_parallel": "End of walk_parallel"
[TRACE][walker/src/lib.rs::452] "End of walk_parallel": "End of walk_parallel"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/.keep"
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
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002"
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
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0003.bin"
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
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0003.bin"
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
[TRACE][file/src/common/mod.rs::210] all_paths: {
    XvcPath(
        "dir-0001",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            160,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700685641,
                tv_nsec: 268293310,
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
                tv_sec: 1700685641,
                tv_nsec: 311409837,
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
                tv_sec: 1700685641,
                tv_nsec: 269285718,
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
                tv_sec: 1700685641,
                tv_nsec: 269580216,
            },
        ),
    },
    XvcPath(
        ".keep",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            0,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700644173,
                tv_nsec: 820168674,
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
                tv_sec: 1700685641,
                tv_nsec: 311310213,
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
                tv_sec: 1700685641,
                tv_nsec: 267607607,
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
                tv_sec: 1700685641,
                tv_nsec: 268978096,
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
                tv_sec: 1700685641,
                tv_nsec: 268566974,
            },
        ),
    },
    XvcPath(
        "dir-0002",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            160,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700685641,
                tv_nsec: 269338718,
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
                tv_sec: 1700685641,
                tv_nsec: 268074936,
            },
        ),
    },
}
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 1 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs::229] glob_matcher: GlobSet {
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
                    D 000000: /x00-/x08 => 0
                    F 000016:
                    * 000032: /x00-/x02 => 48, /x03 => 112, /x04-/x08 => 48
                     matches: 0
                     >000048: /x00-/x02 => 48, /x03 => 112, /x04-/x08 => 48
                      000064: /x00-/x02 => 0, /x03 => 112, /x04-/x08 => 0
                      000080: /x00-/x02 => 48, /x03 => 112, /x04-/x06 => 48, /x07 => 96, /x08 => 48
                      000096: /x00 => 48, /x01 => 32, /x02 => 48, /x03 => 112, /x04-/x08 => 48
                      000112: /x00-/x02 => 48, /x03 => 112, /x04 => 48, /x05 => 80, /x06-/x08 => 48
                    match kind: Standard
                    prefilter: true
                    state length: 8
                    pattern length: 1
                    shortest pattern length: 4
                    longest pattern length: 4
                    alphabet length: 9
                    stride: 16
                    byte classes: ByteClasses(0 => [0-44], 1 => [45], 2 => [46-99], 3 => [100], 4 => [101-104], 5 => [105], 6 => [106-113], 7 => [114], 8 => [115-255])
                    memory usage: 548
                    )
                    ,
                ),
                map: [
                    0,
                ],
                longest: 4,
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
[TRACE][file/src/track/mod.rs::121] targets: {
    XvcPath(
        "dir-0002/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700685641,
                tv_nsec: 269580216,
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
                tv_sec: 1700685641,
                tv_nsec: 268566974,
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
                tv_sec: 1700685641,
                tv_nsec: 267607607,
            },
        ),
    },
    XvcPath(
        "dir-0001",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            160,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700685641,
                tv_nsec: 268293310,
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
                tv_sec: 1700685641,
                tv_nsec: 268978096,
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
                tv_sec: 1700685641,
                tv_nsec: 269285718,
            },
        ),
    },
    XvcPath(
        "dir-0002",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            160,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700685641,
                tv_nsec: 269338718,
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
                tv_sec: 1700685641,
                tv_nsec: 268074936,
            },
        ),
    },
}
[TRACE][file/src/common/compare.rs::38] pmm: {
    XvcPath(
        "dir-0002/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700685641,
                tv_nsec: 269580216,
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
                tv_sec: 1700685641,
                tv_nsec: 268566974,
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
                tv_sec: 1700685641,
                tv_nsec: 267607607,
            },
        ),
    },
    XvcPath(
        "dir-0001",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            160,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700685641,
                tv_nsec: 268293310,
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
                tv_sec: 1700685641,
                tv_nsec: 268978096,
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
                tv_sec: 1700685641,
                tv_nsec: 269285718,
            },
        ),
    },
    XvcPath(
        "dir-0002",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            160,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700685641,
                tv_nsec: 269338718,
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
                tv_sec: 1700685641,
                tv_nsec: 268074936,
            },
        ),
    },
}
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "dir-0002/file-0003.bin",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    2,
    15303842246208353857,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    3,
    15303842246208353857,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "dir-0001/file-0001.bin",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    4,
    15303842246208353857,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "dir-0001",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    5,
    15303842246208353857,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "dir-0002/file-0001.bin",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    6,
    15303842246208353857,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "dir-0002/file-0002.bin",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    7,
    15303842246208353857,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "dir-0002",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    8,
    15303842246208353857,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    9,
    15303842246208353857,
)
[TRACE][file/src/common/compare.rs::469] file_entities: {
    XvcEntity(
        7,
        15303842246208353857,
    ),
    XvcEntity(
        2,
        15303842246208353857,
    ),
    XvcEntity(
        4,
        15303842246208353857,
    ),
    XvcEntity(
        3,
        15303842246208353857,
    ),
    XvcEntity(
        6,
        15303842246208353857,
    ),
    XvcEntity(
        9,
        15303842246208353857,
    ),
}
[TRACE][file/src/common/compare.rs::480] dir_entities: {
    XvcEntity(
        5,
        15303842246208353857,
    ),
    XvcEntity(
        8,
        15303842246208353857,
    ),
}
[TRACE][file/src/common/compare.rs::162] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0003.bin",
    ),
}
[TRACE][file/src/common/compare.rs::203] actual: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][file/src/common/compare.rs::205] path: AbsolutePath(
    "[CWD]/dir-0001/file-0003.bin",
)
[TRACE][file/src/common/compare.rs::162] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0002/file-0003.bin",
    ),
}
[TRACE][file/src/common/compare.rs::162] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0002/file-0002.bin",
    ),
}
[TRACE][file/src/common/compare.rs::203] actual: XvcPath(
    "dir-0002/file-0002.bin",
)
[TRACE][file/src/common/compare.rs::205] path: AbsolutePath(
    "[CWD]/dir-0002/file-0002.bin",
)
[TRACE][file/src/common/compare.rs::203] actual: XvcPath(
    "dir-0002/file-0003.bin",
)
[TRACE][file/src/common/compare.rs::205] path: AbsolutePath(
    "[CWD]/dir-0002/file-0003.bin",
)
[TRACE][file/src/common/compare.rs::207] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            128,
            79,
            184,
            237,
            187,
            18,
            46,
            115,
            95,
            172,
            215,
            249,
            67,
            193,
            187,
            231,
            84,
            233,
            57,
            169,
            104,
            243,
            133,
            193,
            47,
            86,
            177,
            4,
            17,
            164,
            160,
            21,
        ],
    },
)
[TRACE][file/src/common/compare.rs::145] stored_content_digest: None
[TRACE][file/src/common/compare.rs::146] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            128,
            79,
            184,
            237,
            187,
            18,
            46,
            115,
            95,
            172,
            215,
            249,
            67,
            193,
            187,
            231,
            84,
            233,
            57,
            169,
            104,
            243,
            133,
            193,
            47,
            86,
            177,
            4,
            17,
            164,
            160,
            21,
        ],
    },
)
[TRACE][file/src/common/compare.rs::209] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                128,
                79,
                184,
                237,
                187,
                18,
                46,
                115,
                95,
                172,
                215,
                249,
                67,
                193,
                187,
                231,
                84,
                233,
                57,
                169,
                104,
                243,
                133,
                193,
                47,
                86,
                177,
                4,
                17,
                164,
                160,
                21,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::162] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0002/file-0001.bin",
    ),
}
[TRACE][file/src/common/compare.rs::203] actual: XvcPath(
    "dir-0002/file-0001.bin",
)
[TRACE][file/src/common/compare.rs::205] path: AbsolutePath(
    "[CWD]/dir-0002/file-0001.bin",
)
[TRACE][file/src/common/compare.rs::207] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            60,
            146,
            85,
            66,
            78,
            19,
            217,
            195,
            138,
            55,
            197,
            221,
            211,
            118,
            225,
            7,
            12,
            221,
            93,
            230,
            105,
            150,
            251,
            200,
            33,
            148,
            196,
            98,
            246,
            83,
            133,
            109,
        ],
    },
)
[TRACE][file/src/common/compare.rs::145] stored_content_digest: None
[TRACE][file/src/common/compare.rs::146] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            60,
            146,
            85,
            66,
            78,
            19,
            217,
            195,
            138,
            55,
            197,
            221,
            211,
            118,
            225,
            7,
            12,
            221,
            93,
            230,
            105,
            150,
            251,
            200,
            33,
            148,
            196,
            98,
            246,
            83,
            133,
            109,
        ],
    },
)
[TRACE][file/src/common/compare.rs::209] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                60,
                146,
                85,
                66,
                78,
                19,
                217,
                195,
                138,
                55,
                197,
                221,
                211,
                118,
                225,
                7,
                12,
                221,
                93,
                230,
                105,
                150,
                251,
                200,
                33,
                148,
                196,
                98,
                246,
                83,
                133,
                109,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::162] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0002.bin",
    ),
}
[TRACE][file/src/common/compare.rs::203] actual: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][file/src/common/compare.rs::205] path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/compare.rs::207] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            107,
            198,
            95,
            88,
            30,
            58,
            3,
            237,
            177,
            39,
            182,
            59,
            113,
            197,
            105,
            11,
            225,
            118,
            226,
            254,
            38,
            82,
            102,
            247,
            10,
            188,
            101,
            247,
            38,
            19,
            246,
            46,
        ],
    },
)
[TRACE][file/src/common/compare.rs::145] stored_content_digest: None
[TRACE][file/src/common/compare.rs::146] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            107,
            198,
            95,
            88,
            30,
            58,
            3,
            237,
            177,
            39,
            182,
            59,
            113,
            197,
            105,
            11,
            225,
            118,
            226,
            254,
            38,
            82,
            102,
            247,
            10,
            188,
            101,
            247,
            38,
            19,
            246,
            46,
        ],
    },
)
[TRACE][file/src/common/compare.rs::209] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                107,
                198,
                95,
                88,
                30,
                58,
                3,
                237,
                177,
                39,
                182,
                59,
                113,
                197,
                105,
                11,
                225,
                118,
                226,
                254,
                38,
                82,
                102,
                247,
                10,
                188,
                101,
                247,
                38,
                19,
                246,
                46,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::162] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "dir-0001/file-0001.bin",
    ),
}
[TRACE][file/src/common/compare.rs::203] actual: XvcPath(
    "dir-0001/file-0001.bin",
)
[TRACE][file/src/common/compare.rs::205] path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][file/src/common/compare.rs::207] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            60,
            146,
            85,
            66,
            78,
            19,
            217,
            195,
            138,
            55,
            197,
            221,
            211,
            118,
            225,
            7,
            12,
            221,
            93,
            230,
            105,
            150,
            251,
            200,
            33,
            148,
            196,
            98,
            246,
            83,
            133,
            109,
        ],
    },
)
[TRACE][file/src/common/compare.rs::145] stored_content_digest: None
[TRACE][file/src/common/compare.rs::146] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            60,
            146,
            85,
            66,
            78,
            19,
            217,
            195,
            138,
            55,
            197,
            221,
            211,
            118,
            225,
            7,
            12,
            221,
            93,
            230,
            105,
            150,
            251,
            200,
            33,
            148,
            196,
            98,
            246,
            83,
            133,
            109,
        ],
    },
)
[TRACE][file/src/common/compare.rs::209] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                60,
                146,
                85,
                66,
                78,
                19,
                217,
                195,
                138,
                55,
                197,
                221,
                211,
                118,
                225,
                7,
                12,
                221,
                93,
                230,
                105,
                150,
                251,
                200,
                33,
                148,
                196,
                98,
                246,
                83,
                133,
                109,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::207] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            128,
            79,
            184,
            237,
            187,
            18,
            46,
            115,
            95,
            172,
            215,
            249,
            67,
            193,
            187,
            231,
            84,
            233,
            57,
            169,
            104,
            243,
            133,
            193,
            47,
            86,
            177,
            4,
            17,
            164,
            160,
            21,
        ],
    },
)
[TRACE][file/src/common/compare.rs::145] stored_content_digest: None
[TRACE][file/src/common/compare.rs::146] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            128,
            79,
            184,
            237,
            187,
            18,
            46,
            115,
            95,
            172,
            215,
            249,
            67,
            193,
            187,
            231,
            84,
            233,
            57,
            169,
            104,
            243,
            133,
            193,
            47,
            86,
            177,
            4,
            17,
            164,
            160,
            21,
        ],
    },
)
[TRACE][file/src/common/compare.rs::209] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                128,
                79,
                184,
                237,
                187,
                18,
                46,
                115,
                95,
                172,
                215,
                249,
                67,
                193,
                187,
                231,
                84,
                233,
                57,
                169,
                104,
                243,
                133,
                193,
                47,
                86,
                177,
                4,
                17,
                164,
                160,
                21,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::207] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            107,
            198,
            95,
            88,
            30,
            58,
            3,
            237,
            177,
            39,
            182,
            59,
            113,
            197,
            105,
            11,
            225,
            118,
            226,
            254,
            38,
            82,
            102,
            247,
            10,
            188,
            101,
            247,
            38,
            19,
            246,
            46,
        ],
    },
)
[TRACE][file/src/common/compare.rs::145] stored_content_digest: None
[TRACE][file/src/common/compare.rs::146] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            107,
            198,
            95,
            88,
            30,
            58,
            3,
            237,
            177,
            39,
            182,
            59,
            113,
            197,
            105,
            11,
            225,
            118,
            226,
            254,
            38,
            82,
            102,
            247,
            10,
            188,
            101,
            247,
            38,
            19,
            246,
            46,
        ],
    },
)
[TRACE][file/src/common/compare.rs::209] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                107,
                198,
                95,
                88,
                30,
                58,
                3,
                237,
                177,
                39,
                182,
                59,
                113,
                197,
                105,
                11,
                225,
                118,
                226,
                254,
                38,
                82,
                102,
                247,
                10,
                188,
                101,
                247,
                38,
                19,
                246,
                46,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::558] file_content_digest_diff_store.keys(): [
    XvcEntity(
        3,
        15303842246208353857,
    ),
    XvcEntity(
        6,
        15303842246208353857,
    ),
    XvcEntity(
        2,
        15303842246208353857,
    ),
    XvcEntity(
        4,
        15303842246208353857,
    ),
    XvcEntity(
        7,
        15303842246208353857,
    ),
    XvcEntity(
        9,
        15303842246208353857,
    ),
]
[TRACE][file/src/track/mod.rs::186] content_digest_diff: HStore {
    map: {
        XvcEntity(
            4,
            15303842246208353857,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        60,
                        146,
                        85,
                        66,
                        78,
                        19,
                        217,
                        195,
                        138,
                        55,
                        197,
                        221,
                        211,
                        118,
                        225,
                        7,
                        12,
                        221,
                        93,
                        230,
                        105,
                        150,
                        251,
                        200,
                        33,
                        148,
                        196,
                        98,
                        246,
                        83,
                        133,
                        109,
                    ],
                },
            ),
        },
        XvcEntity(
            6,
            15303842246208353857,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        60,
                        146,
                        85,
                        66,
                        78,
                        19,
                        217,
                        195,
                        138,
                        55,
                        197,
                        221,
                        211,
                        118,
                        225,
                        7,
                        12,
                        221,
                        93,
                        230,
                        105,
                        150,
                        251,
                        200,
                        33,
                        148,
                        196,
                        98,
                        246,
                        83,
                        133,
                        109,
                    ],
                },
            ),
        },
        XvcEntity(
            9,
            15303842246208353857,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        107,
                        198,
                        95,
                        88,
                        30,
                        58,
                        3,
                        237,
                        177,
                        39,
                        182,
                        59,
                        113,
                        197,
                        105,
                        11,
                        225,
                        118,
                        226,
                        254,
                        38,
                        82,
                        102,
                        247,
                        10,
                        188,
                        101,
                        247,
                        38,
                        19,
                        246,
                        46,
                    ],
                },
            ),
        },
        XvcEntity(
            8,
            15303842246208353857,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        249,
                        209,
                        178,
                        22,
                        179,
                        237,
                        43,
                        44,
                        169,
                        183,
                        39,
                        137,
                        172,
                        245,
                        250,
                        158,
                        131,
                        0,
                        9,
                        99,
                        154,
                        148,
                        29,
                        34,
                        239,
                        135,
                        237,
                        221,
                        93,
                        31,
                        31,
                        15,
                    ],
                },
            ),
        },
        XvcEntity(
            5,
            15303842246208353857,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        249,
                        209,
                        178,
                        22,
                        179,
                        237,
                        43,
                        44,
                        169,
                        183,
                        39,
                        137,
                        172,
                        245,
                        250,
                        158,
                        131,
                        0,
                        9,
                        99,
                        154,
                        148,
                        29,
                        34,
                        239,
                        135,
                        237,
                        221,
                        93,
                        31,
                        31,
                        15,
                    ],
                },
            ),
        },
        XvcEntity(
            2,
            15303842246208353857,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        128,
                        79,
                        184,
                        237,
                        187,
                        18,
                        46,
                        115,
                        95,
                        172,
                        215,
                        249,
                        67,
                        193,
                        187,
                        231,
                        84,
                        233,
                        57,
                        169,
                        104,
                        243,
                        133,
                        193,
                        47,
                        86,
                        177,
                        4,
                        17,
                        164,
                        160,
                        21,
                    ],
                },
            ),
        },
        XvcEntity(
            7,
            15303842246208353857,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        107,
                        198,
                        95,
                        88,
                        30,
                        58,
                        3,
                        237,
                        177,
                        39,
                        182,
                        59,
                        113,
                        197,
                        105,
                        11,
                        225,
                        118,
                        226,
                        254,
                        38,
                        82,
                        102,
                        247,
                        10,
                        188,
                        101,
                        247,
                        38,
                        19,
                        246,
                        46,
                    ],
                },
            ),
        },
        XvcEntity(
            3,
            15303842246208353857,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        128,
                        79,
                        184,
                        237,
                        187,
                        18,
                        46,
                        115,
                        95,
                        172,
                        215,
                        249,
                        67,
                        193,
                        187,
                        231,
                        84,
                        233,
                        57,
                        169,
                        104,
                        243,
                        133,
                        193,
                        47,
                        86,
                        177,
                        4,
                        17,
                        164,
                        160,
                        21,
                    ],
                },
            ),
        },
    },
}
[TRACE][file/src/common/mod.rs::474] records.len(): 0
[TRACE][file/src/common/mod.rs::476] new_store.len(): 8
[TRACE][file/src/common/mod.rs::474] records.len(): 0
[TRACE][file/src/common/mod.rs::476] new_store.len(): 8
[TRACE][file/src/common/mod.rs::474] records.len(): 0
[TRACE][file/src/common/mod.rs::476] new_store.len(): 8
[TRACE][file/src/common/mod.rs::474] records.len(): 0
[TRACE][file/src/common/mod.rs::476] new_store.len(): 8
[TRACE][file/src/common/mod.rs::474] records.len(): 0
[TRACE][file/src/common/mod.rs::476] new_store.len(): 8
[TRACE][file/src/track/mod.rs::194] targets: {
    XvcPath(
        "dir-0002/file-0003.bin",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700685641,
                tv_nsec: 269580216,
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
                tv_sec: 1700685641,
                tv_nsec: 268566974,
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
                tv_sec: 1700685641,
                tv_nsec: 267607607,
            },
        ),
    },
    XvcPath(
        "dir-0001",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            160,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700685641,
                tv_nsec: 268293310,
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
                tv_sec: 1700685641,
                tv_nsec: 268978096,
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
                tv_sec: 1700685641,
                tv_nsec: 269285718,
            },
        ),
    },
    XvcPath(
        "dir-0002",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            160,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700685641,
                tv_nsec: 269338718,
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
                tv_sec: 1700685641,
                tv_nsec: 268074936,
            },
        ),
    },
}
[TRACE][walker/src/lib.rs::688] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::689] ignore_path: "[CWD]/.gitignore"
[TRACE][walker/src/lib.rs::697] &content: "
## Following are required for Xvc to function correctly.
.xvc/*
!.xvc/store/
!.xvc/ec/
!.xvc/config.toml

"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 1 regexes
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[TRACE][file/src/track/mod.rs::224] file_targets: [
    XvcPath(
        "dir-0002/file-0003.bin",
    ),
    XvcPath(
        "dir-0001/file-0003.bin",
    ),
    XvcPath(
        "dir-0001/file-0001.bin",
    ),
    XvcPath(
        "dir-0002/file-0001.bin",
    ),
    XvcPath(
        "dir-0002/file-0002.bin",
    ),
    XvcPath(
        "dir-0001/file-0002.bin",
    ),
]
[TRACE][file/src/track/mod.rs::225] dir_targets: []
[TRACE][walker/src/lib.rs::688] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::689] ignore_path: "[CWD]/.gitignore"
[TRACE][walker/src/lib.rs::697] &content: "
## Following are required for Xvc to function correctly.
.xvc/*
!.xvc/store/
!.xvc/ec/
!.xvc/config.toml

"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 1 regexes
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[TRACE][file/src/track/mod.rs::264] xvc_paths_to_carry: HStore {
    map: {
        XvcEntity(
            3,
            15303842246208353857,
        ): XvcPath(
            "dir-0001/file-0003.bin",
        ),
        XvcEntity(
            9,
            15303842246208353857,
        ): XvcPath(
            "dir-0001/file-0002.bin",
        ),
        XvcEntity(
            4,
            15303842246208353857,
        ): XvcPath(
            "dir-0001/file-0001.bin",
        ),
        XvcEntity(
            6,
            15303842246208353857,
        ): XvcPath(
            "dir-0002/file-0001.bin",
        ),
        XvcEntity(
            2,
            15303842246208353857,
        ): XvcPath(
            "dir-0002/file-0003.bin",
        ),
        XvcEntity(
            7,
            15303842246208353857,
        ): XvcPath(
            "dir-0002/file-0002.bin",
        ),
    },
}
[TRACE][file/src/carry_in/mod.rs::238] ignore_writer: Sender { .. }
[TRACE][file/src/carry_in/mod.rs::239] ignore_thread: JoinHandle { .. }
[TRACE][walker/src/lib.rs::688] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::689] ignore_path: "[CWD]/.gitignore"
[TRACE][file/src/carry_in/mod.rs::245] cache_path: XvcCachePath(
    "b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin",
)
[TRACE][walker/src/lib.rs::697] &content: "
## Following are required for Xvc to function correctly.
.xvc/*
!.xvc/store/
!.xvc/ec/
!.xvc/config.toml

"
[TRACE][file/src/carry_in/mod.rs::247] abs_cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::278] &cache_path: XvcCachePath(
    "b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::279] &xp: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][file/src/common/mod.rs::455] path: AbsolutePath(
    "[CWD]/dir-0001/file-0003.bin",
)
[TRACE][file/src/common/mod.rs::457] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin",
)
[TRACE][file/src/common/mod.rs::426] cache_dir: "[CWD]/.xvc/b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 1 regexes
[TRACE][file/src/common/mod.rs::437] &file_perm.clone(): Permissions(
    FilePermissions {
        mode: 33188,
    },
)
[TRACE][file/src/carry_in/mod.rs::245] cache_path: XvcCachePath(
    "b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin",
)
[TRACE][file/src/common/mod.rs::440] &file_perm.clone(): Permissions(
    FilePermissions {
        mode: 33060,
    },
)
[TRACE][file/src/carry_in/mod.rs::287] target_path: AbsolutePath(
    "[CWD]/dir-0001/file-0003.bin",
)
[TRACE][file/src/common/mod.rs::290] parent: XvcPath(
    "dir-0001",
)
[TRACE][file/src/carry_in/mod.rs::247] abs_cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin",
)
[TRACE][file/src/common/mod.rs::292] parent_dir: AbsolutePath(
    "[CWD]/dir-0001",
)
[TRACE][file/src/carry_in/mod.rs::245] cache_path: XvcCachePath(
    "b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin",
)
[TRACE][file/src/common/mod.rs::305] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin",
)
[TRACE][file/src/common/mod.rs::307] path: AbsolutePath(
    "[CWD]/dir-0001/file-0003.bin",
)
[TRACE][file/src/common/mod.rs::314] path: AbsolutePath(
    "[CWD]/dir-0001/file-0003.bin",
)
[TRACE][file/src/common/mod.rs::315] recheck_method: Copy
[TRACE][file/src/common/mod.rs::321] "Before copy": "Before copy"
[TRACE][file/src/carry_in/mod.rs::247] abs_cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin",
)
[TRACE][file/src/common/mod.rs::322] &cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::278] &cache_path: XvcCachePath(
    "b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::287] target_path: AbsolutePath(
    "[CWD]/dir-0002/file-0003.bin",
)
[TRACE][file/src/carry_in/mod.rs::279] &xp: XvcPath(
    "dir-0002/file-0001.bin",
)
[TRACE][file/src/common/mod.rs::455] path: AbsolutePath(
    "[CWD]/dir-0002/file-0001.bin",
)
[TRACE][file/src/common/mod.rs::457] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin",
)
[TRACE][file/src/common/mod.rs::426] cache_dir: "[CWD]/.xvc/b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d"
[TRACE][file/src/carry_in/mod.rs::245] cache_path: XvcCachePath(
    "b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::247] abs_cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::278] &cache_path: XvcCachePath(
    "b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::279] &xp: XvcPath(
    "dir-0002/file-0002.bin",
)
[TRACE][file/src/common/mod.rs::455] path: AbsolutePath(
    "[CWD]/dir-0002/file-0002.bin",
)
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[TRACE][file/src/common/mod.rs::457] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin",
)
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[TRACE][file/src/common/mod.rs::426] cache_dir: "[CWD]/.xvc/b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[TRACE][file/src/carry_in/mod.rs::245] cache_path: XvcCachePath(
    "b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::247] abs_cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin",
)
[TRACE][file/src/common/mod.rs::290] parent: XvcPath(
    "dir-0002",
)
[TRACE][file/src/carry_in/mod.rs::278] &cache_path: XvcCachePath(
    "b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin",
)
[TRACE][file/src/common/mod.rs::292] parent_dir: AbsolutePath(
    "[CWD]/dir-0002",
)
[TRACE][file/src/carry_in/mod.rs::279] &xp: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][file/src/common/mod.rs::455] path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/mod.rs::305] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin",
)
[TRACE][file/src/common/mod.rs::457] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin",
)
[TRACE][file/src/common/mod.rs::307] path: AbsolutePath(
    "[CWD]/dir-0002/file-0003.bin",
)
[TRACE][file/src/common/mod.rs::314] path: AbsolutePath(
    "[CWD]/dir-0002/file-0003.bin",
)
[TRACE][file/src/common/mod.rs::315] recheck_method: Copy
[TRACE][file/src/common/mod.rs::321] "Before copy": "Before copy"
[TRACE][file/src/common/mod.rs::322] &cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin",
)
[TRACE][file/src/common/mod.rs::323] &path: AbsolutePath(
    "[CWD]/dir-0002/file-0003.bin",
)
[TRACE][file/src/common/mod.rs::437] &file_perm.clone(): Permissions(
    FilePermissions {
        mode: 33188,
    },
)
[TRACE][file/src/common/mod.rs::426] cache_dir: "[CWD]/.xvc/b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e"
[TRACE][file/src/common/mod.rs::327] &perm: Permissions(
    FilePermissions {
        mode: 33060,
    },
)
[TRACE][file/src/common/mod.rs::329] &perm: Permissions(
    FilePermissions {
        mode: 33206,
    },
)
[TRACE][file/src/common/mod.rs::367] "Return recheck_from_cache": "Return recheck_from_cache"
[TRACE][file/src/carry_in/mod.rs::304] &cache_path: XvcCachePath(
    "b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::305] recheck_method: Copy
[TRACE][file/src/carry_in/mod.rs::306] &xp: XvcPath(
    "dir-0002/file-0003.bin",
)
[TRACE][file/src/carry_in/mod.rs::245] cache_path: XvcCachePath(
    "b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::247] abs_cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin",
)
[TRACE][file/src/common/mod.rs::437] &file_perm.clone(): Permissions(
    FilePermissions {
        mode: 33188,
    },
)
[TRACE][file/src/carry_in/mod.rs::278] &cache_path: XvcCachePath(
    "b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::279] &xp: XvcPath(
    "dir-0001/file-0001.bin",
)
[TRACE][file/src/common/mod.rs::455] path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][file/src/common/mod.rs::440] &file_perm.clone(): Permissions(
    FilePermissions {
        mode: 33060,
    },
)
[TRACE][file/src/common/mod.rs::457] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin",
)
[TRACE][file/src/common/mod.rs::426] cache_dir: "[CWD]/.xvc/b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d"
[TRACE][file/src/carry_in/mod.rs::287] target_path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/mod.rs::290] parent: XvcPath(
    "dir-0001",
)
[TRACE][file/src/common/mod.rs::292] parent_dir: AbsolutePath(
    "[CWD]/dir-0001",
)
[TRACE][file/src/common/mod.rs::305] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin",
)
[TRACE][file/src/common/mod.rs::307] path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/mod.rs::314] path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/mod.rs::315] recheck_method: Copy
[TRACE][file/src/common/mod.rs::321] "Before copy": "Before copy"
[TRACE][file/src/common/mod.rs::322] &cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin",
)
[TRACE][file/src/common/mod.rs::323] &path: AbsolutePath(
    "[CWD]/dir-0001/file-0002.bin",
)
[TRACE][file/src/common/mod.rs::437] &file_perm.clone(): Permissions(
    FilePermissions {
        mode: 33188,
    },
)
[TRACE][file/src/common/mod.rs::440] &file_perm.clone(): Permissions(
    FilePermissions {
        mode: 33060,
    },
)
[TRACE][file/src/common/mod.rs::327] &perm: Permissions(
    FilePermissions {
        mode: 33060,
    },
)
[TRACE][file/src/common/mod.rs::329] &perm: Permissions(
    FilePermissions {
        mode: 33206,
    },
)
[TRACE][file/src/carry_in/mod.rs::287] target_path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][file/src/common/mod.rs::367] "Return recheck_from_cache": "Return recheck_from_cache"
[TRACE][file/src/common/mod.rs::290] parent: XvcPath(
    "dir-0001",
)
[TRACE][file/src/carry_in/mod.rs::304] &cache_path: XvcCachePath(
    "b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::305] recheck_method: Copy
[TRACE][file/src/common/mod.rs::292] parent_dir: AbsolutePath(
    "[CWD]/dir-0001",
)
[TRACE][file/src/carry_in/mod.rs::306] &xp: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][file/src/common/mod.rs::305] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin",
)
[TRACE][file/src/common/mod.rs::307] path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][file/src/common/mod.rs::314] path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][file/src/common/mod.rs::315] recheck_method: Copy
[TRACE][file/src/common/mod.rs::321] "Before copy": "Before copy"
[TRACE][walker/src/lib.rs::688] ignore_root: "[CWD]"
[TRACE][file/src/common/mod.rs::322] &cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin",
)
[TRACE][walker/src/lib.rs::689] ignore_path: "[CWD]/dir-0001/.gitignore"
[TRACE][file/src/common/mod.rs::323] &path: AbsolutePath(
    "[CWD]/dir-0001/file-0001.bin",
)
[TRACE][walker/src/lib.rs::697] &content: "### Following 3 lines are added by xvc on Wed, 22 Nov 2023 20:40:41 +0000
/file-0003.bin
/file-0001.bin
/file-0002.bin
"
[TRACE][file/src/common/mod.rs::327] &perm: Permissions(
    FilePermissions {
        mode: 33060,
    },
)
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[INFO] [CARRY] dir-0001/file-0003.bin -> b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin
[INFO] [EXISTS] [CWD]/.xvc/b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin for dir-0002/file-0003.bin
[INFO] [REMOVE] [CWD]/dir-0002/file-0003.bin
[INFO] [COPY] [CWD]/.xvc/b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin -> [CWD]/dir-0002/file-0003.bin
[INFO] [RECHECK] b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin -> dir-0002/file-0003.bin
[INFO] [CARRY] dir-0001/file-0002.bin -> b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin
[INFO] [COPY] [CWD]/.xvc/b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin -> [CWD]/dir-0001/file-0002.bin
[INFO] [CARRY] dir-0001/file-0001.bin -> b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin
[INFO] [RECHECK] b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin -> dir-0001/file-0002.bin
[INFO] [COPY] [CWD]/.xvc/b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin -> [CWD]/dir-0001/file-0001.bin
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 1 regexes
[TRACE][file/src/common/mod.rs::323] &path: AbsolutePath(
    "[CWD]/dir-0001/file-0003.bin",
)
thread '<unnamed>' panicked at file/src/carry_in/mod.rs:280:13:
IoError { source: Os { code: 13, kind: PermissionDenied, message: "Permission denied" } }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[TRACE][file/src/common/mod.rs::327] &perm: Permissions(
    FilePermissions {
        mode: 33060,
    },
)
[TRACE][file/src/common/mod.rs::329] &perm: Permissions(
    FilePermissions {
        mode: 33206,
    },
)
[TRACE][file/src/common/mod.rs::367] "Return recheck_from_cache": "Return recheck_from_cache"
[TRACE][file/src/carry_in/mod.rs::304] &cache_path: XvcCachePath(
    "b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::305] recheck_method: Copy
[TRACE][file/src/carry_in/mod.rs::306] &xp: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][file/src/common/mod.rs::440] &file_perm.clone(): Permissions(
    FilePermissions {
        mode: 33060,
    },
)
[TRACE][file/src/carry_in/mod.rs::287] target_path: AbsolutePath(
    "[CWD]/dir-0002/file-0002.bin",
)
[TRACE][file/src/common/mod.rs::290] parent: XvcPath(
    "dir-0002",
)
[TRACE][file/src/common/mod.rs::292] parent_dir: AbsolutePath(
    "[CWD]/dir-0002",
)
[TRACE][file/src/common/mod.rs::305] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin",
)
[TRACE][file/src/common/mod.rs::307] path: AbsolutePath(
    "[CWD]/dir-0002/file-0002.bin",
)
[TRACE][file/src/common/mod.rs::314] path: AbsolutePath(
    "[CWD]/dir-0002/file-0002.bin",
)
[TRACE][file/src/common/mod.rs::315] recheck_method: Copy
[TRACE][file/src/common/mod.rs::321] "Before copy": "Before copy"
[TRACE][file/src/common/mod.rs::322] &cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin",
)
[TRACE][file/src/common/mod.rs::323] &path: AbsolutePath(
    "[CWD]/dir-0002/file-0002.bin",
)
[TRACE][file/src/common/mod.rs::327] &perm: Permissions(
    FilePermissions {
        mode: 33060,
    },
)
[TRACE][file/src/common/mod.rs::329] &perm: Permissions(
    FilePermissions {
        mode: 33206,
    },
)
[TRACE][file/src/common/mod.rs::367] "Return recheck_from_cache": "Return recheck_from_cache"
[TRACE][file/src/carry_in/mod.rs::304] &cache_path: XvcCachePath(
    "b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin",
)
[TRACE][file/src/carry_in/mod.rs::305] recheck_method: Copy
[TRACE][file/src/carry_in/mod.rs::306] &xp: XvcPath(
    "dir-0002/file-0002.bin",
)
[TRACE][file/src/common/mod.rs::329] &perm: Permissions(
    FilePermissions {
        mode: 33206,
    },
)
thread '[TRACE][file/src/common/mod.rs::367] "Return recheck_from_cache": "Return recheck_from_cache"
<unnamed>' panicked at lib/src/cli/mod.rs[TRACE][file/src/carry_in/mod.rs::304] &cache_path: XvcCachePath(
    "b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin",
)
:[TRACE][file/src/carry_in/mod.rs::305] recheck_method: Copy
300:[TRACE][file/src/carry_in/mod.rs::306] &xp: XvcPath(
    "dir-0001/file-0001.bin",
)
52:
[PANIC] IoError { source: Os { code: 13, kind: PermissionDenied, message: "Permission denied" } }, [file/src/carry_in/mod.rs::280]
[TRACE][walker/src/lib.rs::688] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::689] ignore_path: "[CWD]/dir-0002/.gitignore"
[TRACE][walker/src/lib.rs::697] &content: "### Following 3 lines are added by xvc on Wed, 22 Nov 2023 20:40:41 +0000
/file-0003.bin
/file-0001.bin
/file-0002.bin
"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 1 regexes
thread 'main' panicked at lib/src/cli/mod.rs:406:37:
called `Result::unwrap()` on an `Err` value: Any { .. }
[TRACE][walker/src/lib.rs::688] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::689] ignore_path: "[CWD]/.gitignore"
[TRACE][walker/src/lib.rs::697] &content: "
## Following are required for Xvc to function correctly.
.xvc/*
!.xvc/store/
!.xvc/ec/
!.xvc/config.toml

"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 1 regexes
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes

```

Once you added the file to the cache, you can delete the workspace copy.

```console
$ rm dir-0001/file-0001.bin
$ lsd -l dir-0001/file-*
total[..]
drwxr-xr-x [..] dir-0001
drwxr-xr-x [..] dir-0002

```

Then, recheck the file. By default, it makes a copy of the file.

```console
$ xvc file recheck dir-0001/file-0001.bin

$ lsd -l
.rw-rw-rw- [..] data.txt

```

You can track and recheck complete directories

```console
$ xvc file track dir-0002/
$ rm -rf dir-0002/
$ xvc -v file recheck dir-0002/
$ ls -l dir-0002/
total 24
-rw-rw-rw- [..] file-0001.bin
-rw-rw-rw- [..] file-0002.bin
-rw-rw-rw- [..] file-0003.bin

```
You can use glob patterns to recheck files.
```console
```console
$ xvc file track 'dir-*'


You can update the recheck method of a file. Otherwise it will be kept as same before.

```console
$ rm -rf dir-0002/
$ xvc -v file recheck dir-0002/ --as symlink
$ ls -l dir-0002/
total 0
lrwxr-xr-x [..] file-0001.bin -> [CWD]/.xvc/b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin
lrwxr-xr-x [..] file-0002.bin -> [CWD]/.xvc/b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin
lrwxr-xr-x [..] file-0003.bin -> [CWD]/.xvc/b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin

$ rm -rf dir-0002/
$ xvc -v file recheck dir-0002/ 

$ ls -l dir-0002/
total 0
lrwxr-xr-x [..] file-0001.bin -> [CWD]/.xvc/b3/3c9/255/424e13d9c38a37c5ddd376e1070cdd5de66996fbc82194c462f653856d/0.bin
lrwxr-xr-x [..] file-0002.bin -> [CWD]/.xvc/b3/6bc/65f/581e3a03edb127b63b71c5690be176e2fe265266f70abc65f72613f62e/0.bin
lrwxr-xr-x [..] file-0003.bin -> [CWD]/.xvc/b3/804/fb8/edbb122e735facd7f943c1bbe754e939a968f385c12f56b10411a4a015/0.bin

```

Symlink and hardlinks are read-only.
You can recheck as copy to update.

```console
$ zsh -c 'echo "120912" >> dir-0002/file-0001.bin'
? 1
zsh:1: permission denied: dir-0002/file-0001.bin

$ xvc file recheck dir-0002/file-0001.bin --as copy

$ zsh -c 'echo "120912" >> dir-0002/file-0001.bin'

```
Note that, as files in the cache are kept read-only, hardlinks and symlinks are also read only. Files rechecked as copy are made read-write explicitly.

```console
$ xvc -vv file recheck data.txt --as hardlink

$ ls -l
total[..]
drwxr-xr-x [..] dir-0001
drwxr-xr-x [..] dir-0002

```

Reflinks are supported by Xvc, but the underlying file system should also support it.
Otherwise it uses `copy`.

```console
$ rm -f data.txt
$ xvc file recheck data.txt --as reflink

```

The above command will create a read only link in macOS APFS and a copy in ext4 or NTFS file systems.


