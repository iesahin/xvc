# xvc file remove

## Synopsis

```console
$ xvc file remove --help
Remove files from Xvc and possibly storages

Usage: xvc file remove [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to remove

Options:
      --from-cache
          Remove files from cache

      --from-storage <FROM_STORAGE>
          Remove files from storage

      --all-versions
          Remove all versions of the file

      --only-version <ONLY_VERSION>
          Remove only the specified version of the file

          Versions are specified with the content hash 123-456-789abcd. Dashes are optional. Prefix must be unique. If the prefix is not unique, the command will fail.

      --force
          Remove the targets even if they are used by other targets (via deduplication)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```


## Examples

This command deletes files from the Xvc cache or storage. It doesn't remove the file from Xvc tracking.

```admonition tip
If you want to remove a workspace file or link, you can use usual `rm` command. If the file is tracked and carried in to the cache, you can always [recheck](xvc-file-recheck.md) it.
```

This command only works if the file is tracked by Xvc.

```console
$ git init
...

$ xvc init

$ xvc file track 'd*.txt'

$ xvc file list
FC        [..] c85f3e81 c85f3e81 data.txt
FX        [..]          ac46bf74 .xvcignore
FX        [..] .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


$ tree .xvc/b3/
.xvc/b3/
└── c85
    └── f3e
        └── 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
            └── 0.txt

4 directories, 1 file

```

If you don't specify either `--from-cache` or `--from-storage`, this command does nothing.

```console
$ xvc file remove data.txt
error: the following required arguments were not provided:
  --from-cache
  --from-storage <FROM_STORAGE>

Usage: xvc file remove --from-cache --from-storage <FROM_STORAGE> <TARGETS>...

For more information, try '--help'.

```


You can remove the file from the cache. The file is still tracked by Xvc and available in the workspace.

```console
$ xvc file remove --from-cache data.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
[DELETE] [CWD]/.xvc/b3/c85/f3e
[DELETE] [CWD]/.xvc/b3/c85
[DELETE] [CWD]/.xvc/b3

$ ls -l
total 8
-rw-rw-rw-  1 iex  staff  19 Jan 31 11:00 data.txt

$ tree .xvc/b3/
.xvc/b3/  [error opening dir]

0 directories, 0 files

```

You can carry the missing file from the workspace to the cache. Use `--force` to overwrite the cache as carry-in
doesn't overwrite the cache by default.

```console
$ xvc file carry-in --force data.txt

$ xvc file list
FC         [..] c85f3e81 c85f3e81 data.txt
FX         [..]          ac46bf74 .xvcignore
FX         [..]          [..] .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


$ tree .xvc/b3/
.xvc/b3/
└── c85
    └── f3e
        └── 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
            └── 0.txt

4 directories, 1 file

```

You can specify a version of a file to delete from the cache. The versions can
be specified like `123-456-789abcd`. Dashes are optional. The prefix must be unique.

```console
$ perl -pi -e 's/a/e/g' data.txt

$ xvc file carry-in data.txt

$ tree .xvc/b3/
.xvc/b3/
├── 660
│   └── 2cf
│       └── f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
│           └── 0.txt
└── c85
    └── f3e
        └── 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
            └── 0.txt

7 directories, 2 files

$ xvc file list
FC         [..] 6602cff6 6602cff6 data.txt
FX         [..]          ac46bf74 .xvcignore
FX         [..]          [..] .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


$ xvc file remove --from-cache --only-version c85-f3e data.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
[DELETE] [CWD]/.xvc/b3/c85/f3e
[DELETE] [CWD]/.xvc/b3/c85

$ tree .xvc/b3/
.xvc/b3/
└── 660
    └── 2cf
        └── f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
            └── 0.txt

4 directories, 1 file

```

You can also remove all versions of a file from the cache.

