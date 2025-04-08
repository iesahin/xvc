# `xvc storage new rclone`

## Purpose

Adds a new storage configuration that uses an existing [Rclone](https://rclone.org/) remote.

This command leverages your system's `rclone` installation. Ensure `rclone` is
installed and the remote you intend to use is already configured via `rclone
config`.

## Synopsis

```console
$ xvc storage new rclone --help
Add a new rclone storage

Usage: xvc storage new rclone [OPTIONS] --name <NAME> --remote-name <REMOTE_NAME>

Options:
      --name <NAME>            Unique name for the storage
      --remote-name <REMOTE_NAME>  Name of the pre-configured rclone remote (e.g., `my-gdrive`, `s3-bucket`)
      --storage-prefix <STORAGE_PREFIX>
                               Optional path/prefix within the rclone remote to store data under [default: ]
  -h, --help                   Print help
```

## Examples

First, ensure you have rclone installed and a remote configured (e.g., named
my-cloud-drive via rclone config show).

The command works only in Xvc repositories. Initialize one if needed: 

```console
$ git init
Initialized empty Git repository in .../.git/
$ xvc init
INFO  Xvc project initialized in .../.xvc
```

Create some sample data using the test helper:

```console
$ xvc-test-helper create-directory-tree --directories 1 --files 3 --seed 20230211
$ tree dir-0001
dir-0001
├── file-0001.bin
├── file-0002.bin
└── file-0003.bin

1 directory, 3 files
```

Xvc only sends and receives tracked files. Track the sample directory:

```console
$ xvc file track dir-0001
INFO  Searching files finished. Found 3 file entries. Updated 0, added 3.
INFO  Hashing data finished. Processed 3 files. Added 3, updated 0. Total size ...

$ xvc file track dir-0001
INFO  Searching files finished. Found 3 file entries. Updated 0, added 3.
INFO  Hashing data finished. Processed 3 files. Added 3, updated 0. Total size ...
```

Now, you can define the configured rclone remote as an Xvc storage and begin to
use it. Let's name the Xvc storage cloud-storage and link it to the rclone
remote named my-cloud-drive.

```console
$ xvc storage new rclone --name cloud-storage --remote-name my-cloud-drive
INFO  Initialized:
my-cloud-drive:/.xvc-guid

INFO  Added Rclone Storage: XvcRcloneStorage { ... name: "cloud-storage", remote: "my-cloud-drive", ... }
```


Send the tracked files to this new storage:

```console
$ xvc file send dir-0001 --to cloud-storage
INFO  Searching files finished. Found 3 file entries. Updated 0, added 0.
INFO  Calculated send tasks. Files to send: 3
INFO  Processed 1/3: Sending -> dir-0001/file-0001.bin
INFO  Processed 2/3: Sending -> dir-0001/file-0002.bin
INFO  Processed 3/3: Sending -> dir-0001/file-0003.bin
INFO  Sending files finished. Processed: 3 files. Sent: 3 files. Skipped: 0 files. Failed: 0 files. Total size: ...

```


You can remove the files you sent from your local cache and workspace if you no longer need them locally:

```console
$ xvc file remove --from-cache dir-0001/
INFO  Searching files finished. Found 3 file entries. Updated 0, added 0.
INFO  Removing files from cache: 3. Total size: ...
[DELETE] [CWD]/.xvc/b3/.../.../0.bin
[DELETE] [CWD]/.xvc/b3/.../.../0.bin
[DELETE] [CWD]/.xvc/b3/.../.../0.bin

$ rm -rf dir-0001/
```

Then, get the files back from the rclone storage when needed:

```console
$ xvc file bring --from cloud-storage dir-0001
INFO  Searching files finished. Found 3 file entries. Updated 0, added 0.
INFO  Calculated bring tasks. Files to bring: 3. Total size: ...
INFO  Processed 1/3: Bringing -> dir-0001/file-0001.bin
INFO  Processed 2/3: Bringing -> dir-0001/file-0002.bin
INFO  Processed 3/3: Bringing -> dir-0001/file-0003.bin
INFO  Bringing files finished. Processed: 3 files. Brought: 3 files. Skipped: 0 files. Failed: 0 files. Total size: ...
INFO  Linking files finished. Processed: 3 files. Linked: 3 files. Skipped: 0 files. Failed: 0 files.

$ tree dir-0001
dir-0001
├── file-0001.bin
├── file-0002.bin
└── file-0003.bin

1 directory, 3 files

```

If you want to permanently remove a file and all of its versions from a
specific storage, you can use the xvc file remove --from-storage command:

```console
$ xvc file remove --from-storage cloud-storage dir-0001/
INFO  Searching files finished. Found 3 file entries. Updated 0, added 0.
INFO  Removing files from storage: 3. Total size: ...
INFO  Processed 1/3: Removing from storage -> dir-0001/file-0001.bin
INFO  Processed 2/3: Removing from storage -> dir-0001/file-0002.bin
INFO  Processed 3/3: Removing from storage -> dir-0001/file-0003.bin
```
