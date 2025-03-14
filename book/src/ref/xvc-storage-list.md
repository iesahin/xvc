# xvc storage list

## Purpose

List all configured storages with their names and guids.

## Synopsis

```console
$ xvc storage list --help
List all configured storages

Usage: xvc storage list

Options:
  -h, --help  Print help

```

## Examples

The command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Define two local storages:

```console
$ xvc storage new local --name backup-1 --path '../backup-1'
$ xvc storage new local --name backup-2 --path '../backup-2'

```

You can list the storages and their GUIDs.

```console
$ xvc storage list
Local:   backup-1	675bc29a-24fc-40b1-af91-265f58c1b4fc	../backup-1

Local:   backup-2	305b79ef-78d1-44c3-97c4-4242c90cab15	../backup-2


```

## Caveats

This one uses the local configuration and doesn't try to connect storages.

If a storage is listed, it doesn't mean it's guaranteed to be able to pull or push. 

Xvc never stores credentials for storages. 

