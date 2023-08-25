# Glob Items Dependency

A step can depend on multiple files specified with globs. When any of the files change, or a new file is added or
removed from the files specified by glob, the step is invalidated.

Unline glob dependency, glob items dependency keeps track of the individual files that belong to a glob. If your
command run with the list of files from a glob and you want to track added and removed files, use this. Otherwise if
your command for all the files in a glob and don't need to track which files have changed, use the glob dependency.

This one injects `${XVC_GLOB_ADDED_ITEMS}`, `${XVC_GLOB_REMOVED_ITEMS}` and `${XVC_GLOB_ALL_ITEMS}` to the command
environment.

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

Add a step to list the added files.

```console
$ xvc pipeline step new --step-name files-changed --command 'echo "### Added Files:\n${XVC_GLOB_ADDED_ITEMS}\n### Removed Files:\n${XVC_GLOB_REMOVED_ITEMS}"'

$ xvc pipeline step dependency --step-name files-changed --glob-items 'dir-*/*'

```

The step is invalidated when a file described by the glob is added, removed or changed.

```console
$ xvc pipeline run
[OUT] [files-changed] ### Added Files:
dir-0002/file-0001.bin
dir-0001/file-0001.bin
dir-0002/file-0003.bin
dir-0002/file-0002.bin
dir-0001/file-0002.bin
dir-0001/file-0003.bin
### Removed Files:



$ xvc pipeline run

```

If you add or remove a file from the files specified by the glob, they are printed.

```console
$ rm dir-0001/file-0001.bin

$ xvc pipeline run
[OUT] [files-changed] ### Added Files:

### Removed Files:
dir-0001/file-0001.bin


```

When you change a file, it's printed in both added and removed files:

```console
$ xvc-test-helper generate-filled-file dir-0001/file-0002.bin

$ xvc pipeline run

```
