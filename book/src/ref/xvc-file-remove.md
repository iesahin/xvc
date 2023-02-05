# xvc file remove

## Synopsis

```console
$ xvc file remove --help
error: unrecognized subcommand 'remove'

  note: subcommand 'move' exists
  note: to pass 'remove' as a value, use 'xvc file -- remove'

Usage: xvc file [OPTIONS] <COMMAND>

For more information, try '--help'.

```


## Examples

This command files from the cache, the workspace or storages. It doesn't remove the file from Xvc tracking.

It only works if the file is tracked by Xvc.

```console
$ git init
...

$ xvc init

$ xvc file track 'd*.txt'

$ xvc file list
FC          19 2023-01-31 08:00:58 c85f3e81 c85f3e81 data.txt
FX         130 2023-02-05 11:39:31          ac46bf74 .xvcignore
FX         191 2023-02-05 11:39:32          0561cc21 .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


```

You can remove the file from the workspace.

```console
$ xvc file remove --from-workspace data.txt
error: unrecognized subcommand 'remove'

  note: subcommand 'move' exists
  note: to pass 'remove' as a value, use 'xvc file -- remove'

Usage: xvc file [OPTIONS] <COMMAND>

For more information, try '--help'.

$ xvc file list
FC          19 2023-01-31 08:00:58 c85f3e81 c85f3e81 data.txt
FX         130 2023-02-05 11:39:31          ac46bf74 .xvcignore
FX         191 2023-02-05 11:39:32          0561cc21 .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


```

You can recheck the file if it still resides in the cache.

```console
$ xvc file recheck data.txt

$ ls -l
total 8
-rw-rw-rw-  1 iex  staff  19 Jan 31 11:00 data.txt

```

You can remove the file from the cache and keep the workspace version.

```console
$ xvc file remove --from-cache data.txt
error: unrecognized subcommand 'remove'

  note: subcommand 'move' exists
  note: to pass 'remove' as a value, use 'xvc file -- remove'

Usage: xvc file [OPTIONS] <COMMAND>

For more information, try '--help'.

$ ls -l
total 8
-rw-rw-rw-  1 iex  staff  19 Jan 31 11:00 data.txt

```

You can carry-in the file from the workspace to the cache.

```console
$ xvc file carry-in data.txt

$ xvc file list
FC          19 2023-01-31 08:00:58 c85f3e81 c85f3e81 data.txt
FX         130 2023-02-05 11:39:31          ac46bf74 .xvcignore
FX         191 2023-02-05 11:39:32          0561cc21 .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


```

You can remove all versions of a file from the cache.

```console
$ perl -pi -e 's/a/e/g' data.txt

$ xvc file carry-in data.txt

$ xvc file list
FC          19 2023-02-05 11:39:33 6602cff6 6602cff6 data.txt
FX         130 2023-02-05 11:39:31          ac46bf74 .xvcignore
FX         191 2023-02-05 11:39:32          0561cc21 .gitignore
Total #: 3 Workspace Size:         340 Cached Size:          19


$ xvc file remove --from-cache --all-versions data.txt
error: unrecognized subcommand 'remove'

  note: subcommand 'move' exists
  note: to pass 'remove' as a value, use 'xvc file -- remove'

Usage: xvc file [OPTIONS] <COMMAND>

For more information, try '--help'.

```

You can use this command to remove the files from storages as well.

```console
$ xvc file carry-in data.txt

$ xvc storage new local --name local-storage --path '../local-storage'

$ xvc file send data.txt --to local-storage

$ xvc file remove data.txt --from-storage local-storage
error: unrecognized subcommand 'remove'

  note: subcommand 'move' exists
  note: to pass 'remove' as a value, use 'xvc file -- remove'

Usage: xvc file [OPTIONS] <COMMAND>

For more information, try '--help'.

```

If multiple paths are pointing to the same cache file (deduplication), the cache file will not be deleted.
In this case, `remove` reports other paths pointing to the same cache file. You must `--force` delete the cache file.

```console
$ xvc file track data.txt

$ xvc file copy data.txt data2.txt --as symlink

$ xvc file remove --from-cache data.txt
error: unrecognized subcommand 'remove'

  note: subcommand 'move' exists
  note: to pass 'remove' as a value, use 'xvc file -- remove'

Usage: xvc file [OPTIONS] <COMMAND>

For more information, try '--help'.

$ ls -lR .xvc/b3/
total 0
drwxr-xr-x  3 iex  staff  96 Feb  5 14:39 660
drwxr-xr-x  3 iex  staff  96 Feb  5 14:39 c85

.xvc/b3//660:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  5 14:39 2cf

.xvc/b3//660/2cf:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  5 14:39 f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367

.xvc/b3//660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367:
total 8
-r--r--r--  1 iex  staff  19 Feb  5 14:39 0.txt

.xvc/b3//c85:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  5 14:39 f3e

.xvc/b3//c85/f3e:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  5 14:39 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496

.xvc/b3//c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496:
total 8
-r--r--r--  1 iex  staff  19 Jan 31 11:00 0.txt

$ xvc file remove --from-cache --force data.txt
error: unrecognized subcommand 'remove'

  note: subcommand 'move' exists
  note: to pass 'remove' as a value, use 'xvc file -- remove'

Usage: xvc file [OPTIONS] <COMMAND>

For more information, try '--help'.

$ ls -lR .xvc/b3/
total 0
drwxr-xr-x  3 iex  staff  96 Feb  5 14:39 660
drwxr-xr-x  3 iex  staff  96 Feb  5 14:39 c85

.xvc/b3//660:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  5 14:39 2cf

.xvc/b3//660/2cf:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  5 14:39 f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367

.xvc/b3//660/2cf/f6a4cbc23a78205463b7086d1b0831d3d74c063122f20c1c2ea0c2d367:
total 8
-r--r--r--  1 iex  staff  19 Feb  5 14:39 0.txt

.xvc/b3//c85:
total 0
drwxr-xr-x  3 iex  staff  96 Feb  5 14:39 f3e

.xvc/b3//c85/f3e:
total 0
dr-xr-xr-x  3 iex  staff  96 Feb  5 14:39 8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496

.xvc/b3//c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496:
total 8
-r--r--r--  1 iex  staff  19 Jan 31 11:00 0.txt

```
