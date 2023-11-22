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
$ xvc-test-helper create-directory-tree --directories 2 --files 3 --seed 231123
$ xvc-test-helper create-directory-tree --root dir-0001 --directories 2 --files 2 --seed 231123
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
total 24
-rw-rw-rw- [..] file-0001.bin
-rw-rw-rw- [..] file-0002.bin
-rw-rw-rw- [..] file-0003.bin

```

You can update the recheck method of a file. Otherwise it will be kept as same before.

```console
$ rm -rf dir-0002/
$ xvc -v file recheck dir-0002/ --as symlink
$ ls -l dir-0002/
total 0
lrwxr-xr-x [..] file-0001.bin -> [CWD]/.xvc/b3/13d/5c5/3e0ecb114dc368d025b6533fe2fb0957aff14ea093bae1eb9c16a0fb34/0.bin
lrwxr-xr-x [..] file-0002.bin -> [CWD]/.xvc/b3/fac/78e/b458466ade07920b89179e446dd89db34118fe3be9e2e15f3bc9ec0615/0.bin
lrwxr-xr-x [..] file-0003.bin -> [CWD]/.xvc/b3/477/8e5/eb6f4fd2f0eb13448048d3184d0c5217d9cd35430560410988b7917a9c/0.bin

$ rm -rf dir-0002/
$ xvc -v file recheck dir-0002/ 

$ ls -l dir-0002/
total 0
lrwxr-xr-x [..] file-0001.bin -> [CWD]/.xvc/b3/13d/5c5/3e0ecb114dc368d025b6533fe2fb0957aff14ea093bae1eb9c16a0fb34/0.bin
lrwxr-xr-x [..] file-0002.bin -> [CWD]/.xvc/b3/fac/78e/b458466ade07920b89179e446dd89db34118fe3be9e2e15f3bc9ec0615/0.bin
lrwxr-xr-x [..] file-0003.bin -> [CWD]/.xvc/b3/477/8e5/eb6f4fd2f0eb13448048d3184d0c5217d9cd35430560410988b7917a9c/0.bin

```

Symlink and hardlinks are read-only.
You can delete the symlink, and recheck as copy to update.

```console
$ echo "120912" >> dir-0002/file-0001.bin
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
drwxr-xr-x  8 iex  staff  256 Nov 22 23:14 dir-0001
drwxr-xr-x  5 iex  staff  160 Nov 22 23:14 dir-0002

```

Note that, as files in the cache are kept read-only, hardlinks and symlinks are also read only. Files rechecked as copy are made read-write explicitly.

Reflinks are supported by Xvc, but the underlying file system should also support it.
Otherwise it uses `copy`.

```console
$ rm -f data.txt
$ xvc file recheck data.txt --as reflink

```

The above command will create a read only link in macOS APFS and a copy in ext4 or NTFS file systems.


