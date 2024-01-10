# xvc pipeline step remove

## Purpose

Remove a step and all its dependencies and outputs from the pipeline.

## Synopsis

```console
$ xvc pipeline step remove --help
Remove a step from a pipeline

Usage: xvc pipeline step remove --step-name <STEP_NAME>

Options:
  -s, --step-name <STEP_NAME>  Name of the step to remove
  -h, --help                   Print help

```

## Examples

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Let's create a few steps and make them depend on each other.
```console
$ xvc pipeline step new --step-name hello --command 'echo hello >> hello.txt'
? 2
error: a value is required for '--command <COMMAND>' but none was supplied

For more information, try '--help'.

$ xvc pipeline step new --step-name world --command 'echo world >> world.txt'
? 2
error: a value is required for '--command <COMMAND>' but none was supplied

For more information, try '--help'.

$ xvc pipeline step new --step-name from --command 'echo from >> from.txt'
? 2
error: a value is required for '--command <COMMAND>' but none was supplied

For more information, try '--help'.

$ xvc pipeline step new --step-name xvc --command 'echo xvc >> xvc.txt'
? 2
error: a value is required for '--command <COMMAND>' but none was supplied

For more information, try '--help'.

```
Let's specify the outputs as well.
```console
$ xvc pipeline step output --step-name hello --output-file hello.txt
? 2
error: unexpected argument '--output' found

  tip: a similar argument exists: '--output-file'

Usage: xvc pipeline step output <--step-name <STEP_NAME>|--output-file <FILES>|--output-metric <METRICS>|--output-image <IMAGES>>

For more information, try '--help'.

$ xvc pipeline step output --step-name world --output-file world.txt
? 2
error: unexpected argument '--output' found

  tip: a similar argument exists: '--output-file'

Usage: xvc pipeline step output <--step-name <STEP_NAME>|--output-file <FILES>|--output-metric <METRICS>|--output-image <IMAGES>>

For more information, try '--help'.

$ xvc pipeline step output --step-name from --output-file from.txt
? 2
error: unexpected argument '--output' found

  tip: a similar argument exists: '--output-file'

Usage: xvc pipeline step output <--step-name <STEP_NAME>|--output-file <FILES>|--output-metric <METRICS>|--output-image <IMAGES>>

For more information, try '--help'.

$ xvc pipeline step output --step-name xvc --output-file xvc.txt
? 2
error: unexpected argument '--output' found

  tip: a similar argument exists: '--output-file'

Usage: xvc pipeline step output <--step-name <STEP_NAME>|--output-file <FILES>|--output-metric <METRICS>|--output-image <IMAGES>>

For more information, try '--help'.

```

Now we can add dependencies between them.
```console
$ xvc pipeline step dependency --step-name xvc --step from
[ERROR] Pipeline Error: Step xvc not found in pipeline

$ xvc pipeline step dependency --step-name from --file world.txt
[ERROR] Pipeline Error: Step from not found in pipeline

$ xvc pipeline step dependency --step-name world --step hello
[ERROR] Pipeline Error: Step world not found in pipeline

```


Now the pipeline looks like this:
```console
$ xvc pipeline dag --format mermaid
flowchart TD


```

When we remove a step, all its dependencies and outputs are removed as well.
```console
$ xvc pipeline step remove --step-name from
[ERROR] Pipeline Error: Step from not found in pipeline

```

```console
$ xvc pipeline dag --format mermaid
flowchart TD


```

