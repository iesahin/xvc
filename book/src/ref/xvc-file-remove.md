# xvc file remove

## Synopsis

```console
$ xvc file remove --help
thread 'main' panicked at 'Command remove: Argument or group 'all-versions' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: clap::builder::debug_asserts::assert_app
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
   3: clap::builder::command::Command::_build_self
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:3920:13
   4: clap::builder::command::Command::_build_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:4007:9
   5: clap::parser::parser::Parser::parse_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:687:27
   6: clap::parser::parser::Parser::get_matches_with
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:474:17
   7: clap::parser::parser::Parser::parse_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:704:37
   8: clap::parser::parser::Parser::get_matches_with
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:474:17
   9: clap::builder::command::Command::_do_parse
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:3796:29
  10: clap::builder::command::Command::try_get_matches_from_mut
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:708:9
  11: clap::builder::command::Command::get_matches_from
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:578:9
  12: clap::derive::Parser::parse_from
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/derive.rs:107:27
  13: xvc::cli::XvcCLI::from_args_os
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:131:22
  14: xvc::main
             at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:11:20
  15: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

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
FX         191 2023-02-07 11:03:33          2c6ebabe .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


```

If you don't specify either `--from-cache` or `--from-storage`, this command does nothing.

```console
$ xvc file remove data.txt
thread 'main' panicked at 'Command remove: Argument or group 'all-versions' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: clap::builder::debug_asserts::assert_app
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
   3: clap::builder::command::Command::_build_self
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:3920:13
   4: clap::builder::command::Command::_build_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:4007:9
   5: clap::parser::parser::Parser::parse_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:687:27
   6: clap::parser::parser::Parser::get_matches_with
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:474:17
   7: clap::parser::parser::Parser::parse_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:704:37
   8: clap::parser::parser::Parser::get_matches_with
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:474:17
   9: clap::builder::command::Command::_do_parse
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:3796:29
  10: clap::builder::command::Command::try_get_matches_from_mut
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:708:9
  11: clap::builder::command::Command::get_matches_from
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:578:9
  12: clap::derive::Parser::parse_from
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/derive.rs:107:27
  13: xvc::cli::XvcCLI::from_args_os
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:131:22
  14: xvc::main
             at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:11:20
  15: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

```


You can remove the file from the cache and keep the workspace version.

```console
$ xvc file remove --from-cache data.txt
thread 'main' panicked at 'Command remove: Argument or group 'all-versions' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: clap::builder::debug_asserts::assert_app
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
   3: clap::builder::command::Command::_build_self
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:3920:13
   4: clap::builder::command::Command::_build_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:4007:9
   5: clap::parser::parser::Parser::parse_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:687:27
   6: clap::parser::parser::Parser::get_matches_with
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:474:17
   7: clap::parser::parser::Parser::parse_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:704:37
   8: clap::parser::parser::Parser::get_matches_with
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:474:17
   9: clap::builder::command::Command::_do_parse
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:3796:29
  10: clap::builder::command::Command::try_get_matches_from_mut
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:708:9
  11: clap::builder::command::Command::get_matches_from
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:578:9
  12: clap::derive::Parser::parse_from
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/derive.rs:107:27
  13: xvc::cli::XvcCLI::from_args_os
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:131:22
  14: xvc::main
             at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:11:20
  15: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

$ ls -l
total 8
-rw-rw-rw-  1 iex  staff  19 Jan 31 11:00 data.txt

```

You can carry-in the file from the workspace to the cache.

```console
$ xvc file carry-in data.txt

$ xvc file list
FC          19 2023-01-31 08:00:58 c85f3e81 c85f3e81 data.txt
FX         130 2023-02-07 11:03:32          ac46bf74 .xvcignore
FX         191 2023-02-07 11:03:33          2c6ebabe .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


```

You can remove all versions of a file from the cache.

```console
$ perl -pi -e 's/a/e/g' data.txt

$ xvc file carry-in data.txt

$ xvc file list
FC          19 2023-02-07 11:03:34 6602cff6 6602cff6 data.txt
FX         130 2023-02-07 11:03:32          ac46bf74 .xvcignore
FX         191 2023-02-07 11:03:33          2c6ebabe .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


$ xvc file remove --from-cache --all-versions data.txt
thread 'main' panicked at 'Command remove: Argument or group 'all-versions' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: clap::builder::debug_asserts::assert_app
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
   3: clap::builder::command::Command::_build_self
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:3920:13
   4: clap::builder::command::Command::_build_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:4007:9
   5: clap::parser::parser::Parser::parse_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:687:27
   6: clap::parser::parser::Parser::get_matches_with
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:474:17
   7: clap::parser::parser::Parser::parse_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:704:37
   8: clap::parser::parser::Parser::get_matches_with
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:474:17
   9: clap::builder::command::Command::_do_parse
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:3796:29
  10: clap::builder::command::Command::try_get_matches_from_mut
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:708:9
  11: clap::builder::command::Command::get_matches_from
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:578:9
  12: clap::derive::Parser::parse_from
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/derive.rs:107:27
  13: xvc::cli::XvcCLI::from_args_os
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:131:22
  14: xvc::main
             at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:11:20
  15: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

```

