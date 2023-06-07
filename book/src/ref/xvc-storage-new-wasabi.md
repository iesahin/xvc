# xvc storage new wasabi

## Purpose

Configure a [Wasabi](https://wasabi.com/) service as an Xvc storage.

## Synopsis

```console
$ xvc storage new wasabi --help
Add a new Wasabi storage

Reads credentials from `WASABI_ACCESS_KEY_ID` and `WASABI_SECRET_ACCESS_KEY` environment variables. Alternatively you can use `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>` and `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>` environment variables if you have multiple storages of this type.

Usage: xvc storage new wasabi [OPTIONS] --name <NAME> --bucket-name <BUCKET_NAME>

Options:
  -n, --name <NAME>
          Name of the storage

          This must be unique among all storages of the project

      --bucket-name <BUCKET_NAME>
          Bucket name

      --endpoint <ENDPOINT>
          Endpoint for the server, complete with the region if there is

          e.g. for eu-central-1 region, use s3.eu-central-1.wasabisys.com as the endpoint.

          [default: s3.wasabisys.com]

      --storage-prefix <STORAGE_PREFIX>
          You can set a directory in the bucket with this prefix

          [default: ]

  -h, --help
          Print help (see a summary with '-h')

```

## Examples


Before calling any commands that use this storage, you must set the following environment variables.

- `WASABI_ACCESS_KEY_ID` or `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>`: The access key of the Wasabi
  account. The second form is used when you have multiple storages with different access keys.
- `WASABI_SECRET_ACCESS_KEY` or `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>`: The secret key of the Wasabi account. The second form is used when you have multiple storages with different access keys.

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
$ xvc storage new wasabi --name backup --bucket-name xvc-test --endpoint s3.wasabisys.com --storage-prefix xvc-storage

```

Send files to this storage.

```console
$ xvc file send dir-0001 --to backup

```

You can remove the files you sent from your cache and workspace.

```console
$ xvc file remove --from-cache dir-0001/
[DELETE] [CWD]/.xvc/b3/1bc/b82/80fcea6acf2362a4ec4ef8512fe2f791f412fed1635009293abedcad88/0.bin
[DELETE] [CWD]/.xvc/b3/1bc/b82/80fcea6acf2362a4ec4ef8512fe2f791f412fed1635009293abedcad88
[DELETE] [CWD]/.xvc/b3/1bc/b82
[DELETE] [CWD]/.xvc/b3/1bc
[DELETE] [CWD]/.xvc/b3/863/86d/62e50462e37699d86e9b436526cb3fe40c66e38030e4e25ae4e168193a/0.bin
[DELETE] [CWD]/.xvc/b3/863/86d/62e50462e37699d86e9b436526cb3fe40c66e38030e4e25ae4e168193a
[DELETE] [CWD]/.xvc/b3/863/86d
[DELETE] [CWD]/.xvc/b3/863
[DELETE] [CWD]/.xvc/b3/f60/f11/901bf063f1448d095f336929929e153025a3ec238128a42ff6e5f080ef/0.bin
[DELETE] [CWD]/.xvc/b3/f60/f11/901bf063f1448d095f336929929e153025a3ec238128a42ff6e5f080ef
[DELETE] [CWD]/.xvc/b3/f60/f11
[DELETE] [CWD]/.xvc/b3/f60
[DELETE] [CWD]/.xvc/b3

$ rm -rf dir-0001/
```

Then get back them from storage.

```console
$ xvc file bring --from backup dir-0001

$ tree dir-0001
dir-0001
├── file-0001.bin
├── file-0002.bin
└── file-0003.bin

1 directory, 3 files

```

If you want to remove a file and all of its versions from storage, you can use `xvc file remove` command.

```console
$ xvc file remove --from-storage backup dir-0001/

```
