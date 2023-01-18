# The Architecture of Xvc Entity Component System

Xvc uses an entity component system (ECS) in its core.
[ECS architecture] is popular among game development, but didn't find popularity in other areas.
It's an alternative to Object-Oriented Programming.

[ECS architecture]:  https://en.wikipedia.org/wiki/Entity_component_system

There are a few basic notions of ECS architecture.
Although it may differ in other frameworks, Xvc assumes the following:

- An entity is a neutral way of tracking components and their relationships.
It doesn't contain any semantics other than _being an entity._
An _entity_ in Xvc is an atomic integer tuple. (`XvcEntity`)

- A component is a bundle of associated data about these entities.
All semantics of entities are described through components.
Xvc uses components to keep track of different aspects of file system objects, dependencies, remotes, etc.

- A system is where the components are created and modified.
Xvc considers all modules that interact with components as separate systems.

Suppose you're want to track a new file in Xvc.
Xvc creates a new entity for this file.
Associates the path (`XvcPath`) with this entity.
Checks the file metadata, creates an instance of `XvcMetadata`, and associates it with this entity.
If this object is commit to Xvc cache, an `XvcDigest` struct is associated with the entity.

The difference from OOP is that there is no _basic_ or _main_ object.
If you want to work only with digests and want to find the workspace paths associated with them, you can write a function (system) that starts from `XvcDigest` records and collect the associated paths.
If you want to get only the files larger than a certain size, you can work with `XvcMetadata`, filter them and get the paths later.
In contrast, in an OOP setting, these kind of data is associated with paths and when you want to do such operations, you need to load paths and all their associations first.
OOP way of doing things is usually against the principle of locality.

The whole idea is to be flexible for further changes.
As of now, Xvc doesn't have different notion of _data_ and _models._
It doesn't have different functionality for files that are models or data.
In the future, however, when this will be added, an `XvcModel` component will be created and associated with the same entity of an `XvcPath`.
It will allow to work with some paths as model files but it doesn't require _paths_ to be known beforehand.
There may be other metadata, like _features_ or _version_ associated with models that are more important.
There may be some models without a file system path, maybe living only in memory or in the cloud.
Those kind of models might be checked by verifying whether the model has a corresponding `XvcPath` component or not.

In contrast, OOP would define this either by _inheritance_ (a model is a path) or _containment_ (a model has a path).
When you select any of these, it becomes a _relationship_ that must be maintained indefinitely.
When you only have an integer that identifies these components, it's much easier to describe _models without a path_ later.
There is no predefined relationship between paths and models.

The architecture is approximately similar to database modeling.
Components are in-memory tables, albeit they are small and mostly contain a few fields.
Entities are sequential primary keys.
Systems are _insert_, _query_ and _update_ mechanisms.

## Stores

An `XvcStore` in its basic definition is a map structure between `XvcEntity` and a component type `T`
It has facilities for persistence, iteration, search and filtering.
It can be considered a _system_ in the usual ECS sense.

### Loading and Saving Stores

As our goal is to track data files with Git, stores save and load binary files' metadata to text files.
Instead of storing the binary data itself in Git, Xvc stores information about these files to track whether they are changed.  
By default, these metadata are persisted to JSON.
Component types must be serializable because of this.
They are meant to be stored to disk in JSON format.
Nevertheless, as they are almost always composed of basic types [serde] supports, this doesn't pose a difficulty in usage.
The JSON files are then commit to Git.

Note that, there are usually multiple branches in Git repositories.
Also multiple users may work on the same branch.

When these text files are reused by the stores, they are modified and this may lead to merge conflicts.
We don't want our users to deal with merge conflicts with entities and
components in text files.
This also makes it possible to use binary formats like MessagePack in the
future.

Suppose user A made a change in `XvcStore<XvcPath>` by adding a few files.
Another user B made another change to the project, by adding another set of files in another copy of the project.
This will lead to merge conflicts:

- `XvcEntity` counter will have different values in A and B's repositories.
- `XvcStore<XvcPath>` will have different records in A and B's repositories.

