# Comparisons

In order to avoid unnecessary work, we need to find differences across versions.
What has changed between the previous version and this version of type `T`?

Xvc is built bottom up, with vertical, long functions that do one thing. 
For example, `xvc file track` is written separately from `xvc file checkout`, and the commonalities are arised _after_ these implementations. 
We consider _implementation is a form of planning_ philosophy. 
We didn't start from traits and try to fit everything to these. 
Instead we began from concrete enums and structs, saw some of these share common functionality and thought to group this common functionality as a trait after implementing several concrete functions.

We saw that the `diff` pattern across all functionality.
In `xvc pipeline`, dependencies need to detect changes to decide whether to invalidate them.
In `xvc file`, files and directories need to detect changes to decide whether they should be commit to cache. 

It's easy to make comparison/subtraction when the data types are numeric. 
For a signed integer, you can get a single numeric value as _diff_ with `diff = a - b`.
For complex data structures, representing the change is usually not straightforward. 

We keep track of everything in the repository in [stores](./arch/ecs.md).
These serialize a type `T` to a file, and get it back when needed.
Diff pattern works with these types. 
Sometimes, there happens to be no record of something we have in the repository. 
Sometimes, we only have only the record, and not the actual thing on disk. 
The diff should also handle this. 

Instead of trying to come up some wizardy, at the end, we decided to represent this with five conditions. 

- `Identical`: When two things of the same type `T` are equal.
Nothing has changed between the actual version and its record.

- `RecordMissing { actual: T }`: If we have something on disk, but can't find the respective record. 
For example, a new file is added to the disk but `xvc file track` detects it for the first time. 
The action is usually creating a record from `actual: T`

- `ActualMissing { record: T }`: We found a record in the store, but the corresponding file is not there.
If a tracked file is deleted, but the record still keeps it. 

- `Difference { record: T, actual: T }`: There is a record, but the actual data isn't identical.
When a tracked file is changed, and its content hash now returns another digest, this can be reflected with `Difference`. 

- `Skipped`: When the comparison seems unnecessary. 
For example, if we know a file hasn't changed by checking its metadata. 
In this case, we don't calculate its content digest and set it to `Skipped`. 

These five conditions are represented in `DeltaField` type. 

As an entity may have more than one component, a comparison may require multiple `DeltaField`s. 
For example, we may want to compare an `XvcPath`, to see whether it has changed. 
This requires comparing its `XvcMetadata`, its `ContentDigest` if it's a file, its `CollectionDigest` if it's a directory, etc. 
There are various such `Delta` types. 

## Comparing files

Files are compared with several aspects. 
We assume their relative path (`XvcPath`) doesn't change.
Other features like `XvcMetadata`, `ContentDigest`, etc. could be modified and are tracked. 

The following struct is used to compare two files:

```rust
pub struct FileDelta {
    pub delta_md: DeltaField<XvcMetadata>,
    pub delta_content_digest: DeltaField<ContentDigest>,
    pub delta_metadata_digest: DeltaField<MetadataDigest>,
    pub delta_cache_type: DeltaField<CacheType>,
    pub delta_text_or_binary: DeltaField<DataTextOrBinary>,
}

```

When the user first start tracking a file, all delta fields are of the value `RecordMissing`. 
It contains the actual value on disk. 
These are recorded to stores. 

When they edit the file, its `delta_md` changes. 
Xvc checks whether the `delta_content_digest` has also changed. 

When the user wants to check out the file in a different `cache_type`, for example changing the workspace version from _Copy_ to _Hardlink_, `delta_cache_type` field contains a `Difference` value. 

## Comparing directories

A directory is considered as a collection of paths. 

Its comparison is based on the (non-ignored) paths it contains. 

```rust
pub struct DirectoryDelta {
    pub delta_xvc_metadata: DeltaField<XvcMetadata>,
    pub delta_collection_digest: DeltaField<CollectionDigest>,
    pub delta_metadata_digest: DeltaField<MetadataDigest>,
    pub delta_content_digest: DeltaField<ContentDigest>,
}
```

We record the size and modification time of the directories as well. 
When these change, they are reflected with `delta_xvc_metadata` file. 

The other fields are generated from the paths the directory contains. 

