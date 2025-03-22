# xvc storage new local

## Purpose

Create a new storage reachable from the local filesystem.
It allows to keep tracked file contents in a different directory for backup or sharing purposes.

## Synopsis

```console
$ xvc storage new local --help
Add a new local storage

A local storage is a directory accessible from the local file system. Xvc will use common file operations for this directory without accessing the network.

Usage: xvc storage new local --path <PATH> --name <NAME>

Options:
      --path <PATH>
          Directory (outside the repository) to be set as a storage

  -n, --name <NAME>
          Name of the storage.
          
          Recommended to keep this name unique to refer easily.

  -h, --help
          Print help (see a summary with '-h')

```

## Examples

The command works only in Xvc repositories.

```console
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

```console
$ xvc file track dir-0001
```

Now, you can define a local directory as storage and begin to use it.

```console
$ xvc storage new local --name backup --path '../my-local-storage'

```

Send files to this storage.

```console
$ xvc file send dir-0001 --to backup

```

You can remove the files you sent from your cache and workspace.

```console
$ xvc file remove --from-cache dir-0001/
[DELETE] [CWD]/.xvc/b3/3c6/70f/e91055c2be2e87890dba1e952d656d1e70dd196bf5530d379243c6e4aa/0.bin
[DELETE] [CWD]/.xvc/b3/7aa/354/0225bd33702c239454b63b31d1ea25721cbbfb491d6139d0b85b82d15d/0.bin
[DELETE] [CWD]/.xvc/b3/d7d/629/677c6d8df55ab3a1d694453c59f3ca0df494d3dc190aeef1e00abd96eb/0.bin

$ rm -rf dir-0001/
```

Then get back them from the storage.

```console
$ xvc file bring --from backup dir-0001

$ tree dir-0001
dir-0001
├── file-0001.bin
├── file-0002.bin
└── file-0003.bin

1 directory, 3 files

```

If you want to remove a file and all of its versions from a storage, you can use `xvc file remove` command.

```console
$ xvc file remove --from-storage backup dir-0001/

```

## Caveats

`--name NAME` is not checked to be unique but you should use unique storage names to refer them later.

`--path PATH` should be accessible for writing and shouldn't already exist.

## Technical Details

The command creates the `PATH` and a new file under `PATH` called `.xvc-guid`.
The file contains the unique identifier for this storage.
The same identifier is also recorded to the project.

A file that's found in `.xvc/{{HASH_PREFIX}}/{{CACHE_PATH}}` is saved to `PATH/{{REPO_ID}}/{{HASH_PREFIX}}/{{CACHE_PATH}}`.
`{{REPO_ID}}` is the unique identifier for the repository created during `xvc init`.
Hence if you use a common storage for different Xvc projects, their files are kept under different directories.
There is no inter-project deduplication. (yet)

In the future, there may be an option to have a common storage for multiple projects at the same location. Please
comment below if this is a common use case.
