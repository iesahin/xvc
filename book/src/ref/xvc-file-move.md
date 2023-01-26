# xvc file move

## Synopsis

```console
$ xvc file copy --help
Copy from source to another location in the workspace

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

Arguments:
  <SOURCE>
          Source file, glob or directory within the workspace.

          If the source ends with a slash, it's considered a directory and all files in that directory are copied.

          If the number of source files is more than one, the destination must be a directory.

  <DESTINATION>
          Location we copy file(s) to within the workspace.

          If the target ends with a slash, it's considered a directory and created if it doesn't exist.

          If the number of source files is more than one, the destination must be a directory.

Options:
      --cache-type <CACHE_TYPE>
          How the targets should be rechecked: One of copy, symlink, hardlink, reflink.

          Note: Reflink uses copy if the underlying file system doesn't support it.

      --force
          Force even if target exists

      --no-recheck
          Do not recheck the destination files This is useful when you want to copy only records, without updating the workspace

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

## Examples

This command is used to move a set of files to another location in the workspace.

By default, it doesn't update the recheck method (cache type) of the targets.
It rechecks them to the destination with the same method.

`xvc file move` works only with the tracked files.

```console
$ git init
...
$ xvc init

$ xvc file track data.txt

$ ls -l
total[..]
-rw-rw-rw-  [..] data.txt

```

Once you add the file to the cache, you can move the file to another location.

```console
$ xvc file move data.txt data2.txt

$ ls
data2.txt

```

Xvc can change the destination file's recheck method.

```console
$ xvc file move data2.txt data3.txt --as symlink

