# File Management


## Synopsis

```console
$ xvc file --help
File and directory management commands

Usage: xvc file [OPTIONS] <COMMAND>

Commands:
  track     Add file and directories to Xvc [aliases: t]
  hash      Get digest hash of files with the supported algorithms [aliases: h]
  recheck   Get files from cache by copy or *link [aliases: checkout, r]
  carry-in  Carry in changed files to cache [aliases: commit, c]
  copy      Copy from source to another location in the workspace [aliases: C]
  move      Move files to another location in the workspace [aliases: M]
  list      List tracked and untracked elements in the workspace [aliases: l]
  send      Send files to external storages [aliases: s, upload, push]
  bring     Bring files from external storages [aliases: b, download, pull]
  remove    Remove files from Xvc cache and storages [aliases: R]
  untrack   Untrack (delete) files from Xvc and storages [aliases: U]
  share     Share a file from (S3 compatible) storage for a limited time [aliases: S]
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...         Verbosity level. Use multiple times to increase command output detail
      --quiet              Suppress error messages
  -C <WORKDIR>             Set the working directory to run the command as if it's in that directory [default: .]
  -c, --config <CONFIG>    Configuration options set from the command line in the form section.key=value
      --no-system-config   Ignore system config file
      --no-user-config     Ignore user config file
      --no-project-config  Ignore project config (.xvc/config)
      --no-local-config    Ignore local config (.xvc/config.local)
      --no-env-config      Ignore configuration options from the environment
  -h, --help               Print help
  -V, --version            Print version

```


## Subcommands


- [`track`](./xvc-file-track.md): Track (add) files with Xvc
- [`recheck`](./xvc-file-recheck.md): Copy/link files in the cache to the
  workspace (checkout)
- [`carry-in`](./xvc-file-carry-in.md): Carry-in (commit) changed files to cache
- [`copy`](./xvc-file-copy.md): Copy files to another location in the workspace
- [`move`](./xvc-file-move.md): Move files to another location in the workspace
- [`list`](./xvc-file-list.md): List tracked files
- [`send`](./xvc-file-send.md): Send (push
- ) files to storage
- [`bring`](./xvc-file-bring.md): Bring (pull) files from storage
- [`hash`](./xvc-file-hash.md): Calculate hashes with supported algorithms similar to sha256sum, blake2sum, etc.
- [`remove`](./xvc-file-remove.md): Remove files from Xvc cache or storages
- [`untrack`](./xvc-file-untrack.md): Untrack (delete) files from Xvc
