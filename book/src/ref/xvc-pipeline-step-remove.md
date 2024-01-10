# xvc pipeline step remove

## Purpose

Remove a step and all its dependencies and outputs from the pipeline.

## Synopsis

```console
$ xvc pipeline step remove --help
Update step options

Usage: xvc pipeline step update [OPTIONS] --step-name <STEP_NAME>

Options:
  -s, --step-name <STEP_NAME>  Name of the step to update. The step should already be defined
  -c, --command <COMMAND>      Step command to run
      --when <WHEN>            When to run the command. One of always, never, by_dependencies (default). This is used to freeze or invalidate a step manually
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
$ xvc pipeline step new --step-name hello --command "echo hello >> hello.txt
$ xvc pipeline step new --step-name world --command "echo world >> world.txt
$ xvc pipeline step new --step-name from --command "echo from >> from.txt
$ xvc pipeline step new --step-name xvc --command "echo xvc >> xvc.txt
```
Let's specify the outputs as well.
```console
$ xvc pipeline step output --step-name hello --output hello.txt
$ xvc pipeline step output --step-name world --output world.txt
$ xvc pipeline step output --step-name from --output from.txt
$ xvc pipeline step output --step-name xvc --output xvc.txt
```

Now we can add dependencies between them.
```console
$ xvc pipeline step dependency --step-name xvc --step from
$ xvc pipeline step dependency --step-name from --file world.txt
$ xvc pipeline step dependency --step-name world --step hello
```


Now the pipeline looks like this:
```console
$ xvc pipeline dag --format mermaid
```

When we remove a step, all its dependencies and outputs are removed as well.
```console
$ xvc pipeline step remove --step-name from
```

```console
$ xvc pipeline dag --format mermaid
```