$ ls -l
total[..]
lrwxr-xr-x  1 [..] data3.txt -> [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt

```

You can move files without them being in the workspace if they are in the cache.

```console
$ rm -f data3.txt

$ xvc file move data3.txt data4.txt
[DEBUG][logging/src/lib.rs::233] Terminal logger enabled with level: Trace
[DEBUG][logging/src/lib.rs::236] File logger enabled with level: Trace to "/var/folders/tk/3vn311ps4kqdhgykj3jg_p8r0000gn/T//xvc.log"
[TRACE][core/src/types/xvcroot.rs::247] "."
[DEBUG][core/src/types/xvcroot.rs::253] XVC DIR: "[CWD]"
[DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][config/src/lib.rs::534] cli_config: [
    "core.verbosity = debug",
    "core.quiet = false",
]
[TRACE][config/src/lib.rs::538] map: {
    "core.quiet": Boolean(
        false,
    ),
    "core.verbosity": String(
        "debug",
    ),
}
[TRACE][config/src/lib.rs::541] conf: XvcConfig {
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
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "cache.type": String(
                    "copy",
                ),
                "file.track.no_parallel": Boolean(
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
                "file.list.no_summary": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "core.guid": String(
                    "f905412297114d12",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
            },
        },
        XvcConfigMap {
            source: Project,
            map: {
                "pipeline.default": String(
                    "default",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "git.command": String(
                    "git",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.list.format": String(
                    "{{aft}}{{rct}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "cache.type": String(
                    "copy",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "core.guid": String(
                    "09a32cd9daefa343",
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
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "09a32cd9daefa343",
            ),
        },
        "file.list.format": XvcConfigValue {
            source: Project,
            value: String(
                "{{aft}}{{rct}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
            ),
        },
        "file.list.recursive": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "cache.type": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
            ),
        },
        "file.carry-in.force": XvcConfigValue {
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
        "core.verbosity": XvcConfigValue {
            source: CommandLine,
            value: String(
                "debug",
            ),
        },
        "cache.algorithm": XvcConfigValue {
            source: Project,
            value: String(
                "blake3",
            ),
        },
        "file.track.force": XvcConfigValue {
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
        "file.carry-in.no_parallel": XvcConfigValue {
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
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
            ),
        },
        "file.list.no_summary": XvcConfigValue {
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
guid = /"f905412297114d12/"
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
# The cache type for XVC. It may take copy, hardlink, symlink, reflink as values.
# The default is copy to make sure the options is portable.
# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# Note that hardlink and symlink are read-only as they link the files in cache.
type = /"copy/"
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
# - {rct}:  recorded cache type. Whether the entry is linked to the workspace
#   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# - {rsz}:  recorded size. The size of the cached content in bytes. It uses
#   MB, GB and TB to represent sizes larged than 1MB.
# - {rts}:  recorded timestamp. The timestamp of the cached content.
#
# There are no escape sequences in the format string.
# If you want to add a tab, type it to the string.
# If you want to add a literal double curly brace, open an issue.
format = /"{{aft}}{{rct}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"

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
[TRACE][ecs/src/ecs/mod.rs::206] dir: "[CWD]/.xvc/ec"
[TRACE][ecs/src/ecs/mod.rs::216] files: [
    "[CWD]/.xvc/ec/1674722697628320",
    "[CWD]/.xvc/ec/1674722697631773",
    "[CWD]/.xvc/ec/1674722698162630",
    "[CWD]/.xvc/ec/1674722698323115",
    "[CWD]/.xvc/ec/1674722698420633",
]
[TRACE][file/src/lib.rs::148] opts: XvcFileCLI {
    verbosity: 0,
    quiet: false,
    workdir: ".",
    config: None,
    no_system_config: false,
    no_user_config: false,
    no_project_config: false,
    no_local_config: false,
    no_env_config: false,
    subcommand: Move(
        MoveCLI {
            cache_type: None,
            no_recheck: false,
            source: "data3.txt",
            destination: "data4.txt",
        },
    ),
}
[TRACE][file/src/common/mod.rs::145] targets: Some(
    [
        "data3.txt",
    ],
)
[TRACE][file/src/common/mod.rs::162] t: "data3.txt"
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 1 literals, 0 basenames, 0 extensions, 1 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs::180] paths: HStore {
    map: {
        XvcEntity(
            2,
            12775947774250101957,
        ): XvcPath(
            "data3.txt",
        ),
    },
}
[TRACE][file/src/common/mod.rs::150] paths: HStore {
    map: {
        XvcEntity(
            2,
            12775947774250101957,
        ): XvcPath(
            "data3.txt",
        ),
    },
}
[TRACE][file/src/common/compare.rs::50] pmm: {
    XvcPath(
        "data3.txt",
    ): XvcMetadata {
        file_type: Missing,
        size: None,
        modified: None,
    },
}
[TRACE][ecs/src/ecs/hstore.rs::105] value: XvcPath(
    "data3.txt",
)
[TRACE][ecs/src/ecs/hstore.rs::110] key: XvcEntity(
    2,
    12775947774250101957,
)
[TRACE][file/src/common/compare.rs::159] xvc_path_diff: Identical
[TRACE][core/src/types/xvcpath.rs::81] abs_path: "[CWD]/data4.txt"
[TRACE][core/src/types/xvcpath.rs::82] current_dir: AbsolutePath(
    "[CWD]",
)
[TRACE][core/src/types/xvcpath.rs::83] xvc_root.absolute_path(): AbsolutePath(
    "[CWD]",
)
[TRACE][file/src/mv/mod.rs::215] source_dest_store: HStore {
    map: {
        XvcEntity(
            2,
            12775947774250101957,
        ): XvcPath(
            "data4.txt",
        ),
    },
}
[TRACE][file/src/mv/mod.rs::233] source_cache_type: Symlink
[TRACE][file/src/mv/mod.rs::234] dest_cache_type: Symlink
[TRACE][file/src/mv/mod.rs::263] source_path: AbsolutePath(
    "[CWD]/data3.txt",
)
[TRACE][file/src/mv/mod.rs::268] recheck_entities: [
    XvcEntity(
        2,
        12775947774250101957,
    ),
]
[TRACE][file/src/mv/mod.rs::275] recheck_entities: [
    XvcEntity(
        2,
        12775947774250101957,
    ),
]
[TRACE][file/src/copy/mod.rs::285] destination_entities: [
    XvcEntity(
        2,
        12775947774250101957,
    ),
]
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 1 regexes
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[TRACE][file/src/common/mod.rs::312] path: AbsolutePath(
    "[CWD]/data4.txt",
)
[TRACE][file/src/common/mod.rs::313] cache_type: Symlink
[TRACE][file/src/common/mod.rs::357] "Before return": "Before return"
[INFO] [SYMLINK] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt -> [CWD]/data4.txt
[TRACE][file/src/common/gitignore.rs::119] dir_map: {}
[DEBUG] Writing directories to .gitignore
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/*", re: "(?-u)^(?:/|/.*/)//.xvc/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), ZeroOrMore]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 1 regexes
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/store/**", re: "(?-u)^(?:/|/.*/)//.xvc/store/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('s'), Literal('t'), Literal('o'), Literal('r'), Literal('e'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::426] glob converted to regex: Glob { glob: "/**/.xvc/ec/**", re: "(?-u)^(?:/|/.*/)//.xvc/ec/.*$", opts: GlobOptions { case_insensitive: false, literal_separator: false, backslash_escape: true }, tokens: Tokens([RecursiveZeroOrMore, Literal('.'), Literal('x'), Literal('v'), Literal('c'), Literal('/'), Literal('e'), Literal('c'), RecursiveSuffix]) }
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 0 literals, 0 basenames, 0 extensions, 0 prefixes, 0 suffixes, 1 required extensions, 2 regexes
[TRACE][file/src/common/gitignore.rs::188] file_map: {
    "[CWD]/data4.txt": "[CWD]/.gitignore",
}
[TRACE][lib/src/cli/mod.rs::312] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::315] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc --debug -vvvv file move data3.txt data4.txt"
[DEBUG] Writing files to .gitignore
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
    "Xvc auto-commit after /'/Users/iex/github.com/iesahin/xvc/target/debug/xvc --debug -vvvv file move data3.txt data4.txt/'",
]
[DEBUG][lib/src/cli/mod.rs::535] Committing .xvc/ to git: [main 2d3565f] Xvc auto-commit after '/Users/iex/github.com/iesahin/xvc/target/debug/xvc --debug -vvvv file move data3.txt data4.txt'
 3 files changed, 4 insertions(+)
 create mode 100644 .xvc/ec/1674722698537265
 create mode 100644 .xvc/store/xvc-path-store/1674722698521593.json


