# xvc pipeline step dependency

## Purpose

Define a dependency to an existing step in the pipeline.

## Synopsis

```console
$ xvc pipeline step dependency --help
Add a dependency to a step in the pipeline

Usage: xvc pipeline step dependency [OPTIONS] --step-name <STEP_NAME>

Options:
  -s, --step-name <STEP_NAME>    Name of the step
      --file <FILES>             Add a file dependency to the step. Can be used multiple times.
      --quiet                    Suppress all output
      --step <STEPS>             Add explicit step dependencies to run
      --pipeline <PIPELINES>     Add explicit pipeline dependencies to run
      --directory <DIRECTORIES>  Add a directory dependency to the step. Can be used multiple times.
      --glob <GLOBS>             Add a glob dependency to the step. Can be used multiple times.
      --param <PARAMS>           Add a parameter dependency to the step in the form filename.yaml::model.units . Can be used multiple times.
      --regex <REGEXPS>          Add a regex dependency in the form filename.txt:/^regex/
      --line <LINES>             Add a line dependency in the form filename.txt::123-234
  -h, --help                     Print help information

```

## Examples

## Caveats
