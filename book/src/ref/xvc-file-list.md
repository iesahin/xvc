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
          - {{aft}}:  actual file type. Whether the entry is a file (F), directory (D),
            symlink (S), hardlink (H) or reflink (R).
          - {{asz}}:  actual size. The size of the workspace file in bytes. It uses MB,
            GB and TB to represent sizes larger than 1MB.
          - {{ats}}:  actual timestamp. The timestamp of the workspace file.
          - {{name}}: The name of the file or directory.
          - {{cst}}:  cache status. One of "=", ">", "<", "X", or "?" to show
            whether the file timestamp is the same as the cached timestamp, newer,
            older, not cached or not tracked.
          - {{rcd8}}:  recorded content digest stored in the cache. First 8 digits.
          - {{rcd64}}:  recorded content digest stored in the cache. All 64 digits.
          - {{rrm}}:  recorded recheck method. Whether the entry is linked to the workspace
            as a copy (C), symlink (S), hardlink (H) or reflink (R).
          - {{rsz}}:  recorded size. The size of the cached content in bytes. It uses
            MB, GB and TB to represent sizes larged than 1MB.
          - {{rts}}:  recorded timestamp. The timestamp of the cached content.
          
          The default format can be set with file.list.format in the config file.

  -s, --sort <SORT>
          Sort criteria.
          
          It can be one of none (default), name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
          
          The default option can be set with file.list.sort in the config file.

      --no-summary
          Don't show total number and size of the listed files.
          
          The default option can be set with file.list.no_summary in the config file.

  -a, --show-dot-files
          Don't hide dot files
          
          If not supplied, hides dot files like .gitignore and .xvcignore

  -h, --help
          Print help (see a summary with '-h')

```

## Examples

For these examples, we'll create a directory tree with five directories, each
having a file.

```console
$ xvc-test-helper create-directory-tree --directories 5 --files 5 --seed 20230213

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

[..] directories, 25 files

```

`xvc file list` command works only in Xvc repositories. As we didn't initialize
a repository yet, it reports an error.

```console
$ xvc file list
[ERROR] File Error: [E2004] Requires xvc repository.

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
DX         224 [..]                   dir-0001
FX        2001 [..]          1953f05d dir-0001/file-0001.bin
FX        2002 [..]          7e807161 dir-0001/file-0002.bin
FX        2003 [..]          d2432259 dir-0001/file-0003.bin
FX        2004 [..]          63535612 dir-0001/file-0004.bin
FX        2005 [..]          447933dc dir-0001/file-0005.bin
DX         224 [..]                   dir-0002
FX        2001 [..]          1953f05d dir-0002/file-0001.bin
FX        2002 [..]          7e807161 dir-0002/file-0002.bin
FX        2003 [..]          d2432259 dir-0002/file-0003.bin
FX        2004 [..]          63535612 dir-0002/file-0004.bin
FX        2005 [..]          447933dc dir-0002/file-0005.bin
DX         224 [..]                   dir-0003
FX        2001 [..]          1953f05d dir-0003/file-0001.bin
FX        2002 [..]          7e807161 dir-0003/file-0002.bin
FX        2003 [..]          d2432259 dir-0003/file-0003.bin
FX        2004 [..]          63535612 dir-0003/file-0004.bin
FX        2005 [..]          447933dc dir-0003/file-0005.bin
DX         224 [..]                   dir-0004
FX        2001 [..]          1953f05d dir-0004/file-0001.bin
FX        2002 [..]          7e807161 dir-0004/file-0002.bin
FX        2003 [..]          d2432259 dir-0004/file-0003.bin
FX        2004 [..]          63535612 dir-0004/file-0004.bin
FX        2005 [..]          447933dc dir-0004/file-0005.bin
DX         224 [..]                   dir-0005
FX        2001 [..]          1953f05d dir-0005/file-0001.bin
FX        2002 [..]          7e807161 dir-0005/file-0002.bin
FX        2003 [..]          d2432259 dir-0005/file-0003.bin
FX        2004 [..]          63535612 dir-0005/file-0004.bin
FX        2005 [..]          447933dc dir-0005/file-0005.bin
Total #: 30 Workspace Size:       51195 Cached Size:           0


