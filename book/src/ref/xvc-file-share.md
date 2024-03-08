# xvc file send

## Synopsis

```console
$ xvc file share --help
Share a file from S3 compatible storage for a limited time

Usage: xvc file share [OPTIONS] --remote <REMOTE> <TARGET>

Arguments:
  <TARGET>  File to send/push/upload to storage

Options:
  -r, --remote <REMOTE>      Storage name or guid to send the files
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
$ xvc file send --remote backup dir-0001/
```

Now you can share the files. It will create a URL for you to share that file.

```console
$ xvc file share --remote backup dir-0001/file-0001.bin
https://xvc-test.s3.eu-central-1.amazonaws.com/xvc-test/xvc-storage/9aecc4d6-3c72-4318-898c-82458b46efd1/b3/7ae/8aa/161dbc0ead08a5ba0389baaa7fdd9cc1c5406b11539c2fbee2d4be83ac/0.bin?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAJOXNDTB55QE3GOEQ%2F20240308%2Feu-central-1%2Fs3%2Faws4_request&X-Amz-Date=20240308T111946Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Signature=5b975934bc9b87f6f61e453de444e45714f8aa2f336fc8963eaa27811bacf0af

```

Note that the default period is 24 hours. You can set another period with `--duration`.

```console
$ xvc file share --duration 1h --remote backup dir-0001/file-0002.bin
https://xvc-test.s3.eu-central-1.amazonaws.com/xvc-test/xvc-storage/9aecc4d6-3c72-4318-898c-82458b46efd1/b3/390/756/dfbe0fab0e46c8a4e2eeecfda15b3a22b20b0582f590867062b1a49b2a/0.bin?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAJOXNDTB55QE3GOEQ%2F20240308%2Feu-central-1%2Fs3%2Faws4_request&X-Amz-Date=20240308T111946Z&X-Amz-Expires=3600&X-Amz-SignedHeaders=host&X-Amz-Signature=0c6dfb51c7e2c8ee5c043e0699c64c121c347637c64df2e54aee4d129099a55b

```

You can get another URL for a shared file with a different period.

```console
$ xvc file share --duration 1m --remote backup dir-0001/file-0002.bin
https://xvc-test.s3.eu-central-1.amazonaws.com/xvc-test/xvc-storage/9aecc4d6-3c72-4318-898c-82458b46efd1/b3/390/756/dfbe0fab0e46c8a4e2eeecfda15b3a22b20b0582f590867062b1a49b2a/0.bin?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAJOXNDTB55QE3GOEQ%2F20240308%2Feu-central-1%2Fs3%2Faws4_request&X-Amz-Date=20240308T111946Z&X-Amz-Expires=60&X-Amz-SignedHeaders=host&X-Amz-Signature=26c5fe21f917195518e9a5c01670dc589af8dfdc345a6400f8082da40b274294

```

See [humantime duration parsing
documentation](https://docs.rs/humantime/latest/humantime/fn.parse_duration.html)
for duration expressions.
