//! A component store for ephemeral operations based on [HashMap].
//! Unlike [XvcStore], it doesn't require `T` to be serializable.
//! It's supposed to be used operations that don't require final result to be recorded to disk.
use super::*;
use crate::error::{Error, Result};
use crate::Storable;
use log::debug;
use rayon::iter::{FromParallelIterator, ParallelIterator};

use std::collections::hash_map::IterMut;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{FromIterator, Iterator};

use super::vstore::VStore;

use std::ops::{Deref, DerefMut};

/// This is a HashMap for more random access and less restrictions, no support for serialization
#[derive(Debug, Clone)]
pub struct HStore<T> {
    /// The wrapped map for the store
    pub map: HashMap<XvcEntity, T>,
}

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

    /// Calls the inner map's insert
    pub fn insert(&mut self, entity: XvcEntity, value: T) -> Option<T> {
        self.map.insert(entity, value)
    }

    /// Returns the inner map's iter_mut
    pub fn iter_mut(&mut self) -> IterMut<'_, XvcEntity, T> {
        self.map.iter_mut()
    }

    /// Return a mutable value for `entity`
    pub fn get_mut(&mut self, entity: &XvcEntity) -> Option<&mut T> {
        self.map.get_mut(entity)
    }
}

impl<T> Default for HStore<T> {
    fn default() -> Self {
        Self::new()
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

    /// Creates values from `iter` and gets new entities from `gen` to create new records.
    pub fn from_iter<I>(gen: &XvcEntityGenerator, iter: I) -> Result<HStore<T>>
    where
        I: IntoIterator<Item = T>,
    {
        let mut hstore = HStore::<T>::new();
        for value in iter {
            let key = gen.next_element();
            hstore.map.insert(key, value);
        }
        Ok(hstore)
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
            if ires != None {
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
            if ires != None {
                debug!("Duplicate value in store: ({:?}, {:?})", e, v);
            }
        }
        imap
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
                Error::CannotFindKeyInStore { key: e.0 }.warn();
            }
        }
        Ok(Self { map })
    }
}
