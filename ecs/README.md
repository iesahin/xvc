# xvc-ecs: Entity-Component System for Xvc

[![Latest Version](https://img.shields.io/crates/v/xvc-ecs.svg)](https://crates.io/crates/xvc-ecs)
[![Docs](https://docs.rs/xvc-ecs/badge.svg)](https://docs.rs/xvc-ecs)

`xvc-ecs` provides the core storage mechanism for the Xvc project, implementing
an Entity-Component System (ECS). It allows associating arbitrary data
(components) with simple integer-based identifiers (entities).

This approach offers flexibility over traditional object-oriented designs,
allowing new components and relationships to be added easily as the software
evolves.

## Core Concepts

* **`XvcEntity`**: A unique identifier for an entity, represented as a `(u64,
u64)` tuple. Entities themselves don't hold data, they just serve as keys.

* **`Storable`**: A trait that components must implement to be stored in
persistent stores like `XvcStore`. It requires `Serialize`, `Deserialize`,
`Clone`, `Debug`, `Ord`, and `PartialEq`, and a `type_description` function
(often implemented via the `persist!` macro).

* **Components**: Plain Rust structs or types that implement `Storable` (if
persistence is needed).

* **Stores**: Data structures that manage the association between entities and
components, or relationships between entities.

* **`XvcEntityGenerator`**: A thread-safe generator used to create new, unique
`XvcEntity` values. It persists its state to ensure uniqueness across
application runs.

## Store Types

`xvc-ecs` provides several types of stores:

1.  **`XvcStore<T>`**: The primary persistent store for components of type `T`.
    * Maps `XvcEntity` to a component `T`.
    * Maintains a reverse index (`T` -> `Vec<XvcEntity>`) for quick lookups by
    value.
    * Uses an `EventLog` (`Add`/`Remove` events) for persistence. Changes are
    journaled and saved to timestamped JSON files.
    * Supports loading from/saving to directories.

2.  **`HStore<T>`**: An *ephemeral* (non-persistent) store based on `HashMap<XvcEntity, T>`.
    * Useful for temporary operations or when serialization is not required.
    * Provides fast lookups.
    * Offers various join operations (`left_join`, `full_join`, `join`).

3.  **`VStore<T>`**: A store based on `Vec<(XvcEntity, T)>`.
    * Allows multiple components of the same type to be associated with the *same* `XvcEntity` (though conversion to `XvcStore` enforces uniqueness later).
    * Also uses `EventLog` for persistence, similar to `XvcStore`.

## Relationship Stores

These stores manage relationships *between* entities/components:

1.  **`R11Store<T, U>`**: Manages a 1-to-1 relationship. Associates a component `T` and a component `U` with the *same* `XvcEntity`.
    * Internally uses two `XvcStore`s (`XvcStore<T>` and `XvcStore<U>`).
    * Allows looking up the related component given an entity.

2.  **`R1NStore<T, U>`**: Manages a 1-to-N (Parent-Child) relationship. One parent entity (associated with component `T`) can have multiple child entities (associated with component `U`).
    * Uses `XvcStore<T>` for parents, `XvcStore<U>` for children, and
    `XvcStore<ChildEntity<U, T>>` to store the child-to-parent links.
    * Allows finding all children of a parent or the parent of a child.

3.  **`RMNStore<T, U>`**: Manages an M-to-N relationship. Allows arbitrary
    connections between entities associated with components `T` and `U`.
    * Uses multiple underlying stores to track relationships in both directions.
    * Provided for completenes, not used or tested extensively.

## Persistence (`EventLog`)

`XvcStore` and `VStore` achieve persistence through an `EventLog`.

* Operations like adding or removing components are recorded as `Event::Add` or
`Event::Remove`.
* These events are collected in the `current` `EventLog`.
* When a store is saved (`to_dir`/`save`), the `current` `EventLog` is
serialized to a new JSON file in the store's directory. The filename is a
timestamp to ensure chronological order.
* When a store is loaded (`from_dir`/`load_store`), all JSON files in the
directory are read in sorted order, and the events are replayed to reconstruct
the store's state. The replayed events form the `previous` `EventLog`.

## Basic Usage

```rust
use xvc_ecs::{
    XvcEntity, XvcEntityGenerator, XvcStore, Storable,
    persist, init_generator, Result, Error
};
use serde::{Serialize, Deserialize};
use std::path::Path;
use tempdir::TempDir; // For example purposes

// 1. Define your component struct
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Position {
    x: i32,
    y: i32,
}

// 2. Make it Storable using the persist! macro
persist!(Position, "position");

// Or implement manually:
// impl Storable for Position {
//     fn type_description() -> String {
//         "position".to_string()
//     }
// }

fn main() -> Result<()> {
    // 3. Initialize the Entity Generator (usually once per application/repo)
    // Use load_generator(path) in subsequent runs
    let entity_gen = init_generator()?;

    // Create a temporary directory for the store
    let temp_dir = TempDir::new("ecs-example")?;
    let store_root = temp_dir.path();

    // 4. Create a store
    let mut position_store = XvcStore::<Position>::new();

    // 5. Generate entities and add components
    let entity1 = entity_gen.next_element();
    let pos1 = Position { x: 10, y: 20 };
    position_store.insert(entity1, pos1.clone());

    let entity2 = entity_gen.next_element();
    let pos2 = Position { x: -5, y: 15 };
    position_store.insert(entity2, pos2.clone());

    println!("Store size: {}", position_store.len()); // Output: Store size: 2

    // 6. Get components
    if let Some(retrieved_pos) = position_store.get(&entity1) {
        println!("Position for {:?}: {:?}", entity1, retrieved_pos);
        assert_eq!(*retrieved_pos, pos1);
    }

    // 7. Find entity by value
    if let Some(found_entity) = position_store.entity_by_value(&pos2) {
         println!("Entity for {:?}: {:?}", pos2, found_entity);
         assert_eq!(found_entity, entity2);
    }

    // 8. Save the store
    // The store will be saved to a subdirectory named "position-store"
    position_store.save(store_root)?;
    println!("Store saved to: {:?}", store_root.join("position-store"));

    // 9. Save the entity generator state (important!)
    // The generator state will be saved to a subdirectory named "entity-gen" (example)
    let gen_dir = store_root.join("entity-gen");
    entity_gen.save(&gen_dir)?;
     println!("Generator state saved to: {:?}", gen_dir);


    // 10. Load the store in a new scope/run
    let loaded_store = XvcStore::<Position>::load_store(store_root)?;
    println!("Loaded store size: {}", loaded_store.len()); // Output: Loaded store size: 2
    assert_eq!(loaded_store.len(), 2);
    assert_eq!(*loaded_store.get(&entity1).unwrap(), pos1);

    // Using R11Store (Example)
    use xvc_ecs::R11Store;
    persist!(String, "string"); // Make String Storable

    let mut r11_store = R11Store::<Position, String>::new();
    let entity3 = entity_gen.next_element();
    r11_store.insert(&entity3, Position { x: 1, y: 1 }, "Label A".to_string());

    if let Some((_, label)) = r11_store.left_to_right(&entity3) {
        println!("Label for {:?}: {}", entity3, label); // Output: Label for XvcEntity(..): Label A
    }


    Ok(())
}
```

