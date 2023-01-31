# xvc file remove

## Synopsis

```console
$ xvc file remove --help
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
```

You can remove the file from the workspace.

```console
$ xvc file remove --from-workspace data.txt

$ xvc file list
```

You can recheck the file if it still resides in the cache.

```console
$ xvc file recheck data.txt

$ ls -l
```

You can remove the file from the cache and keep the workspace version.

```console
$ xvc file remove --from-cache data.txt

$ ls -l
```

You can carry-in the file from the workspace to the cache.

```console
$ xvc file carry-in data.txt

$ xvc file list
```

You can remove all versions of a file from the cache.

```console
$ perl -pi -e 's/a/e/g' data.txt

$ xvc file carry-in data.txt

$ xvc file list

$ xvc file remove --from-cache --all-versions data.txt
```

You can use this command to remove the files from storages as well.

```console
$ xvc file carry-in data.txt

$ xvc storage new local --name local-storage --path '../local-storage'

$ xvc file send data.txt --to local-storage

$ xvc file remove data.txt --from-storage local-storage
```

If multiple paths are pointing to the same cache file (deduplication), the cache file will not be deleted.
In this case, `remove` reports other paths pointing to the same cache file. You must `--force` delete the cache file.

```console
$ xvc file track data.txt

$ xvc file copy data.txt data2.txt --as symlink

$ xvc file remove --from-cache data.txt

$ ls -lR .xvc/b3/

$ xvc file remove --from-cache --force data.txt

$ ls -lR .xvc/b3/
```
