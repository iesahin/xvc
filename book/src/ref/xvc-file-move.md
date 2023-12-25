# xvc file move

## Synopsis

```console
$ xvc file move --help
Move files to another location in the workspace

Usage: xvc file move [OPTIONS] <SOURCE> <DESTINATION>

Arguments:
  <SOURCE>
          Source file, glob or directory within the workspace.
          
          If the source ends with a slash, it's considered a directory and all files in that directory are copied.
          
          If there are multiple source files, the destination must be a directory.

  <DESTINATION>
          Location we move file(s) to within the workspace.
          
          If this ends with a slash, it's considered a directory and created if it doesn't exist.
          
          If the number of source files is more than one, the destination must be a directory.

Options:
      --recheck-method <RECHECK_METHOD>
          How the destination should be rechecked: One of copy, symlink, hardlink, reflink.
          
          Note: Reflink uses copy if the underlying file system doesn't support it.

      --no-recheck
          Do not recheck the destination files This is useful when you want to copy only records, without updating the workspace

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

## Examples

This command is used to move a set of files to another location in the workspace.

By default, it doesn't update the recheck method (cache type) of the targets.
It rechecks them to the destination with the same method.

`xvc file move` works only with the tracked files.

```console
$ git init
...
$ xvc init

$ xvc file track data.txt

$ lsd -l
.rw-rw-rw- [..] data.txt

```

Once you add the file to the cache, you can move the file to another location.

```console
$ xvc file move data.txt data2.txt

$ ls
data2.txt

```

Xvc can change the destination file's recheck method.

```console
$ xvc file move data2.txt data3.txt --as symlink

$ ls -l
total[..]
lrwxr-xr-x  1 [..] data3.txt -> [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt

```

You can move files without them being in the workspace if they are in the cache.

```console
$ rm -f data3.txt

$ xvc file move data3.txt data4.txt

$ ls -l
total 0
lrwxr-xr-x  [..] data4.txt -> [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt

```

You can use glob patterns to move multiple files.
In this case, the destination must be a directory.

```console
$ xvc file copy data4.txt data5.txt

$ xvc file move d*.txt another-set/ --as hardlink

$ xvc file list another-set/
FH          [..] c85f3e81 c85f3e81 another-set/data5.txt
FH          [..] c85f3e81 c85f3e81 another-set/data4.txt
Total #: 2 Workspace Size:          38 Cached Size:          19


```

You can also skip rechecking.
In this case, Xvc won't create any copies in the workspace, and you don't need them to be available in the cache.
They will be listed with `xvc file list` command.

```console
$ xvc file move another-set/data5.txt data6.txt --no-recheck

$ xvc file list
XH                                 c85f3e81          data6.txt
FH          19 [..] c85f3e81 c85f3e81 another-set/data4.txt
DX          96 [..]                   another-set
Total #: 3 Workspace Size:         115 Cached Size:          19


```

Later, you can recheck them in the workspace.

```console
$ xvc file recheck data6.txt

$ lsd -l data6.txt
.rw-rw-rw- [..] data6.txt

```
