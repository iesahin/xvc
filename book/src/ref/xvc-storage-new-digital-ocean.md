# xvc storage new digital-ocean

## Purpose

Configure a [Digital Ocean Spaces](https://www.digitalocean.com/products/spaces) service as an Xvc storage. 

## Synopsis 

```console
$ xvc storage new digital-ocean --help
Add a new Digital Ocean storage

Usage: xvc storage new digital-ocean [OPTIONS] --name <NAME> --bucket-name <BUCKET_NAME> --region <REGION>

Options:
  -n, --name <NAME>
          Name of the storage
          
          This must be unique among all storages of the project

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

