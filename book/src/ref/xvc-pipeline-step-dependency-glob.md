### Glob Dependencies

A step can depend on multiple files specified with globs. The difference with
this and [glob-items dependency](./xvc-pipeline-step-dependency-glob-items.md)
is that this one doesn't track the files, and doesn't pass the list of files in
environment variables to the command.

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Let's create a set of files:

```console
$ xvc-test-helper create-directory-tree --directories 2 --files 3 --seed 2023

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

Add a step to say files has changed when the files have changed.

```console
$ xvc pipeline step new --step-name files-changed --command "echo 'Files have changed.'"

$ xvc pipeline step dependency --step-name files-changed --glob 'dir-*/*'

```

The step is invalidated when a file described by the glob is added, removed or changed.

```console
$ xvc pipeline run
[OUT] [files-changed] Files have changed.
 
[DONE] files-changed (echo 'Files have changed.')

$ xvc pipeline run

```
When a file is removed from the files described by the glob, the step is invalidated.

```console
$ rm dir-0001/file-0001.bin

$ xvc pipeline run
[DEBUG][logging/src/lib.rs::237] Terminal logger enabled with level: Error
[DEBUG][logging/src/lib.rs::240] File logger enabled with level: Trace to "/var/folders/tk/3vn311ps4kqdhgykj3jg_p8r0000gn/T//xvc.log"
[TRACE][core/src/types/xvcroot.rs::247] "."
[DEBUG][core/src/types/xvcroot.rs::253] XVC DIR: "[CWD]"
[DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][ecs/src/ecs/mod.rs::229] dir: "[CWD]/.xvc/ec"
[TRACE][ecs/src/ecs/mod.rs::239] files: [
    "[CWD]/.xvc/ec/1703621513942353",
    "[CWD]/.xvc/ec/1703621513946275",
    "[CWD]/.xvc/ec/1703621514028997",
    "[CWD]/.xvc/ec/1703621514100984",
]
[TRACE][pipeline/src/pipeline/mod.rs::289] pipeline_e: XvcEntity(
    1,
    18401664171466521307,
)
[TRACE][pipeline/src/pipeline/mod.rs::294] pipeline_steps: HStore {
    map: {
        XvcEntity(
            2,
            15424582325429536234,
        ): XvcStep {
            name: "files-changed",
        },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::297] consider_changed: XvcStore {
    map: {
        XvcEntity(
            2,
            15424582325429536234,
        ): ByDependencies,
    },
    entity_index: {
        ByDependencies: [
            XvcEntity(
                2,
                15424582325429536234,
            ),
        ],
    },
    previous: EventLog(
        [
            Add {
                entity: XvcEntity(
                    2,
                    15424582325429536234,
                ),
                value: ByDependencies,
            },
        ],
    ),
    current: EventLog(
        [],
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::300] all_deps.parents.len(): 1
[TRACE][pipeline/src/pipeline/mod.rs::301] all_deps.children.len(): 1
[TRACE][pipeline/src/pipeline/mod.rs::303] all_outs.parents.len(): 1
[TRACE][pipeline/src/pipeline/mod.rs::304] all_outs.children.len(): 0
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][walker/src/lib.rs::489] ignore_fn: ".xvcignore"
[TRACE][walker/src/lib.rs::491] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::499] entry.path(): "[CWD]/dir-0001"
[TRACE][walker/src/lib.rs::499] entry.path(): "[CWD]/.xvc"
[TRACE][walker/src/lib.rs::499] entry.path(): "[CWD]/dir-0002"
[TRACE][walker/src/lib.rs::499] entry.path(): "[CWD]/.git"
[TRACE][walker/src/lib.rs::504] ignore_path: "[CWD]/.xvcignore"
[TRACE][walker/src/lib.rs::598] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::599] ignore_path: "[CWD]/.xvcignore"
[TRACE][walker/src/lib.rs::607] &content: "
# Add patterns of files xvc should ignore, which could improve
# the performance.
# It's in the same format as .gitignore files.

