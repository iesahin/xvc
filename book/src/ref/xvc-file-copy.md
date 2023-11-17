# xvc file copy

## Synopsis

```console
$ xvc file copy --help
Copy from source to another location in the workspace

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

Arguments:
  <SOURCE>
          Source file, glob or directory within the workspace.
          
          If the source ends with a slash, it's considered a directory and all files in that directory are copied.
          
          If the number of source files is more than one, the destination must be a directory.

  <DESTINATION>
          Location we copy file(s) to within the workspace.
          
          If the target ends with a slash, it's considered a directory and created if it doesn't exist.
          
          If the number of source files is more than one, the destination must be a directory.

Options:
      --recheck-method <RECHECK_METHOD>
          How the targets should be rechecked: One of copy, symlink, hardlink, reflink.
          
          Note: Reflink uses copy if the underlying file system doesn't support it.

      --force
          Force even if target exists

      --no-recheck
          Do not recheck the destination files This is useful when you want to copy only records, without updating the workspace

      --name-only
          When copying multiple files, by default whole path is copied to the destination. This option sets the destination to be created with the file name only

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

## Examples

This command is used to copy a set of files to another location in the workspace.

By default, it doesn't update the recheck method (cache type) of the targets.
It rechecks them to the destination with the same method.

`xvc file copy` works only with the tracked files.

```console
$ git init
...
$ xvc init

$ xvc file track data.txt

$ lsd -l
.rw-rw-rw- [..] data.txt

```

Once you add the file to the cache, you can copy the file to another location.

```console
$ xvc file copy data.txt data2.txt

$ ls
data.txt
data2.txt

```

Note that, multiple copies of the same content don't add up to the cache size.

```console
$ xvc file list data.txt
FC          19 [..] c85f3e81 c85f3e81 data.txt
Total #: 1 Workspace Size:          19 Cached Size:          19


$ xvc file list 'data*'
FC          19 [..] c85f3e81 c85f3e81 data2.txt
FC          19 [..] c85f3e81 c85f3e81 data.txt
Total #: 2 Workspace Size:          38 Cached Size:          19


```

Xvc can change the destination file's recheck method.

```console
$ xvc file copy data.txt data3.txt --as symlink

$ lsd -l
.rw-rw-rw- [..] data.txt
.rw-rw-rw- [..] data2.txt
lrwxr-xr-x [..] data3.txt ⇒ [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt

```

You can create _views_ of your data by copying it to another location.

```console
$ xvc file copy 'd*' another-set/ --as hardlink

$ xvc file list another-set/
FH          19 [..] c85f3e81 c85f3e81 another-set/data3.txt
FH          19 [..] c85f3e81 c85f3e81 another-set/data2.txt
FH          19 [..] c85f3e81 c85f3e81 another-set/data.txt
Total #: 3 Workspace Size:          57 Cached Size:          19


```

If the targets you specify are changed, Xvc cancels the copy operation. Please either recheck old versions or carry in new versions.

```console
$ perl -i -pe 's/a/ee/g' data.txt

$ xvc file copy data.txt data5.txt
[ERROR] File Error: Sources have changed, please carry-in or recheck following files before copying:
data.txt

```

You can copy files without them being in the workspace if they are in the cache.

```console
$ rm -f data.txt

$ xvc file copy data.txt data6.txt

$ lsd -l data6.txt
.rw-rw-rw- [..] data6.txt

```

You can also skip rechecking.
In this case, Xvc won't create any copies in the workspace, and you don't need them to be available in the cache.
They will be listed with `xvc file list` command.

```console
$ xvc file copy data.txt data7.txt --no-recheck

$ ls
another-set
data2.txt
data3.txt
data6.txt

$ xvc file list
XC             [..] c85f3e81          data7.txt
FC          19 [..] c85f3e81 c85f3e81 data6.txt
SS        [..] [..] c85f3e81          data3.txt
FC          19 [..] c85f3e81 c85f3e81 data2.txt
XC             [..] c85f3e81          data.txt
FH          19 [..] c85f3e81 c85f3e81 another-set/data3.txt
FH          19 [..] c85f3e81 c85f3e81 another-set/data2.txt
FH          19 [..] c85f3e81 c85f3e81 another-set/data.txt
DX         160 [..]                   another-set
FX         130 [..]          ac46bf74 .xvcignore
FX         [..] .gitignore
Total #: 11 Workspace Size:        [..] Cached Size:          19


```

Later, you can recheck them to work in the workspace.

```console
$ xvc file recheck data7.txt

$ lsd -l data7.txt
.rw-rw-rw- [..] data7.txt

```
