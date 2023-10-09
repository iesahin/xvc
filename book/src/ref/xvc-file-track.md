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
$ xvc-test-helper create-directory-tree --directories 4 --files 3  --seed 20231021
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
├── dir-0003
│   ├── file-0001.bin
│   ├── file-0002.bin
│   └── file-0003.bin
└── dir-0004
    ├── file-0001.bin
    ├── file-0002.bin
    └── file-0003.bin

5 directories, 12 files

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
.xvc/b3
├── 493
│   └── eeb
│       └── 6525ea5e94e1e760371108e4a525c696c773a774a4818e941fd6d1af79
│           └── 0.bin
├── ab3
│   └── 619
│       └── 814cae0456a5a291e4d5c8d339a8389630e476f9f9e8d3a09accc919f0
│           └── 0.bin
└── e51
    └── 7d6
        └── b9a3617fdcd96bd128142a39f1eca26ed77a338d2b93ba4921a0116c70
            └── 0.bin

10 directories, 3 files

```

There are different _recheck (checkout) methods_ that Xvc connects the
workspace file to the cache. The default method for this is copying the file to
the workspace. This way a separate copy of the cache file is created in the workspace. 

If you want to make this connection with symbolic links, you can specify it with `--recheck-method` option. 

```console
$ xvc file track --recheck-method symlink dir-0003/file-0001.bin
$ ls -l dir-0003/file-0001.bin
lrwxr-xr-x  1 iex  staff  181 Oct  9 11:40 dir-0003/file-0001.bin -> [CWD]/.xvc/b3/e51/7d6/b9a3617fdcd96bd128142a39f1eca26ed77a338d2b93ba4921a0116c70/0.bin

```

You can also use `--hardlink` and `--reflink` options. Please see [`xvc file recheck`](/ref/xvc-file-recheck/) reference for details.  

```console
$ xvc file track --recheck-method hardlink dir-0003/file-0002.bin
$ xvc file track --recheck-method reflink dir-0003/file-0003.bin
$ ls -l dir-0003/
total 16
lrwxr-xr-x  1 iex  staff   181 Oct  9 11:40 file-0001.bin -> [CWD]/.xvc/b3/e51/7d6/b9a3617fdcd96bd128142a39f1eca26ed77a338d2b93ba4921a0116c70/0.bin
-r--r--r--  2 iex  staff  2002 Oct  9 11:40 file-0002.bin
-r--r--r--  1 iex  staff  2003 Oct  9 11:40 file-0003.bin

```


```admonish info
Note that, unlike DVC that specifies checkout/recheck option repository wide,
Xvc lets you specify per file. You can recheck files data files as symbolic
links (which are non-writable) and save space and make model files as copies of
the cached original and commit (carry-in) every time they change.

```

When you track a file in Xvc, it's automatically commit (carry-in) to the cache
directory. If you want to postpone this operation and don't need a cached copy
for a file, you can use `--no-commit` option. You can later use [xvc file
carry-in](/ref/xvc-file-carry-in) command to move these files to the repository
cache.  

```console
$ xvc file track --no-commit --recheck-method symlink dir-0004/
$ ls -l dir-0004/
total 24
-rw-r--r--  1 iex  staff  2001 Oct  9 11:40 file-0001.bin
-rw-r--r--  1 iex  staff  2002 Oct  9 11:40 file-0002.bin
-rw-r--r--  1 iex  staff  2003 Oct  9 11:40 file-0003.bin

$ xvc file list dir-0004/
FC        2003 2023-10-09 08:40:14 ab361981 ab361981 dir-0004/file-0003.bin
FC        2002 2023-10-09 08:40:14 493eeb65 493eeb65 dir-0004/file-0002.bin
FC        2001 2023-10-09 08:40:14 e517d6b9 e517d6b9 dir-0004/file-0001.bin
Total #: 3 Workspace Size:        6006 Cached Size:        6006


```
You can carry-in (commit) these files to the cache with `xvc file carry-in` command. 

```console
$ xvc file carry-in dir-0004/
? 2
error: unexpected argument '--recheck-method' found

  tip: to pass '--recheck-method' as a value, use '-- --recheck-method'

Usage: xvc file carry-in <--text-or-binary <TEXT_OR_BINARY>|--force|--no-parallel|TARGETS>

For more information, try '--help'.

$ ls -l dir-0004/
total 24
-rw-r--r--  1 iex  staff  2001 Oct  9 11:40 file-0001.bin
-rw-r--r--  1 iex  staff  2002 Oct  9 11:40 file-0002.bin
-rw-r--r--  1 iex  staff  2003 Oct  9 11:40 file-0003.bin

```


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



