# xvc pipeline step output

## Purpose

Define an output (file, metrics or plots) to an already existing step in the pipeline.

## Synopsis

```console
$ xvc pipeline step output --help
Add an output to a step

Usage: xvc pipeline step output [OPTIONS] --step-name <STEP_NAME>

Options:
  -s, --step-name <STEP_NAME>    Name of the step to add the output to
      --output-file <FILES>      Add a file output to the step. Can be used multiple times
      --output-metric <METRICS>  Add a metric output to the step. Can be used multiple times
      --output-image <IMAGES>    Add an image output to the step. Can be used multiple times
  -h, --help                     Print help information

```

## Examples

## Caveats
