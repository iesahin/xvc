# xvc file track

## Purpose

`xvc file track` is used to register any kind of file to Xvc for tracking versions.

## Synopsis 

```console
$ xvc file track --help
Add file and directories to Xvc

Usage: xvc file track [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to track

Options:
      --cache-type <CACHE_TYPE>
          How to track the file contents in cache: One of copy, symlink, hardlink, reflink.
          
          Note: Reflink uses copy if the underlying file system doesn't support it.

      --no-commit
          Do not copy/link added files to the file cache

      --text-or-binary <TEXT_OR_BINARY>
          Calculate digests as text or binary file without checking contents, or by automatically. (Default: auto)

      --force
          Add targets even if they are already tracked

      --no-parallel
          Don't use parallelism

  -h, --help
          Print help information (use `-h` for a summary)

```

## Examples

By default, the command runs similar to `git add` and `git commit`. 

```console,ignore
$ xvc file track my-large-image.jpeg
```

You can track directories with the same command. 

```console,ignore
$ xvc file track my-large-directory/
```

You can specify more than one target in a single command. 

```console,ignore
$ xvc file track my-large-image.jpeg my-large-directory
```


## Caveats

- This command doesn't discriminate symbolic links or hardlinks. 
Links are followed and any broken links may cause errors. 

- Under the hood, Xvc tracks only the files, not directories. 
Directories are considered as path collections.
It doesn't matter if you track a directory or files in it separately.

## Technical Details

- Detecting changes in files and directories employ different kinds of [associated digests](/concepts/associated-digest.md).
If a file has different metadata digest, its content digest is calculated.
If file's content digest has changed, the file is considered changed.
A directory that contains different set of files, or files with changed content is considered changed.



