### Step

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

```

When run, the dependency will be run first and the step will be run after.

```console
$ xvc pipeline run
[OUT] [hello] hello

[DONE] [hello] (echo hello)

[OUT] [world] world

[DONE] [world] (echo world)


```

If the dependency is not run, the dependent step won't run either.

```console
$ xvc pipeline step update --step-name hello --when never
$ xvc pipeline run

```

If you want to run the dependent always, you can set it to run always explicitly.

```console
$ xvc pipeline step update --step-name world --when always
$ xvc pipeline run
[OUT] [world] world

[DONE] [world] (echo world)


```
