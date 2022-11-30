# Xvc Cache

The cache is where Xvc copies the files it tracks. 

It's located under `.xvc` directory. 

Instead of the file tree that's normally used to _address_ files, it uses _content digest_ of files to organize them. 

In a standard file hierarchy, we have files in paths like `/home/iesahin/Photos/my-photo.png`. 
Xvc doesn't use such a tree in its cache. 
It uses paths like `.xvc/b3/a12/b45/d789a...f54/0.png` to refer to files. 

Producing the cache path from its content leads cache paths change when the files are updated. 
For example, if you save another photo on top of `my-photo.png`, the first version will be lost. 
However, as these two versions produce different digests, they can be stored in different locations in cache. 

There are 4 parts of this cache path. 

`.xvc` part is the standard directory `xvc init` command creates. It resides in the root folder of your project.

`b3/` denotes the [digest type] of the content digest. 
Xvc supports more than one algorithm to calculate content digests. 
[HashAlgorithm][https://docs.rs/xvc-core/0.4.0/xvc_core/types/hashalgorithm/enum.HashAlgorithm.html] enum shows which algorithms are supported. 
Each of these algorithms has a 2-letter prefix. 
- `b3` :obs_right_arrow_with_tail: BLAKE3
- b2 :obs_right_arrow_with_tail:BLAKE2s
- s3 :obs_right_arrow_with_tail: SHA2-256
- s2 :obs_right_arrow_with_tail: SHA3-256

Note that, all these digest algorithms produce 256bits/32 bytes of digests. 
This is converted to 64 hexadecimal digits. 
In order to keep the total path length shorter, currently Xvc requires digests to be 32 bytes in length. 

The third part in cache path is this 64 hexadecimal digits in the form `a12/b45/d789...f54/`. 
64 digits are split into directories to keep the number of directories under one directory lower. 
Had Xvc put all cache elements in a single directory, it could lead to degraded performance in some file systems. 
With this arrangement, `b3/` can contain at most 4096 directories, that contain 4096 directories each. 
With usual distribution and good hash algorithms, there won't be more than 4000 elements per directory until 68 billion files in the cache. (4096³)

The fourth part is the `0.png` part, that's the file itself with the same extension but with `0` as the basename.
Xvc uses digest as a directory instead of file name.
There may be times when the file in the cache should be used manually, on remote storages for example. 
The extension is kept for this reason, to make sure that the OS recognizes the file type correctly. 

The rename to `0` means, that this is the whole file. 
In the future, when Xvc will support splitting large files to transfer to remotes, all parts of the file will be put into this directory. 

Storages also use the same cache structure, with an added `GUID` part to use a single storage for multiple projects. 




