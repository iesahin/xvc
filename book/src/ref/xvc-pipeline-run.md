# xvc pipeline run

## Synopsis

```console
$ xvc pipeline run --help
Run a pipeline

Usage: xvc pipeline run [OPTIONS]

Options:
  -p, --pipeline-name <PIPELINE_NAME>  Name of the pipeline to run
  -h, --help                           Print help

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
$ xvc pipeline dag --format=mermaid
flowchart TD
    n0["hello"]


```

You can run the default pipeline without specifying its name.

```console
$ xvc pipeline run
[OUT] [hello] hello
 
[DONE] hello (echo hello)

```

Note that, when a step has no dependencies, it's set to always run if it's not set to run never explicitly.

```console
$ xvc pipeline step update --step-name hello --when never

$ xvc pipeline run

```

### Run a specific pipeline

You can run a specific pipeline by specifying its name with `--name` option.

```console
$ xvc pipeline new --pipeline-name my-pipeline
? 2
error: unexpected argument '--name' found

Usage: xvc pipeline new [OPTIONS] --pipeline-name <PIPELINE_NAME>

For more information, try '--help'.

$ xvc pipeline --pipeline-name my-pipeline step new --step-name my-hello --command "echo 'hello from my-pipeline'"
? 2
error: unexpected argument '--name' found

Usage: xvc pipeline [OPTIONS] <COMMAND>

For more information, try '--help'.

```

```console
$ xvc pipeline run --pipeline-name my-pipeline
? 2
error: unexpected argument '--name' found

Usage: xvc pipeline run [OPTIONS]

For more information, try '--help'.

```
