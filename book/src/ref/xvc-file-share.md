# xvc file send

## Synopsis

```console
$ xvc file share --help

```

# Examples

This command requires an Xvc repository to share files from S3 and compatible storages.

```console
$ git init

$ xvc init

$ xvc-test-helper create-directory-tree --directories 1 --files 3  --seed 20240228

$ tree dir-0001
dir-0001
├── file-0001.bin
├── file-0002.bin
└── file-0003.bin

1 directory, 3 files

```

You can share a file tracked by Xvc by first configuring an S3 compatible storage.

Xvc only sends and receives tracked files.

```console
$ xvc file track dir-0001
```

You can define a storage bucket as storage and begin to use it.

```console
$ xvc storage new s3 --name backup --bucket-name xvc-test --region eu-central-1 --storage-prefix xvc-storage

```

Now you can share the files. It will first send the file if not found, and will create a URL for you to share that file.

```console
$ xvc file share dir-0001/file-0001.bin

```

Note that the default period is 24 hours. You can set another period with `--period`.

```console
$ xvc file share --period 1h dir-0001/file-0002.bin
```

You can get another URL for a shared file with a different period.

```console
$ xvc file share --period 1m dir-0001/file-0002.bin
```

Note that `m` suffix refers to minute, not month. The suffixes for the command are selected to be unambiguous.
