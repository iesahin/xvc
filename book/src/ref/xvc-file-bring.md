# xvc file bring

## Synopsis

```console
$ xvc file bring --help
Bring (download, pull, fetch) files from storage.

You can configure a new storage with [`xvc storage new`][xvc_storage::new] and use it to download and upload tracked files.

Usage: xvc file bring [OPTIONS] --storage <STORAGE> [TARGETS]...

Arguments:
  [TARGETS]...
          Targets to bring from the storage

Options:
  -s, --storage <STORAGE>
          Storage name or guid to send the files

      --force
          Force even if the files are already present in the workspace

      --no-checkout
          Don't checkout after bringing the file to cache.
          
          This is similar to `fetch` command in Git. It just updates the cache, and doesn't bring the file to workspace.

      --checkout-as <CHECKOUT_AS>
          Checkout the file in one of the four alternative ways. (See `xvc file checkout`) and [CacheType][CacheType]

  -h, --help
          Print help information (use `-h` for a summary)

```
