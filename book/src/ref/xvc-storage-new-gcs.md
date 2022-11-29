# xvc storage new gcs

## Purpose

Configure an [Google Cloud Storage](https://cloud.google.com/storage) service as an Xvc storage. 

## Synopsis 

```console
$ xvc storage new gcs --help
Add a new Google Cloud Storage storage

Usage: xvc storage new gcs [OPTIONS] --name <NAME> --bucket-name <BUCKET_NAME> --region <REGION>

Options:
  -n, --name <NAME>
          Name of the storage
          
          This must be unique among all storages of the project

      --bucket-name <BUCKET_NAME>
          Bucket name

      --region <REGION>
          Region of the server, e.g., europe-west3

      --storage-prefix <STORAGE_PREFIX>
          You can set a directory in the bucket with this prefix
          
          [default: ]

  -h, --help
          Print help information (use `-h` for a summary)
```

## Examples

