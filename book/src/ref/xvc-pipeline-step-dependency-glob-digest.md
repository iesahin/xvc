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
```

Add a step to say files has changed when the files have changed.

```console
$ xvc pipeline step new --step-name files-changed --command "echo 'Files have changed.'"

$ xvc pipeline step dependency --step-name files-changed --glob 'dir-*/*'

```

The step is invalidated when a file described by the glob is added, removed or changed.

```console
$ xvc pipeline run

$ xvc pipeline run

```

```console
$ rm dir-0001/file-0001

$ xvc pipeline run
```