Instead of saving and loading to monolithical files, `XvcStore` saves and loads _event logs._
There are two kind of events in a store:

- `Add(XvcEntity, T)`: Adds an element `T` to a store.
- `Remove(XvcEntity)`: Removes the element with entity id.

These events are saved into files.
When the store is loaded, all files after the last full snapshot are loaded and replayed.

When you add an item to a store, it saves the `Add` event to a log.
These events are then put into a vector.
A `BTreeMap` is also created by this vector.

When an item is deleted, a `Remove` event is added to the event vector.
While loading, stores removes the elements with `Remove` events from the `BTreeMap`.
So the final set of elements doesn't contain the removed item.

The second problem with multiple branches is duplicate entities in separate
branches. Xvc uses a _counter_ to generate unique entity ids.
When a store is loaded, it checks the last entity id in the event log and uses
it as the starting point for the counter. But using this counter as is causes
duplicate values in different branches. Xvc solves this by adding a random value
to these counter values.

Since v0.5, `XvcEntity` is a tuple of 64-bit integers. The first is loaded from
the disk and is an atomic counter. The second is a random value that is renewed
at every command invocation. Therefore we have a unique entity id for every run,
that's also sortable by the first value. Easy sorting with integers is sometimes
required for stable lists.

### Inverted Index

Stores also have a inverted index for quick lookup.
They store value of `T` as key and a list of entities that correspond to this key.
For example, when we have a path that we stored, it's a single operation to get the corresponding `XvcEntity` and after this, all recorded metadata about this path is available.

All search, iteration and filtering functionality is performed using these two internal maps.

In summary, a store has four components.

- An immutable log of previous events: `Vec<Event<T>>`
- A mutable log of current events: `Vec<Event<T>>`
- A mutable map of the current data: `BTreeMap<XvcEntity, T>`
- A mutable map of the entities from values: `BTreeMap<T, Vec<XvcEntity>>`

Note that, when two branches perform the same operation, the event logs will be
different, as the random part of `XvcEntity` is different. When two parties
branches merge, the inverted index may contain conflicting values. In this case,
a `fsck` command is used to merge the store files and merge conflicting entity
ids.

Insert, update and delete operations affect mutable log and maps.
Queries, iteration and such non-destructive operations are done with the maps.
When loading, all log files are merged in immutable log.
No standard operation touches the event logs.
All log modifications are done outside of the normal worflow.
When saving, only the mutable log is saved.
Note that only can only be added to the log, they are not removed.
(See `xvc fsck --merge-stores` for merging store files.)

### Relationship Stores

`XvcStore` keeps component-per-entity.
Each component is a flat structure that doesn't refer to other components.

Xvc also has _relation_ stores that represent relationships between entities, and components.
Similar to the database Entity-Relationship model, there are three kinds of the relationship store:

`R11Store<T, U>` keeps two sets of components associated with the same entity.
It represents a 1-1 relationship between `T` and `U`.
It contains two `XvcStore`s for each component type.
These two stores are indexed with the same `XvcEntity` values.
For example, an `R11Store<XvcPath, XvcMetadata>` keeps track of path metadata for the identical `XvcEntity` keys.

`R1NStore<T, U>` keeps parent-child relationships.
It represents a 1-N relationship between `T` and `U`.
On top of two `XvcStore`s, this one keeps track of relationships with a third `XvcStore<XvcEntity>`.
It lists which `U`'s are children of `T`s.
For example, a value of `XvcPipeline` can have multiple `XvcStep`s.
These are represented with `R1NStore<XvcPipeline, XvcStep>`.
This struct has `parent-to-child` and `child-to-parent` functions that can be used get children of a parent, or parent of child element.

The third type is `RMNStore<T, U>`.
This one keeps arbitrary number of relationships between `T` and `U`.
Any number of `T`s may correspond to any number of `U`s.
This type of store keeps the relationships in two `XvcStore<XvcEntity>`'s.
