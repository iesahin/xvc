# Data-Model Pipelines

## Synopsis

```console
$ xvc pipeline --help
Pipeline management commands

Usage: xvc pipeline [OPTIONS] <COMMAND>

Commands:
  new     Create a new pipeline [aliases: n]
  update  Update the name and other attributes of a pipeline [aliases: u]
  delete  Delete a pipeline [aliases: D]
  run     Run a pipeline [aliases: r]
  list    List all pipelines [aliases: l]
  dag     Generate a Graphviz or mermaid diagram of the pipeline [aliases: d]
  export  Export the pipeline to a YAML or JSON file to edit [aliases: e]
  import  Import the pipeline from a file [aliases: i]
  step    Step creation, dependency, output commands [aliases: s]
  help    Print this message or the help of the given subcommand(s)

Options:
  -p, --pipeline-name <PIPELINE_NAME>  Name of the pipeline this command applies to
  -h, --help                           Print help

```
