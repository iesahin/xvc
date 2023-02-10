# xvc file remove

## Synopsis

```console
$ xvc file remove --help
Remove files from Xvc and possibly storages

Usage: xvc file remove [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to remove

Options:
      --from-cache
          Remove files from cache

      --from-storage <FROM_STORAGE>
          Remove files from storage

      --all-versions
          Remove all versions of the file

      --only-version <ONLY_VERSION>
          Remove only the specified version of the file

          Versions are specified like b3-123-456-789abcd where b3 is the hash algorithm prefix and the rest is a (at least 3 digit) prefix of the content hash. Prefix must be unique. If the prefix is not unique, the command will fail. Dashes are optional.

      --force
          Remove the targets even if they are used by other targets (via deduplication)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```


## Examples

This command deletes files from the Xvc cache or storage. It doesn't remove the file from Xvc tracking.

```admonition tip
If you want to remove a workspace file or link, you can use usual `rm` command. If the file is tracked and carried in to the cache, you can always [recheck](xvc-file-recheck.md) it.
```

This command only works if the file is tracked by Xvc.

```console
$ git init
...

$ xvc init

$ xvc file track 'd*.txt'

$ xvc file list
FC        [..] c85f3e81 c85f3e81 data.txt
FX        [..]          ac46bf74 .xvcignore
FX        [..] .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


$ tree .xvc/b3/
.xvc/b3/
└── c85
    └── f3e
        └── 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
            └── 0.txt

4 directories, 1 file

```

If you don't specify either `--from-cache` or `--from-storage`, this command does nothing.

```console
$ xvc file remove data.txt
[ERROR] File Error: At least one of --from-cache or --from-storage must be specified

```


You can remove the file from the cache. The file is still tracked by Xvc and available in the workspace.

```console
$ xvc file remove --from-cache data.txt

$ ls -l
total 8
-rw-rw-rw-  1 iex  staff  19 Jan 31 11:00 data.txt

$ tree .xvc/b3/

```

You can carry-in the missing file from the workspace to the cache.

```console
$ xvc file carry-in data.txt

$ xvc file list
FC          19 2023-01-31 08:00:58 c85f3e81 c85f3e81 data.txt
FX         130 2023-02-08 10:44:09          ac46bf74 .xvcignore
FX         191 2023-02-08 10:44:09          eb676f07 .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19

$ tree .xvc/b3/

```

You can specify a version of a file to delete from the cache. The versions can
be specified like `123-456-789abcd`. The prefix must be unique.

```console

$ perl -pi -e 's/a/e/g' data.txt

$ xvc file carry-in data.txt

$ tree .xvc/b3/
.xvc/b3/
├── 660
│   └── 2cf
│       └── f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
│           └── 0.txt
└── c85
    └── f3e
        └── 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
            └── 0.txt

7 directories, 2 files

$ xvc file list
FC          19 2023-02-08 10:44:10 6602cff6 6602cff6 data.txt
FX         130 2023-02-08 10:44:09          ac46bf74 .xvcignore
FX         191 2023-02-08 10:44:09          eb676f07 .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


$ xvc file remove --from-cache --only-version b3-c85-f3e data.txt

$ tree .xvc/b3/

```

You can also remove all versions of a file from the cache.

```console

$ xvc file remove --from-cache --all-versions data.txt

$ tree .xvc/b3/

```

It's possible to filter the cache versions by size or timestamp to remove.

Suppose you have three versions of `data.txt` in the cache. The first version is 19 bytes, the second is 2000 bytes and
the third is 3000 bytes.

```console

$ rm data.txt

$ xvc-test-helper generate-random-file --size 2000 --filename data.txt

$ xvc file carry-in data.txt

$ rm data.txt

$ xvc-test-helper generate-random-file --size 3000 --filename data.txt

$ xvc file carry-in data.txt

$ ls -l .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

You can remove all versions of the file larger than 2000 bytes.

```console
$ xvc file remove --from-cache --larger-than 2000 data.txt
$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

You can remove all versions of the file smaller than 500 bytes.

```console
$ xvc file remove --from-cache --smaller-than 500 data.txt
$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

You can remove all versions carried in before or after a certain timestamp.

```console
$ xvc-test-helper generate-random-file --size 2000 --filename data.txt

$ touch -t 202201010000 data.txt
$ xvc file carry-in data.txt

$ xvc-test-helper generate-random-file --size 2000 --filename data.txt

$ touch -t 202301010000 data.txt
$ xvc file carry-in data.txt

$ xvc-test-helper generate-random-file --size 2000 --filename data.txt

$ touch -t 202401010000 data.txt
$ xvc file carry-in data.txt

$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

Now remove all versions carried in before 2023-01-01.

```console
$ xvc file remove --from-cache --before 2023-01-01 data.txt
$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

Remove all versions carried in after 2023-01-02.

```console
$ xvc file remove --from-cache --after 2023-01-02 data.txt
$ ls -lR .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```

You can use this command to remove cached files from (remote) storages as well.

```console
$ xvc storage new local --name local-storage --path '../local-storage'

$ xvc file send data.txt --to local-storage
$ ls -l ../local-storage/*/b3/*/*/*/0.*
ls: ../local-storage/*/b3/*/*/*/0.*: No such file or directory

$ xvc file remove data.txt --from-storage local-storage
$ ls -lR ../local-storage/*/b3/*/*/*/0.*
ls: ../local-storage/*/b3/*/*/*/0.*: No such file or directory

```


If multiple paths are pointing to the same cache file (deduplication), the cache file will not be deleted.
In this case, `remove` reports other paths pointing to the same cache file. You must `--force` delete the cache file.

```console
$ xvc-test-helper generate-random-file --size 2000 --filename data.txt

$ xvc file carry-in data.txt

$ xvc file copy data.txt data2.txt --as symlink
$ xvc file list
SS         182 2023-02-08 10:44:12 d8e64688          data2.txt
FC        3000 2023-02-08 10:44:12 d8e64688 d8e64688 data.txt
FX         130 2023-02-08 10:44:09          ac46bf74 .xvcignore
FX         276 2023-02-08 10:44:12          5a975193 .gitignore
Total #: 4 Workspace Size:        3588 Cached Size:          19


$ xvc file remove --from-cache data.txt

$ ls -l .xvc/b3/*/*/*/0.*
ls: .xvc/b3/*/*/*/0.*: No such file or directory

```
