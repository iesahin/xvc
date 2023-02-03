# xvc file untrack

## Synopsis

```console
$ xvc file untrack --help
Untrack (delete) files from Xvc and possibly storages

Usage: xvc file untrack [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...  Files/directories to untrack

Options:
      --restore-versions <RESTORE_VERSIONS>
          Restore all versions to a directory before deleting the cache files
      --delete-from-storages <DELETE_FROM_STORAGES>
          Delete all files also from given storages
  -h, --help
          Print help

```


## Examples

This command removes a file from Xvc tracking and optionally deletes it from the local filesystem, cache, and the storages.

It only works if the file is tracked by Xvc.

```console
$ git init
...

$ xvc init

$ xvc file track 'd*.txt'

$ xvc file list
FC          19 [..] c85f3e81 c85f3e81 data.txt
FX         130 [..]          [..] .xvcignore
FX         191 [..]          [..] .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


```

Without any options, it removes the file from Xvc tracking and the cache.

```admonition warning

`xvc file untrack` doesn't modify the `.gitignore` files to remove the previously tracked files. You must do it manually if you want to track the file with Git.

```

```console
$ xvc file untrack data.txt

$ git status
On branch main
nothing to commit, working tree clean

```

If you have [rechecked](/concepts/recheck.md) the file as symlink or reflink, it will be copied to the workspace.

```console
$ xvc file track data.txt --as symlink

$ ls -l
total 0
lrwxr-xr-x  1 iex  staff  183 Feb  3 13:44 data.txt -> [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt

$ xvc file untrack data.txt
[DEBUG][logging/src/lib.rs::233] Terminal logger enabled with level: Trace
[DEBUG][logging/src/lib.rs::236] File logger enabled with level: Trace to "/var/folders/tk/3vn311ps4kqdhgykj3jg_p8r0000gn/T//xvc.log"
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
                "file.list.sort": String(
                    "name-desc",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "git.use_git": Boolean(
                    true,
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
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.guid": String(
                    "50e43df9ab126c8e",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.force": Boolean(
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
                    "802aaf026834b78e",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
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
        "file.track.text_or_binary": XvcConfigValue {
            source: Project,
            value: String(
                "auto",
            ),
        },
        "file.carry-in.force": XvcConfigValue {
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
        "file.track.no_commit": XvcConfigValue {
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
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "802aaf026834b78e",
            ),
        },
        "pipeline.default_params_file": XvcConfigValue {
            source: Project,
            value: String(
                "params.yaml",
            ),
        },
        "pipeline.current_pipeline": XvcConfigValue {
            source: Project,
            value: String(
                "default",
            ),
        },
        "file.list.no_summary": XvcConfigValue {
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
        "file.recheck.method": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
            ),
        },
        "pipeline.default": XvcConfigValue {
            source: Project,
            value: String(
                "default",
            ),
        },
        "core.quiet": XvcConfigValue {
            source: CommandLine,
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
        "file.list.sort": XvcConfigValue {
            source: Project,
            value: String(
                "name-desc",
            ),
        },
        "file.list.recursive": XvcConfigValue {
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
guid = /"50e43df9ab126c8e/"
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
    "[CWD]/.xvc/ec/1675421046007201",
    "[CWD]/.xvc/ec/1675421046011023",
    "[CWD]/.xvc/ec/1675421046376459",
    "[CWD]/.xvc/ec/1675421047128174",
]
[TRACE][file/src/lib.rs::152] opts: XvcFileCLI {
    verbosity: 0,
    quiet: false,
    workdir: ".",
    config: None,
    no_system_config: false,
    no_user_config: false,
    no_project_config: false,
    no_local_config: false,
    no_env_config: false,
    subcommand: Untrack(
        UntrackCLI {
            restore_versions: None,
            delete_from_storages: None,
            targets: [
                "data.txt",
            ],
        },
    ),
}
[TRACE][file/src/common/mod.rs::145] targets: Some(
    [
        "data.txt",
    ],
)
[TRACE][file/src/common/mod.rs::162] t: "data.txt"
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 1 literals, 0 basenames, 0 extensions, 1 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs::180] paths: HStore {
    map: {
        XvcEntity(
            3,
            382180981756757293,
        ): XvcPath(
            "data.txt",
        ),
    },
}
[TRACE][file/src/common/mod.rs::150] paths: HStore {
    map: {
        XvcEntity(
            3,
            382180981756757293,
        ): XvcPath(
            "data.txt",
        ),
    },
}
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 1 regexes
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[TRACE][file/src/common/mod.rs::320] path: AbsolutePath(
    "[CWD]/data.txt",
)
[TRACE][file/src/common/mod.rs::321] recheck_method: Copy
[TRACE][file/src/common/mod.rs::324] "Before copy": "Before copy"
[TRACE][file/src/common/mod.rs::328] &perm: Permissions(
    FilePermissions {
        mode: 33060,
    },
)
[TRACE][file/src/common/mod.rs::330] &perm: Permissions(
    FilePermissions {
        mode: 33206,
    },
)
[INFO] [COPY] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt -> [CWD]/data.txt
[TRACE][file/src/common/mod.rs::365] "Before return": "Before return"
[DEBUG] Writing directories to .gitignore
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 1 regexes
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[DEBUG] Writing files to .gitignore
[TRACE][lib/src/cli/mod.rs::312] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::315] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv --debug file untrack data.txt"
[DEBUG][lib/src/cli/mod.rs::510] Using Git: /opt/homebrew/bin/git
[TRACE][lib/src/cli/mod.rs::417] args: [
    "-C",
    "[CWD]",
    "diff",
    "--name-only",
    "--cached",
]
[TRACE][lib/src/cli/mod.rs::443] git_diff_staged_out: ""
[TRACE][lib/src/cli/mod.rs::417] args: [
    "-C",
    "[CWD]",
    "add",
    "[CWD]/.xvc",
    "*.gitignore",
    "*.xvcignore",
]
[DEBUG][lib/src/cli/mod.rs::525] Adding .xvc/ to git:
[TRACE][lib/src/cli/mod.rs::417] args: [
    "-C",
    "[CWD]",
    "commit",
    "-m",
    "Xvc auto-commit after /'/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv --debug file untrack data.txt/'",
]
[DEBUG][lib/src/cli/mod.rs::535] Committing .xvc/ to git: [main be8b41e] Xvc auto-commit after '/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv --debug file untrack data.txt'
 5 files changed, 5 insertions(+)
 create mode 100644 .xvc/store/content-digest-store/1675421047238042.json
 create mode 100644 .xvc/store/file-text-or-binary-store/1675421047238225.json
 create mode 100644 .xvc/store/recheck-method-store/1675421047238374.json
 create mode 100644 .xvc/store/xvc-metadata-store/1675421047238536.json
 create mode 100644 .xvc/store/xvc-path-store/1675421047238647.json


$ ls -l
total 8
-rw-rw-rw-  1 iex  staff  19 Jan 31 11:00 data.txt

```

