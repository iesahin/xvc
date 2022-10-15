use super::event::Event;
use super::event::EventLog;
use super::*;
use crate::error::{Error, Result};
use crate::Storable;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;

use std::ops::Deref;
use std::ops::Index;

/// A database table like store for type `T`
///
/// It's used as the general purpose persistence data structure for all components.
/// It contains an `XvcEntity -> T` map, and an index `T -> Vec<XvcEntity>` for these entries for
/// quick reverse lookup.
/// It loads these data from [EventLog] collections.
/// It has basic functionality to insert, delete, filter and iterate over the elements.
#[derive(Debug, Clone, Hash)]
pub struct XvcStore<T>
where
    T: Storable,
{
    map: BTreeMap<XvcEntity, T>,
    entity_index: BTreeMap<T, Vec<XvcEntity>>,
    previous: EventLog<T>,
    current: EventLog<T>,
}

impl<T> Deref for XvcStore<T>
where
    T: Storable,
{
    type Target = BTreeMap<XvcEntity, T>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<T> Index<&XvcEntity> for XvcStore<T>
where
    T: Storable,
{
    type Output = T;

    fn index(&self, entity: &XvcEntity) -> &Self::Output {
        self.map.index(entity)
    }
}

impl<T> Default for XvcStore<T>
where
    T: Storable,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> XvcStore<T>
where
    T: Storable,
{
    /// Creates an empty store with empty maps and event logs.
    pub fn new() -> Self {
        Self {
            map: BTreeMap::<XvcEntity, T>::new(),
            entity_index: BTreeMap::new(),
            previous: EventLog::<T>::new(),
            current: EventLog::<T>::new(),
        }
    }

    /// Inserts an entity into the current event log, the map and the index.
    pub fn insert(&mut self, entity: XvcEntity, value: T) -> Option<T> {
        self.current.add(entity, value.clone());

        match self.entity_index.get_mut(&value) {
            Some(v) => {
                v.push(entity);
            }
            None => {
                self.entity_index.insert(value.clone(), vec![entity]);
            }
        }

        self.map.insert(entity, value)
    }

    /// Updates the data associated with an entity.
    ///
    /// This is equivalent to [remove] and [insert], and adds two events to the event log.
    /// Returns the previous value if there is one.
    pub fn update(&mut self, entity: XvcEntity, value: T) -> Option<T> {
        if self.map.contains_key(&entity) {
            self.remove(entity);
        }
        self.insert(entity, value)
    }

    /// Removes the data associated with entity.
    ///
    /// It returns the value if found, otherwise returns `None`.
    pub fn remove(&mut self, entity: XvcEntity) -> Option<T> {
        if let Some(value) = self.map.remove(&entity) {
            if let Some(vec_e) = self.entity_index.get_mut(&value) {
                self.current.remove(entity);
                vec_e.retain(|e| *e != entity);
                return Some(value);
            }
        }
        Option::<T>::None
    }

    /// Adds the `other` store to this by iterating over elements.
    ///
    /// It uses [Self::insert] for inserting all elements.
    /// This means all elements will have distinct [Event::Add] values.
    pub fn append(&mut self, other: &Self) -> Result<()> {
        other.iter().for_each(|(e, v)| {
            self.insert(*e, v.clone());
        });
        Ok(())
    }

    fn build_map(events: &EventLog<T>) -> BTreeMap<XvcEntity, T> {
        let mut map = BTreeMap::<XvcEntity, T>::new();

        for event in events.iter() {
            match event {
                Event::Add { entity, value } => map.insert(*entity, value.clone()),
                Event::Remove { entity } => map.remove(&entity),
            };
        }

        map
    }

    fn build_index(map: &BTreeMap<XvcEntity, T>) -> BTreeMap<T, Vec<XvcEntity>> {
        let mut entity_index = BTreeMap::<T, Vec<XvcEntity>>::new();

        map.iter()
            .for_each(|(entity, value)| match entity_index.get_mut(&value) {
                Some(v) => {
                    v.push(*entity);
                }
                None => {
                    entity_index.insert(value.clone(), vec![*entity]);
                }
            });

        entity_index
    }

    /// Loads the timestamp named [EventLog] files from `dir` and replays them to build maps.
    pub fn from_dir(dir: &Path) -> Result<Self> {
        let previous = EventLog::<T>::from_dir(dir)?;
        let map = Self::build_map(&previous);
        let entity_index = Self::build_index(&map);

        Ok(Self {
            map,
            entity_index,
            previous,
            current: EventLog::new(),
        })
    }

    /// Saves the current [EventLog] to the directory.
    /// This is enough to reload it to the saved state.
    pub fn to_dir(&self, dir: &Path) -> Result<()> {
        self.current.to_dir(dir)
    }

    // fn from_file(path: &Path) -> Result<Self> {
    //     let previous = EventLog::<T>::from_file(path)?;
    //     let map = Self::build_map(&previous);
    //     let entity_index = Self::build_index(&map);
    //
    //     Ok(Self {
    //         map,
    //         entity_index,
    //         previous,
    //         current: EventLog::new(),
    //     })
    // }

    /// Return the number of elements
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// A subset of this maps identified by `XvcEntity` elements of the iterator.
    ///
    /// This can be used to split the map arbitrarily.
    pub fn subset<I>(&self, keys: I) -> Result<Self>
    where
        I: Iterator<Item = XvcEntity>,
    {
        let mut store = XvcStore::<T>::new();
        for e in keys {
            if let Some(v) = self.map.get(&e) {
                store.insert(e, v.clone());
            } else {
                Error::CannotFindKeyInStore { key: e.0 }.warn();
            }
        }
        Ok(store)
    }

    /// Creates a new map by calling the `predicate` with each value.
    ///
    /// `predicate` must be a function or closure that returns `bool`.
    pub fn filter<F>(&self, predicate: F) -> Self
    where
        F: Fn(&XvcEntity, &T) -> bool,
    {
        let mut s = Self::new();
        for (e, v) in self.map.iter() {
            if predicate(e, v) {
                s.insert(*e, v.clone());
            }
        }

        s
    }

    /// Returns the first element of the map
    ///
    /// This is useful when there is only one element after [Self::filter]
    pub fn first(&self) -> Option<(&XvcEntity, &T)> {
        self.map.iter().next()
    }

    /// Returns the entities for a `value`.
    ///
    /// There may be more than one entity for a given value, hence it returns a `Vec`.
    /// It uses internal reverse index for fast lookup.
    pub fn entities_for(&self, value: &T) -> Option<&Vec<XvcEntity>>
    where
        T: PartialEq,
    {
        self.entity_index.get(value)
    }

    /// Returns the first entity matched with [Self::entities_for]
    pub fn entity_by_value(&self, value: &T) -> Option<XvcEntity> {
        match self.entities_for(value) {
            Some(vec_e) => vec_e.get(0).copied(),
            None => None,
        }
    }

    /// Return the previous (immutable) [EventLog].
    ///
    /// The event log contains the [Event] records before the last load.
    pub fn previous_events(&self) -> &EventLog<T> {
        &self.previous
    }

    /// Return the current (mutable) [EventLog].
    ///
    /// The event log contains the [Event] records since the last load.
    pub fn current_events(&self) -> &EventLog<T> {
        &self.current
    }

    /// Performs a join with [XvcEntity] keys.
    ///
    /// The returned store contains `(T, Option<U>)` values that correspond to the identical
    /// `XvcEntity` values.
    ///
    /// ## Example
    ///
    /// If this store has
    ///
    /// `{10: "John Doe", 12: "George Mason", 19: "Ali Canfield"}`,
    /// and the `other` store contains
    /// `{10: "Carpenter", 17: "Developer", 19: "Artist" }`
    ///
    /// `join` will return
    ///
    /// `{10: ("John Doe", Some("Carpenter")), 12: ("George Mason", None), 19: ("Ali Canfield",
    /// Some("Artist")}`
    ///
    /// In SQL terms, this is a LEFT JOIN.
    ///
    /// Note that, it may be more convenient to keep this relationship in a [crate::R11Store]
    /// relative to your use case.
    pub fn join<U>(&self, other: XvcStore<U>) -> XvcStore<(T, Option<U>)>
    where
        U: Storable,
    {
        let mut joined = XvcStore::<(T, Option<U>)>::new();
        for (entity, value) in self.map.iter() {
            joined.insert(*entity, (value.clone(), other.get(entity).cloned()));
        }

        joined
    }

    /// Returns an inverted map to get entity values quickly
    ///
    /// This uses entity_index with a caveat:
    /// - Each value of T must be unique.
    ///
    /// Otherwise this panics
    pub fn index_map(&self) -> Result<BTreeMap<T, XvcEntity>> {
        let mut map = BTreeMap::new();

        for (val, vec_e) in &self.entity_index {
            map.insert(val.clone(), vec_e[0]);
        }

        Ok(map)
    }
}

impl<T> XvcStore<T>
where
    T: Storable,
{
    fn store_path(store_root: &Path) -> PathBuf {
        store_root.join(format!("{}-store", <T as Storable>::type_description()))
    }

    /// Loads a store from the directory built from `store_root` and the type name.
    pub fn load_store(store_root: &Path) -> Result<Self> {
        let dir = Self::store_path(store_root);
        Self::from_dir(&dir)
    }

    /// Records the given `store` to the directory built from `store_root` and type description of `T`.
    pub fn save_store(store: &XvcStore<T>, store_root: &Path) -> Result<()> {
        let dir = Self::store_path(store_root);
        store.to_dir(&dir)
    }

    /// Saves the current store using [save_store] to a directory built from `store_root` and type
    /// description of `T`.
    pub fn save(&self, store_root: &Path) -> Result<()> {
        Self::save_store(self, store_root)
    }
}

#[cfg(test)]
mod test {
    use tempdir::TempDir;

    use super::*;

    #[test]
    fn new() -> Result<()> {
        let mut store = XvcStore::<String>::new();
        store.insert(0.into(), "0".into());
        store.insert(1.into(), "1".into());
        assert_eq!(store.len(), 2);

        assert_eq!(*store.get(&XvcEntity(0)).unwrap(), String::from("0"));
        assert_eq!(*store.get(&XvcEntity(1)).unwrap(), String::from("1"));
        Ok(())
    }

    #[test]
    fn serde() -> Result<()> {
        let td = TempDir::new("bstore-test")?;
        let dir = td.path();

        let mut store = XvcStore::<String>::new();

        store.insert(0.into(), "0".into());
        store.insert(1.into(), "1".into());
        store.insert(2.into(), "2".into());

        store.to_dir(&dir)?;

        let reincarnation = XvcStore::<String>::from_dir(&dir)?;

        assert!(store.len() == reincarnation.len());
        assert!(store.current_events().len() == reincarnation.previous_events().len());

        assert!(reincarnation.current_events().is_empty());

        let n_files_before = jwalk::WalkDir::new(&dir).into_iter().count();
        reincarnation.to_dir(&dir)?;
        let n_files_after = jwalk::WalkDir::new(&dir).into_iter().count();

        assert!(
            n_files_before == n_files_after,
            "before: {n_files_before} after: {n_files_after}"
        );

        Ok(())
    }
}
