# xvc remote new minio

## Purpose

Create a new Xvc remote on a [MinIO](https://min.io) instance. 
It allows to store tracked file contents on a remote Minio server. 

## Synopsis 

```text
{{#include xvc-remote-new-minio.txt}}
```

## Credentials

Xvc doesn't store any credentials. 
Xvc gets server credentials from two environment variables: `XVC_REMOTE_ACCESS_KEY_ID` and `XVC_REMOTE_SECRET_KEY`. 
You must supply the credentials in these two environment variables before running any command that connects to the remote. 

These environment variables can contain user name and password to Minio server. 
If you have created service accounts associated with the user name and password, you can also set the keys to those as well. 

```shell
$ export XVC_REMOTE_ACCESS_KEY_ID=myname
$ export XVC_REMOTE_SECRET_KEY=mypassword
$ xvc remote new-minio --name minio-remote --endpoint 'http://example.com:9001' --bucket-name xvc-bucket --region us-east-1 --remote-prefix my-project
```

## Examples

You can create a new remote by supplying the credentials and required parameters. 

```shell
$ export XVC_REMOTE_ACCESS_KEY_ID=myname
$ export XVC_REMOTE_SECRET_KEY=mypassword
$ xvc remote new-minio --name minio-remote --endpoint 'http://example.com:9001' --bucket-name xvc-bucket --region us-east-1 --remote-prefix my-project
```

After defining the remote, you can push, fetch, and pull files with `xvc file push` and `xvc file pull` commands. jjj

## Caveats

`--name NAME` is not verified to be unique but you should use unique remote names to refer them later. 
You can also use remote GUIDs listed by `xvc remote list` to refer to remotes. 

You must have a valid connection to the server. 

Xvc uses Minio API port (9001, by default) to connect to the server. 
You must ensure that it's accessible. 

For reasons caused from the underlying library, Xvc tries to connect `http://xvc-bucket.example.com:9001` if you give `http://example.com:9001` as the endpoint, and `xvc-bucket` as the bucket name. 
You may need to consider this when you have servers running in exact URLs. 
If you have a `http://minio.example.com:9001` as a Minio server, you may want to supply `http://example.com:9001` as the endpoint, and `minio` as the bucket name to form the correct URL. 
This behavior may change in the future. 


## Technical Details

This command requires Xvc to be compiled with `minio` feature, which is _on_ by default. 
It uses Rust async features via `rust-s3` crate, and may add some bulk to the binary. 
If you want to compile Xvc without these features, please refer to [How to Compile Xvc](/how-to/compile.md) document.

The command creates `.xvc-guid` file in `http://{{BUCKET-NAME}}.{{ENDPOINT}}/{{REMOTE-PREFIX}}/.xvc-guid`. 
The file contains the unique identifier for this remote. 
The same identifier is also recorded to the project. 

A file that's found in `.xvc/{{HASH_PREFIX}}/{{CACHE_PATH}}` is saved to `http://{{BUCKET-NAME}}.{{ENDPOINT}}/{{REMOTE-PREFIX}}/{{REPO_ID}}/{{HASH_PREFIX}}/{{CACHE_PATH}}`. 
`{{REPO_ID}}` is the unique identifier for the repository created during `xvc init`. 
Hence if you use a common remote for different Xvc projects, their files are kept under different directories. 
There is no inter-project deduplication.


