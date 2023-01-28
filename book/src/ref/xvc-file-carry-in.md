# xvc file carry-in

Copies the file changes to cache.

## Synopsis

```console
$ xvc file carry-in --help
Carry (commit) changed files to cache

Usage: xvc file carry-in [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to add

Options:
      --text-or-binary <TEXT_OR_BINARY>
          Calculate digests as text or binary file without checking contents, or by automatically. (Default: auto)

      --force
          Carry in targets even their content digests are not changed.
          
          This removes the file in cache and re-adds it.

      --no-parallel
          Don't use parallelism

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

## Examples

Carry in command works with Xvc repositories.

```console
$ git init
...
$ xvc init

```

We first track a file.

```console
$ xvc file track data.txt
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: ConfigKeyNotFound { key: "file.recheck.method" }', core/src/types/recheckmethod.rs:84:1
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: core::result::unwrap_failed
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/result.rs:1785:5
   3: core::result::Result<T,E>::unwrap
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/result.rs:1107:23
   4: <xvc_core::types::recheckmethod::RecheckMethod as xvc_config::FromConfigKey<xvc_core::types::recheckmethod::RecheckMethod>>::from_conf
             at /Users/iex/github.com/iesahin/xvc/config/src/lib.rs:648:17
   5: <xvc_file::track::TrackCLI as xvc_config::UpdateFromXvcConfig>::update_from_conf::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/file/src/track/mod.rs:75:32
   6: core::option::Option<T>::unwrap_or_else
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/option.rs:825:21
   7: <xvc_file::track::TrackCLI as xvc_config::UpdateFromXvcConfig>::update_from_conf
             at /Users/iex/github.com/iesahin/xvc/file/src/track/mod.rs:73:26
   8: xvc_file::track::cmd_track
             at /Users/iex/github.com/iesahin/xvc/file/src/track/mod.rs:123:16
   9: xvc_file::run
             at /Users/iex/github.com/iesahin/xvc/file/src/lib.rs:150:43
  10: xvc::cli::dispatch::{{closure}}::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:274:24
  11: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:438:31
  12: core::ops::function::FnOnce::call_once{{vtable.shim}}
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
  13: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/alloc/src/boxed.rs:1940:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', lib/src/cli/mod.rs:394:6
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: core::result::unwrap_failed
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/result.rs:1785:5
   3: core::result::Result<T,E>::unwrap
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/result.rs:1107:23
   4: xvc::cli::dispatch
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:243:5
   5: xvc::main
             at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:12:5
   6: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

$ xvc file list data.txt
Frct          19 2023-01-25 08:51:03          c85f3e81 data.txt
Total #: 1 Workspace Size:          19 Cached Size:           0


```

We update the file with a command.

```console
$ perl -i -pe 's/a/ee/g' data.txt

$ cat data.txt
Oh, deetee, my, deetee

$ xvc file list data.txt
Frct          23 2023-01-28 17:08:26          e37c686a data.txt
Total #: 1 Workspace Size:          23 Cached Size:           0


```

Note that the size of the file has increased, as we replace each `a` with an `ee`.

```console
$ xvc file carry-in data.txt

$ xvc file list data.txt
Frct          23 2023-01-28 17:08:26          e37c686a data.txt
Total #: 1 Workspace Size:          23 Cached Size:           0


```