You can use this command to remove the files from storages as well.

```console
$ xvc file carry-in data.txt

$ xvc storage new local --name local-storage --path '../local-storage'

$ xvc file send data.txt --to local-storage

$ xvc file remove data.txt --from-storage local-storage
thread 'main' panicked at 'Command remove: Argument or group 'all-versions' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: clap::builder::debug_asserts::assert_app
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
   3: clap::builder::command::Command::_build_self
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:3920:13
   4: clap::builder::command::Command::_build_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:4007:9
   5: clap::parser::parser::Parser::parse_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:687:27
   6: clap::parser::parser::Parser::get_matches_with
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:474:17
   7: clap::parser::parser::Parser::parse_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:704:37
   8: clap::parser::parser::Parser::get_matches_with
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:474:17
   9: clap::builder::command::Command::_do_parse
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:3796:29
  10: clap::builder::command::Command::try_get_matches_from_mut
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:708:9
  11: clap::builder::command::Command::get_matches_from
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:578:9
  12: clap::derive::Parser::parse_from
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/derive.rs:107:27
  13: xvc::cli::XvcCLI::from_args_os
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:131:22
  14: xvc::main
             at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:11:20
  15: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

```

If multiple paths are pointing to the same cache file (deduplication), the cache file will not be deleted.
In this case, `remove` reports other paths pointing to the same cache file. You must `--force` delete the cache file.

```console
$ xvc file track data.txt

$ xvc file copy data.txt data2.txt --as symlink

$ xvc file remove --from-cache data.txt
thread 'main' panicked at 'Command remove: Argument or group 'all-versions' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: clap::builder::debug_asserts::assert_app
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
   3: clap::builder::command::Command::_build_self
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:3920:13
   4: clap::builder::command::Command::_build_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:4007:9
   5: clap::parser::parser::Parser::parse_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:687:27
   6: clap::parser::parser::Parser::get_matches_with
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:474:17
   7: clap::parser::parser::Parser::parse_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:704:37
   8: clap::parser::parser::Parser::get_matches_with
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:474:17
   9: clap::builder::command::Command::_do_parse
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:3796:29
  10: clap::builder::command::Command::try_get_matches_from_mut
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:708:9
  11: clap::builder::command::Command::get_matches_from
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:578:9
  12: clap::derive::Parser::parse_from
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/derive.rs:107:27
  13: xvc::cli::XvcCLI::from_args_os
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:131:22
  14: xvc::main
             at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:11:20
  15: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

$ ls -lR .xvc/b3/
total 0
drwxr-xr-x  3 iex  staff  96 Feb  7 14:03 660
drwxr-xr-x  3 iex  staff  96 Feb  7 14:03 c85

.xvc/b3//660:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  7 14:03 2cf

.xvc/b3//660/2cf:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  7 14:03 f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367

.xvc/b3//660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367:
total 8
-r--r--r--  1 iex  staff  19 Feb  7 14:03 0.txt

.xvc/b3//c85:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  7 14:03 f3e

.xvc/b3//c85/f3e:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  7 14:03 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496

.xvc/b3//c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496:
total 8
-r--r--r--  1 iex  staff  19 Jan 31 11:00 0.txt

$ xvc file remove --from-cache --force data.txt
thread 'main' panicked at 'Command remove: Argument or group 'all-versions' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: clap::builder::debug_asserts::assert_app
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
   3: clap::builder::command::Command::_build_self
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:3920:13
   4: clap::builder::command::Command::_build_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:4007:9
   5: clap::parser::parser::Parser::parse_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:687:27
   6: clap::parser::parser::Parser::get_matches_with
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:474:17
   7: clap::parser::parser::Parser::parse_subcommand
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:704:37
   8: clap::parser::parser::Parser::get_matches_with
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/parser/parser.rs:474:17
   9: clap::builder::command::Command::_do_parse
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:3796:29
  10: clap::builder::command::Command::try_get_matches_from_mut
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:708:9
  11: clap::builder::command::Command::get_matches_from
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/command.rs:578:9
  12: clap::derive::Parser::parse_from
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/derive.rs:107:27
  13: xvc::cli::XvcCLI::from_args_os
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:131:22
  14: xvc::main
             at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:11:20
  15: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

$ ls -lR .xvc/b3/
total 0
drwxr-xr-x  3 iex  staff  96 Feb  7 14:03 660
drwxr-xr-x  3 iex  staff  96 Feb  7 14:03 c85

.xvc/b3//660:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  7 14:03 2cf

.xvc/b3//660/2cf:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  7 14:03 f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367

.xvc/b3//660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367:
total 8
-r--r--r--  1 iex  staff  19 Feb  7 14:03 0.txt

.xvc/b3//c85:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  7 14:03 f3e

.xvc/b3//c85/f3e:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  7 14:03 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496

.xvc/b3//c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496:
total 8
-r--r--r--  1 iex  staff  19 Jan 31 11:00 0.txt

```
