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
? 2
error: unexpected argument '--name' found

Usage: xvc pipeline step new [OPTIONS] --step-name <STEP_NAME> --command <COMMAND>

For more information, try '--help'.

$ xvc pipeline step new --step-name train --command "echo 'train'"
? 2
error: unexpected argument '--name' found

Usage: xvc pipeline step new [OPTIONS] --step-name <STEP_NAME> --command <COMMAND>

For more information, try '--help'.

```

```console
$ xvc pipeline dag
digraph {
}


```
