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
          
          Although not strictly required, keep this unique to refer easily.

      --bucket-name <BUCKET_NAME>
          Bucket name
		  
	      This must be created with correct read/write rights before running this command.

      --endpoint <ENDPOINT>
          Endpoint for the server, complete with the region if there is.
          
          e.g. for eu-central-1 region, use s3.eu-central-1.wasabisys.com as the endpoint.
          
          [default: s3.wasabisys.com]

      --storage-prefix <STORAGE_PREFIX>
          You can set a prefix in the bucket.
          It will be used like a directory for all files to contain Xvc-related files.
          
          [default: ]

  -h, --help
          Print help information (use `-h` for a summary)

```

## Examples
