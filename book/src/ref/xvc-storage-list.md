# xvc storage list

## Purpose

List all configured storages with their names and guids. 

## Synopsis 

```console
$ xvc storage list --help
List all configured storages

Usage: xvc storage list

Options:
  -h, --help  Print help information
```

## Examples

List all storages in the repository:

```shell
$ xvc storage list 
```

## Caveats

This one uses the local configuration and doesn't try to connect storages.
If it's listed with the command, it doesn't mean it's guaranteed to be able to pull or push. 


