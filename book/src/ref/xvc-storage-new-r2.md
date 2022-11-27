# xvc storage new r2

## Purpose

Configure [Cloudflare R2](https://www.cloudflare.com/products/r2/) as an Xvc storage. 

## Synopsis 

```console
$ xvc storage new r2 --help
Add a new R2 remote

Usage: xvc storage new r2 [OPTIONS] --name <NAME> --account-id <ACCOUNT_ID> --bucket-name <BUCKET_NAME>

Options:
  -n, --name <NAME>
          Name of the remote
          
          This must be unique among all remotes of the project

      --account-id <ACCOUNT_ID>
          R2 account ID

      --bucket-name <BUCKET_NAME>
          Bucket name

      --remote-prefix <REMOTE_PREFIX>
          You can set a directory in the bucket with this prefix
          
          [default: ]

  -h, --help
          Print help information (use `-h` for a summary)

```

## Examples

