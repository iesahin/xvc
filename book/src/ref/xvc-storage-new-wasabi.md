# xvc storage new wasabi

## Purpose

Configure a [Wasabi](https://wasabi.com/) service as an Xvc storage. 

## Synopsis 

```console
$ xvc storage new wasabi --help
Add a new Wasabi storage

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
