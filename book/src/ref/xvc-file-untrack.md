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
$ xvc --debug -vvvv file untrack data.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt
[DELETE] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496

$ git status
On branch main
Untracked files:
  (use "git add <file>..." to include in what will be committed)
	.xvc/store/content-digest-store/1675596367767119.json
	.xvc/store/file-text-or-binary-store/1675596367767334.json
	.xvc/store/recheck-method-store/1675596367767495.json
	.xvc/store/xvc-metadata-store/1675596367767624.json
	.xvc/store/xvc-path-store/1675596367767759.json

nothing added to commit but untracked files present (use "git add" to track)

```

If you have [rechecked](/concepts/recheck.md) the file as symlink or reflink, it will be copied to the workspace.

```console
$ xvc file track data.txt --as symlink

$ ls -l
total 0
lrwxr-xr-x  [..] data.txt -> [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt

$ xvc file untrack data.txt

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

$ ls -l data-versions/
total 16
-r--r--r--  1 iex  staff  19 Feb  3 13:46 data-b3-660-2cf-f6a4.txt
-r--r--r--  1 iex  staff  19 Jan 31 11:00 data-b3-c85-f3e-8108.txt

```

With `--delete-from-storages`, this command removes the cached files from (remote) storages.

```console
$ xvc file track data.txt

$ xvc storage new local --name local-storage --path '../local-storage'

$ xvc file send data.txt --to local-storage

$ xvc file untrack data.txt --delete-from-storages local-storage
```

If multiple paths are pointing to the same cache file (with deduplication), the cache file will not be
deleted. In this case, `untrack` reports other paths pointing to the same cache file. You must untrack all of them to
delete the cache file.

```console
$ xvc file track data.txt

$ xvc file copy data.txt data2.txt --as symlink

$ xvc file untrack data.txt
Not deleting b3/660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367/0.txt (for data.txt) because it's also used by data2.txt

$ ls -lR .xvc/b3/
total 0
drwxr-xr-x  3 iex  staff  96 Feb  3 13:46 660
drwxr-xr-x  3 iex  staff  96 Feb  3 13:46 c85

.xvc/b3//660:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  3 13:46 2cf

.xvc/b3//660/2cf:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  3 13:46 f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367

.xvc/b3//660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367:
total 8
-r--r--r--  1 iex  staff  19 Feb  3 13:46 0.txt

.xvc/b3//c85:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  3 13:46 f3e

.xvc/b3//c85/f3e:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  3 13:46 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496

.xvc/b3//c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496:
total 8
-r--r--r--  1 iex  staff  19 Jan 31 11:00 0.txt

$ xvc file untrack data2.txt

$ ls -lR .xvc/b3/
total 0
drwxr-xr-x  3 iex  staff  96 Feb  3 13:46 660
drwxr-xr-x  3 iex  staff  96 Feb  3 13:46 c85

.xvc/b3//660:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  3 13:46 2cf

.xvc/b3//660/2cf:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  3 13:46 f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367

.xvc/b3//660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367:
total 8
-r--r--r--  1 iex  staff  19 Feb  3 13:46 0.txt

.xvc/b3//c85:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  3 13:46 f3e

.xvc/b3//c85/f3e:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  3 13:46 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496

.xvc/b3//c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496:
total 8
-r--r--r--  1 iex  staff  19 Jan 31 11:00 0.txt

```
