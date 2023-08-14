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
$ xvc --debug pipeline run
thread '<unnamed>' panicked at 'internal error: entered unreachable code: Both record and actual are None', pipeline/src/pipeline/deps/file.rs:83:29
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[ERROR] Error in step thread: Any { .. }

```

You can add multiple dependencies to a step with multiple invocations.

```console
$ xvc pipeline step dependency --step-name file-dependency --file data2.txt
```

A step will run if any of its dependencies have changed.

```console
$ xvc pipeline run
thread '<unnamed>' panicked at 'internal error: entered unreachable code: Both record and actual are None', pipeline/src/pipeline/deps/file.rs:83:29
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[ERROR] Error in step thread: Any { .. }

```

By default, they are not run if none of the dependencies have changed.

```console
$ xvc pipeline run
thread '<unnamed>' panicked at 'internal error: entered unreachable code: Both record and actual are None', pipeline/src/pipeline/deps/file.rs:83:29
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[ERROR] Error in step thread: Any { .. }

```

However, if you want to run the step even if none of the dependencies have changed, you can set the `--when` option to `always`.

```console
$ xvc pipeline step update --step-name file-dependency --when always
```

Now the step will run even if none of the dependencies have changed.

```console
$ xvc pipeline run
thread '<unnamed>' panicked at 'internal error: entered unreachable code: Both record and actual are None', pipeline/src/pipeline/deps/file.rs:83:29
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[ERROR] Error in step thread: Any { .. }

```
