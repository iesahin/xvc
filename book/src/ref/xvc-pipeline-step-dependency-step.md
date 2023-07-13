
### Step Dependencies

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

You can add a step dependency to a step. These steps specify dependency relationships explicitly, without relying on
 changed files or directories.

```console
$ xvc pipeline step new --step-name world --command "echo world"
$ xvc pipeline step new --step-name hello --command "echo hello"
$ xvc pipeline step dependency --step-name world --step hello
$ xvc pipeline step dependency --step-name hello --step file-dependency
```

When run, the dependency will be run first and the step will be run after.

```console
$ xvc pipeline run
[OUT] [file-dependency] data.txt has changed


```
