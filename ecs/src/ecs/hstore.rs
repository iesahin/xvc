//! A component store for ephemeral operations based on [HashMap].
//! Unlike [XvcStore], it doesn't require `T` to be serializable.
//! It's supposed to be used operations that don't require final result to be recorded to disk.
use super::*;
use crate::error::{Error, Result};
use crate::{Storable, XvcStore};
use log::debug;
use rayon::iter::{FromParallelIterator, ParallelIterator};

use std::collections::hash_map::IterMut;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::convert::From;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{FromIterator, Iterator};
use std::sync::{Arc, RwLock};

use super::vstore::VStore;

use std::ops::{Deref, DerefMut};

/// This is a HashMap for more random access and less restrictions, no support for serialization
#[derive(Debug, Clone)]
pub struct HStore<T> {
    /// The wrapped map for the store
    pub map: HashMap<XvcEntity, T>,
}

/// A shared version of [HStore] for use in multithreaded environments.
pub type SharedHStore<T> = Arc<RwLock<HStore<T>>>;

impl<T> Deref for HStore<T> {
    type Target = HashMap<XvcEntity, T>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<T> DerefMut for HStore<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<T> From<HashMap<XvcEntity, T>> for HStore<T> {
    fn from(map: HashMap<XvcEntity, T>) -> Self {
        Self { map }
    }
}

impl<T> FromIterator<(XvcEntity, T)> for HStore<T> {
    fn from_iter<I: IntoIterator<Item = (XvcEntity, T)>>(iter: I) -> Self {
        Self {
            map: HashMap::<XvcEntity, T>::from_iter(iter),
        }
    }
}

impl<T> FromParallelIterator<(XvcEntity, T)> for HStore<T>
where
    T: Send,
{
    fn from_par_iter<I>(par_iter: I) -> Self
    where
        I: rayon::iter::IntoParallelIterator<Item = (XvcEntity, T)>,
    {
        let par_iter = par_iter.into_par_iter();
        let map: HashMap<XvcEntity, T> = par_iter.collect();
        Self { map }
    }
}

impl<T> HStore<T>
where
    T: Storable,
{
    /// Convert to [VStore]
    pub fn to_vstore(&self) -> Result<VStore<T>> {
        let mut store = VStore::new();
        for (k, v) in self.map.iter() {
            store.insert(*k, v.clone());
        }
        Ok(store)
    }

    /// Returns the inner map's iter_mut
    pub fn iter_mut(&mut self) -> IterMut<'_, XvcEntity, T> {
        self.map.iter_mut()
    }

    /// Return a mutable value for `entity`
    pub fn get_mut(&mut self, entity: &XvcEntity) -> Option<&mut T> {
        self.map.get_mut(entity)
    }

    /// This is used to create a store from actual values where the entity may
    /// or may not already be in the store.
    pub fn from_storable<I>(values: I, store: &XvcStore<T>, gen: &XvcEntityGenerator) -> HStore<T>
    where
        I: IntoIterator<Item = T>,
    {
        let mut hstore = HStore::<T>::new();
        for value in values {
            let key = match store.entity_by_value(&value) {
                Some(e) => e,
                None => gen.next_element(),
            };
            hstore.map.insert(key, value.clone());
        }
        hstore
    }
}

impl<T> Default for HStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Storable> From<&XvcStore<T>> for HStore<T> {
    fn from(store: &XvcStore<T>) -> Self {
        let map = HashMap::from_iter(store.iter().map(|(k, v)| (*k, v.clone())));
        Self { map }
    }
}

impl<T> HStore<T> {
    /// Create an empty HStore.
    ///
    /// Calls inner map's [HashMap::new].
    pub fn new() -> HStore<T> {
        HStore {
            map: HashMap::<XvcEntity, T>::new(),
        }
    }

    /// Creates an empty HStore.
    ///
    /// Creates an inner map with the given `capacity`.
    pub fn with_capacity(capacity: usize) -> HStore<T> {
        HStore {
            map: HashMap::<XvcEntity, T>::with_capacity(capacity),
        }
    }

    /// Creates values from `func` and gets new entities from `gen` to create new records.
    pub fn from_func<F>(gen: &XvcEntityGenerator, func: F) -> Result<HStore<T>>
    where
        F: Fn() -> Result<Option<T>>,
    {
        let mut hstore = HStore::<T>::new();
        loop {
            let value = match func() {
                Ok(Some(v)) => v,
                Ok(None) => break,
                Err(err) => return Err(err),
            };
            let key = gen.next_element();
            hstore.map.insert(key, value);
        }
        Ok(hstore)
    }

    /// Return the number of elements in this HStore
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Return true if there are no elements in this HStore
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Calls the inner map's insert
    pub fn insert(&mut self, entity: XvcEntity, value: T) -> Option<T> {
        self.map.insert(entity, value)
    }

