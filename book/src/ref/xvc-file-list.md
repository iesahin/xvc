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
FX        1005 2023-01-04 03:40:33   dir-0005/file-0005.bin           b6e61654
FX        1004 2023-01-04 03:40:33   dir-0005/file-0004.bin           084efe94
FX        1003 2023-01-04 03:40:33   dir-0005/file-0003.bin           8dbfe4b4
FX        1002 2023-01-04 03:40:33   dir-0005/file-0002.bin           591d98e1
FX        1001 2023-01-04 03:40:33   dir-0005/file-0001.bin           b5a1cf4a
DX         224 2023-01-04 03:40:33   dir-0005                   
FX        1005 2023-01-04 03:40:33   dir-0004/file-0005.bin           a0d7be02
FX        1004 2023-01-04 03:40:33   dir-0004/file-0004.bin           295b4394
FX        1003 2023-01-04 03:40:33   dir-0004/file-0003.bin           68d4a3f4
FX        1002 2023-01-04 03:40:33   dir-0004/file-0002.bin           8496fe40
FX        1001 2023-01-04 03:40:33   dir-0004/file-0001.bin           fc45416d
DX         224 2023-01-04 03:40:33   dir-0004                   
FX        1005 2023-01-04 03:40:33   dir-0003/file-0005.bin           6faa48b6
FX        1004 2023-01-04 03:40:33   dir-0003/file-0004.bin           ccfea0c5
FX        1003 2023-01-04 03:40:33   dir-0003/file-0003.bin           346b1673
FX        1002 2023-01-04 03:40:33   dir-0003/file-0002.bin           26cc18c8
FX        1001 2023-01-04 03:40:33   dir-0003/file-0001.bin           18579f42
DX         224 2023-01-04 03:40:33   dir-0003                   
FX        1005 2023-01-04 03:40:33   dir-0002/file-0005.bin           17a273eb
FX        1004 2023-01-04 03:40:33   dir-0002/file-0004.bin           4e28dc0a
FX        1003 2023-01-04 03:40:33   dir-0002/file-0003.bin           7d9a5037
FX        1002 2023-01-04 03:40:33   dir-0002/file-0002.bin           ff57d975
FX        1001 2023-01-04 03:40:33   dir-0002/file-0001.bin           6d98a860
DX         224 2023-01-04 03:40:33   dir-0002                   
FX        1005 2023-01-04 03:40:33   dir-0001/file-0005.bin           09469916
FX        1004 2023-01-04 03:40:33   dir-0001/file-0004.bin           3efcabea
FX        1003 2023-01-04 03:40:33   dir-0001/file-0003.bin           68204256
FX        1002 2023-01-04 03:40:33   dir-0001/file-0002.bin           80196c4f
FX        1001 2023-01-04 03:40:33   dir-0001/file-0001.bin           42efd7b5
DX         224 2023-01-04 03:40:33   dir-0001                   
FX         130 2023-01-04 03:40:33   .xvcignore           ac46bf74
FX         107 2023-01-04 03:40:33   .gitignore           ce9fcf30
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
FX        1005 2023-01-04 03:40:33   dir-0001/file-0005.bin           09469916
FX        1004 2023-01-04 03:40:33   dir-0001/file-0004.bin           3efcabea
FX        1003 2023-01-04 03:40:33   dir-0001/file-0003.bin           68204256
FX        1002 2023-01-04 03:40:33   dir-0001/file-0002.bin           80196c4f
FX        1001 2023-01-04 03:40:33   dir-0001/file-0001.bin           42efd7b5
FX         149 2023-01-04 03:40:34   dir-0001/.gitignore           f9208cb4
Total #: 6 Workspace Size:        5164 Cached Size:           0


```

If you add another set of files as hardlinks to the cached copies, it will
print the second letter as `H`.

```console
$ xvc file track dir-0002 --cache-type hardlink

$ xvc file list dir-0002
FX        1005 2023-01-04 03:40:33   dir-0005/file-0005.bin           b6e61654
FX        1004 2023-01-04 03:40:33   dir-0005/file-0004.bin           084efe94
FX        1003 2023-01-04 03:40:33   dir-0005/file-0003.bin           8dbfe4b4
FX        1002 2023-01-04 03:40:33   dir-0005/file-0002.bin           591d98e1
FX        1001 2023-01-04 03:40:33   dir-0005/file-0001.bin           b5a1cf4a
DX         224 2023-01-04 03:40:33   dir-0005                   
FX        1005 2023-01-04 03:40:33   dir-0004/file-0005.bin           a0d7be02
FX        1004 2023-01-04 03:40:33   dir-0004/file-0004.bin           295b4394
FX        1003 2023-01-04 03:40:33   dir-0004/file-0003.bin           68d4a3f4
FX        1002 2023-01-04 03:40:33   dir-0004/file-0002.bin           8496fe40
FX        1001 2023-01-04 03:40:33   dir-0004/file-0001.bin           fc45416d
DX         224 2023-01-04 03:40:33   dir-0004                   
FX        1005 2023-01-04 03:40:33   dir-0003/file-0005.bin           6faa48b6
FX        1004 2023-01-04 03:40:33   dir-0003/file-0004.bin           ccfea0c5
FX        1003 2023-01-04 03:40:33   dir-0003/file-0003.bin           346b1673
FX        1002 2023-01-04 03:40:33   dir-0003/file-0002.bin           26cc18c8
FX        1001 2023-01-04 03:40:33   dir-0003/file-0001.bin           18579f42
DX         224 2023-01-04 03:40:33   dir-0003                   
FH        1005 2023-01-04 03:40:33   dir-0002/file-0005.bin  17a273eb 17a273eb
FH        1004 2023-01-04 03:40:33   dir-0002/file-0004.bin  4e28dc0a 4e28dc0a
FH        1003 2023-01-04 03:40:33   dir-0002/file-0003.bin  7d9a5037 7d9a5037
FH        1002 2023-01-04 03:40:33   dir-0002/file-0002.bin  ff57d975 ff57d975
FH        1001 2023-01-04 03:40:33   dir-0002/file-0001.bin  6d98a860 6d98a860
FX         149 2023-01-04 03:40:35   dir-0002/.gitignore           80093db8
DX         256 2023-01-04 03:40:35   dir-0002                   
FC        1005 2023-01-04 03:40:33   dir-0001/file-0005.bin  09469916 09469916
FC        1004 2023-01-04 03:40:33   dir-0001/file-0004.bin  3efcabea 3efcabea
FC        1003 2023-01-04 03:40:33   dir-0001/file-0003.bin  68204256 68204256
FC        1002 2023-01-04 03:40:33   dir-0001/file-0002.bin  80196c4f 80196c4f
FC        1001 2023-01-04 03:40:33   dir-0001/file-0001.bin  42efd7b5 42efd7b5
FX         149 2023-01-04 03:40:34   dir-0001/.gitignore           f9208cb4
DX         256 2023-01-04 03:40:34   dir-0001                   
FX         130 2023-01-04 03:40:33   .xvcignore           ac46bf74
FX         107 2023-01-04 03:40:33   .gitignore           ce9fcf30
Total #: 34 Workspace Size:       26794 Cached Size:       10030


