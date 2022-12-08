# xvc file checkout

This is an alias of [`xvc file recheck`](/ref/xvc-file-recheck.md). 
Please see that page for examples.

## Synopsis

```console
$ xvc file checkout --help
Get files from cache by copy or *link

Usage: xvc file recheck [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to recheck

Options:
      --cache-type <CACHE_TYPE>
          How to track the file contents in cache: One of copy, symlink, hardlink, reflink.
          
          Note: Reflink uses copy if the underlying file system doesn't support it.

      --no-parallel
          Don't use parallelism

      --force
          Force even if target exists

      --text-or-binary <TEXT_OR_BINARY>
          Recheck files as text, binary (Default: auto)
          
          Text files may go OS specific line ending replacements.

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```
