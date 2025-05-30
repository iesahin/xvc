# xvc file share

## Synopsis

```console
$ xvc file share --help
Share a file from (S3 compatible) storage for a limited time

Usage: xvc file share [OPTIONS] --storage <STORAGE> <TARGET>

Arguments:
  <TARGET>  File to send/push/upload to storage

Options:
  -s, --storage <STORAGE>    Storage name or guid to send the files
  -d, --duration <DURATION>  Period to send the files to. You can use s, m, h, d, w suffixes [default: 24h]
  -h, --help                 Print help

```

# Examples

This command requires an Xvc repository to share files from S3 and compatible storages.

```console
$ git init
Initialized empty Git repository in [CWD]/.git/

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

You must first send files to the remote storage.

```console
$ xvc file send --storage backup dir-0001/
```

Now you can share the files. It will create a URL for you to share that file. (Here we use cut to make the command repeatable)

```console
$ zsh -cl 'xvc file share --storage backup dir-0001/file-0001.bin | cut -c -50'
https://xvc-test.s3.eu-central-1.amazonaws.com/xvc

```

Note that the default period is 24 hours. You can set another period with `--duration`.

```console
$ zsh -cl 'xvc file share --duration 1h --storage backup dir-0001/file-0002.bin | cut -c -50'
https://xvc-test.s3.eu-central-1.amazonaws.com/xvc

```

You can get another URL for a shared file with a different period.

```console
$ zsh -cl 'xvc file share --duration 1m --storage backup dir-0001/file-0002.bin | cut -c -50'
https://xvc-test.s3.eu-central-1.amazonaws.com/xvc

```

See [humantime duration parsing
documentation](https://docs.rs/humantime/latest/humantime/fn.parse_duration.html)
for duration expressions.
