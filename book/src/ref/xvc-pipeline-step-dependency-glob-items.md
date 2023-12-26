### Glob Items Dependency

A step can depend on multiple files specified with globs. When any of the files change, or a new file is added or
removed from the files specified by glob, the step is invalidated.

Unline glob dependency, glob items dependency keeps track of the individual files that belong to a glob. If your
command run with the list of files from a glob and you want to track added and removed files, use this. Otherwise if
your command for all the files in a glob and don't need to track which files have changed, use the glob dependency.

This one injects `${XVC_GLOB_ADDED_ITEMS}`, `${XVC_GLOB_REMOVED_ITEMS}`, `${XVC_GLOB_CHANGED_ITEMS}` and `${XVC_GLOB_ALL_ITEMS}` to the command
environment.

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

Add a step to list the added files.

```console
$ xvc pipeline step new --step-name files-changed --command 'echo "### Added Files:\n${XVC_GLOB_ADDED_ITEMS}\n### Removed Files:\n${XVC_GLOB_REMOVED_ITEMS}\n### Changed Files:\n${XVC_GLOB_CHANGED_ITEMS}"'

$ xvc pipeline step dependency --step-name files-changed --glob-items 'dir-*/*'

```

The step is invalidated when a file described by the glob is added, removed or changed.

