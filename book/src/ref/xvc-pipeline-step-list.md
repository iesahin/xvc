# xvc pipeline step list

## Purpose

List the steps and their commands in a pipeline

## Synopsis

```console
$ xvc pipeline step list --help
List steps in a pipeline

Usage: xvc pipeline step list [OPTIONS]

Options:
      --names-only  Show only the names, otherwise print commands as well
  -h, --help        Print help

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
hello: echo hello (by_dependencies)
world: echo world (always)

```

It will list the commands and when they will run (always, never, by_dependencies) by default. If you only need the names of steps, you can use `--names-only` flag. 


```console
$ xvc pipeline step list --names-only
world
hello

```
