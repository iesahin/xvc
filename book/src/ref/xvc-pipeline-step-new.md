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

```
