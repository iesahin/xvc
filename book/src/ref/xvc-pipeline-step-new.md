# xvc pipeline step new

## Purpose

Create a new step in the pipeline.

## Synopsis

```console
$ xvc pipeline step new --help
Add a new step

Usage: xvc pipeline step new [OPTIONS] --step-name <STEP_NAME> --command <COMMAND>

Options:
  -s, --step-name <STEP_NAME>  Name of the new step
  -c, --command <COMMAND>      Step command to run
      --when <WHEN>            When to run the command. One of always, never, by_dependencies (default). This is used to freeze or invalidate a step manually
  -h, --help                   Print help

```

## Examples

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

You can create a new step with a name and a command.

```console
$ xvc pipeline step new --step-name hello --command "echo hello"
```

By default a step will run only if its dependencies have changed. (`--when by_dependencies`).

If you want to run the command always, regardless of the changes in dependencies, you can set `--when` to `always`.

```console
$ xvc pipeline step new --step-name world --command "echo world" --when always
```

If you want a step to never run, you can set `--when` to `never`.

```console
$ xvc pipeline step new --step-name never --command "echo never" --when never
```

You can update when the step will run with [`xvc pipeline step update`](/ref/xvc-pipeline-step-update.md).

You can get the list of steps in the pipeline with `export` or `dag`.

