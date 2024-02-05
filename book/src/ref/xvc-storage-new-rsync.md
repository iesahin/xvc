# xvc storage new s3

## Purpose

Configure an S3 (or a compatible) service as an Xvc storage.

## Synopsis

```console
$ xvc storage new rsync --help
Add a new rsync storages

Uses rsync in separate processes to communicate. This can be used when you already have an SSH/Rsync connection. It doesn't prompt for any passwords. The connection must be set up with ssh keys beforehand.

Usage: xvc storage new rsync [OPTIONS] --name <NAME> --host <HOST> --storage-dir <STORAGE_DIR>

Options:
  -n, --name <NAME>
          Name of the storage.
          
          Recommended to keep this name unique to refer easily.

      --host <HOST>
          Hostname for the connection in the form host.example.com  (without @, : or protocol)

      --port <PORT>
          Port number for the connection in the form 22. Doesn't add port number to connection string if not given

      --user <USER>
          User name for the connection, the part before @ in user@example.com (without @, hostname). User name isn't included in connection strings if not given

      --storage-dir <STORAGE_DIR>
          storage directory in the host to store the files

  -h, --help
          Print help (see a summary with '-h')

```

## Examples

You must setup an SSH connection

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

You can define a storage bucket as storage and begin to use it.

```console
$ xvc storage new rsync --name backup --host one.emresult.com --storage-dir /tmp/xvc-backup/
? 2
error: unexpected argument '--bucket-name' found

Usage: xvc storage new rsync <--name <NAME>|--host <HOST>|--port <PORT>|--user <USER>|--storage-dir <STORAGE_DIR>>

For more information, try '--help'.

```

Send files to this storage.

```console
$ xvc file send dir-0001 --to backup
? 101
thread '<unnamed>' panicked at storage/src/storage/mod.rs:491:9:
Cannot find remote backup
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at lib/src/cli/mod.rs:267:52:
[PANIC] Cannot find remote backup [storage/src/storage/mod.rs::491]
thread 'main' panicked at lib/src/cli/mod.rs:410:37:
called `Result::unwrap()` on an `Err` value: Any { .. }

```

You can remove the files you sent from your cache and workspace.

```console
$ xvc file remove --from-cache dir-0001/
[DELETE] [CWD]/.xvc/b3/3c6/70f/e91055c2be2e87890dba1e952d656d1e70dd196bf5530d379243c6e4aa/0.bin
[DELETE] [CWD]/.xvc/b3/3c6/70f/e91055c2be2e87890dba1e952d656d1e70dd196bf5530d379243c6e4aa
[DELETE] [CWD]/.xvc/b3/3c6/70f
[DELETE] [CWD]/.xvc/b3/3c6
[DELETE] [CWD]/.xvc/b3/7aa/354/0225bd33702c239454b63b31d1ea25721cbbfb491d6139d0b85b82d15d/0.bin
[DELETE] [CWD]/.xvc/b3/7aa/354/0225bd33702c239454b63b31d1ea25721cbbfb491d6139d0b85b82d15d
[DELETE] [CWD]/.xvc/b3/7aa/354
[DELETE] [CWD]/.xvc/b3/7aa
[DELETE] [CWD]/.xvc/b3/d7d/629/677c6d8df55ab3a1d694453c59f3ca0df494d3dc190aeef1e00abd96eb/0.bin
[DELETE] [CWD]/.xvc/b3/d7d/629/677c6d8df55ab3a1d694453c59f3ca0df494d3dc190aeef1e00abd96eb
[DELETE] [CWD]/.xvc/b3/d7d/629
[DELETE] [CWD]/.xvc/b3/d7d
[DELETE] [CWD]/.xvc/b3

$ rm -rf dir-0001/
```

Then get back them from the storage.

```console
$ xvc file bring --from backup dir-0001
? 101
thread '<unnamed>' panicked at storage/src/storage/mod.rs:491:9:
Cannot find remote backup
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at lib/src/cli/mod.rs:267:52:
[PANIC] Cannot find remote backup [storage/src/storage/mod.rs::491]
thread 'main' panicked at lib/src/cli/mod.rs:410:37:
called `Result::unwrap()` on an `Err` value: Any { .. }

$ tree dir-0001
? 2
dir-0001  [error opening dir]

0 directories, 0 files

```

If you want to remove a file and all of its versions from a storage, you can use `xvc file remove` command.

```console
$ xvc file remove --from-storage backup dir-0001/
? 101
thread '<unnamed>' panicked at storage/src/storage/mod.rs:491:9:
Cannot find remote backup
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at lib/src/cli/mod.rs:267:52:
[PANIC] Cannot find remote backup [storage/src/storage/mod.rs::491]
thread 'main' panicked at lib/src/cli/mod.rs:410:37:
called `Result::unwrap()` on an `Err` value: Any { .. }

```
