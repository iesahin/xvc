# xvc file remove

## Synopsis

```console
$ xvc file remove --help
thread 'main' panicked at 'Command remove: Argument or group 'before' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
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
thread 'main' panicked at 'Command remove: Argument or group 'before' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
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


You can remove the file from the cache. The file is still tracked by Xvc and available in the workspace.

```console
$ xvc file remove --from-cache data.txt
thread 'main' panicked at 'Command remove: Argument or group 'before' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
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

$ tree .xvc/b3/
.xvc/b3/
└── c85
    └── f3e
        └── 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
            └── 0.txt

4 directories, 1 file

```

You can carry-in the missing file from the workspace to the cache.

```console
$ xvc file carry-in data.txt

$ xvc file list
FC          19 2023-01-31 08:00:58 c85f3e81 c85f3e81 data.txt
FX         130 2023-02-10 08:08:37          ac46bf74 .xvcignore
FX         191 2023-02-10 08:08:38          33d60e0a .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


$ tree .xvc/b3/
.xvc/b3/
└── c85
    └── f3e
        └── 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
            └── 0.txt

4 directories, 1 file

```

You can specify a version of a file to delete from the cache. The versions can
be specified like `123-456-789abcd`. Dashes are optional. The prefix must be unique.

$ perl -pi -e 's/a/e/g' data.txt

$ xvc file carry-in data.txt

$ tree .xvc/b3/
.xvc/b3/
├── 660
│   └── 2cf
│       └── f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
│           └── 0.txt
└── c85
    └── f3e
        └── 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
            └── 0.txt

7 directories, 2 files

$ xvc file list
FC          19 2023-02-08 10:44:10 6602cff6 6602cff6 data.txt
FX         130 2023-02-08 10:44:09          ac46bf74 .xvcignore
FX         191 2023-02-08 10:44:09          eb676f07 .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


$ xvc file remove --from-cache --only-version c85-f3e data.txt

$ tree .xvc/b3/

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
thread 'main' panicked at 'Command remove: Argument or group 'before' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
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

$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

You can remove all versions of the file smaller than 500 bytes.

```console
$ xvc file remove --from-cache --smaller-than 500 data.txt
thread 'main' panicked at 'Command remove: Argument or group 'before' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
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
thread 'main' panicked at 'Command remove: Argument or group 'before' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
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

$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

Remove all versions carried in after 2023-01-02.

```console
$ xvc file remove --from-cache --after 2023-01-02 data.txt
thread 'main' panicked at 'Command remove: Argument or group 'before' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
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
thread 'main' panicked at 'Command remove: Argument or group 'before' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
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
SS         182 2023-02-10 08:08:39 3c9cfe82          data2.txt
FC        2000 2023-02-10 08:08:39 3c9cfe82 3c9cfe82 data.txt
FX         130 2023-02-10 08:08:37          ac46bf74 .xvcignore
FX         276 2023-02-10 08:08:39          52eb9b82 .gitignore
Total #: 4 Workspace Size:        2588 Cached Size:        2000


$ xvc file remove --from-cache data.txt
thread 'main' panicked at 'Command remove: Argument or group 'before' specified in 'conflicts_with*' for 'only_version' does not exist', /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-4.1.1/src/builder/debug_asserts.rs:223:13
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

$ ls -l .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```
