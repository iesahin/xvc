# xvc file bring

## Synopsis

```console
$ xvc file bring --help
Bring files from external storages

Usage: xvc file bring [OPTIONS] --storage <STORAGE> [TARGETS]...

Arguments:
  [TARGETS]...
          Targets to bring from the storage

Options:
  -s, --storage <STORAGE>
          Storage name or guid to send the files

      --force
          Force even if the files are already present in the workspace

      --no-recheck
          Don't recheck (checkout) after bringing the file to cache.
          
          This makes the command similar to `git fetch` in Git. It just updates the cache, and doesn't copy/link the file to workspace.

      --recheck-as <RECHECK_AS>
          Recheck (checkout) the file in one of the four alternative ways. (See `xvc file recheck`) and [RecheckMethod]

  -h, --help
          Print help (see a summary with '-h')

```
