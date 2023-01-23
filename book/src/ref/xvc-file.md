# File Management


## Synopsis

```console
$ xvc file --help
File and directory management commands

Usage: xvc file [OPTIONS] <COMMAND>

Commands:
  track     Add file and directories to Xvc
  hash      Get digest hash of files with the supported algorithms
  recheck   Get files from cache by copy or *link
  carry-in  Carry (commit) changed files to cache
  copy      Copy from source to another location in the workspace
  list      List tracked and untracked elements in the workspace
  send      Send (push, upload) files to external storages
  bring     Bring (download, pull, fetch) files from external storages
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


- [`track`](./xvc-file-track.md): Begin tracking (add) files with XVC
- [`hash`](./xvc-file-hash.md): Calculate hash of given file
- [`recheck`](./xvc-file-recheck.md): Copy/link files in the cache to the
  workspace (checkout)
- [`carry-in`](./xvc-file-carry-in.md): Carry (commit) changed files to cache
- [`copy`](./xvc-file-copy.md): Copy files in the workspace to another location
- [`list`](./xvc-file-list.md): List files tracked with XVC
- [`send`](./xvc-file-send.md): Send (push
- ) files to remote
- [`bring`](./xvc-file-bring.md): Bring (pull) files from remote
