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

We create a dependency between the two steps by using the `--dependencies` flag to make them run sequentially.

```console
$ xvc pipeline step new --step-name preprocess --command "echo 'preprocess'"

$ xvc pipeline step new --step-name train --command "echo 'train'"

$ xvc pipeline step dependency --step-name train --step preprocess

```

```console
$ xvc pipeline dag
digraph {
    0 [ label = "step: START (always, )" ]
    1 [ label = "step: START (always, )" ]
    2 [ label = "step: train (by_dependencies, echo 'train')" ]
    3 [ label = "step: train (by_dependencies, echo 'train')" ]
    4 [ label = "step: preprocess (by_dependencies, echo 'preprocess')" ]
    0 -> 0 [ label = "" ]
    1 -> 1 [ label = "" ]
    2 -> 2 [ label = "" ]
    3 -> 3 [ label = "" ]
    4 -> 4 [ label = "" ]
}


```

When you add a dependency between two steps, the graph shows it as a node.

```console
$ xvc pipeline step dependency --step-name preprocess --glob 'data/*'

$ xvc pipeline dag
digraph {
    0 [ label = "step: START (always, )" ]
    1 [ label = "step: START (always, )" ]
    2 [ label = "step: train (by_dependencies, echo 'train')" ]
    3 [ label = "step: train (by_dependencies, echo 'train')" ]
    4 [ label = "step: preprocess (by_dependencies, echo 'preprocess')" ]
    0 -> 0 [ label = "" ]
    1 -> 1 [ label = "" ]
    2 -> 2 [ label = "" ]
    3 -> 3 [ label = "" ]
    4 -> 4 [ label = "" ]
}


```
