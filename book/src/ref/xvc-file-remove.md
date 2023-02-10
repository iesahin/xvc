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

You can carry in the missing file from the workspace to the cache.

```console
$ xvc file carry-in --force data.txt

$ xvc file list
FC          19 2023-01-31 08:00:58 c85f3e81 c85f3e81 data.txt
FX         130 2023-02-10 08:17:47          ac46bf74 .xvcignore
FX         191 2023-02-10 08:17:48          f7fc434b .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


$ tree .xvc/
.xvc/
├── config.local.toml
├── config.toml
├── ec
│   ├── 1676017067548911
│   ├── 1676017067552618
│   └── 1676017068076104
└── store
    ├── content-digest-store
    │   └── 1676017068061761.json
    ├── file-text-or-binary-store
    │   └── 1676017068061613.json
    ├── recheck-method-store
    │   └── 1676017068061442.json
    ├── xvc-metadata-store
    │   └── 1676017068061255.json
    ├── xvc-path-store
    │   └── 1676017068061046.json
    └── xvc-pipeline-store
        └── 1676017067551973.json

9 directories, 11 files

```

You can specify a version of a file to delete from the cache. The versions can
be specified like `123-456-789abcd`. Dashes are optional. The prefix must be unique.

```console
$ perl -pi -e 's/a/e/g' data.txt

$ xvc file carry-in data.txt

$ tree .xvc/b3/
.xvc/b3/
└── 660
    └── 2cf
        └── f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
            └── 0.txt

4 directories, 1 file

$ xvc file list
FC          19 2023-02-10 08:17:49 6602cff6 6602cff6 data.txt
FX         130 2023-02-10 08:17:47          ac46bf74 .xvcignore
FX         191 2023-02-10 08:17:48          f7fc434b .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


$ xvc file remove --from-cache --only-version c85-f3e data.txt

$ tree .xvc/b3/
.xvc/b3/
└── 660
    └── 2cf
        └── f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
            └── 0.txt

4 directories, 1 file

```

You can also remove all versions of a file from the cache.

$ xvc file remove --from-cache --all-versions data.txt

$ tree .xvc/b3/

It's possible to filter the cache versions by size or timestamp to remove.

Suppose you have three versions of `data.txt` in the cache. The first version is 19 bytes, the second is 2000 bytes and
the third is 3000 bytes.

$ rm data.txt

$ xvc-test-helper generate-random-file --size 2000 --filename data.txt

$ xvc file carry-in data.txt

$ rm data.txt

$ xvc-test-helper generate-random-file --size 3000 --filename data.txt

$ xvc file carry-in data.txt

$ ls -l .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

You can remove all versions of the file larger than 2000 bytes.

```console
$ xvc file remove --from-cache --larger-than 2000 data.txt
error: unexpected argument '--larger-than' found

  note: to pass '--larger-than' as a value, use '-- --larger-than'

Usage: xvc file remove <--from-cache|--from-storage <FROM_STORAGE>|--all-versions|--only-version <ONLY_VERSION>|--force|TARGETS>

For more information, try '--help'.

$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

You can remove all versions of the file smaller than 500 bytes.

```console
$ xvc file remove --from-cache --smaller-than 500 data.txt
error: unexpected argument '--smaller-than' found

  note: to pass '--smaller-than' as a value, use '-- --smaller-than'

Usage: xvc file remove <--from-cache|--from-storage <FROM_STORAGE>|--all-versions|--only-version <ONLY_VERSION>|--force|TARGETS>

For more information, try '--help'.

$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

You can remove all versions carried in before or after a certain timestamp.

```console
$ xvc-test-helper generate-random-file --size 2000 --filename data.txt

$ touch -t 202201010000 data.txt
$ xvc file carry-in data.txt

$ xvc-test-helper generate-random-file --size 2000 --filename data.txt

$ touch -t 202301010000 data.txt
$ xvc file carry-in data.txt

$ xvc-test-helper generate-random-file --size 2000 --filename data.txt

$ touch -t 202401010000 data.txt
$ xvc file carry-in data.txt

$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

Now remove all versions carried in before 2023-01-01.

```console
$ xvc file remove --from-cache --before 2023-01-01 data.txt
error: unexpected argument '--before' found

  note: argument '--force' exists

Usage: xvc file remove <--from-cache|--from-storage <FROM_STORAGE>|--all-versions|--only-version <ONLY_VERSION>|--force|TARGETS>

For more information, try '--help'.

$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

Remove all versions carried in after 2023-01-02.

```console
$ xvc file remove --from-cache --after 2023-01-02 data.txt
error: unexpected argument '--after' found

  note: to pass '--after' as a value, use '-- --after'

Usage: xvc file remove <--from-cache|--from-storage <FROM_STORAGE>|--all-versions|--only-version <ONLY_VERSION>|--force|TARGETS>

For more information, try '--help'.

$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

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
             at /Users/iex/github.com/iesahin/xvc/file/src/remove/mod.rs:201:9
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
SS         182 2023-02-10 08:17:50 1f83cd30          data2.txt
FC        2000 2023-02-10 08:17:50 1f83cd30 1f83cd30 data.txt
FX         130 2023-02-10 08:17:47          ac46bf74 .xvcignore
FX         276 2023-02-10 08:17:50          23bf2fc5 .gitignore
Total #: 4 Workspace Size:        2588 Cached Size:        2000


$ xvc file remove --from-cache data.txt
Not deleting b3/1f8/3cd/30a644da0e094f30a759185f8632dbebc52d391dc729d071621d895343/0.txt (for data.txt) because it's also used by data2.txt

$ ls -l .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```
