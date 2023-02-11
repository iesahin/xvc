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

          Versions are specified with the content hash 123-456-789abcd. Dashes are optional. Prefix must be unique. If the prefix is not unique, the command will fail.

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
error: the following required arguments were not provided:
  --from-cache
  --from-storage <FROM_STORAGE>

Usage: xvc file remove --from-cache --from-storage <FROM_STORAGE> <TARGETS>...

For more information, try '--help'.

```


You can remove the file from the cache. The file is still tracked by Xvc and available in the workspace.

```console
$ xvc file remove --from-cache data.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
[DELETE] [CWD]/.xvc/b3/c85/f3e
[DELETE] [CWD]/.xvc/b3/c85
[DELETE] [CWD]/.xvc/b3

$ ls -l
total 8
-rw-rw-rw-  1 iex  staff  19 Jan 31 11:00 data.txt

$ ls .xvc/
.xvc/b3/  [error opening dir]

0 directories, 0 files

```

You can carry the missing file from the workspace to the cache. Use `--force` to overwrite the cache as carry-in
doesn't overwrite the cache by default.

```console
$ xvc file carry-in --force data.txt

$ xvc file list
FC         [..] c85f3e81 c85f3e81 data.txt
FX         [..]          ac46bf74 .xvcignore
FX         [..]          [..] .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


$ tree .xvc/b3/
.xvc/b3/
└── c85
    └── f3e
        └── 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
            └── 0.txt

4 directories, 1 file

```

You can specify a version of a file to delete from the cache. The versions can
be specified like `123-456-789abcd`. Dashes are optional. The prefix must be unique.

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
FC         [..] 6602cff6 6602cff6 data.txt
FX         [..]          ac46bf74 .xvcignore
FX         [..]          [..] .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


$ xvc file remove --from-cache --only-version c85-f3e data.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
[DELETE] [CWD]/.xvc/b3/c85/f3e
[DELETE] [CWD]/.xvc/b3/c85

$ tree .xvc/b3/
.xvc/b3/
└── 660
    └── 2cf
        └── f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
            └── 0.txt

4 directories, 1 file

```

You can also remove all versions of a file from the cache.

```console
$ xvc-test-helper generate-filled-file --value 0 --filename data.txt

$ xvc file carry-in data.txt

$ rm data.txt

$ xvc-test-helper generate-filled-file --value 1 --filename data.txt

$ xvc file carry-in data.txt

$ tree .xvc/b3/
.xvc/b3/
├── 660
│   └── 2cf
│       └── f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
│           └── 0.txt
├── 88d
│   └── ba6
│       └── ea92dd3ced3c319072cc2c814c7b8eb3269b920ac8c83e6b5c33699420
│           └── 0.txt
└── d6f
    └── d9d
        └── e5bccf223f523b316c9cd1cf9a9d87ea42473d68e011dad13f09bf8917
            └── 0.txt

10 directories, 3 files

$ xvc file remove --from-cache --all-versions data.txt
[DELETE] [CWD]/.xvc/b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367/0.txt
[DELETE] [CWD]/.xvc/b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
[DELETE] [CWD]/.xvc/b3/660/2cf
[DELETE] [CWD]/.xvc/b3/660
[DELETE] [CWD]/.xvc/b3/d6f/d9d/e5bccf223f523b316c9cd1cf9a9d87ea42473d68e011dad13f09bf8917/0.txt
[DELETE] [CWD]/.xvc/b3/d6f/d9d/e5bccf223f523b316c9cd1cf9a9d87ea42473d68e011dad13f09bf8917
[DELETE] [CWD]/.xvc/b3/d6f/d9d
[DELETE] [CWD]/.xvc/b3/d6f
[DELETE] [CWD]/.xvc/b3/88d/ba6/ea92dd3ced3c319072cc2c814c7b8eb3269b920ac8c83e6b5c33699420/0.txt
[DELETE] [CWD]/.xvc/b3/88d/ba6/ea92dd3ced3c319072cc2c814c7b8eb3269b920ac8c83e6b5c33699420
[DELETE] [CWD]/.xvc/b3/88d/ba6
[DELETE] [CWD]/.xvc/b3/88d
[DELETE] [CWD]/.xvc/b3

$ ls .xvc/

```

You can use this command to remove cached files from (remote) storages as well.

```console
$ xvc-test-helper generate-filled-file --value 2 --filename data.txt
$ xvc file carry-in data.txt

$ xvc storage new local --name local-storage --path '../local-storage'
$ xvc file send data.txt --to local-storage

$ tree ../local-storage/
../local-storage/
└── [..]
    └── b3
        └── fa8
            └── af1
                └── 7567c147993830cdd42cea9d8a8f157c9b98b4e7ef5677f417a5d8ae61
                    └── 0.txt

6 directories, 1 file

$ xvc file remove data.txt --from-storage local-storage

$ tree ../local-storage/
../local-storage/
└── [..]
    └── b3
        └── fa8
            └── af1
                └── 7567c147993830cdd42cea9d8a8f157c9b98b4e7ef5677f417a5d8ae61

6 directories, 0 files

```

Note that, storage delete implementations differ slightly not to remove the directories. This is to avoid unnecessary
round trip existence checks.

If multiple paths are pointing to the same cache file (deduplication), the cache file will not be deleted.
In this case, `remove` reports other paths pointing to the same cache file. You must `--force` delete the cache file.

```console
$ xvc-test-helper generate-filled-file --value 3 --filename data.txt

$ xvc file carry-in data.txt

$ xvc file copy data.txt data2.txt --as symlink
$ xvc file list
SS        [..] [..] ba3d2f3e          data2.txt
FC        1024 [..] ba3d2f3e ba3d2f3e data.txt
FX         130 [..]          ac46bf74 .xvcignore
FX         276 [..]          [..] .gitignore
Total #: 4 Workspace Size:        [..] Cached Size:        1024


$ xvc file remove --from-cache data.txt
Not deleting b3/ba3/d2f/3e2f90b8f4a3365b29ed29bd31d4b3483c876811ee6e4b2ddd3d74af01/0.txt (for data.txt) because it's also used by data2.txt

$ tree .xvc/b3/
.xvc/b3/
├── ba3
│   └── d2f
│       └── 3e2f90b8f4a3365b29ed29bd31d4b3483c876811ee6e4b2ddd3d74af01
│           └── 0.txt
└── fa8
    └── af1
        └── 7567c147993830cdd42cea9d8a8f157c9b98b4e7ef5677f417a5d8ae61
            └── 0.txt

7 directories, 2 files

```
