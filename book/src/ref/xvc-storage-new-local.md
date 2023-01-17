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

Create a new Xvc backup storage on a path

```shell
$ xvc storage new-local --name backup --path /media/bigdisk/backups/my-project-xvc
```

## Caveats

`--name NAME` is not checked to be unique but you should use unique storage names to refer them later. 

`--path PATH`  should be accessible for writing and shouldn't already exist.


## Technical Details

The command creates the `PATH` and a new file under `PATH` called `.xvc-guid`. 
The file contains the unique identifier for this storage. 
The same identifier is also recorded to the project. 

A file that's found in `.xvc/{{HASH_PREFIX}}/{{CACHE_PATH}}` is saved to `PATH/{{REPO_ID}}/{{HASH_PREFIX}}/{{CACHE_PATH}}`. 
`{{REPO_ID}}` is the unique identifier for the repository created during `xvc init`. 
Hence if you use a common storage for different Xvc projects, their files are kept under different directories. 
There is no inter-project deduplication.
