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
$ xvc-test-helper create-directory-tree --directories 5 --files 3 --seed 231123
$ tree
.
└── dir-0001
    ├── file-0001.bin
    ├── file-0002.bin
    └── file-0003.bin

2 directories, 3 files

```

Start by tracking files. 

```console
$ git init
...
$ xvc init

$ xvc file track dir-*

```

Once you added the file to the cache, you can delete the workspace copy.

```console
$ rm dir-0001/file-0001.bin
$ lsd -l dir-0001/file-*
total[..]
drwxr-xr-x [..] dir-0001
drwxr-xr-x [..] dir-0002

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
? 1
ls: dir-0002/: No such file or directory

```
You can use glob patterns to recheck files.
```console
```console
$ xvc file track 'dir-*'


You can update the recheck method of a file. Otherwise it will be kept as same before.

```console
$ rm -rf dir-0002/
$ xvc -v file recheck dir-0002/ --as symlink
$ ls -l dir-0002/
? 1
ls: dir-0002/: No such file or directory

$ rm -rf dir-0002/
$ xvc -v file recheck dir-0002/ 

$ ls -l dir-0002/
? 1
ls: dir-0002/: No such file or directory

```

Symlink and hardlinks are read-only.
You can recheck as copy to update.

```console
$ zsh -c 'echo "120912" >> dir-0002/file-0001.bin'
? 1
zsh:1: no such file or directory: dir-0002/file-0001.bin

$ xvc file recheck dir-0002/file-0001.bin --as copy

$ zsh -c 'echo "120912" >> dir-0002/file-0001.bin'
? 1
zsh:1: no such file or directory: dir-0002/file-0001.bin

```
Note that, as files in the cache are kept read-only, hardlinks and symlinks are also read only. Files rechecked as copy are made read-write explicitly.

```console
$ xvc -vv file recheck data.txt --as hardlink

$ ls -l
total[..]
drwxr-xr-x [..] dir-0001

```

Reflinks are supported by Xvc, but the underlying file system should also support it.
Otherwise it uses `copy`.

```console
$ rm -f data.txt
$ xvc file recheck data.txt --as reflink

```

The above command will create a read only link in macOS APFS and a copy in ext4 or NTFS file systems.


