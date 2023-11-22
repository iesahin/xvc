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
├── chinese_mnist.zip
├── data.txt
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

5 directories, 12 files

```

Start by tracking a file.

```console
$ git init
...
$ xvc init

$ xvc file track dir-0001/file-0001.bin

$ lsd -l
.rw-rw-rw- [..] data.txt

```

Once you added the file to the cache, you can delete the workspace copy.

```console
$ rm dir-0001/file-0001.bin
$ ls -l dir-0001
total[..]
-rw-r--r--  1 iex  staff  10792680 Nov 22 11:35 chinese_mnist.zip
drwxr-xr-x  7 iex  staff       224 Nov 22 12:06 dir-0001
drwxr-xr-x  5 iex  staff       160 Nov 22 12:06 dir-0002

```

Then, recheck the file. By default, it makes a copy of the file.

```console
$ xvc file recheck data.txt

$ lsd -l
.rw-rw-rw- [..] data.txt

```

Xvc only updates the recheck method if the file is not changed.

```console
$ xvc file recheck data.txt --as symlink

$ ls -l data.txt
l[..] data.txt -> [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt

```

Symlink and hardlinks are read-only.
You can delete the symlink, and replace with an updated copy.
(As `perl -i` does below.)

```console
$ perl -i -pe 's/a/ee/g' data.txt

$ xvc file recheck data.txt --as copy
[ERROR] data.txt has changed on disk. Either carry in, force, or delete the target to recheck. 

$ rm data.txt

```

```console
$ xvc -vv file recheck data.txt --as hardlink
[INFO] [HARDLINK] [CWD]/.xvc/b3/c85/f3e/8108a0d53da6b4869e5532a3b72301ed58d5824ed1394d52dbcabe9496/0.txt -> [CWD]/data.txt

$ ls -l
total[..]
-rw-r--r--  1 iex  staff  10792680 Nov 22 11:35 chinese_mnist.zip
-r--r--r--@ 2 iex  staff        19 Oct 18 10:45 data.txt
drwxr-xr-x  7 iex  staff       224 Nov 22 12:06 dir-0001
drwxr-xr-x  5 iex  staff       160 Nov 22 12:06 dir-0002

```

Note that, as files in the cache are kept read-only, hardlinks and symlinks are also read only. Files rechecked as copy are made read-write explicitly.

Reflinks are supported by Xvc, but the underlying file system should also support it.
Otherwise it uses `copy`.

```console
$ rm -f data.txt
$ xvc file recheck data.txt --as reflink

```

The above command will create a read only link in macOS APFS and a copy in ext4 or NTFS file systems.

