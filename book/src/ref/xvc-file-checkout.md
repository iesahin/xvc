# xvc file checkout

## Synopsis

```console
$ xvc file checkout --help
Get file from cache

Usage: xvc file checkout [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to add

Options:
      --cache-type <CACHE_TYPE>
          How to track the file contents in cache: One of copy, symlink, hardlink, reflink.
          
          Note: Reflink uses copy if the underlying file system doesn't support it.

      --no-parallel
          Don't use parallelism

      --force
          Force the checkout even if target has not cached or no changes happened

      --text-or-binary <TEXT_OR_BINARY>
          Checkout the files as text, binary (Default: auto)

  -h, --help
          Print help information (use `-h` for a summary)

```
