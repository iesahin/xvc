# Xvc Cache

The cache is where Xvc copies the files it tracks.

It's located under the `.xvc` directory.

Instead of the file tree that's normally used to _address_ files, it uses the _content digest_ of files to organize them.

In a standard file hierarchy, we have files in paths like `/home/iesahin/Photos/my-photo.png`.
Xvc doesn't use such a tree in its cache.
It uses paths like `.xvc/b3/a12/b45/d789a...f54/0.png` to refer to files.

Producing the cache path from its content causes cache paths to change when the files are updated.
For example, in a standard file system, if you save another photo on top of `my-photo.png`, the first version will be
lost.
Xvc stores these two versions in different locations in the cache, so they are not lost.

There are 4 parts of this cache path.

`.xvc` part is the standard directory `xvc init` command creates. It resides in the root folder of your project.

`b3/` denotes the [digest type] of the content digest.
Xvc supports more than one algorithm to calculate content digests.
[HashAlgorithm][https://docs.rs/xvc-core/latest/xvc_core/types/hashalgorithm/enum.HashAlgorithm.html] enum shows which algorithms are supported.
Each of these algorithms has a 2-letter prefix.
- `b3`:  BLAKE3
- `b2`:  BLAKE2s
- `s3`:  SHA2-256
- `s2`:  SHA3-256

Note that, all these digest algorithms produce 256bits/32 bytes digests.
This digest is converted to 64 hexadecimal digits.
To keep the total path length shorter, Xvc requires digests to be 32 bytes in length.

The third part in the cache path is these 64 hexadecimal digits in the form `a12/b45/d789...f54/`.
64 digits are split into directories to keep the number of directories under one directory lower.
Had Xvc put all cache elements in a single directory, it could lead to degraded performance in some file systems.
With this arrangement, `b3/` can contain at most 4096 directories, that contain 4096 directories each.
With usual distribution and good hash algorithms, there won't be more than 4000 elements per directory until 64 billion
files are in the cache. (4000³)

The fourth part is the `0.png` part, that's the file itself with the same extension but with `0` as the basename.
Xvc uses digest as a directory instead of the file name.
There may be times when the file in the cache should be used manually, on cloud storage for example.
The extension is kept for this reason, to make sure that the OS recognizes the file type correctly.

The rename to `0` means, that this is the whole file.
In the future, when Xvc will support splitting large files to transfer to remotes, all parts of the file will be put into this directory.

Storages also use the same cache structure, with an added `GUID` part to use single storage for multiple projects.




