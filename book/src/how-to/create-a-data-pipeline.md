# How to create a data pipeline with Xvc

A data pipeline starts from data and ends with models. 

```console
$ git init
Initialized empty Git repository in [CWD]/.git/

$ xvc init
```


```console
$ tree data
data
├── contracts
│   ├── A Consulting Agreement- Consumer Recreations Services V10.DOC
│   ├── AGREEMENT TO SETTLE (BCHRT).docx
│   ├── House-Rental-Contract (HLoom).docx
│   ├── Investment-Contract (HLoom).docx
│   ├── Limited Warranty (Pro remodeler).docx
│   ├── Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx
│   ├── Non-Compete (Signaturely).docx
│   ├── Project-Manager-Contract (Hloom).docx
│   ├── Roofing Contract (Signaturely).docx
│   ├── Services Contract -Cyberdyne Systems V12.docx
│   ├── Website Work-for_hire (Signaturely).docx
│   └── XYZ Corp Employment Agreement.docx
└── non-contracts
    ├── 10 steps for marketing your law firm.docx
    ├── 20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx
    ├── Determining Culture Fit.docx
    ├── How Does Working In-House Differ from Private Practice_.docx
    ├── Invoice (HLoom).docx
    ├── Is Remote Work Working.docx
    ├── Women who broke barriers in the music industry.docx
    └── invoice-spiceimporter.docx

3 directories, 20 files

```



