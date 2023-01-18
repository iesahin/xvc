# xvc file copy

## Synopsis

```console
$ xvc file copy --help
```

## Examples

Copies a set of targets to another location in the path.
By default it doesn't update the recheck method (cache type) of the targets. 

It works only with tracked files. 

```console
$ git init
...
$ xvc init

$ xvc file track data.txt

$ ls -l
total[..]
-rw-rw-rw- [..] data.txt

```

Once you added the file to the cache, you can copy the file to another location. 

```console
$ xvc file copy data.txt data2.txt

$ ls 
data.txt data2.txt

```

Xvc updates the cache type if the file is not changed.

```console
$ xvc file copy data.txt data3.txt --as symlink

$ ls -l
total[..]
-rw-rw-rw- [..] data.txt
-rw-rw-rw- [..] data2.txt
l[..] data3.txt -> [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt

```

You can create _views_ of your data by copying it to another location.

```console
$ xvc file copy 'd*' another-set/ --as hardlink

$ xvc file list another-set/
```

If the targets you specify are changed, they are not copied. 

```console
$ perl -i -pe 's/a/ee/g' data.txt

$ xvc file copy data.txt data5.txt
[ERROR] data.txt has changed on disk. Either carry in, force, or delete it to copy. 

```

You can copy files _virtually_, without them being in the workspace. 

```console
$ rm -f data.txt

$ xvc file copy data.txt data6.txt

$ ls -l data6.txt
```

You can also skip rechecking. 
In this case the copies won't be created in the workspace. 
They will be listed with `xvc file list` command.

```console
$ xvc file copy data.txt data7.txt --no-recheck

$ ls

$ xvc file list

```

Later, you can recheck them. 

```console
$ xvc file recheck data7.txt

$ ls -l data7.txt
```
