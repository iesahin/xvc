# xvc file untrack

## Synopsis

```console
$ xvc file untrack --help
Untrack (delete) files from Xvc and possibly storages

Usage: xvc file untrack [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...  Files/directories to untrack

Options:
      --restore-versions <RESTORE_VERSIONS>
          Restore all versions to a directory before deleting the cache files
  -h, --help
          Print help

```

## Examples

This command removes a file from Xvc tracking and optionally deletes it from the local filesystem, cache, and the storages.

It only works if the file is tracked by Xvc.

```console
$ git init
...

$ xvc init

$ xvc file track 'd*.txt'

$ xvc file list
FC          19 [..] c85f3e81 c85f3e81 data.txt
FX         130 [..]          [..] .xvcignore
FX         [..] .gitignore
Total #: 3 Workspace Size:         [..] Cached Size:          19


```

Without any options, it removes the file from Xvc tracking and the cache.

```admonition warning

`xvc file untrack` doesn't modify the `.gitignore` files to remove the previously tracked files. You must do it manually if you want to track the file with Git.

```

```console
$ xvc file untrack data.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
[DELETE] [CWD]/.xvc/b3/c85/f3e
[DELETE] [CWD]/.xvc/b3/c85
[DELETE] [CWD]/.xvc/b3

$ git status
On branch [..]
nothing to commit, working tree clean

```

If you have [rechecked](/concepts/recheck.md) the file as symlink or reflink, it will be copied to the workspace.

```console
$ xvc file track data.txt --as symlink

$ lsd -l
lrwxr-xr-x [..] data.txt ⇒ [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt

$ xvc file untrack data.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
[DELETE] [CWD]/.xvc/b3/c85/f3e
[DELETE] [CWD]/.xvc/b3/c85
[DELETE] [CWD]/.xvc/b3

$ ls -l
total 8
-rw-rw-rw-@ 1 iex  staff  19 Oct 18 10:45 data.txt

```

If there are multiple versions of the file, it removes them all and restores the latest version.

If you want to restore all versions of the file, you can specify a directory to restore them.

```console
$ xvc file track data.txt

$ perl -pi -e 's/a/e/g' data.txt

$ xvc file carry-in data.txt

$ xvc file untrack data.txt --restore-versions data-versions/
[COPY] [CWD]/.xvc/b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367/0.txt -> [CWD]/data-versions/data-b3-660-2cf-f6a4.txt
[COPY] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt -> [CWD]/data-versions/data-b3-c85-f3e-8108.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
[DELETE] [CWD]/.xvc/b3/c85/f3e
[DELETE] [CWD]/.xvc/b3/c85
[DELETE] [CWD]/.xvc/b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367/0.txt
[DELETE] [CWD]/.xvc/b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
[DELETE] [CWD]/.xvc/b3/660/2cf
[DELETE] [CWD]/.xvc/b3/660
[DELETE] [CWD]/.xvc/b3

$ ls -l data-versions/
total 16
-r--r--r--  [..] data-b3-660-2cf-f6a4.txt
-r--r--r--@ 1 iex  staff  19 Oct 18 10:45 data-b3-c85-f3e-8108.txt

```

If multiple paths are pointing to the same cache file (with deduplication), the cache file will not be
deleted. In this case, `untrack` reports other paths pointing to the same cache file. You must untrack all of them to
delete the cache file.

```console
$ xvc file track data.txt

$ xvc file copy data.txt data2.txt --as symlink

$ xvc file untrack data.txt
Not deleting b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367/0.txt (for data.txt) because it's also used by data2.txt

$ tree .xvc/b3/
.xvc/b3/
└── 660
    └── 2cf
        └── f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
            └── 0.txt

4 directories, 1 file

$ xvc file untrack data2.txt
[DELETE] [CWD]/.xvc/b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367/0.txt
[DELETE] [CWD]/.xvc/b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
[DELETE] [CWD]/.xvc/b3/660/2cf
[DELETE] [CWD]/.xvc/b3/660
[DELETE] [CWD]/.xvc/b3

```
