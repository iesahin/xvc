# xvc file recheck

## Synopsis

```console
$ xvc file recheck --help
Get files from cache by copy or *link

Usage: xvc file recheck [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to recheck

Options:
      --cache-type <CACHE_TYPE>
          How to track the file contents in cache: One of copy, symlink, hardlink, reflink.
          
          Note: Reflink uses copy if the underlying file system doesn't support it.

      --no-parallel
          Don't use parallelism

      --force
          Force even if target exists

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

This command has an alias [`xvc file checkout`](/ref/xvc-file-checkout.md) if you feel more at home with Git terminology.

## Examples

Rechecking is analogous to [git checkout](https://git-scm.com/docs/git-checkout).
It copies or links a cached file to the workspace.

Start by tracking a file.

```console
$ git init
...
$ xvc init

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

$ ls -l
total[..]
-rw-r--r--  1 iex  staff  19 Jan 25 11:51 data.txt

```

Once you added the file to the cache, you can delete the workspace copy.

```console
$ rm data.txt
$ ls -l
total[..]

```

Then, recheck the file. By default, it makes a copy of the file.

```console
$ xvc file recheck data.txt
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
   5: <xvc_file::recheck::RecheckCLI as xvc_config::UpdateFromXvcConfig>::update_from_conf::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/file/src/recheck/mod.rs:64:32
   6: core::option::Option<T>::unwrap_or_else
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/option.rs:825:21
   7: <xvc_file::recheck::RecheckCLI as xvc_config::UpdateFromXvcConfig>::update_from_conf
             at /Users/iex/github.com/iesahin/xvc/file/src/recheck/mod.rs:62:26
   8: xvc_file::recheck::cmd_recheck
             at /Users/iex/github.com/iesahin/xvc/file/src/recheck/mod.rs:90:16
   9: xvc_file::run
             at /Users/iex/github.com/iesahin/xvc/file/src/lib.rs:161:45
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

$ ls -l
total [..]

```

Xvc updates the cache type if the file is not changed.

```console
$ xvc file recheck data.txt --as symlink

$ ls -l data.txt
ls: data.txt: No such file or directory

```

Symlink and hardlinks are read-only. 
You can delete the symlink, and replace with an updated copy. 
(As `perl -i` does below.)

```console
$ perl -i -pe 's/a/ee/g' data.txt
Can't open data.txt: No such file or directory.

$ xvc file recheck data.txt --as copy

$ rm data.txt
rm: data.txt: No such file or directory

```

```console
$ xvc -vv file recheck data.txt --as hardlink

$ ls -l
total[..]

```

Note that, as files in the cache are kept read-only, hardlinks and symlinks are also read only. Files rechecked as copy are made read-write explicitly.

Reflinks are supported by Xvc, but the underlying file system should also support it.
Otherwise it uses `copy`.

```console
$ rm -f data.txt
$ xvc file recheck data.txt --as reflink

```

The above command will create a read only link in macOS APFS and a copy in ext4 or NTFS file systems.
