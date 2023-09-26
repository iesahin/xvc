# xvc pipeline dag

## Synopsis

```console
$ xvc pipeline dag --help
Generate a dot or mermaid diagram for the pipeline

Usage: xvc pipeline dag [OPTIONS]

Options:
  -n, --name <NAME>      Name of the pipeline to generate the diagram
      --file <FILE>      Output file. Writes to stdout if not set
      --format <FORMAT>  Format for graph. Either dot or mermaid [default: dot]
  -h, --help             Print help

```

You can visualize the pipeline you defined with [xvc pipeline](/ref/xvc-pipeline/) set of command with the `xvc pipeline
dag` command. It will generate a dot or mermaid diagram for the pipeline.

## Examples

As all other pipeline commands, this requires an Xvc repository.

```console
$ git init
Initialized empty Git repository in [CWD]/.git/

$ xvc init
```

All steps of the pipeline are shown as nodes in the graph.

```console
$ xvc pipeline step new --step-name preprocess --command "echo 'preprocess'"

$ xvc pipeline step new --step-name train --command "echo 'train'"

```

```console
$ xvc pipeline dag
digraph {
    0 [ label = "step: START (always, )" ]
    1 [ label = "step: preprocess (by_dependencies, echo 'preprocess')" ]
    2 [ label = "step: END (never, )" ]
    3 [ label = "step: train (by_dependencies, echo 'train')" ]
    0 -> 1 [ label = "" ]
    0 -> 3 [ label = "" ]
    1 -> 2 [ label = "" ]
    3 -> 2 [ label = "" ]
}


```
