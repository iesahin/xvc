# xvc

[![codecov](https://codecov.io/gh/iesahin/xvc/branch/main/graph/badge.svg?token=xa3ru5KhRq)](https://codecov.io/gh/iesahin/xvc)
[![build](https://img.shields.io/github/actions/workflow/status/iesahin/xvc/rust.yml?branch=main)](https://github.com/iesahin/xvc/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/xvc)](https://crates.io/crates/xvc)
[![docs.rs](https://img.shields.io/docsrs/xvc)](https://docs.rs/xvc/)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

A fast and robust MLOps tool to manage data and pipelines

## ⌛ When to use xvc?

- When you have a photo, audio, media, or document collection to backup/version with Git, but don't want to copy that huge data to all Git clones.
- When you manage a large number of _unstructured_ data, like images, documents, and audio files.
- When you want to version data files, and want to track versions across datasets.
- When you want to store this data in local, SSH-accessible, or S3-compatible cloud storage.
- When you create data pipelines on top of this data and want to run these pipelines when the data, code, or other dependencies change.
- When you want to track which subset of the data you're working with, and how it changes by your operations.
- When you have binary artifacts that you use as dependencies and would like to have a `make` alternative that considers _content changes_ rather than timestamps.

## ✳️ What is xvc for?

- (for x = files) Track large files on Git, store them in the cloud, create view-only subsets, retrieve them only when necessary.
- (for x = pipelines) Define and run data -> model pipelines whose dependencies may be files, hyperparameters, regex searches, arbitrary URLs, and more.
- (for x = data) Annotate data and run queries and retrieve subsets of it. ([TODO](https://github.com/iesahin/xvc/discussions/208))
- (for x = experiments) Run isolated experiments, share them and store them in Git when necessary ([TODO](https://github.com/iesahin/xvc/discussions/207))
- (for x = models) Associate models with datasets, metadata and features, then track, store, and deploy them ([TODO](https://github.com/iesahin/xvc/discussions/211))

## 🔽 Installation

You can get the binary files for Linux, macOS, and Windows from [releases](https://github.com/iesahin/xvc/releases/latest) page. Extract and copy the file to your `$PATH`.

Alternatively, if you have Rust [installed], you can build xvc:

```shell
$ cargo install xvc
```

[installed]: https://www.rust-lang.org/tools/install

## 🏃🏾 Quicktart

Xvc tracks your files and directories on top of Git. To start run the following command in the repository.

```console
$ git init # if you're not already in a Git repository
Initialized empty Git repository in [CWD]/.git/

$ xvc init
```

It initializes the metafiles in `.xvc/` directory and adds `.xvcignore` file for paths you may want to hide from Xvc.

Add your data files and directories for tracking.

```shell
$ xvc file track my-data/ --as symlink
```

The command calculates data content hashes (with BLAKE-3, by default) and records them.
It commits these changes to Git.
It also copies these files to content-addressed directories under `.xvc/b3` and creates read-only symbolic links to them.

You can specify different [recheck (checkout) methods](https://docs.xvc.dev/ref/xvc-file-recheck/) for files and directories, depending on your use case.
If you need to track model files that change frequently, you can set recheck method `--as copy` (the default).

```shell
$ xvc file track my-models/ --as copy
```

Configure a cloud storage to share the files you added.

```shell
$ xvc storage new s3 --name my-remote --region us-east-1 --bucket-name my-xvc-remote
```

You can send the files to this storage.

```shell
$ xvc file send --to my-remote
```

When you (or someone else) want to access these files later, you can clone the Git repository and get the files from the
storage.

```shell
$ git clone https://example.com/my-machine-learning-project
Cloning into 'my-machine-learning-project'...

$ cd my-machine-learning-project
$ xvc file bring my-data/ --from my-remote

```

You don't have to reconfigure the storage after cloning, but you need to have valid credentials as environment variables
to access the storage.
Xvc never stores any credentials.

If you have commands that depend on data or code elements, you can configure a pipeline.

For this example, we'll use [a Python script](https://github.com/iesahin/xvc/blob/main/workflow_tests/templates/README.in/generate_data.py) to generate a data set with random names with random IQ scores.

The script uses the Faker library and this library must be available where you run the pipeline. To make it repeatable, we start the pipeline by adding a step that installs dependencies.

```console
$ xvc pipeline step new --step-name install-deps --command 'python3 -m pip --user install -r requirements.txt'
```

We'll make this this step to depend on `requirements.txt` file, so when the file changes it will make the step run. 

```console
$ xvc pipeline step dependency --step-name install-deps --file requirements.txt
```

Xvc allows to create dependencies between pipeline steps. Dependent steps wait for dependencies to finish successfully. 

Now we create a step to run the script and make `install-deps` step a dependency of it. 

```console
$ xvc pipeline step new --step-name generate-data --command 'python generate_data.py'
$ xvc pipeline step dependency --step-name generate-data --step install-deps
```

After you define the pipeline, you can run it by:

```console
$ xvc pipeline run
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
    "core.verbosity": String(
        "quiet",
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
                "pipeline.default": String(
                    "default",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "pipeline.process_pool_size": Integer(
                    4,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "core.guid": String(
                    "2f7fec0f50a80cab",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
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
                "git.command": String(
                    "git",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "pipeline.default": String(
                    "default",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "core.guid": String(
                    "08069b366ac4f066",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "pipeline.process_pool_size": Integer(
                    4,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.command": String(
                    "git",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
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
                    "quiet",
                ),
                "core.quiet": Boolean(
                    false,
                ),
            },
        },
    ],
    the_config: {
        "file.recheck.method": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
            ),
        },
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
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
        "file.carry-in.no_parallel": XvcConfigValue {
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
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
        "file.carry-in.force": XvcConfigValue {
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
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "quiet",
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
        "git.auto_stage": XvcConfigValue {
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
        "core.quiet": XvcConfigValue {
            source: CommandLine,
            value: Boolean(
                false,
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "08069b366ac4f066",
            ),
        },
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
            ),
        },
        "file.list.no_summary": XvcConfigValue {
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
guid = /"2f7fec0f50a80cab/"
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
    "[CWD]/.xvc/ec/1697895835105968",
    "[CWD]/.xvc/ec/1697895835109582",
    "[CWD]/.xvc/ec/1697895835215494",
    "[CWD]/.xvc/ec/1697895835454154",
    "[CWD]/.xvc/ec/1697895835855598",
    "[CWD]/.xvc/ec/1697895836048368",
]
[TRACE][pipeline/src/lib.rs::360] name: Some(
    "default",
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
        path: "[CWD]/requirements.txt",
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
                    tv_sec: 1697833897,
                    tv_nsec: 438894855,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1697833919,
                    tv_nsec: 53743685,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1697833897,
                    tv_nsec: 438727564,
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
                    tv_sec: 1697895835,
                    tv_nsec: 106077873,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1697895835,
                    tv_nsec: 126617751,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1697895835,
                    tv_nsec: 98646145,
                },
            ),
            ..
        },
    },
    PathMetadata {
        path: "[CWD]/generate_data.py",
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
                    tv_sec: 1697619236,
                    tv_nsec: 954471630,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1697833919,
                    tv_nsec: 53723435,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1697619236,
                    tv_nsec: 954371716,
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
                    tv_sec: 1697895835,
                    tv_nsec: 106293375,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1697895835,
                    tv_nsec: 170627530,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1697895835,
                    tv_nsec: 106201207,
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
                    tv_sec: 1697895836,
                    tv_nsec: 84927864,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1697895834,
                    tv_nsec: 621067914,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1697895834,
                    tv_nsec: 621067914,
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
                    tv_sec: 1697895835,
                    tv_nsec: 106162457,
                },
            ),
            accessed: Ok(
                SystemTime {
                    tv_sec: 1697895835,
                    tv_nsec: 170788407,
                },
            ),
            created: Ok(
                SystemTime {
                    tv_sec: 1697895835,
                    tv_nsec: 106113082,
                },
            ),
            ..
        },
    },
]
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/requirements.txt"
[TRACE][walker/src/lib.rs::424] child_path.path: "[CWD]/.xvc"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/generate_data.py"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/.gitignore"
[TRACE][walker/src/lib.rs::424] child_path.path: "[CWD]/.git"
[TRACE][walker/src/lib.rs::412] child_path.path: "[CWD]/.xvcignore"
[TRACE][walker/src/lib.rs::452] "End of walk_parallel": "End of walk_parallel"
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/requirements.txt"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/generate_data.py"
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
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/.xvcignore"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][walker/src/notify.rs::160] watcher: FsEventWatcher {
    paths: 0x0000600002244000,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000600001d44010,
    runloop: Some(
        (
            0x000060000194c0c0,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::301] pipeline_len: 2
[TRACE][pipeline/src/pipeline/mod.rs::327] &dependency_graph: {
    XvcEntity(
        4,
        4493270232624921351,
    ): [
        (
            XvcEntity(
                2,
                9635879217041193202,
            ),
            Outgoing,
        ),
    ],
    XvcEntity(
        2,
        9635879217041193202,
    ): [
        (
            XvcEntity(
                4,
                4493270232624921351,
            ),
            Incoming,
        ),
    ],
}
[TRACE][pipeline/src/pipeline/mod.rs::339] &dependency_graph: {
    XvcEntity(
        4,
        4493270232624921351,
    ): [
        (
            XvcEntity(
                2,
                9635879217041193202,
            ),
            Outgoing,
        ),
    ],
    XvcEntity(
        2,
        9635879217041193202,
    ): [
        (
            XvcEntity(
                4,
                4493270232624921351,
            ),
            Incoming,
        ),
    ],
}
[INFO][pipeline/src/pipeline/mod.rs::343] Pipeline Graph:
digraph {
    0 [ label = "(4, 4493270232624921351)" ]
    1 [ label = "(2, 9635879217041193202)" ]
    0 -> 1 [ label = "Step" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::408] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                9635879217041193202,
            ): Begin(
                FromInit,
            ),
            XvcEntity(
                4,
                4493270232624921351,
            ): Begin(
                FromInit,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/notify-6.1.1/src/fsevent.rs::536] FSEvent: path = `[CWD]/.xvc/store/xvc-pipeline-run-dir-store`, flag = StreamFlags(ITEM_CREATED | IS_DIR)
[TRACE][walker/src/notify.rs::56] event: Ok(
    Event {
        kind: Create(
            Folder,
        ),
        paths: [
            "[CWD]/.xvc/store/xvc-pipeline-run-dir-store",
        ],
        attr:tracker: None,
        attr:flag: None,
        attr:info: None,
        attr:source: None,
    },
)
[TRACE][pipeline/src/pipeline/mod.rs::517] &step_thread_store: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): ScopedJoinHandle { .. },
        XvcEntity(
            4,
            4493270232624921351,
        ): ScopedJoinHandle { .. },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::521] (step_e, &jh): (
    XvcEntity(
        2,
        9635879217041193202,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::632] params.recorded_dependencies: R1NStore {
    parents: XvcStore {
        map: {
            XvcEntity(
                2,
                9635879217041193202,
            ): XvcStep {
                name: "install-deps",
            },
            XvcEntity(
                4,
                4493270232624921351,
            ): XvcStep {
                name: "generate-data",
            },
        },
        entity_index: {
            XvcStep {
                name: "generate-data",
            }: [
                XvcEntity(
                    4,
                    4493270232624921351,
                ),
            ],
            XvcStep {
                name: "install-deps",
            }: [
                XvcEntity(
                    2,
                    9635879217041193202,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        2,
                        9635879217041193202,
                    ),
                    value: XvcStep {
                        name: "install-deps",
                    },
                },
                Add {
                    entity: XvcEntity(
                        2,
                        9635879217041193202,
                    ),
                    value: XvcStep {
                        name: "install-deps",
                    },
                },
                Add {
                    entity: XvcEntity(
                        4,
                        4493270232624921351,
                    ),
                    value: XvcStep {
                        name: "generate-data",
                    },
                },
                Add {
                    entity: XvcEntity(
                        4,
                        4493270232624921351,
                    ),
                    value: XvcStep {
                        name: "generate-data",
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
                13544821595291233598,
            ): File(
                FileDep {
                    path: XvcPath(
                        "requirements.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
            XvcEntity(
                5,
                2191402314532019960,
            ): Step(
                StepDep {
                    name: "install-deps",
                },
            ),
        },
        entity_index: {
            Step(
                StepDep {
                    name: "install-deps",
                },
            ): [
                XvcEntity(
                    5,
                    2191402314532019960,
                ),
            ],
            File(
                FileDep {
                    path: XvcPath(
                        "requirements.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ): [
                XvcEntity(
                    3,
                    13544821595291233598,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        13544821595291233598,
                    ),
                    value: File(
                        FileDep {
                            path: XvcPath(
                                "requirements.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        2191402314532019960,
                    ),
                    value: Step(
                        StepDep {
                            name: "install-deps",
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
                13544821595291233598,
            ): ChildEntity(
                XvcEntity(
                    2,
                    9635879217041193202,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
            XvcEntity(
                5,
                2191402314532019960,
            ): ChildEntity(
                XvcEntity(
                    4,
                    4493270232624921351,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
        },
        entity_index: {
            ChildEntity(
                XvcEntity(
                    2,
                    9635879217041193202,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    3,
                    13544821595291233598,
                ),
            ],
            ChildEntity(
                XvcEntity(
                    4,
                    4493270232624921351,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    5,
                    2191402314532019960,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        13544821595291233598,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            9635879217041193202,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        2191402314532019960,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            4,
                            4493270232624921351,
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
[TRACE][pipeline/src/pipeline/mod.rs::633] step_e: XvcEntity(
    2,
    9635879217041193202,
)
[TRACE][pipeline/src/pipeline/mod.rs::574] dep_neighbors: Neighbors {
    iter: Iter(
        [
            (
                XvcEntity(
                    4,
                    4493270232624921351,
                ),
                Incoming,
            ),
        ],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::634] dependency_steps(step_e, params.dependency_graph)?: {}
[TRACE][pipeline/src/pipeline/mod.rs::574] dep_neighbors: Neighbors {
    iter: Iter(
        [
            (
                XvcEntity(
                    4,
                    4493270232624921351,
                ),
                Incoming,
            ),
        ],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][core/src/types/xvcpath.rs::88] abs_path: "[CWD]/.xvc/store/xvc-pipeline-run-dir-store"
[TRACE][core/src/types/xvcpath.rs::89] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::90] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::632] params.recorded_dependencies: R1NStore {
    parents: XvcStore {
        map: {
            XvcEntity(
                2,
                9635879217041193202,
            ): XvcStep {
                name: "install-deps",
            },
            XvcEntity(
                4,
                4493270232624921351,
            ): XvcStep {
                name: "generate-data",
            },
        },
        entity_index: {
            XvcStep {
                name: "generate-data",
            }: [
                XvcEntity(
                    4,
                    4493270232624921351,
                ),
            ],
            XvcStep {
                name: "install-deps",
            }: [
                XvcEntity(
                    2,
                    9635879217041193202,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        2,
                        9635879217041193202,
                    ),
                    value: XvcStep {
                        name: "install-deps",
                    },
                },
                Add {
                    entity: XvcEntity(
                        2,
                        9635879217041193202,
                    ),
                    value: XvcStep {
                        name: "install-deps",
                    },
                },
                Add {
                    entity: XvcEntity(
                        4,
                        4493270232624921351,
                    ),
                    value: XvcStep {
                        name: "generate-data",
                    },
                },
                Add {
                    entity: XvcEntity(
                        4,
                        4493270232624921351,
                    ),
                    value: XvcStep {
                        name: "generate-data",
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
                13544821595291233598,
            ): File(
                FileDep {
                    path: XvcPath(
                        "requirements.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ),
            XvcEntity(
                5,
                2191402314532019960,
            ): Step(
                StepDep {
                    name: "install-deps",
                },
            ),
        },
        entity_index: {
            Step(
                StepDep {
                    name: "install-deps",
                },
            ): [
                XvcEntity(
                    5,
                    2191402314532019960,
                ),
            ],
            File(
                FileDep {
                    path: XvcPath(
                        "requirements.txt",
                    ),
                    xvc_metadata: None,
                    content_digest: None,
                },
            ): [
                XvcEntity(
                    3,
                    13544821595291233598,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        13544821595291233598,
                    ),
                    value: File(
                        FileDep {
                            path: XvcPath(
                                "requirements.txt",
                            ),
                            xvc_metadata: None,
                            content_digest: None,
                        },
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        2191402314532019960,
                    ),
                    value: Step(
                        StepDep {
                            name: "install-deps",
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
                13544821595291233598,
            ): ChildEntity(
                XvcEntity(
                    2,
                    9635879217041193202,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
            XvcEntity(
                5,
                2191402314532019960,
            ): ChildEntity(
                XvcEntity(
                    4,
                    4493270232624921351,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
        },
        entity_index: {
            ChildEntity(
                XvcEntity(
                    2,
                    9635879217041193202,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    3,
                    13544821595291233598,
                ),
            ],
            ChildEntity(
                XvcEntity(
                    4,
                    4493270232624921351,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    5,
                    2191402314532019960,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        13544821595291233598,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            9635879217041193202,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
                Add {
                    entity: XvcEntity(
                        5,
                        2191402314532019960,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            4,
                            4493270232624921351,
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
[TRACE][pipeline/src/pipeline/mod.rs::633] step_e: XvcEntity(
    4,
    4493270232624921351,
)
[TRACE][pipeline/src/pipeline/mod.rs::574] dep_neighbors: Neighbors {
    iter: Iter(
        [
            (
                XvcEntity(
                    2,
                    9635879217041193202,
                ),
                Outgoing,
            ),
        ],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::634] dependency_steps(step_e, params.dependency_graph)?: {
    XvcEntity(
        2,
        9635879217041193202,
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::574] dep_neighbors: Neighbors {
    iter: Iter(
        [
            (
                XvcEntity(
                    2,
                    9635879217041193202,
                ),
                Outgoing,
            ),
        ],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::671] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::781] step.name: "install-deps"
[TRACE][pipeline/src/pipeline/mod.rs::782] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::784] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::671] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::781] step.name: "install-deps"
[TRACE][pipeline/src/pipeline/mod.rs::782] &r_next_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::784] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::671] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::781] step.name: "install-deps"
[TRACE][pipeline/src/pipeline/mod.rs::782] &r_next_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::784] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::671] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::1071] parent_entity: XvcEntity(
    2,
    9635879217041193202,
)
[TRACE][pipeline/src/pipeline/mod.rs::671] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::781] step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::782] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::784] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::1074] deps: HStore {
    map: {
        XvcEntity(
            3,
            13544821595291233598,
        ): File(
            FileDep {
                path: XvcPath(
                    "requirements.txt",
                ),
                xvc_metadata: None,
                content_digest: None,
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::671] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::781] step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::782] &r_next_state: WaitingDependencySteps(
    FromDependencyStepsRunning,
)
[TRACE][pipeline/src/pipeline/mod.rs::784] &step_state: WaitingDependencySteps(
    FromDependencyStepsRunning,
)
[TRACE][pipeline/src/pipeline/mod.rs::671] &step_state: WaitingDependencySteps(
    FromDependencyStepsRunning,
)
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Begin(
            FromInit,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/deps/compare.rs::429] &stored: File(
    FileDep {
        path: XvcPath(
            "requirements.txt",
        ),
        xvc_metadata: None,
        content_digest: None,
    },
)
[TRACE][pipeline/src/pipeline/deps/compare.rs::475] actual: FileDep {
    path: XvcPath(
        "requirements.txt",
    ),
    xvc_metadata: Some(
        XvcMetadata {
            file_type: File,
            size: Some(
                6,
            ),
            modified: Some(
                SystemTime {
                    tv_sec: 1697833897,
                    tv_nsec: 438894855,
                },
            ),
        },
    ),
    content_digest: None,
}
[TRACE][pipeline/src/pipeline/mod.rs::1091] step_dependency_diffs: HStore {
    map: {
        XvcEntity(
            3,
            13544821595291233598,
        ): RecordMissing {
            actual: File(
                FileDep {
                    path: XvcPath(
                        "requirements.txt",
                    ),
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                6,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1697833897,
                                    tv_nsec: 438894855,
                                },
                            ),
                        },
                    ),
                    content_digest: None,
                },
            ),
        },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1097] diff: RecordMissing {
    actual: File(
        FileDep {
            path: XvcPath(
                "requirements.txt",
            ),
            xvc_metadata: Some(
                XvcMetadata {
                    file_type: File,
                    size: Some(
                        6,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1697833897,
                            tv_nsec: 438894855,
                        },
                    ),
                },
            ),
            content_digest: None,
        },
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::1098] diff.changed(): true
[TRACE][pipeline/src/pipeline/mod.rs::1103] changed: true
[TRACE][pipeline/src/pipeline/mod.rs::781] step.name: "install-deps"
[TRACE][pipeline/src/pipeline/mod.rs::782] &r_next_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::784] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::671] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::1128] deps: HStore {
    map: {
        XvcEntity(
            3,
            13544821595291233598,
        ): File(
            FileDep {
                path: XvcPath(
                    "requirements.txt",
                ),
                xvc_metadata: None,
                content_digest: None,
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/deps/compare.rs::278] actual: FileDep {
    path: XvcPath(
        "requirements.txt",
    ),
    xvc_metadata: Some(
        XvcMetadata {
            file_type: File,
            size: Some(
                6,
            ),
            modified: Some(
                SystemTime {
                    tv_sec: 1697833897,
                    tv_nsec: 438894855,
                },
            ),
        },
    ),
    content_digest: Some(
        ContentDigest(
            XvcDigest {
                algorithm: Blake3,
                digest: [
                    232,
                    40,
                    83,
                    0,
                    143,
                    240,
                    127,
                    164,
                    19,
                    249,
                    146,
                    78,
                    143,
                    73,
                    96,
                    30,
                    4,
                    5,
                    224,
                    10,
                    135,
                    134,
                    230,
                    79,
                    14,
                    139,
                    250,
                    87,
                    190,
                    249,
                    204,
                    104,
                ],
            },
        ),
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::781] step.name: "install-deps"
[TRACE][pipeline/src/pipeline/mod.rs::782] &r_next_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::784] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::671] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::781] step.name: "install-deps"
[TRACE][pipeline/src/pipeline/mod.rs::782] &r_next_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::784] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::671] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
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
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "core.guid": String(
                            "2f7fec0f50a80cab",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
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
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "pipeline.default": String(
                            "default",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "08069b366ac4f066",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
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
                            "quiet",
                        ),
                        "core.quiet": Boolean(
                            false,
                        ),
                    },
                },
            ],
            the_config: {
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
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
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
                "file.carry-in.force": XvcConfigValue {
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
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "quiet",
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
                "git.auto_stage": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
                    value: Boolean(
                        false,
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "08069b366ac4f066",
                    ),
                },
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
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
guid = /"2f7fec0f50a80cab/"
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
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 6,
            random: 13800902881225386326,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                ".xvcignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    130,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1697895835,
                        tv_nsec: 106162457,
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
                        tv_sec: 1697895835,
                        tv_nsec: 106293375,
                    },
                ),
            },
            XvcPath(
                "requirements.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    6,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1697833897,
                        tv_nsec: 438894855,
                    },
                ),
            },
            XvcPath(
                "generate_data.py",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    739,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1697619236,
                        tv_nsec: 954471630,
                    },
                ),
            },
        },
        poisoned: false,
        ..
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
                name: "install-deps",
            },
            step_command: XvcStepCommand {
                command: "python3 -m pip --user install -r requirements.txt",
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
                    13544821595291233598,
                ): RecordMissing {
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "requirements.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        6,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1697833897,
                                            tv_nsec: 438894855,
                                        },
                                    ),
                                },
                            ),
                            content_digest: Some(
                                ContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            232,
                                            40,
                                            83,
                                            0,
                                            143,
                                            240,
                                            127,
                                            164,
                                            19,
                                            249,
                                            146,
                                            78,
                                            143,
                                            73,
                                            96,
                                            30,
                                            4,
                                            5,
                                            224,
                                            10,
                                            135,
                                            134,
                                            230,
                                            79,
                                            14,
                                            139,
                                            250,
                                            87,
                                            190,
                                            249,
                                            204,
                                            104,
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
        9635879217041193202,
    ),
    step: XvcStep {
        name: "install-deps",
    },
    step_command: XvcStepCommand {
        command: "python3 -m pip --user install -r requirements.txt",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    9635879217041193202,
                ): CheckingSuperficialDiffs(
                    FromCheckedOutputs,
                ),
                XvcEntity(
                    4,
                    4493270232624921351,
                ): WaitingDependencySteps(
                    FromDependencyStepsRunning,
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
                4,
                4493270232624921351,
            ): XvcStep {
                name: "generate-data",
            },
            XvcEntity(
                2,
                9635879217041193202,
            ): XvcStep {
                name: "install-deps",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    9635879217041193202,
                ): XvcStep {
                    name: "install-deps",
                },
                XvcEntity(
                    4,
                    4493270232624921351,
                ): XvcStep {
                    name: "generate-data",
                },
            },
            entity_index: {
                XvcStep {
                    name: "generate-data",
                }: [
                    XvcEntity(
                        4,
                        4493270232624921351,
                    ),
                ],
                XvcStep {
                    name: "install-deps",
                }: [
                    XvcEntity(
                        2,
                        9635879217041193202,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            9635879217041193202,
                        ),
                        value: XvcStep {
                            name: "install-deps",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            9635879217041193202,
                        ),
                        value: XvcStep {
                            name: "install-deps",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            4,
                            4493270232624921351,
                        ),
                        value: XvcStep {
                            name: "generate-data",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            4,
                            4493270232624921351,
                        ),
                        value: XvcStep {
                            name: "generate-data",
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
                    13544821595291233598,
                ): File(
                    FileDep {
                        path: XvcPath(
                            "requirements.txt",
                        ),
                        xvc_metadata: None,
                        content_digest: None,
                    },
                ),
                XvcEntity(
                    5,
                    2191402314532019960,
                ): Step(
                    StepDep {
                        name: "install-deps",
                    },
                ),
            },
            entity_index: {
                Step(
                    StepDep {
                        name: "install-deps",
                    },
                ): [
                    XvcEntity(
                        5,
                        2191402314532019960,
                    ),
                ],
                File(
                    FileDep {
                        path: XvcPath(
                            "requirements.txt",
                        ),
                        xvc_metadata: None,
                        content_digest: None,
                    },
                ): [
                    XvcEntity(
                        3,
                        13544821595291233598,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            13544821595291233598,
                        ),
                        value: File(
                            FileDep {
                                path: XvcPath(
                                    "requirements.txt",
                                ),
                                xvc_metadata: None,
                                content_digest: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2191402314532019960,
                        ),
                        value: Step(
                            StepDep {
                                name: "install-deps",
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
                    13544821595291233598,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        9635879217041193202,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    5,
                    2191402314532019960,
                ): ChildEntity(
                    XvcEntity(
                        4,
                        4493270232624921351,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        9635879217041193202,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        3,
                        13544821595291233598,
                    ),
                ],
                ChildEntity(
                    XvcEntity(
                        4,
                        4493270232624921351,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        5,
                        2191402314532019960,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            13544821595291233598,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                9635879217041193202,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2191402314532019960,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                4,
                                4493270232624921351,
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
[TRACE][pipeline/src/pipeline/mod.rs::781] step.name: "install-deps"
[TRACE][pipeline/src/pipeline/mod.rs::782] &r_next_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::784] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::671] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/command.rs::81] self.environment: {}
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromStartProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromStartProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::781] step.name: "install-deps"
[TRACE][pipeline/src/pipeline/mod.rs::782] &r_next_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::784] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::671] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1482] params: StepStateParams {
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
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "core.guid": String(
                            "2f7fec0f50a80cab",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
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
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "pipeline.default": String(
                            "default",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "08069b366ac4f066",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
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
                            "quiet",
                        ),
                        "core.quiet": Boolean(
                            false,
                        ),
                    },
                },
            ],
            the_config: {
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
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
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
                "file.carry-in.force": XvcConfigValue {
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
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "quiet",
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
                "git.auto_stage": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
                    value: Boolean(
                        false,
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "08069b366ac4f066",
                    ),
                },
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
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
guid = /"2f7fec0f50a80cab/"
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
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 6,
            random: 13800902881225386326,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                ".xvcignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    130,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1697895835,
                        tv_nsec: 106162457,
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
                        tv_sec: 1697895835,
                        tv_nsec: 106293375,
                    },
                ),
            },
            XvcPath(
                "requirements.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    6,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1697833897,
                        tv_nsec: 438894855,
                    },
                ),
            },
            XvcPath(
                "generate_data.py",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    739,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1697619236,
                        tv_nsec: 954471630,
                    },
                ),
            },
        },
        poisoned: false,
        ..
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
                name: "install-deps",
            },
            step_command: XvcStepCommand {
                command: "python3 -m pip --user install -r requirements.txt",
            },
            birth: Some(
                Instant {
                    tv_sec: 650632,
                    tv_nsec: 44381083,
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
                        pid: 57876,
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
                    13544821595291233598,
                ): RecordMissing {
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "requirements.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        6,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1697833897,
                                            tv_nsec: 438894855,
                                        },
                                    ),
                                },
                            ),
                            content_digest: Some(
                                ContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            232,
                                            40,
                                            83,
                                            0,
                                            143,
                                            240,
                                            127,
                                            164,
                                            19,
                                            249,
                                            146,
                                            78,
                                            143,
                                            73,
                                            96,
                                            30,
                                            4,
                                            5,
                                            224,
                                            10,
                                            135,
                                            134,
                                            230,
                                            79,
                                            14,
                                            139,
                                            250,
                                            87,
                                            190,
                                            249,
                                            204,
                                            104,
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
        9635879217041193202,
    ),
    step: XvcStep {
        name: "install-deps",
    },
    step_command: XvcStepCommand {
        command: "python3 -m pip --user install -r requirements.txt",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    9635879217041193202,
                ): Running(
                    FromWaitProcess,
                ),
                XvcEntity(
                    4,
                    4493270232624921351,
                ): WaitingDependencySteps(
                    FromDependencyStepsRunning,
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
                4,
                4493270232624921351,
            ): XvcStep {
                name: "generate-data",
            },
            XvcEntity(
                2,
                9635879217041193202,
            ): XvcStep {
                name: "install-deps",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    9635879217041193202,
                ): XvcStep {
                    name: "install-deps",
                },
                XvcEntity(
                    4,
                    4493270232624921351,
                ): XvcStep {
                    name: "generate-data",
                },
            },
            entity_index: {
                XvcStep {
                    name: "generate-data",
                }: [
                    XvcEntity(
                        4,
                        4493270232624921351,
                    ),
                ],
                XvcStep {
                    name: "install-deps",
                }: [
                    XvcEntity(
                        2,
                        9635879217041193202,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            9635879217041193202,
                        ),
                        value: XvcStep {
                            name: "install-deps",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            9635879217041193202,
                        ),
                        value: XvcStep {
                            name: "install-deps",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            4,
                            4493270232624921351,
                        ),
                        value: XvcStep {
                            name: "generate-data",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            4,
                            4493270232624921351,
                        ),
                        value: XvcStep {
                            name: "generate-data",
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
                    13544821595291233598,
                ): File(
                    FileDep {
                        path: XvcPath(
                            "requirements.txt",
                        ),
                        xvc_metadata: None,
                        content_digest: None,
                    },
                ),
                XvcEntity(
                    5,
                    2191402314532019960,
                ): Step(
                    StepDep {
                        name: "install-deps",
                    },
                ),
            },
            entity_index: {
                Step(
                    StepDep {
                        name: "install-deps",
                    },
                ): [
                    XvcEntity(
                        5,
                        2191402314532019960,
                    ),
                ],
                File(
                    FileDep {
                        path: XvcPath(
                            "requirements.txt",
                        ),
                        xvc_metadata: None,
                        content_digest: None,
                    },
                ): [
                    XvcEntity(
                        3,
                        13544821595291233598,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            13544821595291233598,
                        ),
                        value: File(
                            FileDep {
                                path: XvcPath(
                                    "requirements.txt",
                                ),
                                xvc_metadata: None,
                                content_digest: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2191402314532019960,
                        ),
                        value: Step(
                            StepDep {
                                name: "install-deps",
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
                    13544821595291233598,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        9635879217041193202,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    5,
                    2191402314532019960,
                ): ChildEntity(
                    XvcEntity(
                        4,
                        4493270232624921351,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        9635879217041193202,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        3,
                        13544821595291233598,
                    ),
                ],
                ChildEntity(
                    XvcEntity(
                        4,
                        4493270232624921351,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        5,
                        2191402314532019960,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            13544821595291233598,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                9635879217041193202,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2191402314532019960,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                4,
                                4493270232624921351,
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
        pid: 57876,
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
        pid: 57876,
        ext: (),
    },
    detached: true,
}
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
[OUT] [install-deps]  
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Running(
            FromWaitProcess,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::928] params.step.name: "generate-data"
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
        pid: 57876,
        ext: (),
    },
    detached: true,
}
[ERROR] Step install-deps finished UNSUCCESSFULLY with command python3 -m pip --user install -r requirements.txt
[OUT] [install-deps]  
[TRACE][pipeline/src/pipeline/mod.rs::1573] return_state: Some(
    Broken(
        FromProcessReturnedNonZero,
    ),
)
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
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "core.guid": String(
                            "2f7fec0f50a80cab",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
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
                        "git.command": String(
                            "git",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "pipeline.default": String(
                            "default",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "core.guid": String(
                            "08069b366ac4f066",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
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
                            "quiet",
                        ),
                        "core.quiet": Boolean(
                            false,
                        ),
                    },
                },
            ],
            the_config: {
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
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
                "file.carry-in.no_parallel": XvcConfigValue {
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
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
                "file.carry-in.force": XvcConfigValue {
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
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "quiet",
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
                "git.auto_stage": XvcConfigValue {
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
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
                    value: Boolean(
                        false,
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "08069b366ac4f066",
                    ),
                },
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
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
guid = /"2f7fec0f50a80cab/"
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
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 6,
            random: 13800902881225386326,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmm: RwLock {
        data: {
            XvcPath(
                ".xvcignore",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    130,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1697895835,
                        tv_nsec: 106162457,
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
                        tv_sec: 1697895835,
                        tv_nsec: 106293375,
                    },
                ),
            },
            XvcPath(
                "requirements.txt",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    6,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1697833897,
                        tv_nsec: 438894855,
                    },
                ),
            },
            XvcPath(
                "generate_data.py",
            ): XvcMetadata {
                file_type: File,
                size: Some(
                    739,
                ),
                modified: Some(
                    SystemTime {
                        tv_sec: 1697619236,
                        tv_nsec: 954471630,
                    },
                ),
            },
        },
        poisoned: false,
        ..
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
                name: "install-deps",
            },
            step_command: XvcStepCommand {
                command: "python3 -m pip --user install -r requirements.txt",
            },
            birth: Some(
                Instant {
                    tv_sec: 650632,
                    tv_nsec: 44381083,
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
                            2,
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
                    13544821595291233598,
                ): RecordMissing {
                    actual: File(
                        FileDep {
                            path: XvcPath(
                                "requirements.txt",
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        6,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1697833897,
                                            tv_nsec: 438894855,
                                        },
                                    ),
                                },
                            ),
                            content_digest: Some(
                                ContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            232,
                                            40,
                                            83,
                                            0,
                                            143,
                                            240,
                                            127,
                                            164,
                                            19,
                                            249,
                                            146,
                                            78,
                                            143,
                                            73,
                                            96,
                                            30,
                                            4,
                                            5,
                                            224,
                                            10,
                                            135,
                                            134,
                                            230,
                                            79,
                                            14,
                                            139,
                                            250,
                                            87,
                                            190,
                                            249,
                                            204,
                                            104,
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
        9635879217041193202,
    ),
    step: XvcStep {
        name: "install-deps",
    },
    step_command: XvcStepCommand {
        command: "python3 -m pip --user install -r requirements.txt",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    9635879217041193202,
                ): Running(
                    FromWaitProcess,
                ),
                XvcEntity(
                    4,
                    4493270232624921351,
                ): WaitingDependencySteps(
                    FromDependencyStepsRunning,
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
                4,
                4493270232624921351,
            ): XvcStep {
                name: "generate-data",
            },
            XvcEntity(
                2,
                9635879217041193202,
            ): XvcStep {
                name: "install-deps",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    9635879217041193202,
                ): XvcStep {
                    name: "install-deps",
                },
                XvcEntity(
                    4,
                    4493270232624921351,
                ): XvcStep {
                    name: "generate-data",
                },
            },
            entity_index: {
                XvcStep {
                    name: "generate-data",
                }: [
                    XvcEntity(
                        4,
                        4493270232624921351,
                    ),
                ],
                XvcStep {
                    name: "install-deps",
                }: [
                    XvcEntity(
                        2,
                        9635879217041193202,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            9635879217041193202,
                        ),
                        value: XvcStep {
                            name: "install-deps",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            9635879217041193202,
                        ),
                        value: XvcStep {
                            name: "install-deps",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            4,
                            4493270232624921351,
                        ),
                        value: XvcStep {
                            name: "generate-data",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            4,
                            4493270232624921351,
                        ),
                        value: XvcStep {
                            name: "generate-data",
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
                    13544821595291233598,
                ): File(
                    FileDep {
                        path: XvcPath(
                            "requirements.txt",
                        ),
                        xvc_metadata: None,
                        content_digest: None,
                    },
                ),
                XvcEntity(
                    5,
                    2191402314532019960,
                ): Step(
                    StepDep {
                        name: "install-deps",
                    },
                ),
            },
            entity_index: {
                Step(
                    StepDep {
                        name: "install-deps",
                    },
                ): [
                    XvcEntity(
                        5,
                        2191402314532019960,
                    ),
                ],
                File(
                    FileDep {
                        path: XvcPath(
                            "requirements.txt",
                        ),
                        xvc_metadata: None,
                        content_digest: None,
                    },
                ): [
                    XvcEntity(
                        3,
                        13544821595291233598,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            13544821595291233598,
                        ),
                        value: File(
                            FileDep {
                                path: XvcPath(
                                    "requirements.txt",
                                ),
                                xvc_metadata: None,
                                content_digest: None,
                            },
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2191402314532019960,
                        ),
                        value: Step(
                            StepDep {
                                name: "install-deps",
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
                    13544821595291233598,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        9635879217041193202,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
                XvcEntity(
                    5,
                    2191402314532019960,
                ): ChildEntity(
                    XvcEntity(
                        4,
                        4493270232624921351,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        9635879217041193202,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        3,
                        13544821595291233598,
                    ),
                ],
                ChildEntity(
                    XvcEntity(
                        4,
                        4493270232624921351,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        5,
                        2191402314532019960,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            13544821595291233598,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                9635879217041193202,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                    Add {
                        entity: XvcEntity(
                            5,
                            2191402314532019960,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                4,
                                4493270232624921351,
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
[TRACE][pipeline/src/pipeline/mod.rs::781] step.name: "install-deps"
[TRACE][pipeline/src/pipeline/mod.rs::782] &r_next_state: Broken(
    FromProcessReturnedNonZero,
)
[TRACE][pipeline/src/pipeline/mod.rs::784] &step_state: Broken(
    FromProcessReturnedNonZero,
)
[TRACE][pipeline/src/pipeline/mod.rs::671] &step_state: Broken(
    FromProcessReturnedNonZero,
)
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::521] (step_e, &jh): (
    XvcEntity(
        4,
        4493270232624921351,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::891] dep_states: HStore {
    map: {
        XvcEntity(
            2,
            9635879217041193202,
        ): Broken(
            FromProcessReturnedNonZero,
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::781] step.name: "generate-data"
[TRACE][pipeline/src/pipeline/mod.rs::782] &r_next_state: Broken(
    FromDependencyStepsFinishedBroken,
)
[TRACE][pipeline/src/pipeline/mod.rs::784] &step_state: Broken(
    FromDependencyStepsFinishedBroken,
)
[TRACE][pipeline/src/pipeline/mod.rs::671] &step_state: Broken(
    FromDependencyStepsFinishedBroken,
)
[TRACE][pipeline/src/pipeline/mod.rs::593] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::528] "Before state updater": "Before state updater"
[TRACE][pipeline/src/pipeline/mod.rs::542] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                9635879217041193202,
            ): Broken(
                FromProcessReturnedNonZero,
            ),
            XvcEntity(
                4,
                4493270232624921351,
            ): Broken(
                FromDependencyStepsFinishedBroken,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::546] done_successfully: Ok(
    false,
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

Xvc allows many kinds of dependnecies, like [files](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#file-dependencies), 
[groups of files and directories defined by globs](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#glob-dependencies), 
[regular expression searches in files](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#regex-dependencies), 
[line ranges in files](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#line-dependencies), 
[hyper-parameters defined in YAML, JSON or TOML files](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#hyper-parameter-dependencies)
[HTTP URLs](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#url-dependencies),
[shell command outputs](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#generic-command-dependencies), 
and [other steps](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#step-dependencies). 

Suppose you're only interested in the IQ scores of Greg's and how they differ from the rest in the dataset we created above. Let's create a regex search dependency to the data file that will show all Greg's IQ scores. 

```console
$ xvc pipeline step new --step-name greg-iq --command 'echo "${XVC_REGEX_ALL_ITEMS}" > greg-iq-scores.csv '
$ xvc pipeline step dependency --step-name greg-iq --regex-items 'random_names_iq_scores.csv:/.*Greg.*'
```



You can get the pipeline in Graphviz DOT format to convert to an image.

```console
$ xvc pipeline dag
digraph pipeline{n0[shape=box;label="install-deps";];n1[shape=note;label="requirements.txt";];n0->n1;n2[shape=box;label="generate-data";];n0[shape=box;label="install-deps";];n2->n0;n3[shape=box;label="greg-iq";];n4[shape=signature;label="random_names_iq_scores.csv:/.*Greg.*";];n3->n4;}

```

You can also export and import the pipeline to JSON to edit in your editor.

```console
$ xvc pipeline export > my-pipeline.json
? 2
error: unexpected argument '>' found

Usage: xvc pipeline export [OPTIONS]

For more information, try '--help'.

$ nvim my-pipeline.json
$ xvc pipeline import --file my-pipeline.json --overwrite
[ERROR] Pipeline Error: I/O Error: No such file or directory (os error 2)

```

You can run the pipeline with.

```shell
$ xvc pipeline run
```

If the steps you defined doesn't depend to each other, they are run in parallel.

You can define fairly complex dependencies with globs, files, directories, regular expression searches in files, lines in files, other steps and pipelines with `xvc pipeline step dependency` commands.
More dependency types like database queries, content from URLs, S3 (and compatible) buckets, REST and GraphQL results are in the backlog.
Please create an issue or discussion for any other kinds of dependencies that you'd like to be included.

Please check [`docs.xvc.dev`](https://docs.xvc.dev) for documentation.

## 🤟 Big Thanks

xvc stands on the following (giant) crates:

- [trycmd] is used to run all example commands in the [reference and how-to documentation](https://docs.xvc.dev) at
  every PR. It makes sure that the documentation is always up-to-date and shown commands work as described. We start
  development by writing documentation and implementing them thanks to [trycmd].

- [serde] allows all data structures to be stored in text files. Special thanks from [`xvc-ecs`] for serializing components in an ECS with a single line of code.

- Xvc processes files in parallel with pipelines and parallel iterators thanks to [crossbeam] and [rayon].

- Thanks to [strum], Xvc uses enums extensively and converts almost everything to typed values from strings.

- Xvc has a deep CLI that has subcommands of subcommands (e.g. `xvc storage new s3`), and all these work with minimum bugs thanks to [clap].

- Xvc uses [rust-s3] to connect to S3 and compatible storage services. It employs excellent [tokio] for fast async Rust. These cloud storage features can be turned off thanks to Rust conditional compilation.

- Without implementations of [BLAKE3], BLAKE2, SHA-2 and SHA-3 from Rust [crypto] crate, Xvc couldn't detect file changes so fast.

- Many thanks to small and well built crates, [reflink], [relative-path], [path-absolutize], [glob] for file system and glob handling.

- Thanks to [sad_machine] for providing a State Machine implementation that I used in `xvc pipeline run`. A DAG composed of State Machines made running pipeline steps in parallel with a clean separation of process states.

- Thanks to [thiserror] and [anyhow] for making error handling a breeze. These two crates make me feel I'm doing something good for the humanity when handling errors.

- Xvc is split into many crates and owes this organization to [cargo workspaces].

[crossbeam]: https://docs.rs/crossbeam/
[cargo workspaces]: https://crates.io/crates/cargo-workspaces
[rayon]: https://docs.rs/rayon/
[strum]: https://docs.rs/strum/
[clap]: https://docs.rs/clap/
[serde]: https://serde.rs
[blake3]: https://docs.rs/blake3/
[crypto]: https://docs.rs/rust-crypto/
[reflink]: https://docs.rs/reflink/
[relative-path]: https://docs.rs/relative-path/
[path-absolutize]: https://docs.rs/path-absolutize/
[glob]: https://docs.rs/glob/
[wax]: https://docs.rs/wax/
[trycmd]: https://docs.rs/trycmd/
[sad_machine]: https://docs.rs/sad_machine/
[thiserror]: https://docs.rs/thiserror/
[anyhow]: https://docs.rs/anyhow/
[rust-s3]: https://docs.rs/rust-s3/
[`xvc-ecs`]: https://docs.rs/xvc-ecs/
[tokio]: https://tokio.rs

And, biggest thanks to Rust designers, developers and contributors. Although I can't see myself expert to appreciate it all, it's a fabulous language and environment to work with.

## 🚁 Support

- You can use [Discussions](https://github.com/iesahin/xvc/discussions) to ask questions. I'll answer as much as possible. Thank you.
- I don't follow any other sites regularly. You can also reach me at [emre@xvc.dev](mailto:emre@xvc.dev)

## 👐 Contributing

- Star this repo. I feel very happy for five minutes for every star and send my best wishes to you. That's a certain win to spend your two seconds for me. Thanks.
- Use xvc. Tell me how it works for you, read the [documentation](https://docs.xvc.dev), [report bugs](https://github.com/iesahin/xvc/issues), [discuss features](https://github.com/iesahin/xvc/discussions).
- Note that, I don't accept large code PRs. Please open an issue to discuss your idea and write/modify a
  reference page before sending a PR. I'm happy to discuss and help you to implement your idea.

## 📜 License

Xvc is licensed under the [Apache 2.0 License](https://github.com/iesahin/xvc/blob/main/LICENSE).

## 🌦️ Future and Maintenance

This is mostly a one-man project and users may consider the [bus factor](https://en.wikipedia.org/wiki/Bus_factor) before spending time on it.

I'm using Xvc daily and I'm happy with it. Tracking all my files with Git via arbitrary servers and cloud providers is
something I always need. I'm happy to improve and maintain it as long as I use it.

Also, I'm applying my technical/architectural ideas to see their effectiveness
and I have more ideas to implement. I don't expect to be bored from this soon.

I'm in a phase of my life where material success doesn't entice me.
I have a daily routine that I love, which includes spending 1-2 hours to Xvc.
That probably won't change much even if I earn a billion dollars.
I don't want to convert Xvc to a business.
I may create paid apps that use Xvc as a library if I think they will be useful but these will probably be separate projects.
In my opinion, trying to monetize FOSS prematurely deteriorates it more than other factors.

Xvc is like a _runnable CV_ for me. It signals _I built this and I can built similar software for you._ This is
another motivation for me to keep it alive. I hate updating my [vita](https://emresahin.net/cv/), and instead of
updating it, I prefer to show my work.

## ⚠️ Disclaimer

This software is fresh and ambitious. Although I use it and test it close to real-world conditions, it didn't go under
the test of time. **Xvc can eat your files and spit them into the eternal void!** Please take backups.