.DS_Store
"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.13/src/lib.rs::431] built glob set; 0 literals, 3 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][walker/src/lib.rs::749] is_abs: true
[TRACE][walker/src/lib.rs::753] path_str: "[CWD]/dir-0001"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "/dir-0001"
[TRACE][walker/src/lib.rs::489] ignore_fn: ".xvcignore"
[TRACE][walker/src/lib.rs::491] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::749] is_abs: true
[TRACE][walker/src/lib.rs::753] path_str: "[CWD]/.xvc"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "/.xvc"
[TRACE][walker/src/lib.rs::749] is_abs: true
[TRACE][walker/src/lib.rs::753] path_str: "[CWD]/dir-0002"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "/dir-0002"
[TRACE][walker/src/lib.rs::489] ignore_fn: ".xvcignore"
[TRACE][walker/src/lib.rs::491] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::749] is_abs: true
[TRACE][walker/src/lib.rs::753] path_str: "[CWD]/.git"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "/.git"
[TRACE][core/src/util/file.rs::55] ignore_rules: IgnoreRules {
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
}
[TRACE][walker/src/notify.rs::170] watcher: FsEventWatcher {
    paths: 0x0000600000374180,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x00006000026740f0,
    runloop: Some(
        (
            0x00006000038740c0,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][core/src/util/file.rs::126] background_thread: Mutex {
    data: JoinHandle { .. },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::306] &pmp: XvcPathMetadataProvider {
    xvc_root: XvcRootInner {
        absolute_path: AbsolutePath(
            "[CWD]",
        ),
        xvc_dir: AbsolutePath(
            "[CWD]/.xvc",
        ),
        store_dir: AbsolutePath(
            "[CWD]/.xvc/store",
        ),
        config: XvcConfig {
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
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "core.guid": String(
                            "bda17b5d5b37c25d",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "a29807b4fb6778c3",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "git.use_git": Boolean(
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
                    map: {
                        "TRYCMD_TESTS": String(
                            "pipeline",
                        ),
                        "TRYCMD_DURATION": Integer(
                            300,
                        ),
                    },
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
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
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
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "TRYCMD_DURATION": XvcConfigValue {
                    source: Environment,
                    value: Integer(
                        300,
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
                "file.list.show_dot_files": XvcConfigValue {
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
                        "a29807b4fb6778c3",
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "TRYCMD_TESTS": XvcConfigValue {
                    source: Environment,
                    value: String(
                        "pipeline",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"bda17b5d5b37c25d/"
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
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 4,
            random: 18006038384920420146,
            dirty: false,
        },
    },
    path_map: RwLock {
        data: {},
        poisoned: false,
        ..
    },
    kill_switch_sender: Sender { .. },
    background_thread: Mutex {
        data: JoinHandle { .. },
        poisoned: false,
        ..
    },
    output_sender: Sender { .. },
    ignore_rules: IgnoreRules {
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
}
[TRACE][pipeline/src/pipeline/mod.rs::309] pipeline_len: 1
[TRACE][pipeline/src/pipeline/mod.rs::335] &dependency_graph: {
    XvcEntity(
        2,
        15424582325429536234,
    ): [],
}
[TRACE][pipeline/src/pipeline/mod.rs::347] &dependency_graph: {
    XvcEntity(
        2,
        15424582325429536234,
    ): [],
}
[INFO][pipeline/src/pipeline/mod.rs::351] Pipeline Graph:
digraph {
    0 [ label = "(2, 15424582325429536234)" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::416] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                15424582325429536234,
            ): Begin(
                FromInit,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::512] &step_thread_store: HStore {
    map: {
        XvcEntity(
            2,
            15424582325429536234,
        ): ScopedJoinHandle { .. },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::516] (step_e, &jh): (
    XvcEntity(
        2,
        15424582325429536234,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::627] params.recorded_dependencies: R1NStore {
    parents: XvcStore {
        map: {
            XvcEntity(
                2,
                15424582325429536234,
            ): XvcStep {
                name: "files-changed",
            },
        },
        entity_index: {
            XvcStep {
                name: "files-changed",
            }: [
                XvcEntity(
                    2,
                    15424582325429536234,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        2,
                        15424582325429536234,
                    ),
                    value: XvcStep {
                        name: "files-changed",
                    },
                },
                Add {
                    entity: XvcEntity(
                        2,
                        15424582325429536234,
                    ),
                    value: XvcStep {
                        name: "files-changed",
                    },
                },
            ],
        ),
        current: EventLog(
            [],
        ),
    },
    children: XvcStore {
        map: {
            XvcEntity(
                3,
                12694358487624719733,
            ): Glob(
                GlobDep {
                    glob: "dir-*/*",
                    xvc_paths_digest: Some(
                        PathCollectionDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    49,
                                    162,
                                    30,
                                    159,
                                    84,
                                    130,
                                    216,
                                    40,
                                    10,
                                    153,
                                    159,
                                    205,
                                    122,
                                    77,
                                    103,
                                    149,
                                    241,
                                    191,
                                    130,
                                    93,
                                    210,
                                    188,
                                    18,
                                    152,
                                    244,
                                    17,
                                    202,
                                    101,
                                    152,
                                    51,
                                    148,
                                    152,
                                ],
                            },
                        ),
                    ),
                    xvc_metadata_digest: Some(
                        PathCollectionMetadataDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    148,
                                    200,
                                    77,
                                    111,
                                    95,
                                    49,
                                    203,
                                    169,
                                    52,
                                    159,
                                    162,
                                    100,
                                    63,
                                    48,
                                    217,
                                    238,
                                    116,
                                    87,
                                    229,
                                    106,
                                    58,
                                    83,
                                    128,
                                    153,
                                    228,
                                    142,
                                    163,
                                    210,
                                    223,
                                    23,
                                    76,
                                    189,
                                ],
                            },
                        ),
                    ),
                    content_digest: Some(
                        PathCollectionContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    146,
                                    40,
                                    186,
                                    211,
                                    142,
                                    254,
                                    60,
                                    163,
                                    73,
                                    143,
                                    25,
                                    241,
                                    168,
                                    128,
                                    104,
                                    92,
                                    108,
                                    192,
                                    103,
                                    67,
                                    36,
                                    149,
                                    66,
                                    207,
                                    25,
                                    0,
                                    121,
                                    115,
                                    185,
                                    214,
                                    187,
                                    207,
                                ],
                            },
                        ),
                    ),
                },
            ),
        },
        entity_index: {
            Glob(
                GlobDep {
                    glob: "dir-*/*",
                    xvc_paths_digest: Some(
                        PathCollectionDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    49,
                                    162,
                                    30,
                                    159,
                                    84,
                                    130,
                                    216,
                                    40,
                                    10,
                                    153,
                                    159,
                                    205,
                                    122,
                                    77,
                                    103,
                                    149,
                                    241,
                                    191,
                                    130,
                                    93,
                                    210,
                                    188,
                                    18,
                                    152,
                                    244,
                                    17,
                                    202,
                                    101,
                                    152,
                                    51,
                                    148,
                                    152,
                                ],
                            },
                        ),
                    ),
                    xvc_metadata_digest: Some(
                        PathCollectionMetadataDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    148,
                                    200,
                                    77,
                                    111,
                                    95,
                                    49,
                                    203,
                                    169,
                                    52,
                                    159,
                                    162,
                                    100,
                                    63,
                                    48,
                                    217,
                                    238,
                                    116,
                                    87,
                                    229,
                                    106,
                                    58,
                                    83,
                                    128,
                                    153,
                                    228,
                                    142,
                                    163,
                                    210,
                                    223,
                                    23,
                                    76,
                                    189,
                                ],
                            },
                        ),
                    ),
                    content_digest: Some(
                        PathCollectionContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    146,
                                    40,
                                    186,
                                    211,
                                    142,
                                    254,
                                    60,
                                    163,
                                    73,
                                    143,
                                    25,
                                    241,
                                    168,
                                    128,
                                    104,
                                    92,
                                    108,
                                    192,
                                    103,
                                    67,
                                    36,
                                    149,
                                    66,
                                    207,
                                    25,
                                    0,
                                    121,
                                    115,
                                    185,
                                    214,
                                    187,
                                    207,
                                ],
                            },
                        ),
                    ),
                },
            ): [
                XvcEntity(
                    3,
                    12694358487624719733,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        12694358487624719733,
                    ),
                    value: Glob(
                        GlobDep {
                            glob: "dir-*/*",
                            xvc_paths_digest: None,
                            xvc_metadata_digest: None,
                            content_digest: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        3,
                        12694358487624719733,
                    ),
                    value: Glob(
                        GlobDep {
                            glob: "dir-*/*",
                            xvc_paths_digest: Some(
                                PathCollectionDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            49,
                                            162,
                                            30,
                                            159,
                                            84,
                                            130,
                                            216,
                                            40,
                                            10,
                                            153,
                                            159,
                                            205,
                                            122,
                                            77,
                                            103,
                                            149,
                                            241,
                                            191,
                                            130,
                                            93,
                                            210,
                                            188,
                                            18,
                                            152,
                                            244,
                                            17,
                                            202,
                                            101,
                                            152,
                                            51,
                                            148,
                                            152,
                                        ],
                                    },
                                ),
                            ),
                            xvc_metadata_digest: Some(
                                PathCollectionMetadataDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            148,
                                            200,
                                            77,
                                            111,
                                            95,
                                            49,
                                            203,
                                            169,
                                            52,
                                            159,
                                            162,
                                            100,
                                            63,
                                            48,
                                            217,
                                            238,
                                            116,
                                            87,
                                            229,
                                            106,
                                            58,
                                            83,
                                            128,
                                            153,
                                            228,
                                            142,
                                            163,
                                            210,
                                            223,
                                            23,
                                            76,
                                            189,
                                        ],
                                    },
                                ),
                            ),
                            content_digest: Some(
                                PathCollectionContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            146,
                                            40,
                                            186,
                                            211,
                                            142,
                                            254,
                                            60,
                                            163,
                                            73,
                                            143,
                                            25,
                                            241,
                                            168,
                                            128,
                                            104,
                                            92,
                                            108,
                                            192,
                                            103,
                                            67,
                                            36,
                                            149,
                                            66,
                                            207,
                                            25,
                                            0,
                                            121,
                                            115,
                                            185,
                                            214,
                                            187,
                                            207,
                                        ],
                                    },
                                ),
                            ),
                        },
                    ),
                },
            ],
        ),
        current: EventLog(
            [],
        ),
    },
    child_parents: XvcStore {
        map: {
            XvcEntity(
                3,
                12694358487624719733,
            ): ChildEntity(
                XvcEntity(
                    2,
                    15424582325429536234,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
        },
        entity_index: {
            ChildEntity(
                XvcEntity(
                    2,
                    15424582325429536234,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    3,
                    12694358487624719733,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        12694358487624719733,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            15424582325429536234,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
            ],
        ),
        current: EventLog(
            [],
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::628] step_e: XvcEntity(
    2,
    15424582325429536234,
)
[TRACE][pipeline/src/pipeline/mod.rs::568] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::629] dependency_steps(step_e, params.dependency_graph)?: {}
[TRACE][pipeline/src/pipeline/mod.rs::568] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::666] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::776] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::777] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::779] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::666] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::776] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::777] &r_next_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::779] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::666] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::776] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::777] &r_next_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::779] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::666] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1067] parent_entity: XvcEntity(
    2,
    15424582325429536234,
)
[TRACE][pipeline/src/pipeline/mod.rs::1070] deps: HStore {
    map: {
        XvcEntity(
            3,
            12694358487624719733,
        ): Glob(
            GlobDep {
                glob: "dir-*/*",
                xvc_paths_digest: Some(
                    PathCollectionDigest(
                        XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                49,
                                162,
                                30,
                                159,
                                84,
                                130,
                                216,
                                40,
                                10,
                                153,
                                159,
                                205,
                                122,
                                77,
                                103,
                                149,
                                241,
                                191,
                                130,
                                93,
                                210,
                                188,
                                18,
                                152,
                                244,
                                17,
                                202,
                                101,
                                152,
                                51,
                                148,
                                152,
                            ],
                        },
                    ),
                ),
                xvc_metadata_digest: Some(
                    PathCollectionMetadataDigest(
                        XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                148,
                                200,
                                77,
                                111,
                                95,
                                49,
                                203,
                                169,
                                52,
                                159,
                                162,
                                100,
                                63,
                                48,
                                217,
                                238,
                                116,
                                87,
                                229,
                                106,
                                58,
                                83,
                                128,
                                153,
                                228,
                                142,
                                163,
                                210,
                                223,
                                23,
                                76,
                                189,
                            ],
                        },
                    ),
                ),
                content_digest: Some(
                    PathCollectionContentDigest(
                        XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                146,
                                40,
                                186,
                                211,
                                142,
                                254,
                                60,
                                163,
                                73,
                                143,
                                25,
                                241,
                                168,
                                128,
                                104,
                                92,
                                108,
                                192,
                                103,
                                67,
                                36,
                                149,
                                66,
                                207,
                                25,
                                0,
                                121,
                                115,
                                185,
                                214,
                                187,
                                207,
                            ],
                        },
                    ),
                ),
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/deps/compare.rs::426] &stored: Glob(
    GlobDep {
        glob: "dir-*/*",
        xvc_paths_digest: Some(
            PathCollectionDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        49,
                        162,
                        30,
                        159,
                        84,
                        130,
                        216,
                        40,
                        10,
                        153,
                        159,
                        205,
                        122,
                        77,
                        103,
                        149,
                        241,
                        191,
                        130,
                        93,
                        210,
                        188,
                        18,
                        152,
                        244,
                        17,
                        202,
                        101,
                        152,
                        51,
                        148,
                        152,
                    ],
                },
            ),
        ),
        xvc_metadata_digest: Some(
            PathCollectionMetadataDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        148,
                        200,
                        77,
                        111,
                        95,
                        49,
                        203,
                        169,
                        52,
                        159,
                        162,
                        100,
                        63,
                        48,
                        217,
                        238,
                        116,
                        87,
                        229,
                        106,
                        58,
                        83,
                        128,
                        153,
                        228,
                        142,
                        163,
                        210,
                        223,
                        23,
                        76,
                        189,
                    ],
                },
            ),
        ),
        content_digest: Some(
            PathCollectionContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        146,
                        40,
                        186,
                        211,
                        142,
                        254,
                        60,
                        163,
                        73,
                        143,
                        25,
                        241,
                        168,
                        128,
                        104,
                        92,
                        108,
                        192,
                        103,
                        67,
                        36,
                        149,
                        66,
                        207,
                        25,
                        0,
                        121,
                        115,
                        185,
                        214,
                        187,
                        207,
                    ],
                },
            ),
        ),
    },
)
[TRACE][core/src/util/file.rs::215] glob: "dir-*/*"
[TRACE][core/src/util/file.rs::185] glob: "dir-*/*"
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0001/file-0002.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0001/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][core/src/util/file.rs::201] &md: Metadata {
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
            tv_sec: 1703621513,
            tv_nsec: 988701273,
        },
    ),
    accessed: Ok(
        SystemTime {
            tv_sec: 1703621514,
            tv_nsec: 199603463,
        },
    ),
    created: Ok(
        SystemTime {
            tv_sec: 1703621513,
            tv_nsec: 988526232,
        },
    ),
    ..
}
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0001/file-0003.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0001/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][core/src/util/file.rs::201] &md: Metadata {
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
            tv_sec: 1703621513,
            tv_nsec: 989114189,
        },
    ),
    accessed: Ok(
        SystemTime {
            tv_sec: 1703621514,
            tv_nsec: 199712087,
        },
    ),
    created: Ok(
        SystemTime {
            tv_sec: 1703621513,
            tv_nsec: 988730065,
        },
    ),
    ..
}
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0002/file-0001.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0002/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0002/file-0001.bin",
)
[TRACE][core/src/util/file.rs::201] &md: Metadata {
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
            tv_sec: 1703621513,
            tv_nsec: 989363355,
        },
    ),
    accessed: Ok(
        SystemTime {
            tv_sec: 1703621514,
            tv_nsec: 199805754,
        },
    ),
    created: Ok(
        SystemTime {
            tv_sec: 1703621513,
            tv_nsec: 989197147,
        },
    ),
    ..
}
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0002/file-0002.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0002/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0002/file-0002.bin",
)
[TRACE][core/src/util/file.rs::201] &md: Metadata {
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
            tv_sec: 1703621513,
            tv_nsec: 989563646,
        },
    ),
    accessed: Ok(
        SystemTime {
            tv_sec: 1703621514,
            tv_nsec: 199896962,
        },
    ),
    created: Ok(
        SystemTime {
            tv_sec: 1703621513,
            tv_nsec: 989390230,
        },
    ),
    ..
}
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0002/file-0003.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0002/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0002/file-0003.bin",
)
[TRACE][core/src/util/file.rs::201] &md: Metadata {
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
            tv_sec: 1703621513,
            tv_nsec: 989767729,
        },
    ),
    accessed: Ok(
        SystemTime {
            tv_sec: 1703621514,
            tv_nsec: 199984712,
        },
    ),
    created: Ok(
        SystemTime {
            tv_sec: 1703621513,
            tv_nsec: 989589854,
        },
    ),
    ..
}
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0002/file-0003.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0002/file-0001.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0002/file-0002.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][pipeline/src/pipeline/deps/compare.rs::530] actual: GlobDep {
    glob: "dir-*/*",
    xvc_paths_digest: Some(
        PathCollectionDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    173,
                    163,
                    185,
                    62,
                    107,
                    202,
                    4,
                    178,
                    48,
                    9,
                    69,
                    215,
                    247,
                    188,
                    114,
                    224,
                    82,
                    134,
                    138,
                    165,
                    186,
                    154,
                    113,
                    201,
                    102,
                    179,
                    107,
                    10,
                    226,
                    152,
                    2,
                    157,
                ],
            },
        ),
    ),
    xvc_metadata_digest: Some(
        PathCollectionMetadataDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    60,
                    52,
                    240,
                    69,
                    59,
                    102,
                    143,
                    14,
                    146,
                    83,
                    185,
                    17,
                    170,
                    44,
                    73,
                    163,
                    189,
                    223,
                    232,
                    197,
                    72,
                    7,
                    236,
                    145,
                    32,
                    88,
                    195,
                    46,
                    80,
                    5,
                    66,
                    102,
                ],
            },
        ),
    ),
    content_digest: None,
}
[TRACE][pipeline/src/pipeline/deps/glob.rs::137] record: GlobDep {
    glob: "dir-*/*",
    xvc_paths_digest: Some(
        PathCollectionDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    49,
                    162,
                    30,
                    159,
                    84,
                    130,
                    216,
                    40,
                    10,
                    153,
                    159,
                    205,
                    122,
                    77,
                    103,
                    149,
                    241,
                    191,
                    130,
                    93,
                    210,
                    188,
                    18,
                    152,
                    244,
                    17,
                    202,
                    101,
                    152,
                    51,
                    148,
                    152,
                ],
            },
        ),
    ),
    xvc_metadata_digest: Some(
        PathCollectionMetadataDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    148,
                    200,
                    77,
                    111,
                    95,
                    49,
                    203,
                    169,
                    52,
                    159,
                    162,
                    100,
                    63,
                    48,
                    217,
                    238,
                    116,
                    87,
                    229,
                    106,
                    58,
                    83,
                    128,
                    153,
                    228,
                    142,
                    163,
                    210,
                    223,
                    23,
                    76,
                    189,
                ],
            },
        ),
    ),
    content_digest: Some(
        PathCollectionContentDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    146,
                    40,
                    186,
                    211,
                    142,
                    254,
                    60,
                    163,
                    73,
                    143,
                    25,
                    241,
                    168,
                    128,
                    104,
                    92,
                    108,
                    192,
                    103,
                    67,
                    36,
                    149,
                    66,
                    207,
                    25,
                    0,
                    121,
                    115,
                    185,
                    214,
                    187,
                    207,
                ],
            },
        ),
    ),
}
[TRACE][pipeline/src/pipeline/deps/glob.rs::138] actual: GlobDep {
    glob: "dir-*/*",
    xvc_paths_digest: Some(
        PathCollectionDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    173,
                    163,
                    185,
                    62,
                    107,
                    202,
                    4,
                    178,
                    48,
                    9,
                    69,
                    215,
                    247,
                    188,
                    114,
                    224,
                    82,
                    134,
                    138,
                    165,
                    186,
                    154,
                    113,
                    201,
                    102,
                    179,
                    107,
                    10,
                    226,
                    152,
                    2,
                    157,
                ],
            },
        ),
    ),
    xvc_metadata_digest: Some(
        PathCollectionMetadataDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    60,
                    52,
                    240,
                    69,
                    59,
                    102,
                    143,
                    14,
                    146,
                    83,
                    185,
                    17,
                    170,
                    44,
                    73,
                    163,
                    189,
                    223,
                    232,
                    197,
                    72,
                    7,
                    236,
                    145,
                    32,
                    88,
                    195,
                    46,
                    80,
                    5,
                    66,
                    102,
                ],
            },
        ),
    ),
    content_digest: None,
}
[TRACE][core/src/types/diff.rs::295] record: Some(
    PathCollectionDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                49,
                162,
                30,
                159,
                84,
                130,
                216,
                40,
                10,
                153,
                159,
                205,
                122,
                77,
                103,
                149,
                241,
                191,
                130,
                93,
                210,
                188,
                18,
                152,
                244,
                17,
                202,
                101,
                152,
                51,
                148,
                152,
            ],
        },
    ),
)
[TRACE][core/src/types/diff.rs::296] actual: Some(
    PathCollectionDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                173,
                163,
                185,
                62,
                107,
                202,
                4,
                178,
                48,
                9,
                69,
                215,
                247,
                188,
                114,
                224,
                82,
                134,
                138,
                165,
                186,
                154,
                113,
                201,
                102,
                179,
                107,
                10,
                226,
                152,
                2,
                157,
            ],
        },
    ),
)
[TRACE][pipeline/src/pipeline/deps/glob.rs::143] path_collection_diff: Different {
    record: PathCollectionDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                49,
                162,
                30,
                159,
                84,
                130,
                216,
                40,
                10,
                153,
                159,
                205,
                122,
                77,
                103,
                149,
                241,
                191,
                130,
                93,
                210,
                188,
                18,
                152,
                244,
                17,
                202,
                101,
                152,
                51,
                148,
                152,
            ],
        },
    ),
    actual: PathCollectionDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                173,
                163,
                185,
                62,
                107,
                202,
                4,
                178,
                48,
                9,
                69,
                215,
                247,
                188,
                114,
                224,
                82,
                134,
                138,
                165,
                186,
                154,
                113,
                201,
                102,
                179,
                107,
                10,
                226,
                152,
                2,
                157,
            ],
        },
    ),
}
[TRACE][core/src/types/diff.rs::295] record: Some(
    PathCollectionMetadataDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                148,
                200,
                77,
                111,
                95,
                49,
                203,
                169,
                52,
                159,
                162,
                100,
                63,
                48,
                217,
                238,
                116,
                87,
                229,
                106,
                58,
                83,
                128,
                153,
                228,
                142,
                163,
                210,
                223,
                23,
                76,
                189,
            ],
        },
    ),
)
[TRACE][core/src/types/diff.rs::296] actual: Some(
    PathCollectionMetadataDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                60,
                52,
                240,
                69,
                59,
                102,
                143,
                14,
                146,
                83,
                185,
                17,
                170,
                44,
                73,
                163,
                189,
                223,
                232,
                197,
                72,
                7,
                236,
                145,
                32,
                88,
                195,
                46,
                80,
                5,
                66,
                102,
            ],
        },
    ),
)
[TRACE][pipeline/src/pipeline/deps/glob.rs::148] path_collection_metadata_diff: Different {
    record: PathCollectionMetadataDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                148,
                200,
                77,
                111,
                95,
                49,
                203,
                169,
                52,
                159,
                162,
                100,
                63,
                48,
                217,
                238,
                116,
                87,
                229,
                106,
                58,
                83,
                128,
                153,
                228,
                142,
                163,
                210,
                223,
                23,
                76,
                189,
            ],
        },
    ),
    actual: PathCollectionMetadataDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                60,
                52,
                240,
                69,
                59,
                102,
                143,
                14,
                146,
                83,
                185,
                17,
                170,
                44,
                73,
                163,
                189,
                223,
                232,
                197,
                72,
                7,
                236,
                145,
                32,
                88,
                195,
                46,
                80,
                5,
                66,
                102,
            ],
        },
    ),
}
[TRACE][pipeline/src/pipeline/deps/glob.rs::150] "Different": "Different"
[TRACE][pipeline/src/pipeline/mod.rs::1087] step_dependency_diffs: HStore {
    map: {
        XvcEntity(
            3,
            12694358487624719733,
        ): Different {
            record: Glob(
                GlobDep {
                    glob: "dir-*/*",
                    xvc_paths_digest: Some(
                        PathCollectionDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    49,
                                    162,
                                    30,
                                    159,
                                    84,
                                    130,
                                    216,
                                    40,
                                    10,
                                    153,
                                    159,
                                    205,
                                    122,
                                    77,
                                    103,
                                    149,
                                    241,
                                    191,
                                    130,
                                    93,
                                    210,
                                    188,
                                    18,
                                    152,
                                    244,
                                    17,
                                    202,
                                    101,
                                    152,
                                    51,
                                    148,
                                    152,
                                ],
                            },
                        ),
                    ),
                    xvc_metadata_digest: Some(
                        PathCollectionMetadataDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    148,
                                    200,
                                    77,
                                    111,
                                    95,
                                    49,
                                    203,
                                    169,
                                    52,
                                    159,
                                    162,
                                    100,
                                    63,
                                    48,
                                    217,
                                    238,
                                    116,
                                    87,
                                    229,
                                    106,
                                    58,
                                    83,
                                    128,
                                    153,
                                    228,
                                    142,
                                    163,
                                    210,
                                    223,
                                    23,
                                    76,
                                    189,
                                ],
                            },
                        ),
                    ),
                    content_digest: Some(
                        PathCollectionContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    146,
                                    40,
                                    186,
                                    211,
                                    142,
                                    254,
                                    60,
                                    163,
                                    73,
                                    143,
                                    25,
                                    241,
                                    168,
                                    128,
                                    104,
                                    92,
                                    108,
                                    192,
                                    103,
                                    67,
                                    36,
                                    149,
                                    66,
                                    207,
                                    25,
                                    0,
                                    121,
                                    115,
                                    185,
                                    214,
                                    187,
                                    207,
                                ],
                            },
                        ),
                    ),
                },
            ),
            actual: Glob(
                GlobDep {
                    glob: "dir-*/*",
                    xvc_paths_digest: Some(
                        PathCollectionDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    173,
                                    163,
                                    185,
                                    62,
                                    107,
                                    202,
                                    4,
                                    178,
                                    48,
                                    9,
                                    69,
                                    215,
                                    247,
                                    188,
                                    114,
                                    224,
                                    82,
                                    134,
                                    138,
                                    165,
                                    186,
                                    154,
                                    113,
                                    201,
                                    102,
                                    179,
                                    107,
                                    10,
                                    226,
                                    152,
                                    2,
                                    157,
                                ],
                            },
                        ),
                    ),
                    xvc_metadata_digest: Some(
                        PathCollectionMetadataDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    60,
                                    52,
                                    240,
                                    69,
                                    59,
                                    102,
                                    143,
                                    14,
                                    146,
                                    83,
                                    185,
                                    17,
                                    170,
                                    44,
                                    73,
                                    163,
                                    189,
                                    223,
                                    232,
                                    197,
                                    72,
                                    7,
                                    236,
                                    145,
                                    32,
                                    88,
                                    195,
                                    46,
                                    80,
                                    5,
                                    66,
                                    102,
                                ],
                            },
                        ),
                    ),
                    content_digest: None,
                },
            ),
        },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1093] diff: Different {
    record: Glob(
        GlobDep {
            glob: "dir-*/*",
            xvc_paths_digest: Some(
                PathCollectionDigest(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            49,
                            162,
                            30,
                            159,
                            84,
                            130,
                            216,
                            40,
                            10,
                            153,
                            159,
                            205,
                            122,
                            77,
                            103,
                            149,
                            241,
                            191,
                            130,
                            93,
                            210,
                            188,
                            18,
                            152,
                            244,
                            17,
                            202,
                            101,
                            152,
                            51,
                            148,
                            152,
                        ],
                    },
                ),
            ),
            xvc_metadata_digest: Some(
                PathCollectionMetadataDigest(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            148,
                            200,
                            77,
                            111,
                            95,
                            49,
                            203,
                            169,
                            52,
                            159,
                            162,
                            100,
                            63,
                            48,
                            217,
                            238,
                            116,
                            87,
                            229,
                            106,
                            58,
                            83,
                            128,
                            153,
                            228,
                            142,
                            163,
                            210,
                            223,
                            23,
                            76,
                            189,
                        ],
                    },
                ),
            ),
            content_digest: Some(
                PathCollectionContentDigest(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            146,
                            40,
                            186,
                            211,
                            142,
                            254,
                            60,
                            163,
                            73,
                            143,
                            25,
                            241,
                            168,
                            128,
                            104,
                            92,
                            108,
                            192,
                            103,
                            67,
                            36,
                            149,
                            66,
                            207,
                            25,
                            0,
                            121,
                            115,
                            185,
                            214,
                            187,
                            207,
                        ],
                    },
                ),
            ),
        },
    ),
    actual: Glob(
        GlobDep {
            glob: "dir-*/*",
            xvc_paths_digest: Some(
                PathCollectionDigest(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            173,
                            163,
                            185,
                            62,
                            107,
                            202,
                            4,
                            178,
                            48,
                            9,
                            69,
                            215,
                            247,
                            188,
                            114,
                            224,
                            82,
                            134,
                            138,
                            165,
                            186,
                            154,
                            113,
                            201,
                            102,
                            179,
                            107,
                            10,
                            226,
                            152,
                            2,
                            157,
                        ],
                    },
                ),
            ),
            xvc_metadata_digest: Some(
                PathCollectionMetadataDigest(
                    XvcDigest {
                        algorithm: Blake3,
                        digest: [
                            60,
                            52,
                            240,
                            69,
                            59,
                            102,
                            143,
                            14,
                            146,
                            83,
                            185,
                            17,
                            170,
                            44,
                            73,
                            163,
                            189,
                            223,
                            232,
                            197,
                            72,
                            7,
                            236,
                            145,
                            32,
                            88,
                            195,
                            46,
                            80,
                            5,
                            66,
                            102,
                        ],
                    },
                ),
            ),
            content_digest: None,
        },
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::1094] diff.changed(): true
[TRACE][pipeline/src/pipeline/mod.rs::1099] changed: true
[TRACE][pipeline/src/pipeline/mod.rs::776] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::777] &r_next_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::779] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::666] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1124] deps: HStore {
    map: {
        XvcEntity(
            3,
            12694358487624719733,
        ): Glob(
            GlobDep {
                glob: "dir-*/*",
                xvc_paths_digest: Some(
                    PathCollectionDigest(
                        XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                49,
                                162,
                                30,
                                159,
                                84,
                                130,
                                216,
                                40,
                                10,
                                153,
                                159,
                                205,
                                122,
                                77,
                                103,
                                149,
                                241,
                                191,
                                130,
                                93,
                                210,
                                188,
                                18,
                                152,
                                244,
                                17,
                                202,
                                101,
                                152,
                                51,
                                148,
                                152,
                            ],
                        },
                    ),
                ),
                xvc_metadata_digest: Some(
                    PathCollectionMetadataDigest(
                        XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                148,
                                200,
                                77,
                                111,
                                95,
                                49,
                                203,
                                169,
                                52,
                                159,
                                162,
                                100,
                                63,
                                48,
                                217,
                                238,
                                116,
                                87,
                                229,
                                106,
                                58,
                                83,
                                128,
                                153,
                                228,
                                142,
                                163,
                                210,
                                223,
                                23,
                                76,
                                189,
                            ],
                        },
                    ),
                ),
                content_digest: Some(
                    PathCollectionContentDigest(
                        XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                146,
                                40,
                                186,
                                211,
                                142,
                                254,
                                60,
                                163,
                                73,
                                143,
                                25,
                                241,
                                168,
                                128,
                                104,
                                92,
                                108,
                                192,
                                103,
                                67,
                                36,
                                149,
                                66,
                                207,
                                25,
                                0,
                                121,
                                115,
                                185,
                                214,
                                187,
                                207,
                            ],
                        },
                    ),
                ),
            },
        ),
    },
}
[TRACE][core/src/util/file.rs::215] glob: "dir-*/*"
[TRACE][core/src/util/file.rs::185] glob: "dir-*/*"
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0001/file-0002.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0001/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0001/file-0003.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0001/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0002/file-0001.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0002/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0002/file-0001.bin",
)
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0002/file-0002.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0002/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0002/file-0002.bin",
)
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0002/file-0003.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0002/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0002/file-0003.bin",
)
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0002/file-0003.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0002/file-0001.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0002/file-0002.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][pipeline/src/pipeline/deps/glob.rs::137] record: GlobDep {
    glob: "dir-*/*",
    xvc_paths_digest: Some(
        PathCollectionDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    49,
                    162,
                    30,
                    159,
                    84,
                    130,
                    216,
                    40,
                    10,
                    153,
                    159,
                    205,
                    122,
                    77,
                    103,
                    149,
                    241,
                    191,
                    130,
                    93,
                    210,
                    188,
                    18,
                    152,
                    244,
                    17,
                    202,
                    101,
                    152,
                    51,
                    148,
                    152,
                ],
            },
        ),
    ),
    xvc_metadata_digest: Some(
        PathCollectionMetadataDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    148,
                    200,
                    77,
                    111,
                    95,
                    49,
                    203,
                    169,
                    52,
                    159,
                    162,
                    100,
                    63,
                    48,
                    217,
                    238,
                    116,
                    87,
                    229,
                    106,
                    58,
                    83,
                    128,
                    153,
                    228,
                    142,
                    163,
                    210,
                    223,
                    23,
                    76,
                    189,
                ],
            },
        ),
    ),
    content_digest: Some(
        PathCollectionContentDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    146,
                    40,
                    186,
                    211,
                    142,
                    254,
                    60,
                    163,
                    73,
                    143,
                    25,
                    241,
                    168,
                    128,
                    104,
                    92,
                    108,
                    192,
                    103,
                    67,
                    36,
                    149,
                    66,
                    207,
                    25,
                    0,
                    121,
                    115,
                    185,
                    214,
                    187,
                    207,
                ],
            },
        ),
    ),
}
[TRACE][pipeline/src/pipeline/deps/glob.rs::138] actual: GlobDep {
    glob: "dir-*/*",
    xvc_paths_digest: Some(
        PathCollectionDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    173,
                    163,
                    185,
                    62,
                    107,
                    202,
                    4,
                    178,
                    48,
                    9,
                    69,
                    215,
                    247,
                    188,
                    114,
                    224,
                    82,
                    134,
                    138,
                    165,
                    186,
                    154,
                    113,
                    201,
                    102,
                    179,
                    107,
                    10,
                    226,
                    152,
                    2,
                    157,
                ],
            },
        ),
    ),
    xvc_metadata_digest: Some(
        PathCollectionMetadataDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    60,
                    52,
                    240,
                    69,
                    59,
                    102,
                    143,
                    14,
                    146,
                    83,
                    185,
                    17,
                    170,
                    44,
                    73,
                    163,
                    189,
                    223,
                    232,
                    197,
                    72,
                    7,
                    236,
                    145,
                    32,
                    88,
                    195,
                    46,
                    80,
                    5,
                    66,
                    102,
                ],
            },
        ),
    ),
    content_digest: None,
}
[TRACE][core/src/types/diff.rs::295] record: Some(
    PathCollectionDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                49,
                162,
                30,
                159,
                84,
                130,
                216,
                40,
                10,
                153,
                159,
                205,
                122,
                77,
                103,
                149,
                241,
                191,
                130,
                93,
                210,
                188,
                18,
                152,
                244,
                17,
                202,
                101,
                152,
                51,
                148,
                152,
            ],
        },
    ),
)
[TRACE][core/src/types/diff.rs::296] actual: Some(
    PathCollectionDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                173,
                163,
                185,
                62,
                107,
                202,
                4,
                178,
                48,
                9,
                69,
                215,
                247,
                188,
                114,
                224,
                82,
                134,
                138,
                165,
                186,
                154,
                113,
                201,
                102,
                179,
                107,
                10,
                226,
                152,
                2,
                157,
            ],
        },
    ),
)
[TRACE][pipeline/src/pipeline/deps/glob.rs::143] path_collection_diff: Different {
    record: PathCollectionDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                49,
                162,
                30,
                159,
                84,
                130,
                216,
                40,
                10,
                153,
                159,
                205,
                122,
                77,
                103,
                149,
                241,
                191,
                130,
                93,
                210,
                188,
                18,
                152,
                244,
                17,
                202,
                101,
                152,
                51,
                148,
                152,
            ],
        },
    ),
    actual: PathCollectionDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                173,
                163,
                185,
                62,
                107,
                202,
                4,
                178,
                48,
                9,
                69,
                215,
                247,
                188,
                114,
                224,
                82,
                134,
                138,
                165,
                186,
                154,
                113,
                201,
                102,
                179,
                107,
                10,
                226,
                152,
                2,
                157,
            ],
        },
    ),
}
[TRACE][core/src/types/diff.rs::295] record: Some(
    PathCollectionMetadataDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                148,
                200,
                77,
                111,
                95,
                49,
                203,
                169,
                52,
                159,
                162,
                100,
                63,
                48,
                217,
                238,
                116,
                87,
                229,
                106,
                58,
                83,
                128,
                153,
                228,
                142,
                163,
                210,
                223,
                23,
                76,
                189,
            ],
        },
    ),
)
[TRACE][core/src/types/diff.rs::296] actual: Some(
    PathCollectionMetadataDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                60,
                52,
                240,
                69,
                59,
                102,
                143,
                14,
                146,
                83,
                185,
                17,
                170,
                44,
                73,
                163,
                189,
                223,
                232,
                197,
                72,
                7,
                236,
                145,
                32,
                88,
                195,
                46,
                80,
                5,
                66,
                102,
            ],
        },
    ),
)
[TRACE][pipeline/src/pipeline/deps/glob.rs::148] path_collection_metadata_diff: Different {
    record: PathCollectionMetadataDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                148,
                200,
                77,
                111,
                95,
                49,
                203,
                169,
                52,
                159,
                162,
                100,
                63,
                48,
                217,
                238,
                116,
                87,
                229,
                106,
                58,
                83,
                128,
                153,
                228,
                142,
                163,
                210,
                223,
                23,
                76,
                189,
            ],
        },
    ),
    actual: PathCollectionMetadataDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                60,
                52,
                240,
                69,
                59,
                102,
                143,
                14,
                146,
                83,
                185,
                17,
                170,
                44,
                73,
                163,
                189,
                223,
                232,
                197,
                72,
                7,
                236,
                145,
                32,
                88,
                195,
                46,
                80,
                5,
                66,
                102,
            ],
        },
    ),
}
[TRACE][pipeline/src/pipeline/deps/glob.rs::150] "Different": "Different"
[TRACE][core/src/util/file.rs::215] glob: "dir-*/*"
[TRACE][core/src/util/file.rs::185] glob: "dir-*/*"
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0001/file-0002.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0001/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0001/file-0003.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0001/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0002/file-0001.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0002/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0002/file-0001.bin",
)
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0002/file-0002.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0002/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0002.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0002/file-0002.bin",
)
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0002/file-0003.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0002/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0002/file-0003.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0002/file-0003.bin",
)
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0002/file-0003.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0002/file-0001.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0002/file-0002.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/types/diff.rs::295] record: Some(
    PathCollectionDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                49,
                162,
                30,
                159,
                84,
                130,
                216,
                40,
                10,
                153,
                159,
                205,
                122,
                77,
                103,
                149,
                241,
                191,
                130,
                93,
                210,
                188,
                18,
                152,
                244,
                17,
                202,
                101,
                152,
                51,
                148,
                152,
            ],
        },
    ),
)
[TRACE][core/src/types/diff.rs::296] actual: Some(
    PathCollectionDigest(
        XvcDigest {
            algorithm: Blake3,
            digest: [
                173,
                163,
                185,
                62,
                107,
                202,
                4,
                178,
                48,
                9,
                69,
                215,
                247,
                188,
                114,
                224,
                82,
                134,
                138,
                165,
                186,
                154,
                113,
                201,
                102,
                179,
                107,
                10,
                226,
                152,
                2,
                157,
            ],
        },
    ),
)
[TRACE][pipeline/src/pipeline/mod.rs::776] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::777] &r_next_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::779] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::666] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::776] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::777] &r_next_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::779] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::666] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1599] params: StepStateParams {
    xvc_root: XvcRootInner {
        absolute_path: AbsolutePath(
            "[CWD]",
        ),
        xvc_dir: AbsolutePath(
            "[CWD]/.xvc",
        ),
        store_dir: AbsolutePath(
            "[CWD]/.xvc/store",
        ),
        config: XvcConfig {
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
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "core.guid": String(
                            "bda17b5d5b37c25d",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "a29807b4fb6778c3",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "git.use_git": Boolean(
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
                    map: {
                        "TRYCMD_TESTS": String(
                            "pipeline",
                        ),
                        "TRYCMD_DURATION": Integer(
                            300,
                        ),
                    },
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
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
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
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "TRYCMD_DURATION": XvcConfigValue {
                    source: Environment,
                    value: Integer(
                        300,
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
                "file.list.show_dot_files": XvcConfigValue {
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
                        "a29807b4fb6778c3",
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "TRYCMD_TESTS": XvcConfigValue {
                    source: Environment,
                    value: String(
                        "pipeline",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"bda17b5d5b37c25d/"
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
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 4,
            random: 18006038384920420146,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmp: XvcPathMetadataProvider {
        xvc_root: XvcRootInner {
            absolute_path: AbsolutePath(
                "[CWD]",
            ),
            xvc_dir: AbsolutePath(
                "[CWD]/.xvc",
            ),
            store_dir: AbsolutePath(
                "[CWD]/.xvc/store",
            ),
            config: XvcConfig {
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
                            "git.use_git": Boolean(
                                true,
                            ),
                            "file.carry-in.force": Boolean(
                                false,
                            ),
                            "file.track.force": Boolean(
                                false,
                            ),
                            "pipeline.current_pipeline": String(
                                "default",
                            ),
                            "pipeline.default": String(
                                "default",
                            ),
                            "file.track.no_commit": Boolean(
                                false,
                            ),
                            "pipeline.default_params_file": String(
                                "params.yaml",
                            ),
                            "file.list.sort": String(
                                "name-desc",
                            ),
                            "file.recheck.method": String(
                                "copy",
                            ),
                            "pipeline.process_pool_size": Integer(
                                4,
                            ),
                            "git.command": String(
                                "git",
                            ),
                            "file.list.format": String(
                                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                            ),
                            "file.track.no_parallel": Boolean(
                                false,
                            ),
                            "file.track.text_or_binary": String(
                                "auto",
                            ),
                            "git.auto_commit": Boolean(
                                true,
                            ),
                            "git.auto_stage": Boolean(
                                false,
                            ),
                            "file.list.show_dot_files": Boolean(
                                false,
                            ),
                            "file.list.no_summary": Boolean(
                                false,
                            ),
                            "file.carry-in.no_parallel": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "error",
                            ),
                            "core.guid": String(
                                "bda17b5d5b37c25d",
                            ),
                            "cache.algorithm": String(
                                "blake3",
                            ),
                            "file.list.recursive": Boolean(
                                false,
                            ),
                        },
                    },
                    XvcConfigMap {
                        source: Project,
                        map: {
                            "file.list.no_summary": Boolean(
                                false,
                            ),
                            "file.list.show_dot_files": Boolean(
                                false,
                            ),
                            "file.track.no_parallel": Boolean(
                                false,
                            ),
                            "core.guid": String(
                                "a29807b4fb6778c3",
                            ),
                            "file.track.force": Boolean(
                                false,
                            ),
                            "cache.algorithm": String(
                                "blake3",
                            ),
                            "git.command": String(
                                "git",
                            ),
                            "file.recheck.method": String(
                                "copy",
                            ),
                            "pipeline.default_params_file": String(
                                "params.yaml",
                            ),
                            "pipeline.process_pool_size": Integer(
                                4,
                            ),
                            "file.carry-in.force": Boolean(
                                false,
                            ),
                            "git.auto_commit": Boolean(
                                true,
                            ),
                            "file.track.text_or_binary": String(
                                "auto",
                            ),
                            "file.list.sort": String(
                                "name-desc",
                            ),
                            "file.carry-in.no_parallel": Boolean(
                                false,
                            ),
                            "file.track.no_commit": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "error",
                            ),
                            "pipeline.current_pipeline": String(
                                "default",
                            ),
                            "git.auto_stage": Boolean(
                                false,
                            ),
                            "pipeline.default": String(
                                "default",
                            ),
                            "file.list.recursive": Boolean(
                                false,
                            ),
                            "file.list.format": String(
                                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                            ),
                            "git.use_git": Boolean(
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
                        map: {
                            "TRYCMD_TESTS": String(
                                "pipeline",
                            ),
                            "TRYCMD_DURATION": Integer(
                                300,
                            ),
                        },
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
                    "pipeline.default": XvcConfigValue {
                        source: Project,
                        value: String(
                            "default",
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
                    "pipeline.current_pipeline": XvcConfigValue {
                        source: Project,
                        value: String(
                            "default",
                        ),
                    },
                    "file.track.no_parallel": XvcConfigValue {
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
                    "git.use_git": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            true,
                        ),
                    },
                    "TRYCMD_DURATION": XvcConfigValue {
                        source: Environment,
                        value: Integer(
                            300,
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
                    "file.list.show_dot_files": XvcConfigValue {
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
                            "a29807b4fb6778c3",
                        ),
                    },
                    "core.quiet": XvcConfigValue {
                        source: CommandLine,
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
                    "file.carry-in.no_parallel": XvcConfigValue {
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
                    "cache.algorithm": XvcConfigValue {
                        source: Project,
                        value: String(
                            "blake3",
                        ),
                    },
                    "TRYCMD_TESTS": XvcConfigValue {
                        source: Environment,
                        value: String(
                            "pipeline",
                        ),
                    },
                    "file.recheck.method": XvcConfigValue {
                        source: Project,
                        value: String(
                            "copy",
                        ),
                    },
                    "file.list.format": XvcConfigValue {
                        source: Project,
                        value: String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                    },
                    "file.list.no_summary": XvcConfigValue {
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
                    "file.list.recursive": XvcConfigValue {
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
                    "file.track.no_commit": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                },
                init_params: XvcConfigInitParams {
                    default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"bda17b5d5b37c25d/"
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
            },
            local_config_path: AbsolutePath(
                "[CWD]/.xvc/config.local.toml",
            ),
            project_config_path: AbsolutePath(
                "[CWD]/.xvc/config.toml",
            ),
            entity_generator: XvcEntityGenerator {
                counter: 4,
                random: 18006038384920420146,
                dirty: false,
            },
        },
        path_map: RwLock {
            data: {
                XvcPath(
                    "dir-0002/file-0003.bin",
                ): XvcMetadata {
                    file_type: File,
                    size: Some(
                        2003,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1703621513,
                            tv_nsec: 989767729,
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
                            tv_sec: 1703621513,
                            tv_nsec: 989114189,
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
                            tv_sec: 1703621513,
                            tv_nsec: 989363355,
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
                            tv_sec: 1703621513,
                            tv_nsec: 988701273,
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
                            tv_sec: 1703621513,
                            tv_nsec: 989563646,
                        },
                    ),
                },
            },
            poisoned: false,
            ..
        },
        kill_switch_sender: Sender { .. },
        background_thread: Mutex {
            data: JoinHandle { .. },
            poisoned: false,
            ..
        },
        output_sender: Sender { .. },
        ignore_rules: IgnoreRules {
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
    },
    run_conditions: RunConditions {
        never: false,
        always: false,
        ignore_broken_dep_steps: false,
        ignore_missing_outputs: true,
    },
    pipeline_rundir: XvcPath(
        "",
    ),
    terminate_timeout_processes: true,
    algorithm: Blake3,
    command_process: RwLock {
        data: CommandProcess {
            environment: {},
            step: XvcStep {
                name: "files-changed",
            },
            step_command: XvcStepCommand {
                command: "echo 'Files have changed.'",
            },
            birth: None,
            process: None,
            stdout_sender: Sender { .. },
            stderr_sender: Sender { .. },
            stdout_receiver: Receiver { .. },
            stderr_receiver: Receiver { .. },
        },
        poisoned: false,
        ..
    },
    available_process_slots: RwLock {
        data: 4,
        poisoned: false,
        ..
    },
    process_poll_milliseconds: 10,
    dependency_diffs: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    3,
                    12694358487624719733,
                ): Different {
                    record: Glob(
                        GlobDep {
                            glob: "dir-*/*",
                            xvc_paths_digest: Some(
                                PathCollectionDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            49,
                                            162,
                                            30,
                                            159,
                                            84,
                                            130,
                                            216,
                                            40,
                                            10,
                                            153,
                                            159,
                                            205,
                                            122,
                                            77,
                                            103,
                                            149,
                                            241,
                                            191,
                                            130,
                                            93,
                                            210,
                                            188,
                                            18,
                                            152,
                                            244,
                                            17,
                                            202,
                                            101,
                                            152,
                                            51,
                                            148,
                                            152,
                                        ],
                                    },
                                ),
                            ),
                            xvc_metadata_digest: Some(
                                PathCollectionMetadataDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            148,
                                            200,
                                            77,
                                            111,
                                            95,
                                            49,
                                            203,
                                            169,
                                            52,
                                            159,
                                            162,
                                            100,
                                            63,
                                            48,
                                            217,
                                            238,
                                            116,
                                            87,
                                            229,
                                            106,
                                            58,
                                            83,
                                            128,
                                            153,
                                            228,
                                            142,
                                            163,
                                            210,
                                            223,
                                            23,
                                            76,
                                            189,
                                        ],
                                    },
                                ),
                            ),
                            content_digest: Some(
                                PathCollectionContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            146,
                                            40,
                                            186,
                                            211,
                                            142,
                                            254,
                                            60,
                                            163,
                                            73,
                                            143,
                                            25,
                                            241,
                                            168,
                                            128,
                                            104,
                                            92,
                                            108,
                                            192,
                                            103,
                                            67,
                                            36,
                                            149,
                                            66,
                                            207,
                                            25,
                                            0,
                                            121,
                                            115,
                                            185,
                                            214,
                                            187,
                                            207,
                                        ],
                                    },
                                ),
                            ),
                        },
                    ),
                    actual: Glob(
                        GlobDep {
                            glob: "dir-*/*",
                            xvc_paths_digest: Some(
                                PathCollectionDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            173,
                                            163,
                                            185,
                                            62,
                                            107,
                                            202,
                                            4,
                                            178,
                                            48,
                                            9,
                                            69,
                                            215,
                                            247,
                                            188,
                                            114,
                                            224,
                                            82,
                                            134,
                                            138,
                                            165,
                                            186,
                                            154,
                                            113,
                                            201,
                                            102,
                                            179,
                                            107,
                                            10,
                                            226,
                                            152,
                                            2,
                                            157,
                                        ],
                                    },
                                ),
                            ),
                            xvc_metadata_digest: Some(
                                PathCollectionMetadataDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            60,
                                            52,
                                            240,
                                            69,
                                            59,
                                            102,
                                            143,
                                            14,
                                            146,
                                            83,
                                            185,
                                            17,
                                            170,
                                            44,
                                            73,
                                            163,
                                            189,
                                            223,
                                            232,
                                            197,
                                            72,
                                            7,
                                            236,
                                            145,
                                            32,
                                            88,
                                            195,
                                            46,
                                            80,
                                            5,
                                            66,
                                            102,
                                        ],
                                    },
                                ),
                            ),
                            content_digest: Some(
                                PathCollectionContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            116,
                                            232,
                                            17,
                                            6,
                                            155,
                                            173,
                                            113,
                                            66,
                                            19,
                                            82,
                                            174,
                                            165,
                                            250,
                                            67,
                                            115,
                                            165,
                                            245,
                                            181,
                                            5,
                                            57,
                                            255,
                                            251,
                                            210,
                                            190,
                                            179,
                                            68,
                                            108,
                                            229,
                                            226,
                                            5,
                                            139,
                                            21,
                                        ],
                                    },
                                ),
                            ),
                        },
                    ),
                },
            },
        },
        poisoned: false,
        ..
    },
    output_diffs: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    step_e: XvcEntity(
        2,
        15424582325429536234,
    ),
    step: XvcStep {
        name: "files-changed",
    },
    step_command: XvcStepCommand {
        command: "echo 'Files have changed.'",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    15424582325429536234,
                ): WaitingToRun(
                    FromDiffsHasChanged,
                ),
            },
        },
        poisoned: false,
        ..
    },
    step_timeout: 10000s,
    all_steps: HStore {
        map: {
            XvcEntity(
                2,
                15424582325429536234,
            ): XvcStep {
                name: "files-changed",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    15424582325429536234,
                ): XvcStep {
                    name: "files-changed",
                },
            },
            entity_index: {
                XvcStep {
                    name: "files-changed",
                }: [
                    XvcEntity(
                        2,
                        15424582325429536234,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            15424582325429536234,
                        ),
                        value: XvcStep {
                            name: "files-changed",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            15424582325429536234,
                        ),
                        value: XvcStep {
                            name: "files-changed",
                        },
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        children: XvcStore {
            map: {
                XvcEntity(
                    3,
                    12694358487624719733,
                ): Glob(
                    GlobDep {
                        glob: "dir-*/*",
                        xvc_paths_digest: Some(
                            PathCollectionDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        49,
                                        162,
                                        30,
                                        159,
                                        84,
                                        130,
                                        216,
                                        40,
                                        10,
                                        153,
                                        159,
                                        205,
                                        122,
                                        77,
                                        103,
                                        149,
                                        241,
                                        191,
                                        130,
                                        93,
                                        210,
                                        188,
                                        18,
                                        152,
                                        244,
                                        17,
                                        202,
                                        101,
                                        152,
                                        51,
                                        148,
                                        152,
                                    ],
                                },
                            ),
                        ),
                        xvc_metadata_digest: Some(
                            PathCollectionMetadataDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        148,
                                        200,
                                        77,
                                        111,
                                        95,
                                        49,
                                        203,
                                        169,
                                        52,
                                        159,
                                        162,
                                        100,
                                        63,
                                        48,
                                        217,
                                        238,
                                        116,
                                        87,
                                        229,
                                        106,
                                        58,
                                        83,
                                        128,
                                        153,
                                        228,
                                        142,
                                        163,
                                        210,
                                        223,
                                        23,
                                        76,
                                        189,
                                    ],
                                },
                            ),
                        ),
                        content_digest: Some(
                            PathCollectionContentDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        146,
                                        40,
                                        186,
                                        211,
                                        142,
                                        254,
                                        60,
                                        163,
                                        73,
                                        143,
                                        25,
                                        241,
                                        168,
                                        128,
                                        104,
                                        92,
                                        108,
                                        192,
                                        103,
                                        67,
                                        36,
                                        149,
                                        66,
                                        207,
                                        25,
                                        0,
                                        121,
                                        115,
                                        185,
                                        214,
                                        187,
                                        207,
                                    ],
                                },
                            ),
                        ),
                    },
                ),
            },
            entity_index: {
                Glob(
                    GlobDep {
                        glob: "dir-*/*",
                        xvc_paths_digest: Some(
                            PathCollectionDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        49,
                                        162,
                                        30,
                                        159,
                                        84,
                                        130,
                                        216,
                                        40,
                                        10,
                                        153,
                                        159,
                                        205,
                                        122,
                                        77,
                                        103,
                                        149,
                                        241,
                                        191,
                                        130,
                                        93,
                                        210,
                                        188,
                                        18,
                                        152,
                                        244,
                                        17,
                                        202,
                                        101,
                                        152,
                                        51,
                                        148,
                                        152,
                                    ],
                                },
                            ),
                        ),
                        xvc_metadata_digest: Some(
                            PathCollectionMetadataDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        148,
                                        200,
                                        77,
                                        111,
                                        95,
                                        49,
                                        203,
                                        169,
                                        52,
                                        159,
                                        162,
                                        100,
                                        63,
                                        48,
                                        217,
                                        238,
                                        116,
                                        87,
                                        229,
                                        106,
                                        58,
                                        83,
                                        128,
                                        153,
                                        228,
                                        142,
                                        163,
                                        210,
                                        223,
                                        23,
                                        76,
                                        189,
                                    ],
                                },
                            ),
                        ),
                        content_digest: Some(
                            PathCollectionContentDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        146,
                                        40,
                                        186,
                                        211,
                                        142,
                                        254,
                                        60,
                                        163,
                                        73,
                                        143,
                                        25,
                                        241,
                                        168,
                                        128,
                                        104,
                                        92,
                                        108,
                                        192,
                                        103,
                                        67,
                                        36,
                                        149,
                                        66,
                                        207,
                                        25,
                                        0,
                                        121,
                                        115,
                                        185,
                                        214,
                                        187,
                                        207,
                                    ],
                                },
                            ),
                        ),
                    },
                ): [
                    XvcEntity(
                        3,
                        12694358487624719733,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            12694358487624719733,
                        ),
                        value: Glob(
                            GlobDep {
                                glob: "dir-*/*",
                                xvc_paths_digest: None,
                                xvc_metadata_digest: None,
                                content_digest: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            12694358487624719733,
                        ),
                        value: Glob(
                            GlobDep {
                                glob: "dir-*/*",
                                xvc_paths_digest: Some(
                                    PathCollectionDigest(
                                        XvcDigest {
                                            algorithm: Blake3,
                                            digest: [
                                                49,
                                                162,
                                                30,
                                                159,
                                                84,
                                                130,
                                                216,
                                                40,
                                                10,
                                                153,
                                                159,
                                                205,
                                                122,
                                                77,
                                                103,
                                                149,
                                                241,
                                                191,
                                                130,
                                                93,
                                                210,
                                                188,
                                                18,
                                                152,
                                                244,
                                                17,
                                                202,
                                                101,
                                                152,
                                                51,
                                                148,
                                                152,
                                            ],
                                        },
                                    ),
                                ),
                                xvc_metadata_digest: Some(
                                    PathCollectionMetadataDigest(
                                        XvcDigest {
                                            algorithm: Blake3,
                                            digest: [
                                                148,
                                                200,
                                                77,
                                                111,
                                                95,
                                                49,
                                                203,
                                                169,
                                                52,
                                                159,
                                                162,
                                                100,
                                                63,
                                                48,
                                                217,
                                                238,
                                                116,
                                                87,
                                                229,
                                                106,
                                                58,
                                                83,
                                                128,
                                                153,
                                                228,
                                                142,
                                                163,
                                                210,
                                                223,
                                                23,
                                                76,
                                                189,
                                            ],
                                        },
                                    ),
                                ),
                                content_digest: Some(
                                    PathCollectionContentDigest(
                                        XvcDigest {
                                            algorithm: Blake3,
                                            digest: [
                                                146,
                                                40,
                                                186,
                                                211,
                                                142,
                                                254,
                                                60,
                                                163,
                                                73,
                                                143,
                                                25,
                                                241,
                                                168,
                                                128,
                                                104,
                                                92,
                                                108,
                                                192,
                                                103,
                                                67,
                                                36,
                                                149,
                                                66,
                                                207,
                                                25,
                                                0,
                                                121,
                                                115,
                                                185,
                                                214,
                                                187,
                                                207,
                                            ],
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        child_parents: XvcStore {
            map: {
                XvcEntity(
                    3,
                    12694358487624719733,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        15424582325429536234,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        15424582325429536234,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        3,
                        12694358487624719733,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            12694358487624719733,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                15424582325429536234,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
    },
    step_dependencies: {},
    step_outputs: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::776] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::777] &r_next_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::779] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::666] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/command.rs::96] self.environment: {}
[TRACE][pipeline/src/pipeline/mod.rs::776] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::777] &r_next_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::779] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::666] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1478] params: StepStateParams {
    xvc_root: XvcRootInner {
        absolute_path: AbsolutePath(
            "[CWD]",
        ),
        xvc_dir: AbsolutePath(
            "[CWD]/.xvc",
        ),
        store_dir: AbsolutePath(
            "[CWD]/.xvc/store",
        ),
        config: XvcConfig {
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
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "core.guid": String(
                            "bda17b5d5b37c25d",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "a29807b4fb6778c3",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "git.use_git": Boolean(
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
                    map: {
                        "TRYCMD_TESTS": String(
                            "pipeline",
                        ),
                        "TRYCMD_DURATION": Integer(
                            300,
                        ),
                    },
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
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
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
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "TRYCMD_DURATION": XvcConfigValue {
                    source: Environment,
                    value: Integer(
                        300,
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
                "file.list.show_dot_files": XvcConfigValue {
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
                        "a29807b4fb6778c3",
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "TRYCMD_TESTS": XvcConfigValue {
                    source: Environment,
                    value: String(
                        "pipeline",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"bda17b5d5b37c25d/"
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
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 4,
            random: 18006038384920420146,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmp: XvcPathMetadataProvider {
        xvc_root: XvcRootInner {
            absolute_path: AbsolutePath(
                "[CWD]",
            ),
            xvc_dir: AbsolutePath(
                "[CWD]/.xvc",
            ),
            store_dir: AbsolutePath(
                "[CWD]/.xvc/store",
            ),
            config: XvcConfig {
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
                            "git.use_git": Boolean(
                                true,
                            ),
                            "file.carry-in.force": Boolean(
                                false,
                            ),
                            "file.track.force": Boolean(
                                false,
                            ),
                            "pipeline.current_pipeline": String(
                                "default",
                            ),
                            "pipeline.default": String(
                                "default",
                            ),
                            "file.track.no_commit": Boolean(
                                false,
                            ),
                            "pipeline.default_params_file": String(
                                "params.yaml",
                            ),
                            "file.list.sort": String(
                                "name-desc",
                            ),
                            "file.recheck.method": String(
                                "copy",
                            ),
                            "pipeline.process_pool_size": Integer(
                                4,
                            ),
                            "git.command": String(
                                "git",
                            ),
                            "file.list.format": String(
                                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                            ),
                            "file.track.no_parallel": Boolean(
                                false,
                            ),
                            "file.track.text_or_binary": String(
                                "auto",
                            ),
                            "git.auto_commit": Boolean(
                                true,
                            ),
                            "git.auto_stage": Boolean(
                                false,
                            ),
                            "file.list.show_dot_files": Boolean(
                                false,
                            ),
                            "file.list.no_summary": Boolean(
                                false,
                            ),
                            "file.carry-in.no_parallel": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "error",
                            ),
                            "core.guid": String(
                                "bda17b5d5b37c25d",
                            ),
                            "cache.algorithm": String(
                                "blake3",
                            ),
                            "file.list.recursive": Boolean(
                                false,
                            ),
                        },
                    },
                    XvcConfigMap {
                        source: Project,
                        map: {
                            "file.list.no_summary": Boolean(
                                false,
                            ),
                            "file.list.show_dot_files": Boolean(
                                false,
                            ),
                            "file.track.no_parallel": Boolean(
                                false,
                            ),
                            "core.guid": String(
                                "a29807b4fb6778c3",
                            ),
                            "file.track.force": Boolean(
                                false,
                            ),
                            "cache.algorithm": String(
                                "blake3",
                            ),
                            "git.command": String(
                                "git",
                            ),
                            "file.recheck.method": String(
                                "copy",
                            ),
                            "pipeline.default_params_file": String(
                                "params.yaml",
                            ),
                            "pipeline.process_pool_size": Integer(
                                4,
                            ),
                            "file.carry-in.force": Boolean(
                                false,
                            ),
                            "git.auto_commit": Boolean(
                                true,
                            ),
                            "file.track.text_or_binary": String(
                                "auto",
                            ),
                            "file.list.sort": String(
                                "name-desc",
                            ),
                            "file.carry-in.no_parallel": Boolean(
                                false,
                            ),
                            "file.track.no_commit": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "error",
                            ),
                            "pipeline.current_pipeline": String(
                                "default",
                            ),
                            "git.auto_stage": Boolean(
                                false,
                            ),
                            "pipeline.default": String(
                                "default",
                            ),
                            "file.list.recursive": Boolean(
                                false,
                            ),
                            "file.list.format": String(
                                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                            ),
                            "git.use_git": Boolean(
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
                        map: {
                            "TRYCMD_TESTS": String(
                                "pipeline",
                            ),
                            "TRYCMD_DURATION": Integer(
                                300,
                            ),
                        },
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
                    "pipeline.default": XvcConfigValue {
                        source: Project,
                        value: String(
                            "default",
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
                    "pipeline.current_pipeline": XvcConfigValue {
                        source: Project,
                        value: String(
                            "default",
                        ),
                    },
                    "file.track.no_parallel": XvcConfigValue {
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
                    "git.use_git": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            true,
                        ),
                    },
                    "TRYCMD_DURATION": XvcConfigValue {
                        source: Environment,
                        value: Integer(
                            300,
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
                    "file.list.show_dot_files": XvcConfigValue {
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
                            "a29807b4fb6778c3",
                        ),
                    },
                    "core.quiet": XvcConfigValue {
                        source: CommandLine,
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
                    "file.carry-in.no_parallel": XvcConfigValue {
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
                    "cache.algorithm": XvcConfigValue {
                        source: Project,
                        value: String(
                            "blake3",
                        ),
                    },
                    "TRYCMD_TESTS": XvcConfigValue {
                        source: Environment,
                        value: String(
                            "pipeline",
                        ),
                    },
                    "file.recheck.method": XvcConfigValue {
                        source: Project,
                        value: String(
                            "copy",
                        ),
                    },
                    "file.list.format": XvcConfigValue {
                        source: Project,
                        value: String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                    },
                    "file.list.no_summary": XvcConfigValue {
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
                    "file.list.recursive": XvcConfigValue {
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
                    "file.track.no_commit": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                },
                init_params: XvcConfigInitParams {
                    default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"bda17b5d5b37c25d/"
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
            },
            local_config_path: AbsolutePath(
                "[CWD]/.xvc/config.local.toml",
            ),
            project_config_path: AbsolutePath(
                "[CWD]/.xvc/config.toml",
            ),
            entity_generator: XvcEntityGenerator {
                counter: 4,
                random: 18006038384920420146,
                dirty: false,
            },
        },
        path_map: RwLock {
            data: {
                XvcPath(
                    "dir-0002/file-0003.bin",
                ): XvcMetadata {
                    file_type: File,
                    size: Some(
                        2003,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1703621513,
                            tv_nsec: 989767729,
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
                            tv_sec: 1703621513,
                            tv_nsec: 989114189,
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
                            tv_sec: 1703621513,
                            tv_nsec: 989363355,
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
                            tv_sec: 1703621513,
                            tv_nsec: 988701273,
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
                            tv_sec: 1703621513,
                            tv_nsec: 989563646,
                        },
                    ),
                },
            },
            poisoned: false,
            ..
        },
        kill_switch_sender: Sender { .. },
        background_thread: Mutex {
            data: JoinHandle { .. },
            poisoned: false,
            ..
        },
        output_sender: Sender { .. },
        ignore_rules: IgnoreRules {
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
    },
    run_conditions: RunConditions {
        never: false,
        always: false,
        ignore_broken_dep_steps: false,
        ignore_missing_outputs: true,
    },
    pipeline_rundir: XvcPath(
        "",
    ),
    terminate_timeout_processes: true,
    algorithm: Blake3,
    command_process: RwLock {
        data: CommandProcess {
            environment: {},
            step: XvcStep {
                name: "files-changed",
            },
            step_command: XvcStepCommand {
                command: "echo 'Files have changed.'",
            },
            birth: Some(
                Instant {
                    tv_sec: 1717390,
                    tv_nsec: 760199541,
                },
            ),
            process: Some(
                Popen {
                    stdin: None,
                    stdout: Some(
                        File {
                            fd: 7,
                            read: true,
                            write: false,
                        },
                    ),
                    stderr: Some(
                        File {
                            fd: 9,
                            read: true,
                            write: false,
                        },
                    ),
                    child_state: Running {
                        pid: 62206,
                        ext: (),
                    },
                    detached: true,
                },
            ),
            stdout_sender: Sender { .. },
            stderr_sender: Sender { .. },
            stdout_receiver: Receiver { .. },
            stderr_receiver: Receiver { .. },
        },
        poisoned: false,
        ..
    },
    available_process_slots: RwLock {
        data: 3,
        poisoned: false,
        ..
    },
    process_poll_milliseconds: 10,
    dependency_diffs: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    3,
                    12694358487624719733,
                ): Different {
                    record: Glob(
                        GlobDep {
                            glob: "dir-*/*",
                            xvc_paths_digest: Some(
                                PathCollectionDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            49,
                                            162,
                                            30,
                                            159,
                                            84,
                                            130,
                                            216,
                                            40,
                                            10,
                                            153,
                                            159,
                                            205,
                                            122,
                                            77,
                                            103,
                                            149,
                                            241,
                                            191,
                                            130,
                                            93,
                                            210,
                                            188,
                                            18,
                                            152,
                                            244,
                                            17,
                                            202,
                                            101,
                                            152,
                                            51,
                                            148,
                                            152,
                                        ],
                                    },
                                ),
                            ),
                            xvc_metadata_digest: Some(
                                PathCollectionMetadataDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            148,
                                            200,
                                            77,
                                            111,
                                            95,
                                            49,
                                            203,
                                            169,
                                            52,
                                            159,
                                            162,
                                            100,
                                            63,
                                            48,
                                            217,
                                            238,
                                            116,
                                            87,
                                            229,
                                            106,
                                            58,
                                            83,
                                            128,
                                            153,
                                            228,
                                            142,
                                            163,
                                            210,
                                            223,
                                            23,
                                            76,
                                            189,
                                        ],
                                    },
                                ),
                            ),
                            content_digest: Some(
                                PathCollectionContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            146,
                                            40,
                                            186,
                                            211,
                                            142,
                                            254,
                                            60,
                                            163,
                                            73,
                                            143,
                                            25,
                                            241,
                                            168,
                                            128,
                                            104,
                                            92,
                                            108,
                                            192,
                                            103,
                                            67,
                                            36,
                                            149,
                                            66,
                                            207,
                                            25,
                                            0,
                                            121,
                                            115,
                                            185,
                                            214,
                                            187,
                                            207,
                                        ],
                                    },
                                ),
                            ),
                        },
                    ),
                    actual: Glob(
                        GlobDep {
                            glob: "dir-*/*",
                            xvc_paths_digest: Some(
                                PathCollectionDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            173,
                                            163,
                                            185,
                                            62,
                                            107,
                                            202,
                                            4,
                                            178,
                                            48,
                                            9,
                                            69,
                                            215,
                                            247,
                                            188,
                                            114,
                                            224,
                                            82,
                                            134,
                                            138,
                                            165,
                                            186,
                                            154,
                                            113,
                                            201,
                                            102,
                                            179,
                                            107,
                                            10,
                                            226,
                                            152,
                                            2,
                                            157,
                                        ],
                                    },
                                ),
                            ),
                            xvc_metadata_digest: Some(
                                PathCollectionMetadataDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            60,
                                            52,
                                            240,
                                            69,
                                            59,
                                            102,
                                            143,
                                            14,
                                            146,
                                            83,
                                            185,
                                            17,
                                            170,
                                            44,
                                            73,
                                            163,
                                            189,
                                            223,
                                            232,
                                            197,
                                            72,
                                            7,
                                            236,
                                            145,
                                            32,
                                            88,
                                            195,
                                            46,
                                            80,
                                            5,
                                            66,
                                            102,
                                        ],
                                    },
                                ),
                            ),
                            content_digest: Some(
                                PathCollectionContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            116,
                                            232,
                                            17,
                                            6,
                                            155,
                                            173,
                                            113,
                                            66,
                                            19,
                                            82,
                                            174,
                                            165,
                                            250,
                                            67,
                                            115,
                                            165,
                                            245,
                                            181,
                                            5,
                                            57,
                                            255,
                                            251,
                                            210,
                                            190,
                                            179,
                                            68,
                                            108,
                                            229,
                                            226,
                                            5,
                                            139,
                                            21,
                                        ],
                                    },
                                ),
                            ),
                        },
                    ),
                },
            },
        },
        poisoned: false,
        ..
    },
    output_diffs: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    step_e: XvcEntity(
        2,
        15424582325429536234,
    ),
    step: XvcStep {
        name: "files-changed",
    },
    step_command: XvcStepCommand {
        command: "echo 'Files have changed.'",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    15424582325429536234,
                ): Running(
                    FromWaitProcess,
                ),
            },
        },
        poisoned: false,
        ..
    },
    step_timeout: 10000s,
    all_steps: HStore {
        map: {
            XvcEntity(
                2,
                15424582325429536234,
            ): XvcStep {
                name: "files-changed",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    15424582325429536234,
                ): XvcStep {
                    name: "files-changed",
                },
            },
            entity_index: {
                XvcStep {
                    name: "files-changed",
                }: [
                    XvcEntity(
                        2,
                        15424582325429536234,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            15424582325429536234,
                        ),
                        value: XvcStep {
                            name: "files-changed",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            15424582325429536234,
                        ),
                        value: XvcStep {
                            name: "files-changed",
                        },
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        children: XvcStore {
            map: {
                XvcEntity(
                    3,
                    12694358487624719733,
                ): Glob(
                    GlobDep {
                        glob: "dir-*/*",
                        xvc_paths_digest: Some(
                            PathCollectionDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        49,
                                        162,
                                        30,
                                        159,
                                        84,
                                        130,
                                        216,
                                        40,
                                        10,
                                        153,
                                        159,
                                        205,
                                        122,
                                        77,
                                        103,
                                        149,
                                        241,
                                        191,
                                        130,
                                        93,
                                        210,
                                        188,
                                        18,
                                        152,
                                        244,
                                        17,
                                        202,
                                        101,
                                        152,
                                        51,
                                        148,
                                        152,
                                    ],
                                },
                            ),
                        ),
                        xvc_metadata_digest: Some(
                            PathCollectionMetadataDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        148,
                                        200,
                                        77,
                                        111,
                                        95,
                                        49,
                                        203,
                                        169,
                                        52,
                                        159,
                                        162,
                                        100,
                                        63,
                                        48,
                                        217,
                                        238,
                                        116,
                                        87,
                                        229,
                                        106,
                                        58,
                                        83,
                                        128,
                                        153,
                                        228,
                                        142,
                                        163,
                                        210,
                                        223,
                                        23,
                                        76,
                                        189,
                                    ],
                                },
                            ),
                        ),
                        content_digest: Some(
                            PathCollectionContentDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        146,
                                        40,
                                        186,
                                        211,
                                        142,
                                        254,
                                        60,
                                        163,
                                        73,
                                        143,
                                        25,
                                        241,
                                        168,
                                        128,
                                        104,
                                        92,
                                        108,
                                        192,
                                        103,
                                        67,
                                        36,
                                        149,
                                        66,
                                        207,
                                        25,
                                        0,
                                        121,
                                        115,
                                        185,
                                        214,
                                        187,
                                        207,
                                    ],
                                },
                            ),
                        ),
                    },
                ),
            },
            entity_index: {
                Glob(
                    GlobDep {
                        glob: "dir-*/*",
                        xvc_paths_digest: Some(
                            PathCollectionDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        49,
                                        162,
                                        30,
                                        159,
                                        84,
                                        130,
                                        216,
                                        40,
                                        10,
                                        153,
                                        159,
                                        205,
                                        122,
                                        77,
                                        103,
                                        149,
                                        241,
                                        191,
                                        130,
                                        93,
                                        210,
                                        188,
                                        18,
                                        152,
                                        244,
                                        17,
                                        202,
                                        101,
                                        152,
                                        51,
                                        148,
                                        152,
                                    ],
                                },
                            ),
                        ),
                        xvc_metadata_digest: Some(
                            PathCollectionMetadataDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        148,
                                        200,
                                        77,
                                        111,
                                        95,
                                        49,
                                        203,
                                        169,
                                        52,
                                        159,
                                        162,
                                        100,
                                        63,
                                        48,
                                        217,
                                        238,
                                        116,
                                        87,
                                        229,
                                        106,
                                        58,
                                        83,
                                        128,
                                        153,
                                        228,
                                        142,
                                        163,
                                        210,
                                        223,
                                        23,
                                        76,
                                        189,
                                    ],
                                },
                            ),
                        ),
                        content_digest: Some(
                            PathCollectionContentDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        146,
                                        40,
                                        186,
                                        211,
                                        142,
                                        254,
                                        60,
                                        163,
                                        73,
                                        143,
                                        25,
                                        241,
                                        168,
                                        128,
                                        104,
                                        92,
                                        108,
                                        192,
                                        103,
                                        67,
                                        36,
                                        149,
                                        66,
                                        207,
                                        25,
                                        0,
                                        121,
                                        115,
                                        185,
                                        214,
                                        187,
                                        207,
                                    ],
                                },
                            ),
                        ),
                    },
                ): [
                    XvcEntity(
                        3,
                        12694358487624719733,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            12694358487624719733,
                        ),
                        value: Glob(
                            GlobDep {
                                glob: "dir-*/*",
                                xvc_paths_digest: None,
                                xvc_metadata_digest: None,
                                content_digest: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            12694358487624719733,
                        ),
                        value: Glob(
                            GlobDep {
                                glob: "dir-*/*",
                                xvc_paths_digest: Some(
                                    PathCollectionDigest(
                                        XvcDigest {
                                            algorithm: Blake3,
                                            digest: [
                                                49,
                                                162,
                                                30,
                                                159,
                                                84,
                                                130,
                                                216,
                                                40,
                                                10,
                                                153,
                                                159,
                                                205,
                                                122,
                                                77,
                                                103,
                                                149,
                                                241,
                                                191,
                                                130,
                                                93,
                                                210,
                                                188,
                                                18,
                                                152,
                                                244,
                                                17,
                                                202,
                                                101,
                                                152,
                                                51,
                                                148,
                                                152,
                                            ],
                                        },
                                    ),
                                ),
                                xvc_metadata_digest: Some(
                                    PathCollectionMetadataDigest(
                                        XvcDigest {
                                            algorithm: Blake3,
                                            digest: [
                                                148,
                                                200,
                                                77,
                                                111,
                                                95,
                                                49,
                                                203,
                                                169,
                                                52,
                                                159,
                                                162,
                                                100,
                                                63,
                                                48,
                                                217,
                                                238,
                                                116,
                                                87,
                                                229,
                                                106,
                                                58,
                                                83,
                                                128,
                                                153,
                                                228,
                                                142,
                                                163,
                                                210,
                                                223,
                                                23,
                                                76,
                                                189,
                                            ],
                                        },
                                    ),
                                ),
                                content_digest: Some(
                                    PathCollectionContentDigest(
                                        XvcDigest {
                                            algorithm: Blake3,
                                            digest: [
                                                146,
                                                40,
                                                186,
                                                211,
                                                142,
                                                254,
                                                60,
                                                163,
                                                73,
                                                143,
                                                25,
                                                241,
                                                168,
                                                128,
                                                104,
                                                92,
                                                108,
                                                192,
                                                103,
                                                67,
                                                36,
                                                149,
                                                66,
                                                207,
                                                25,
                                                0,
                                                121,
                                                115,
                                                185,
                                                214,
                                                187,
                                                207,
                                            ],
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        child_parents: XvcStore {
            map: {
                XvcEntity(
                    3,
                    12694358487624719733,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        15424582325429536234,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        15424582325429536234,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        3,
                        12694358487624719733,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            12694358487624719733,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                15424582325429536234,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
    },
    step_dependencies: {},
    step_outputs: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1521] &process: Popen {
    stdin: None,
    stdout: Some(
        File {
            fd: 7,
            read: true,
            write: false,
        },
    ),
    stderr: Some(
        File {
            fd: 9,
            read: true,
            write: false,
        },
    ),
    child_state: Running {
        pid: 62206,
        ext: (),
    },
    detached: true,
}
[TRACE][pipeline/src/pipeline/mod.rs::1526] process: Popen {
    stdin: None,
    stdout: Some(
        File {
            fd: 7,
            read: true,
            write: false,
        },
    ),
    stderr: Some(
        File {
            fd: 9,
            read: true,
            write: false,
        },
    ),
    child_state: Running {
        pid: 62206,
        ext: (),
    },
    detached: true,
}
[OUT] [files-changed] Files have changed.
 
[TRACE][pipeline/src/pipeline/mod.rs::597] s: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1521] &process: Popen {
    stdin: None,
    stdout: Some(
        File {
            fd: 7,
            read: true,
            write: false,
        },
    ),
    stderr: Some(
        File {
            fd: 9,
            read: true,
            write: false,
        },
    ),
    child_state: Running {
        pid: 62206,
        ext: (),
    },
    detached: true,
}
[TRACE][pipeline/src/pipeline/mod.rs::1573] return_state: Some(
    DoneByRunning(
        FromProcessCompletedSuccessfully,
    ),
)
[DONE] files-changed (echo 'Files have changed.')
[TRACE][pipeline/src/pipeline/mod.rs::1579] params: StepStateParams {
    xvc_root: XvcRootInner {
        absolute_path: AbsolutePath(
            "[CWD]",
        ),
        xvc_dir: AbsolutePath(
            "[CWD]/.xvc",
        ),
        store_dir: AbsolutePath(
            "[CWD]/.xvc/store",
        ),
        config: XvcConfig {
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
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "core.guid": String(
                            "bda17b5d5b37c25d",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "a29807b4fb6778c3",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "git.use_git": Boolean(
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
                    map: {
                        "TRYCMD_TESTS": String(
                            "pipeline",
                        ),
                        "TRYCMD_DURATION": Integer(
                            300,
                        ),
                    },
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
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
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
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
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
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "TRYCMD_DURATION": XvcConfigValue {
                    source: Environment,
                    value: Integer(
                        300,
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
                "file.list.show_dot_files": XvcConfigValue {
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
                        "a29807b4fb6778c3",
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "TRYCMD_TESTS": XvcConfigValue {
                    source: Environment,
                    value: String(
                        "pipeline",
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
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
                "file.list.recursive": XvcConfigValue {
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
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"bda17b5d5b37c25d/"
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
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 4,
            random: 18006038384920420146,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmp: XvcPathMetadataProvider {
        xvc_root: XvcRootInner {
            absolute_path: AbsolutePath(
                "[CWD]",
            ),
            xvc_dir: AbsolutePath(
                "[CWD]/.xvc",
            ),
            store_dir: AbsolutePath(
                "[CWD]/.xvc/store",
            ),
            config: XvcConfig {
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
                            "git.use_git": Boolean(
                                true,
                            ),
                            "file.carry-in.force": Boolean(
                                false,
                            ),
                            "file.track.force": Boolean(
                                false,
                            ),
                            "pipeline.current_pipeline": String(
                                "default",
                            ),
                            "pipeline.default": String(
                                "default",
                            ),
                            "file.track.no_commit": Boolean(
                                false,
                            ),
                            "pipeline.default_params_file": String(
                                "params.yaml",
                            ),
                            "file.list.sort": String(
                                "name-desc",
                            ),
                            "file.recheck.method": String(
                                "copy",
                            ),
                            "pipeline.process_pool_size": Integer(
                                4,
                            ),
                            "git.command": String(
                                "git",
                            ),
                            "file.list.format": String(
                                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                            ),
                            "file.track.no_parallel": Boolean(
                                false,
                            ),
                            "file.track.text_or_binary": String(
                                "auto",
                            ),
                            "git.auto_commit": Boolean(
                                true,
                            ),
                            "git.auto_stage": Boolean(
                                false,
                            ),
                            "file.list.show_dot_files": Boolean(
                                false,
                            ),
                            "file.list.no_summary": Boolean(
                                false,
                            ),
                            "file.carry-in.no_parallel": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "error",
                            ),
                            "core.guid": String(
                                "bda17b5d5b37c25d",
                            ),
                            "cache.algorithm": String(
                                "blake3",
                            ),
                            "file.list.recursive": Boolean(
                                false,
                            ),
                        },
                    },
                    XvcConfigMap {
                        source: Project,
                        map: {
                            "file.list.no_summary": Boolean(
                                false,
                            ),
                            "file.list.show_dot_files": Boolean(
                                false,
                            ),
                            "file.track.no_parallel": Boolean(
                                false,
                            ),
                            "core.guid": String(
                                "a29807b4fb6778c3",
                            ),
                            "file.track.force": Boolean(
                                false,
                            ),
                            "cache.algorithm": String(
                                "blake3",
                            ),
                            "git.command": String(
                                "git",
                            ),
                            "file.recheck.method": String(
                                "copy",
                            ),
                            "pipeline.default_params_file": String(
                                "params.yaml",
                            ),
                            "pipeline.process_pool_size": Integer(
                                4,
                            ),
                            "file.carry-in.force": Boolean(
                                false,
                            ),
                            "git.auto_commit": Boolean(
                                true,
                            ),
                            "file.track.text_or_binary": String(
                                "auto",
                            ),
                            "file.list.sort": String(
                                "name-desc",
                            ),
                            "file.carry-in.no_parallel": Boolean(
                                false,
                            ),
                            "file.track.no_commit": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "error",
                            ),
                            "pipeline.current_pipeline": String(
                                "default",
                            ),
                            "git.auto_stage": Boolean(
                                false,
                            ),
                            "pipeline.default": String(
                                "default",
                            ),
                            "file.list.recursive": Boolean(
                                false,
                            ),
                            "file.list.format": String(
                                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                            ),
                            "git.use_git": Boolean(
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
                        map: {
                            "TRYCMD_TESTS": String(
                                "pipeline",
                            ),
                            "TRYCMD_DURATION": Integer(
                                300,
                            ),
                        },
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
                    "pipeline.default": XvcConfigValue {
                        source: Project,
                        value: String(
                            "default",
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
                    "pipeline.current_pipeline": XvcConfigValue {
                        source: Project,
                        value: String(
                            "default",
                        ),
                    },
                    "file.track.no_parallel": XvcConfigValue {
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
                    "git.use_git": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            true,
                        ),
                    },
                    "TRYCMD_DURATION": XvcConfigValue {
                        source: Environment,
                        value: Integer(
                            300,
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
                    "file.list.show_dot_files": XvcConfigValue {
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
                            "a29807b4fb6778c3",
                        ),
                    },
                    "core.quiet": XvcConfigValue {
                        source: CommandLine,
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
                    "file.carry-in.no_parallel": XvcConfigValue {
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
                    "cache.algorithm": XvcConfigValue {
                        source: Project,
                        value: String(
                            "blake3",
                        ),
                    },
                    "TRYCMD_TESTS": XvcConfigValue {
                        source: Environment,
                        value: String(
                            "pipeline",
                        ),
                    },
                    "file.recheck.method": XvcConfigValue {
                        source: Project,
                        value: String(
                            "copy",
                        ),
                    },
                    "file.list.format": XvcConfigValue {
                        source: Project,
                        value: String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                    },
                    "file.list.no_summary": XvcConfigValue {
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
                    "file.list.recursive": XvcConfigValue {
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
                    "file.track.no_commit": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                },
                init_params: XvcConfigInitParams {
                    default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"bda17b5d5b37c25d/"
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
            },
            local_config_path: AbsolutePath(
                "[CWD]/.xvc/config.local.toml",
            ),
            project_config_path: AbsolutePath(
                "[CWD]/.xvc/config.toml",
            ),
            entity_generator: XvcEntityGenerator {
                counter: 4,
                random: 18006038384920420146,
                dirty: false,
            },
        },
        path_map: RwLock {
            data: {
                XvcPath(
                    "dir-0002/file-0003.bin",
                ): XvcMetadata {
                    file_type: File,
                    size: Some(
                        2003,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1703621513,
                            tv_nsec: 989767729,
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
                            tv_sec: 1703621513,
                            tv_nsec: 989114189,
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
                            tv_sec: 1703621513,
                            tv_nsec: 989363355,
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
                            tv_sec: 1703621513,
                            tv_nsec: 988701273,
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
                            tv_sec: 1703621513,
                            tv_nsec: 989563646,
                        },
                    ),
                },
            },
            poisoned: false,
            ..
        },
        kill_switch_sender: Sender { .. },
        background_thread: Mutex {
            data: JoinHandle { .. },
            poisoned: false,
            ..
        },
        output_sender: Sender { .. },
        ignore_rules: IgnoreRules {
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
    },
    run_conditions: RunConditions {
        never: false,
        always: false,
        ignore_broken_dep_steps: false,
        ignore_missing_outputs: true,
    },
    pipeline_rundir: XvcPath(
        "",
    ),
    terminate_timeout_processes: true,
    algorithm: Blake3,
    command_process: RwLock {
        data: CommandProcess {
            environment: {},
            step: XvcStep {
                name: "files-changed",
            },
            step_command: XvcStepCommand {
                command: "echo 'Files have changed.'",
            },
            birth: Some(
                Instant {
                    tv_sec: 1717390,
                    tv_nsec: 760199541,
                },
            ),
            process: Some(
                Popen {
                    stdin: None,
                    stdout: Some(
                        File {
                            fd: 7,
                            read: true,
                            write: false,
                        },
                    ),
                    stderr: Some(
                        File {
                            fd: 9,
                            read: true,
                            write: false,
                        },
                    ),
                    child_state: Finished(
                        Exited(
                            0,
                        ),
                    ),
                    detached: true,
                },
            ),
            stdout_sender: Sender { .. },
            stderr_sender: Sender { .. },
            stdout_receiver: Receiver { .. },
            stderr_receiver: Receiver { .. },
        },
        poisoned: false,
        ..
    },
    available_process_slots: RwLock {
        data: <locked>,
        poisoned: false,
        ..
    },
    process_poll_milliseconds: 10,
    dependency_diffs: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    3,
                    12694358487624719733,
                ): Different {
                    record: Glob(
                        GlobDep {
                            glob: "dir-*/*",
                            xvc_paths_digest: Some(
                                PathCollectionDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            49,
                                            162,
                                            30,
                                            159,
                                            84,
                                            130,
                                            216,
                                            40,
                                            10,
                                            153,
                                            159,
                                            205,
                                            122,
                                            77,
                                            103,
                                            149,
                                            241,
                                            191,
                                            130,
                                            93,
                                            210,
                                            188,
                                            18,
                                            152,
                                            244,
                                            17,
                                            202,
                                            101,
                                            152,
                                            51,
                                            148,
                                            152,
                                        ],
                                    },
                                ),
                            ),
                            xvc_metadata_digest: Some(
                                PathCollectionMetadataDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            148,
                                            200,
                                            77,
                                            111,
                                            95,
                                            49,
                                            203,
                                            169,
                                            52,
                                            159,
                                            162,
                                            100,
                                            63,
                                            48,
                                            217,
                                            238,
                                            116,
                                            87,
                                            229,
                                            106,
                                            58,
                                            83,
                                            128,
                                            153,
                                            228,
                                            142,
                                            163,
                                            210,
                                            223,
                                            23,
                                            76,
                                            189,
                                        ],
                                    },
                                ),
                            ),
                            content_digest: Some(
                                PathCollectionContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            146,
                                            40,
                                            186,
                                            211,
                                            142,
                                            254,
                                            60,
                                            163,
                                            73,
                                            143,
                                            25,
                                            241,
                                            168,
                                            128,
                                            104,
                                            92,
                                            108,
                                            192,
                                            103,
                                            67,
                                            36,
                                            149,
                                            66,
                                            207,
                                            25,
                                            0,
                                            121,
                                            115,
                                            185,
                                            214,
                                            187,
                                            207,
                                        ],
                                    },
                                ),
                            ),
                        },
                    ),
                    actual: Glob(
                        GlobDep {
                            glob: "dir-*/*",
                            xvc_paths_digest: Some(
                                PathCollectionDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            173,
                                            163,
                                            185,
                                            62,
                                            107,
                                            202,
                                            4,
                                            178,
                                            48,
                                            9,
                                            69,
                                            215,
                                            247,
                                            188,
                                            114,
                                            224,
                                            82,
                                            134,
                                            138,
                                            165,
                                            186,
                                            154,
                                            113,
                                            201,
                                            102,
                                            179,
                                            107,
                                            10,
                                            226,
                                            152,
                                            2,
                                            157,
                                        ],
                                    },
                                ),
                            ),
                            xvc_metadata_digest: Some(
                                PathCollectionMetadataDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            60,
                                            52,
                                            240,
                                            69,
                                            59,
                                            102,
                                            143,
                                            14,
                                            146,
                                            83,
                                            185,
                                            17,
                                            170,
                                            44,
                                            73,
                                            163,
                                            189,
                                            223,
                                            232,
                                            197,
                                            72,
                                            7,
                                            236,
                                            145,
                                            32,
                                            88,
                                            195,
                                            46,
                                            80,
                                            5,
                                            66,
                                            102,
                                        ],
                                    },
                                ),
                            ),
                            content_digest: Some(
                                PathCollectionContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            116,
                                            232,
                                            17,
                                            6,
                                            155,
                                            173,
                                            113,
                                            66,
                                            19,
                                            82,
                                            174,
                                            165,
                                            250,
                                            67,
                                            115,
                                            165,
                                            245,
                                            181,
                                            5,
                                            57,
                                            255,
                                            251,
                                            210,
                                            190,
                                            179,
                                            68,
                                            108,
                                            229,
                                            226,
                                            5,
                                            139,
                                            21,
                                        ],
                                    },
                                ),
                            ),
                        },
                    ),
                },
            },
        },
        poisoned: false,
        ..
    },
    output_diffs: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    step_e: XvcEntity(
        2,
        15424582325429536234,
    ),
    step: XvcStep {
        name: "files-changed",
    },
    step_command: XvcStepCommand {
        command: "echo 'Files have changed.'",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    15424582325429536234,
                ): Running(
                    FromWaitProcess,
                ),
            },
        },
        poisoned: false,
        ..
    },
    step_timeout: 10000s,
    all_steps: HStore {
        map: {
            XvcEntity(
                2,
                15424582325429536234,
            ): XvcStep {
                name: "files-changed",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    15424582325429536234,
                ): XvcStep {
                    name: "files-changed",
                },
            },
            entity_index: {
                XvcStep {
                    name: "files-changed",
                }: [
                    XvcEntity(
                        2,
                        15424582325429536234,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            15424582325429536234,
                        ),
                        value: XvcStep {
                            name: "files-changed",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            15424582325429536234,
                        ),
                        value: XvcStep {
                            name: "files-changed",
                        },
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        children: XvcStore {
            map: {
                XvcEntity(
                    3,
                    12694358487624719733,
                ): Glob(
                    GlobDep {
                        glob: "dir-*/*",
                        xvc_paths_digest: Some(
                            PathCollectionDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        49,
                                        162,
                                        30,
                                        159,
                                        84,
                                        130,
                                        216,
                                        40,
                                        10,
                                        153,
                                        159,
                                        205,
                                        122,
                                        77,
                                        103,
                                        149,
                                        241,
                                        191,
                                        130,
                                        93,
                                        210,
                                        188,
                                        18,
                                        152,
                                        244,
                                        17,
                                        202,
                                        101,
                                        152,
                                        51,
                                        148,
                                        152,
                                    ],
                                },
                            ),
                        ),
                        xvc_metadata_digest: Some(
                            PathCollectionMetadataDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        148,
                                        200,
                                        77,
                                        111,
                                        95,
                                        49,
                                        203,
                                        169,
                                        52,
                                        159,
                                        162,
                                        100,
                                        63,
                                        48,
                                        217,
                                        238,
                                        116,
                                        87,
                                        229,
                                        106,
                                        58,
                                        83,
                                        128,
                                        153,
                                        228,
                                        142,
                                        163,
                                        210,
                                        223,
                                        23,
                                        76,
                                        189,
                                    ],
                                },
                            ),
                        ),
                        content_digest: Some(
                            PathCollectionContentDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        146,
                                        40,
                                        186,
                                        211,
                                        142,
                                        254,
                                        60,
                                        163,
                                        73,
                                        143,
                                        25,
                                        241,
                                        168,
                                        128,
                                        104,
                                        92,
                                        108,
                                        192,
                                        103,
                                        67,
                                        36,
                                        149,
                                        66,
                                        207,
                                        25,
                                        0,
                                        121,
                                        115,
                                        185,
                                        214,
                                        187,
                                        207,
                                    ],
                                },
                            ),
                        ),
                    },
                ),
            },
            entity_index: {
                Glob(
                    GlobDep {
                        glob: "dir-*/*",
                        xvc_paths_digest: Some(
                            PathCollectionDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        49,
                                        162,
                                        30,
                                        159,
                                        84,
                                        130,
                                        216,
                                        40,
                                        10,
                                        153,
                                        159,
                                        205,
                                        122,
                                        77,
                                        103,
                                        149,
                                        241,
                                        191,
                                        130,
                                        93,
                                        210,
                                        188,
                                        18,
                                        152,
                                        244,
                                        17,
                                        202,
                                        101,
                                        152,
                                        51,
                                        148,
                                        152,
                                    ],
                                },
                            ),
                        ),
                        xvc_metadata_digest: Some(
                            PathCollectionMetadataDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        148,
                                        200,
                                        77,
                                        111,
                                        95,
                                        49,
                                        203,
                                        169,
                                        52,
                                        159,
                                        162,
                                        100,
                                        63,
                                        48,
                                        217,
                                        238,
                                        116,
                                        87,
                                        229,
                                        106,
                                        58,
                                        83,
                                        128,
                                        153,
                                        228,
                                        142,
                                        163,
                                        210,
                                        223,
                                        23,
                                        76,
                                        189,
                                    ],
                                },
                            ),
                        ),
                        content_digest: Some(
                            PathCollectionContentDigest(
                                XvcDigest {
                                    algorithm: Blake3,
                                    digest: [
                                        146,
                                        40,
                                        186,
                                        211,
                                        142,
                                        254,
                                        60,
                                        163,
                                        73,
                                        143,
                                        25,
                                        241,
                                        168,
                                        128,
                                        104,
                                        92,
                                        108,
                                        192,
                                        103,
                                        67,
                                        36,
                                        149,
                                        66,
                                        207,
                                        25,
                                        0,
                                        121,
                                        115,
                                        185,
                                        214,
                                        187,
                                        207,
                                    ],
                                },
                            ),
                        ),
                    },
                ): [
                    XvcEntity(
                        3,
                        12694358487624719733,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            12694358487624719733,
                        ),
                        value: Glob(
                            GlobDep {
                                glob: "dir-*/*",
                                xvc_paths_digest: None,
                                xvc_metadata_digest: None,
                                content_digest: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            3,
                            12694358487624719733,
                        ),
                        value: Glob(
                            GlobDep {
                                glob: "dir-*/*",
                                xvc_paths_digest: Some(
                                    PathCollectionDigest(
                                        XvcDigest {
                                            algorithm: Blake3,
                                            digest: [
                                                49,
                                                162,
                                                30,
                                                159,
                                                84,
                                                130,
                                                216,
                                                40,
                                                10,
                                                153,
                                                159,
                                                205,
                                                122,
                                                77,
                                                103,
                                                149,
                                                241,
                                                191,
                                                130,
                                                93,
                                                210,
                                                188,
                                                18,
                                                152,
                                                244,
                                                17,
                                                202,
                                                101,
                                                152,
                                                51,
                                                148,
                                                152,
                                            ],
                                        },
                                    ),
                                ),
                                xvc_metadata_digest: Some(
                                    PathCollectionMetadataDigest(
                                        XvcDigest {
                                            algorithm: Blake3,
                                            digest: [
                                                148,
                                                200,
                                                77,
                                                111,
                                                95,
                                                49,
                                                203,
                                                169,
                                                52,
                                                159,
                                                162,
                                                100,
                                                63,
                                                48,
                                                217,
                                                238,
                                                116,
                                                87,
                                                229,
                                                106,
                                                58,
                                                83,
                                                128,
                                                153,
                                                228,
                                                142,
                                                163,
                                                210,
                                                223,
                                                23,
                                                76,
                                                189,
                                            ],
                                        },
                                    ),
                                ),
                                content_digest: Some(
                                    PathCollectionContentDigest(
                                        XvcDigest {
                                            algorithm: Blake3,
                                            digest: [
                                                146,
                                                40,
                                                186,
                                                211,
                                                142,
                                                254,
                                                60,
                                                163,
                                                73,
                                                143,
                                                25,
                                                241,
                                                168,
                                                128,
                                                104,
                                                92,
                                                108,
                                                192,
                                                103,
                                                67,
                                                36,
                                                149,
                                                66,
                                                207,
                                                25,
                                                0,
                                                121,
                                                115,
                                                185,
                                                214,
                                                187,
                                                207,
                                            ],
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        child_parents: XvcStore {
            map: {
                XvcEntity(
                    3,
                    12694358487624719733,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        15424582325429536234,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        15424582325429536234,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        3,
                        12694358487624719733,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            12694358487624719733,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                15424582325429536234,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
    },
    step_dependencies: {},
    step_outputs: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::776] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::777] &r_next_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::779] &step_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::666] &step_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::523] "Before state updater": "Before state updater"
[TRACE][pipeline/src/pipeline/mod.rs::597] s: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::533] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                15424582325429536234,
            ): DoneByRunning(
                FromProcessCompletedSuccessfully,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::540] done_successfully: Ok(
    true,
)
[TRACE][lib/src/cli/mod.rs::381] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::384] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc --debug pipeline run"
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
[TRACE][lib/src/cli/mod.rs::584] git_add_output: "add '.xvc/store/xvc-dependency-store/1703621514426117.json'
"
[TRACE][lib/src/cli/mod.rs::436] args: [
    "-C",
    "[CWD]",
    "commit",
    "-m",
    "Xvc auto-commit after /'/Users/iex/github.com/iesahin/xvc/target/debug/xvc --debug pipeline run/'",
]

```
