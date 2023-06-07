# xvc storage new wasabi

## Purpose

Configure a [Wasabi](https://wasabi.com/) service as an Xvc storage.

## Synopsis

```console
$ xvc storage new wasabi --help
Add a new Wasabi storage

Reads credentials from `WASABI_ACCESS_KEY_ID` and `WASABI_SECRET_ACCESS_KEY` environment variables. Alternatively you can use `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>` and `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>` environment variables if you have multiple storages of this type.

Usage: xvc storage new wasabi [OPTIONS] --name <NAME> --bucket-name <BUCKET_NAME>

Options:
  -n, --name <NAME>
          Name of the storage

          This must be unique among all storages of the project

      --bucket-name <BUCKET_NAME>
          Bucket name

      --endpoint <ENDPOINT>
          Endpoint for the server, complete with the region if there is

          e.g. for eu-central-1 region, use s3.eu-central-1.wasabisys.com as the endpoint.

          [default: s3.wasabisys.com]

      --storage-prefix <STORAGE_PREFIX>
          You can set a directory in the bucket with this prefix

          [default: ]

  -h, --help
          Print help (see a summary with '-h')

```

## Examples


Before calling any commands that use this storage, you must set the following environment variables.

- `WASABI_ACCESS_KEY_ID` or `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>`: The access key of the Wasabi
  account. The second form is used when you have multiple storage accounts with different access keys.
- `WASABI_SECRET_ACCESS_KEY` or `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>`: The secret key of the Wasabi account.
  The second form is used when you have multiple storage accounts with different access keys.

The command works only in Xvc repositories.

```console
$ git init
...
$ xvc init

$ xvc-test-helper create-directory-tree --directories 1 --files 3  --seed 20230211

$ tree dir-0001
dir-0001
├── file-0001.bin
├── file-0002.bin
└── file-0003.bin

1 directory, 3 files

```

Xvc only sends and receives tracked files.

```console
$ xvc file track dir-0001
```

You can define a storage bucket as storage and begin to use it.

```console
$ xvc storage new wasabi --name backup --bucket-name xvc-test --endpoint s3.wasabisys.com --storage-prefix xvc-storage

```

Send files to this storage.

```console
$ xvc file send dir-0001 --to backup

```

You can remove the files you sent from your cache and workspace.

```console
$ xvc file remove --from-cache dir-0001/
[DELETE] [CWD]/.xvc/b3/1bc/b82/80fcea6acf2362a4ec4ef8512fe2f791f412fed1635009293abedcad88/0.bin
[DELETE] [CWD]/.xvc/b3/1bc/b82/80fcea6acf2362a4ec4ef8512fe2f791f412fed1635009293abedcad88
[DELETE] [CWD]/.xvc/b3/1bc/b82
[DELETE] [CWD]/.xvc/b3/1bc
[DELETE] [CWD]/.xvc/b3/863/86d/62e50462e37699d86e9b436526cb3fe40c66e38030e4e25ae4e168193a/0.bin
[DELETE] [CWD]/.xvc/b3/863/86d/62e50462e37699d86e9b436526cb3fe40c66e38030e4e25ae4e168193a
[DELETE] [CWD]/.xvc/b3/863/86d
[DELETE] [CWD]/.xvc/b3/863
[DELETE] [CWD]/.xvc/b3/f60/f11/901bf063f1448d095f336929929e153025a3ec238128a42ff6e5f080ef/0.bin
[DELETE] [CWD]/.xvc/b3/f60/f11/901bf063f1448d095f336929929e153025a3ec238128a42ff6e5f080ef
[DELETE] [CWD]/.xvc/b3/f60/f11
[DELETE] [CWD]/.xvc/b3/f60
[DELETE] [CWD]/.xvc/b3

$ rm -rf dir-0001/
```

Then get back them from storage.

```console
$ xvc file bring --from backup dir-0001

$ tree dir-0001
dir-0001
├── file-0001.bin
├── file-0002.bin
└── file-0003.bin

1 directory, 3 files

```

If you want to remove a file and all of its versions from storage, you can use `xvc file remove` command.

```console
$ xvc file remove --from-storage backup dir-0001/
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
                "git.command": String(
                    "git",
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "core.guid": String(
                    "63eae118747e2f6e",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "pipeline.default": String(
                    "default",
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "core.verbosity": String(
                    "error",
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.track.no_commit": Boolean(
                    false,
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.track.no_parallel": Boolean(
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
                "git.command": String(
                    "git",
                ),
                "git.auto_stage": Boolean(
                    false,
                ),
                "file.track.force": Boolean(
                    false,
                ),
                "pipeline.current_pipeline": String(
                    "default",
                ),
                "file.list.sort": String(
                    "name-desc",
                ),
                "file.carry-in.force": Boolean(
                    false,
                ),
                "file.carry-in.no_parallel": Boolean(
                    false,
                ),
                "file.list.format": String(
                    "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                ),
                "core.verbosity": String(
                    "error",
                ),
                "git.use_git": Boolean(
                    true,
                ),
                "pipeline.default_params_file": String(
                    "params.yaml",
                ),
                "git.auto_commit": Boolean(
                    true,
                ),
                "file.recheck.method": String(
                    "copy",
                ),
                "core.guid": String(
                    "5628e0defc4d2468",
                ),
                "file.track.no_parallel": Boolean(
                    false,
                ),
                "file.list.no_summary": Boolean(
                    false,
                ),
                "file.track.text_or_binary": String(
                    "auto",
                ),
                "file.list.recursive": Boolean(
                    false,
                ),
                "cache.algorithm": String(
                    "blake3",
                ),
                "file.track.no_commit": Boolean(
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
        "git.command": XvcConfigValue {
            source: Project,
            value: String(
                "git",
            ),
        },
        "file.track.force": XvcConfigValue {
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
        "git.auto_stage": XvcConfigValue {
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
        "file.track.no_parallel": XvcConfigValue {
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
        "file.recheck.method": XvcConfigValue {
            source: Project,
            value: String(
                "copy",
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
        "file.track.no_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                false,
            ),
        },
        "core.guid": XvcConfigValue {
            source: Project,
            value: String(
                "5628e0defc4d2468",
            ),
        },
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
        "git.auto_commit": XvcConfigValue {
            source: Project,
            value: Boolean(
                true,
            ),
        },
    },
    init_params: XvcConfigInitParams {
        default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"63eae118747e2f6e/"
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
                "core.verbosity = quiet",
                "core.quiet = false",
            ],
        ),
    },
}
[TRACE][ecs/src/ecs/mod.rs::229] dir: "[CWD]/.xvc/ec"
[TRACE][ecs/src/ecs/mod.rs::239] files: [
    "[CWD]/.xvc/ec/1686123796492693",
    "[CWD]/.xvc/ec/1686123796495433",
    "[CWD]/.xvc/ec/1686123796804571",
    "[CWD]/.xvc/ec/1686123798376786",
    "[CWD]/.xvc/ec/1686123806052848",
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
    subcommand: Remove(
        RemoveCLI {
            from_cache: false,
            from_storage: Some(
                Name(
                    "backup",
                ),
            ),
            all_versions: false,
            only_version: None,
            force: false,
            targets: [
                "dir-0001/",
            ],
        },
    ),
}
[TRACE][file/src/common/mod.rs::136] targets: Some(
    [
        "dir-0001/",
    ],
)
[TRACE][file/src/common/mod.rs::153] t: "dir-0001/"
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.9/src/lib.rs::431] built glob set; 1 literals, 0 basenames, 0 extensions, 1 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][file/src/common/mod.rs::171] paths: HStore {
    map: {
        XvcEntity(
            2,
            3908817039491306506,
        ): XvcPath(
            "dir-0001/file-0002.bin",
        ),
        XvcEntity(
            4,
            3908817039491306506,
        ): XvcPath(
            "dir-0001/file-0001.bin",
        ),
        XvcEntity(
            3,
            3908817039491306506,
        ): XvcPath(
            "dir-0001/file-0003.bin",
        ),
    },
}
[TRACE][file/src/common/mod.rs::141] paths: HStore {
    map: {
        XvcEntity(
            2,
            3908817039491306506,
        ): XvcPath(
            "dir-0001/file-0002.bin",
        ),
        XvcEntity(
            4,
            3908817039491306506,
        ): XvcPath(
            "dir-0001/file-0001.bin",
        ),
        XvcEntity(
            3,
            3908817039491306506,
        ): XvcPath(
            "dir-0001/file-0003.bin",
        ),
    },
}
[TRACE][file/src/remove/mod.rs::153] candidate_paths: [
    (
        XvcEntity(
            2,
            3908817039491306506,
        ),
        XvcCachePath(
            "b3/1bc/b82/80fcea6acf2362a4ec4ef8512fe2f791f412fed1635009293abedcad88/0.bin",
        ),
    ),
    (
        XvcEntity(
            4,
            3908817039491306506,
        ),
        XvcCachePath(
            "b3/863/86d/62e50462e37699d86e9b436526cb3fe40c66e38030e4e25ae4e168193a/0.bin",
        ),
    ),
    (
        XvcEntity(
            3,
            3908817039491306506,
        ),
        XvcCachePath(
            "b3/f60/f11/901bf063f1448d095f336929929e153025a3ec238128a42ff6e5f080ef/0.bin",
        ),
    ),
]
[TRACE][storage/src/storage/wasabi.rs::147] bucket: Bucket {
    name: "xvc-test",
    region: Custom {
        region: "",
        endpoint: "s3.wasabisys.com",
    },
    credentials: Credentials {
        access_key: Some(
            "37EWXPBP8513ZQ94E4MG",
        ),
        secret_key: Some(
            "UJsHEkS83HhwZ27wrWwCHpmih5zweZ2zGMZ19X5d",
        ),
        security_token: None,
        session_token: None,
        expiration: None,
    },
    extra_headers: {},
    extra_query: {},
    request_timeout: Some(
        60s,
    ),
    path_style: false,
    listobjects_v2: true,
}
[TRACE][storage/src/storage/wasabi.rs::353] cache_path: XvcCachePath(
    "b3/1bc/b82/80fcea6acf2362a4ec4ef8512fe2f791f412fed1635009293abedcad88/0.bin",
)
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/reqwest-0.11.13/src/connect.rs::429] starting new connection: https://xvc-test.s3.wasabisys.com/
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.8.5/src/poll.rs::532] registering event source with poller: token=Token(0), interests=READABLE | WRITABLE
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::341] signal: Want
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::355] signal found waiting giver, notifying
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::200] poll_want: taker wants!
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::341] signal: Want
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::341] signal: Want
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::200] poll_want: taker wants!
[TRACE][storage/src/storage/wasabi.rs::353] cache_path: XvcCachePath(
    "b3/863/86d/62e50462e37699d86e9b436526cb3fe40c66e38030e4e25ae4e168193a/0.bin",
)
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.8.5/src/poll.rs::663] deregistering event source from poller
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::330] signal: Closed
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/reqwest-0.11.13/src/connect.rs::429] starting new connection: https://xvc-test.s3.wasabisys.com/
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.8.5/src/poll.rs::532] registering event source with poller: token=Token(16777216), interests=READABLE | WRITABLE
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::341] signal: Want
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::355] signal found waiting giver, notifying
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::200] poll_want: taker wants!
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::341] signal: Want
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::355] signal found waiting giver, notifying
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::341] signal: Want
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::200] poll_want: taker wants!
[TRACE][storage/src/storage/wasabi.rs::353] cache_path: XvcCachePath(
    "b3/f60/f11/901bf063f1448d095f336929929e153025a3ec238128a42ff6e5f080ef/0.bin",
)
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.8.5/src/poll.rs::663] deregistering event source from poller
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::330] signal: Closed
[DEBUG][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/reqwest-0.11.13/src/connect.rs::429] starting new connection: https://xvc-test.s3.wasabisys.com/
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.8.5/src/poll.rs::532] registering event source with poller: token=Token(33554432), interests=READABLE | WRITABLE
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::341] signal: Want
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::355] signal found waiting giver, notifying
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::200] poll_want: taker wants!
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::341] signal: Want
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::200] poll_want: taker wants!
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::341] signal: Want
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.8.5/src/poll.rs::663] deregistering event source from poller
[TRACE][/Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/want-0.3.0/src/lib.rs::330] signal: Closed
[TRACE][lib/src/cli/mod.rs::381] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::384] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc --debug file remove --from-storage backup dir-0001/"
[TRACE][lib/src/cli/mod.rs::433] args: [
    "-C",
    "[CWD]",
    "diff",
    "--name-only",
    "--cached",
]
[TRACE][lib/src/cli/mod.rs::463] git_diff_staged_out: ""
[TRACE][lib/src/cli/mod.rs::433] args: [
    "-C",
    "[CWD]",
    "add",
    "--verbose",
    "[CWD]/.xvc",
    "*.gitignore",
    "*.xvcignore",
]
[TRACE][lib/src/cli/mod.rs::582] git_add_output: ""

```
