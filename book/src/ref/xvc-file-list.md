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
          
          The following are the keys for each row:
          
          - {{acd8}}:  actual content digest from the workspace file. First 8 digits.
          
          - {{acd64}}:  actual content digest. All 64 digits.
          
          - {{aft}}:  actual file type. Whether the entry is a file (F), directory (D), symlink (S), hardlink (H) or reflink (R).
          
          - {{asz}}:  actual size. The size of the workspace file in bytes. It uses MB, GB and TB to represent sizes larger than 1MB.
          
          - {{ats}}:  actual timestamp. The timestamp of the workspace file.
          
          - {{name}}: The name of the file or directory.
          
          - {{cst}}:  cache status. One of "=", ">", "<", "X", or "?" to show whether the file timestamp is the same as the cached timestamp, newer, older, not cached or not tracked.
          
          - {{rcd8}}:  recorded content digest stored in the cache. First 8 digits.
          
          - {{rcd64}}:  recorded content digest stored in the cache. All 64 digits.
          
          - {{rct}}:  recorded cache type. Whether the entry is linked to the workspace as a copy (C), symlink (S), hardlink (H) or reflink (R).
          
          - {{rsz}}:  recorded size. The size of the cached content in bytes. It uses MB, GB and TB to represent sizes larged than 1MB.
          
          - {{rts}}:  recorded timestamp. The timestamp of the cached content.
          
          The default format can be set with file.list.format in the config file.

  -s, --sort <SORT>
          Sort criteria.
          
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
$ xvc-test-helper create-directory-tree --directories 5 --files 5 --fill 23

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

...

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
$ xvc file list --sort name-asc
FX         107 2023-01-16 10:59:30   .gitignore           ce9fcf30
FX         130 2023-01-16 10:59:30   .xvcignore           ac46bf74
DX         224 2023-01-16 10:59:30   dir-0001                   
FX        1001 2023-01-16 10:59:30   dir-0001/file-0001.bin           189fa49f
FX        1002 2023-01-16 10:59:30   dir-0001/file-0002.bin           8c079454
FX        1003 2023-01-16 10:59:30   dir-0001/file-0003.bin           2856fe70
FX        1004 2023-01-16 10:59:30   dir-0001/file-0004.bin           3640687a
FX        1005 2023-01-16 10:59:30   dir-0001/file-0005.bin           e23e79a0
DX         224 2023-01-16 10:59:30   dir-0002                   
FX        1001 2023-01-16 10:59:30   dir-0002/file-0001.bin           189fa49f
FX        1002 2023-01-16 10:59:30   dir-0002/file-0002.bin           8c079454
FX        1003 2023-01-16 10:59:30   dir-0002/file-0003.bin           2856fe70
FX        1004 2023-01-16 10:59:30   dir-0002/file-0004.bin           3640687a
FX        1005 2023-01-16 10:59:30   dir-0002/file-0005.bin           e23e79a0
DX         224 2023-01-16 10:59:30   dir-0003                   
FX        1001 2023-01-16 10:59:30   dir-0003/file-0001.bin           189fa49f
FX        1002 2023-01-16 10:59:30   dir-0003/file-0002.bin           8c079454
FX        1003 2023-01-16 10:59:30   dir-0003/file-0003.bin           2856fe70
FX        1004 2023-01-16 10:59:30   dir-0003/file-0004.bin           3640687a
FX        1005 2023-01-16 10:59:30   dir-0003/file-0005.bin           e23e79a0
DX         224 2023-01-16 10:59:30   dir-0004                   
FX        1001 2023-01-16 10:59:30   dir-0004/file-0001.bin           189fa49f
FX        1002 2023-01-16 10:59:30   dir-0004/file-0002.bin           8c079454
FX        1003 2023-01-16 10:59:30   dir-0004/file-0003.bin           2856fe70
FX        1004 2023-01-16 10:59:30   dir-0004/file-0004.bin           3640687a
FX        1005 2023-01-16 10:59:30   dir-0004/file-0005.bin           e23e79a0
DX         224 2023-01-16 10:59:30   dir-0005                   
FX        1001 2023-01-16 10:59:30   dir-0005/file-0001.bin           189fa49f
FX        1002 2023-01-16 10:59:30   dir-0005/file-0002.bin           8c079454
FX        1003 2023-01-16 10:59:30   dir-0005/file-0003.bin           2856fe70
FX        1004 2023-01-16 10:59:30   dir-0005/file-0004.bin           3640687a
FX        1005 2023-01-16 10:59:30   dir-0005/file-0005.bin           e23e79a0
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
FC        1005 [..]   dir-0001/file-0005.bin  e23e79a0 e23e79a0
FC        1004 [..]   dir-0001/file-0004.bin  3640687a 3640687a
FC        1003 [..]   dir-0001/file-0003.bin  2856fe70 2856fe70
FC        1002 [..]   dir-0001/file-0002.bin  8c079454 8c079454
FC        1001 [..]   dir-0001/file-0001.bin  189fa49f 189fa49f
Total #: 5 Workspace Size:        5015 Cached Size:        5015


