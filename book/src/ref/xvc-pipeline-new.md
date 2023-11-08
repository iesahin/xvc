# xvc pipeline new

## Synopsis

```console
$ xvc pipeline new --help
Create a new pipeline

Usage: xvc pipeline new [OPTIONS] --pipeline-name <PIPELINE_NAME>

Options:
  -p, --pipeline-name <PIPELINE_NAME>  Name of the pipeline this command applies to
  -w, --workdir <WORKDIR>              Default working directory
  -h, --help                           Print help

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
$ xvc pipeline new --pipeline-name my-pipeline
? 2
error: unexpected argument '--name' found

Usage: xvc pipeline new [OPTIONS] --pipeline-name <PIPELINE_NAME>

For more information, try '--help'.

```

By default it will run the commands in the repository root.

```console
$ xvc pipeline list
+---------+---------+
| Name    | Run Dir |
+===================+
| default |         |
+---------+---------+

```

If you want to define a pipeline specific to a directory, you can set the working directory.

```console
$ xvc-test-helper create-directory-tree --directories 1 --files 3  --seed 20230215
$ xvc pipeline new --pipeline-name another-pipeline --workdir dir-0001
? 2
error: unexpected argument '--name' found

Usage: xvc pipeline new [OPTIONS] --pipeline-name <PIPELINE_NAME>

For more information, try '--help'.

```

The pipeline will run the commands in the specified directory.

```console
$ xvc pipeline list
+---------+---------+
| Name    | Run Dir |
+===================+
| default |         |
+---------+---------+

```
