### File

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
[OUT] [file-dependency] data.txt has changed

[DONE] [file-dependency] (echo data.txt has changed)


```

You can add multiple dependencies to a step with multiple invocations.

```console
$ xvc pipeline step dependency --step-name file-dependency --file data2.txt

```

A step will run if any of its dependencies have changed.

```console
$ xvc pipeline run
[OUT] [file-dependency] data.txt has changed

[DONE] [file-dependency] (echo data.txt has changed)


```

By default, they are not run if none of the dependencies have changed.

```console
$ xvc pipeline run

```

However, if you want to run the step even if none of the dependencies have changed, you can set the `--when` option to `always`.

```console
$ xvc pipeline step update --step-name file-dependency --when always
```

Now the step will run even if none of the dependencies have changed.

```console
$ xvc pipeline run
[OUT] [file-dependency] data.txt has changed

[DONE] [file-dependency] (echo data.txt has changed)


```