$ ls -l
total 0
lrwxr-xr-x  1 iex  staff  180 Jan 26 11:44 data4.txt -> [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt

```

You can use glob patterns to move multiple files.
In this case, the destination must be a directory.

```console
$ xvc file copy data4.txt data5.txt

$ xvc file move d*.txt another-set/ --as hardlink

$ xvc file list another-set/
FH          [..] c85f3e81 c85f3e81 another-set/data5.txt
FH          [..] c85f3e81 c85f3e81 another-set/data4.txt
Total #: 2 Workspace Size:          38 Cached Size:          19


```

You can also skip rechecking.
In this case, Xvc won't create any copies in the workspace, and you don't need them to be available in the cache.
They will be listed with `xvc file list` command.

```console
$ xvc file move another-set/data5.txt data6.txt --no-recheck

$ xvc file list
XH                                 c85f3e81          data6.txt
FH          19 [..] c85f3e81 c85f3e81 another-set/data4.txt
DX          96 [..]                   another-set
FX         130 [..]          ac46bf74 .xvcignore
FX         534 [..]          [..] .gitignore
Total #: 5 Workspace Size:         779 Cached Size:          19


```

Later, you can recheck them in the workspace.

```console
$ xvc file recheck data6.txt

$ ls -l data6.txt
-rw-rw-rw-  [..] data6.txt

```