```console
$ xvc-test-helper generate-filled-file --fill 0 --filename data.txt

$ xvc file carry-in data.txt

$ rm data.txt

$ xvc-test-helper generate-filled-file --fill 1 --filename data.txt
error: unrecognized subcommand 'generate-fill-file'

  note: subcommand 'generate-random-text-file', 'generate-random-file', 'generate-filled-file' exist
  note: to pass 'generate-fill-file' as a value, use 'xvc-test-helper -- generate-fill-file'

Usage: xvc-test-helper <COMMAND>

For more information, try '--help'.

$ xvc file carry-in data.txt
thread '<unnamed>' panicked at 'The number of xvc paths and the number of cache paths should be the same.', file/src/carry_in/mod.rs:231:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: xvc_file::carry_in::carry_in
             at /Users/iex/github.com/iesahin/xvc/file/src/carry_in/mod.rs:231:5
   3: xvc_file::carry_in::cmd_carry_in
             at /Users/iex/github.com/iesahin/xvc/file/src/carry_in/mod.rs:201:5
   4: xvc_file::run
             at /Users/iex/github.com/iesahin/xvc/file/src/lib.rs:164:45
   5: xvc::cli::dispatch::{{closure}}::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:343:24
   6: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:438:31
   7: core::ops::function::FnOnce::call_once{{vtable.shim}}
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
   8: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/alloc/src/boxed.rs:1940:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', lib/src/cli/mod.rs:403:37
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: core::result::unwrap_failed
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/result.rs:1785:5
   3: core::result::Result<T,E>::unwrap
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/result.rs:1107:23
   4: xvc::cli::dispatch::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:403:15
   5: crossbeam_utils::thread::scope::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:65
   6: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panic/unwind_safe.rs:271:9
   7: std::panicking::try::do_call
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:492:40
   8: ___rust_try
   9: std::panicking::try
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:456:19
  10: std::panic::catch_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panic.rs:137:14
  11: crossbeam_utils::thread::scope
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:18
  12: xvc::cli::dispatch
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:239:5
  13: xvc::main
             at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:12:5
  14: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

$ tree .xvc/b3/
.xvc/b3/
├── 660
│   └── 2cf
│       └── f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
│           └── 0.txt
└── d6f
    └── d9d
        └── e5bccf223f523b316c9cd1cf9a9d87ea42473d68e011dad13f09bf8917
            └── 0.txt

7 directories, 2 files

$ xvc -vvvv file remove --from-cache --all-versions data.txt
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
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "core.guid": String(
                    "c56351256fe33e69",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "git.auto_stage": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "git.command": String(
                    "git",
                ),
                "core.guid": String(
                    "68b8b29bc483cc2d",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.track.force": Boolean(
                    false,
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
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.recheck.method": String(
                    "copy",
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
        "file.track.text_or_binary": XvcConfigValue {
            source: Project,
            value: String(
                "auto",
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
        "git.use_git": XvcConfigValue {
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
        "file.track.no_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "68b8b29bc483cc2d",
            ),
        },
        "file.list.recursive": XvcConfigValue {
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
        "pipeline.default": XvcConfigValue {
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
        "git.auto_stage": XvcConfigValue {
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
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
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
        "pipeline.current_pipeline": XvcConfigValue {
            source: Project,
            value: String(
                "default",
            ),
        },
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"c56351256fe33e69/"
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
    "[CWD]/.xvc/ec/1676107963829143",
    "[CWD]/.xvc/ec/1676107963832434",
    "[CWD]/.xvc/ec/1676107964374969",
]
[TRACE][file/src/lib.rs::156] opts: XvcFileCLI {
    verbosity: 0,
    quiet: false,
    workdir: ".",
    config: None,
    no_system_config: false,
    no_user_config: false,
    no_project_config: false,
    no_local_config: false,
    no_env_config: false,
    subcommand: Remove(
        RemoveCLI {
            from_cache: true,
            from_storage: None,
            all_versions: true,
            only_version: None,
            force: false,
            targets: [
                "data.txt",
            ],
        },
    ),
}
[TRACE][file/src/common/mod.rs::146] targets: Some(
    [
        "data.txt",
    ],
)
[TRACE][file/src/common/mod.rs::163] t: "data.txt"
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 1 literals, 0 basenames, 0 extensions, 1 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs::181] paths: HStore {
    map: {
        XvcEntity(
            2,
            17173162408444757106,
        ): XvcPath(
            "data.txt",
        ),
    },
}
[TRACE][file/src/common/mod.rs::151] paths: HStore {
    map: {
        XvcEntity(
            2,
            17173162408444757106,
        ): XvcPath(
            "data.txt",
        ),
    },
}
[TRACE][file/src/remove/mod.rs::152] candidate_paths: [
    (
        XvcEntity(
            2,
            17173162408444757106,
        ),
        XvcCachePath(
            "b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt",
        ),
    ),
    (
        XvcEntity(
            2,
            17173162408444757106,
        ),
        XvcCachePath(
            "b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367/0.txt",
        ),
    ),
    (
        XvcEntity(
            2,
            17173162408444757106,
        ),
        XvcCachePath(
            "b3/d6f/d9d/e5bccf223f523b316c9cd1cf9a9d87ea42473d68e011dad13f09bf8917/0.txt",
        ),
    ),
]
[TRACE][file/src/remove/mod.rs::205] xcp: XvcCachePath(
    "b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt",
)
[TRACE][core/src/types/xvcpath.rs::267] abs_cp: AbsolutePath(
    "[CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt",
)
[TRACE][core/src/types/xvcpath.rs::285] rel_path: "b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt"
[TRACE][core/src/types/xvcpath.rs::288] parent_abs_cp: "[CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496"
thread '<unnamed>' panicked at 'IoError { source: Os { code: 2, kind: NotFound, message: "No such file or directory" } }', file/src/remove/mod.rs:206:13
stack backtrace:
thread '<unnamed>' panicked at '[PANIC] IoError { source: Os { code: 2, kind: NotFound, message: "No such file or directory" } }, [file/src/remove/mod.rs::206]', lib/src/cli/mod.rs:300:52
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: xvc_file::remove::cmd_remove::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/file/src/remove/mod.rs:206:13
   3: <core::slice::iter::Iter<T> as core::iter::traits::iterator::Iterator>::for_each
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/slice/iter/macros.rs:211:21
   4: xvc_file::remove::cmd_remove
             at /Users/iex/github.com/iesahin/xvc/file/src/remove/mod.rs:204:9
   5: xvc_file::run
             at /Users/iex/github.com/iesahin/xvc/file/src/lib.rs:204:44
   6: xvc::cli::dispatch::{{closure}}::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:343:24
   7: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:438:31
   8: core::ops::function::FnOnce::call_once{{vtable.shim}}
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
   9: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/alloc/src/boxed.rs:1940:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: xvc::cli::dispatch::{{closure}}::{{closure}}
thread '             at main/Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs' panicked at ':called `Result::unwrap()` on an `Err` value: Any { .. }300', :lib/src/cli/mod.rs:403:37
52
   3: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:438:31
   4: core::ops::function::FnOnce::call_once{{vtable.shim}}
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
   5: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/alloc/src/boxed.rs:1940:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: core::result::unwrap_failed
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/result.rs:1785:5
   3: core::result::Result<T,E>::unwrap
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/result.rs:1107:23
   4: xvc::cli::dispatch::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:403:15
   5: crossbeam_utils::thread::scope::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:65
   6: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panic/unwind_safe.rs:271:9
   7: std::panicking::try::do_call
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:492:40
   8: ___rust_try
   9: std::panicking::try
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:456:19
  10: std::panic::catch_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panic.rs:137:14
  11: crossbeam_utils::thread::scope
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:18
  12: xvc::cli::dispatch
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:239:5
  13: xvc::main
             at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:12:5
  14: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

$ tree .xvc/b3/
.xvc/b3/
├── 660
│   └── 2cf
│       └── f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
│           └── 0.txt
└── d6f
    └── d9d
        └── e5bccf223f523b316c9cd1cf9a9d87ea42473d68e011dad13f09bf8917
            └── 0.txt

7 directories, 2 files

```

