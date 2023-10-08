# xvc file track

## Purpose

`xvc file track` is used to register any kind of file to Xvc for tracking versions.

## Synopsis 

```console
$ xvc file track --help
Add file and directories to Xvc

Usage: xvc file track [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to track

Options:
      --recheck-method <RECHECK_METHOD>
          How to track the file contents in cache: One of copy, symlink, hardlink, reflink.
          
          Note: Reflink uses copy if the underlying file system doesn't support it.

      --no-commit
          Do not copy/link added files to the file cache

      --text-or-binary <TEXT_OR_BINARY>
          Calculate digests as text or binary file without checking contents, or by automatically. (Default: auto)

      --force
          Add targets even if they are already tracked

      --no-parallel
          Don't use parallelism

  -h, --help
          Print help (see a summary with '-h')

```

## Examples


File tracking works only in Xvc repositories.

```console
$ git init
...
$ xvc init

```
Let's create a directory tree for these examples. 

```console
$ xvc-test-helper create-directory-tree --directories 3 --files 3  --seed 20231021
$ tree
.
├── dir-0001
│   ├── file-0001.bin
│   ├── file-0002.bin
│   └── file-0003.bin
├── dir-0002
│   ├── file-0001.bin
│   ├── file-0002.bin
│   └── file-0003.bin
└── dir-0003
    ├── file-0001.bin
    ├── file-0002.bin
    └── file-0003.bin

4 directories, 9 files

```

By default, the command runs similar to `git add` and `git commit`. 

You can track individual files.

```console
$ xvc file track dir-0001/file-0001.bin
```

You can track directories with the same command. 

```console
$ xvc file track dir-0002/
```

You can specify more than one target in a single command. 

```console
$ xvc file track dir-0001/file-0002.bin dir-0001/file-0003.bin
```

When you track a file, Xvc moves the file to the cache directory under `.xvc/`
and _connects_ the workspace file with the cached file. This _connection_ is
called rechecking and analogous to Git checkout. For example, the above
commands create a directory tree under `.xvc` as follows: 

```console
$ tree .xvc/b3
? 2
.xvc/b2  [error opening dir]

0 directories, 0 files

```

There are different _recheck (checkout) methods_ that Xvc connects the
workspace file to the cache. The default method for this is copying the file to
the workspace. This way a separate copy of the cache file is created in the workspace. 

If you want to make this connection with symbolic links, you can specify it with `--recheck-method` option. 

```console
$ xvc file track --recheck-method symlink dir-0003/file-0001.bin
$ ls -l dir-0003/file-0001.bin
lrwxr-xr-x  1 iex  staff  179 Oct  8 17:29 dir-0003/file-0001.bin -> [CWD]/.xvc/b3/e51/7d6/b9a3617fdcd96bd128142a39f1eca26ed77a338d2b93ba4921a0116c70/0.bin

```

You can also use `--hardlink` and `--reflink` options. Please see [`xvc file recheck`](/ref/xvc-file-recheck/) reference for details.  

```console
$ xvc file track --recheck-method hardlink dir-0003/file-0002.bin
$ xvc file track --recheck-method reflink dir-0003/file-0003.bin
$ ls -l dir-0003/
```


```admonish info
Note that, unlike DVC that specifies checkout/recheck option repository wide,
Xvc lets you specify per file. You can recheck files data files as symbolic
links (which are non-writable) and save space and make model files as copies of
the cached original and commit (carry-in) every time they change.

```

When you track a file in Xvc, it's automatically commit (carry-in) to the cache
directory. If you want to postpone this operation and don't need a cached copy
for a file, you can use `--no-commit` option.


## Caveats

- This command doesn't discriminate symbolic links or hardlinks. 
Links are followed and any broken links may cause errors. 

- Under the hood, Xvc tracks only the files, not directories. 
Directories are considered as path collections.
It doesn't matter if you track a directory or files in it separately.

## Technical Details

- Detecting changes in files and directories employ different kinds of [associated digests](/concepts/associated-digest.md).
If a file has different metadata digest, its content digest is calculated.
If file's content digest has changed, the file is considered changed.
A directory that contains different set of files, or files with changed content is considered changed.