```console
$ xvc pipeline export
{
  "name": "default",
  "steps": [
    {
      "command": "echo hello",
      "dependencies": [],
      "invalidate": "ByDependencies",
      "name": "hello",
      "outputs": []
    },
    {
      "command": "echo world",
      "dependencies": [],
      "invalidate": "Always",
      "name": "world",
      "outputs": []
    },
    {
      "command": "echo never",
      "dependencies": [],
      "invalidate": "Never",
      "name": "never",
      "outputs": []
    }
  ],
  "version": 1,
  "workdir": ""
}

$ xvc -vvvv pipeline dag
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
    "core.quiet": Boolean(
        false,
    ),
    "core.verbosity": String(
        "debug",
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
                "file.recheck.method": String(
                    "copy",
                ),
                "core.verbosity": String(
                    "error",
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
                "git.command": String(
                    "git",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "core.guid": String(
                    "273658f1bc753063",
                ),
                "file.track.no_commit": Boolean(
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
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "core.guid": String(
                    "0fc5e43251f9c10a",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "pipeline.default": String(
                    "default",
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
        "git.auto_commit": XvcConfigValue {
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
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
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
        "file.list.no_summary": XvcConfigValue {
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
        "git.use_git": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
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
        "file.track.no_parallel": XvcConfigValue {
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
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
                "0fc5e43251f9c10a",
            ),
        },
        "pipeline.current_pipeline": XvcConfigValue {
            source: Project,
            value: String(
                "default",
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
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"273658f1bc753063/"
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
    "[CWD]/.xvc/ec/1676528692985194",
    "[CWD]/.xvc/ec/1676528692987973",
    "[CWD]/.xvc/ec/1676528693092327",
    "[CWD]/.xvc/ec/1676528693231347",
    "[CWD]/.xvc/ec/1676528693354536",
]
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][core/src/types/xvcpath.rs::83] abs_path: "[CWD]/.gitignore"
[TRACE][core/src/types/xvcpath.rs::84] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::85] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::83] abs_path: "[CWD]/.xvcignore"
[TRACE][core/src/types/xvcpath.rs::84] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::85] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][pipeline/src/pipeline/api/dag.rs::76] pipeline_steps: HStore {
    map: {
        XvcEntity(
            2,
            17491803953838224831,
        ): XvcStep {
            name: "hello",
        },
        XvcEntity(
            3,
            927614548765042231,
        ): XvcStep {
            name: "world",
        },
        XvcEntity(
            4,
            2663543683435889209,
        ): XvcStep {
            name: "never",
        },
    },
}
[TRACE][pipeline/src/pipeline/api/dag.rs::88] dependency_graph: {
    XvcEntity(
        1,
        12543788146615817680,
    ): [
        (
            XvcEntity(
                2,
                17491803953838224831,
            ),
            Outgoing,
        ),
        (
            XvcEntity(
                3,
                927614548765042231,
            ),
            Outgoing,
        ),
        (
            XvcEntity(
                4,
                2663543683435889209,
            ),
            Outgoing,
        ),
    ],
    XvcEntity(
        2,
        17491803953838224831,
    ): [
        (
            XvcEntity(
                1,
                12543788146615817680,
            ),
            Incoming,
        ),
    ],
    XvcEntity(
        3,
        927614548765042231,
    ): [
        (
            XvcEntity(
                1,
                12543788146615817680,
            ),
            Incoming,
        ),
    ],
    XvcEntity(
        4,
        2663543683435889209,
    ): [
        (
            XvcEntity(
                1,
                12543788146615817680,
            ),
            Incoming,
        ),
    ],
}
thread '<unnamed>' panicked at 'no entry found for key', pipeline/src/pipeline/api/dag.rs:91:21
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic_display
   3: core::panicking::panic_str
   4: core::option::expect_failed
   5: core::option::Option<T>::expect
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/option.rs:741:21
   6: <std::collections::hash::map::HashMap<K,V,S> as core::ops::index::Index<&Q>>::index
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/collections/hash/map.rs:1340:9
   7: xvc_pipeline::pipeline::api::dag::cmd_dag::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/api/dag.rs:91:21
   8: xvc_pipeline::pipeline::api::dag::cmd_dag::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/api/dag.rs:123:22
   9: core::iter::adapters::map::map_fold::{{closure}}
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/iter/adapters/map.rs:84:28
  10: core::iter::traits::iterator::Iterator::fold
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/iter/traits/iterator.rs:2414:21
  11: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/iter/adapters/map.rs:124:9
  12: core::iter::traits::iterator::Iterator::for_each
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/iter/traits/iterator.rs:831:9
  13: <hashbrown::map::HashMap<K,V,S,A> as core::iter::traits::collect::Extend<(K,V)>>::extend
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/vendor/hashbrown/src/map.rs:6407:9
  14: <std::collections::hash::map::HashMap<K,V,S> as core::iter::traits::collect::Extend<(K,V)>>::extend
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/collections/hash/map.rs:3037:9
  15: <std::collections::hash::map::HashMap<K,V,S> as core::iter::traits::collect::FromIterator<(K,V)>>::from_iter
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/collections/hash/map.rs:3022:9
  16: <xvc_ecs::ecs::hstore::HStore<T> as core::iter::traits::collect::FromIterator<(xvc_ecs::ecs::XvcEntity,T)>>::from_iter
             at /Users/iex/github.com/iesahin/xvc/ecs/src/ecs/hstore.rs:55:18
  17: core::iter::traits::iterator::Iterator::collect
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/iter/traits/iterator.rs:1836:9
  18: xvc_pipeline::pipeline::api::dag::cmd_dag
             at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/api/dag.rs:121:38
  19: xvc_pipeline::cmd_pipeline
             at /Users/iex/github.com/iesahin/xvc/pipeline/src/lib.rs:419:13
  20: xvc::cli::dispatch::{{closure}}::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:349:24
  21: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:438:31
  22: core::ops::function::FnOnce::call_once{{vtable.shim}}
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/ops/function.rs:507:5
  23: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/alloc/src/boxed.rs:2000:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', lib/src/cli/mod.rs:403:37
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   3: core::result::Result<T,E>::unwrap
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/result.rs:1113:23
   4: xvc::cli::dispatch::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:403:15
   5: crossbeam_utils::thread::scope::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:65
   6: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/panic/unwind_safe.rs:271:9
   7: std::panicking::try::do_call
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panicking.rs:483:40
   8: ___rust_try
   9: std::panicking::try
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panicking.rs:447:19
  10: std::panic::catch_unwind
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panic.rs:137:14
  11: crossbeam_utils::thread::scope
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:18
  12: xvc::cli::dispatch
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:239:5
  13: xvc::main
             at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:12:5
  14: core::ops::function::FnOnce::call_once
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/ops/function.rs:507:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

```
