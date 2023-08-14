### File Dependencies

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Begin by adding a new step.

```console
$ xvc pipeline step new --step-name file-dependency --command "echo data.txt has changed"
```

Add a file dependency to the step.

```console
$ xvc pipeline step dependency --step-name file-dependency --file data.txt
```

When you run the command, it will print `data.txt has changed` if the file `data.txt` has changed.

```console
$ xvc pipeline run
thread 'thread '<unnamed>' panicked at '[PANIC] PathNotFound { path: "data.txt" }, [pipeline/src/pipeline/mod.rs::1129]', lib/src/cli/mod.rs:263:52
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
<unnamed>' panicked at 'PathNotFound { path: "data.txt" }', pipeline/src/pipeline/mod.rs:1129:28

```

You can add multiple dependencies to a step with multiple invocations.

```console
$ xvc pipeline step dependency --step-name file-dependency --file data2.txt
```

A step will run if any of its dependencies have changed.

```console
$ xvc pipeline run
thread 'thread '<unnamed>' panicked at '<unnamed>PathNotFound { path: "data2.txt" }' panicked at '', [PANIC] PathNotFound { path: "data2.txt" }, [pipeline/src/pipeline/mod.rs::1129]', lib/src/cli/mod.rspipeline/src/pipeline/mod.rs::263:52
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
1129:28

```

By default, they are not run if none of the dependencies have changed.

```console
$ xvc pipeline run
thread '<unnamed>' panicked at 'PathNotFound { path: "data2.txt" }', thread 'pipeline/src/pipeline/mod.rs<unnamed>:' panicked at '1129[PANIC] PathNotFound { path: "data2.txt" }, [pipeline/src/pipeline/mod.rs::1129]:', 28lib/src/cli/mod.rs:263:52

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

However, if you want to run the step even if none of the dependencies have changed, you can set the `--when` option to `always`.

```console
$ xvc pipeline step update --step-name file-dependency --when always
```

Now the step will run even if none of the dependencies have changed.

```console
$ xvc pipeline run
thread 'thread '<unnamed><unnamed>' panicked at '' panicked at 'PathNotFound { path: "data.txt" }[PANIC] PathNotFound { path: "data.txt" }, [pipeline/src/pipeline/mod.rs::1129]', ', pipeline/src/pipeline/mod.rslib/src/cli/mod.rs:1129:28
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
:263:52

```
