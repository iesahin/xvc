# xvc storage new dropbox

## Purpose

Configure a Dropbox service as an Xvc storage.

## Synopsis

```console,ignore
$ xvc storage new dropbox --help
Add a new Dropbox storage

Reads credentials from `DROPBOX_ACCESS_TOKEN` environment variable. Alternatively you can use `XVC_STORAGE_DROPBOX_TOKEN_<storage_name>` environment variable if you have multiple storages of this type.

Usage: xvc storage new dropbox [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>
          Name of the storage
          
          This must be unique among all storages of the project

      --storage-prefix <STORAGE_PREFIX>
          You can set a directory in Dropbox with this prefix
          
          [default: ]

  -h, --help
          Print help (see a summary with '-h')

```

## Examples

Before calling any commands that use this storage, you must set the following environment variables.

- `DROPBOX_ACCESS_TOKEN` or `XVC_STORAGE_DROPBOX_TOKEN_<storage_name>`: The access token of the Dropbox account. The second form is used when you have multiple accounts and you want to use a specific one.

The command works only in Xvc repositories.

```console,ignore
$ git init
...
$ xvc init

$ xvc-test-helper create-directory-tree --directories 1 --files 3  --seed 20230211

$ tree dir-0001
dir-0001
├── file-0001.bin
├── file-0002.bin
└── file-0003.bin

1 directory, 3 files

```

Xvc only sends and receives tracked files.

```console,ignore
$ xvc file track dir-0001
```

You can define a Dropbox storage and begin to use it.

```console,ignore
$ xvc storage new dropbox --name backup --storage-prefix xvc-storage

```

Send files to this storage.

```console,ignore
$ xvc file send dir-0001 --to backup

```

You can remove the files you sent from your cache and workspace.

```console,ignore
$ xvc file remove --from-cache dir-0001/
...
```

Then get back them from the storage.

```console,ignore
$ xvc file bring --from backup dir-0001

$ tree dir-0001
dir-0001
├── file-0001.bin
├── file-0002.bin
└── file-0003.bin

1 directory, 3 files

```

If you want to remove a file and all of its versions from a storage, you can use `xvc file remove` command.

```console,ignore
$ xvc file remove --from-storage backup dir-0001/

```
