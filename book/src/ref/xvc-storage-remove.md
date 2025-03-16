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
Local:   backup-1	631e1654-5c60-424b-b7ec-b75e4059fb2f	../backup-1

Local:   backup-2	3eb92021-4767-4807-8e25-ed311e972c48	../backup-2


```

Now when we remove `backup-1` and get the list, only one of them is listed.

```console
$ xvc storage remove --name backup-1
Removed Storage Local:   backup-1	631e1654-5c60-424b-b7ec-b75e4059fb2f	../backup-1

$ xvc storage list
Local:   backup-2	3eb92021-4767-4807-8e25-ed311e972c48	../backup-2


```

## Caveats

This one uses the local configuration and doesn't try to connect storages.

If a storage is listed, it doesn't mean it's guaranteed to be able to pull or push. 

Xvc never stores credentials for storages. 


