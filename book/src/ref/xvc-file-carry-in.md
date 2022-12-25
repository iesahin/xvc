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
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information

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

$ xvc file list -a data.txt
C=	[..] 	                  19	data.txt	c85f3e81


```

We update the file with a command. 

```console
$ perl -i -pe 's/a/ee/g' data.txt

$ cat data.txt
Oh, deetee, my, deetee

$ xvc file list -a data.txt
C<	[..] 	                  23	data.txt	e37c686a


```

Note that the size of the file has increased, as we replace each `a` with an `ee`. 


```console
$ xvc --debug file carry-in data.txt

$ xvc file list data.txt
C=	[..] 	                  23	data.txt


```
