# xvc pipeline new

## Synopsis

```console
$ xvc pipeline new --help
Create a new pipeline

Usage: xvc pipeline new [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>        Name of the pipeline this command applies to
  -w, --workdir <WORKDIR>  Default working directory
  -h, --help               Print help

```

## Examples

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

You can create a new pipeline with a name.

```console
$ xvc pipeline new --name my-pipeline
```

By default it will run the commands in the repository root.

```console
$ xvc pipeline list
+-------------+---------+
| Name        | Run Dir |
+=======================+
| default     |         |
|-------------+---------|
| my-pipeline |         |
+-------------+---------+

```

If you want to define a pipeline specific to a directory, you can set the working directory.

```console
$ xvc-test-helper create-directory-tree --directories 1 --files 3  --seed 20230215
$ xvc pipeline new --name another-pipeline --workdir dir-0001
[ERROR] Pipeline Error: Xvc Core Error: ECS Error: Key is already in the store: my-pipeline

```

The pipeline will run the commands in the specified directory.

```console
$ xvc pipeline list
```
