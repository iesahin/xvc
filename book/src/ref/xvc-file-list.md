# xvc file list


## Synopsis 

```console
$ xvc file list --help
List tracked and untracked elements in the workspace

Usage: xvc file list [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to list.
          
          If not supplied, lists all files under the current directory.

Options:
  -f, --format <FORMAT>
          A string for each row of the output table
          
          The following are the keys for each row: - {{acd}}:  actual content digest. The hash of the workspace file's content.
          
          - {{aft}}:  actual file type. Whether the entry is a file (F), directory (D), symlink (S), hardlink (H) or reflink (R).
          
          - {{asz}}:  actual size. The size of the workspace file in bytes. It uses MB, GB and TB to represent sizes larger than 1MB.
          
          - {{ats}}:  actual timestamp. The timestamp of the workspace file.
          
          - {{name}}: The name of the file or directory.
          
          - {{cst}}:  cache status. One of "=", ">", "<", "X", or "?" to show whether the file timestamp is the same as the cached timestamp, newer, older, not cached or not tracked.
          
          - {{rcd}}:  recorded content digest. The hash of the cached content.
          
          - {{rct}}:  recorded cache type. Whether the entry is linked to the workspace as a copy (C), symlink (S), hardlink (H) or reflink (R).
          
          - {{rsz}}:  recorded size. The size of the cached content in bytes. It uses MB, GB and TB to represent sizes larged than 1MB.
          
          - {{rts}}:  recorded timestamp. The timestamp of the cached content.
          
          The default format can be set with file.list.format in the config file.

  -s, --sort-criteria <SORT_CRITERIA>
          Sort column.
          
          It can be one of none (default), name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
          
          The default option can be set with file.list.sort in the config file.

      --no-summary
          Don't show total number and size of the listed files.
          
          The default option can be set with file.list.no_summary in the config file.

  -h, --help
          Print help information (use `-h` for a summary)

```

## Examples

For these examples, we'll create a directory tree with five directories, each
having a file.

```console
$ xvc-test-helper create-directory-tree --directories 5 --files 5

$ tree
.
├── dir-0001
│   ├── file-0001.bin
│   ├── file-0002.bin
│   ├── file-0003.bin
│   ├── file-0004.bin
│   └── file-0005.bin
├── dir-0002
│   ├── file-0001.bin
│   ├── file-0002.bin
│   ├── file-0003.bin
│   ├── file-0004.bin
│   └── file-0005.bin
├── dir-0003
│   ├── file-0001.bin
│   ├── file-0002.bin
│   ├── file-0003.bin
│   ├── file-0004.bin
│   └── file-0005.bin
├── dir-0004
│   ├── file-0001.bin
│   ├── file-0002.bin
│   ├── file-0003.bin
│   ├── file-0004.bin
│   └── file-0005.bin
└── dir-0005
    ├── file-0001.bin
    ├── file-0002.bin
    ├── file-0003.bin
    ├── file-0004.bin
    └── file-0005.bin

6 directories, 25 files

```

`xvc file list` command works only in Xvc repositories. As we didn't initialize
a repository yet, it lists nothing.

```console
$ xvc file list 
```

Let's initialize the repository. 

```console
$ git init
...

$ xvc init

```

Now it lists all files and directories.

```console
$ xvc file list
FX        1005 2023-01-04 07:26:18   dir-0005/file-0005.bin           a5679e49
FX        1004 2023-01-04 07:26:18   dir-0005/file-0004.bin           d4f0f90f
FX        1003 2023-01-04 07:26:18   dir-0005/file-0003.bin           7fc33c28
FX        1002 2023-01-04 07:26:18   dir-0005/file-0002.bin           7a741ad2
FX        1001 2023-01-04 07:26:18   dir-0005/file-0001.bin           0afb8ab9
DX         224 2023-01-04 07:26:18   dir-0005                   
FX        1005 2023-01-04 07:26:18   dir-0004/file-0005.bin           a5679e49
FX        1004 2023-01-04 07:26:18   dir-0004/file-0004.bin           d4f0f90f
FX        1003 2023-01-04 07:26:18   dir-0004/file-0003.bin           7fc33c28
FX        1002 2023-01-04 07:26:18   dir-0004/file-0002.bin           7a741ad2
FX        1001 2023-01-04 07:26:18   dir-0004/file-0001.bin           0afb8ab9
DX         224 2023-01-04 07:26:18   dir-0004                   
FX        1005 2023-01-04 07:26:18   dir-0003/file-0005.bin           a5679e49
FX        1004 2023-01-04 07:26:18   dir-0003/file-0004.bin           d4f0f90f
FX        1003 2023-01-04 07:26:18   dir-0003/file-0003.bin           7fc33c28
FX        1002 2023-01-04 07:26:18   dir-0003/file-0002.bin           7a741ad2
FX        1001 2023-01-04 07:26:18   dir-0003/file-0001.bin           0afb8ab9
DX         224 2023-01-04 07:26:18   dir-0003                   
FX        1005 2023-01-04 07:26:18   dir-0002/file-0005.bin           a5679e49
FX        1004 2023-01-04 07:26:18   dir-0002/file-0004.bin           d4f0f90f
FX        1003 2023-01-04 07:26:18   dir-0002/file-0003.bin           7fc33c28
FX        1002 2023-01-04 07:26:18   dir-0002/file-0002.bin           7a741ad2
FX        1001 2023-01-04 07:26:18   dir-0002/file-0001.bin           0afb8ab9
DX         224 2023-01-04 07:26:18   dir-0002                   
FX        1005 2023-01-04 07:26:18   dir-0001/file-0005.bin           a5679e49
FX        1004 2023-01-04 07:26:18   dir-0001/file-0004.bin           d4f0f90f
FX        1003 2023-01-04 07:26:18   dir-0001/file-0003.bin           7fc33c28
FX        1002 2023-01-04 07:26:18   dir-0001/file-0002.bin           7a741ad2
FX        1001 2023-01-04 07:26:18   dir-0001/file-0001.bin           0afb8ab9
DX         224 2023-01-04 07:26:18   dir-0001                   
FX         130 2023-01-04 07:26:18   .xvcignore           ac46bf74
FX         107 2023-01-04 07:26:18   .gitignore           ce9fcf30
Total #: 32 Workspace Size:       26432 Cached Size:           0


```