```console
$ xvc pipeline run
[OUT] [files-changed] ### Added Files:
dir-0001/file-0001.bin
dir-0001/file-0002.bin
dir-0001/file-0003.bin
dir-0002/file-0001.bin
dir-0002/file-0002.bin
dir-0002/file-0003.bin
### Removed Files:

### Changed Files:

 
[DONE] files-changed (echo "### Added Files:/n${XVC_GLOB_ADDED_ITEMS}/n### Removed Files:/n${XVC_GLOB_REMOVED_ITEMS}/n### Changed Files:/n${XVC_GLOB_CHANGED_ITEMS}")

$ xvc --debug pipeline run
[DEBUG][logging/src/lib.rs::237] Terminal logger enabled with level: Error
[DEBUG][logging/src/lib.rs::240] File logger enabled with level: Trace to "/var/folders/tk/3vn311ps4kqdhgykj3jg_p8r0000gn/T//xvc.log"
[TRACE][core/src/types/xvcroot.rs::247] "."
[DEBUG][core/src/types/xvcroot.rs::253] XVC DIR: "[CWD]"
[DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][ecs/src/ecs/mod.rs::229] dir: "[CWD]/.xvc/ec"
[TRACE][ecs/src/ecs/mod.rs::239] files: [
    "[CWD]/.xvc/ec/1703621569033665",
    "[CWD]/.xvc/ec/1703621569036208",
    "[CWD]/.xvc/ec/1703621569226287",
    "[CWD]/.xvc/ec/1703621569326541",
]
[TRACE][pipeline/src/pipeline/mod.rs::289] pipeline_e: XvcEntity(
    1,
    10394362838158298552,
)
[TRACE][pipeline/src/pipeline/mod.rs::294] pipeline_steps: HStore {
    map: {
        XvcEntity(
            2,
            1816263282869468159,
        ): XvcStep {
            name: "files-changed",
        },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::297] consider_changed: XvcStore {
    map: {
        XvcEntity(
            2,
            1816263282869468159,
        ): ByDependencies,
    },
    entity_index: {
        ByDependencies: [
            XvcEntity(
                2,
                1816263282869468159,
            ),
        ],
    },
    previous: EventLog(
        [
            Add {
                entity: XvcEntity(
                    2,
                    1816263282869468159,
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
    paths: 0x0000600000b10090,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000600002e1c0f0,
    runloop: Some(
        (
            0x00006000030040c0,
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
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
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
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "core.guid": String(
                            "f0430a8dd1732403",
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "core.guid": String(
                            "9132eb1ac816d387",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.command": String(
                            "git",
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
                            300,
                        ),
                        "TRYCMD_TESTS": String(
                            "pipeline",
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
                "file.track.force": XvcConfigValue {
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
                "file.carry-in.force": XvcConfigValue {
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
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "git.auto_stage": XvcConfigValue {
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
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
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
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "pipeline.process_pool_size": XvcConfigValue {
                    source: Project,
                    value: Integer(
                        4,
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
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "file.list.show_dot_files": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "9132eb1ac816d387",
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
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
                "TRYCMD_DURATION": XvcConfigValue {
                    source: Environment,
                    value: Integer(
                        300,
                    ),
                },
                "TRYCMD_TESTS": XvcConfigValue {
                    source: Environment,
                    value: String(
                        "pipeline",
                    ),
                },
            },
            init_params: XvcConfigInitParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"f0430a8dd1732403/"
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
            random: 15270554167383768940,
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
        1816263282869468159,
    ): [],
}
[TRACE][pipeline/src/pipeline/mod.rs::347] &dependency_graph: {
    XvcEntity(
        2,
        1816263282869468159,
    ): [],
}
[INFO][pipeline/src/pipeline/mod.rs::351] Pipeline Graph:
digraph {
    0 [ label = "(2, 1816263282869468159)" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::416] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                1816263282869468159,
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
            1816263282869468159,
        ): ScopedJoinHandle { .. },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::516] (step_e, &jh): (
    XvcEntity(
        2,
        1816263282869468159,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::627] params.recorded_dependencies: R1NStore {
    parents: XvcStore {
        map: {
            XvcEntity(
                2,
                1816263282869468159,
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
                    1816263282869468159,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        2,
                        1816263282869468159,
                    ),
                    value: XvcStep {
                        name: "files-changed",
                    },
                },
                Add {
                    entity: XvcEntity(
                        2,
                        1816263282869468159,
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
                11319049037403028065,
            ): GlobItems(
                GlobItemsDep {
                    glob: "dir-*/*",
                    xvc_path_metadata_map: {
                        XvcPath(
                            "dir-0001/file-0001.bin",
                        ): XvcMetadata {
                            file_type: File,
                            size: Some(
                                2001,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1703621569,
                                    tv_nsec: 179347038,
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
                                    tv_sec: 1703621569,
                                    tv_nsec: 179598704,
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
                                    tv_sec: 1703621569,
                                    tv_nsec: 179798828,
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
                                    tv_sec: 1703621569,
                                    tv_nsec: 180025828,
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
                                    tv_sec: 1703621569,
                                    tv_nsec: 180217327,
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
                                    tv_sec: 1703621569,
                                    tv_nsec: 181367491,
                                },
                            ),
                        },
                    },
                    xvc_path_content_digest_map: {
                        XvcPath(
                            "dir-0001/file-0001.bin",
                        ): ContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    41,
                                    56,
                                    242,
                                    4,
                                    207,
                                    182,
                                    83,
                                    74,
                                    214,
                                    3,
                                    155,
                                    105,
                                    139,
                                    227,
                                    253,
                                    211,
                                    180,
                                    115,
                                    4,
                                    103,
                                    23,
                                    9,
                                    172,
                                    168,
                                    42,
                                    79,
                                    31,
                                    202,
                                    218,
                                    61,
                                    80,
                                    139,
                                ],
                            },
                        ),
                        XvcPath(
                            "dir-0001/file-0002.bin",
                        ): ContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    251,
                                    41,
                                    227,
                                    163,
                                    207,
                                    179,
                                    235,
                                    143,
                                    41,
                                    74,
                                    186,
                                    182,
                                    30,
                                    0,
                                    142,
                                    234,
                                    202,
                                    240,
                                    77,
                                    88,
                                    158,
                                    119,
                                    54,
                                    236,
                                    54,
                                    43,
                                    57,
                                    218,
                                    104,
                                    69,
                                    133,
                                    82,
                                ],
                            },
                        ),
                        XvcPath(
                            "dir-0001/file-0003.bin",
                        ): ContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    146,
                                    60,
                                    185,
                                    120,
                                    0,
                                    210,
                                    119,
                                    13,
                                    198,
                                    123,
                                    54,
                                    87,
                                    185,
                                    79,
                                    109,
                                    122,
                                    187,
                                    131,
                                    178,
                                    202,
                                    14,
                                    3,
                                    230,
                                    118,
                                    26,
                                    235,
                                    34,
                                    134,
                                    145,
                                    65,
                                    49,
                                    205,
                                ],
                            },
                        ),
                        XvcPath(
                            "dir-0002/file-0001.bin",
                        ): ContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    41,
                                    56,
                                    242,
                                    4,
                                    207,
                                    182,
                                    83,
                                    74,
                                    214,
                                    3,
                                    155,
                                    105,
                                    139,
                                    227,
                                    253,
                                    211,
                                    180,
                                    115,
                                    4,
                                    103,
                                    23,
                                    9,
                                    172,
                                    168,
                                    42,
                                    79,
                                    31,
                                    202,
                                    218,
                                    61,
                                    80,
                                    139,
                                ],
                            },
                        ),
                        XvcPath(
                            "dir-0002/file-0002.bin",
                        ): ContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    251,
                                    41,
                                    227,
                                    163,
                                    207,
                                    179,
                                    235,
                                    143,
                                    41,
                                    74,
                                    186,
                                    182,
                                    30,
                                    0,
                                    142,
                                    234,
                                    202,
                                    240,
                                    77,
                                    88,
                                    158,
                                    119,
                                    54,
                                    236,
                                    54,
                                    43,
                                    57,
                                    218,
                                    104,
                                    69,
                                    133,
                                    82,
                                ],
                            },
                        ),
                        XvcPath(
                            "dir-0002/file-0003.bin",
                        ): ContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    146,
                                    60,
                                    185,
                                    120,
                                    0,
                                    210,
                                    119,
                                    13,
                                    198,
                                    123,
                                    54,
                                    87,
                                    185,
                                    79,
                                    109,
                                    122,
                                    187,
                                    131,
                                    178,
                                    202,
                                    14,
                                    3,
                                    230,
                                    118,
                                    26,
                                    235,
                                    34,
                                    134,
                                    145,
                                    65,
                                    49,
                                    205,
                                ],
                            },
                        ),
                    },
                },
            ),
        },
        entity_index: {
            GlobItems(
                GlobItemsDep {
                    glob: "dir-*/*",
                    xvc_path_metadata_map: {
                        XvcPath(
                            "dir-0001/file-0001.bin",
                        ): XvcMetadata {
                            file_type: File,
                            size: Some(
                                2001,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1703621569,
                                    tv_nsec: 179347038,
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
                                    tv_sec: 1703621569,
                                    tv_nsec: 179598704,
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
                                    tv_sec: 1703621569,
                                    tv_nsec: 179798828,
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
                                    tv_sec: 1703621569,
                                    tv_nsec: 180025828,
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
                                    tv_sec: 1703621569,
                                    tv_nsec: 180217327,
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
                                    tv_sec: 1703621569,
                                    tv_nsec: 181367491,
                                },
                            ),
                        },
                    },
                    xvc_path_content_digest_map: {
                        XvcPath(
                            "dir-0001/file-0001.bin",
                        ): ContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    41,
                                    56,
                                    242,
                                    4,
                                    207,
                                    182,
                                    83,
                                    74,
                                    214,
                                    3,
                                    155,
                                    105,
                                    139,
                                    227,
                                    253,
                                    211,
                                    180,
                                    115,
                                    4,
                                    103,
                                    23,
                                    9,
                                    172,
                                    168,
                                    42,
                                    79,
                                    31,
                                    202,
                                    218,
                                    61,
                                    80,
                                    139,
                                ],
                            },
                        ),
                        XvcPath(
                            "dir-0001/file-0002.bin",
                        ): ContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    251,
                                    41,
                                    227,
                                    163,
                                    207,
                                    179,
                                    235,
                                    143,
                                    41,
                                    74,
                                    186,
                                    182,
                                    30,
                                    0,
                                    142,
                                    234,
                                    202,
                                    240,
                                    77,
                                    88,
                                    158,
                                    119,
                                    54,
                                    236,
                                    54,
                                    43,
                                    57,
                                    218,
                                    104,
                                    69,
                                    133,
                                    82,
                                ],
                            },
                        ),
                        XvcPath(
                            "dir-0001/file-0003.bin",
                        ): ContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    146,
                                    60,
                                    185,
                                    120,
                                    0,
                                    210,
                                    119,
                                    13,
                                    198,
                                    123,
                                    54,
                                    87,
                                    185,
                                    79,
                                    109,
                                    122,
                                    187,
                                    131,
                                    178,
                                    202,
                                    14,
                                    3,
                                    230,
                                    118,
                                    26,
                                    235,
                                    34,
                                    134,
                                    145,
                                    65,
                                    49,
                                    205,
                                ],
                            },
                        ),
                        XvcPath(
                            "dir-0002/file-0001.bin",
                        ): ContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    41,
                                    56,
                                    242,
                                    4,
                                    207,
                                    182,
                                    83,
                                    74,
                                    214,
                                    3,
                                    155,
                                    105,
                                    139,
                                    227,
                                    253,
                                    211,
                                    180,
                                    115,
                                    4,
                                    103,
                                    23,
                                    9,
                                    172,
                                    168,
                                    42,
                                    79,
                                    31,
                                    202,
                                    218,
                                    61,
                                    80,
                                    139,
                                ],
                            },
                        ),
                        XvcPath(
                            "dir-0002/file-0002.bin",
                        ): ContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    251,
                                    41,
                                    227,
                                    163,
                                    207,
                                    179,
                                    235,
                                    143,
                                    41,
                                    74,
                                    186,
                                    182,
                                    30,
                                    0,
                                    142,
                                    234,
                                    202,
                                    240,
                                    77,
                                    88,
                                    158,
                                    119,
                                    54,
                                    236,
                                    54,
                                    43,
                                    57,
                                    218,
                                    104,
                                    69,
                                    133,
                                    82,
                                ],
                            },
                        ),
                        XvcPath(
                            "dir-0002/file-0003.bin",
                        ): ContentDigest(
                            XvcDigest {
                                algorithm: Blake3,
                                digest: [
                                    146,
                                    60,
                                    185,
                                    120,
                                    0,
                                    210,
                                    119,
                                    13,
                                    198,
                                    123,
                                    54,
                                    87,
                                    185,
                                    79,
                                    109,
                                    122,
                                    187,
                                    131,
                                    178,
                                    202,
                                    14,
                                    3,
                                    230,
                                    118,
                                    26,
                                    235,
                                    34,
                                    134,
                                    145,
                                    65,
                                    49,
                                    205,
                                ],
                            },
                        ),
                    },
                },
            ): [
                XvcEntity(
                    3,
                    11319049037403028065,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        11319049037403028065,
                    ),
                    value: GlobItems(
                        GlobItemsDep {
                            glob: "dir-*/*",
                            xvc_path_metadata_map: {},
                            xvc_path_content_digest_map: {},
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        3,
                        11319049037403028065,
                    ),
                    value: GlobItems(
                        GlobItemsDep {
                            glob: "dir-*/*",
                            xvc_path_metadata_map: {
                                XvcPath(
                                    "dir-0001/file-0001.bin",
                                ): XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        2001,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1703621569,
                                            tv_nsec: 179347038,
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
                                            tv_sec: 1703621569,
                                            tv_nsec: 179598704,
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
                                            tv_sec: 1703621569,
                                            tv_nsec: 179798828,
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
                                            tv_sec: 1703621569,
                                            tv_nsec: 180025828,
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
                                            tv_sec: 1703621569,
                                            tv_nsec: 180217327,
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
                                            tv_sec: 1703621569,
                                            tv_nsec: 181367491,
                                        },
                                    ),
                                },
                            },
                            xvc_path_content_digest_map: {
                                XvcPath(
                                    "dir-0001/file-0001.bin",
                                ): ContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            41,
                                            56,
                                            242,
                                            4,
                                            207,
                                            182,
                                            83,
                                            74,
                                            214,
                                            3,
                                            155,
                                            105,
                                            139,
                                            227,
                                            253,
                                            211,
                                            180,
                                            115,
                                            4,
                                            103,
                                            23,
                                            9,
                                            172,
                                            168,
                                            42,
                                            79,
                                            31,
                                            202,
                                            218,
                                            61,
                                            80,
                                            139,
                                        ],
                                    },
                                ),
                                XvcPath(
                                    "dir-0001/file-0002.bin",
                                ): ContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            251,
                                            41,
                                            227,
                                            163,
                                            207,
                                            179,
                                            235,
                                            143,
                                            41,
                                            74,
                                            186,
                                            182,
                                            30,
                                            0,
                                            142,
                                            234,
                                            202,
                                            240,
                                            77,
                                            88,
                                            158,
                                            119,
                                            54,
                                            236,
                                            54,
                                            43,
                                            57,
                                            218,
                                            104,
                                            69,
                                            133,
                                            82,
                                        ],
                                    },
                                ),
                                XvcPath(
                                    "dir-0001/file-0003.bin",
                                ): ContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            146,
                                            60,
                                            185,
                                            120,
                                            0,
                                            210,
                                            119,
                                            13,
                                            198,
                                            123,
                                            54,
                                            87,
                                            185,
                                            79,
                                            109,
                                            122,
                                            187,
                                            131,
                                            178,
                                            202,
                                            14,
                                            3,
                                            230,
                                            118,
                                            26,
                                            235,
                                            34,
                                            134,
                                            145,
                                            65,
                                            49,
                                            205,
                                        ],
                                    },
                                ),
                                XvcPath(
                                    "dir-0002/file-0001.bin",
                                ): ContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            41,
                                            56,
                                            242,
                                            4,
                                            207,
                                            182,
                                            83,
                                            74,
                                            214,
                                            3,
                                            155,
                                            105,
                                            139,
                                            227,
                                            253,
                                            211,
                                            180,
                                            115,
                                            4,
                                            103,
                                            23,
                                            9,
                                            172,
                                            168,
                                            42,
                                            79,
                                            31,
                                            202,
                                            218,
                                            61,
                                            80,
                                            139,
                                        ],
                                    },
                                ),
                                XvcPath(
                                    "dir-0002/file-0002.bin",
                                ): ContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            251,
                                            41,
                                            227,
                                            163,
                                            207,
                                            179,
                                            235,
                                            143,
                                            41,
                                            74,
                                            186,
                                            182,
                                            30,
                                            0,
                                            142,
                                            234,
                                            202,
                                            240,
                                            77,
                                            88,
                                            158,
                                            119,
                                            54,
                                            236,
                                            54,
                                            43,
                                            57,
                                            218,
                                            104,
                                            69,
                                            133,
                                            82,
                                        ],
                                    },
                                ),
                                XvcPath(
                                    "dir-0002/file-0003.bin",
                                ): ContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            146,
                                            60,
                                            185,
                                            120,
                                            0,
                                            210,
                                            119,
                                            13,
                                            198,
                                            123,
                                            54,
                                            87,
                                            185,
                                            79,
                                            109,
                                            122,
                                            187,
                                            131,
                                            178,
                                            202,
                                            14,
                                            3,
                                            230,
                                            118,
                                            26,
                                            235,
                                            34,
                                            134,
                                            145,
                                            65,
                                            49,
                                            205,
                                        ],
                                    },
                                ),
                            },
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
                11319049037403028065,
            ): ChildEntity(
                XvcEntity(
                    2,
                    1816263282869468159,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
        },
        entity_index: {
            ChildEntity(
                XvcEntity(
                    2,
                    1816263282869468159,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    3,
                    11319049037403028065,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        11319049037403028065,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            1816263282869468159,
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
    1816263282869468159,
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
[TRACE][pipeline/src/pipeline/mod.rs::776] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::777] &r_next_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::779] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::666] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
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
    1816263282869468159,
)
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1070] deps: HStore {
    map: {
        XvcEntity(
            3,
            11319049037403028065,
        ): GlobItems(
            GlobItemsDep {
                glob: "dir-*/*",
                xvc_path_metadata_map: {
                    XvcPath(
                        "dir-0001/file-0001.bin",
                    ): XvcMetadata {
                        file_type: File,
                        size: Some(
                            2001,
                        ),
                        modified: Some(
                            SystemTime {
                                tv_sec: 1703621569,
                                tv_nsec: 179347038,
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
                                tv_sec: 1703621569,
                                tv_nsec: 179598704,
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
                                tv_sec: 1703621569,
                                tv_nsec: 179798828,
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
                                tv_sec: 1703621569,
                                tv_nsec: 180025828,
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
                                tv_sec: 1703621569,
                                tv_nsec: 180217327,
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
                                tv_sec: 1703621569,
                                tv_nsec: 181367491,
                            },
                        ),
                    },
                },
                xvc_path_content_digest_map: {
                    XvcPath(
                        "dir-0001/file-0001.bin",
                    ): ContentDigest(
                        XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                41,
                                56,
                                242,
                                4,
                                207,
                                182,
                                83,
                                74,
                                214,
                                3,
                                155,
                                105,
                                139,
                                227,
                                253,
                                211,
                                180,
                                115,
                                4,
                                103,
                                23,
                                9,
                                172,
                                168,
                                42,
                                79,
                                31,
                                202,
                                218,
                                61,
                                80,
                                139,
                            ],
                        },
                    ),
                    XvcPath(
                        "dir-0001/file-0002.bin",
                    ): ContentDigest(
                        XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                251,
                                41,
                                227,
                                163,
                                207,
                                179,
                                235,
                                143,
                                41,
                                74,
                                186,
                                182,
                                30,
                                0,
                                142,
                                234,
                                202,
                                240,
                                77,
                                88,
                                158,
                                119,
                                54,
                                236,
                                54,
                                43,
                                57,
                                218,
                                104,
                                69,
                                133,
                                82,
                            ],
                        },
                    ),
                    XvcPath(
                        "dir-0001/file-0003.bin",
                    ): ContentDigest(
                        XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                146,
                                60,
                                185,
                                120,
                                0,
                                210,
                                119,
                                13,
                                198,
                                123,
                                54,
                                87,
                                185,
                                79,
                                109,
                                122,
                                187,
                                131,
                                178,
                                202,
                                14,
                                3,
                                230,
                                118,
                                26,
                                235,
                                34,
                                134,
                                145,
                                65,
                                49,
                                205,
                            ],
                        },
                    ),
                    XvcPath(
                        "dir-0002/file-0001.bin",
                    ): ContentDigest(
                        XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                41,
                                56,
                                242,
                                4,
                                207,
                                182,
                                83,
                                74,
                                214,
                                3,
                                155,
                                105,
                                139,
                                227,
                                253,
                                211,
                                180,
                                115,
                                4,
                                103,
                                23,
                                9,
                                172,
                                168,
                                42,
                                79,
                                31,
                                202,
                                218,
                                61,
                                80,
                                139,
                            ],
                        },
                    ),
                    XvcPath(
                        "dir-0002/file-0002.bin",
                    ): ContentDigest(
                        XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                251,
                                41,
                                227,
                                163,
                                207,
                                179,
                                235,
                                143,
                                41,
                                74,
                                186,
                                182,
                                30,
                                0,
                                142,
                                234,
                                202,
                                240,
                                77,
                                88,
                                158,
                                119,
                                54,
                                236,
                                54,
                                43,
                                57,
                                218,
                                104,
                                69,
                                133,
                                82,
                            ],
                        },
                    ),
                    XvcPath(
                        "dir-0002/file-0003.bin",
                    ): ContentDigest(
                        XvcDigest {
                            algorithm: Blake3,
                            digest: [
                                146,
                                60,
                                185,
                                120,
                                0,
                                210,
                                119,
                                13,
                                198,
                                123,
                                54,
                                87,
                                185,
                                79,
                                109,
                                122,
                                187,
                                131,
                                178,
                                202,
                                14,
                                3,
                                230,
                                118,
                                26,
                                235,
                                34,
                                134,
                                145,
                                65,
                                49,
                                205,
                            ],
                        },
                    ),
                },
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/deps/compare.rs::426] &stored: GlobItems(
    GlobItemsDep {
        glob: "dir-*/*",
        xvc_path_metadata_map: {
            XvcPath(
                "dir-0001/file-0001.bin",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    2001,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1703621569,
                        tv_nsec: 179347038,
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
                        tv_sec: 1703621569,
                        tv_nsec: 179598704,
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
                        tv_sec: 1703621569,
                        tv_nsec: 179798828,
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
                        tv_sec: 1703621569,
                        tv_nsec: 180025828,
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
                        tv_sec: 1703621569,
                        tv_nsec: 180217327,
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
                        tv_sec: 1703621569,
                        tv_nsec: 181367491,
                    },
                ),
            },
        },
        xvc_path_content_digest_map: {
            XvcPath(
                "dir-0001/file-0001.bin",
            ): ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        41,
                        56,
                        242,
                        4,
                        207,
                        182,
                        83,
                        74,
                        214,
                        3,
                        155,
                        105,
                        139,
                        227,
                        253,
                        211,
                        180,
                        115,
                        4,
                        103,
                        23,
                        9,
                        172,
                        168,
                        42,
                        79,
                        31,
                        202,
                        218,
                        61,
                        80,
                        139,
                    ],
                },
            ),
            XvcPath(
                "dir-0001/file-0002.bin",
            ): ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        251,
                        41,
                        227,
                        163,
                        207,
                        179,
                        235,
                        143,
                        41,
                        74,
                        186,
                        182,
                        30,
                        0,
                        142,
                        234,
                        202,
                        240,
                        77,
                        88,
                        158,
                        119,
                        54,
                        236,
                        54,
                        43,
                        57,
                        218,
                        104,
                        69,
                        133,
                        82,
                    ],
                },
            ),
            XvcPath(
                "dir-0001/file-0003.bin",
            ): ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        146,
                        60,
                        185,
                        120,
                        0,
                        210,
                        119,
                        13,
                        198,
                        123,
                        54,
                        87,
                        185,
                        79,
                        109,
                        122,
                        187,
                        131,
                        178,
                        202,
                        14,
                        3,
                        230,
                        118,
                        26,
                        235,
                        34,
                        134,
                        145,
                        65,
                        49,
                        205,
                    ],
                },
            ),
            XvcPath(
                "dir-0002/file-0001.bin",
            ): ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        41,
                        56,
                        242,
                        4,
                        207,
                        182,
                        83,
                        74,
                        214,
                        3,
                        155,
                        105,
                        139,
                        227,
                        253,
                        211,
                        180,
                        115,
                        4,
                        103,
                        23,
                        9,
                        172,
                        168,
                        42,
                        79,
                        31,
                        202,
                        218,
                        61,
                        80,
                        139,
                    ],
                },
            ),
            XvcPath(
                "dir-0002/file-0002.bin",
            ): ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        251,
                        41,
                        227,
                        163,
                        207,
                        179,
                        235,
                        143,
                        41,
                        74,
                        186,
                        182,
                        30,
                        0,
                        142,
                        234,
                        202,
                        240,
                        77,
                        88,
                        158,
                        119,
                        54,
                        236,
                        54,
                        43,
                        57,
                        218,
                        104,
                        69,
                        133,
                        82,
                    ],
                },
            ),
            XvcPath(
                "dir-0002/file-0003.bin",
            ): ContentDigest(
                XvcDigest {
                    algorithm: Blake3,
                    digest: [
                        146,
                        60,
                        185,
                        120,
                        0,
                        210,
                        119,
                        13,
                        198,
                        123,
                        54,
                        87,
                        185,
                        79,
                        109,
                        122,
                        187,
                        131,
                        178,
                        202,
                        14,
                        3,
                        230,
                        118,
                        26,
                        235,
                        34,
                        134,
                        145,
                        65,
                        49,
                        205,
                    ],
                },
            ),
        },
    },
)
[TRACE][core/src/util/file.rs::332] full_glob: "dir-*/*"
[TRACE][core/src/util/file.rs::215] glob: "dir-*/*"
[TRACE][core/src/util/file.rs::185] glob: "dir-*/*"
[TRACE][walker/src/lib.rs::749] is_abs: false
[TRACE][walker/src/lib.rs::753] path_str: "dir-0001/file-0001.bin"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "dir-0001/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/dir-0001/file-0001.bin"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/util/file.rs::196] xvc_path: XvcPath(
    "dir-0001/file-0001.bin",
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
            tv_sec: 1703621569,
            tv_nsec: 179347038,
        },
    ),
    accessed: Ok(
        SystemTime {
            tv_sec: 1703621569,
            tv_nsec: 425611509,
        },
    ),
    created: Ok(
        SystemTime {
            tv_sec: 1703621569,
            tv_nsec: 179114830,
        },
    ),
    ..
}
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
            tv_sec: 1703621569,
            tv_nsec: 179598704,
        },
    ),
    accessed: Ok(
        SystemTime {
            tv_sec: 1703621569,
            tv_nsec: 425788092,
        },
    ),
    created: Ok(
        SystemTime {
            tv_sec: 1703621569,
            tv_nsec: 179384913,
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
            tv_sec: 1703621569,
            tv_nsec: 179798828,
        },
    ),
    accessed: Ok(
        SystemTime {
            tv_sec: 1703621569,
            tv_nsec: 425898925,
        },
    ),
    created: Ok(
        SystemTime {
            tv_sec: 1703621569,
            tv_nsec: 179626537,
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
            tv_sec: 1703621569,
            tv_nsec: 180025828,
        },
    ),
    accessed: Ok(
        SystemTime {
            tv_sec: 1703621569,
            tv_nsec: 425993049,
        },
    ),
    created: Ok(
        SystemTime {
            tv_sec: 1703621569,
            tv_nsec: 179858953,
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
            tv_sec: 1703621569,
            tv_nsec: 180217327,
        },
    ),
    accessed: Ok(
        SystemTime {
            tv_sec: 1703621569,
            tv_nsec: 426084091,
        },
    ),
    created: Ok(
        SystemTime {
            tv_sec: 1703621569,
            tv_nsec: 180052703,
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
            tv_sec: 1703621569,
            tv_nsec: 181367491,
        },
    ),
    accessed: Ok(
        SystemTime {
            tv_sec: 1703621569,
            tv_nsec: 426176299,
        },
    ),
    created: Ok(
        SystemTime {
            tv_sec: 1703621569,
            tv_nsec: 180242702,
        },
    ),
    ..
}
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0002/file-0002.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0001/file-0003.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0002/file-0003.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0001/file-0002.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0001/file-0001.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/util/file.rs::220] p: XvcPath(
    "dir-0002/file-0001.bin",
)
[TRACE][core/src/util/file.rs::222] "matched: {p}": "matched: {p}"
[TRACE][core/src/types/diff.rs::295] record: Some(
    XvcMetadata {
        file_type: File,
        size: Some(
            2001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703621569,
                tv_nsec: 179347038,
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::296] actual: Some(
    XvcMetadata {
        file_type: File,
        size: Some(
            2001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703621569,
                tv_nsec: 179347038,
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::295] record: Some(
    XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703621569,
                tv_nsec: 179598704,
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::296] actual: Some(
    XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703621569,
                tv_nsec: 179598704,
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::295] record: Some(
    XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703621569,
                tv_nsec: 179798828,
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::296] actual: Some(
    XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703621569,
                tv_nsec: 179798828,
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::295] record: Some(
    XvcMetadata {
        file_type: File,
        size: Some(
            2001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703621569,
                tv_nsec: 180025828,
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::296] actual: Some(
    XvcMetadata {
        file_type: File,
        size: Some(
            2001,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703621569,
                tv_nsec: 180025828,
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::295] record: Some(
    XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703621569,
                tv_nsec: 180217327,
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::296] actual: Some(
    XvcMetadata {
        file_type: File,
        size: Some(
            2002,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703621569,
                tv_nsec: 180217327,
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::295] record: Some(
    XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703621569,
                tv_nsec: 181367491,
            },
        ),
    },
)
[TRACE][core/src/types/diff.rs::296] actual: Some(
    XvcMetadata {
        file_type: File,
        size: Some(
            2003,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1703621569,
                tv_nsec: 181367491,
            },
        ),
    },
)
[TRACE][pipeline/src/pipeline/mod.rs::1087] step_dependency_diffs: HStore {
    map: {
        XvcEntity(
            3,
            11319049037403028065,
        ): Identical,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1093] diff: Identical
[TRACE][pipeline/src/pipeline/mod.rs::1094] diff.changed(): false
[TRACE][pipeline/src/pipeline/mod.rs::1099] changed: false
[TRACE][pipeline/src/pipeline/mod.rs::776] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::777] &r_next_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::779] &step_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::666] &step_state: ComparingDiffsAndOutputs(
    FromSuperficialDiffsNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::1016] params.step_dependencies: {}
[TRACE][pipeline/src/pipeline/mod.rs::1054] changed: false
[TRACE][pipeline/src/pipeline/mod.rs::776] step.name: "files-changed"
[TRACE][pipeline/src/pipeline/mod.rs::777] &r_next_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::779] &step_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::666] &step_state: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::587] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::523] "Before state updater": "Before state updater"
[TRACE][pipeline/src/pipeline/mod.rs::597] s: DoneWithoutRunning(
    FromDiffsHasNotChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::533] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                1816263282869468159,
            ): DoneWithoutRunning(
                FromDiffsHasNotChanged,
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
[TRACE][lib/src/cli/mod.rs::584] git_add_output: ""

```

If you add or remove a file from the files specified by the glob, they are printed.

```console
$ rm dir-0001/file-0001.bin

$ xvc pipeline run
[OUT] [files-changed] ### Added Files:

### Removed Files:
dir-0001/file-0001.bin
### Changed Files:

 
[DONE] files-changed (echo "### Added Files:/n${XVC_GLOB_ADDED_ITEMS}/n### Removed Files:/n${XVC_GLOB_REMOVED_ITEMS}/n### Changed Files:/n${XVC_GLOB_CHANGED_ITEMS}")

```

When you change a file, it's printed in both added and removed files:

```console
$ xvc-test-helper generate-filled-file dir-0001/file-0002.bin

$ xvc pipeline run
[OUT] [files-changed] ### Added Files:

### Removed Files:

### Changed Files:
dir-0001/file-0002.bin
 
[DONE] files-changed (echo "### Added Files:/n${XVC_GLOB_ADDED_ITEMS}/n### Removed Files:/n${XVC_GLOB_REMOVED_ITEMS}/n### Changed Files:/n${XVC_GLOB_CHANGED_ITEMS}")

```
