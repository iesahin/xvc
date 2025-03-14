# xvc storage remove

## Purpose

Remove unused or inaccessible storages from the configuration

## Synopsis 

```console
$ xvc storage remove --help
Remove a storage configuration.

This doesn't delete any files in the storage.

Usage: xvc storage remove --name <NAME>

Options:
  -n, --name <NAME>
          Name of the storage to be deleted

  -h, --help
          Print help (see a summary with '-h')

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

Now when we remove `backup-1` and get the list, only one of them is listed.

```console
$ xvc storage remove --name backup-1
Removed Storage Local:   backup-1	675bc29a-24fc-40b1-af91-265f58c1b4fc	../backup-1

$ xvc storage list
Local:   backup-1	675bc29a-24fc-40b1-af91-265f58c1b4fc	../backup-1

Local:   backup-2	305b79ef-78d1-44c3-97c4-4242c90cab15	../backup-2


```

## Caveats

This one uses the local configuration and doesn't try to connect storages.

If a storage is listed, it doesn't mean it's guaranteed to be able to pull or push. 

Xvc never stores credentials for storages. 


