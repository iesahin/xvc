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
      --delete-from-storages <DELETE_FROM_STORAGES>
          Delete all files also from given storages
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
FX         191 [..]          [..] .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


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
On branch main
nothing to commit, working tree clean

```

If you have [rechecked](/concepts/recheck.md) the file as symlink or reflink, it will be copied to the workspace.

```console
$ xvc file track data.txt --as symlink

$ ls -l
total 0
lrwxr-xr-x  [..] data.txt -> [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt

$ xvc file untrack data.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496
[DELETE] [CWD]/.xvc/b3/c85/f3e
[DELETE] [CWD]/.xvc/b3/c85
[DELETE] [CWD]/.xvc/b3

$ ls -l
total 8
-rw-rw-rw-  [..] data.txt

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
-r--r--r--  1 iex  staff  19 Feb  6 12:29 data-b3-660-2cf-f6a4.txt
-r--r--r--  1 iex  staff  19 Jan 31 11:00 data-b3-c85-f3e-8108.txt

```

With `--delete-from-storages`, this command removes the cached files from (remote) storages.

```console
$ xvc file track data.txt

$ xvc storage new local --name local-storage --path '../local-storage'

$ xvc file send data.txt --to local-storage

$ xvc file untrack data.txt --delete-from-storages local-storage
[DELETE] [CWD]/.xvc/b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367/0.txt
[DELETE] [CWD]/.xvc/b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367
[DELETE] [CWD]/.xvc/b3/660/2cf
[DELETE] [CWD]/.xvc/b3/660
[DELETE] [CWD]/.xvc/b3

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

$ tree .xvc/b3/
.xvc/
├── config.local.toml
├── config.toml
├── ec
│   ├── 1675675777573863
│   ├── 1675675777577799
│   ├── 1675675778068509
│   ├── 1675675778878118
│   ├── 1675675779306978
│   ├── 1675675779839786
│   ├── 1675675779929794
│   ├── 1675675780420030
│   └── 1675675780615919
└── store
    ├── content-digest-store
    │   ├── 1675675778052274.json
    │   ├── 1675675778516231.json
    │   ├── 1675675778860444.json
    │   ├── 1675675778979816.json
    │   ├── 1675675779282414.json
    │   ├── 1675675779447491.json
    │   ├── 1675675779532185.json
    │   ├── 1675675779819592.json
    │   ├── 1675675780082188.json
    │   ├── 1675675780389323.json
    │   ├── 1675675780602445.json
    │   ├── 1675675780707313.json
    │   └── 1675675780799664.json
    ├── file-text-or-binary-store
    │   ├── 1675675778052141.json
    │   ├── 1675675778516399.json
    │   ├── 1675675778860268.json
    │   ├── 1675675778979993.json
    │   ├── 1675675779282133.json
    │   ├── 1675675779532334.json
    │   ├── 1675675779819377.json
    │   ├── 1675675780082351.json
    │   ├── 1675675780389024.json
    │   ├── 1675675780602775.json
    │   ├── 1675675780707505.json
    │   └── 1675675780799846.json
    ├── recheck-method-store
    │   ├── 1675675778051973.json
    │   ├── 1675675778516485.json
    │   ├── 1675675778860108.json
    │   ├── 1675675778980085.json
    │   ├── 1675675779281318.json
    │   ├── 1675675779532420.json
    │   ├── 1675675779819199.json
    │   ├── 1675675780082426.json
    │   ├── 1675675780388764.json
    │   ├── 1675675780603028.json
    │   ├── 1675675780707586.json
    │   └── 1675675780799948.json
    ├── remote-store
    │   └── 1675675779929178.json
    ├── storage-event-remote-r1n-store
    │   └── 1675675779929576.json
    ├── storage-event-store
    │   └── 1675675779929348.json
    ├── xvc-metadata-store
    │   ├── 1675675778051644.json
    │   ├── 1675675778516582.json
    │   ├── 1675675778859889.json
    │   ├── 1675675778980184.json
    │   ├── 1675675779281087.json
    │   ├── 1675675779532507.json
    │   ├── 1675675779818994.json
    │   ├── 1675675780082499.json
    │   ├── 1675675780388430.json
    │   ├── 1675675780603185.json
    │   ├── 1675675780707669.json
    │   └── 1675675780800039.json
    ├── xvc-path-store
    │   ├── 1675675778051427.json
    │   ├── 1675675778516662.json
    │   ├── 1675675778859626.json
    │   ├── 1675675778980278.json
    │   ├── 1675675779280642.json
    │   ├── 1675675779532591.json
    │   ├── 1675675779818721.json
    │   ├── 1675675780082580.json
    │   ├── 1675675780371349.json
    │   ├── 1675675780603107.json
    │   ├── 1675675780707758.json
    │   └── 1675675780800150.json
    └── xvc-pipeline-store
        └── 1675675777577452.json

12 directories, 76 files

```
