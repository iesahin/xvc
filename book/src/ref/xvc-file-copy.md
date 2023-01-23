# xvc file copy

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

      --no-parallel
          Don't use parallelism

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

This command is used to copy a set of files to another location in the workspace.

By default, it doesn't update the recheck method (cache type) of the targets.
It rechecks them to the destination with the same method.

`xvc file copy` works only with the tracked files.

```console
$ git init
...
$ xvc init

$ xvc file track data.txt

$ ls -l
total[..]
-rw-rw-rw-  1 iex  staff  19 Jan 19 10:47 data.txt

```

Once you add the file to the cache, you can copy the file to another location.

```console
$ xvc file copy data.txt data2.txt

$ ls
data.txt
data2.txt

```

Xvc updates the cache type if the file is not changed.

```console
$ xvc file copy data.txt data3.txt --as symlink

$ ls -l
total[..]
-rw-rw-rw-  1 iex  staff   19 Jan 19 10:47 data.txt
-rw-rw-rw-  1 iex  staff   19 Jan 19 10:47 data2.txt
lrwxr-xr-x  1 iex  staff  180 Jan 22 11:32 data3.txt -> [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt

```

You can create _views_ of your data by copying it to another location.

```console
$ xvc file copy 'd*' another-set/ --as hardlink

$ xvc file list another-set/
FH          19 2023-01-19 07:47:07   another-set/data3.txt  c85f3e81 c85f3e81
FH          19 2023-01-19 07:47:07   another-set/data2.txt  c85f3e81 c85f3e81
FH          19 2023-01-19 07:47:07   another-set/data.txt  c85f3e81 c85f3e81
Total #: 3 Workspace Size:          57 Cached Size:          19


```

If the targets you specify are changed, they are not copied.

```console
$ perl -i -pe 's/a/ee/g' data.txt

$ xvc file copy data.txt data5.txt

```

You can copy files _virtually_ without them being in the workspace.

```console
$ rm -f data.txt

$ xvc file copy data.txt data6.txt

$ ls -l data6.txt
-rw-rw-rw-  1 iex  staff  19 Jan 19 10:47 data6.txt

```

You can also skip rechecking.
In this case, xvc won't create copies in the workspace.
They will be listed with `xvc file list` command.

```console
$ xvc file copy data.txt data7.txt --no-recheck

$ ls
another-set
data2.txt
data3.txt
data5.txt
data6.txt

$ xvc file list
thread '<unnamed>' panicked at 'called `Option::unwrap()` on a `None` value', file/src/list/mod.rs:572:74
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: core::panicking::panic
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:48:5
   3: core::option::Option<T>::unwrap
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/option.rs:775:21
   4: xvc_file::list::cmd_list
             at /Users/iex/github.com/iesahin/xvc/file/src/list/mod.rs:572:39
   5: xvc_file::run
             at /Users/iex/github.com/iesahin/xvc/file/src/lib.rs:162:42
   6: xvc::cli::dispatch::{{closure}}::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:274:24
   7: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:438:31
   8: core::ops::function::FnOnce::call_once{{vtable.shim}}
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
   9: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
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

```

Later, you can recheck them.

```console
$ xvc file recheck data7.txt

$ ls -l data7.txt
-rw-rw-rw-  1 iex  staff  19 Jan 19 10:47 data7.txt

```