```console
$ xvc --debug file track --no-parallel data
? 101
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
    "core.quiet": Boolean(
        false,
    ),
    "core.verbosity": String(
        "quiet",
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
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "git.command": String(
                    "git",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "pipeline.process_pool_size": Integer(
                    4,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "core.guid": String(
                    "a3def4755628fb77",
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "pipeline.process_pool_size": Integer(
                    4,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.recheck.method": String(
                    "copy",
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
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "core.guid": String(
                    "5a6d6f2cbd2b0f4c",
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.text_or_binary": String(
                    "auto",
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
                "core.quiet": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "quiet",
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
        "file.list.no_summary": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "quiet",
            ),
        },
        "file.carry-in.no_parallel": XvcConfigValue {
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
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "5a6d6f2cbd2b0f4c",
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
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
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
        "file.track.force": XvcConfigValue {
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
        "pipeline.process_pool_size": XvcConfigValue {
            source: Project,
            value: Integer(
                4,
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
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
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
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
            ),
        },
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
            ),
        },
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"a3def4755628fb77/"
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
    "[CWD]/.xvc/ec/1700210973594996",
    "[CWD]/.xvc/ec/1700210973599528",
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
            no_parallel: true,
            targets: Some(
                [
                    "data",
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
                    tv_sec: 1700210973,
                    tv_nsec: 595144502,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700210973,
                    tv_nsec: 617155870,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700210973,
                    tv_nsec: 590860551,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/convert-to-text.zsh",
        metadata: Metadata {
            file_type: FileType(
                FileType {
                    mode: 33261,
                },
            ),
            is_dir: false,
            is_file: true,
            permissions: Permissions(
                FilePermissions {
                    mode: 33261,
                },
            ),
            modified: Ok(
                SystemTime {
                    tv_sec: 1700163709,
                    tv_nsec: 431025363,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 220622557,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163709,
                    tv_nsec: 430634536,
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
                    tv_sec: 1700210973,
                    tv_nsec: 596499902,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700210973,
                    tv_nsec: 631561203,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700210973,
                    tv_nsec: 596406737,
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
                    tv_sec: 1700210973,
                    tv_nsec: 626104308,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700210973,
                    tv_nsec: 177390822,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700210973,
                    tv_nsec: 177390822,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data",
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
                    tv_sec: 1700210973,
                    tv_nsec: 94593035,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700210973,
                    tv_nsec: 635928652,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700210973,
                    tv_nsec: 94280039,
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
                    tv_sec: 1700210973,
                    tv_nsec: 595705328,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700210973,
                    tv_nsec: 632057863,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700210973,
                    tv_nsec: 595360416,
                },
            ),
            ..
        },
    },
]
[TRACE][walker/src/lib.rs::424] child_path.path: "[CWD]/.xvc"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/convert-to-text.zsh"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/.gitignore"
[TRACE][walker/src/lib.rs::424] child_path.path: "[CWD]/.git"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data"
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
[TRACE][walker/src/lib.rs::407] child_paths: [
    PathMetadata {
        path: "[CWD]/data/.DS_Store",
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
                    tv_sec: 1700163690,
                    tv_nsec: 60510471,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 59905188,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 59905188,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/contracts",
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
                    tv_sec: 1700210973,
                    tv_nsec: 96309266,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700210973,
                    tv_nsec: 635967693,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700210973,
                    tv_nsec: 94321330,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/non-contracts",
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
                    tv_sec: 1700210973,
                    tv_nsec: 97426627,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700210973,
                    tv_nsec: 635970568,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700210973,
                    tv_nsec: 94366913,
                },
            ),
            ..
        },
    },
]
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/.DS_Store"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/contracts"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/non-contracts"
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
        path: "[CWD]/data/contracts/Investment-Contract (HLoom).docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 60969173,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 220952585,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 60704135,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/contracts/.DS_Store",
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
                    tv_sec: 1700163690,
                    tv_nsec: 61256711,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 61046922,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 61046922,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/contracts/Limited Warranty (Pro remodeler).docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 61441083,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221015082,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 61321668,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/contracts/AGREEMENT TO SETTLE (BCHRT).docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 61995117,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 220819757,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 61502791,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/contracts/House-Rental-Contract (HLoom).docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 62479403,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 220885213,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 62060450,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/contracts/XYZ Corp Employment Agreement.docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 63048728,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221464230,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 62541068,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/contracts/A Consulting Agreement- Consumer Recreations Services V10.DOC",
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
                    tv_sec: 1700163690,
                    tv_nsec: 63950049,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 220748385,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 63116602,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/contracts/Project-Manager-Contract (Hloom).docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 64109505,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221203449,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 64022715,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/contracts/Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 64280670,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221081038,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 64176880,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/contracts/Website Work-for_hire (Signaturely).docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 64436501,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221401024,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 64347502,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/contracts/Non-Compete (Signaturely).docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 64629915,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221141702,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 64504500,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/contracts/Roofing Contract (Signaturely).docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 64779496,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221265655,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 64692331,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/contracts/Services Contract -Cyberdyne Systems V12.docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 65227740,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221333527,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 64846954,
                },
            ),
            ..
        },
    },
]
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/contracts/Investment-Contract (HLoom).docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/contracts/.DS_Store"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/contracts/Limited Warranty (Pro remodeler).docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/contracts/AGREEMENT TO SETTLE (BCHRT).docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/contracts/House-Rental-Contract (HLoom).docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/contracts/XYZ Corp Employment Agreement.docx"
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
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/contracts/A Consulting Agreement- Consumer Recreations Services V10.DOC"
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
        path: "[CWD]/data/non-contracts/Invoice (HLoom).docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 65650234,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221777133,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 65415071,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/non-contracts/How Does Working In-House Differ from Private Practice_.docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 65962772,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221725010,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 65719984,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/non-contracts/Determining Culture Fit.docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 66122937,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221668638,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 66025021,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/non-contracts/invoice-spiceimporter.docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 66382308,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221947584,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 66190810,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/non-contracts/20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 66540472,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221607765,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 66451349,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/non-contracts/.DS_Store",
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
                    tv_sec: 1700163690,
                    tv_nsec: 66810635,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 66603388,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 66603388,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/non-contracts/10 steps for marketing your law firm.docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 66977300,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221535893,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 66874509,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/non-contracts/Women who broke barriers in the music industry.docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 67429585,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221891712,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 67048382,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/data/non-contracts/Is Remote Work Working.docx",
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
                    tv_sec: 1700163690,
                    tv_nsec: 67812955,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1700209945,
                    tv_nsec: 221838714,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1700163690,
                    tv_nsec: 67491960,
                },
            ),
            ..
        },
    },
]
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/contracts/Project-Manager-Contract (Hloom).docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/contracts/Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/contracts/Website Work-for_hire (Signaturely).docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/contracts/Non-Compete (Signaturely).docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/contracts/Roofing Contract (Signaturely).docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/contracts/Services Contract -Cyberdyne Systems V12.docx"
[TRACE][walker/src/lib.rs::452] "End of walk_parallel": "End of walk_parallel"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/non-contracts/Invoice (HLoom).docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/non-contracts/How Does Working In-House Differ from Private Practice_.docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/non-contracts/Determining Culture Fit.docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/non-contracts/invoice-spiceimporter.docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/non-contracts/20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/non-contracts/.DS_Store"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/non-contracts/10 steps for marketing your law firm.docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/non-contracts/Women who broke barriers in the music industry.docx"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/data/non-contracts/Is Remote Work Working.docx"
[TRACE][walker/src/lib.rs::452] "End of walk_parallel": "End of walk_parallel"
[TRACE][walker/src/lib.rs::452] "End of walk_parallel": "End of walk_parallel"
[TRACE][walker/src/lib.rs::452] "End of walk_parallel": "End of walk_parallel"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/convert-to-text.zsh"
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
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data"
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
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/.DS_Store"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/contracts"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/non-contracts"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/contracts/Investment-Contract (HLoom).docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/contracts/.DS_Store"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/contracts/Limited Warranty (Pro remodeler).docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/contracts/AGREEMENT TO SETTLE (BCHRT).docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/contracts/House-Rental-Contract (HLoom).docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/contracts/XYZ Corp Employment Agreement.docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/contracts/A Consulting Agreement- Consumer Recreations Services V10.DOC"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/contracts/Project-Manager-Contract (Hloom).docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/contracts/Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/contracts/Website Work-for_hire (Signaturely).docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/contracts/Non-Compete (Signaturely).docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/contracts/Roofing Contract (Signaturely).docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/contracts/Services Contract -Cyberdyne Systems V12.docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/non-contracts/Invoice (HLoom).docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/non-contracts/How Does Working In-House Differ from Private Practice_.docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/non-contracts/Determining Culture Fit.docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/non-contracts/invoice-spiceimporter.docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/non-contracts/20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/non-contracts/.DS_Store"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/non-contracts/10 steps for marketing your law firm.docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/non-contracts/Women who broke barriers in the music industry.docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data/non-contracts/Is Remote Work Working.docx"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][file/src/common/mod.rs::210] all_paths: {
    XvcPath(
        "data/contracts",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            480,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700210973,
                tv_nsec: 96309266,
            },
        ),
    },
    XvcPath(
        "data/contracts/Roofing Contract (Signaturely).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            17302,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64779496,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/.DS_Store",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            6148,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66810635,
            },
        ),
    },
    XvcPath(
        "data/.DS_Store",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            6148,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 60510471,
            },
        ),
    },
    XvcPath(
        "data/contracts/Investment-Contract (HLoom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            22290,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 60969173,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/invoice-spiceimporter.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            21719,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66382308,
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
                tv_sec: 1700210973,
                tv_nsec: 596499902,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/10 steps for marketing your law firm.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            11133,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66977300,
            },
        ),
    },
    XvcPath(
        "data/contracts/Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            45096,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64280670,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/Women who broke barriers in the music industry.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            9441,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 67429585,
            },
        ),
    },
    XvcPath(
        "data/contracts/Limited Warranty (Pro remodeler).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            19263,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 61441083,
            },
        ),
    },
    XvcPath(
        "data/contracts/Services Contract -Cyberdyne Systems V12.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            40728,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 65227740,
            },
        ),
    },
    XvcPath(
        "data/contracts/.DS_Store",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            6148,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 61256711,
            },
        ),
    },
    XvcPath(
        "data/contracts/XYZ Corp Employment Agreement.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            42357,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 63048728,
            },
        ),
    },
    XvcPath(
        "data/non-contracts",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            352,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700210973,
                tv_nsec: 97426627,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/Invoice (HLoom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            43002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 65650234,
            },
        ),
    },
    XvcPath(
        "data/contracts/A Consulting Agreement- Consumer Recreations Services V10.DOC",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            102912,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 63950049,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            21169,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66540472,
            },
        ),
    },
    XvcPath(
        "data/contracts/Non-Compete (Signaturely).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            8301,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64629915,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/How Does Working In-House Differ from Private Practice_.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            8224,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 65962772,
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
                tv_sec: 1700210973,
                tv_nsec: 595705328,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/Is Remote Work Working.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            8950,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 67812955,
            },
        ),
    },
    XvcPath(
        "convert-to-text.zsh",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            157,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163709,
                tv_nsec: 431025363,
            },
        ),
    },
    XvcPath(
        "data/contracts/Project-Manager-Contract (Hloom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            24201,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64109505,
            },
        ),
    },
    XvcPath(
        "data/contracts/House-Rental-Contract (HLoom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            23062,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 62479403,
            },
        ),
    },
    XvcPath(
        "data",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            160,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700210973,
                tv_nsec: 94593035,
            },
        ),
    },
    XvcPath(
        "data/contracts/AGREEMENT TO SETTLE (BCHRT).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            17930,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 61995117,
            },
        ),
    },
    XvcPath(
        "data/contracts/Website Work-for_hire (Signaturely).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            18700,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64436501,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/Determining Culture Fit.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            10144,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66122937,
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
                    * 000032: /x00-/x04 => 48, /x05 => 128, /x06-/x08 => 48
                     matches: 0
                     >000048: /x00-/x04 => 48, /x05 => 128, /x06-/x08 => 48
                      000064: /x00-/x04 => 0, /x05 => 128, /x06-/x08 => 0
                      000080: /x00-/x04 => 48, /x05 => 128, /x06 => 48, /x07 => 96, /x08 => 48
                      000096: /x00-/x02 => 48, /x03 => 112, /x04 => 48, /x05 => 128, /x06-/x08 => 48
                      000112: /x00 => 48, /x01 => 32, /x02-/x04 => 48, /x05 => 128, /x06-/x08 => 48
                      000128: /x00-/x02 => 48, /x03 => 80, /x04 => 48, /x05 => 128, /x06-/x08 => 48
                    match kind: Standard
                    prefilter: true
                    state length: 9
                    pattern length: 1
                    shortest pattern length: 5
                    longest pattern length: 5
                    alphabet length: 9
                    stride: 16
                    byte classes: ByteClasses(0 => [0-46], 1 => [47], 2 => [48-96], 3 => [97], 4 => [98-99], 5 => [100], 6 => [101-115], 7 => [116], 8 => [117-255])
                    memory usage: 613
                    )
                    ,
                ),
                map: [
                    0,
                ],
                longest: 5,
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
        "data/non-contracts/Is Remote Work Working.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            8950,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 67812955,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/Determining Culture Fit.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            10144,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66122937,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/invoice-spiceimporter.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            21719,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66382308,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/Invoice (HLoom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            43002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 65650234,
            },
        ),
    },
    XvcPath(
        "data/contracts",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            480,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700210973,
                tv_nsec: 96309266,
            },
        ),
    },
    XvcPath(
        "data/contracts/A Consulting Agreement- Consumer Recreations Services V10.DOC",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            102912,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 63950049,
            },
        ),
    },
    XvcPath(
        "data/contracts/Investment-Contract (HLoom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            22290,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 60969173,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            21169,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66540472,
            },
        ),
    },
    XvcPath(
        "data/contracts/House-Rental-Contract (HLoom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            23062,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 62479403,
            },
        ),
    },
    XvcPath(
        "data/contracts/Services Contract -Cyberdyne Systems V12.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            40728,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 65227740,
            },
        ),
    },
    XvcPath(
        "data/contracts/AGREEMENT TO SETTLE (BCHRT).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            17930,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 61995117,
            },
        ),
    },
    XvcPath(
        "data/contracts/Website Work-for_hire (Signaturely).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            18700,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64436501,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/.DS_Store",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            6148,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66810635,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/10 steps for marketing your law firm.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            11133,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66977300,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/Women who broke barriers in the music industry.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            9441,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 67429585,
            },
        ),
    },
    XvcPath(
        "data/contracts/Limited Warranty (Pro remodeler).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            19263,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 61441083,
            },
        ),
    },
    XvcPath(
        "data/non-contracts",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            352,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700210973,
                tv_nsec: 97426627,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/How Does Working In-House Differ from Private Practice_.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            8224,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 65962772,
            },
        ),
    },
    XvcPath(
        "data/contracts/Project-Manager-Contract (Hloom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            24201,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64109505,
            },
        ),
    },
    XvcPath(
        "data/contracts/XYZ Corp Employment Agreement.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            42357,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 63048728,
            },
        ),
    },
    XvcPath(
        "data/.DS_Store",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            6148,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 60510471,
            },
        ),
    },
    XvcPath(
        "data/contracts/Roofing Contract (Signaturely).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            17302,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64779496,
            },
        ),
    },
    XvcPath(
        "data/contracts/.DS_Store",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            6148,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 61256711,
            },
        ),
    },
    XvcPath(
        "data/contracts/Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            45096,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64280670,
            },
        ),
    },
    XvcPath(
        "data/contracts/Non-Compete (Signaturely).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            8301,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64629915,
            },
        ),
    },
}
[TRACE][file/src/common/compare.rs::38] pmm: {
    XvcPath(
        "data/non-contracts/Is Remote Work Working.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            8950,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 67812955,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/Determining Culture Fit.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            10144,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66122937,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/invoice-spiceimporter.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            21719,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66382308,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/Invoice (HLoom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            43002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 65650234,
            },
        ),
    },
    XvcPath(
        "data/contracts",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            480,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700210973,
                tv_nsec: 96309266,
            },
        ),
    },
    XvcPath(
        "data/contracts/A Consulting Agreement- Consumer Recreations Services V10.DOC",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            102912,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 63950049,
            },
        ),
    },
    XvcPath(
        "data/contracts/Investment-Contract (HLoom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            22290,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 60969173,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            21169,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66540472,
            },
        ),
    },
    XvcPath(
        "data/contracts/House-Rental-Contract (HLoom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            23062,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 62479403,
            },
        ),
    },
    XvcPath(
        "data/contracts/Services Contract -Cyberdyne Systems V12.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            40728,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 65227740,
            },
        ),
    },
    XvcPath(
        "data/contracts/AGREEMENT TO SETTLE (BCHRT).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            17930,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 61995117,
            },
        ),
    },
    XvcPath(
        "data/contracts/Website Work-for_hire (Signaturely).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            18700,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64436501,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/.DS_Store",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            6148,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66810635,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/10 steps for marketing your law firm.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            11133,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66977300,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/Women who broke barriers in the music industry.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            9441,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 67429585,
            },
        ),
    },
    XvcPath(
        "data/contracts/Limited Warranty (Pro remodeler).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            19263,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 61441083,
            },
        ),
    },
    XvcPath(
        "data/non-contracts",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            352,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700210973,
                tv_nsec: 97426627,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/How Does Working In-House Differ from Private Practice_.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            8224,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 65962772,
            },
        ),
    },
    XvcPath(
        "data/contracts/Project-Manager-Contract (Hloom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            24201,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64109505,
            },
        ),
    },
    XvcPath(
        "data/contracts/XYZ Corp Employment Agreement.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            42357,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 63048728,
            },
        ),
    },
    XvcPath(
        "data/.DS_Store",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            6148,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 60510471,
            },
        ),
    },
    XvcPath(
        "data/contracts/Roofing Contract (Signaturely).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            17302,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64779496,
            },
        ),
    },
    XvcPath(
        "data/contracts/.DS_Store",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            6148,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 61256711,
            },
        ),
    },
    XvcPath(
        "data/contracts/Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            45096,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64280670,
            },
        ),
    },
    XvcPath(
        "data/contracts/Non-Compete (Signaturely).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            8301,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64629915,
            },
        ),
    },
}
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/non-contracts/Is Remote Work Working.docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    2,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/non-contracts/Determining Culture Fit.docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    3,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/non-contracts/invoice-spiceimporter.docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    4,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/non-contracts/Invoice (HLoom).docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    5,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/contracts",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    6,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/contracts/A Consulting Agreement- Consumer Recreations Services V10.DOC",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    7,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/contracts/Investment-Contract (HLoom).docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    8,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/non-contracts/20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    9,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/contracts/House-Rental-Contract (HLoom).docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    10,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/contracts/Services Contract -Cyberdyne Systems V12.docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    11,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/contracts/AGREEMENT TO SETTLE (BCHRT).docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    12,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/contracts/Website Work-for_hire (Signaturely).docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    13,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/non-contracts/.DS_Store",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    14,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/non-contracts/10 steps for marketing your law firm.docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    15,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/non-contracts/Women who broke barriers in the music industry.docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    16,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/contracts/Limited Warranty (Pro remodeler).docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    17,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/non-contracts",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    18,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/non-contracts/How Does Working In-House Differ from Private Practice_.docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    19,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/contracts/Project-Manager-Contract (Hloom).docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    20,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/contracts/XYZ Corp Employment Agreement.docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    21,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/.DS_Store",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    22,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/contracts/Roofing Contract (Signaturely).docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    23,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/contracts/.DS_Store",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    24,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/contracts/Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    25,
    18200810214204044675,
)
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data/contracts/Non-Compete (Signaturely).docx",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    26,
    18200810214204044675,
)
[TRACE][file/src/common/compare.rs::457] file_entities: {
    XvcEntity(
        16,
        18200810214204044675,
    ),
    XvcEntity(
        2,
        18200810214204044675,
    ),
    XvcEntity(
        21,
        18200810214204044675,
    ),
    XvcEntity(
        12,
        18200810214204044675,
    ),
    XvcEntity(
        4,
        18200810214204044675,
    ),
    XvcEntity(
        7,
        18200810214204044675,
    ),
    XvcEntity(
        5,
        18200810214204044675,
    ),
    XvcEntity(
        19,
        18200810214204044675,
    ),
    XvcEntity(
        11,
        18200810214204044675,
    ),
    XvcEntity(
        8,
        18200810214204044675,
    ),
    XvcEntity(
        25,
        18200810214204044675,
    ),
    XvcEntity(
        23,
        18200810214204044675,
    ),
    XvcEntity(
        13,
        18200810214204044675,
    ),
    XvcEntity(
        24,
        18200810214204044675,
    ),
    XvcEntity(
        3,
        18200810214204044675,
    ),
    XvcEntity(
        15,
        18200810214204044675,
    ),
    XvcEntity(
        14,
        18200810214204044675,
    ),
    XvcEntity(
        22,
        18200810214204044675,
    ),
    XvcEntity(
        26,
        18200810214204044675,
    ),
    XvcEntity(
        20,
        18200810214204044675,
    ),
    XvcEntity(
        9,
        18200810214204044675,
    ),
    XvcEntity(
        17,
        18200810214204044675,
    ),
    XvcEntity(
        10,
        18200810214204044675,
    ),
}
[TRACE][file/src/common/compare.rs::468] dir_entities: {
    XvcEntity(
        6,
        18200810214204044675,
    ),
    XvcEntity(
        18,
        18200810214204044675,
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/non-contracts/Women who broke barriers in the music industry.docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/non-contracts/Women who broke barriers in the music industry.docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/non-contracts/Women who broke barriers in the music industry.docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            198,
            130,
            225,
            139,
            142,
            135,
            26,
            80,
            26,
            32,
            51,
            109,
            79,
            180,
            228,
            11,
            138,
            178,
            128,
            34,
            228,
            226,
            187,
            1,
            14,
            117,
            75,
            28,
            217,
            226,
            110,
            44,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            198,
            130,
            225,
            139,
            142,
            135,
            26,
            80,
            26,
            32,
            51,
            109,
            79,
            180,
            228,
            11,
            138,
            178,
            128,
            34,
            228,
            226,
            187,
            1,
            14,
            117,
            75,
            28,
            217,
            226,
            110,
            44,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                198,
                130,
                225,
                139,
                142,
                135,
                26,
                80,
                26,
                32,
                51,
                109,
                79,
                180,
                228,
                11,
                138,
                178,
                128,
                34,
                228,
                226,
                187,
                1,
                14,
                117,
                75,
                28,
                217,
                226,
                110,
                44,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/non-contracts/Is Remote Work Working.docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/non-contracts/Is Remote Work Working.docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/non-contracts/Is Remote Work Working.docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            253,
            3,
            63,
            211,
            248,
            131,
            219,
            46,
            130,
            137,
            98,
            4,
            83,
            133,
            5,
            159,
            25,
            160,
            48,
            251,
            152,
            126,
            196,
            190,
            231,
            68,
            254,
            180,
            77,
            157,
            18,
            17,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            253,
            3,
            63,
            211,
            248,
            131,
            219,
            46,
            130,
            137,
            98,
            4,
            83,
            133,
            5,
            159,
            25,
            160,
            48,
            251,
            152,
            126,
            196,
            190,
            231,
            68,
            254,
            180,
            77,
            157,
            18,
            17,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                253,
                3,
                63,
                211,
                248,
                131,
                219,
                46,
                130,
                137,
                98,
                4,
                83,
                133,
                5,
                159,
                25,
                160,
                48,
                251,
                152,
                126,
                196,
                190,
                231,
                68,
                254,
                180,
                77,
                157,
                18,
                17,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/contracts/XYZ Corp Employment Agreement.docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/contracts/XYZ Corp Employment Agreement.docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/contracts/XYZ Corp Employment Agreement.docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            141,
            17,
            62,
            17,
            192,
            139,
            168,
            60,
            26,
            180,
            158,
            132,
            11,
            199,
            11,
            119,
            184,
            88,
            222,
            58,
            0,
            132,
            104,
            135,
            40,
            229,
            210,
            251,
            121,
            167,
            221,
            35,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            141,
            17,
            62,
            17,
            192,
            139,
            168,
            60,
            26,
            180,
            158,
            132,
            11,
            199,
            11,
            119,
            184,
            88,
            222,
            58,
            0,
            132,
            104,
            135,
            40,
            229,
            210,
            251,
            121,
            167,
            221,
            35,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                141,
                17,
                62,
                17,
                192,
                139,
                168,
                60,
                26,
                180,
                158,
                132,
                11,
                199,
                11,
                119,
                184,
                88,
                222,
                58,
                0,
                132,
                104,
                135,
                40,
                229,
                210,
                251,
                121,
                167,
                221,
                35,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/contracts/AGREEMENT TO SETTLE (BCHRT).docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/contracts/AGREEMENT TO SETTLE (BCHRT).docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/contracts/AGREEMENT TO SETTLE (BCHRT).docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            60,
            141,
            114,
            229,
            5,
            15,
            124,
            179,
            208,
            107,
            139,
            87,
            228,
            82,
            176,
            162,
            77,
            254,
            245,
            39,
            202,
            96,
            78,
            8,
            65,
            199,
            87,
            132,
            166,
            7,
            56,
            231,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            60,
            141,
            114,
            229,
            5,
            15,
            124,
            179,
            208,
            107,
            139,
            87,
            228,
            82,
            176,
            162,
            77,
            254,
            245,
            39,
            202,
            96,
            78,
            8,
            65,
            199,
            87,
            132,
            166,
            7,
            56,
            231,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                60,
                141,
                114,
                229,
                5,
                15,
                124,
                179,
                208,
                107,
                139,
                87,
                228,
                82,
                176,
                162,
                77,
                254,
                245,
                39,
                202,
                96,
                78,
                8,
                65,
                199,
                87,
                132,
                166,
                7,
                56,
                231,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/non-contracts/invoice-spiceimporter.docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/non-contracts/invoice-spiceimporter.docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/non-contracts/invoice-spiceimporter.docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            190,
            117,
            25,
            70,
            74,
            197,
            61,
            220,
            12,
            108,
            42,
            111,
            157,
            196,
            217,
            63,
            76,
            120,
            60,
            228,
            62,
            46,
            106,
            154,
            218,
            179,
            13,
            193,
            221,
            29,
            188,
            123,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            190,
            117,
            25,
            70,
            74,
            197,
            61,
            220,
            12,
            108,
            42,
            111,
            157,
            196,
            217,
            63,
            76,
            120,
            60,
            228,
            62,
            46,
            106,
            154,
            218,
            179,
            13,
            193,
            221,
            29,
            188,
            123,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                190,
                117,
                25,
                70,
                74,
                197,
                61,
                220,
                12,
                108,
                42,
                111,
                157,
                196,
                217,
                63,
                76,
                120,
                60,
                228,
                62,
                46,
                106,
                154,
                218,
                179,
                13,
                193,
                221,
                29,
                188,
                123,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/contracts/A Consulting Agreement- Consumer Recreations Services V10.DOC",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/contracts/A Consulting Agreement- Consumer Recreations Services V10.DOC",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/contracts/A Consulting Agreement- Consumer Recreations Services V10.DOC",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            217,
            212,
            153,
            214,
            72,
            135,
            48,
            210,
            13,
            18,
            253,
            19,
            190,
            232,
            238,
            143,
            114,
            87,
            206,
            211,
            93,
            150,
            80,
            32,
            119,
            33,
            22,
            99,
            129,
            243,
            47,
            69,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            217,
            212,
            153,
            214,
            72,
            135,
            48,
            210,
            13,
            18,
            253,
            19,
            190,
            232,
            238,
            143,
            114,
            87,
            206,
            211,
            93,
            150,
            80,
            32,
            119,
            33,
            22,
            99,
            129,
            243,
            47,
            69,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                217,
                212,
                153,
                214,
                72,
                135,
                48,
                210,
                13,
                18,
                253,
                19,
                190,
                232,
                238,
                143,
                114,
                87,
                206,
                211,
                93,
                150,
                80,
                32,
                119,
                33,
                22,
                99,
                129,
                243,
                47,
                69,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/non-contracts/Invoice (HLoom).docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/non-contracts/Invoice (HLoom).docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/non-contracts/Invoice (HLoom).docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            211,
            10,
            224,
            175,
            180,
            211,
            234,
            167,
            173,
            251,
            55,
            97,
            57,
            206,
            197,
            6,
            3,
            84,
            201,
            220,
            39,
            120,
            104,
            213,
            234,
            39,
            96,
            20,
            35,
            21,
            163,
            198,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            211,
            10,
            224,
            175,
            180,
            211,
            234,
            167,
            173,
            251,
            55,
            97,
            57,
            206,
            197,
            6,
            3,
            84,
            201,
            220,
            39,
            120,
            104,
            213,
            234,
            39,
            96,
            20,
            35,
            21,
            163,
            198,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                211,
                10,
                224,
                175,
                180,
                211,
                234,
                167,
                173,
                251,
                55,
                97,
                57,
                206,
                197,
                6,
                3,
                84,
                201,
                220,
                39,
                120,
                104,
                213,
                234,
                39,
                96,
                20,
                35,
                21,
                163,
                198,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/non-contracts/How Does Working In-House Differ from Private Practice_.docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/non-contracts/How Does Working In-House Differ from Private Practice_.docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/non-contracts/How Does Working In-House Differ from Private Practice_.docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            100,
            18,
            204,
            44,
            7,
            36,
            123,
            96,
            236,
            87,
            36,
            108,
            12,
            250,
            55,
            211,
            7,
            158,
            23,
            247,
            4,
            177,
            38,
            224,
            241,
            56,
            217,
            151,
            191,
            139,
            207,
            225,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            100,
            18,
            204,
            44,
            7,
            36,
            123,
            96,
            236,
            87,
            36,
            108,
            12,
            250,
            55,
            211,
            7,
            158,
            23,
            247,
            4,
            177,
            38,
            224,
            241,
            56,
            217,
            151,
            191,
            139,
            207,
            225,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                100,
                18,
                204,
                44,
                7,
                36,
                123,
                96,
                236,
                87,
                36,
                108,
                12,
                250,
                55,
                211,
                7,
                158,
                23,
                247,
                4,
                177,
                38,
                224,
                241,
                56,
                217,
                151,
                191,
                139,
                207,
                225,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/contracts/Services Contract -Cyberdyne Systems V12.docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/contracts/Services Contract -Cyberdyne Systems V12.docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/contracts/Services Contract -Cyberdyne Systems V12.docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            56,
            168,
            86,
            108,
            233,
            78,
            94,
            61,
            64,
            183,
            76,
            11,
            106,
            175,
            34,
            19,
            177,
            184,
            117,
            167,
            149,
            40,
            138,
            58,
            161,
            197,
            125,
            75,
            58,
            16,
            233,
            227,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            56,
            168,
            86,
            108,
            233,
            78,
            94,
            61,
            64,
            183,
            76,
            11,
            106,
            175,
            34,
            19,
            177,
            184,
            117,
            167,
            149,
            40,
            138,
            58,
            161,
            197,
            125,
            75,
            58,
            16,
            233,
            227,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                56,
                168,
                86,
                108,
                233,
                78,
                94,
                61,
                64,
                183,
                76,
                11,
                106,
                175,
                34,
                19,
                177,
                184,
                117,
                167,
                149,
                40,
                138,
                58,
                161,
                197,
                125,
                75,
                58,
                16,
                233,
                227,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/contracts/Investment-Contract (HLoom).docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/contracts/Investment-Contract (HLoom).docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/contracts/Investment-Contract (HLoom).docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            195,
            99,
            214,
            230,
            88,
            205,
            107,
            139,
            79,
            146,
            188,
            168,
            118,
            136,
            228,
            169,
            248,
            38,
            201,
            111,
            223,
            81,
            156,
            64,
            215,
            26,
            64,
            45,
            168,
            13,
            18,
            151,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            195,
            99,
            214,
            230,
            88,
            205,
            107,
            139,
            79,
            146,
            188,
            168,
            118,
            136,
            228,
            169,
            248,
            38,
            201,
            111,
            223,
            81,
            156,
            64,
            215,
            26,
            64,
            45,
            168,
            13,
            18,
            151,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                195,
                99,
                214,
                230,
                88,
                205,
                107,
                139,
                79,
                146,
                188,
                168,
                118,
                136,
                228,
                169,
                248,
                38,
                201,
                111,
                223,
                81,
                156,
                64,
                215,
                26,
                64,
                45,
                168,
                13,
                18,
                151,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/contracts/Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/contracts/Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/contracts/Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            26,
            49,
            86,
            120,
            34,
            250,
            158,
            114,
            129,
            12,
            212,
            37,
            112,
            89,
            225,
            1,
            36,
            98,
            77,
            56,
            204,
            237,
            164,
            172,
            203,
            249,
            100,
            35,
            37,
            116,
            102,
            226,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            26,
            49,
            86,
            120,
            34,
            250,
            158,
            114,
            129,
            12,
            212,
            37,
            112,
            89,
            225,
            1,
            36,
            98,
            77,
            56,
            204,
            237,
            164,
            172,
            203,
            249,
            100,
            35,
            37,
            116,
            102,
            226,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                26,
                49,
                86,
                120,
                34,
                250,
                158,
                114,
                129,
                12,
                212,
                37,
                112,
                89,
                225,
                1,
                36,
                98,
                77,
                56,
                204,
                237,
                164,
                172,
                203,
                249,
                100,
                35,
                37,
                116,
                102,
                226,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/contracts/Roofing Contract (Signaturely).docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/contracts/Roofing Contract (Signaturely).docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/contracts/Roofing Contract (Signaturely).docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            187,
            56,
            156,
            181,
            102,
            222,
            50,
            71,
            30,
            124,
            222,
            156,
            132,
            20,
            231,
            31,
            34,
            75,
            214,
            92,
            19,
            254,
            82,
            227,
            100,
            91,
            91,
            29,
            45,
            127,
            252,
            85,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            187,
            56,
            156,
            181,
            102,
            222,
            50,
            71,
            30,
            124,
            222,
            156,
            132,
            20,
            231,
            31,
            34,
            75,
            214,
            92,
            19,
            254,
            82,
            227,
            100,
            91,
            91,
            29,
            45,
            127,
            252,
            85,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                187,
                56,
                156,
                181,
                102,
                222,
                50,
                71,
                30,
                124,
                222,
                156,
                132,
                20,
                231,
                31,
                34,
                75,
                214,
                92,
                19,
                254,
                82,
                227,
                100,
                91,
                91,
                29,
                45,
                127,
                252,
                85,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/contracts/Website Work-for_hire (Signaturely).docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/contracts/Website Work-for_hire (Signaturely).docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/contracts/Website Work-for_hire (Signaturely).docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            82,
            241,
            157,
            196,
            71,
            70,
            154,
            106,
            137,
            143,
            196,
            25,
            74,
            98,
            17,
            109,
            202,
            84,
            216,
            153,
            149,
            236,
            159,
            225,
            42,
            202,
            141,
            2,
            251,
            209,
            228,
            42,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            82,
            241,
            157,
            196,
            71,
            70,
            154,
            106,
            137,
            143,
            196,
            25,
            74,
            98,
            17,
            109,
            202,
            84,
            216,
            153,
            149,
            236,
            159,
            225,
            42,
            202,
            141,
            2,
            251,
            209,
            228,
            42,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                82,
                241,
                157,
                196,
                71,
                70,
                154,
                106,
                137,
                143,
                196,
                25,
                74,
                98,
                17,
                109,
                202,
                84,
                216,
                153,
                149,
                236,
                159,
                225,
                42,
                202,
                141,
                2,
                251,
                209,
                228,
                42,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/contracts/.DS_Store",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/contracts/.DS_Store",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/contracts/.DS_Store",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            235,
            121,
            69,
            35,
            241,
            36,
            122,
            69,
            106,
            24,
            83,
            24,
            161,
            232,
            0,
            18,
            36,
            68,
            166,
            51,
            241,
            208,
            34,
            169,
            58,
            8,
            160,
            139,
            88,
            14,
            7,
            72,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            235,
            121,
            69,
            35,
            241,
            36,
            122,
            69,
            106,
            24,
            83,
            24,
            161,
            232,
            0,
            18,
            36,
            68,
            166,
            51,
            241,
            208,
            34,
            169,
            58,
            8,
            160,
            139,
            88,
            14,
            7,
            72,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                235,
                121,
                69,
                35,
                241,
                36,
                122,
                69,
                106,
                24,
                83,
                24,
                161,
                232,
                0,
                18,
                36,
                68,
                166,
                51,
                241,
                208,
                34,
                169,
                58,
                8,
                160,
                139,
                88,
                14,
                7,
                72,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/non-contracts/Determining Culture Fit.docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/non-contracts/Determining Culture Fit.docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/non-contracts/Determining Culture Fit.docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            213,
            94,
            23,
            50,
            21,
            109,
            15,
            61,
            86,
            87,
            175,
            19,
            30,
            113,
            182,
            113,
            37,
            194,
            30,
            195,
            115,
            138,
            84,
            237,
            144,
            222,
            239,
            165,
            168,
            219,
            182,
            147,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            213,
            94,
            23,
            50,
            21,
            109,
            15,
            61,
            86,
            87,
            175,
            19,
            30,
            113,
            182,
            113,
            37,
            194,
            30,
            195,
            115,
            138,
            84,
            237,
            144,
            222,
            239,
            165,
            168,
            219,
            182,
            147,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                213,
                94,
                23,
                50,
                21,
                109,
                15,
                61,
                86,
                87,
                175,
                19,
                30,
                113,
                182,
                113,
                37,
                194,
                30,
                195,
                115,
                138,
                84,
                237,
                144,
                222,
                239,
                165,
                168,
                219,
                182,
                147,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/non-contracts/10 steps for marketing your law firm.docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/non-contracts/10 steps for marketing your law firm.docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/non-contracts/10 steps for marketing your law firm.docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            213,
            78,
            143,
            168,
            230,
            39,
            93,
            128,
            243,
            254,
            107,
            87,
            29,
            176,
            239,
            183,
            20,
            31,
            177,
            56,
            28,
            27,
            210,
            234,
            161,
            200,
            3,
            41,
            17,
            100,
            109,
            46,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            213,
            78,
            143,
            168,
            230,
            39,
            93,
            128,
            243,
            254,
            107,
            87,
            29,
            176,
            239,
            183,
            20,
            31,
            177,
            56,
            28,
            27,
            210,
            234,
            161,
            200,
            3,
            41,
            17,
            100,
            109,
            46,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                213,
                78,
                143,
                168,
                230,
                39,
                93,
                128,
                243,
                254,
                107,
                87,
                29,
                176,
                239,
                183,
                20,
                31,
                177,
                56,
                28,
                27,
                210,
                234,
                161,
                200,
                3,
                41,
                17,
                100,
                109,
                46,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/non-contracts/.DS_Store",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/non-contracts/.DS_Store",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/non-contracts/.DS_Store",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            43,
            3,
            143,
            145,
            232,
            24,
            32,
            216,
            28,
            184,
            205,
            47,
            74,
            28,
            227,
            79,
            200,
            4,
            77,
            235,
            82,
            187,
            22,
            14,
            71,
            250,
            192,
            160,
            232,
            45,
            39,
            10,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            43,
            3,
            143,
            145,
            232,
            24,
            32,
            216,
            28,
            184,
            205,
            47,
            74,
            28,
            227,
            79,
            200,
            4,
            77,
            235,
            82,
            187,
            22,
            14,
            71,
            250,
            192,
            160,
            232,
            45,
            39,
            10,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                43,
                3,
                143,
                145,
                232,
                24,
                32,
                216,
                28,
                184,
                205,
                47,
                74,
                28,
                227,
                79,
                200,
                4,
                77,
                235,
                82,
                187,
                22,
                14,
                71,
                250,
                192,
                160,
                232,
                45,
                39,
                10,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/.DS_Store",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/.DS_Store",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/.DS_Store",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            144,
            81,
            228,
            149,
            75,
            108,
            103,
            104,
            7,
            10,
            129,
            159,
            230,
            63,
            173,
            15,
            148,
            162,
            186,
            25,
            3,
            100,
            215,
            170,
            203,
            191,
            141,
            156,
            97,
            72,
            200,
            204,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            144,
            81,
            228,
            149,
            75,
            108,
            103,
            104,
            7,
            10,
            129,
            159,
            230,
            63,
            173,
            15,
            148,
            162,
            186,
            25,
            3,
            100,
            215,
            170,
            203,
            191,
            141,
            156,
            97,
            72,
            200,
            204,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                144,
                81,
                228,
                149,
                75,
                108,
                103,
                104,
                7,
                10,
                129,
                159,
                230,
                63,
                173,
                15,
                148,
                162,
                186,
                25,
                3,
                100,
                215,
                170,
                203,
                191,
                141,
                156,
                97,
                72,
                200,
                204,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/contracts/Non-Compete (Signaturely).docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/contracts/Non-Compete (Signaturely).docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/contracts/Non-Compete (Signaturely).docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            240,
            59,
            125,
            5,
            72,
            126,
            178,
            227,
            154,
            224,
            72,
            33,
            91,
            63,
            89,
            13,
            135,
            68,
            161,
            105,
            131,
            156,
            48,
            48,
            42,
            170,
            220,
            68,
            247,
            201,
            162,
            177,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            240,
            59,
            125,
            5,
            72,
            126,
            178,
            227,
            154,
            224,
            72,
            33,
            91,
            63,
            89,
            13,
            135,
            68,
            161,
            105,
            131,
            156,
            48,
            48,
            42,
            170,
            220,
            68,
            247,
            201,
            162,
            177,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                240,
                59,
                125,
                5,
                72,
                126,
                178,
                227,
                154,
                224,
                72,
                33,
                91,
                63,
                89,
                13,
                135,
                68,
                161,
                105,
                131,
                156,
                48,
                48,
                42,
                170,
                220,
                68,
                247,
                201,
                162,
                177,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/contracts/Project-Manager-Contract (Hloom).docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/contracts/Project-Manager-Contract (Hloom).docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/contracts/Project-Manager-Contract (Hloom).docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            222,
            22,
            9,
            80,
            93,
            229,
            249,
            166,
            55,
            70,
            76,
            131,
            216,
            231,
            123,
            34,
            221,
            17,
            102,
            244,
            5,
            161,
            58,
            20,
            195,
            212,
            27,
            172,
            200,
            97,
            32,
            116,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            222,
            22,
            9,
            80,
            93,
            229,
            249,
            166,
            55,
            70,
            76,
            131,
            216,
            231,
            123,
            34,
            221,
            17,
            102,
            244,
            5,
            161,
            58,
            20,
            195,
            212,
            27,
            172,
            200,
            97,
            32,
            116,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                222,
                22,
                9,
                80,
                93,
                229,
                249,
                166,
                55,
                70,
                76,
                131,
                216,
                231,
                123,
                34,
                221,
                17,
                102,
                244,
                5,
                161,
                58,
                20,
                195,
                212,
                27,
                172,
                200,
                97,
                32,
                116,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/non-contracts/20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/non-contracts/20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/non-contracts/20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            218,
            10,
            8,
            169,
            104,
            68,
            188,
            162,
            128,
            99,
            48,
            92,
            24,
            191,
            11,
            240,
            39,
            66,
            213,
            118,
            89,
            176,
            111,
            93,
            87,
            191,
            201,
            175,
            119,
            171,
            226,
            32,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            218,
            10,
            8,
            169,
            104,
            68,
            188,
            162,
            128,
            99,
            48,
            92,
            24,
            191,
            11,
            240,
            39,
            66,
            213,
            118,
            89,
            176,
            111,
            93,
            87,
            191,
            201,
            175,
            119,
            171,
            226,
            32,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                218,
                10,
                8,
                169,
                104,
                68,
                188,
                162,
                128,
                99,
                48,
                92,
                24,
                191,
                11,
                240,
                39,
                66,
                213,
                118,
                89,
                176,
                111,
                93,
                87,
                191,
                201,
                175,
                119,
                171,
                226,
                32,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/contracts/Limited Warranty (Pro remodeler).docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/contracts/Limited Warranty (Pro remodeler).docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/contracts/Limited Warranty (Pro remodeler).docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            99,
            140,
            24,
            89,
            29,
            45,
            130,
            228,
            98,
            60,
            155,
            211,
            103,
            63,
            118,
            53,
            186,
            127,
            113,
            248,
            201,
            239,
            202,
            90,
            47,
            170,
            251,
            204,
            255,
            237,
            87,
            112,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            99,
            140,
            24,
            89,
            29,
            45,
            130,
            228,
            98,
            60,
            155,
            211,
            103,
            63,
            118,
            53,
            186,
            127,
            113,
            248,
            201,
            239,
            202,
            90,
            47,
            170,
            251,
            204,
            255,
            237,
            87,
            112,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                99,
                140,
                24,
                89,
                29,
                45,
                130,
                228,
                98,
                60,
                155,
                211,
                103,
                63,
                118,
                53,
                186,
                127,
                113,
                248,
                201,
                239,
                202,
                90,
                47,
                170,
                251,
                204,
                255,
                237,
                87,
                112,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::150] xvc_path_diff: RecordMissing {
    actual: XvcPath(
        "data/contracts/House-Rental-Contract (HLoom).docx",
    ),
}
[TRACE][file/src/common/compare.rs::191] actual: XvcPath(
    "data/contracts/House-Rental-Contract (HLoom).docx",
)
[TRACE][file/src/common/compare.rs::193] path: AbsolutePath(
    "[CWD]/data/contracts/House-Rental-Contract (HLoom).docx",
)
[TRACE][file/src/common/compare.rs::195] actual_digest: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            192,
            255,
            225,
            124,
            51,
            74,
            8,
            117,
            20,
            170,
            202,
            81,
            5,
            15,
            152,
            107,
            10,
            125,
            2,
            78,
            181,
            19,
            54,
            207,
            175,
            226,
            211,
            176,
            29,
            118,
            5,
            195,
        ],
    },
)
[TRACE][file/src/common/compare.rs::133] stored_content_digest: None
[TRACE][file/src/common/compare.rs::134] actual: ContentDigest(
    XvcDigest {
        algorithm: Blake3,
        digest: [
            192,
            255,
            225,
            124,
            51,
            74,
            8,
            117,
            20,
            170,
            202,
            81,
            5,
            15,
            152,
            107,
            10,
            125,
            2,
            78,
            181,
            19,
            54,
            207,
            175,
            226,
            211,
            176,
            29,
            118,
            5,
            195,
        ],
    },
)
[TRACE][file/src/common/compare.rs::197] res: RecordMissing {
    actual: ContentDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                192,
                255,
                225,
                124,
                51,
                74,
                8,
                117,
                20,
                170,
                202,
                81,
                5,
                15,
                152,
                107,
                10,
                125,
                2,
                78,
                181,
                19,
                54,
                207,
                175,
                226,
                211,
                176,
                29,
                118,
                5,
                195,
            ],
        },
    ),
}
[TRACE][file/src/common/compare.rs::546] file_content_digest_diff_store.keys(): [
    XvcEntity(
        13,
        18200810214204044675,
    ),
    XvcEntity(
        4,
        18200810214204044675,
    ),
    XvcEntity(
        7,
        18200810214204044675,
    ),
    XvcEntity(
        3,
        18200810214204044675,
    ),
    XvcEntity(
        17,
        18200810214204044675,
    ),
    XvcEntity(
        21,
        18200810214204044675,
    ),
    XvcEntity(
        15,
        18200810214204044675,
    ),
    XvcEntity(
        16,
        18200810214204044675,
    ),
    XvcEntity(
        20,
        18200810214204044675,
    ),
    XvcEntity(
        26,
        18200810214204044675,
    ),
    XvcEntity(
        2,
        18200810214204044675,
    ),
    XvcEntity(
        14,
        18200810214204044675,
    ),
    XvcEntity(
        24,
        18200810214204044675,
    ),
    XvcEntity(
        9,
        18200810214204044675,
    ),
    XvcEntity(
        12,
        18200810214204044675,
    ),
    XvcEntity(
        23,
        18200810214204044675,
    ),
    XvcEntity(
        22,
        18200810214204044675,
    ),
    XvcEntity(
        10,
        18200810214204044675,
    ),
    XvcEntity(
        8,
        18200810214204044675,
    ),
    XvcEntity(
        5,
        18200810214204044675,
    ),
    XvcEntity(
        25,
        18200810214204044675,
    ),
    XvcEntity(
        19,
        18200810214204044675,
    ),
    XvcEntity(
        11,
        18200810214204044675,
    ),
]
[TRACE][file/src/track/mod.rs::184] content_digest_diff: HStore {
    map: {
        XvcEntity(
            16,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        198,
                        130,
                        225,
                        139,
                        142,
                        135,
                        26,
                        80,
                        26,
                        32,
                        51,
                        109,
                        79,
                        180,
                        228,
                        11,
                        138,
                        178,
                        128,
                        34,
                        228,
                        226,
                        187,
                        1,
                        14,
                        117,
                        75,
                        28,
                        217,
                        226,
                        110,
                        44,
                    ],
                },
            ),
        },
        XvcEntity(
            4,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        190,
                        117,
                        25,
                        70,
                        74,
                        197,
                        61,
                        220,
                        12,
                        108,
                        42,
                        111,
                        157,
                        196,
                        217,
                        63,
                        76,
                        120,
                        60,
                        228,
                        62,
                        46,
                        106,
                        154,
                        218,
                        179,
                        13,
                        193,
                        221,
                        29,
                        188,
                        123,
                    ],
                },
            ),
        },
        XvcEntity(
            6,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        47,
                        48,
                        45,
                        117,
                        73,
                        53,
                        90,
                        54,
                        125,
                        81,
                        64,
                        40,
                        44,
                        78,
                        254,
                        67,
                        26,
                        27,
                        143,
                        146,
                        89,
                        89,
                        137,
                        238,
                        206,
                        112,
                        215,
                        221,
                        224,
                        241,
                        113,
                        96,
                    ],
                },
            ),
        },
        XvcEntity(
            17,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        99,
                        140,
                        24,
                        89,
                        29,
                        45,
                        130,
                        228,
                        98,
                        60,
                        155,
                        211,
                        103,
                        63,
                        118,
                        53,
                        186,
                        127,
                        113,
                        248,
                        201,
                        239,
                        202,
                        90,
                        47,
                        170,
                        251,
                        204,
                        255,
                        237,
                        87,
                        112,
                    ],
                },
            ),
        },
        XvcEntity(
            12,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        60,
                        141,
                        114,
                        229,
                        5,
                        15,
                        124,
                        179,
                        208,
                        107,
                        139,
                        87,
                        228,
                        82,
                        176,
                        162,
                        77,
                        254,
                        245,
                        39,
                        202,
                        96,
                        78,
                        8,
                        65,
                        199,
                        87,
                        132,
                        166,
                        7,
                        56,
                        231,
                    ],
                },
            ),
        },
        XvcEntity(
            21,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        141,
                        17,
                        62,
                        17,
                        192,
                        139,
                        168,
                        60,
                        26,
                        180,
                        158,
                        132,
                        11,
                        199,
                        11,
                        119,
                        184,
                        88,
                        222,
                        58,
                        0,
                        132,
                        104,
                        135,
                        40,
                        229,
                        210,
                        251,
                        121,
                        167,
                        221,
                        35,
                    ],
                },
            ),
        },
        XvcEntity(
            7,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        217,
                        212,
                        153,
                        214,
                        72,
                        135,
                        48,
                        210,
                        13,
                        18,
                        253,
                        19,
                        190,
                        232,
                        238,
                        143,
                        114,
                        87,
                        206,
                        211,
                        93,
                        150,
                        80,
                        32,
                        119,
                        33,
                        22,
                        99,
                        129,
                        243,
                        47,
                        69,
                    ],
                },
            ),
        },
        XvcEntity(
            15,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        213,
                        78,
                        143,
                        168,
                        230,
                        39,
                        93,
                        128,
                        243,
                        254,
                        107,
                        87,
                        29,
                        176,
                        239,
                        183,
                        20,
                        31,
                        177,
                        56,
                        28,
                        27,
                        210,
                        234,
                        161,
                        200,
                        3,
                        41,
                        17,
                        100,
                        109,
                        46,
                    ],
                },
            ),
        },
        XvcEntity(
            2,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        253,
                        3,
                        63,
                        211,
                        248,
                        131,
                        219,
                        46,
                        130,
                        137,
                        98,
                        4,
                        83,
                        133,
                        5,
                        159,
                        25,
                        160,
                        48,
                        251,
                        152,
                        126,
                        196,
                        190,
                        231,
                        68,
                        254,
                        180,
                        77,
                        157,
                        18,
                        17,
                    ],
                },
            ),
        },
        XvcEntity(
            20,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        222,
                        22,
                        9,
                        80,
                        93,
                        229,
                        249,
                        166,
                        55,
                        70,
                        76,
                        131,
                        216,
                        231,
                        123,
                        34,
                        221,
                        17,
                        102,
                        244,
                        5,
                        161,
                        58,
                        20,
                        195,
                        212,
                        27,
                        172,
                        200,
                        97,
                        32,
                        116,
                    ],
                },
            ),
        },
        XvcEntity(
            26,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        240,
                        59,
                        125,
                        5,
                        72,
                        126,
                        178,
                        227,
                        154,
                        224,
                        72,
                        33,
                        91,
                        63,
                        89,
                        13,
                        135,
                        68,
                        161,
                        105,
                        131,
                        156,
                        48,
                        48,
                        42,
                        170,
                        220,
                        68,
                        247,
                        201,
                        162,
                        177,
                    ],
                },
            ),
        },
        XvcEntity(
            24,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        235,
                        121,
                        69,
                        35,
                        241,
                        36,
                        122,
                        69,
                        106,
                        24,
                        83,
                        24,
                        161,
                        232,
                        0,
                        18,
                        36,
                        68,
                        166,
                        51,
                        241,
                        208,
                        34,
                        169,
                        58,
                        8,
                        160,
                        139,
                        88,
                        14,
                        7,
                        72,
                    ],
                },
            ),
        },
        XvcEntity(
            23,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        187,
                        56,
                        156,
                        181,
                        102,
                        222,
                        50,
                        71,
                        30,
                        124,
                        222,
                        156,
                        132,
                        20,
                        231,
                        31,
                        34,
                        75,
                        214,
                        92,
                        19,
                        254,
                        82,
                        227,
                        100,
                        91,
                        91,
                        29,
                        45,
                        127,
                        252,
                        85,
                    ],
                },
            ),
        },
        XvcEntity(
            25,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        26,
                        49,
                        86,
                        120,
                        34,
                        250,
                        158,
                        114,
                        129,
                        12,
                        212,
                        37,
                        112,
                        89,
                        225,
                        1,
                        36,
                        98,
                        77,
                        56,
                        204,
                        237,
                        164,
                        172,
                        203,
                        249,
                        100,
                        35,
                        37,
                        116,
                        102,
                        226,
                    ],
                },
            ),
        },
        XvcEntity(
            18,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        26,
                        113,
                        193,
                        237,
                        51,
                        36,
                        17,
                        156,
                        191,
                        99,
                        238,
                        245,
                        184,
                        177,
                        235,
                        44,
                        238,
                        10,
                        99,
                        243,
                        35,
                        110,
                        218,
                        69,
                        58,
                        252,
                        115,
                        53,
                        252,
                        173,
                        38,
                        76,
                    ],
                },
            ),
        },
        XvcEntity(
            3,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        213,
                        94,
                        23,
                        50,
                        21,
                        109,
                        15,
                        61,
                        86,
                        87,
                        175,
                        19,
                        30,
                        113,
                        182,
                        113,
                        37,
                        194,
                        30,
                        195,
                        115,
                        138,
                        84,
                        237,
                        144,
                        222,
                        239,
                        165,
                        168,
                        219,
                        182,
                        147,
                    ],
                },
            ),
        },
        XvcEntity(
            14,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        43,
                        3,
                        143,
                        145,
                        232,
                        24,
                        32,
                        216,
                        28,
                        184,
                        205,
                        47,
                        74,
                        28,
                        227,
                        79,
                        200,
                        4,
                        77,
                        235,
                        82,
                        187,
                        22,
                        14,
                        71,
                        250,
                        192,
                        160,
                        232,
                        45,
                        39,
                        10,
                    ],
                },
            ),
        },
        XvcEntity(
            10,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        192,
                        255,
                        225,
                        124,
                        51,
                        74,
                        8,
                        117,
                        20,
                        170,
                        202,
                        81,
                        5,
                        15,
                        152,
                        107,
                        10,
                        125,
                        2,
                        78,
                        181,
                        19,
                        54,
                        207,
                        175,
                        226,
                        211,
                        176,
                        29,
                        118,
                        5,
                        195,
                    ],
                },
            ),
        },
        XvcEntity(
            5,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        211,
                        10,
                        224,
                        175,
                        180,
                        211,
                        234,
                        167,
                        173,
                        251,
                        55,
                        97,
                        57,
                        206,
                        197,
                        6,
                        3,
                        84,
                        201,
                        220,
                        39,
                        120,
                        104,
                        213,
                        234,
                        39,
                        96,
                        20,
                        35,
                        21,
                        163,
                        198,
                    ],
                },
            ),
        },
        XvcEntity(
            11,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        56,
                        168,
                        86,
                        108,
                        233,
                        78,
                        94,
                        61,
                        64,
                        183,
                        76,
                        11,
                        106,
                        175,
                        34,
                        19,
                        177,
                        184,
                        117,
                        167,
                        149,
                        40,
                        138,
                        58,
                        161,
                        197,
                        125,
                        75,
                        58,
                        16,
                        233,
                        227,
                    ],
                },
            ),
        },
        XvcEntity(
            22,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        144,
                        81,
                        228,
                        149,
                        75,
                        108,
                        103,
                        104,
                        7,
                        10,
                        129,
                        159,
                        230,
                        63,
                        173,
                        15,
                        148,
                        162,
                        186,
                        25,
                        3,
                        100,
                        215,
                        170,
                        203,
                        191,
                        141,
                        156,
                        97,
                        72,
                        200,
                        204,
                    ],
                },
            ),
        },
        XvcEntity(
            8,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        195,
                        99,
                        214,
                        230,
                        88,
                        205,
                        107,
                        139,
                        79,
                        146,
                        188,
                        168,
                        118,
                        136,
                        228,
                        169,
                        248,
                        38,
                        201,
                        111,
                        223,
                        81,
                        156,
                        64,
                        215,
                        26,
                        64,
                        45,
                        168,
                        13,
                        18,
                        151,
                    ],
                },
            ),
        },
        XvcEntity(
            13,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        82,
                        241,
                        157,
                        196,
                        71,
                        70,
                        154,
                        106,
                        137,
                        143,
                        196,
                        25,
                        74,
                        98,
                        17,
                        109,
                        202,
                        84,
                        216,
                        153,
                        149,
                        236,
                        159,
                        225,
                        42,
                        202,
                        141,
                        2,
                        251,
                        209,
                        228,
                        42,
                    ],
                },
            ),
        },
        XvcEntity(
            9,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        218,
                        10,
                        8,
                        169,
                        104,
                        68,
                        188,
                        162,
                        128,
                        99,
                        48,
                        92,
                        24,
                        191,
                        11,
                        240,
                        39,
                        66,
                        213,
                        118,
                        89,
                        176,
                        111,
                        93,
                        87,
                        191,
                        201,
                        175,
                        119,
                        171,
                        226,
                        32,
                    ],
                },
            ),
        },
        XvcEntity(
            19,
            18200810214204044675,
        ): RecordMissing {
            actual: ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        100,
                        18,
                        204,
                        44,
                        7,
                        36,
                        123,
                        96,
                        236,
                        87,
                        36,
                        108,
                        12,
                        250,
                        55,
                        211,
                        7,
                        158,
                        23,
                        247,
                        4,
                        177,
                        38,
                        224,
                        241,
                        56,
                        217,
                        151,
                        191,
                        139,
                        207,
                        225,
                    ],
                },
            ),
        },
    },
}
[TRACE][file/src/common/mod.rs::474] records.len(): 0
[TRACE][file/src/common/mod.rs::476] new_store.len(): 25
[TRACE][file/src/common/mod.rs::474] records.len(): 0
[TRACE][file/src/common/mod.rs::476] new_store.len(): 25
[TRACE][file/src/common/mod.rs::474] records.len(): 0
[TRACE][file/src/common/mod.rs::476] new_store.len(): 25
[TRACE][file/src/common/mod.rs::474] records.len(): 0
[TRACE][file/src/common/mod.rs::476] new_store.len(): 25
[TRACE][file/src/common/mod.rs::474] records.len(): 0
[TRACE][file/src/common/mod.rs::476] new_store.len(): 25
[TRACE][file/src/track/mod.rs::192] targets: {
    XvcPath(
        "data/non-contracts/Is Remote Work Working.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            8950,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 67812955,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/Determining Culture Fit.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            10144,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66122937,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/invoice-spiceimporter.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            21719,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66382308,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/Invoice (HLoom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            43002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 65650234,
            },
        ),
    },
    XvcPath(
        "data/contracts",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            480,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700210973,
                tv_nsec: 96309266,
            },
        ),
    },
    XvcPath(
        "data/contracts/A Consulting Agreement- Consumer Recreations Services V10.DOC",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            102912,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 63950049,
            },
        ),
    },
    XvcPath(
        "data/contracts/Investment-Contract (HLoom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            22290,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 60969173,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            21169,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66540472,
            },
        ),
    },
    XvcPath(
        "data/contracts/House-Rental-Contract (HLoom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            23062,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 62479403,
            },
        ),
    },
    XvcPath(
        "data/contracts/Services Contract -Cyberdyne Systems V12.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            40728,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 65227740,
            },
        ),
    },
    XvcPath(
        "data/contracts/AGREEMENT TO SETTLE (BCHRT).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            17930,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 61995117,
            },
        ),
    },
    XvcPath(
        "data/contracts/Website Work-for_hire (Signaturely).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            18700,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64436501,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/.DS_Store",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            6148,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66810635,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/10 steps for marketing your law firm.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            11133,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 66977300,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/Women who broke barriers in the music industry.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            9441,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 67429585,
            },
        ),
    },
    XvcPath(
        "data/contracts/Limited Warranty (Pro remodeler).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            19263,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 61441083,
            },
        ),
    },
    XvcPath(
        "data/non-contracts",
    ): XvcMetadata {
        file_type: Directory,
        size: Some(
            352,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700210973,
                tv_nsec: 97426627,
            },
        ),
    },
    XvcPath(
        "data/non-contracts/How Does Working In-House Differ from Private Practice_.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            8224,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 65962772,
            },
        ),
    },
    XvcPath(
        "data/contracts/Project-Manager-Contract (Hloom).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            24201,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64109505,
            },
        ),
    },
    XvcPath(
        "data/contracts/XYZ Corp Employment Agreement.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            42357,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 63048728,
            },
        ),
    },
    XvcPath(
        "data/.DS_Store",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            6148,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 60510471,
            },
        ),
    },
    XvcPath(
        "data/contracts/Roofing Contract (Signaturely).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            17302,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64779496,
            },
        ),
    },
    XvcPath(
        "data/contracts/.DS_Store",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            6148,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 61256711,
            },
        ),
    },
    XvcPath(
        "data/contracts/Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            45096,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64280670,
            },
        ),
    },
    XvcPath(
        "data/contracts/Non-Compete (Signaturely).docx",
    ): XvcMetadata {
        file_type: File,
        size: Some(
            8301,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1700163690,
                tv_nsec: 64629915,
            },
        ),
    },
}
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/data"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
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
[TRACE][file/src/track/mod.rs::222] file_targets: [
    XvcPath(
        "data/non-contracts/Is Remote Work Working.docx",
    ),
    XvcPath(
        "data/non-contracts/Determining Culture Fit.docx",
    ),
    XvcPath(
        "data/non-contracts/invoice-spiceimporter.docx",
    ),
    XvcPath(
        "data/non-contracts/Invoice (HLoom).docx",
    ),
    XvcPath(
        "data/contracts/A Consulting Agreement- Consumer Recreations Services V10.DOC",
    ),
    XvcPath(
        "data/contracts/Investment-Contract (HLoom).docx",
    ),
    XvcPath(
        "data/non-contracts/20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx",
    ),
    XvcPath(
        "data/contracts/House-Rental-Contract (HLoom).docx",
    ),
    XvcPath(
        "data/contracts/Services Contract -Cyberdyne Systems V12.docx",
    ),
    XvcPath(
        "data/contracts/AGREEMENT TO SETTLE (BCHRT).docx",
    ),
    XvcPath(
        "data/contracts/Website Work-for_hire (Signaturely).docx",
    ),
    XvcPath(
        "data/non-contracts/.DS_Store",
    ),
    XvcPath(
        "data/non-contracts/10 steps for marketing your law firm.docx",
    ),
    XvcPath(
        "data/non-contracts/Women who broke barriers in the music industry.docx",
    ),
    XvcPath(
        "data/contracts/Limited Warranty (Pro remodeler).docx",
    ),
    XvcPath(
        "data/non-contracts/How Does Working In-House Differ from Private Practice_.docx",
    ),
    XvcPath(
        "data/contracts/Project-Manager-Contract (Hloom).docx",
    ),
    XvcPath(
        "data/contracts/XYZ Corp Employment Agreement.docx",
    ),
    XvcPath(
        "data/.DS_Store",
    ),
    XvcPath(
        "data/contracts/Roofing Contract (Signaturely).docx",
    ),
    XvcPath(
        "data/contracts/.DS_Store",
    ),
    XvcPath(
        "data/contracts/Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx",
    ),
    XvcPath(
        "data/contracts/Non-Compete (Signaturely).docx",
    ),
]
[TRACE][file/src/track/mod.rs::223] dir_targets: [
    XvcPath(
        "data",
    ),
]
[TRACE][walker/src/lib.rs::688] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::689] ignore_path: "[CWD]/.gitignore"
[TRACE][walker/src/lib.rs::697] &content: "
## Following are required for Xvc to function correctly.
.xvc/*
!.xvc/store/
!.xvc/ec/
!.xvc/config.toml

### Following 1 lines are added by xvc on Fri, 17 Nov 2023 08:49:33 +0000
/data/
"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/data/**", re: "(?-u)^(?:/|/.*/)data/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('d'), Literal('a'), Literal('t'), Literal('a'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 2 regexes
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/non-contracts/Is Remote Work Working.docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/non-contracts/Determining Culture Fit.docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/non-contracts/invoice-spiceimporter.docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/non-contracts/Invoice (HLoom).docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/contracts/A Consulting Agreement- Consumer Recreations Services V10.DOC
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/contracts/Investment-Contract (HLoom).docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/non-contracts/20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/contracts/House-Rental-Contract (HLoom).docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/contracts/Services Contract -Cyberdyne Systems V12.docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/contracts/AGREEMENT TO SETTLE (BCHRT).docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/contracts/Website Work-for_hire (Signaturely).docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/non-contracts/.DS_Store
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/non-contracts/10 steps for marketing your law firm.docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/non-contracts/Women who broke barriers in the music industry.docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/contracts/Limited Warranty (Pro remodeler).docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/non-contracts/How Does Working In-House Differ from Private Practice_.docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/contracts/Project-Manager-Contract (Hloom).docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/contracts/XYZ Corp Employment Agreement.docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/.DS_Store
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/contracts/Roofing Contract (Signaturely).docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/contracts/.DS_Store
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/contracts/Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx
[INFO][file/src/common/gitignore.rs::184] Already gitignored: data/contracts/Non-Compete (Signaturely).docx
[TRACE][file/src/carry_in/mod.rs::238] ignore_writer: Sender { .. }
[TRACE][file/src/carry_in/mod.rs::239] ignore_thread: JoinHandle { .. }
[TRACE][file/src/carry_in/mod.rs::245] cache_path: XvcCachePath(
    "b3/1a7/1c1/ed3324119cbf63eef5b8b1eb2cee0a63f3236eda453afc7335fcad264c/0.",
)
[TRACE][file/src/carry_in/mod.rs::247] abs_cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/1a7/1c1/ed3324119cbf63eef5b8b1eb2cee0a63f3236eda453afc7335fcad264c/0.",
)
[TRACE][file/src/carry_in/mod.rs::278] &cache_path: XvcCachePath(
    "b3/1a7/1c1/ed3324119cbf63eef5b8b1eb2cee0a63f3236eda453afc7335fcad264c/0.",
)
[TRACE][file/src/carry_in/mod.rs::279] &xp: XvcPath(
    "data/non-contracts",
)
[TRACE][file/src/common/mod.rs::455] path: AbsolutePath(
    "[CWD]/data/non-contracts",
)
[TRACE][file/src/common/mod.rs::457] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/1a7/1c1/ed3324119cbf63eef5b8b1eb2cee0a63f3236eda453afc7335fcad264c/0.",
)
[TRACE][file/src/common/mod.rs::426] cache_dir: "[CWD]/.xvc/b3/1a7/1c1/ed3324119cbf63eef5b8b1eb2cee0a63f3236eda453afc7335fcad264c"
[TRACE][walker/src/lib.rs::688] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::689] ignore_path: "[CWD]/.gitignore"
[TRACE][walker/src/lib.rs::697] &content: "
## Following are required for Xvc to function correctly.
.xvc/*
!.xvc/store/
!.xvc/ec/
!.xvc/config.toml

### Following 1 lines are added by xvc on Fri, 17 Nov 2023 08:49:33 +0000
/data/
"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/data/**", re: "(?-u)^(?:/|/.*/)data/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('d'), Literal('a'), Literal('t'), Literal('a'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 2 regexes
[TRACE][file/src/common/mod.rs::437] &file_perm.clone(): Permissions(
    FilePermissions {
        mode: 16877,
    },
)
[TRACE][file/src/common/mod.rs::440] &file_perm.clone(): Permissions(
    FilePermissions {
        mode: 16749,
    },
)
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true, empty_alternates: false }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[TRACE][file/src/carry_in/mod.rs::287] target_path: AbsolutePath(
    "[CWD]/data/non-contracts",
)
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[TRACE][file/src/common/mod.rs::290] parent: XvcPath(
    "data",
)
[TRACE][file/src/common/mod.rs::292] parent_dir: AbsolutePath(
    "[CWD]/data",
)
[TRACE][file/src/common/mod.rs::305] cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/1a7/1c1/ed3324119cbf63eef5b8b1eb2cee0a63f3236eda453afc7335fcad264c/0.",
)
[TRACE][file/src/common/mod.rs::307] path: AbsolutePath(
    "[CWD]/data/non-contracts",
)
[TRACE][file/src/common/mod.rs::314] path: AbsolutePath(
    "[CWD]/data/non-contracts",
)
[TRACE][file/src/common/mod.rs::315] recheck_method: Copy
[TRACE][file/src/common/mod.rs::321] "Before copy": "Before copy"
[TRACE][file/src/common/mod.rs::322] &cache_path: AbsolutePath(
    "[CWD]/.xvc/b3/1a7/1c1/ed3324119cbf63eef5b8b1eb2cee0a63f3236eda453afc7335fcad264c/0.",
)
[TRACE][file/src/common/mod.rs::323] &path: AbsolutePath(
    "[CWD]/data/non-contracts",
)
thread '<unnamed>' panicked at file/src/carry_in/mod.rs:294:9:
IoError { source: Error { kind: InvalidInput, message: "the source path is neither a regular file nor a symlink to a regular file" } }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at lib/src/cli/mod.rs:263:52:
[PANIC] IoError { source: Error { kind: InvalidInput, message: "the source path is neither a regular file nor a symlink to a regular file" } }, [file/src/carry_in/mod.rs::294]
thread 'main' panicked at lib/src/cli/mod.rs:406:37:
called `Result::unwrap()` on an `Err` value: Any { .. }

$ xvc pipeline step new -s convert-docx-to-txt --command "./convert-docx-to-txt.zsh" 
```


