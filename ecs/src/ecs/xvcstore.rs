//! The basic store type used across Xvc.
//!
//! It's used to store and retrieve a [crate::Storable] type to text files.
use super::event::Event;
use super::event::EventLog;
use super::*;
use crate::error::{Error, Result};
use crate::{HStore, Storable};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;

use std::ops::Deref;
use std::ops::Index;
use std::sync::{Arc, RwLock};

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

/// A shared version of [XvcStore] that we use to pass around for thread safety.
pub type SharedXStore<T> = Arc<RwLock<XvcStore<T>>>;

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
        Self::from_event_logs(EventLog::<T>::new(), EventLog::<T>::new())
    }

    /// Creates a store from previous and current [EventLog].
    ///
    /// This is used when conversions between stores are required.
    /// When elements are inserted with [XvcStore::insert], they are added to `current` and
    /// serialized to disk over and over.
    /// See https://github.com/iesahin/xvc/issues/45
    ///
    pub fn from_event_logs(previous: EventLog<T>, current: EventLog<T>) -> Self {
        let map = Self::build_map(&previous, &current);
        let entity_index = Self::build_index(&map);
        Self {
            map,
            entity_index,
            previous,
            current,
        }
    }

    /// Returns all events associated with the entity
    pub fn all_event_log_for_entity(&self, entity: XvcEntity) -> Result<EventLog<T>> {
        let mut prev_events = Self::filter_event_log_by_entity(&self.previous, entity)?;
        let mut current_events = Self::filter_event_log_by_entity(&self.current, entity)?;
        prev_events.append(&mut current_events);
        Ok(EventLog::from_events(prev_events))
    }

    /// Returns (loaded) previous events for the entity
    ///
    /// Doesn't return events in the current invocation
    pub fn previous_event_log_for_entity(&self, entity: XvcEntity) -> Result<EventLog<T>> {
        Ok(EventLog::from_events(Self::filter_event_log_by_entity(
            &self.previous,
            entity,
        )?))
    }

    fn filter_event_log_by_entity(event_log: &EventLog<T>, xe: XvcEntity) -> Result<Vec<Event<T>>> {
        let events = event_log
            .iter()
            .filter_map(|e| match e {
                event @ Event::Add { entity, .. } => {
                    if *entity == xe {
                        Some(event.clone())
                    } else {
                        None
                    }
                }
                event @ Event::Remove { entity } => {
                    if *entity == xe {
                        Some(event.clone())
                    } else {
                        None
                    }
                }
            })
            .collect();
        Ok(events)
    }

    /// Inserts an entity into the current event log, the map and the index.
    ///
    /// Note that this shouldn't be used in store conversions (from [crate::VStore] or
    /// [crate::HStore]). This function adds events to `current` set, and these are serialized.
    /// See https://github.com/iesahin/xvc/issues/45
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

    fn build_map(previous: &EventLog<T>, current: &EventLog<T>) -> BTreeMap<XvcEntity, T> {
        let mut map = BTreeMap::<XvcEntity, T>::new();

        for event in previous.iter() {
            match event {
                Event::Add { entity, value } => map.insert(*entity, value.clone()),
                Event::Remove { entity } => map.remove(entity),
            };
        }

        for event in current.iter() {
            match event {
                Event::Add { entity, value } => map.insert(*entity, value.clone()),
                Event::Remove { entity } => map.remove(entity),
            };
        }

        map
    }

    fn build_index(map: &BTreeMap<XvcEntity, T>) -> BTreeMap<T, Vec<XvcEntity>> {
        let mut entity_index = BTreeMap::<T, Vec<XvcEntity>>::new();

        map.iter()
            .for_each(|(entity, value)| match entity_index.get_mut(value) {
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
        let current = EventLog::<T>::new();
        Ok(Self::from_event_logs(previous, current))
    }

    /// Saves the current [EventLog] to the directory.
    /// This is enough to reload it to the saved state.
    pub fn to_dir(&self, dir: &Path) -> Result<()> {
        self.current.to_dir(dir)
    }

    /// Return the number of elements
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if the map is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// A subset of this maps identified by `XvcEntity` elements of the iterator.
    ///
    /// This can be used to split the map arbitrarily.
    pub fn subset<I>(&self, keys: I) -> Result<HStore<T>>
    where
        I: Iterator<Item = XvcEntity>,
    {
        let mut store = HStore::<T>::new();
        for e in keys {
            if let Some(v) = self.map.get(&e) {
                store.map.insert(e, v.clone());
            } else {
                Error::CannotFindKeyInStore { key: e.to_string() }.warn();
            }
        }
        Ok(store)
    }

    /// Creates a new map by calling the `predicate` with each value.
    ///
    /// `predicate` must be a function or closure that returns `bool`.
    ///
    /// This returns [HStore] not to create a new event log.
    pub fn filter<F>(&self, predicate: F) -> HStore<T>
    where
        F: Fn(&XvcEntity, &T) -> bool,
    {
        let mut s = HashMap::new();
        for (e, v) in self.map.iter() {
            if predicate(e, v) {
                s.insert(*e, v.clone());
            }
        }

        HStore::from(s)
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
            Some(vec_e) => vec_e.first().copied(),
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
        store.insert((0, 123).into(), "0".into());
        store.insert((1, 123).into(), "1".into());
        assert_eq!(store.len(), 2);

        assert_eq!(*store.get(&XvcEntity(0, 123)).unwrap(), String::from("0"));
        assert_eq!(*store.get(&XvcEntity(1, 123)).unwrap(), String::from("1"));
        Ok(())
    }

    #[test]
    fn serde() -> Result<()> {
        let td = TempDir::new("bstore-test")?;
        let dir = td.path();

        let mut store = XvcStore::<String>::new();

        store.insert((0, 123).into(), "0".into());
        store.insert((1, 123).into(), "1".into());
        store.insert((2, 123).into(), "2".into());

        store.to_dir(dir)?;

        let reincarnation = XvcStore::<String>::from_dir(dir)?;

        assert!(store.len() == reincarnation.len());
        assert!(store.current_events().len() == reincarnation.previous_events().len());

        assert!(reincarnation.current_events().is_empty());

        let n_files_before = jwalk::WalkDir::new(dir).into_iter().count();
        reincarnation.to_dir(dir)?;
        let n_files_after = jwalk::WalkDir::new(dir).into_iter().count();

        assert!(
            n_files_before == n_files_after,
            "before: {n_files_before} after: {n_files_after}"
        );

        Ok(())
    }
}
