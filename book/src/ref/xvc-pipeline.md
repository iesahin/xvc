# Data-Model Pipelines

## Synopsis

```console
$ xvc pipeline --help
Pipeline management commands

Usage: xvc pipeline [OPTIONS] <COMMAND>

Commands:
  new     Create a new pipeline
  update  Update the name and other attributes of a pipeline
  delete  Delete a pipeline
  run     Run a pipeline
  list    List all pipelines
  dag     Generate a dot or mermaid diagram for the pipeline
  export  Export the pipeline to a YAML or JSON file to edit
  import  Import the pipeline from a file
  step    Step creation, dependency, output commands
  help    Print this message or the help of the given subcommand(s)

Options:
  -p, --pipeline-name <PIPELINE_NAME>  Name of the pipeline this command applies to
  -h, --help                           Print help

```
