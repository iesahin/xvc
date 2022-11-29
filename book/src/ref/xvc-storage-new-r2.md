# xvc storage new r2

## Purpose

Configure [Cloudflare R2](https://www.cloudflare.com/products/r2/) as an Xvc storage. 

## Synopsis 

```console
$ xvc storage new r2 --help
Add a new R2 storage

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
          Print help information (use `-h` for a summary)

```

## Examples