```

By default the command hides dotfiles. If you also want to show them, you can use `--show-dot-files`/`-a` flag. 

```console
$ xvc file list --sort name-asc --show-dot-files
```

## Output Format

With the default output format, the first two letters show the path type and
recheck method, respectively.

For example, if you track `dir-0001` as `copy`, the first letter is `F` for the
files and `D` for the directories. The second letter is `C` for files, meaning
the file is a copy of the cached file, and it's `X` for directories that means
they are not in the cache. Similar to Git, Xvc doesn't track only files and
directories are considered as collection of files.

```console
$ xvc file track dir-0001/

$ xvc file list dir-0001/
FC        2005 [..] 447933dc 447933dc dir-0001/file-0005.bin
FC        2004 [..] 63535612 63535612 dir-0001/file-0004.bin
FC        2003 [..] d2432259 d2432259 dir-0001/file-0003.bin
FC        2002 [..] 7e807161 7e807161 dir-0001/file-0002.bin
FC        2001 [..] 1953f05d 1953f05d dir-0001/file-0001.bin
Total #: 5 Workspace Size:       10015 Cached Size:       10015


```

If you add another set of files as hardlinks to the cached copies, it will
print the second letter as `H`.

```console
$ xvc file track dir-0002 --recheck-method hardlink

$ xvc file list dir-0002
FH        2005 [..] 447933dc 447933dc dir-0002/file-0005.bin
FH        2004 [..] 63535612 63535612 dir-0002/file-0004.bin
FH        2003 [..] d2432259 d2432259 dir-0002/file-0003.bin
FH        2002 [..] 7e807161 7e807161 dir-0002/file-0002.bin
FH        2001 [..] 1953f05d 1953f05d dir-0002/file-0001.bin
Total #: 5 Workspace Size:       10015 Cached Size:       10015


```

Note, as hardlinks are files with the same inode in the file system
with alternative paths, they are detected as `F`.

Symbolic links are typically reported as `SS` in the first letters.
It means they are symbolic links on the file system and their recheck method is also
symbolic links.

```console
$ xvc file track dir-0003 --recheck-method symlink

$ xvc file list dir-0003
SS         [..] 447933dc          dir-0003/file-0005.bin
SS         [..] 63535612          dir-0003/file-0004.bin
SS         [..] d2432259          dir-0003/file-0003.bin
SS         [..] 7e807161          dir-0003/file-0002.bin
SS         [..] 1953f05d          dir-0003/file-0001.bin
Total #: 5 Workspace Size:         [..] Cached Size:       10015


```

Although not all filesystems support it, `R` represents reflinks.

## Globs

You may use globs to list files.

```console
$ xvc file list 'dir-*/*-0001.bin'
FX        2001 [..]          1953f05d dir-0005/file-0001.bin
FX        2001 [..]          1953f05d dir-0004/file-0001.bin
SS         [..] 1953f05d          dir-0003/file-0001.bin
FH        2[..] 1953f05d 1953f05d dir-0002/file-0001.bin
FC        2[..] 1953f05d 1953f05d dir-0001/file-0001.bin
Total #: 5 Workspace Size:        [..] Cached Size:        2001


```

Note that all these files are identical. They are cached once, and only one of
them takes space in the cache.

You can also use multiple targets as globs.

```console
$ xvc file list '*/*-0001.bin' '*/*-0002.bin'
FX        2002 [..]          7e807161 dir-0005/file-0002.bin
FX        2001 [..]          1953f05d dir-0005/file-0001.bin
FX        2002 [..]          7e807161 dir-0004/file-0002.bin
FX        2001 [..]          1953f05d dir-0004/file-0001.bin
SS        [..] 7e807161          dir-0003/file-0002.bin
SS        [..] 1953f05d          dir-0003/file-0001.bin
FH        [..] 7e807161 7e807161 dir-0002/file-0002.bin
FH        [..] 1953f05d 1953f05d dir-0002/file-0001.bin
FC        [..] 7e807161 7e807161 dir-0001/file-0002.bin
FC        [..] 1953f05d 1953f05d dir-0001/file-0001.bin
Total #: 10 Workspace Size:       [..] Cached Size:        4003


