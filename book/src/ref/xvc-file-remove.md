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

          Versions are specified like b3-123-456-789abcd where b3 is the hash algorithm prefix and the rest is a (at least 3 digit) prefix of the content hash. Prefix must be unique. If the prefix is not unique, the command will fail. Dashes are optional.

      --before <BEFORE>
          Remove all versions of the file carried in earlier than the given timestamp. Timestamps are specified like 2023-01-01T12:34:56Z in RFC3339 format

      --after <AFTER>
          Remove all versions of the file carried in after than the given timestamp. Timestamps are specified like 2023-01-01T12:34:56Z in RFC3339 format

      --larger-than <LARGER_THAN>
          Remove all versions of the targets larger than the given size. Size can be specified like 1 KiB, 1 TB or 1.5 MB. See https://docs.rs/parse-size/latest/parse_size/ for more details

      --smaller-than <SMALLER_THAN>
          Remove all versions of the targets smaller than the given size. Size can be specified like 1 KiB, 1 TB or 1.5 MB. See https://docs.rs/parse-size/latest/parse_size/ for more details

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


```

If you don't specify either `--from-cache` or `--from-storage`, this command does nothing.

```console
$ xvc file remove data.txt
[ERROR] File Error: At least one of --from-cache or --from-storage must be specified

```


You can remove the file from the cache and keep the workspace version.

```console
$ xvc file remove --from-cache data.txt

$ ls -l
total 8
-rw-rw-rw-  1 iex  staff  19 Jan 31 11:00 data.txt

```

You can recheck the file from the cache to the workspace.

```console
$ xvc file recheck data.txt

$ xvc file list
FC          19 2023-01-31 08:00:58 c85f3e81 c85f3e81 data.txt
FX         130 2023-02-08 10:32:34          ac46bf74 .xvcignore
FX         191 2023-02-08 10:32:35          3120da77 .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


```

You can remove all versions of a file from the cache.

```console
$ perl -pi -e 's/a/e/g' data.txt

$ xvc file carry-in data.txt

$ xvc file list
FC          19 2023-02-08 10:32:36 6602cff6 6602cff6 data.txt
FX         130 2023-02-08 10:32:34          ac46bf74 .xvcignore
FX         191 2023-02-08 10:32:35          3120da77 .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


$ xvc file remove --from-cache --all-versions data.txt

```

It's possible to filter the cache versions by size or timestamp to remove.

Suppose you have three versions of `data.txt` in the cache. The first version is 19 bytes, the second is 2000 bytes and
the third is 3000 bytes.

```console
$ rm data.txt
$ xvc-test-helper generate-random-file --size 2000 --filename data.txt
error: unexpected argument 'data.txt' found

Usage: xvc-test-helper generate-random-file [OPTIONS] --filename <FILENAME>

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
             at /Users/iex/github.com/iesahin/xvc/file/src/carry_in/mod.rs:202:5
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

$ rm data.txt
rm: data.txt: No such file or directory

$ xvc-test-helper generate-random-file --size 3000 --filename data.txt
error: unexpected argument 'data.txt' found

Usage: xvc-test-helper generate-random-file [OPTIONS] --filename <FILENAME>

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
             at /Users/iex/github.com/iesahin/xvc/file/src/carry_in/mod.rs:202:5
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

$ ls -l .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

You can remove all versions of the file larger than 2000 bytes.

```console
$ xvc file remove --from-cache --larger-than 2000 data.txt
$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

You can remove all versions of the file smaller than 500 bytes.

```console
$ xvc file remove --from-cache --smaller-than 500 data.txt
$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

You can remove all versions carried in before or after a certain timestamp.

```console
$ xvc-test-helper generate-random-file --size 2000 --filename data.txt
error: unexpected argument 'data.txt' found

Usage: xvc-test-helper generate-random-file [OPTIONS] --filename <FILENAME>

For more information, try '--help'.

$ touch -t 202201010000 data.txt
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
             at /Users/iex/github.com/iesahin/xvc/file/src/carry_in/mod.rs:202:5
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

$ xvc-test-helper generate-random-file --size 2000 --filename data.txt
error: unexpected argument 'data.txt' found

Usage: xvc-test-helper generate-random-file [OPTIONS] --filename <FILENAME>

For more information, try '--help'.

$ touch -t 202301010000 data.txt
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
             at /Users/iex/github.com/iesahin/xvc/file/src/carry_in/mod.rs:202:5
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

$ xvc-test-helper generate-random-file --size 2000 --filename data.txt
error: unexpected argument 'data.txt' found

Usage: xvc-test-helper generate-random-file [OPTIONS] --filename <FILENAME>

For more information, try '--help'.

$ touch -t 202401010000 data.txt
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
             at /Users/iex/github.com/iesahin/xvc/file/src/carry_in/mod.rs:202:5
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

$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

Now remove all versions carried in before 2023-01-01.

```console
$ xvc file remove --from-cache --before 2023-01-01 data.txt
$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

Remove all versions carried in after 2023-01-02.

```console
$ xvc file remove --from-cache --after 2023-01-02 data.txt
$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

You can use this command to remove cached files from (remote) storages as well.

```console
$ xvc storage new local --name local-storage --path '../local-storage'

$ xvc file send data.txt --to local-storage
$ ls -l ../local-storage/*/b3/*/*/*/0.*
total 0
drwxr-xr-x  3 iex  staff  96 Feb  8 13:32 f5501eb892725998

../local-storage/f5501eb892725998:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  8 13:32 b3

../local-storage/f5501eb892725998/b3:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  8 13:32 660

../local-storage/f5501eb892725998/b3/660:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  8 13:32 2cf

../local-storage/f5501eb892725998/b3/660/2cf:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  8 13:32 f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367

../local-storage/f5501eb892725998/b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367:
total 8
-r--r--r--  1 iex  staff  19 Feb  8 13:32 0.txt

$ xvc file remove data.txt --from-storage local-storage
$ ls -lR ../local-storage/*/b3/*/*/*/0.*
total 0
drwxr-xr-x  3 iex  staff  96 Feb  8 13:32 f5501eb892725998

../local-storage/f5501eb892725998:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  8 13:32 b3

../local-storage/f5501eb892725998/b3:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  8 13:32 660

../local-storage/f5501eb892725998/b3/660:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  8 13:32 2cf

../local-storage/f5501eb892725998/b3/660/2cf:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  8 13:32 f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367

../local-storage/f5501eb892725998/b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367:
total 8
-r--r--r--  1 iex  staff  19 Feb  8 13:32 0.txt

```


If multiple paths are pointing to the same cache file (deduplication), the cache file will not be deleted.
In this case, `remove` reports other paths pointing to the same cache file. You must `--force` delete the cache file.

```console
$ xvc-test-helper generate-random-file --size 2000 --filename data.txt
error: unexpected argument 'data.txt' found

Usage: xvc-test-helper generate-random-file [OPTIONS] --filename <FILENAME>

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
             at /Users/iex/github.com/iesahin/xvc/file/src/carry_in/mod.rs:202:5
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

$ xvc file copy data.txt data2.txt --as symlink
$ xvc file list
SS         182 2023-02-08 10:32:37 6602cff6          data2.txt
XC                                 6602cff6          data.txt
FX         130 2023-02-08 10:32:34          ac46bf74 .xvcignore
FX         276 2023-02-08 10:32:37          c485e90c .gitignore
Total #: 4 Workspace Size:         588 Cached Size:          19


$ xvc file remove --from-cache data.txt

$ ls -l .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```