You can use this command to remove cached files from (remote) storages as well.

```console
$ xvc storage new local --name local-storage --path '../local-storage'
$ xvc file send data.txt --to local-storage
$ ls -l ../local-storage/*/b3/*/*/*/0.*
ls: ../local-storage/*/b3/*/*/*/0.*: No such file or directory

$ xvc file remove data.txt --from-storage local-storage
thread '<unnamed>' panicked at 'not yet implemented', storage/src/storage/local.rs:208:9
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: core::panicking::panic
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:48:5
   3: <xvc_storage::storage::local::XvcLocalStorage as xvc_storage::storage::XvcStorageOperations>::delete
             at /Users/iex/github.com/iesahin/xvc/storage/src/storage/local.rs:208:9
   4: <xvc_storage::storage::XvcStorage as xvc_storage::storage::XvcStorageOperations>::delete
             at /Users/iex/github.com/iesahin/xvc/storage/src/storage/mod.rs:300:38
   5: xvc_file::remove::cmd_remove
             at /Users/iex/github.com/iesahin/xvc/file/src/remove/mod.rs:212:9
   6: xvc_file::run
             at /Users/iex/github.com/iesahin/xvc/file/src/lib.rs:204:44
   7: xvc::cli::dispatch::{{closure}}::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:343:24
   8: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:438:31
   9: core::ops::function::FnOnce::call_once{{vtable.shim}}
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
  10: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/alloc/src/boxed.rs:1940:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', lib/src/cli/mod.rs:403:37
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: core::result::unwrap_failed
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/result.rs:1785:5
   3: core::result::Result<T,E>::unwrap
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/result.rs:1107:23
   4: xvc::cli::dispatch::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:403:15
   5: crossbeam_utils::thread::scope::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:65
   6: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panic/unwind_safe.rs:271:9
   7: std::panicking::try::do_call
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:492:40
   8: ___rust_try
   9: std::panicking::try
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:456:19
  10: std::panic::catch_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panic.rs:137:14
  11: crossbeam_utils::thread::scope
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:18
  12: xvc::cli::dispatch
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:239:5
  13: xvc::main
             at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:12:5
  14: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

$ ls -lR ../local-storage/*/b3/*/*/*/0.*
ls: ../local-storage/*/b3/*/*/*/0.*: No such file or directory

```


If multiple paths are pointing to the same cache file (deduplication), the cache file will not be deleted.
In this case, `remove` reports other paths pointing to the same cache file. You must `--force` delete the cache file.

```console
$ xvc-test-helper generate-random-file --size 2000 --filename data.txt

$ xvc file carry-in data.txt

$ xvc file copy data.txt data2.txt --as symlink
$ xvc file list
SS         179 2023-02-11 09:32:46 ab07c948          data2.txt
FC        2000 2023-02-11 09:32:46 ab07c948 ab07c948 data.txt
FX         130 2023-02-11 09:32:43          ac46bf74 .xvcignore
FX         276 2023-02-11 09:32:46          c684c873 .gitignore
Total #: 4 Workspace Size:        2585 Cached Size:        2000


$ xvc file remove --from-cache data.txt
Not deleting b3/ab0/7c9/48da686b2eed9aa93eeb295741e7f26365440693335f6ab5dc7c2c17fc/0.txt (for data.txt) because it's also used by data2.txt

$ ls -l .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```