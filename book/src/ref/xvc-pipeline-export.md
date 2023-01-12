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
  -h, --help             Print help information

```
