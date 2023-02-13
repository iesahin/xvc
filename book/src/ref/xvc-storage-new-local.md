# xvc storage new local

## Purpose

Create a new storage reachable from the local filesystem.
It allows to keep tracked file contents in a different directory for backup or sharing purposes.

## Synopsis

```console
$ xvc storage new local --help
Add a new local storage

A local storage is a directory accessible from the local file system. Xvc will use common file operations for this directory without accessing the network.

Usage: xvc storage new local --path <PATH> --name <NAME>

Options:
      --path <PATH>
          Directory (outside the repository) to be set as a storage

  -n, --name <NAME>
          Name of the storage.
          
          Recommended to keep this name unique to refer easily.

  -h, --help
          Print help (see a summary with '-h')

```

## Examples

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

Now, you can define a local directory as storage and begin to use it.

```console
$ xvc storage new local --name backup --path '../my-local-storage'
error: unrecognized subcommand 'new-local'

  note: subcommand 'new' exists
  note: to pass 'new-local' as a value, use 'xvc storage -- new-local'

Usage: xvc storage <COMMAND>

For more information, try '--help'.

```

Send files to this storage.

```console
$ xvc file send dir-0001 --to backup
thread '<unnamed>' panicked at 'Cannot find remote backup', storage/src/storage/mod.rs:458:9
stack backtrace:
thread '<unnamed>' panicked at '[PANIC] Cannot find remote backup [storage/src/storage/mod.rs::458]', lib/src/cli/mod.rs:263:52
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: xvc_storage::storage::get_storage_record
             at /Users/iex/github.com/iesahin/xvc/storage/src/storage/mod.rs:458:9
   3: xvc_file::send::cmd_send
             at /Users/iex/github.com/iesahin/xvc/file/src/send/mod.rs:37:18
   4: xvc_file::run
             at /Users/iex/github.com/iesahin/xvc/file/src/lib.rs:179:42
   5: xvc::cli::dispatch::{{closure}}::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:343:24
   6: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:438:31
   7: core::ops::function::FnOnce::call_once{{vtable.shim}}
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
   8: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/alloc/src/boxed.rs:1940:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', lib/src/cli/mod.rs:403:37
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: xvc::cli::dispatch::{{closure}}::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:263:52
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

Then get back them from the storage.

```console
$ xvc file bring --from backup dir-0001
thread '<unnamed>' panicked at 'Cannot find remote backup', storage/src/storage/mod.rs:458:9
thread '<unnamed>stack backtrace:
' panicked at '[PANIC] Cannot find remote backup [storage/src/storage/mod.rs::458]', lib/src/cli/mod.rs:263:52
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: xvc_storage::storage::get_storage_record
             at /Users/iex/github.com/iesahin/xvc/storage/src/storage/mod.rs:458:9
   3: xvc_file::bring::fetch
             at /Users/iex/github.com/iesahin/xvc/file/src/bring/mod.rs:64:19
   4: xvc_file::bring::cmd_bring
             at /Users/iex/github.com/iesahin/xvc/file/src/bring/mod.rs:158:5
   5: xvc_file::run
             at /Users/iex/github.com/iesahin/xvc/file/src/lib.rs:184:43
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
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:263:52
   3: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:438:31
   4: core::ops::function::FnOnce::call_once{{vtable.shim}}
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
   5: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
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

$ tree dir-0001
dir-0001  [error opening dir]

0 directories, 0 files

```

If you want to remove a file and all of its versions from a storage, you can use `xvc file remove` command.

```console
$ xvc
Xvc CLI to manage data and ML pipelines

Usage: xvc [OPTIONS] <COMMAND>

Commands:
  file          File and directory management commands
  init          Initialize an Xvc project
  pipeline      Pipeline management commands
  storage       Storage (cloud) management commands
  root          Find the root directory of a project
  check-ignore  Check whether files are ignored with `.xvcignore`
  aliases       Print command aliases to be sourced in shell files
  help          Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...             Output verbosity. Use multiple times to increase the output detail
      --quiet                  Suppress all output
      --debug                  Turn on all logging to $TMPDIR/xvc.log
  -C <WORKDIR>                 Set working directory for the command. It doesn't create a new shell, or change the directory [default: .]
  -c, --config <CONFIG>        Configuration options set from the command line in the form section.key=value You can use multiple times
      --no-system-config       Ignore system configuration file
      --no-user-config         Ignore user configuration file
      --no-project-config      Ignore project configuration file (.xvc/config)
      --no-local-config        Ignore local (gitignored) configuration file (.xvc/config.local)
      --no-env-config          Ignore configuration options obtained from environment variables
      --skip-git               Don't run automated Git operations for this command. If you want to run git commands yourself all the time, you can set `git.auto_commit` and `git.auto_stage` options in the configuration to False
      --from-ref <FROM_REF>    Checkout the given Git reference (branch, tag, commit etc.) before performing the Xvc operation. This runs `git checkout <given-value>` before running the command
      --to-branch <TO_BRANCH>  If given, create (or checkout) the given branch before committing results of the operation. This runs `git checkout --branch <given-value>` before committing the changes
  -h, --help                   Print help
  -V, --version                Print version

$ xvc file remove --from-storage backup dir-0001/
thread '<unnamed>' panicked at 'Cannot find remote backup', storage/src/storage/mod.rs:458:9
stack backtrace:
thread '<unnamed>' panicked at '[PANIC] Cannot find remote backup [storage/src/storage/mod.rs::458]', lib/src/cli/mod.rs:263:52
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: xvc_storage::storage::get_storage_record
             at /Users/iex/github.com/iesahin/xvc/storage/src/storage/mod.rs:458:9
   3: xvc_file::remove::cmd_remove
             at /Users/iex/github.com/iesahin/xvc/file/src/remove/mod.rs:214:23
   4: xvc_file::run
             at /Users/iex/github.com/iesahin/xvc/file/src/lib.rs:204:44
   5: xvc::cli::dispatch::{{closure}}::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:343:24
   6: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:438:31
   7: core::ops::function::FnOnce::call_once{{vtable.shim}}
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
   8: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/alloc/src/boxed.rs:1940:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: xvc::cli::dispatch::{{thread 'closuremain}' panicked at '}called `Result::unwrap()` on an `Err` value: Any { .. }::', {lib/src/cli/mod.rs{:closure403}:}37

             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:263:52
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

```

## Caveats

`--name NAME` is not checked to be unique but you should use unique storage names to refer them later.

`--path PATH`  should be accessible for writing and shouldn't already exist.


## Technical Details

The command creates the `PATH` and a new file under `PATH` called `.xvc-guid`.
The file contains the unique identifier for this storage.
The same identifier is also recorded to the project.

A file that's found in `.xvc/{{HASH_PREFIX}}/{{CACHE_PATH}}` is saved to `PATH/{{REPO_ID}}/{{HASH_PREFIX}}/{{CACHE_PATH}}`.
`{{REPO_ID}}` is the unique identifier for the repository created during `xvc init`.
Hence if you use a common storage for different Xvc projects, their files are kept under different directories.
There is no inter-project deduplication.