    /// Returns a [HashSet] of values.
    pub fn to_hset(&self) -> HashSet<T>
    where
        T: std::fmt::Debug + Eq + Hash + Clone,
    {
        let mut set = HashSet::<T>::with_capacity(self.len());
        for (e, v) in self.iter() {
            if !set.insert(v.clone()) {
                debug!("Duplicate value in store: ({:?}, {:?})", e, v);
            }
        }
        set
    }

    /// Returns a [BTreeSet] of values.
    pub fn to_bset(&self) -> BTreeSet<T>
    where
        T: std::fmt::Debug + Ord + Clone,
    {
        let mut set = BTreeSet::<T>::new();
        for (e, v) in self.iter() {
            if !set.insert(v.clone()) {
                debug!("Duplicate value in store: ({:?}, {:?})", e, &v);
            }
        }
        set
    }

    /// Returns a map from values to entities.
    ///
    /// Skips if there are duplicate values in the map.
    pub fn inverted_hmap(&self) -> HashMap<T, XvcEntity>
    where
        T: std::fmt::Debug + Eq + Hash + Clone,
    {
        let mut imap = HashMap::<T, XvcEntity>::with_capacity(self.len());
        for (e, v) in self.iter() {
            let ires = imap.insert(v.clone(), *e);
            if ires.is_some() {
                debug!("Duplicate value in store: ({:?}, {:?})", e, v);
            }
        }
        imap
    }

    /// Returns a map from values to entities.
    ///
    /// Skips if there are duplicate values in the map.
    pub fn inverted_bmap(&self) -> BTreeMap<T, XvcEntity>
    where
        T: std::fmt::Debug + Ord + Clone,
    {
        let mut imap = BTreeMap::<T, XvcEntity>::new();
        for (e, v) in self.map.iter() {
            let ires = imap.insert(v.clone(), *e);
            if ires.is_some() {
                debug!("Duplicate value in store: ({:?}, {:?})", e, v);
            }
        }
        imap
    }

    /// Performs a left join with [XvcEntity] keys.
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
    /// `left_join` will return
    ///
    /// `{10: ("John Doe", Some("Carpenter")), 12: ("George Mason", None), 19: ("Ali Canfield",
    /// Some("Artist")}`
    ///
    /// Note that, it may be more convenient to keep this relationship in a [crate::R11Store]
    /// if your stores don't use filtering
    pub fn left_join<U>(&self, other: HStore<U>) -> HStore<(T, Option<U>)>
    where
        T: Storable,
        U: Storable,
    {
        let mut joined = HStore::<(T, Option<U>)>::new();
        for (entity, value) in self.map.iter() {
            joined.insert(*entity, (value.clone(), other.get(entity).cloned()));
        }

        joined
    }

    /// Performs a full join with [XvcEntity] keys.
    ///
    /// The returned store contains `(Option<T>, Option<U>)` values that correspond to the
    /// identical `XvcEntity` values.
    ///
    /// Note that, it may be more convenient to keep this relationship in a [crate::R11Store]
    /// if your stores don't use filtering
    ///
    /// ```rust
    ///
    /// use xvc_ecs::{XvcEntity, HStore};
    ///
    /// let mut store1 = HStore::<String>::new();
    /// store1.insert(10u128.into(), "John Doe".into());
    /// store1.insert(12u128.into(), "George Mason".into());
    /// store1.insert(19u128.into(), "Ali Canfield".into());
    ///
    /// let mut store2 = HStore::<String>::new();
    /// store2.insert(10u128.into(), "Carpenter".into());
    /// store2.insert(17u128.into(), "Developer".into());
    /// store2.insert(15u128.into(), "Plumber".into());
    /// store2.insert(19u128.into(), "Artist".into());
    ///
    /// let result = store1.full_join(store2);
    ///
    /// assert_eq!(result.len(), 5);
    /// assert_eq!(result[&10u128.into()], (Some("John Doe".into()), Some("Carpenter".into())));
    /// assert_eq!(result[&12u128.into()], (Some("George Mason".into()), None));
    /// assert_eq!(result[&15u128.into()], (None, Some("Plumber".into())));
    /// assert_eq!(result[&17u128.into()], (None, Some("Developer".into())));
    /// assert_eq!(result[&19u128.into()], (Some("Ali Canfield".into()), Some("Artist".into())));
    /// ```
    pub fn full_join<U>(&self, other: HStore<U>) -> HStore<(Option<T>, Option<U>)>
    where
        T: Storable,
        U: Storable,
    {
        let all_keys = self.keys().chain(other.keys()).collect::<HashSet<_>>();
        let mut joined = HStore::<(Option<T>, Option<U>)>::new();
        for entity in all_keys.into_iter() {
            joined.insert(
                *entity,
                (self.get(entity).cloned(), other.get(entity).cloned()),
            );
        }

        joined
    }

