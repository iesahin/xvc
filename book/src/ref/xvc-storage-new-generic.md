# xvc remote new-generic

## Purpose

Create a new remote that uses shell commands to store and retrieve cache files. 
It allows to keep tracked files in any kind of service that allows to command line access. 

## Synopsis 

```text
{{#include xvc-remote-new-generic.txt}}
```

You can use the following placeholders in your commands. 
These are replaced with the actual paths in runtime and commands are run with concrete paths. 

- `{URL}` : The content of `--url` option. (default "")
- `{REMOTE_DIR}` Content of `--remote-dir`  option. (default "")
- `{RELATIVE_CACHE_PATH}` The portion of the cache path after `.xvc/`. 
- `{ABSOLUTE_CACHE_PATH}` The absolute local path for the cache element. 
- `{RELATIVE_CACHE_DIR}` The portion of directory that contains the file after `.xvc/`.
- `{ABSOLUTE_CACHE_DIR}` The portion of the local directory that contains the file after `.xvc`.
- `{XVC_GUID}`: Repository GUID used in remotes to differ repository elements
- `{FULL_REMOTE_PATH}`: Concatenation of `{URL}{REMOTE_DIR}{XVC_GUID}/{RELATIVE_CACHE_PATH}` 
- `{FULL_REMOTE_DIR}`: Concatenation of `{URL}{REMOTE_DIR}{XVC_GUID}/{RELATIVE_CACHE_DIR}`
- `{LOCAL_GUID_FILE_PATH}`: The path that contains guid of the remote locally. Used only in `--init` option. 
- `{REMOTE_GUID_FILE_PATH}`: The path that should have guid of the remote, in remote. Used only in `--init` option. 

## Examples

### Create a generic remote in the same filesystem

You can create a remote that's using shell commands to store files in another location in the file system. 

There are two variables that you can use in the commands. 
For a remote in the same file system, `--url` could be blank and `--remote-dir` could be the location you want to define. 

```shell
$ xvc remote new-generic
    --url ""
    --remote-dir $HOME/my-xvc-remote
    ...
```

You need to specify the commands for the following operations: 

- `init`: The command that's used to create the directory that will be used as a remote. 
It should also copy `XVC_REMOTE_GUID_FILENAME` (currently `.xvc-guid`) to that location. 
This file is used to identify the location as an Xvc remote. 

```shell
$ xvc remote new-generic
      ...
      --init 'mkdir -p {REMOTE_DIR} ; cp {LOCAL_GUID_FILE_PATH} {REMOTE_GUID_FILE_PATH}'
      ...
```

Note that if the command doesn't contain `{LOCAL_GUID_FILE_PATH}` and `{REMOTE_GUID_FILE_PATH}` variables, the command won't be run. 

- `list`: This operation should list all files under `{URL}{REMOTE_DIR}`.
The list is filtered through a regex that matches the repository and format of the paths. 
All paths should be listed in separate lines. 

```shell
$ xvc remote new-generic
        ...
        --list 'ls -1 {URL}{REMOTE_DIR}'
        ...
```

- `upload`: The command that will copy a file from local cache to the remote. 
Normally it use `{ABSOLUTE_CACHE_PATH}` variable.
For the local file system, we also need to create a directory before copying. 

```shell
$ xvc remote new-generic
     ...
     --upload 'mkdir -p {FULL_REMOTE_DIR} && cp {ABSOLUTE_CACHE_PATH} {FULL_REMOTE_PATH}'
     ...
```

- `download`: This command will be used to copy from remote to the local cache. 
It must create local cache directory as well. 

```shell
$ xvc remote new-generic
    ...
    --download 'mkdir -p {ABSOLUTE_CACHE_DIR} && cp {FULL_REMOTE_PATH} {ABSOLUTE_CACHE_PATH}'
    ...
```

- `delete`: This operation is used to delete the _remote_ file. 
It shouldn't touch the local file in any way, otherwise you may lose data. 

```shell
$ xvc remote new-generic
    ...
    --delete 'rm -f {FULL_REMOTE_PATH} ; rmdir {FULL_REMOTE_DIR}'
    ...
```

In total, the command you write is the following. 
It defines all operations of this remote. 

```shell
$ xvc remote new-generic
    --url ""
    --remote-dir $HOME/my-xvc-remote
    --init 'mkdir -p {REMOTE_DIR} ; cp {LOCAL_GUID_FILE_PATH} {REMOTE_GUID_FILE_PATH}'
    --list 'ls -1 {URL}{REMOTE_DIR}'
    --upload 'mkdir -p {FULL_REMOTE_DIR} && cp {ABSOLUTE_CACHE_PATH} {FULL_REMOTE_PATH}'
    --download 'mkdir -p {ABSOLUTE_CACHE_DIR} && cp {FULL_REMOTE_PATH} {ABSOLUTE_CACHE_PATH}'
    --delete 'rm -f {FULL_REMOTE_PATH} ; rmdir {FULL_REMOTE_DIR}'
```

### Create a remote using rclone

### Create a remote using rsync

Rsync is found for all popular platforms to copy file contents. 
Xvc can use it to maintain a remote if you already have a working rsync setup.

We need to define operations for `init`, `upload`, `download`, `list` and `delete` with rsync or ssh. 
Some of the commands need `ssh` to perform operations, like creating a directory. 
We'll use placeholders for paths. 

As rsync URL format is slightly different than SSH, we will define the commands verbosely. 

Suppose you want to use your account at `user@example.com` to store your Xvc files. 
You want to store the files under `/home/user/my-xvc-remote`. 

We assume you have configured public key authentication for your account. 
Xvc doesn't receive user input during remote operations, and can't receive your password during runs. 

We first define these as our `--url` and `--remote-dir` options. 

```shell
$ xvc --url user@example.com 
        --remote-dir '/home/user/my-xvc-remote'
        ...
```

Initialization command must create this directory and copy the remote GUID file to its respective location. 

```shell
$ xvc 
  ...
  --init "ssh {URL} 'mkdir -p {REMOTE_DIR}' ; rsync -av '{LOCAL_GUID_FILE_PATH}' '{URL}:{REMOTE_GUID_FILE_PATH}'"
```

Note the use of `:` in `rsync` command. 
As it doesn't support `ssh://` URLs currently, we are using a form that's compatible with both ssh and rsync as URL.
It may be possible to use `&&` between `ssh` and `rsync` commands, but if the first command fails (e.g. the directory already exists), we still want to copy the guid file. 




### Create a remote with s5cmd

## Caveats

## Technical Details

The paths in `list` commands are filtered through a regex. 
They are matched against `{REPO_GUID}/{RELATIVE_CACHE_DIR}/0` pattern and only the `{RELATIVE_CACHE_DIR}` portion is reported.
Any line that doesn't conform to this pattern is ignored. 
You can any listing command that returns a recursive file list, and only the pattern matching elements are considered.
