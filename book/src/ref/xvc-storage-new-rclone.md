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

Uses the rclone configuration to connect to the storage. The remotestorage must already be configure with `rclone config`.

Usage: xvc storage new rclone [OPTIONS] --name <NAME> --remote-name <REMOTE_NAME>

Options:
  -n, --name <NAME>
          Name of the storage
          
          This must be unique among all storages of the project

      --remote-name <REMOTE_NAME>
          The name of the remote in rclone configuration
          
          This is the "remote" part in "remote://dir/" URL.

      --storage-prefix <STORAGE_PREFIX>
          The directory in the remote to store the files.
          
          This is the "dir" part in "remote://dir/" URL.
          
          [default: ]

  -h, --help
          Print help (see a summary with '-h')

```

## Examples

First, ensure you have rclone installed and a remote configured (e.g., named
my-cloud-drive via rclone config show).

The command works only in Xvc repositories. Initialize one if needed: 

```console
$ git init
Initialized empty Git repository in [CWD]/.git/

$ xvc init

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

$ xvc file track dir-0001

```

For the purposes of these examples, we'll create an alias to a temporary
directory for rclone remote storage. You must configure this with [rclone
config](https://rclone.org/commands/rclone_config/) before setting up the
storage.

First, we drop artifacts from the previous runs of these tests.

```console
$ zsh -c '[[ -d $TMPDIR/rclone-storage-for-xvc-test/ ]] && rm -rf $TMPDIR/rclone-storage-for-xvc-test'

$ zsh -c 'mkdir -p $TMPDIR/rclone-storage-for-xvc-test/'

$ rclone config delete my-rclone-remote

$ rclone config create my-rclone-remote alias remote=$TMPDIR/rclone-storage-for-xvc-test
[my-rclone-remote]
type = alias
remote = $TMPDIR/rclone-storage-for-xvc-test

```

Now, you can define the configured rclone remote as an Xvc storage and begin to
use it. Let's name the Xvc storage cloud-storage and link it to the rclone
remote named my-cloud-drive.

```console
$ xvc storage new rclone --name cloud-storage --remote-name my-rclone-remote

```


Send the tracked files to this new storage:

```console
$ xvc file send dir-0001 --to cloud-storage

```


You can remove the files you sent from your local cache and workspace if you no longer need them locally:

```console
$ xvc file remove --from-cache dir-0001/
[DELETE] [CWD]/.xvc/b3/3c6/70f/e91055c2be2e87890dba1e952d656d1e70dd196bf5530d379243c6e4aa/0.bin
[DELETE] [CWD]/.xvc/b3/7aa/354/0225bd33702c239454b63b31d1ea25721cbbfb491d6139d0b85b82d15d/0.bin
[DELETE] [CWD]/.xvc/b3/d7d/629/677c6d8df55ab3a1d694453c59f3ca0df494d3dc190aeef1e00abd96eb/0.bin

$ rm -rf dir-0001/
```

Then, get the files back from the rclone storage when needed:

```console
$ xvc file bring --from cloud-storage dir-0001

$ tree dir-0001
dir-0001

0 directories, 0 files

```

If you want to permanently remove a file and all of its versions from a
specific storage, you can use the xvc file remove --from-storage command:

```console
$ xvc file remove --from-storage cloud-storage dir-0001/

```
