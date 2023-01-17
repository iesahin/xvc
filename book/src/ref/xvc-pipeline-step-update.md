# xvc pipeline step update

## Purpose

Update the name, running condition, or command of a step.

## Synopsis

```console
$ xvc pipeline step update --help
Update step options

Usage: xvc pipeline step update [OPTIONS] --step-name <STEP_NAME>

Options:
  -s, --step-name <STEP_NAME>  Name of the step to update. The step should already be defined
  -c, --command <COMMAND>      Step command to run
      --when <WHEN>            When to run the command. One of always, never, by_dependencies (default). This is used to freeze or invalidate a step manually
  -h, --help                   Print help

```

## Examples

## Caveats
