# xvc pipeline import

## Synopsis

```console
$ xvc pipeline import --help
Import the pipeline from a file

Usage: xvc pipeline import [OPTIONS]

Options:
  -p, --pipeline-name <PIPELINE_NAME>  Name of the pipeline to import. If not set, the name from the file is used
      --file <FILE>                    File to read the pipeline. Use stdin if not specified
      --format <FORMAT>                Input format. One of json or yaml. If not set, the format is guessed from the file extension. If the file extension is not set, json is used as default
      --overwrite                      Overwrite the pipeline even if the name already exists
  -h, --help                           Print help

```

## Examples

This command is used to import pipelines exported with [`xvc pipeline export`](/ref/xvc-pipeline-export.md).

You can edit and import the pipelines exported with the command.

```admonition warning
Xvc doesn't guarantee that the format of these files will be compatible across versions. You can use these files to share pipeline definitions but it may not be a good way to store pipeline definitions for longer periods.
```

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

The following file generated with `xvc pipeline export`.

```console
$ cat pipeline.yaml
version: 1
name: default
workdir: ''
steps:
- name: step1
  command: touch abc.txt
  invalidate: ByDependencies
  dependencies: []
  outputs: []
- name: step2
  command: touch def.txt
  invalidate: ByDependencies
  dependencies:
  - !Step
    name: step1
  - !Generic
    generic_command: ping -c 2 example.com
    output_digest: null
  - !GlobItems
    glob: '*.txt'
    xvc_path_metadata_map: {}
    xvc_path_content_digest_map: {}
  - !Glob
    glob: '*.txt'
    xvc_paths_digest: null
    xvc_metadata_digest: null
    content_digest: null
  - !RegexItems
    path: requirements.txt
    regex: ^tensorflow
    lines: []
    xvc_metadata: null
  - !Regex
    path: requirements.txt
    regex: ^tensorflow
    lines_digest: null
    xvc_metadata: null
  - !Param
    format: YAML
    path: params.yaml
    key: model.conv_units
    value: null
    xvc_metadata: null
  - !LineItems
    path: params.yaml
    begin: 1
    end: 20
    xvc_metadata: null
    lines: []
  - !Lines
    path: params.yaml
    begin: 1
    end: 20
    xvc_metadata: null
    digest: null
  - !UrlDigest
    url: https://example.com/
    etag: null
    last_modified: null
    url_content_digest: null
  outputs:
  - !File
    path: def.txt
  - !Metric
    path: metrics.json
    format: JSON
  - !Image
    path: plots/confusion.png

```

You can import this file to construct the pipeline at once.
Note that the `export` command outputs JSON by default.

```console
$ xvc pipeline import --file pipeline.yaml --overwrite

$ xvc pipeline export
{
  "name": "default",
  "steps": [
    {
      "command": "touch abc.txt",
      "dependencies": [],
      "invalidate": "ByDependencies",
      "name": "step1",
      "outputs": []
    },
    {
      "command": "touch def.txt",
      "dependencies": [
        {
          "Step": {
            "name": "step1"
          }
        },
        {
          "Generic": {
            "generic_command": "ping -c 2 example.com",
            "output_digest": null
          }
        },
        {
          "GlobItems": {
            "glob": "*.txt",
            "xvc_path_content_digest_map": {},
            "xvc_path_metadata_map": {}
          }
        },
        {
          "Glob": {
            "content_digest": null,
            "glob": "*.txt",
            "xvc_metadata_digest": null,
            "xvc_paths_digest": null
          }
        },
        {
          "RegexItems": {
            "lines": [],
            "path": "requirements.txt",
            "regex": "^tensorflow",
            "xvc_metadata": null
          }
        },
        {
          "Regex": {
            "lines_digest": null,
            "path": "requirements.txt",
            "regex": "^tensorflow",
            "xvc_metadata": null
          }
        },
        {
          "Param": {
            "format": "YAML",
            "key": "model.conv_units",
            "path": "params.yaml",
            "value": null,
            "xvc_metadata": null
          }
        },
        {
          "LineItems": {
            "begin": 1,
            "end": 20,
            "lines": [],
            "path": "params.yaml",
            "xvc_metadata": null
          }
        },
        {
          "Lines": {
            "begin": 1,
            "digest": null,
            "end": 20,
            "path": "params.yaml",
            "xvc_metadata": null
          }
        },
        {
          "UrlDigest": {
            "etag": null,
            "last_modified": null,
            "url": "https://example.com/",
            "url_content_digest": null
          }
        }
      ],
      "invalidate": "ByDependencies",
      "name": "step2",
      "outputs": [
        {
          "File": {
            "path": "def.txt"
          }
        },
        {
          "Metric": {
            "format": "JSON",
            "path": "metrics.json"
          }
        },
        {
          "Image": {
            "path": "plots/confusion.png"
          }
        }
      ]
    }
  ],
  "version": 1,
  "workdir": ""
}

```

If you don't supply the `--overwrite` option, Xvc will report an error and quit.

```console
$ xvc pipeline import --file pipeline.yaml
? 1
[ERROR] Pipeline Error: Pipeline default already found
Error: PipelineError { source: PipelineAlreadyFound { name: "default" } }

```

You can specify a new name for the pipeline and it will override the name set in the file.
This way you can edit and import similar pipelines with minor differences.

```console
$ xvc pipeline import --pipeline-name another-pipeline --file pipeline.yaml

```

You can also use stdin to import a pipeline but you must specify the input format.