```

## Sorting

You may sort `xvc file list` output by name, by modification time and by file
size.

Use `--sort` option to specify the sort criteria.

```console
$ xvc file list --sort name-desc dir-0001/
FC        2005 [..] 447933dc 447933dc dir-0001/file-0005.bin
FC        2004 [..] 63535612 63535612 dir-0001/file-0004.bin
FC        2003 [..] d2432259 d2432259 dir-0001/file-0003.bin
FC        2002 [..] 7e807161 7e807161 dir-0001/file-0002.bin
FC        2001 [..] 1953f05d 1953f05d dir-0001/file-0001.bin
Total #: 5 Workspace Size:       10015 Cached Size:       10015


```

```console
$ xvc file list --sort name-asc dir-0001/
FC        2001 [..] 1953f05d 1953f05d dir-0001/file-0001.bin
FC        2002 [..] 7e807161 7e807161 dir-0001/file-0002.bin
FC        2003 [..] d2432259 d2432259 dir-0001/file-0003.bin
FC        2004 [..] 63535612 63535612 dir-0001/file-0004.bin
FC        2005 [..] 447933dc 447933dc dir-0001/file-0005.bin
Total #: 5 Workspace Size:       10015 Cached Size:       10015


```

## Column Format

You can specify the columns that the command prints.

For example, if you only want to see the file names, use `{{name}}` as the
format string.

The following command sorts all files with their sizes in the workspace, and
prints their size and name.

```console
$ xvc file list --format '{{asz}} {{name}}' --sort size-desc dir-0001/
       2005 dir-0001/file-0005.bin
       2004 dir-0001/file-0004.bin
       2003 dir-0001/file-0003.bin
       2002 dir-0001/file-0002.bin
       2001 dir-0001/file-0001.bin
Total #: 5 Workspace Size:       10015 Cached Size:       10015


```

If you want to compare the recorded (cached) hashes and actual hashes in the workspace, you can use `{{acd}} {{rcd}} {{name}}` format string.

```console
$ xvc file list --format '{{acd8}} {{rcd8}} {{name}}' --sort ts-asc dir-0001
1953f05d 1953f05d dir-0001/file-0001.bin
7e807161 7e807161 dir-0001/file-0002.bin
d2432259 d2432259 dir-0001/file-0003.bin
63535612 63535612 dir-0001/file-0004.bin
447933dc 447933dc dir-0001/file-0005.bin
Total #: 5 Workspace Size:       10015 Cached Size:       10015


```

```admonish info
If `{{acd8}}` or `{{acd64}}` is not present in the format string, Xvc doesn't calculate these hashes. If you have large number of files where the default format (that includes actual content hashes) runs slowly, you may customize it to not to include these columns.
```

If you want to get a quick glimpse of what needs to carried in, or rechecked,
you can use cache status `{{cst}}` column.

```console
$ xvc-test-helper generate-random-file --size 100 dir-0001/a-new-file.bin

$ xvc file list --format '{{cst}} {{name}}' dir-0001/
= dir-0001/file-0005.bin
= dir-0001/file-0004.bin
= dir-0001/file-0003.bin
= dir-0001/file-0002.bin
= dir-0001/file-0001.bin
X dir-0001/a-new-file.bin
Total #: 6 Workspace Size:       10115 Cached Size:       10015


```

The cache status column shows `=` for unchanged files in the cache, `X` for
untracked files, `>` for files that there is newer version in the cache, and `<`
for files that there is a newer version in the workspace. The comparison is done
between recorded timestamp and actual timestamp with an accuracy of 1 second.
