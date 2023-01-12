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
  -h, --help             Print help information

```