```

Note, as hardlinks are actually files with the same inode in the file system
with alternative paths, they are detected as `F`. 

Symbolic links are typically reported as `SS` in the first letters. 
It means they are symbolic links on the file system and their cache type is also
symbolic links. 

```console
$ xvc file track dir-0003 --cache-type symlink

$ xvc file list dir-0003
FX        1005 2023-01-04 03:40:33   dir-0005/file-0005.bin           b6e61654
FX        1004 2023-01-04 03:40:33   dir-0005/file-0004.bin           084efe94
FX        1003 2023-01-04 03:40:33   dir-0005/file-0003.bin           8dbfe4b4
FX        1002 2023-01-04 03:40:33   dir-0005/file-0002.bin           591d98e1
FX        1001 2023-01-04 03:40:33   dir-0005/file-0001.bin           b5a1cf4a
DX         224 2023-01-04 03:40:33   dir-0005                   
FX        1005 2023-01-04 03:40:33   dir-0004/file-0005.bin           a0d7be02
FX        1004 2023-01-04 03:40:33   dir-0004/file-0004.bin           295b4394
FX        1003 2023-01-04 03:40:33   dir-0004/file-0003.bin           68d4a3f4
FX        1002 2023-01-04 03:40:33   dir-0004/file-0002.bin           8496fe40
FX        1001 2023-01-04 03:40:33   dir-0004/file-0001.bin           fc45416d
DX         224 2023-01-04 03:40:33   dir-0004                   
SS         180 2023-01-04 03:40:35   dir-0003/file-0005.bin  6faa48b6         
SS         180 2023-01-04 03:40:35   dir-0003/file-0004.bin  ccfea0c5         
SS         180 2023-01-04 03:40:35   dir-0003/file-0003.bin  346b1673         
SS         180 2023-01-04 03:40:35   dir-0003/file-0002.bin  26cc18c8         
SS         180 2023-01-04 03:40:35   dir-0003/file-0001.bin  18579f42         
FX         149 2023-01-04 03:40:35   dir-0003/.gitignore           9a4a84b1
DX         256 2023-01-04 03:40:35   dir-0003                   
FH        1005 2023-01-04 03:40:33   dir-0002/file-0005.bin  17a273eb 17a273eb
FH        1004 2023-01-04 03:40:33   dir-0002/file-0004.bin  4e28dc0a 4e28dc0a
FH        1003 2023-01-04 03:40:33   dir-0002/file-0003.bin  7d9a5037 7d9a5037
FH        1002 2023-01-04 03:40:33   dir-0002/file-0002.bin  ff57d975 ff57d975
FH        1001 2023-01-04 03:40:33   dir-0002/file-0001.bin  6d98a860 6d98a860
FX         149 2023-01-04 03:40:35   dir-0002/.gitignore           80093db8
DX         256 2023-01-04 03:40:35   dir-0002                   
FC        1005 2023-01-04 03:40:33   dir-0001/file-0005.bin  09469916 09469916
FC        1004 2023-01-04 03:40:33   dir-0001/file-0004.bin  3efcabea 3efcabea
FC        1003 2023-01-04 03:40:33   dir-0001/file-0003.bin  68204256 68204256
FC        1002 2023-01-04 03:40:33   dir-0001/file-0002.bin  80196c4f 80196c4f
FC        1001 2023-01-04 03:40:33   dir-0001/file-0001.bin  42efd7b5 42efd7b5
FX         149 2023-01-04 03:40:34   dir-0001/.gitignore           f9208cb4
DX         256 2023-01-04 03:40:34   dir-0001                   
FX         130 2023-01-04 03:40:33   .xvcignore           ac46bf74
FX         107 2023-01-04 03:40:33   .gitignore           ce9fcf30
Total #: 35 Workspace Size:       22860 Cached Size:       15045


```

Although not all filesystems support, `R` represents reflinks. 

### Sort options

You may sort `xvc file list` output by name, by modification time and by file
size. 