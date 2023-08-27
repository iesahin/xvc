# xvc pipeline import

## Synopsis

```console
$ xvc pipeline import --help
Import the pipeline from a file

Usage: xvc pipeline import [OPTIONS]

Options:
  -n, --name <NAME>      Name of the pipeline to import. If not set, the name from the file is used
      --file <FILE>      File to read the pipeline. Use stdin if not specified
      --format <FORMAT>  Input format. One of json or yaml. If not set, the format is guessed from the file extension. If the file extension is not set, json is used as default
      --overwrite        Overwrite the pipeline even if the name already exists
  -h, --help             Print help

```

# Examples

This command is used to import pipelines exported with [`xvc pipeline export`](/ref/xvc-pipeline-export.md).

You can edit and import the pipelines exported with the command.

```admonition warning
Xvc doesn't guarantee that the format of these files will be compatible across versions. You can use these files to share pipeline definitions but it may not be a good way to store pipeline definitions for longer periods.
```

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

The following file generated with `xvc pipeline export`.

```console
$ cat pipeline.yaml
```

You can import this file to construct the pipeline at once.

```console
$ xvc pipeline import --file pipeline.yaml

$ xvc pipeline step list
```


