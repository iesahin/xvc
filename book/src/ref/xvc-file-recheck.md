# xvc file recheck

## Synopsis

```console
$ xvc file recheck --help
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


This command has an alias [`xvc file checkout`](/ref/xvc-file-checkout.md) if you feel more at home with Git terminology.

## Examples

Rechecking is analogous to [git checkout](https://git-scm.com/docs/git-checkout). 
It copies or links a cached file to the workspace. 


Start by tracking a file. 

```console
$ git init
...
$ xvc init

$ xvc file track data.txt

$ ls -l
total 8
-rw-rw-rw- [..] data.txt

```

Once you added the file to the cache, you can delete the workspace copy.


```console
$ rm data.txt
$ ls -l
total 0

```

Then, recheck the file. By default, it makes a copy of the file.

```console
$ xvc file recheck data.txt

$ ls -l
total 8
-rw-rw-rw- [..] data.txt

```

Xvc doesn't recheck a path if it exists already.

```console
$ xvc -v file recheck data.txt --as symlink
[WARN] data.txt already exists. Use --force to overwrite

$ ls -l data.txt
-rw-rw-rw- [..] data.txt

```

You can force it to do so.

```console
$ xvc -vv file recheck data.txt --as symlink --force
...
[INFO] data.txt already exists. Overwriting.
...
$ ls -l data.txt
lrwxr-xr-x [..] data.txt -> [CWD]/.xvc/b3/[..]/0.txt

```

Hardlinks look like the original file. 

```console
$ rm data.txt
$ xvc file recheck data.txt --as hardlink
$ ls -l
total 8
-r--r--r-- [..] data.txt

```

Note that, as files in the cache are kept read-only, hardlinks and symlinks are also read only. Files rechecked as copy are made read-write explicitly.

Reflinks are supported by Xvc, but the underlying file system should also support it. 
Otherwise it uses `copy`. 

```console
$ rm data.txt
$ xvc file recheck data.txt --as reflink
```

The above command will create a read only link in macOS APFS and a copy in ext4 or NTFS file systems. 
