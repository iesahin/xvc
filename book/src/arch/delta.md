# Comparisons

To avoid unnecessary work, we need to find differences across versions.
What has changed between the previous version and this version of type `T`?

Xvc is built bottom up, with vertical, long functions that do one thing.
For example, `xvc file track` is written separately from `xvc file recheck`, and the commonalities have arisen _after_ these implementations.

I didn't start from traits and try to fit everything to a model.
Instead, we began from concrete enums and structs.
Then saw some of these share common functionality and thought to group this common functionality as a trait after
implementing and refactoring concrete functions.

I saw that the `diff` pattern across all comparison functions.
In `xvc pipeline`, dependencies need to detect changes to decide whether to invalidate them.
In `xvc file`, files and directories need to detect changes to decide whether they should be carried into the cache.

It's easy to make comparison/subtraction when the data types are numeric.
For a signed integer, you can get a single numeric value as _diff_ with `diff = a - b`.
For complex data structures, representing the change is not straightforward.

We keep track of everything in the repository in [stores](./arch/ecs.md).
These serialize a type `T` to a file, and get it back when needed.
Diff pattern works with these types.
Sometimes, there happens to be no record of something we have in the repository.
Sometimes, we only have only the record, and not the actual thing on disk.
The diff should also handle this.

Instead of trying to come up with wizardry, we decided to represent this with five conditions.

- `Identical`: When two things of the same type `T` are equal.
Nothing has changed between the actual version and its record.

- `RecordMissing { actual: T }`: If we have something on workspace, but can't find the respective record.
For example, a new file is added to the workspace, but `xvc file track` detects it for the first time.

- `ActualMissing { record: T }`: We found a record in the store, but the corresponding file in the workspace is not
  where it should be.
For example, a tracked file is deleted by the user, but the record is still there.

- `Difference { record: T, actual: T }`: There is a record, but the actual file in workspace isn't identical with it.
When a tracked file is changed, and its content now returns a different value, this can be reflected with `Difference`.

- `Skipped`: When the comparison seems unnecessary or irrelevant.
For example, if we know a file hasn't changed by checking its metadata.
In this case, we don't calculate its content digest and set it to `Skipped`.

These five conditions are represented in `Diff` type.

As an entity may have more than one component, a comparison may require multiple `Diff`s.
For example, we may want to compare an `XvcPath`, to see whether it has changed.
This requires comparing its `XvcMetadata`, its `ContentDigest` if it's a file, its `CollectionDigest` if it's a
directory, etc.

