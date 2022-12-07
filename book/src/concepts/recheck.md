# recheck

Rechecking in Xvc corresponds to _checking out_ a file in cache to the repository. It might also be called _checkout_ or _carry out_. 

You may want to recheck a file for various purposes. It may be for data files that will mostly be read, or output files that will mostly be written. Correspondingly a recheck can be in various types:  `copy` makes a copy of the cached file in the repository, `symlink` creates a read-only symbolic link, `hardlink` makes a read-only hardlink, and in supporting file systems, `reflink` creates a symbolic link with copy-on-write semantics. 