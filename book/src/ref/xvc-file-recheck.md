# xvc file recheck

## Synopsis

```console
$ xvc file recheck --help
Get files from cache by copy or *link

Usage: xvc file recheck [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to recheck

Options:
      --recheck-method <RECHECK_METHOD>
          How to track the file contents in cache: One of copy, symlink, hardlink, reflink.
          
          Note: Reflink uses copy if the underlying file system doesn't support it.

      --no-parallel
          Don't use parallelism

      --force
          Force even if target exists

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

This command has an alias [`xvc file checkout`](/ref/xvc-file-checkout.md) if you feel more at home with Git terminology.

## Examples

Rechecking is analogous to [git checkout](https://git-scm.com/docs/git-checkout).
It copies or links a cached file to the workspace.

Let's create an example directory hierarchy as a showcase. 

```console
$ xvc-test-helper create-directory-tree --directories 2 --files 3
$ xvc-test-helper create-directory-tree --root dir-0001 --directories 2 --files 2
$ tree
.
├── dir-0001
│   ├── dir-0001
│   │   ├── file-0001.bin
│   │   └── file-0002.bin
│   ├── dir-0002
│   │   ├── file-0001.bin
│   │   └── file-0002.bin
│   ├── file-0001.bin
│   ├── file-0002.bin
│   └── file-0003.bin
└── dir-0002
    ├── file-0001.bin
    ├── file-0002.bin
    └── file-0003.bin

5 directories, 10 files

```

Start by tracking a file.

```console
$ git init
...
$ xvc init

$ xvc file track dir-0001/file-0001.bin

$ lsd -l dir-0001/*
.rw-rw-rw- [..] data.txt

```

Once you added the file to the cache, you can delete the workspace copy.

```console
$ rm dir-0001/file-0001.bin
$ lsd -l dir-0001/*
total[..]
-rw-r--r--  1 iex  staff  10792680 Nov 22 11:35 chinese_mnist.zip
drwxr-xr-x  7 iex  staff       224 Nov 22 12:06 dir-0001
drwxr-xr-x  5 iex  staff       160 Nov 22 12:06 dir-0002

```

Then, recheck the file. By default, it makes a copy of the file.

```console
$ xvc file recheck dir-0001/file-0001.bin

$ lsd -l
.rw-rw-rw- [..] data.txt

```

You can track and recheck complete directories

```console
$ xvc file track dir-0002/
$ rm -rf dir-0002/
$ xvc -v file recheck dir-0002/
$ ls -l dir-0002/
total 24
-rw-rw-rw-  1 iex  staff  2001 Nov 22 20:45 file-0001.bin
-rw-rw-rw-  1 iex  staff  2002 Nov 22 20:45 file-0002.bin
-rw-rw-rw-  1 iex  staff  2003 Nov 22 20:45 file-0003.bin

```

You can update the recheck method of a file. Otherwise it will be kept as same before.

```console
$ rm -rf dir-0002/
$ xvc -v file recheck dir-0002/ --as symlink
$ ls -l dir-0002/
$ rm -rf dir-0002/
$ xvc -v file recheck dir-0002/ 
$ ls -l dir-0002/
```



```console
$ xvc file recheck data.txt --as symlink

$ ls -l data.txt
? 1
ls: data.txt: No such file or directory

```

Symlink and hardlinks are read-only.
You can delete the symlink, and replace with an updated copy.
(As `perl -i` does below.)

```console
$ perl -i -pe 's/a/ee/g' data.txt
Can't open data.txt: No such file or directory.

$ xvc file recheck data.txt --as copy

$ rm data.txt
? 1
rm: data.txt: No such file or directory

```

```console
$ xvc -vv file recheck data.txt --as hardlink

$ ls -l
total[..]
drwxr-xr-x  8 iex  staff  256 Nov 22 20:45 dir-0001
drwxr-xr-x  5 iex  staff  160 Nov 22 20:46 dir-0002

```

Note that, as files in the cache are kept read-only, hardlinks and symlinks are also read only. Files rechecked as copy are made read-write explicitly.

Reflinks are supported by Xvc, but the underlying file system should also support it.
Otherwise it uses `copy`.

```console
$ rm -f data.txt
$ xvc file recheck data.txt --as reflink

```

The above command will create a read only link in macOS APFS and a copy in ext4 or NTFS file systems.


