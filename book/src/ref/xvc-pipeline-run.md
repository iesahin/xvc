# xvc pipeline run

## Synopsis

```console
$ xvc pipeline run --help
Run an Xvc pipeline

Usage: xvc pipeline run [OPTIONS]

Options:
  -n, --name <NAME>  Name of the pipeline to run
  -h, --help         Print help

```

## Examples

Pipelines require Xvc to be initialized before running.

```console
$ git init
...
$ xvc init
```

Xvc defines a default pipeline and any steps added without specifying the pipeline will be added to it.

```console
$ xvc pipeline list
```

Create a new step in this pipeline with [`xvc pipeline step new`](/ref/xvc-pipeline-step-new.md) command.

```console
$ xvc pipeline step new --step-name hello --command "echo hello"
```

You can run the default pipeline without specifying its name.

```console
$ xvc pipeline run
```
