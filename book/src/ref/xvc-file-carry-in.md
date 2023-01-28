# xvc file carry-in

Copies the file changes to cache.

## Synopsis

```console
$ xvc file carry-in --help
Carry (commit) changed files to cache

Usage: xvc file carry-in [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to add

Options:
      --text-or-binary <TEXT_OR_BINARY>
          Calculate digests as text or binary file without checking contents, or by automatically. (Default: auto)

      --force
          Carry in targets even their content digests are not changed.
          
          This removes the file in cache and re-adds it.

      --no-parallel
          Don't use parallelism

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

## Examples

Carry in command works with Xvc repositories.

```console
$ git init
...
$ xvc init

```

We first track a file.

```console
$ xvc file track data.txt

$ xvc file list data.txt
FC          19 [..] c85f3e81 c85f3e81 data.txt
Total #: 1 Workspace Size:          19 Cached Size:          19


```

We update the file with a command.

```console
$ perl -i -pe 's/a/ee/g' data.txt

$ cat data.txt
Oh, deetee, my, deetee

$ xvc file list data.txt
FC          23 [..] c85f3e81 e37c686a data.txt
Total #: 1 Workspace Size:          23 Cached Size:          19


```

Note that the size of the file has increased, as we replace each `a` with an `ee`.

```console
$ xvc file carry-in data.txt

$ xvc file list data.txt
FC          23 [..] e37c686a e37c686a data.txt
Total #: 1 Workspace Size:          23 Cached Size:          19


```