    /// Performs a join with [XvcEntity] keys.
    ///
    /// The returned store contains `(T, U)` values that correspond to the
    /// identical `XvcEntity` values for values that exist in both stores.
    ///
    /// ## Example
    ///
    /// If this store has
    ///
    /// `{10: "John Doe", 12: "George Mason", 19: "Ali Canfield"}`,
    /// and the `other` store contains
    /// `{10: "Carpenter", 17: "Developer", 15: "Plumber",  19: "Artist" }`
    ///
    /// `join` will return
    ///
    /// `{10: ("John Doe", "Carpenter"),
    ///   19: ("Ali Canfield", "Artist")}`
    ///
    /// Note that, it may be more convenient to keep this relationship in a [crate::R11Store]
    /// if your stores don't use filtering
    /// ```rust
    ///
    /// use xvc_ecs::{XvcEntity, HStore};
    ///
    /// let mut store1 = HStore::<String>::new();
    /// store1.insert(10u128.into(), "John Doe".into());
    /// store1.insert(12u128.into(), "George Mason".into());
    /// store1.insert(19u128.into(), "Ali Canfield".into());
    ///
    /// let mut store2 = HStore::<String>::new();
    /// store2.insert(10u128.into(), "Carpenter".into());
    /// store2.insert(17u128.into(), "Developer".into());
    /// store2.insert(15u128.into(), "Plumber".into());
    /// store2.insert(19u128.into(), "Artist".into());
    ///
    /// let result = store1.join(store2);
    ///
    /// assert_eq!(result.len(), 2);
    /// assert_eq!(result[&10u128.into()], ("John Doe".into(), "Carpenter".into()));
    /// assert_eq!(result[&19u128.into()], ("Ali Canfield".into(), "Artist".into()));
    pub fn join<U>(&self, other: HStore<U>) -> HStore<(T, U)>
    where
        T: Storable,
        U: Storable,
    {
        let mut joined = HStore::<(T, U)>::new();
        self.map.iter().for_each(|(entity, value)| {
            if let Some(other_value) = other.get(entity) {
                joined.insert(*entity, (value.clone(), other_value.clone()));
            }
        });

        joined
    }

    /// returns a subset of the store defined by iterator of XvcEntity
    pub fn subset<I>(&self, keys: I) -> Result<HStore<T>>
    where
        I: Iterator<Item = XvcEntity>,
        T: Clone,
    {
        let mut map = HashMap::<XvcEntity, T>::with_capacity(self.len());
        for e in keys {
            if let Some(v) = self.get(&e) {
                map.insert(e, v.clone());
            } else {
                Error::CannotFindKeyInStore { key: e.to_string() }.warn();
            }
        }
        Ok(Self { map })
    }
    /// Creates a new map by calling the `predicate` with each value.
    ///
    /// `predicate` must be a function or closure that returns `bool`.
    ///
    /// It doesn't clone the values.
    pub fn filter<F>(&self, predicate: F) -> HStore<&T>
    where
        F: Fn(&XvcEntity, &T) -> bool,
    {
        let mut m = HashMap::<XvcEntity, &T>::new();
        for (e, v) in self.map.iter() {
            if predicate(e, v) {
                m.insert(*e, v);
            }
        }

        HStore::from(m)
    }

    /// Returns the first element of the map.
    pub fn first(&self) -> Option<(&XvcEntity, &T)> {
        self.map.iter().next()
    }
}

impl<T: Clone> HStore<&T> {
    /// Returns a new map by cloning the values.
    pub fn cloned(&self) -> HStore<T> {
        let mut map = HashMap::<XvcEntity, T>::with_capacity(self.len());
        for (e, v) in self.iter() {
            map.insert(*e, (*v).clone());
        }
        HStore::from(map)
    }
}

impl<T> IntoIterator for HStore<T> {
    type Item = (XvcEntity, T);
    type IntoIter = std::collections::hash_map::IntoIter<XvcEntity, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<T: PartialEq> HStore<T> {
    /// Returns the entities for a `value`.
    ///
    /// There may be more than one entity for a given value, hence it returns a `Vec`.
    /// It uses internal reverse index for fast lookup.
    pub fn entities_for(&self, value: &T) -> Option<Vec<XvcEntity>>
    where
        T: PartialEq,
    {
        let entity_vec: Vec<XvcEntity> = self
            .map
            .iter()
            .filter_map(|(k, v)| if *v == *value { Some(*k) } else { None })
            .collect();
        if entity_vec.is_empty() {
            None
        } else {
            Some(entity_vec)
        }
    }

    /// Returns the first entity matched with [Self::entities_for]
    pub fn entity_by_value(&self, value: &T) -> Option<XvcEntity> {
        match self.entities_for(value) {
            Some(vec_e) => vec_e.first().copied(),
            None => None,
        }
    }
}

impl<T> From<(XvcEntity, T)> for HStore<T> {
    fn from((e, v): (XvcEntity, T)) -> Self {
        let mut store = HStore::<T>::new();
        store.insert(e, v);
        store
    }
}
