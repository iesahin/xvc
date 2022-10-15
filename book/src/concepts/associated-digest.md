# Associated Digest

There may be multiple digests associated with an entity like path, directory or dependency.
An associated digest is all digests associated with an entity. 

## Metadata Digest

Files and directories have metadata.
Metadata shows information about creation, modification, access time of the file, or the size of it. 
Metadata is OS dependent in most cases.
Xvc abstracts file and directory metadata with `XvcMetadata` struct. 
Metadata digest represents this abstraction in 32-bytes to compare changes in files and directories. 

## Content Digest

The content digest of a file is calculated by the data it contains.
It calculates 32-bytes from the content. 
When content changes, this calculation result also change.


## Collection Digest

Some entities in Xvc are composed of multiple elements.
Examples are directories (composed of files), file lines, regex filter results, SQL query results etc. 
Instead trying to compare all elements, Xvc creates a 32-byte digest of the collection with the same conditions.
For example, when a new file is added to a directory, its collection digest also changes.
This is used keep track of changed directories easier than moving members around.
