# xvc storage new s3

## Purpose

Configure an S3 (or a compatible) service as an Xvc storage. 

## Synopsis 

```console
$ xvc storage new s3 --help
Add a new S3 remote

Usage: xvc storage new s3 [OPTIONS] --name <NAME> --bucket-name <BUCKET_NAME> --region <REGION>

Options:
  -n, --name <NAME>
          Name of the remote
          
          This must be unique among all remotes of the project

      --remote-prefix <REMOTE_PREFIX>
          You can set a directory in the bucket with this prefix
          
          [default: ]

      --bucket-name <BUCKET_NAME>
          S3 bucket name

      --region <REGION>
          AWS region

  -h, --help
          Print help information (use `-h` for a summary)

```

## Examples

