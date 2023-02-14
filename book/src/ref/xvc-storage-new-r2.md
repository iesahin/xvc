# xvc storage new r2

## Purpose

Use [Cloudflare R2](https://www.cloudflare.com/products/r2/) as an Xvc storage.

## Synopsis

```console
$ xvc storage new r2 --help
Add a new R2 storage

Reads credentials from `R2_ACCESS_KEY_ID` and `R2_SECRET_ACCESS_KEY` environment variables. Alternatively you can use `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>` and `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>` environment variables if you have multiple storages of this type.

Usage: xvc storage new r2 [OPTIONS] --name <NAME> --account-id <ACCOUNT_ID> --bucket-name <BUCKET_NAME>

Options:
  -n, --name <NAME>
          Name of the storage

          This must be unique among all storages of the project

      --account-id <ACCOUNT_ID>
          R2 account ID

      --bucket-name <BUCKET_NAME>
          Bucket name

      --storage-prefix <STORAGE_PREFIX>
          You can set a directory in the bucket with this prefix

          [default: ]

  -h, --help
          Print help (see a summary with '-h')

```

## Examples



Before calling any commands that use this storage, you must set the following environment variables.

- `R2_ACCESS_KEY_ID` or `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>`: The access key of the Cloudflare R2
  account. The second form is used when you have multiple accounts and you want to use a specific one.
- `R2_SECRET_ACCESS_KEY` or `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>`: The secret key of the Cloudfare R2 account. The second form is used when you have multiple accounts and you want to use a specific
  one.

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
$ xvc storage new r2 --name backup --bucket-name xvc-test --account-id e5dcca29209558eb9de6c07ae53b0a6f --storage-prefix xvc-storage

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

Then get back them from the storage.

```console
$ xvc file bring --from backup dir-0001

$ tree dir-0001
dir-0001
├── file-0001.bin
├── file-0002.bin
└── file-0003.bin

1 directory, 3 files

```

If you want to remove a file and all of its versions from a storage, you can use `xvc file remove` command.

```console
$ xvc file remove --from-storage backup dir-0001/

```