```

If you add another set of files as hardlinks to the cached copies, it will
print the second letter as `H`.

```console
$ xvc file track dir-0002 --cache-type hardlink

$ xvc file list dir-0002
FH        1005 [..]   dir-0002/file-0005.bin  e23e79a0 e23e79a0
FH        1004 [..]   dir-0002/file-0004.bin  3640687a 3640687a
FH        1003 [..]   dir-0002/file-0003.bin  2856fe70 2856fe70
FH        1002 [..]   dir-0002/file-0002.bin  8c079454 8c079454
FH        1001 [..]   dir-0002/file-0001.bin  189fa49f 189fa49f
Total #: 5 Workspace Size:        5015 Cached Size:        5015


```

Note, as hardlinks are actually files with the same inode in the file system
with alternative paths, they are detected as `F`.

Symbolic links are typically reported as `SS` in the first letters.
It means they are symbolic links on the file system and their cache type is also
symbolic links.

```console
$ xvc file track dir-0003 --cache-type symlink

$ xvc file list dir-0003
SS         [..]   dir-0003/file-0005.bin  e23e79a0         
SS         [..]   dir-0003/file-0004.bin  3640687a         
SS         [..]   dir-0003/file-0003.bin  2856fe70         
SS         [..]   dir-0003/file-0002.bin  8c079454         
SS         [..]   dir-0003/file-0001.bin  189fa49f         
Total #: 5 Workspace Size:         [..] Cached Size:        5015


```

Although not all filesystems support, `R` represents reflinks.

## Globs

You may use globs to list files.

```console
$ xvc file list 'dir-*/*-0001.bin' 
FX        1001 [..]   dir-0005/file-0001.bin           189fa49f
FX        1001 [..]   dir-0004/file-0001.bin           189fa49f
SS         [..]   dir-0003/file-0001.bin  189fa49f         
FH        1001 [..]   dir-0002/file-0001.bin  189fa49f 189fa49f
FC        1001 [..]   dir-0001/file-0001.bin  189fa49f 189fa49f
Total #: 5 Workspace Size:        [..] Cached Size:        1001


```

Note that all these files are identical. They are cached once, and only one of
them takes space in the cache.

You can also use multiple targets as globs.

```console
$ xvc file list '*/*-0001.bin' '*/*-0002.bin' 
FX        1002 [..]   dir-0005/file-0002.bin           8c079454
FX        1001 [..]   dir-0005/file-0001.bin           189fa49f
FX        1002 [..]   dir-0004/file-0002.bin           8c079454
FX        1001 [..]   dir-0004/file-0001.bin           189fa49f
SS         [..]   dir-0003/file-0002.bin  8c079454         
SS         [..]   dir-0003/file-0001.bin  189fa49f         
FH        1002 [..]   dir-0002/file-0002.bin  8c079454 8c079454
FH        1001 [..]   dir-0002/file-0001.bin  189fa49f 189fa49f
FC        1002 [..]   dir-0001/file-0002.bin  8c079454 8c079454
FC        1001 [..]   dir-0001/file-0001.bin  189fa49f 189fa49f
Total #: 10 Workspace Size:        [..] Cached Size:        2003


