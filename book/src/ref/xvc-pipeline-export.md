# xvc pipeline export

## Synopsis

```console
$ xvc pipeline export --help
Export the pipeline to a YAML or JSON file to edit

Usage: xvc pipeline export [OPTIONS]

Options:
  -n, --name <NAME>      Name of the pipeline to export
      --file <FILE>      File to write the pipeline. Writes to stdout if not set
      --format <FORMAT>  Output format. One of json or yaml. If not set, the format is guessed from the file extension. If the file extension is not set, json is used as default
  -h, --help             Print help

```

# Examples

You can export the pipeline you created to a JSON or YAML file to edit and restore using [`xvc pipeline
import`](/ref/xvc-pipeline-import/). This allows to fix typos and update commands in place.

```admonition warning
Xvc doesn't guarantee that the format of these files will be compatible across versions. You can use these files to share pipeline definitions but it may not be a good way to store pipeline definitions for longer periods.
```

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Let's start by defining a steps in the pipeline.

```console
$ xvc pipeline step new --step-name step1 --command 'touch abc.txt'
$ xvc pipeline step new --step-name step2 --command 'touch def.txt'
```

Adding a few dependencies.

```console
$ xvc pipeline step dependency -s step2 --step step1
$ xvc pipeline step dependency -s step2 --glob '*.txt'
$ xvc pipeline step dependency -s step2 --glob-items'*.txt'
$ xvc pipeline step dependency -s step2 --param model.conv_units
$ xvc pipeline step dependency -s step2 --regex requirements.txt:/^tensorflow
$ xvc pipeline step dependency -s step2 --regex-items requirements.txt:/^tensorflow
$ xvc pipeline step dependency -s step2 --line-items params.yaml::1-20
$ xvc pipeline step dependency -s step2 --lines params.yaml::1-20
$ xvc pipeline step dependency -s step2 --url 'https://example.com'
$ xvc pipeline step dependency -s step2 --generic 'ping -c 2 example.com'
$ xvc pipeline step output -s step2 --output-metric metrics.json
$ xvc pipeline step output -s step2 --output-file def.txt
$ xvc pipeline step output -s step2 --output-image plots/confusion.png
```

If you don't specify a filename, the default format is JSON and the output will be sent to stdout.

```console
$ xvc pipeline export
```

If you want to set the format, you can specify the `--format` option.

```console
$ xvc pipeline export --format yaml
```

When you specify a file name, the output format is inferred from the extension.

```console
$ xvc pipeline export --file pipeline.yaml

$ cat pipeline.yaml
```
