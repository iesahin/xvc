# xvc file recheck

## Synopsis

```console
$ xvc file recheck --help
Check out file from cache by a copy or link

Usage: xvc file recheck [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Files/directories to recheck 

Options:
      --as <CACHE_TYPE>
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

This command is aliased to `xvc file checkout` in case you feel at home with Git nomenclature.

## Examples

Rechecking is analogous to [git checkout]. 
It copies or links a cached file to the workspace. 


Start by tracking a file. 

```console
$ git init
$ xvc init
$ xvc file track data.txt
$ ls -l
```

Once you added the file to the cache, you can delete the workspace copy.


```console
$ rm data.txt
$ ls -l
```

Then, recheck the file. By default, it makes a copy of the file.

```console
$ xvc file recheck data.txt
$ ls -l
```

Xvc doesn't recheck a path if it exists already.

```console
$ xvc file recheck data.txt --as symlink
? failure
```

You can force it to do so.

```console
$ xvc file recheck data.txt --as symlink --force
? success
```

Hardlinks look like the original file. 

```console
$ ls -l
$ rm data.txt
$ xvc file recheck data.txt --as hardlink
$ ls -l
```

Note that, hardlinks and symlinks are read only. 

Reflinks are supported by Xvc, but the underlying file system should also support it. 
Otherwise it uses `copy`. 

```console
$ ls -l
$ rm data.txt
$ xvc file recheck data.txt --as reflink
$ ls -l
```
