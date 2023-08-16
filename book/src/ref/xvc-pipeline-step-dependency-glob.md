### Glob Digest Dependencies

A step can depend on multiple files specified with globs. The difference with
this and [glob dependency](./xvc-pipeline-step-dependency-glob.md) is that this
one doesn't track the files, and doesn't pass the list of files in an
environment variable to the command.

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Let's create a set of files:

```console
$ xvc-test-helper create-directory-tree --directories 2 --files 3 --seed 2023

$ tree
.
├── dir-0001
│   ├── file-0001.bin
│   ├── file-0002.bin
│   └── file-0003.bin
└── dir-0002
    ├── file-0001.bin
    ├── file-0002.bin
    └── file-0003.bin

3 directories, 6 files

```

Add a step to say files has changed when the files have changed.

```console
$ xvc pipeline step new --step-name files-changed --command "echo 'Files have changed.'"

$ xvc pipeline step dependency --step-name files-changed --glob 'dir-*/*'
error: unexpected argument '--glob' found

  note: argument '--glob_d' exists

Usage: xvc pipeline step dependency <--step-name <STEP_NAME>|--generic <GENERICS>|--url <URLS>|--file <FILES>|--step <STEPS>|--glob_items <GLOB_ITEMS>|--glob_digest <GLOB_DIGESTS>|--param <PARAMS>|--regex <REGEXPS>|--regex_digest <REGEXP_DIGESTS>|--line_items <LINES>|--lines <LINE_DIGESTS>>

For more information, try '--help'.

```

The step is invalidated when a file described by the glob is added, removed or changed.

```console
$ xvc pipeline run
[OUT] [files-changed] Files have changed.
 

$ xvc pipeline run
[OUT] [files-changed] Files have changed.
 

```

```console
$ rm dir-0001/file-0001.bin

$ xvc pipeline run
[OUT] [files-changed] Files have changed.
 

```
