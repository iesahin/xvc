# xvc pipeline run

## Synopsis

```console
$ xvc pipeline run --help
Run a pipeline

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
+---------+---------+
| Name    | Run Dir |
+===================+
| default |         |
+---------+---------+

```

Create a new step in this pipeline with [`xvc pipeline step new`](/ref/xvc-pipeline-step-new.md) command.

```console
$ xvc pipeline step new --step-name hello --command "echo hello"
```

```console
$ xvc pipeline dag
digraph {
    0 [ label = "step: START (always, )" ]
    1 [ label = "step: hello (by_dependencies, echo hello)" ]
    2 [ label = "step: END (never, )" ]
    0 -> 1 [ label = "" ]
    1 -> 2 [ label = "" ]
}


```


You can run the default pipeline without specifying its name.

```console
$ xvc pipeline run
[OUT] hello

[OUT] [EXIT] Successfully

```

Note that, when a step has no dependencies, it's set to always run if it's not set to run never explicitly.

```console
$ xvc pipeline step update --step-name hello --when never

$ xvc pipeline run

```

