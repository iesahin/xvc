# xvc pipeline step new

## Purpose

Create a new step in the pipeline.

## Synopsis

```console
$ xvc pipeline step new --help
Add a new step

Usage: xvc pipeline step new [OPTIONS] --step-name <STEP_NAME> --command <COMMAND>

Options:
  -s, --step-name <STEP_NAME>  Name of the new step
  -c, --command <COMMAND>      Step command to run
      --when <WHEN>            When to run the command. One of always, never, by_dependencies (default). This is used to freeze or invalidate a step manually
  -h, --help                   Print help

```

## Examples

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

You can create a new step with a name and a command.

```console
$ xvc pipeline step new --step-name hello --command "echo hello"
```

By default a step will run only if its dependencies have changed. (`--when by_dependencies`).

If you want to run the command always, regardless of the changes in dependencies, you can set `--when` to `always`.

```console
$ xvc pipeline step new --step-name world --command "echo world" --when always
```

If you want a step to never run, you can set `--when` to `never`.

```console
$ xvc pipeline step new --step-name never --command "echo never" --when never
```

You can update when the step will run with [`xvc pipeline step update`](/ref/xvc-pipeline-step-update.md).

You can get the list of steps in the pipeline with `export` or `dag`.

```console
$ xvc pipeline export
{
  "name": "default",
  "steps": [
    {
      "command": "echo hello",
      "dependencies": [],
      "invalidate": "ByDependencies",
      "name": "hello",
      "outputs": []
    },
    {
      "command": "echo world",
      "dependencies": [],
      "invalidate": "Always",
      "name": "world",
      "outputs": []
    },
    {
      "command": "echo never",
      "dependencies": [],
      "invalidate": "Never",
      "name": "never",
      "outputs": []
    }
  ],
  "version": 1,
  "workdir": ""
}

$ xvc -vvvv pipeline dag --format mermaid
thread '<unnamed>' panicked at 'called `Option::unwrap()` on a `None` value', pipeline/src/pipeline/api/dag.rs:68:45
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic
   3: core::option::Option<T>::unwrap
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/option.rs:778:21
   4: xvc_pipeline::pipeline::api::dag::step_desc
             at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/api/dag.rs:68:9
   5: xvc_pipeline::pipeline::api::dag::cmd_dag::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/api/dag.rs:194:17
   6: core::iter::adapters::map::map_fold::{{closure}}
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/iter/adapters/map.rs:84:28
   7: core::iter::traits::iterator::Iterator::fold
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/iter/traits/iterator.rs:2414:21
   8: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/iter/adapters/map.rs:124:9
   9: core::iter::traits::iterator::Iterator::for_each
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/iter/traits/iterator.rs:831:9
  10: <hashbrown::map::HashMap<K,V,S,A> as core::iter::traits::collect::Extend<(K,V)>>::extend
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/vendor/hashbrown/src/map.rs:6407:9
  11: <std::collections::hash::map::HashMap<K,V,S> as core::iter::traits::collect::Extend<(K,V)>>::extend
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/collections/hash/map.rs:3037:9
  12: <std::collections::hash::map::HashMap<K,V,S> as core::iter::traits::collect::FromIterator<(K,V)>>::from_iter
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/collections/hash/map.rs:3022:9
  13: <xvc_ecs::ecs::hstore::HStore<T> as core::iter::traits::collect::FromIterator<(xvc_ecs::ecs::XvcEntity,T)>>::from_iter
             at /Users/iex/github.com/iesahin/xvc/ecs/src/ecs/hstore.rs:55:18
  14: core::iter::traits::iterator::Iterator::collect
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/iter/traits/iterator.rs:1836:9
  15: xvc_pipeline::pipeline::api::dag::cmd_dag
             at /Users/iex/github.com/iesahin/xvc/pipeline/src/pipeline/api/dag.rs:189:38
  16: xvc_pipeline::cmd_pipeline
             at /Users/iex/github.com/iesahin/xvc/pipeline/src/lib.rs:419:13
  17: xvc::cli::dispatch::{{closure}}::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:349:24
  18: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:438:31
  19: core::ops::function::FnOnce::call_once{{vtable.shim}}
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/ops/function.rs:507:5
  20: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/alloc/src/boxed.rs:2000:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', lib/src/cli/mod.rs:403:37
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   3: core::result::Result<T,E>::unwrap
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/result.rs:1113:23
   4: xvc::cli::dispatch::{{closure}}
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:403:15
   5: crossbeam_utils::thread::scope::{{closure}}
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:65
   6: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/panic/unwind_safe.rs:271:9
   7: std::panicking::try::do_call
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panicking.rs:483:40
   8: ___rust_try
   9: std::panicking::try
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panicking.rs:447:19
  10: std::panic::catch_unwind
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/std/src/panic.rs:137:14
  11: crossbeam_utils::thread::scope
             at /Users/iex/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:161:18
  12: xvc::cli::dispatch
             at /Users/iex/github.com/iesahin/xvc/lib/src/cli/mod.rs:239:5
  13: xvc::main
             at /Users/iex/github.com/iesahin/xvc/workflow_tests/src/main.rs:12:5
  14: core::ops::function::FnOnce::call_once
             at /private/tmp/rust-20230210-12080-46tpq2/rustc-1.67.1-src/library/core/src/ops/function.rs:507:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

```
