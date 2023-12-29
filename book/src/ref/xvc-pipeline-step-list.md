# xvc pipeline step list

## Purpose

List the steps and their commands in a pipeline

## Synopsis

```console
$ xvc pipeline step list --help
Add a new step

Usage: xvc pipeline step new [OPTIONS] --step-name <STEP_NAME> --command <COMMAND>

Options:
  -s, --step-name <STEP_NAME>  Name of the new step
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

You may want to list the steps of a pipeline and their commands. 

```console
$ xvc pipeline step new --step-name hello --command "echo hello"
$ xvc pipeline step new --step-name world --command "echo world" --when always
```


```console
$ xvc pipeline step list

```

It will list the commands and when they will run (always, never, by_dependencies) by default. If you only need the names of steps, you can use `--names-only` flag. 


```console
$ xvc pipeline step list --names-only
```