With the default output format, the first two letters show the path type and
cache type, respectively. 

For example, if you track `dir-0001` as `copy`, the first letter is `F` for the
files and `D` for the directories. The second letter is `C` for files, meaning
the file is a copy of the cached file, and it's `X` for directories that means
they are not in the cache. Similar to Git, Xvc doesn't track only files and
directories are considered as collection of files.

```console
$ xvc file track dir-0001/

$ xvc file list dir-0001/
FC        1005 2023-01-04 07:26:18   dir-0001/file-0005.bin  a5679e49 a5679e49
FC        1004 2023-01-04 07:26:18   dir-0001/file-0004.bin  d4f0f90f d4f0f90f
FC        1003 2023-01-04 07:26:18   dir-0001/file-0003.bin  7fc33c28 7fc33c28
FC        1002 2023-01-04 07:26:18   dir-0001/file-0002.bin  7a741ad2 7a741ad2
FC        1001 2023-01-04 07:26:18   dir-0001/file-0001.bin  0afb8ab9 0afb8ab9
FX         149 2023-01-04 07:26:19   dir-0001/.gitignore           32f8bd0e
Total #: 6 Workspace Size:        5164 Cached Size:        5015


```

If you add another set of files as hardlinks to the cached copies, it will
print the second letter as `H`.

```console
$ xvc file track dir-0002 --cache-type hardlink
thread '<unnamed>' panicked at 'IoError { source: Os { code: 13, kind: PermissionDenied, message: "Permission denied" } }', file/src/carry_in/mod.rs:230:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'thread '<unnamed><unnamed>' panicked at '' panicked at '[PANIC] IoError { source: Os { code: 13, kind: PermissionDenied, message: "Permission denied" } }', lib/src/cli/mod.rs:329:52
thread 'thread '<unnamed><unnamed>' panicked at '' panicked at 'IoError { source: Os { code: 13, kind: PermissionDenied, message: "Permission denied" } }IoError { source: Os { code: 13, kind: PermissionDenied, message: "Permission denied" } }', ', file/src/carry_in/mod.rs:IoError { source: Os { code: 13, kind: PermissionDenied, message: "Permission denied" } }', file/src/carry_in/mod.rsfile/src/carry_in/mod.rs::230230::99
230
thread '<unnamed>' panicked at 'IoError { source: Os { code: 13, kind: PermissionDenied, message: "Permission denied" } }', file/src/carry_in/mod.rs::9230
:9
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', lib/src/cli/mod.rs:373:10

$ xvc file list dir-0002
FH        1005 2023-01-04 07:26:18   dir-0002/file-0005.bin  a5679e49 a5679e49
FH        1004 2023-01-04 07:26:18   dir-0002/file-0004.bin  d4f0f90f d4f0f90f
FH        1003 2023-01-04 07:26:18   dir-0002/file-0003.bin  7fc33c28 7fc33c28
FH        1002 2023-01-04 07:26:18   dir-0002/file-0002.bin  7a741ad2 7a741ad2
FH        1001 2023-01-04 07:26:18   dir-0002/file-0001.bin  0afb8ab9 0afb8ab9
FX         149 2023-01-04 07:26:19   dir-0002/.gitignore           f029ea39
Total #: 6 Workspace Size:        5164 Cached Size:        5015


```

Note, as hardlinks are actually files with the same inode in the file system
with alternative paths, they are detected as `F`. 

Symbolic links are typically reported as `SS` in the first letters. 
It means they are symbolic links on the file system and their cache type is also
symbolic links. 

```console
$ xvc file track dir-0003 --cache-type symlink
thread '<unnamed>' panicked at 'IoError { source: Os { code: 13, kind: PermissionDenied, message: "Permission denied" } }', file/src/carry_in/mod.rs:230:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at '[PANIC] IoError { source: Os { code: 13, kind: PermissionDenied, message: "Permission denied" } }', lib/src/cli/mod.rs:329:52
thread '<unnamed>' panicked at 'IoError { source: Os { code: 13, kind: PermissionDenied, message: "Permission denied" } }', file/src/carry_in/mod.rs:230:9
thread '<unnamed>' panicked at 'IoError { source: Os { code: 13, kind: PermissionDenied, message: "Permission denied" } }', file/src/carry_in/mod.rs:230:9
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', lib/src/cli/mod.rs:373:10
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', file/src/carry_in/mod.rs:230:9
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', file/src/carry_in/mod.rs:230:9

$ xvc file list dir-0003
FS        1005 [..]   dir-0003/file-0005.bin  a5679e49 a5679e49
FS        1004 [..]   dir-0003/file-0004.bin  d4f0f90f d4f0f90f
FS        1003 [..]   dir-0003/file-0003.bin  7fc33c28 7fc33c28
FS        1002 [..]   dir-0003/file-0002.bin  7a741ad2 7a741ad2
FS        1001 [..]   dir-0003/file-0001.bin  0afb8ab9 0afb8ab9
FX         149 [..]   dir-0003/.gitignore           933c8fce
Total #: 6 Workspace Size:        5164 Cached Size:        5015


```

Although not all filesystems support, `R` represents reflinks. 

### Sort options

You may sort `xvc file list` output by name, by modification time and by file
size. 
