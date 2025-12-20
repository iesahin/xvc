# xvc storage new minio

## Purpose

Create a new Xvc storage on a [MinIO](https://min.io) instance.
It allows to store tracked file contents in a Minio server.

## Synopsis

```console,ignore
$ xvc storage new minio --help
Add a new Minio storage

Reads credentials from `MINIO_ACCESS_KEY_ID` and `MINIO_SECRET_ACCESS_KEY` environment variables. Alternatively you can use `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>` and `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>` environment variables if you have multiple storages of this type.

Usage: xvc storage new minio [OPTIONS] --name <NAME> --endpoint <ENDPOINT> --bucket-name <BUCKET_NAME> --region <REGION>

Options:
  -n, --name <NAME>
          Name of the storage

          This must be unique among all storages of the project

      --endpoint <ENDPOINT>
          Minio server url in the form https://myserver.example.com:9090

      --bucket-name <BUCKET_NAME>
          Bucket name

      --region <REGION>
          Region of the server

      --storage-prefix <STORAGE_PREFIX>
          You can set a directory in the bucket with this prefix

          [default: ]

  -h, --help
          Print help (see a summary with '-h')

```

## Examples

Before calling any commands that use this storage, you must set the following environment variables.

- `MINIO_ACCESS_KEY_ID` or `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>`: The access key of the Digital Ocean
  account. The second form is used when you have multiple Digital Ocean accounts and you want to use a specific one.
- `MINIO_SECRET_ACCESS_KEY` or `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>`: The secret key of the Digital
  Ocean account. The second form is used when you have multiple Digital Ocean accounts and you want to use a specific
  one.

The command works only in Xvc repositories.

```console,ignore
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

```console,ignore
$ xvc file track dir-0001
```

You can define a storage bucket as storage and begin to use it.

```console,ignore
$ xvc storage new minio --name backup --endpoint http://e1.xvc.dev:9000 --bucket-name xvc-tests --region us-east-1 --storage-prefix xvc

```

Send files to this storage.

```console,ignore
$ xvc file send dir-0001 --to backup

```

You can remove the files you sent from your cache and workspace.

```console,ignore
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

```console,ignore
$ xvc file bring --from backup dir-0001

$ tree dir-0001
dir-0001
├── file-0001.bin
├── file-0002.bin
└── file-0003.bin

1 directory, 3 files

```

If you want to remove a file and all of its versions from a storage, you can use `xvc file remove` command.

```console,ignore
$ xvc file remove --from-storage backup dir-0001/

```

## Caveats

`--name NAME` is not verified to be unique but you should use unique storage names to refer them later.
You can also use storage GUIDs listed by `xvc storage list` to refer to storages.

You must have a valid connection to the server.

Xvc uses Minio API port (9001, by default) to connect to the server.
Ensure that it's accessible.

For reasons caused from the underlying library, Xvc tries to connect `http://xvc-bucket.example.com:9001` if you give `http://example.com:9001` as the endpoint, and `xvc-bucket` as the bucket name.
You may need to consider this when you have servers running in exact URLs.
If you have a `http://minio.example.com:9001` as a Minio server, you may want to supply `http://example.com:9001` as the endpoint, and `minio` as the bucket name to form the correct URL.
This behavior may change in the future.

## Technical Details

This command requires Xvc to be compiled with `minio` feature, which is _on_ by default.
It uses Rust async features via `rust-s3` crate, and may add some bulk to the binary.
If you want to compile Xvc without these features, please refer to [How to Compile Xvc](/how-to/compile.md) document.

The command creates `.xvc-guid` file in `http://{{BUCKET-NAME}}.{{ENDPOINT}}/{{STORAGE-PREFIX}}/.xvc-guid`.
The file contains the unique identifier for this storage.
The same identifier is also recorded to the project.

A file that's found in `.xvc/{{HASH_PREFIX}}/{{CACHE_PATH}}` is saved to `http://{{BUCKET-NAME}}.{{ENDPOINT}}/{{STORAGE-PREFIX}}/{{REPO_ID}}/{{HASH_PREFIX}}/{{CACHE_PATH}}`.
`{{REPO_ID}}` is the unique identifier for the repository created during `xvc init`.
Hence if you use a common storage for different Xvc projects, their files are kept under different directories.
There is no inter-project deduplication.
