# xvc storage new generic

## Purpose

Create a new storage that uses shell commands to send and retrieve cache files. 
It allows to keep tracked files in any kind of service that can be used command line.

## Synopsis 

```console
$ xvc storage new generic --help
add a new generic remote

Usage: xvc storage new generic [OPTIONS] --name <NAME> --init <INIT_COMMAND> --list <LIST_COMMAND> --download <DOWNLOAD_COMMAND> --upload <UPLOAD_COMMAND> --delete <DELETE_COMMAND>

Options:
  -n, --name <NAME>
          Name of the remote
          
          This must be unique among all remotes of the project

  -i, --init <INIT_COMMAND>
          Command to initialize the remote. This command is run once after defining the remote.
          
          You can use {URL} and {DIR}  as shortcuts.

  -l, --list <LIST_COMMAND>
          Command to list the files in remote
          
          You can use {URL} and {DIR} placeholders and define values for these with --url and --dir options.

  -d, --download <DOWNLOAD_COMMAND>
          Command to download a file from remote.
          
          You can use {URL} and {DIR} placeholders and define values for these with --url and --dir options.

  -u, --upload <UPLOAD_COMMAND>
          Command to upload a file to remote.
          
          You can use {URL} and {DIR} placeholders and define values for these with --url and --dir options.

  -D, --delete <DELETE_COMMAND>
          The delete command to remove a file from remote You can use {URL} and {DIR} placeholders and define values for these with --url and --dir options

  -M, --processes <MAX_PROCESSES>
          Number of maximum processes to run simultaneously
          
          [default: 1]

      --url <URL>
          You can set a string to replace {URL} placeholder in commands

      --storage-dir <STORAGE_DIR>
          You can set a string to replace {DIR} placeholder in commands

  -h, --help
          Print help information (use `-h` for a summary)

```

You can use the following placeholders in your commands. 
These are replaced with the actual paths in runtime and commands are run with concrete paths. 

- `{URL}` : The content of `--url` option. (default "")
- `{STORAGE_DIR}` Content of `--storage-dir`  option. (default "")
- `{RELATIVE_CACHE_PATH}` The portion of the cache path after `.xvc/`. 
- `{ABSOLUTE_CACHE_PATH}` The absolute local path for the cache element. 
- `{RELATIVE_CACHE_DIR}` The portion of directory that contains the file after `.xvc/`.
- `{ABSOLUTE_CACHE_DIR}` The portion of the local directory that contains the file after `.xvc`.
- `{XVC_GUID}`: Repository GUID used in storages to differ repository elements
- `{FULL_STORAGE_PATH}`: Concatenation of `{URL}{STORAGE_DIR}{XVC_GUID}/{RELATIVE_CACHE_PATH}` 
- `{FULL_STORAGE_DIR}`: Concatenation of `{URL}{STORAGE_DIR}{XVC_GUID}/{RELATIVE_CACHE_DIR}`
- `{LOCAL_GUID_FILE_PATH}`: The path that contains guid of the storage locally. Used only in `--init` option. 
- `{STORAGE_GUID_FILE_PATH}`: The path that should have guid of the storage, in storage. Used only in `--init` option. 

## Examples

### Create a generic storage in the same filesystem

You can create a storage that's using shell commands to send and receive files to another location in the file system. 

There are two variables that you can use in the commands. 
For a storage in the same file system, `--url` could be blank and `--storage-dir` could be the location you want to define. 

```shell
$ xvc storage new-generic
    --url ""
    --storage-dir $HOME/my-xvc-storage
    ...
```

You need to specify the commands for the following operations: 

- `init`: The command that's used to create the directory that will be used as a storage. 
It should also copy `XVC_STORAGE_GUID_FILENAME` (currently `.xvc-guid`) to that location. 
This file is used to identify the location as an Xvc storage. 

```shell
$ xvc storage new-generic
      ...
      --init 'mkdir -p {STORAGE_DIR} ; cp {LOCAL_GUID_FILE_PATH} {STORAGE_GUID_FILE_PATH}'
      ...
```

Note that if the command doesn't contain `{LOCAL_GUID_FILE_PATH}` and `{STORAGE_GUID_FILE_PATH}` variables, it won't be run and Xvc will report an error.