```

## Sorting

You may sort `xvc file list` output by name, by modification time and by file
size.

Use `--sort` option to specify the sort criteria.

```console
$ xvc file list --sort name-desc dir-0001/
FC        1005 [..]   dir-0001/file-0005.bin  e23e79a0 e23e79a0
FC        1004 [..]   dir-0001/file-0004.bin  3640687a 3640687a
FC        1003 [..]   dir-0001/file-0003.bin  2856fe70 2856fe70
FC        1002 [..]   dir-0001/file-0002.bin  8c079454 8c079454
FC        1001 [..]   dir-0001/file-0001.bin  189fa49f 189fa49f
Total #: 5 Workspace Size:        5015 Cached Size:        5015


```

```console
$ xvc file list --sort name-asc dir-0001/
FC        1001 [..]   dir-0001/file-0001.bin  189fa49f 189fa49f
FC        1002 [..]   dir-0001/file-0002.bin  8c079454 8c079454
FC        1003 [..]   dir-0001/file-0003.bin  2856fe70 2856fe70
FC        1004 [..]   dir-0001/file-0004.bin  3640687a 3640687a
FC        1005 [..]   dir-0001/file-0005.bin  e23e79a0 e23e79a0
Total #: 5 Workspace Size:        5015 Cached Size:        5015


```

## Column Format

You can specify the columns that the command prints.

For example, if you only want to see the file names, use `{{name}}` as the
format string.

The following command sorts all files with their sizes in the workspace, and
prints their size and name.

```console
$ xvc file list --format '{{asz}} {{name}}' --sort size-desc dir-0001/
       1005 dir-0001/file-0005.bin
       1004 dir-0001/file-0004.bin
       1003 dir-0001/file-0003.bin
       1002 dir-0001/file-0002.bin
       1001 dir-0001/file-0001.bin
Total #: 5 Workspace Size:        5015 Cached Size:        5015


```

If you want to compare the recorded (cached) hashes and actual hashes in the workspace, you can use `{{acd}} {{rcd}} {{name}}` format string.

```console
$ xvc file list --format '{{acd8}} {{rcd8}} {{name}}' --sort ts-asc dir-0001
189fa49f 189fa49f dir-0001/file-0001.bin
8c079454 8c079454 dir-0001/file-0002.bin
2856fe70 2856fe70 dir-0001/file-0003.bin
3640687a 3640687a dir-0001/file-0004.bin
e23e79a0 e23e79a0 dir-0001/file-0005.bin
Total #: 5 Workspace Size:        5015 Cached Size:        5015


```

```admonish info
If `{{acd8}}` or `{{acd64}}` is not present in the format string, Xvc doesn't calculate these hashes. If you have large number of files where the default format (that includes actual content hashes) runs slowly, you may customize it to not to include these columns.
```

If you want to get a quick glimpse of what needs to carried in, or rechecked,
you can use cache status `{{cst}}` column.

```console
$ xvc-test-helper generate-random-file --size 100 --filename dir-0001/a-new-file.bin

$ xvc file list --format '{{cst}} {{name}}' dir-0001/
= dir-0001/file-0005.bin
= dir-0001/file-0004.bin
= dir-0001/file-0003.bin
= dir-0001/file-0002.bin
= dir-0001/file-0001.bin
X dir-0001/a-new-file.bin
Total #: 6 Workspace Size:        5115 Cached Size:        5015


```

The cache status column shows `=` for unchanged files in the cache, `X` for
untracked files, `>` for files that there is newer version in the cache, and `<`
for files that there is a newer version in the workspace. The comparison is done
between recorded timestamp and actual timestamp with an accuracy of 1 second.