If there are multiple versions of the file, it removes them all and restores the latest version.

If you want to restore all versions of the file, you can specify a directory to restore them.

```console
$ xvc file track data.txt

$ perl -pi -e 's/a/e/g' data.txt

$ xvc file carry-in data.txt

$ xvc file untrack data.txt --restore-versions data-versions/
[COPY] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt -> [CWD]/data-versions/data-b3-c85-f3e-8108.txt
[COPY] [CWD]/.xvc/b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367/0.txt -> [CWD]/data-versions/data-b3-660-2cf-f6a4.txt

$ ls -l data-versions/
total 16
-r--r--r--  1 iex  staff  19 Feb  3 13:44 data-b3-660-2cf-f6a4.txt
-r--r--r--  1 iex  staff  19 Jan 31 11:00 data-b3-c85-f3e-8108.txt

```

With `--delete-from-storages`, this command removes the cached files from (remote) storages.

```console
$ xvc file track data.txt

$ xvc storage new local --name local-storage --path '../local-storage'

$ xvc file send data.txt --to local-storage

$ xvc file untrack data.txt --delete-from-storages local-storage
```

If multiple paths are pointing to the same cache file (with deduplication), the cache file will not be
deleted. In this case, `untrack` reports other paths pointing to the same cache file. You must untrack all of them to
delete the cache file.

```console
$ xvc file track data.txt

$ xvc file copy data.txt data2.txt --as symlink

$ xvc file untrack data.txt
Not deleting b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367/0.txt (for data.txt) because it's also used by data2.txt

$ ls -lR .xvc/b3/
total 0
drwxr-xr-x  3 iex  staff  96 Feb  3 13:44 660
drwxr-xr-x  3 iex  staff  96 Feb  3 13:44 c85

.xvc/b3//660:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  3 13:44 2cf

.xvc/b3//660/2cf:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  3 13:44 f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367

.xvc/b3//660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367:
total 8
-r--r--r--  1 iex  staff  19 Feb  3 13:44 0.txt

.xvc/b3//c85:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  3 13:44 f3e

.xvc/b3//c85/f3e:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  3 13:44 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496

.xvc/b3//c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496:
total 8
-r--r--r--  1 iex  staff  19 Jan 31 11:00 0.txt

$ xvc file untrack data2.txt

$ ls -lR .xvc/b3/
total 0
drwxr-xr-x  3 iex  staff  96 Feb  3 13:44 660
drwxr-xr-x  3 iex  staff  96 Feb  3 13:44 c85

.xvc/b3//660:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  3 13:44 2cf

.xvc/b3//660/2cf:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  3 13:44 f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367

.xvc/b3//660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367:
total 8
-r--r--r--  1 iex  staff  19 Feb  3 13:44 0.txt

.xvc/b3//c85:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  3 13:44 f3e

.xvc/b3//c85/f3e:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  3 13:44 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496

.xvc/b3//c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496:
total 8
-r--r--r--  1 iex  staff  19 Jan 31 11:00 0.txt

```