- `list`: This operation should list all files under `{URL}{STORAGE_DIR}`.
The list is filtered through a regex that matches the format of the paths. 
Hence, even the command lists all files in the storage, Xvc will consider only the relevant paths.

All paths should be listed in separate lines. 

```shell
$ xvc storage new-generic
        ...
        --list 'ls -1 {URL}{STORAGE_DIR}'
        ...
```

- `upload`: The command that will copy a file from local cache to the storage. 
Normally, it uses `{ABSOLUTE_CACHE_PATH}` variable.
For the local file system, we also need to create a directory before copying. 

```shell
$ xvc storage new-generic
     ...
     --upload 'mkdir -p {FULL_STORAGE_DIR} && cp {ABSOLUTE_CACHE_PATH} {FULL_STORAGE_PATH}'
     ...
```

- `download`: This command will be used to copy from storage to the local cache. 
It must create local cache directory as well. 

```shell
$ xvc storage new-generic
    ...
    --download 'mkdir -p {ABSOLUTE_CACHE_DIR} && cp {FULL_STORAGE_PATH} {ABSOLUTE_CACHE_PATH}'
    ...
```

- `delete`: This operation is used to delete the _storage_ file. 
It shouldn't touch the local file in any way, otherwise you may lose data. 

```shell
$ xvc storage new-generic
    ...
    --delete 'rm -f {FULL_STORAGE_PATH} ; rmdir {FULL_STORAGE_DIR}'
    ...
```

In total, the command you write is the following. 
It defines all operations of this storage. 

```shell
$ xvc storage new-generic
    --url ""
    --storage-dir $HOME/my-xvc-storage
    --init 'mkdir -p {STORAGE_DIR} ; cp {LOCAL_GUID_FILE_PATH} {STORAGE_GUID_FILE_PATH}'
    --list 'ls -1 {URL}{STORAGE_DIR}'
    --upload 'mkdir -p {FULL_STORAGE_DIR} && cp {ABSOLUTE_CACHE_PATH} {FULL_STORAGE_PATH}'
    --download 'mkdir -p {ABSOLUTE_CACHE_DIR} && cp {FULL_STORAGE_PATH} {ABSOLUTE_CACHE_PATH}'
    --delete 'rm -f {FULL_STORAGE_PATH} ; rmdir {FULL_STORAGE_DIR}'
```

### Create a storage using rclone

### Create a storage using rsync

Rsync is found for all popular platforms to copy file contents. 
Xvc can use it to maintain a storage if you already have a working rsync setup.

We need to define operations for `init`, `upload`, `download`, `list` and `delete` with rsync or ssh. 
Some of the commands need `ssh` to perform operations, like creating a directory. 
We'll use placeholders for paths. 

As rsync URL format is slightly different than SSH, we will define the commands verbosely. 

Suppose you want to use your account at `user@example.com` to store your Xvc files. 
You want to store the files under `/home/user/my-xvc-storage`. 

We assume you have configured public key authentication for your account. 
Xvc doesn't receive user input during storage operations, and can't receive your password during runs. 

We first define these as our `--url` and `--storage-dir` options. 

```shell
$ xvc --url user@example.com 
        --storage-dir '/home/user/my-xvc-storage'
        ...
```

Initialization command must create this directory and copy the storage GUID file to its respective location. 

```shell
$ xvc 
  ...
  --init "ssh {URL} 'mkdir -p {STORAGE_DIR}' ; rsync -av '{LOCAL_GUID_FILE_PATH}' '{URL}:{STORAGE_GUID_FILE_PATH}'"
```

Note the use of `:` in `rsync` command. 
As it doesn't support `ssh://` URLs currently, we are using a form that's compatible with both ssh and rsync as URL.
It may be possible to use `&&` between `ssh` and `rsync` commands, but if the first command fails (e.g. the directory already exists), we still want to copy the guid file. 


## Caveats

## Technical Details

The paths in `list` commands are filtered through a regex. 
They are matched against `{REPO_GUID}/{RELATIVE_CACHE_DIR}/0` pattern and only the `{RELATIVE_CACHE_DIR}` portion is reported.
Any line that doesn't conform to this pattern is ignored. 
You can any listing command that returns a recursive file list, and only the pattern matching elements are considered.
